//! Native Bun adapter for the stable `libbun` facade.
//!
//! This crate is intentionally separate from the stable facade crate because
//! upstream Bun currently requires its pinned nightly toolchain and generated
//! codegen inputs.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::ptr::NonNull;

use bun_core::{String as BunString, ZigString};
use bun_jsc::js_promise::Status as PromiseStatus;
use bun_jsc::virtual_machine::{InitOptions, VirtualMachine};
use bun_jsc::{
    AnyPromise, BuiltinName, JSGlobalObject, JSInternalPromise, JSModuleLoader, JSPromise,
    JSValue, JSType, ZigStringJsc,
};
use bun_runtime as _;
use libbun::OutputStream;
use libbun::{
    BunAsyncHandle, BunEmbeddingRuntime, BunModuleHandle, BunModuleSpec, BunRuntimeConfig,
    ExportCallResult, LibbunError, LibbunResult, OutputRecord, ProviderCallResult, ProviderError,
    PumpBudget, PumpOutcome, SinkPolicy, StructuralValue,
};

#[derive(Debug)]
pub struct NativeBunRuntime {
    vm: NonNull<VirtualMachine>,
    modules: BTreeMap<String, JSValue>,
    pending: BTreeMap<String, JSValue>,
    output: Vec<OutputRecord>,
    stdout: OutputCapture,
    stderr: OutputCapture,
    tempdir: tempfile::TempDir,
    next_module: u64,
    next_async: u64,
    shutdown: bool,
}

#[derive(Debug)]
struct OutputCapture {
    stream: OutputStream,
    policy: SinkPolicy,
    write_file: std::fs::File,
    read_file: std::fs::File,
    read_offset: u64,
}

impl NativeBunRuntime {
    fn vm(&self) -> &VirtualMachine {
        // SAFETY: `vm` is initialized in `initialize` and remains live until
        // `shutdown`, which consumes all public operations through the facade.
        unsafe { self.vm.as_ref() }
    }

    fn vm_mut(&mut self) -> &mut VirtualMachine {
        // SAFETY: `NativeBunRuntime` is `&mut self`-borrowed for all VM-driving
        // methods, matching Bun's single-JS-thread contract.
        unsafe { self.vm.as_mut() }
    }

    fn evaluate_json(&self, value: &StructuralValue) -> LibbunResult<JSValue> {
        let json = serde_json::to_string(&value.0)
            .map_err(|err| LibbunError::export_call(format!("input JSON encode failed: {err}")))?;
        let json = ZigString::init(json.as_bytes());
        self.vm().run_with_api_lock(|| {
            let value = json.to_json_object(self.vm().global());
            if value.is_empty() {
                Err(LibbunError::export_call("input JSON parse failed"))
            } else {
                Ok(value)
            }
        })
    }

    fn value_to_result(&self, value: JSValue) -> LibbunResult<ProviderCallResult> {
        if value.is_undefined() || value.is_null() {
            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
        }

        let mut out = BunString::empty();
        self.vm()
            .run_with_api_lock(|| value.json_stringify_fast(self.vm().global(), &mut out))
            .map_err(|_| LibbunError::export_call("JSON.stringify threw"))?;
        let bytes = out.to_utf8_bytes();
        out.deref();

        if bytes.is_empty() {
            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
        }

        let parsed = serde_json::from_slice(&bytes).map_err(|err| {
            LibbunError::export_call(format!("provider result is not structurally serializable: {err}"))
        })?;
        Ok(ProviderCallResult::Ok(StructuralValue(parsed)))
    }

    fn rejected_to_result(&self, value: JSValue) -> ProviderCallResult {
        let value = value.to_error().unwrap_or(value);
        ProviderCallResult::Err(ProviderError {
            code: "provider_rejected".to_string(),
            message: self.js_error_to_string(value, "provider promise rejected"),
        })
    }

