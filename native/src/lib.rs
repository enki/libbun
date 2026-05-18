//! Native Bun adapter for the stable `libbun` facade.
//!
//! This crate is intentionally separate from the stable facade crate because
//! upstream Bun currently requires its pinned nightly toolchain and generated
//! codegen inputs.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::ptr::NonNull;

use bun_core::String as BunString;
use bun_jsc::js_promise::Status as PromiseStatus;
use bun_jsc::virtual_machine::{InitOptions, VirtualMachine};
use bun_jsc::{AnyPromise, JSInternalPromise, JSModuleLoader, JSValue, JSPromise};
use libbun::{
    BunAsyncHandle, BunEmbeddingRuntime, BunModuleHandle, BunModuleSpec, BunRuntimeConfig,
    ExportCallResult, LibbunError, LibbunResult, OutputRecord, ProviderCallResult, ProviderError,
    PumpBudget, PumpOutcome, StructuralValue,
};

#[derive(Debug)]
pub struct NativeBunRuntime {
    vm: NonNull<VirtualMachine>,
    modules: BTreeMap<String, JSValue>,
    pending: BTreeMap<String, JSValue>,
    output: Vec<OutputRecord>,
    tempdir: tempfile::TempDir,
    next_module: u64,
    next_async: u64,
    shutdown: bool,
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
        let json_literal = serde_json::to_string(&json).map_err(|err| {
            LibbunError::export_call(format!("input JSON literal encode failed: {err}"))
        })?;
        self.evaluate_expression(&format!("JSON.parse({json_literal})"))
    }

    fn evaluate_expression(&self, source: &str) -> LibbunResult<JSValue> {
        let global = self.vm().global();
        let mut exception = JSValue::ZERO;
        let value = JSModuleLoader::evaluate(
            global,
            source.as_ptr(),
            source.len(),
            b"libbun://eval".as_ptr(),
            b"libbun://eval".len(),
            b"libbun://host".as_ptr(),
            b"libbun://host".len(),
            JSValue::UNDEFINED,
            &mut exception,
        );
        if !exception.is_empty() {
            return Err(LibbunError::export_call(
                self.js_error_to_string(exception, "JavaScript evaluation failed"),
            ));
        }
        Ok(value)
    }

    fn value_to_result(&self, value: JSValue) -> LibbunResult<ProviderCallResult> {
        if value.is_undefined() || value.is_null() {
            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
        }

        let mut out = BunString::empty();
        value
            .json_stringify_fast(self.vm().global(), &mut out)
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
        ProviderCallResult::Err(ProviderError {
            code: "provider_rejected".to_string(),
            message: self.js_error_to_string(value, "provider promise rejected"),
        })
    }

    fn js_error_to_string(&self, value: JSValue, fallback: &str) -> String {
        match value.to_slice_or_null(self.vm().global()) {
            Ok(text) => {
                let bytes = text.into_vec();
                String::from_utf8_lossy(&bytes).into_owned()
            }
            _ => fallback.to_string(),
        }
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
}

impl BunEmbeddingRuntime for NativeBunRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        bun_jsc::initialize(false);
        bun_ast::initialize_store();

        let vm = VirtualMachine::init(InitOptions {
            is_main_thread: true,
            mini_mode: false,
            ..Default::default()
        })
        .map_err(|err| LibbunError::initialize(format!("{err:?}")))?;

        let tempdir = tempfile::Builder::new()
            .prefix("libbun-native-")
            .tempdir_in(&config.working_directory)
            .map_err(|err| LibbunError::initialize(format!("tempdir create failed: {err}")))?;

        Ok(Self {
            vm: NonNull::new(vm).ok_or_else(|| LibbunError::initialize("Bun VM init returned null"))?,
            modules: BTreeMap::new(),
            pending: BTreeMap::new(),
            output: Vec::new(),
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

        let module_path = match spec {
            BunModuleSpec::Source { module_id, source } => {
                let path = self.tempdir.path().join(format!("{module_id}.mjs"));
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

        let id = format!("module-{}", self.next_module);
        self.next_module += 1;

        let specifier = path_to_file_specifier(module_path)?;
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
                namespace.protect();
                self.modules.insert(id.clone(), namespace);
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
        let result = function
            .call(self.vm().global(), namespace, &[arg])
            .map_err(|_| LibbunError::export_call(format!("export `{export}` threw")))?;

        if let Some(_promise) = result.as_internal_promise() {
            let id = format!("async-{}", self.next_async);
            self.next_async += 1;
            result.protect();
            self.pending.insert(id.clone(), result);
            return Ok(ExportCallResult::Pending(BunAsyncHandle { id }));
        }

        Ok(ExportCallResult::Ready(self.value_to_result(result)?))
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
        let promise = value
            .as_internal_promise()
            .ok_or_else(|| LibbunError::UnknownAsyncHandle {
                handle: handle.id.clone(),
            })?;
        let result = self.promise_result(promise)?;
        if result.is_some() {
            value.unprotect();
            self.pending.remove(&handle.id);
        }
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
            value.unprotect();
        }
        for (_, value) in std::mem::take(&mut self.modules) {
            value.unprotect();
        }
        self.vm_mut().destroy();
        self.shutdown = true;
        Ok(())
    }
}

fn path_to_file_specifier(path: PathBuf) -> LibbunResult<String> {
    let path = path
        .canonicalize()
        .map_err(|err| LibbunError::module_load(format!("canonicalize failed: {err}")))?;
    Ok(format!("file://{}", path.to_string_lossy()))
}
