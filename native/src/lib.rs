//! Native Bun adapter for the stable `libbun` facade.
//!
//! This crate is intentionally separate from the stable facade crate because
//! upstream Bun currently requires its pinned nightly toolchain and generated
//! codegen inputs.

#[cfg(not(feature = "internal-adapter"))]
compile_error!(
    "`libbun-native` is an internal implementation crate. Build the dynamic \
     plugin instead of statically linking this crate into a downstream host."
);

use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::TryLockError;

use bun_core::{String as BunString, ZigString};
use bun_jsc::js_promise::{Status as PromiseStatus, UnwrapMode, Unwrapped};
use bun_jsc::virtual_machine::{InitOptions, VirtualMachine};
use bun_jsc::{
    AnyPromise, BuiltinName, JSGlobalObject, JSInternalPromise, JSModuleLoader, JSPromise, JSType,
    JSValue, ZigStringJsc,
};
use bun_platform as _;
use bun_runtime as _;
use libbun::OutputStream;
use libbun::{
    BunAsyncHandle, BunEmbeddingRuntime, BunModuleHandle, BunModuleSpec, BunRuntimeConfig,
    ExportCallResult, LibbunError, LibbunResult, OutputRecord, PreparedBundleV1,
    ProviderCallResult, ProviderError, PumpBudget, PumpOutcome, SinkPolicy, StructuralValue,
};

#[derive(Debug)]
pub struct NativeBunRuntime {
    vm: NonNull<VirtualMachine>,
    modules: BTreeMap<String, JSValue>,
    pending: BTreeMap<String, JSValue>,
    output: Vec<OutputRecord>,
    stdout: OutputCapture,
    stderr: OutputCapture,
    log: OutputCapture,
    source_module_paths: Vec<Box<[u8]>>,
    source_module_sources: Vec<Box<[u8]>>,
    prepared_bundle_tempdirs: Vec<tempfile::TempDir>,
    _runtime_guard: MutexGuard<'static, ()>,
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
            LibbunError::export_call(format!(
                "provider result is not structurally serializable: {err}"
            ))
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
            let stack = value
                .get(global, "stack")
                .ok()
                .flatten()
                .and_then(|stack| js_value_to_string(global, stack));
            if let Some(stack) = stack.filter(|stack| !stack.trim().is_empty()) {
                return bounded_js_diagnostic_text(stack);
            }