    fn js_error_to_string(&self, value: JSValue, fallback: &str) -> String {
        let global = self.vm().global();
        if value.is_object() {
            if let Ok(Some(message)) = value.get(global, "message") {
                if let Some(text) = js_value_to_string(global, message) {
                    return text;
                }
            }
            if let Ok(Some(message)) = value.fast_get(global, BuiltinName::Message) {
                if let Some(text) = js_value_to_string(global, message) {
                    return text;
                }
            }
            if let Ok(Some(name)) = value.fast_get(global, BuiltinName::name) {
                if let Some(text) = js_value_to_string(global, name) {
                    return text;
                }
            }
            return fallback.to_string();
        }

        js_value_to_string(global, value).unwrap_or_else(|| fallback.to_string())
    }

    fn promise_result(&self, promise: *mut JSInternalPromise) -> LibbunResult<Option<ProviderCallResult>> {
        let status = JSPromise::status_ptr(promise);
        if status == PromiseStatus::Pending {
            return Ok(None);
        }

        let value = JSInternalPromise::opaque_mut(promise).result(unsafe { &*self.vm().jsc_vm });
        if status == PromiseStatus::Rejected {
            JSInternalPromise::opaque_mut(promise).set_handled();
            return Ok(Some(self.rejected_to_result(value)));
        }
        Ok(Some(self.value_to_result(value)?))
    }

    fn drain_output(&mut self) -> LibbunResult<()> {
        bun_core::Output::flush();
        self.stdout.drain_into(&mut self.output)?;
        self.stderr.drain_into(&mut self.output)?;
        Ok(())
    }
}

