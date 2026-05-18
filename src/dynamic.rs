use std::ffi::c_void;
use std::path::Path;
use std::path::PathBuf;

use libloading::Library;
use serde::de::DeserializeOwned;

use crate::plugin_abi::{
    LIBBUN_PLUGIN_ABI_VERSION, LIBBUN_PLUGIN_STATUS_ERROR, LIBBUN_PLUGIN_STATUS_OK,
    LibbunPluginBuffer, LibbunPluginStatus,
};
use crate::{
    BunAsyncHandle, BunEmbeddingRuntime, BunModuleHandle, BunModuleSpec, BunRuntimeConfig,
    ExportCallResult, LibbunError, LibbunResult, OutputRecord, ProviderCallResult, PumpBudget,
    PumpOutcome, StructuralValue,
};

const LIBBUN_PLUGIN_PATH_ENV: &str = "LIBBUN_PLUGIN_PATH";

type PluginAbiVersionFn = unsafe extern "C" fn() -> u32;
type PluginBufferFreeFn = unsafe extern "C" fn(LibbunPluginBuffer);
type RuntimeCreateFn =
    unsafe extern "C" fn(*const u8, usize, *mut *mut c_void) -> LibbunPluginStatus;
type RuntimeDestroyFn = unsafe extern "C" fn(*mut c_void);
type RuntimeLoadModuleFn =
    unsafe extern "C" fn(*mut c_void, *const u8, usize) -> LibbunPluginStatus;
type RuntimeCallExportFn = unsafe extern "C" fn(
    *mut c_void,
    *const u8,
    usize,
    *const u8,
    usize,
    *const u8,
    usize,
) -> LibbunPluginStatus;
type RuntimePumpEventLoopFn = unsafe extern "C" fn(*mut c_void, u32) -> LibbunPluginStatus;
type RuntimeResolveAsyncFn =
    unsafe extern "C" fn(*mut c_void, *const u8, usize) -> LibbunPluginStatus;
type RuntimeDrainOutputFn = unsafe extern "C" fn(*mut c_void) -> LibbunPluginStatus;
type RuntimeShutdownFn = unsafe extern "C" fn(*mut c_void) -> LibbunPluginStatus;

#[derive(Debug)]
pub struct DynamicBunRuntime {
    plugin: DynamicPlugin,
    runtime: *mut c_void,
    output: Vec<OutputRecord>,
    shutdown: bool,
}

#[derive(Debug)]
struct DynamicPlugin {
    _library: Library,
    buffer_free: PluginBufferFreeFn,
    runtime_create: RuntimeCreateFn,
    runtime_destroy: RuntimeDestroyFn,
    runtime_load_module: RuntimeLoadModuleFn,
    runtime_call_export: RuntimeCallExportFn,
    runtime_pump_event_loop: RuntimePumpEventLoopFn,
    runtime_resolve_async: RuntimeResolveAsyncFn,
    runtime_drain_output: RuntimeDrainOutputFn,
    runtime_shutdown: RuntimeShutdownFn,
}

impl DynamicBunRuntime {
    pub fn load(plugin_path: impl AsRef<Path>, config: BunRuntimeConfig) -> LibbunResult<Self> {
        let plugin = DynamicPlugin::load(plugin_path.as_ref())?;
        let config = serde_json::to_vec(&config).map_err(|err| {
            LibbunError::initialize(format!("dynamic plugin config encode failed: {err}"))
        })?;
        let mut runtime = std::ptr::null_mut();
        let status =
            unsafe { (plugin.runtime_create)(config.as_ptr(), config.len(), &mut runtime) };
        plugin.status_unit(status, LibbunError::initialize)?;
        if runtime.is_null() {
            return Err(LibbunError::initialize(
                "dynamic plugin returned a null runtime handle",
            ));
        }

        let mut runtime = Self {
            plugin,
            runtime,
            output: Vec::new(),
            shutdown: false,
        };
        runtime.collect_output()?;
        Ok(runtime)
    }

    fn collect_output(&mut self) -> LibbunResult<()> {
        let status = unsafe { (self.plugin.runtime_drain_output)(self.runtime) };
        let mut records: Vec<OutputRecord> =
            self.plugin.status_json(status, LibbunError::export_call)?;
        self.output.append(&mut records);
        Ok(())
    }
}

