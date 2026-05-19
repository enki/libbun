use std::ffi::c_void;
use std::panic::AssertUnwindSafe;

use libbun::plugin_abi::{LIBBUN_PLUGIN_ABI_VERSION, LibbunPluginBuffer, LibbunPluginStatus};
use libbun::{
    BunAsyncHandle, BunModuleHandle, BunModuleSpec, BunRuntimeConfig, ExportCallResult,
    LibbunError, OutputRecord, ProviderCallResult, PumpBudget, PumpOutcome, StructuralValue,
};

trait RuntimeTransport {
    fn load_module(&mut self, spec: BunModuleSpec) -> libbun::LibbunResult<BunModuleHandle>;

    fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: StructuralValue,
    ) -> libbun::LibbunResult<ExportCallResult>;

    fn pump_event_loop(&mut self, budget: PumpBudget) -> libbun::LibbunResult<PumpOutcome>;

    fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> libbun::LibbunResult<Option<ProviderCallResult>>;

    fn drain_output(&mut self) -> libbun::LibbunResult<Vec<OutputRecord>>;

    fn shutdown(&mut self) -> libbun::LibbunResult<()>;
}

struct PluginRuntime {
    transport: Box<dyn RuntimeTransport>,
}

#[unsafe(no_mangle)]
pub extern "C" fn libbun_plugin_abi_version() -> u32 {
    LIBBUN_PLUGIN_ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn libbun_plugin_buffer_free(buffer: LibbunPluginBuffer) {
    if buffer.data.is_null() || buffer.len == 0 {
        return;
    }
    unsafe {
        let slice = std::ptr::slice_from_raw_parts_mut(buffer.data, buffer.len);
        drop(Box::from_raw(slice));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_create(
    config_data: *const u8,
    config_len: usize,
    runtime_out: *mut *mut c_void,
) -> LibbunPluginStatus {
    if runtime_out.is_null() {
        return error("runtime output pointer is null");
    }
    unsafe {
        *runtime_out = std::ptr::null_mut();
    }

    ffi_status(|| {
        let config: BunRuntimeConfig = read_json(config_data, config_len)?;
        let transport = transport::create(config)?;
        let runtime = Box::new(PluginRuntime { transport });
        unsafe {
            *runtime_out = Box::into_raw(runtime).cast::<c_void>();
        }
        Ok(serde_json::Value::Null)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_destroy(runtime: *mut c_void) {
    if runtime.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(runtime.cast::<PluginRuntime>()));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_load_module(
    runtime: *mut c_void,
    spec_data: *const u8,
    spec_len: usize,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let spec: BunModuleSpec = read_json(spec_data, spec_len)?;
        runtime_mut(runtime)?.transport.load_module(spec)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_call_export(
    runtime: *mut c_void,
    module_id_data: *const u8,
    module_id_len: usize,
    export_data: *const u8,
    export_len: usize,
    input_data: *const u8,
    input_len: usize,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let module_id = read_string(module_id_data, module_id_len)?;
        let export = read_string(export_data, export_len)?;
        let input: StructuralValue = read_json(input_data, input_len)?;
        runtime_mut(runtime)?.transport.call_export(
            &BunModuleHandle { id: module_id },
            &export,
            input,
        )
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_pump_event_loop(
    runtime: *mut c_void,
    max_ticks: u32,
) -> LibbunPluginStatus {
    ffi_status(|| {
        runtime_mut(runtime)?
            .transport
            .pump_event_loop(PumpBudget { max_ticks })
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_resolve_async(
    runtime: *mut c_void,
    handle_data: *const u8,
    handle_len: usize,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let id = read_string(handle_data, handle_len)?;
        runtime_mut(runtime)?
            .transport
            .resolve_async(&BunAsyncHandle { id })
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_drain_output(
    runtime: *mut c_void,
) -> LibbunPluginStatus {
    ffi_status(|| runtime_mut(runtime)?.transport.drain_output())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_shutdown(
    runtime: *mut c_void,
) -> LibbunPluginStatus {
    ffi_status(|| {
        runtime_mut(runtime)?.transport.shutdown()?;
        Ok(serde_json::Value::Null)
    })
}

fn ffi_status<T>(operation: impl FnOnce() -> libbun::LibbunResult<T>) -> LibbunPluginStatus
where
    T: serde::Serialize,
{
    match std::panic::catch_unwind(AssertUnwindSafe(operation)) {
        Ok(Ok(value)) => match serde_json::to_vec(&value) {
            Ok(bytes) => LibbunPluginStatus::ok(buffer(bytes)),
            Err(err) => error(format!("plugin response encode failed: {err}")),
        },
        Ok(Err(err)) => error(err.to_string()),
        Err(_) => error("plugin operation panicked"),
    }
}

fn error(message: impl AsRef<str>) -> LibbunPluginStatus {
    LibbunPluginStatus::error(buffer(message.as_ref().as_bytes().to_vec()))
}

fn buffer(bytes: Vec<u8>) -> LibbunPluginBuffer {
    let mut bytes = bytes.into_boxed_slice();
    let out = LibbunPluginBuffer {
        data: bytes.as_mut_ptr(),
        len: bytes.len(),
    };
    std::mem::forget(bytes);
    out
}

fn runtime_mut<'a>(runtime: *mut c_void) -> libbun::LibbunResult<&'a mut PluginRuntime> {
    if runtime.is_null() {
        return Err(LibbunError::export_call("runtime handle is null"));
    }
    Ok(unsafe { &mut *runtime.cast::<PluginRuntime>() })
}

fn read_json<T: serde::de::DeserializeOwned>(
    data: *const u8,
    len: usize,
) -> libbun::LibbunResult<T> {
    serde_json::from_slice(read_bytes(data, len)?).map_err(|err| {
        LibbunError::export_call(format!("plugin request JSON decode failed: {err}"))
    })
}

fn read_string(data: *const u8, len: usize) -> libbun::LibbunResult<String> {
    std::str::from_utf8(read_bytes(data, len)?)
        .map(|text| text.to_string())
        .map_err(|err| {
            LibbunError::export_call(format!("plugin request string decode failed: {err}"))
        })
}

fn read_bytes<'a>(data: *const u8, len: usize) -> libbun::LibbunResult<&'a [u8]> {
    if data.is_null() && len != 0 {
        return Err(LibbunError::export_call("plugin request pointer is null"));
    }
    Ok(if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(data, len) }
    })
}

#[cfg(not(target_os = "linux"))]
mod transport {
    use libbun::BunHost;
    use libbun::{
        BunAsyncHandle, BunModuleHandle, BunModuleSpec, BunRuntimeConfig, ExportCallResult,
        OutputRecord, ProviderCallResult, PumpBudget, PumpOutcome, StructuralValue,
    };
    use libbun_native::NativeBunRuntime;

    use crate::RuntimeTransport;

    pub fn create(config: BunRuntimeConfig) -> libbun::LibbunResult<Box<dyn RuntimeTransport>> {
        Ok(Box::new(InProcessTransport {
            host: BunHost::<NativeBunRuntime>::initialize(config)?,
        }))
    }

    struct InProcessTransport {
        host: BunHost<NativeBunRuntime>,
    }

    impl RuntimeTransport for InProcessTransport {
        fn load_module(&mut self, spec: BunModuleSpec) -> libbun::LibbunResult<BunModuleHandle> {
            self.host.load_module(spec)
        }

        fn call_export(
            &mut self,
            module: &BunModuleHandle,
            export: &str,
            input: StructuralValue,
        ) -> libbun::LibbunResult<ExportCallResult> {
            self.host.call_export(module, export, input)
        }

        fn pump_event_loop(&mut self, budget: PumpBudget) -> libbun::LibbunResult<PumpOutcome> {
            self.host.pump_event_loop(budget)
        }

        fn resolve_async(
            &mut self,
            handle: &BunAsyncHandle,
        ) -> libbun::LibbunResult<Option<ProviderCallResult>> {
            self.host.resolve_async(handle)
        }

        fn drain_output(&mut self) -> libbun::LibbunResult<Vec<OutputRecord>> {
            Ok(self.host.drain_captured_output())
        }

        fn shutdown(&mut self) -> libbun::LibbunResult<()> {
            self.host.shutdown()
        }
    }
}

#[cfg(target_os = "linux")]
mod transport {
    use std::ffi::c_void;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::process::Child;
    use std::process::ChildStdin;
    use std::process::ChildStdout;
    use std::process::Command;
    use std::process::Stdio;

    use libbun::helper_protocol::HelperHello;
    use libbun::helper_protocol::HelperRequest;
    use libbun::helper_protocol::HelperRequestPayload;
    use libbun::helper_protocol::HelperResponse;
    use libbun::helper_protocol::HelperResponsePayload;
    use libbun::helper_protocol::LIBBUN_HELPER_PROTOCOL_VERSION;
    use libbun::helper_protocol::LIBBUN_RUNTIME_NATIVE_PATH_ENV;
    use libbun::helper_protocol::read_frame;
    use libbun::helper_protocol::write_frame;
    use libbun::plugin_abi::LIBBUN_PLUGIN_ABI_VERSION;
    use libbun::{
        BunAsyncHandle, BunModuleHandle, BunModuleSpec, BunRuntimeConfig, ExportCallResult,
        LibbunError, OutputRecord, ProviderCallResult, PumpBudget, PumpOutcome, StructuralValue,
    };

    use crate::RuntimeTransport;

    const HELPER_BINARY_NAME: &str = "libbun-runtime-native";

    pub fn create(config: BunRuntimeConfig) -> libbun::LibbunResult<Box<dyn RuntimeTransport>> {
        Ok(Box::new(HelperTransport::start(config)?))
    }

    struct HelperTransport {
        child: Child,
        stdin: ChildStdin,
        stdout: BufReader<ChildStdout>,
        next_id: u64,
        shutdown: bool,
    }

    impl HelperTransport {
        fn start(config: BunRuntimeConfig) -> libbun::LibbunResult<Self> {
            let helper_path = helper_path()?;
            let mut child = Command::new(&helper_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()
                .map_err(|err| {
                    LibbunError::initialize(format!(
                        "failed to start Linux native helper at {}: {err}",
                        helper_path.display()
                    ))
                })?;
            let stdin = child.stdin.take().ok_or_else(|| {
                LibbunError::initialize("Linux native helper stdin pipe was not available")
            })?;
            let stdout = child.stdout.take().ok_or_else(|| {
                LibbunError::initialize("Linux native helper stdout pipe was not available")
            })?;

            let mut transport = Self {
                child,
                stdin,
                stdout: BufReader::new(stdout),
                next_id: 1,
                shutdown: false,
            };
            transport.handshake()?;
            transport.request_unit(
                HelperRequestPayload::Create { config },
                LibbunError::initialize,
            )?;
            Ok(transport)
        }

        fn handshake(&mut self) -> libbun::LibbunResult<()> {
            let response = self.request(
                HelperRequestPayload::Hello(HelperHello::current(std::env::consts::ARCH)),
                LibbunError::initialize,
            )?;
            let HelperResponsePayload::Hello(hello) = response else {
                return Err(LibbunError::initialize(
                    "Linux native helper returned an unexpected handshake response",
                ));
            };
            if hello.plugin_abi_version != LIBBUN_PLUGIN_ABI_VERSION {
                return Err(LibbunError::initialize(format!(
                    "Linux native helper plugin ABI {} is incompatible with plugin ABI {}",
                    hello.plugin_abi_version, LIBBUN_PLUGIN_ABI_VERSION
                )));
            }
            if hello.helper_protocol_version != LIBBUN_HELPER_PROTOCOL_VERSION {
                return Err(LibbunError::initialize(format!(
                    "Linux native helper protocol {} is incompatible with plugin protocol {}",
                    hello.helper_protocol_version, LIBBUN_HELPER_PROTOCOL_VERSION
                )));
            }
            Ok(())
        }

        fn request(
            &mut self,
            payload: HelperRequestPayload,
            map_error: fn(String) -> LibbunError,
        ) -> libbun::LibbunResult<HelperResponsePayload> {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1).max(1);
            write_frame(&mut self.stdin, &HelperRequest { id, payload }).map_err(|err| {
                map_error(format!(
                    "failed to write Linux native helper request: {err}"
                ))
            })?;

            let response: HelperResponse = read_frame(&mut self.stdout)
                .map_err(|err| {
                    LibbunError::export_call(format!(
                        "failed to read Linux native helper response: {err}"
                    ))
                })?
                .ok_or_else(|| {
                    LibbunError::export_call("Linux native helper exited before responding")
                })?;
            if response.id != id {
                return Err(LibbunError::export_call(format!(
                    "Linux native helper response id {} did not match request id {id}",
                    response.id
                )));
            }
            response.result.map_err(map_error)
        }

        fn request_unit(
            &mut self,
            payload: HelperRequestPayload,
            map_error: fn(String) -> LibbunError,
        ) -> libbun::LibbunResult<()> {
            match self.request(payload, map_error)? {
                HelperResponsePayload::Unit => Ok(()),
                _ => Err(LibbunError::export_call(
                    "Linux native helper returned an unexpected unit response",
                )),
            }
        }
    }

    impl RuntimeTransport for HelperTransport {
        fn load_module(&mut self, spec: BunModuleSpec) -> libbun::LibbunResult<BunModuleHandle> {
            match self.request(
                HelperRequestPayload::LoadModule { spec },
                LibbunError::module_load,
            )? {
                HelperResponsePayload::Module(module) => Ok(module),
                _ => Err(LibbunError::module_load(
                    "Linux native helper returned an unexpected module response",
                )),
            }
        }

        fn call_export(
            &mut self,
            module: &BunModuleHandle,
            export: &str,
            input: StructuralValue,
        ) -> libbun::LibbunResult<ExportCallResult> {
            match self.request(
                HelperRequestPayload::CallExport {
                    module: module.clone(),
                    export: export.to_string(),
                    input,
                },
                LibbunError::export_call,
            )? {
                HelperResponsePayload::Export(result) => Ok(result),
                _ => Err(LibbunError::export_call(
                    "Linux native helper returned an unexpected export response",
                )),
            }
        }

        fn pump_event_loop(&mut self, budget: PumpBudget) -> libbun::LibbunResult<PumpOutcome> {
            match self.request(
                HelperRequestPayload::PumpEventLoop { budget },
                LibbunError::event_loop_pump,
            )? {
                HelperResponsePayload::Pump(outcome) => Ok(outcome),
                _ => Err(LibbunError::event_loop_pump(
                    "Linux native helper returned an unexpected pump response",
                )),
            }
        }

        fn resolve_async(
            &mut self,
            handle: &BunAsyncHandle,
        ) -> libbun::LibbunResult<Option<ProviderCallResult>> {
            match self.request(
                HelperRequestPayload::ResolveAsync {
                    handle: handle.clone(),
                },
                LibbunError::export_call,
            )? {
                HelperResponsePayload::Resolve(result) => Ok(result),
                _ => Err(LibbunError::export_call(
                    "Linux native helper returned an unexpected async response",
                )),
            }
        }

        fn drain_output(&mut self) -> libbun::LibbunResult<Vec<OutputRecord>> {
            match self.request(HelperRequestPayload::DrainOutput, LibbunError::export_call)? {
                HelperResponsePayload::Output(records) => Ok(records),
                _ => Err(LibbunError::export_call(
                    "Linux native helper returned an unexpected output response",
                )),
            }
        }

        fn shutdown(&mut self) -> libbun::LibbunResult<()> {
            if self.shutdown {
                return Ok(());
            }
            self.request_unit(HelperRequestPayload::Shutdown, LibbunError::shutdown)?;
            self.shutdown = true;
            Ok(())
        }
    }

    impl Drop for HelperTransport {
        fn drop(&mut self) {
            if !self.shutdown {
                let _ = self.request_unit(HelperRequestPayload::Shutdown, LibbunError::shutdown);
            }
            let _ = self.request_unit(HelperRequestPayload::Exit, LibbunError::shutdown);
            let _ = self.child.wait();
        }
    }

    fn helper_path() -> libbun::LibbunResult<PathBuf> {
        if let Some(path) = std::env::var_os(LIBBUN_RUNTIME_NATIVE_PATH_ENV).map(PathBuf::from) {
            return Ok(path);
        }
        if let Some(plugin_path) = loaded_plugin_path() {
            if let Some(parent) = plugin_path.parent() {
                return Ok(parent.join(HELPER_BINARY_NAME));
            }
        }
        let current_exe = std::env::current_exe().map_err(|err| {
            LibbunError::initialize(format!("failed to inspect current executable path: {err}"))
        })?;
        Ok(current_exe
            .parent()
            .unwrap_or_else(|| current_exe.as_path())
            .join(HELPER_BINARY_NAME))
    }

    fn loaded_plugin_path() -> Option<PathBuf> {
        unsafe {
            let mut info: libc::Dl_info = std::mem::zeroed();
            let symbol = super::libbun_plugin_abi_version as *const () as *const c_void;
            if libc::dladdr(symbol, &mut info) == 0 || info.dli_fname.is_null() {
                return None;
            }
            std::ffi::CStr::from_ptr(info.dli_fname)
                .to_str()
                .ok()
                .map(PathBuf::from)
        }
    }
}