impl OutputCapture {
    fn create(
        dir: &Path,
        name: &str,
        stream: OutputStream,
        policy: SinkPolicy,
    ) -> LibbunResult<Self> {
        let path = dir.join(name);
        let write_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)
            .map_err(|err| LibbunError::initialize(format!("output capture create failed: {err}")))?;
        let read_file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(|err| LibbunError::initialize(format!("output capture open failed: {err}")))?;
        Ok(Self {
            stream,
            policy,
            write_file,
            read_file,
            read_offset: 0,
        })
    }

    fn bun_file(&self) -> bun_core::Output::File {
        bun_core::Output::File(fd_from_file(&self.write_file))
    }

    fn drain_into(&mut self, output: &mut Vec<OutputRecord>) -> LibbunResult<()> {
        let len = self
            .read_file
            .metadata()
            .map_err(|err| LibbunError::export_call(format!("output metadata failed: {err}")))?
            .len();
        if len <= self.read_offset {
            return Ok(());
        }

        self.read_file
            .seek(SeekFrom::Start(self.read_offset))
            .map_err(|err| LibbunError::export_call(format!("output seek failed: {err}")))?;
        let mut bytes = Vec::with_capacity((len - self.read_offset) as usize);
        self.read_file
            .read_to_end(&mut bytes)
            .map_err(|err| LibbunError::export_call(format!("output read failed: {err}")))?;
        self.read_offset = len;

        if self.policy == SinkPolicy::Capture && !bytes.is_empty() {
            output.push(OutputRecord {
                stream: self.stream,
                text: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }
        Ok(())
    }
}

#[cfg(unix)]
fn fd_from_file(file: &std::fs::File) -> bun_core::Fd {
    use std::os::fd::AsRawFd;

    bun_core::Fd::from_native(file.as_raw_fd())
}

#[cfg(windows)]
fn fd_from_file(file: &std::fs::File) -> bun_core::Fd {
    use std::os::windows::io::AsRawHandle;

    bun_core::Fd::from_system(file.as_raw_handle().cast())
}

fn js_value_to_string(global: &JSGlobalObject, value: JSValue) -> Option<String> {
    if !value.is_string() {
        return None;
    }

    match value.to_slice_clone(global) {
        Ok(text) => {
            let bytes = text.into_vec();
            Some(String::from_utf8_lossy(&bytes).into_owned())
        }
        _ => None,
    }
}

impl BunEmbeddingRuntime for NativeBunRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        ensure_macos_compat_symbols();
        bun_core::StackCheck::configure_thread();

        let tempdir = tempfile::Builder::new()
            .prefix("libbun-native-")
            .tempdir_in(&config.working_directory)
            .map_err(|err| LibbunError::initialize(format!("tempdir create failed: {err}")))?;
        let stdout = OutputCapture::create(
            tempdir.path(),
            "stdout.capture",
            OutputStream::Stdout,
            config.stdout,
        )?;
        let stderr = OutputCapture::create(
            tempdir.path(),
            "stderr.capture",
            OutputStream::Stderr,
            config.stderr,
        )?;
        bun_core::Output::Source::set_init(stdout.bun_file(), stderr.bun_file());

        bun_jsc::initialize(false);
        bun_ast::initialize_store();

        let vm = VirtualMachine::init(InitOptions {
            is_main_thread: true,
            mini_mode: false,
            ..Default::default()
        })
        .map_err(|err| LibbunError::initialize(format!("{err:?}")))?;

        Ok(Self {
            vm: NonNull::new(vm).ok_or_else(|| LibbunError::initialize("Bun VM init returned null"))?,
            modules: BTreeMap::new(),
            pending: BTreeMap::new(),
            output: Vec::new(),
            stdout,
            stderr,
            tempdir,
            next_module: 1,
            next_async: 1,
            shutdown: false,
        })
    }

    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }

        let id = format!("module-{}", self.next_module);
        self.next_module += 1;

        let module_path = match spec {
            BunModuleSpec::Source { source, .. } => {
                let path = self.tempdir.path().join(format!("{id}.mjs"));
                std::fs::write(&path, source)
                    .map_err(|err| LibbunError::module_load(format!("source write failed: {err}")))?;
                path
            }
            BunModuleSpec::Path { path } => path,
            BunModuleSpec::PreparedBundle { .. } => {
                return Err(LibbunError::module_load(
                    "prepared Bun bundle loading is not implemented by the native adapter yet",
                ));
            }
        };

        let specifier = path_to_file_specifier(&module_path)?;
        let specifier = BunString::from_bytes(specifier.as_bytes());
        let promise = JSModuleLoader::import_ptr(self.vm().global, &specifier)
            .map_err(|_| LibbunError::module_load("module import threw"))?;
        self.vm_mut()
            .wait_for_promise(AnyPromise::Internal(promise.as_ptr()));

        let Some(result) = self.promise_result(promise.as_ptr())? else {
            return Err(LibbunError::module_load("module import remained pending after wait"));
        };
        match result {
            ProviderCallResult::Ok(_) => {
                let namespace =
                    JSInternalPromise::opaque_mut(promise.as_ptr()).result(unsafe { &*self.vm().jsc_vm });
                self.vm().run_with_api_lock(|| namespace.protect());
                self.modules.insert(id.clone(), namespace);
                self.drain_output()?;
                Ok(BunModuleHandle { id })
            }
            ProviderCallResult::Err(error) => Err(LibbunError::module_load(format!(
                "{}: {}",
                error.code, error.message
            ))),
        }
    }

    fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: StructuralValue,
    ) -> LibbunResult<ExportCallResult> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }

        let namespace = *self
            .modules
            .get(&module.id)
            .ok_or_else(|| LibbunError::module_load("unknown module handle"))?;
        let function = namespace
            .get(self.vm().global(), export)
            .map_err(|_| LibbunError::export_call(format!("export lookup threw: {export}")))?
            .ok_or_else(|| LibbunError::export_call(format!("missing export `{export}`")))?;
        if !function.is_callable() {
            return Err(LibbunError::export_call(format!(
                "export `{export}` is not callable"
            )));
        }

        let arg = self.evaluate_json(&input)?;
        let result = match self
            .vm()
            .run_with_api_lock(|| match function.call(self.vm().global(), namespace, &[arg]) {
                Ok(result) => Ok(result),
                Err(error) => {
                    let exception = self.vm().global().take_exception(error);
                    Err(self.rejected_to_result(exception))
                }
            }) {
            Ok(result) => result,
            Err(error) => {
                self.drain_output()?;
                return Ok(ExportCallResult::Ready(error));
            }
        };

        if result.is_cell() && result.js_type() == JSType::JSPromise {
            let id = format!("async-{}", self.next_async);
            self.next_async += 1;
            self.vm().run_with_api_lock(|| result.protect());
            self.pending.insert(id.clone(), result);
            self.drain_output()?;
            return Ok(ExportCallResult::Pending(BunAsyncHandle { id }));
        }

        let result = self.value_to_result(result)?;
        self.drain_output()?;
        Ok(ExportCallResult::Ready(result))
    }

    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        let mut ticks = 0;
        for _ in 0..budget.max_ticks {
            self.vm_mut().event_loop_mut().tick();
            ticks += 1;
        }
        self.drain_output()?;
        Ok(PumpOutcome {
            ticks,
            pending_async_work: self.pending.len(),
        })
    }

    fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        let value = *self
            .pending
            .get(&handle.id)
            .ok_or_else(|| LibbunError::UnknownAsyncHandle {
                handle: handle.id.clone(),
            })?;
        if !(value.is_cell() && value.js_type() == JSType::JSPromise) {
            return Err(LibbunError::UnknownAsyncHandle {
                handle: handle.id.clone(),
            });
        }
        let promise = value
            .as_internal_promise()
            .ok_or_else(|| LibbunError::UnknownAsyncHandle {
                handle: handle.id.clone(),
            })?;
        let result = self.promise_result(promise)?;
        if result.is_some() {
            self.vm().run_with_api_lock(|| value.unprotect());
            self.pending.remove(&handle.id);
        }
        self.drain_output()?;
        Ok(result)
    }

    fn captured_output(&self) -> &[OutputRecord] {
        &self.output
    }

    fn shutdown(&mut self) -> LibbunResult<()> {
        if self.shutdown {
            return Ok(());
        }
        for (_, value) in std::mem::take(&mut self.pending) {
            self.vm().run_with_api_lock(|| value.unprotect());
        }
        for (_, value) in std::mem::take(&mut self.modules) {
            self.vm().run_with_api_lock(|| value.unprotect());
        }
        self.drain_output()?;
        self.vm_mut().destroy();
        self.shutdown = true;
        Ok(())
    }
}