            let message = value
                .get(global, "message")
                .ok()
                .flatten()
                .and_then(|message| js_value_to_string(global, message))
                .or_else(|| {
                    value
                        .fast_get(global, BuiltinName::Message)
                        .ok()
                        .flatten()
                        .and_then(|message| js_value_to_string(global, message))
                });
            let name = value
                .fast_get(global, BuiltinName::name)
                .ok()
                .flatten()
                .and_then(|name| js_value_to_string(global, name));
            match (name, message) {
                (Some(name), Some(message))
                    if !name.trim().is_empty() && !message.trim().is_empty() =>
                {
                    return bounded_js_diagnostic_text(format!("{name}: {message}"));
                }
                (_, Some(message)) if !message.trim().is_empty() => {
                    return bounded_js_diagnostic_text(message);
                }
                (Some(name), _) if !name.trim().is_empty() => {
                    return bounded_js_diagnostic_text(name);
                }
                _ => {}
            }
            return js_value_to_string_lossy(global, value)
                .map(bounded_js_diagnostic_text)
                .unwrap_or_else(|| fallback.to_string());
        }

        js_value_to_string_lossy(global, value)
            .map(bounded_js_diagnostic_text)
            .unwrap_or_else(|| fallback.to_string())
    }

    fn evaluate_source_module(&mut self, id: &str, source: &str) -> LibbunResult<JSValue> {
        let source_path = std::env::current_dir()
            .map_err(|err| LibbunError::module_load(format!("current_dir failed: {err}")))?
            .join(format!("libbun-{id}/[eval]"))
            .to_string_lossy()
            .into_owned();
        let specifier = source_path.clone();
        self.source_module_paths
            .push(source_path.into_bytes().into_boxed_slice());
        self.source_module_sources
            .push(source.as_bytes().to_vec().into_boxed_slice());
        let (eval_source, source_path_ptr, source_path_len) = {
            let source_path = self
                .source_module_paths
                .last()
                .expect("source module path was just pushed");
            let source = self
                .source_module_sources
                .last()
                .expect("source module source was just pushed");
            let eval_source = bun_ast::Source::init_path_string(&source_path[..], &source[..]);
            (eval_source, source_path.as_ptr(), source_path.len())
        };
        let vm = self.vm_mut();
        vm.module_loader.eval_source = Some(Box::new(eval_source));
        // SAFETY: source_module_paths owns this allocation for the runtime lifetime.
        vm.set_main(unsafe { std::slice::from_raw_parts(source_path_ptr, source_path_len) });
        self.import_module_specifier(&specifier)
    }

    fn import_module_specifier(&mut self, specifier: &str) -> LibbunResult<JSValue> {
        let import_specifier = specifier.to_owned();
        let specifier = BunString::from_bytes(specifier.as_bytes());
        let promise = JSModuleLoader::import_ptr(self.vm().global, &specifier).map_err(|err| {
            let exception = self.vm().global().take_exception(err);
            let error = exception.to_error().unwrap_or(exception);
            LibbunError::module_load(format!(
                "module import threw for specifier `{import_specifier}`: {}",
                self.js_error_to_string(error, "JavaScriptCore did not expose exception details")
            ))
        })?;
        self.resolve_module_promise(
            AnyPromise::Internal(promise.as_ptr()),
            &format!("module import `{import_specifier}`"),
        )
    }

    fn resolve_module_promise(
        &mut self,
        promise: AnyPromise,
        operation: &str,
    ) -> LibbunResult<JSValue> {
        self.vm_mut().wait_for_promise(promise);

        match promise.unwrap(unsafe { &*self.vm().jsc_vm }, UnwrapMode::MarkHandled) {
            Unwrapped::Pending => Err(LibbunError::module_load(format!(
                "{operation} remained pending after wait"
            ))),
            Unwrapped::Rejected(value) => {
                let error = self.rejected_to_result(value);
                match error {
                    ProviderCallResult::Err(error) => Err(LibbunError::module_load(format!(
                        "{}: {}",
                        error.code, error.message
                    ))),
                    ProviderCallResult::Ok(_) => {
                        Err(LibbunError::module_load(format!("{operation} rejected")))
                    }
                }
            }
            Unwrapped::Fulfilled(namespace) => Ok(namespace),
        }
    }

    fn promise_result(
        &self,
        promise: *mut JSInternalPromise,
    ) -> LibbunResult<Option<ProviderCallResult>> {
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
        self.log.drain_into(&mut self.output)?;
        Ok(())
    }

    fn materialize_prepared_bundle(
        &mut self,
        module_id: &str,
        bundle_id: &str,
        bytes: &[u8],
    ) -> LibbunResult<PathBuf> {
        let bundle = PreparedBundleV1::from_bytes(bytes)?;
        bundle.validate_for_current_runtime(bundle_id)?;

        let tempdir = tempfile::Builder::new()
            .prefix("libbun-prepared-bundle-")
            .tempdir()
            .map_err(|err| {
                LibbunError::module_load(format!("prepared bundle tempdir create failed: {err}"))
            })?;
        let bundle_dir = tempdir.path().join(format!("{module_id}.bundle"));
        std::fs::create_dir_all(&bundle_dir).map_err(|err| {
            LibbunError::module_load(format!("prepared bundle directory create failed: {err}"))
        })?;

        for (module_path, module) in &bundle.modules {
            let path = bundle_dir.join(module_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|err| {
                    LibbunError::module_load(format!(
                        "prepared bundle module directory create failed: {err}"
                    ))
                })?;
            }
            std::fs::write(&path, module.source.as_bytes()).map_err(|err| {
                LibbunError::module_load(format!(
                    "prepared bundle module `{module_path}` write failed: {err}"
                ))
            })?;
        }

        let entry_module = bundle_dir.join(bundle.entry_module);
        self.prepared_bundle_tempdirs.push(tempdir);
        Ok(entry_module)
    }
}