impl BunEmbeddingRuntime for DynamicBunRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let plugin_path = std::env::var_os(LIBBUN_PLUGIN_PATH_ENV)
            .map(PathBuf::from)
            .ok_or_else(|| {
                LibbunError::initialize(format!("{LIBBUN_PLUGIN_PATH_ENV} is not set"))
            })?;
        Self::load(plugin_path, config)
    }

    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        let spec = serde_json::to_vec(&spec)
            .map_err(|err| LibbunError::module_load(format!("module spec encode failed: {err}")))?;
        let status =
            unsafe { (self.plugin.runtime_load_module)(self.runtime, spec.as_ptr(), spec.len()) };
        let result = self.plugin.status_json(status, LibbunError::module_load);
        self.collect_output()?;
        result
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
        let input = serde_json::to_vec(&input)
            .map_err(|err| LibbunError::export_call(format!("input encode failed: {err}")))?;
        let status = unsafe {
            (self.plugin.runtime_call_export)(
                self.runtime,
                module.id.as_ptr(),
                module.id.len(),
                export.as_ptr(),
                export.len(),
                input.as_ptr(),
                input.len(),
            )
        };
        let result = self.plugin.status_json(status, LibbunError::export_call);
        self.collect_output()?;
        result
    }

    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        let status =
            unsafe { (self.plugin.runtime_pump_event_loop)(self.runtime, budget.max_ticks) };
        let result = self
            .plugin
            .status_json(status, LibbunError::event_loop_pump);
        self.collect_output()?;
        result
    }

    fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        let status = unsafe {
            (self.plugin.runtime_resolve_async)(self.runtime, handle.id.as_ptr(), handle.id.len())
        };
        let result = self.plugin.status_json(status, LibbunError::export_call);
        self.collect_output()?;
        result
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
        let status = unsafe { (self.plugin.runtime_shutdown)(self.runtime) };
        self.plugin.status_unit(status, LibbunError::shutdown)?;
        self.collect_output()?;
        self.shutdown = true;
        Ok(())
    }
}

impl Drop for DynamicBunRuntime {
    fn drop(&mut self) {
        if !self.runtime.is_null() {
            if !self.shutdown {
                let _ = unsafe { (self.plugin.runtime_shutdown)(self.runtime) };
            }
            unsafe { (self.plugin.runtime_destroy)(self.runtime) };
            self.runtime = std::ptr::null_mut();
        }
    }
}

impl DynamicPlugin {
    fn load(path: &Path) -> LibbunResult<Self> {
        let library = unsafe { Library::new(path) }.map_err(|err| {
            LibbunError::initialize(format!(
                "dynamic plugin load failed at {}: {err}",
                path.display()
            ))
        })?;

        let abi_version: PluginAbiVersionFn = unsafe { library.get(b"libbun_plugin_abi_version") }
            .map(|symbol| *symbol)
            .map_err(|err| LibbunError::initialize(format!("plugin ABI symbol missing: {err}")))?;
        let reported = unsafe { abi_version() };
        if reported != LIBBUN_PLUGIN_ABI_VERSION {
            return Err(LibbunError::initialize(format!(
                "dynamic plugin ABI version {reported} is incompatible with host ABI {LIBBUN_PLUGIN_ABI_VERSION}"
            )));
        }

        let buffer_free = load_symbol(&library, b"libbun_plugin_buffer_free")?;
        let runtime_create = load_symbol(&library, b"libbun_plugin_runtime_create")?;
        let runtime_destroy = load_symbol(&library, b"libbun_plugin_runtime_destroy")?;
        let runtime_load_module = load_symbol(&library, b"libbun_plugin_runtime_load_module")?;
        let runtime_call_export = load_symbol(&library, b"libbun_plugin_runtime_call_export")?;
        let runtime_pump_event_loop =
            load_symbol(&library, b"libbun_plugin_runtime_pump_event_loop")?;
        let runtime_resolve_async = load_symbol(&library, b"libbun_plugin_runtime_resolve_async")?;
        let runtime_drain_output = load_symbol(&library, b"libbun_plugin_runtime_drain_output")?;
        let runtime_shutdown = load_symbol(&library, b"libbun_plugin_runtime_shutdown")?;

        Ok(Self {
            _library: library,
            buffer_free,
            runtime_create,
            runtime_destroy,
            runtime_load_module,
            runtime_call_export,
            runtime_pump_event_loop,
            runtime_resolve_async,
            runtime_drain_output,
            runtime_shutdown,
        })
    }

    fn status_unit(
        &self,
        status: LibbunPluginStatus,
        error: impl FnOnce(String) -> LibbunError,
    ) -> LibbunResult<()> {
        self.status_json::<serde_json::Value>(status, error)
            .map(|_| ())
    }

    fn status_json<T: DeserializeOwned>(
        &self,
        status: LibbunPluginStatus,
        error: impl FnOnce(String) -> LibbunError,
    ) -> LibbunResult<T> {
        let payload = self.take_payload(status.payload);
        match status.code {
            LIBBUN_PLUGIN_STATUS_OK => serde_json::from_slice(&payload)
                .map_err(|err| error(format!("dynamic plugin response decode failed: {err}"))),
            LIBBUN_PLUGIN_STATUS_ERROR => {
                Err(error(String::from_utf8_lossy(&payload).into_owned()))
            }
            other => Err(error(format!(
                "dynamic plugin returned unknown status {other}"
            ))),
        }
    }

    fn take_payload(&self, buffer: LibbunPluginBuffer) -> Vec<u8> {
        if buffer.data.is_null() || buffer.len == 0 {
            return Vec::new();
        }
        let bytes = unsafe { std::slice::from_raw_parts(buffer.data, buffer.len) }.to_vec();
        unsafe { (self.buffer_free)(buffer) };
        bytes
    }
}

fn load_symbol<T: Copy>(library: &Library, symbol: &[u8]) -> LibbunResult<T> {
    unsafe { library.get::<T>(symbol) }
        .map(|symbol| *symbol)
        .map_err(|err| {
            LibbunError::initialize(format!(
                "dynamic plugin symbol `{}` missing: {err}",
                String::from_utf8_lossy(symbol)
            ))
        })
}