fn path_to_file_specifier(path: &Path) -> LibbunResult<String> {
    let path = path
        .canonicalize()
        .map_err(|err| LibbunError::module_load(format!("canonicalize failed: {err}")))?;
    url::Url::from_file_path(&path)
        .map(|url| url.to_string())
        .map_err(|()| {
            LibbunError::module_load(format!(
                "path cannot be represented as a file URL: {}",
                path.display()
            ))
        })
}

#[cfg(target_os = "macos")]
fn ensure_macos_compat_symbols() {
    let symbol = libbun_libcxx_hash_memory_compat as extern "C" fn(*const std::ffi::c_void, usize) -> usize;
    std::hint::black_box(symbol);
}

#[cfg(not(target_os = "macos"))]
fn ensure_macos_compat_symbols() {}

#[cfg(target_os = "macos")]
#[unsafe(export_name = "_ZNSt3__113__hash_memoryEPKvm")]
pub extern "C" fn libbun_libcxx_hash_memory_compat(
    data: *const std::ffi::c_void,
    len: usize,
) -> usize {
    if len == 0 || data.is_null() {
        return 0;
    }

    // Bun's static C++/WebKit objects may be built against an SDK whose libc++
    // expects this ABI helper, while older macOS runtime dylibs do not export it.
    let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), len) };
    if usize::BITS == 64 {
        let mut hash = 0xcbf29ce484222325_u64;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash as usize
    } else {
        let mut hash = 0x811c9dc5_u32;
        for byte in bytes {
            hash ^= u32::from(*byte);
            hash = hash.wrapping_mul(0x01000193);
        }
        hash as usize
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Bun__panic(msg: *const u8, len: usize) -> ! {
    let bytes = if msg.is_null() {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(msg, len) }
    };
    bun_core::Output::panic(format_args!("{}", String::from_utf8_lossy(bytes)));
}

#[unsafe(no_mangle)]
pub extern "C" fn Bun__VM__scriptExecutionStatus(vm: *const VirtualMachine) -> i32 {
    if vm.is_null() {
        return 1;
    }
    let vm = unsafe { &*vm };
    vm.script_execution_status() as i32
}