bun_core::declare_scope!(LibbunNative, visible);

impl OutputCapture {
    fn create(stream: OutputStream, policy: SinkPolicy) -> LibbunResult<Self> {
        let (read_file, write_file) = create_nonblocking_pipe_pair()?;
        Ok(Self {
            stream,
            policy,
            write_file,
            read_file,
        })
    }

    fn bun_file(&self) -> bun_core::Output::File {
        bun_core::Output::File(fd_from_file(&self.write_file))
    }

    fn drain_into(&mut self, output: &mut Vec<OutputRecord>) -> LibbunResult<()> {
        let mut bytes = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match self.read_file.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => bytes.extend_from_slice(&buffer[..read]),
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => {
                    return Err(LibbunError::export_call(format!(
                        "output pipe read failed: {err}"
                    )));
                }
            }
        }

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

#[cfg(unix)]
fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
    use std::os::fd::FromRawFd;

    let mut fds = [0; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        return Err(LibbunError::initialize(format!(
            "output pipe create failed: {}",
            std::io::Error::last_os_error()
        )));
    }

    if let Err(err) = set_nonblocking(fds[0]) {
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
        return Err(err);
    }

    let read_file = unsafe { std::fs::File::from_raw_fd(fds[0]) };
    let write_file = unsafe { std::fs::File::from_raw_fd(fds[1]) };
    Ok((read_file, write_file))
}

