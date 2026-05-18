use std::ffi::c_void;
use std::panic::AssertUnwindSafe;

use libbun::plugin_abi::{LIBBUN_PLUGIN_ABI_VERSION, LibbunPluginBuffer, LibbunPluginStatus};
use libbun::{
    BunAsyncHandle, BunHost, BunModuleSpec, BunRuntimeConfig, LibbunError, OutputRecord,
    PumpBudget, StructuralValue,
};
use libbun_native::NativeBunRuntime;

struct PluginRuntime {
    host: BunHost<NativeBunRuntime>,
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
        let host = BunHost::<NativeBunRuntime>::initialize(config)?;
        let runtime = Box::new(PluginRuntime { host });
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
        let runtime = runtime_mut(runtime)?;
        runtime.host.load_module(spec)
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
        let runtime = runtime_mut(runtime)?;
        runtime
            .host
            .call_export(&libbun::BunModuleHandle { id: module_id }, &export, input)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_pump_event_loop(
    runtime: *mut c_void,
    max_ticks: u32,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let runtime = runtime_mut(runtime)?;
        runtime.host.pump_event_loop(PumpBudget { max_ticks })
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
        let runtime = runtime_mut(runtime)?;
        runtime.host.resolve_async(&BunAsyncHandle { id })
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_drain_output(
    runtime: *mut c_void,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let runtime = runtime_mut(runtime)?;
        let records: Vec<OutputRecord> = runtime.host.drain_captured_output();
        Ok(records)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbun_plugin_runtime_shutdown(
    runtime: *mut c_void,
) -> LibbunPluginStatus {
    ffi_status(|| {
        let runtime = runtime_mut(runtime)?;
        runtime.host.shutdown()?;
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