#[cfg(unix)]
fn set_nonblocking(fd: libc::c_int) -> LibbunResult<()> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 {
        return Err(LibbunError::initialize(format!(
            "output pipe flags read failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    if unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(LibbunError::initialize(format!(
            "output pipe nonblocking setup failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
    Err(LibbunError::initialize(
        "file-free output capture is currently implemented for Unix targets",
    ))
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

fn js_value_to_string_lossy(global: &JSGlobalObject, value: JSValue) -> Option<String> {
    match value.to_slice_clone(global) {
        Ok(text) => {
            let bytes = text.into_vec();
            Some(String::from_utf8_lossy(&bytes).into_owned())
        }
        _ => js_value_to_string(global, value),
    }
}

fn bounded_js_diagnostic_text(text: impl Into<String>) -> String {
    const MAX_JS_DIAGNOSTIC_BYTES: usize = 16 * 1024;
    let mut text = text.into();
    if text.len() <= MAX_JS_DIAGNOSTIC_BYTES {
        return text;
    }

    let mut boundary = MAX_JS_DIAGNOSTIC_BYTES;
    while !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    text.truncate(boundary);
    text.push_str("\n[libbun truncated JavaScript diagnostic after 16384 bytes]");
    text
}

fn apply_environment_overlay(
    vm: &mut VirtualMachine,
    environment: &BTreeMap<String, String>,
) -> LibbunResult<()> {
    if environment.is_empty() {
        return Ok(());
    }

    let env = vm.transpiler.env_mut();
    for (key, value) in environment {
        validate_environment_key(key)?;
        env.map
            .put_alloc_key_and_value(key.as_bytes(), value.as_bytes())
            .map_err(|err| {
                LibbunError::initialize(format!("environment overlay apply failed: {err:?}"))
            })?;
    }
    Ok(())
}

fn validate_environment_key(key: &str) -> LibbunResult<()> {
    if key.is_empty() || key.contains('=') || key.contains('\0') {
        return Err(LibbunError::initialize(format!(
            "invalid environment overlay key `{key}`"
        )));
    }
    Ok(())
}

impl BunEmbeddingRuntime for NativeBunRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let runtime_guard = native_runtime_guard().try_lock().map_err(|err| match err {
            TryLockError::WouldBlock => LibbunError::initialize(
                "another native Bun runtime is already active in this process",
            ),
            TryLockError::Poisoned(_) => {
                LibbunError::initialize("native Bun runtime guard is poisoned")
            }
        })?;
        ensure_macos_compat_symbols();
        bun_core::StackCheck::configure_thread();

        let stdout = OutputCapture::create(OutputStream::Stdout, config.stdout)?;
        let stderr = OutputCapture::create(OutputStream::Stderr, config.stderr)?;
        let log = OutputCapture::create(OutputStream::Log, config.log)?;
        bun_core::Output::Source::set_init(stdout.bun_file(), stderr.bun_file());
        bun_core::Output::init_scoped_debug_writer_at_startup();
        unsafe {
            bun_core::Output::scoped_debug_writer::SCOPED_FILE_WRITER
                .write(bun_core::Output::output_sink().quiet_writer_from_fd(log.bun_file().0));
        }

        bun_jsc::initialize(false);
        bun_ast::initialize_store();

        let vm = VirtualMachine::init(InitOptions {
            is_main_thread: true,
            mini_mode: false,
            ..Default::default()
        })
        .map_err(|err| LibbunError::initialize(format!("{err:?}")))?;
        let vm =
            NonNull::new(vm).ok_or_else(|| LibbunError::initialize("Bun VM init returned null"))?;
        apply_environment_overlay(
            unsafe { vm.as_ptr().as_mut().expect("vm pointer checked") },
            &config.environment,
        )?;
        // Bun's module loader/transpiler expects this VM-owned initialization
        // before any provider import can reach source loading.
        unsafe { vm.as_ptr().as_mut().expect("vm pointer checked") }
            .load_extra_env_and_source_code_printer();

        Ok(Self {
            vm,
            modules: BTreeMap::new(),
            pending: BTreeMap::new(),
            output: Vec::new(),
            stdout,
            stderr,
            log,
            source_module_paths: Vec::new(),
            source_module_sources: Vec::new(),
            prepared_bundle_tempdirs: Vec::new(),
            _runtime_guard: runtime_guard,
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
        bun_core::scoped_log!(LibbunNative, "loading module {}", id);

        let namespace = match spec {
            BunModuleSpec::Source { source, .. } => self.evaluate_source_module(&id, &source)?,
            BunModuleSpec::Path { path } => {
                let specifier = path_to_file_specifier(&path)?;
                self.import_module_specifier(&specifier)?
            }
            BunModuleSpec::PreparedBundle { bundle_id, bytes } => {
                let specifier = path_to_file_specifier(
                    &self.materialize_prepared_bundle(&id, &bundle_id, &bytes)?,
                )?;
                self.import_module_specifier(&specifier)?
            }
        };

        self.vm().run_with_api_lock(|| namespace.protect());
        self.modules.insert(id.clone(), namespace);
        self.drain_output()?;
        Ok(BunModuleHandle { id })
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
        let result = match self.vm().run_with_api_lock(|| {
            match function.call(self.vm().global(), namespace, &[arg]) {
                Ok(result) => Ok(result),
                Err(error) => {
                    let exception = self.vm().global().take_exception(error);
                    Err(self.rejected_to_result(exception))
                }
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
            self.vm_mut().tick();
            self.vm_mut().auto_tick();
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
        let value =
            *self
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
        let promise =
            value
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

    fn drain_captured_output(&mut self) -> Vec<OutputRecord> {
        std::mem::take(&mut self.output)
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
        // `VirtualMachine::destroy` is Bun's worker-thread teardown path. The
        // embedded libbun runtime initializes a main-thread VM, matching Bun's
        // process-lifetime CLI shape, so leave VM-owned native state live until
        // process exit.
        self.shutdown = true;
        Ok(())
    }
}

fn native_runtime_guard() -> &'static Mutex<()> {
    static NATIVE_RUNTIME_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    NATIVE_RUNTIME_GUARD.get_or_init(|| Mutex::new(()))
}

fn path_to_file_specifier(path: &Path) -> LibbunResult<String> {
    // Avoid `std::fs::canonicalize` here. On Linux, Bun's linked mimalloc
    // symbols can interpose the free path for libc `realpath` allocations,
    // which makes canonicalize report a mimalloc invalid-pointer diagnostic.
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| LibbunError::module_load(format!("current_dir failed: {err}")))?
            .join(path)
    };
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
    let symbol =
        libbun_libcxx_hash_memory_compat as extern "C" fn(*const std::ffi::c_void, usize) -> usize;
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
