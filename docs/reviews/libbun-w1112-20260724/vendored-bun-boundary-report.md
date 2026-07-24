# libbun W1-11/W1-12 vendored Bun boundary report

This report freezes the exact candidate hashes, patches, and narrow integration excerpts needed to review the native boundary without attaching the entire vendored Bun snapshot. Every excerpt was read with git show from candidate 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb.

## Reproduction output

Candidate SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb
Candidate tree: cb964de8ab8162449fbe95959bf34d231570aa5c

[tracked boundary file hashes]
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  native/build.rs
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  runtime/build.rs
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  scripts/prepare-native-bun-link.sh
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  BUN_SOURCE_COMMIT
awk: cmd. line:1: {print \}
awk: cmd. line:1:        ^ backslash not last character on line
  vendor/bun/src/jsc/bindings/bindings.cpp

[libbun patch bodies]
diff --git a/scripts/build/deps/mimalloc.ts b/scripts/build/deps/mimalloc.ts
index 4683c79..5c3a9f9 100644
--- a/scripts/build/deps/mimalloc.ts
+++ b/scripts/build/deps/mimalloc.ts
@@ -12,6 +12,8 @@

 import type { Dependency, DirectBuild } from "../source.ts";

+const libbunNativePluginPic = process.env.LIBBUN_NATIVE_PLUGIN_PIC === "1";
+
 const MIMALLOC_COMMIT = "f15aecb94fc8096008bf87b90c53ed682026914a";

 export const mimalloc: Dependency = {
@@ -106,7 +108,9 @@ export const mimalloc: Dependency = {
     // addons because musl's static TLS block is fixed-size. ELF/Mach-O
     // only — clang-cl doesn't recognize -ftls-model (COFF has no TLS
     // models; mimalloc's cmake gates it behind NOT WIN32 too).
-    if (!cfg.windows) {
+    if (libbunNativePluginPic) {
+      cflags.push("-ftls-model=local-dynamic");
+    } else if (!cfg.windows) {
       cflags.push(cfg.abi === "musl" ? "-ftls-model=local-dynamic" : "-ftls-model=initial-exec");
     }

diff --git a/scripts/build/deps/webkit.ts b/scripts/build/deps/webkit.ts
index c3978fa..e7dbef3 100644
--- a/scripts/build/deps/webkit.ts
+++ b/scripts/build/deps/webkit.ts
@@ -46,6 +46,8 @@ import { computeCpuTargetFlags } from "../flags.ts";
 import { slash } from "../shell.ts";
 import { type Dependency, type NestedCmakeBuild, type Source, depBuildDir, depSourceDir } from "../source.ts";

+const libbunNativePluginPic = process.env.LIBBUN_NATIVE_PLUGIN_PIC === "1";
+
 // ───────────────────────────────────────────────────────────────────────────
 // Prebuilt URL computation
 // ───────────────────────────────────────────────────────────────────────────
@@ -235,7 +237,13 @@ export const webkit: Dependency = {
     // -no-pie rides along in CMAKE_C_FLAGS so try_compile() probes link on
     // PIE-default distros — without it the driver still passes -pie and the
     // -fno-pic probe object fails R_X86_64_32S relocation, killing FindThreads.
-    if (cfg.unix && cfg.abi !== "android") optFlags.push("-fno-pic", "-fno-pie", "-no-pie");
+    if (cfg.unix && cfg.abi !== "android") {
+      if (libbunNativePluginPic) {
+        optFlags.push("-fPIC");
+      } else {
+        optFlags.push("-fno-pic", "-fno-pie", "-no-pie");
+      }
+    }
     if (cfg.lto) optFlags.push("-flto=full");
     if (cfg.pgoGenerate) optFlags.push(`-fprofile-generate=${cfg.pgoGenerate}`);
     if (cfg.pgoUse) {
@@ -308,7 +316,9 @@ export const webkit: Dependency = {
       // .data.rel.ro. We link -no-pie so this is dead weight in the RW
       // PT_LOAD. Android (PIE) overrides via the -fPIC in optFlags above
       // never being suppressed there.
-      ...(cfg.abi !== "android" ? { CMAKE_POSITION_INDEPENDENT_CODE: "OFF" } : {}),
+      ...(cfg.abi !== "android"
+        ? { CMAKE_POSITION_INDEPENDENT_CODE: libbunNativePluginPic ? "ON" : "OFF" }
+        : {}),
       PORT: "JSCOnly",
       ENABLE_STATIC_JSC: "ON",
       USE_THIN_ARCHIVES: "OFF",
diff --git a/scripts/build/flags.ts b/scripts/build/flags.ts
index 443a60d..5ae0090 100644
--- a/scripts/build/flags.ts
+++ b/scripts/build/flags.ts
@@ -15,6 +15,8 @@ import { join } from "node:path";
 import { bunExeName, type Config } from "./config.ts";
 import { slash } from "./shell.ts";

+const libbunNativePluginPic = process.env.LIBBUN_NATIVE_PLUGIN_PIC === "1";
+
 export type FlagValue = string | string[] | ((cfg: Config) => string | string[]);

 export interface Flag {
@@ -531,9 +533,14 @@ export const bunOnlyFlags: Flag[] = [
   },
   {
     flag: ["-fno-pic", "-fno-pie"],
-    when: c => c.unix && c.abi !== "android",
+    when: c => c.unix && c.abi !== "android" && !libbunNativePluginPic,
     desc: "No position-independent code (we're a final executable)",
   },
+  {
+    flag: "-fPIC",
+    when: c => c.unix && c.abi !== "android" && libbunNativePluginPic,
+    desc: "Position-independent code for libbun native plugin shared-object inputs",
+  },
   {
     flag: "-fPIC",
     when: c => c.abi === "android",
@@ -941,7 +948,7 @@ export const linkerFlags: Flag[] = [
   },
   {
     flag: ["-fno-pic", "-Wl,-no-pie"],
-    when: c => c.linux && c.abi !== "android",
+    when: c => c.linux && c.abi !== "android" && !libbunNativePluginPic,
     desc: "No PIE (we don't need ASLR; simpler codegen)",
   },
   {
diff --git a/scripts/build/source.ts b/scripts/build/source.ts
index 70707a8..b6f2c57 100644
--- a/scripts/build/source.ts
+++ b/scripts/build/source.ts
@@ -31,6 +31,8 @@ import type { Ninja } from "./ninja.ts";
 import { quote, quoteArgs, slash } from "./shell.ts";
 import { streamPath } from "./stream.ts";

+const libbunNativePluginPic = process.env.LIBBUN_NATIVE_PLUGIN_PIC === "1";
+
 /**
  * If the source dir exists with a stale (or missing) identity stamp,
  * delete it. Called at configure time so ninja's startup stat sees the
@@ -1201,7 +1203,8 @@ function emitNestedCmake(
   //
   // Windows has no PIC concept (all code is relocatable), so both branches
   // are guarded — no-op there.
-  if (spec.pic) {
+  const usePic = spec.pic || (libbunNativePluginPic && cfg.unix && cfg.abi !== "android");
+  if (usePic) {
     if (!cfg.windows) {
       cflags += " -fPIC";
       cxxflags += " -fPIC";
@@ -1489,7 +1492,8 @@ function emitDirect(
   // codegen as cmake deps would. spec.pic → -fPIC; otherwise on darwin
   // undo apple-clang's PIC default to match the non-PIE final binary.
   const picFlags: string[] = [];
-  if (spec.pic) {
+  const usePic = spec.pic || (libbunNativePluginPic && cfg.unix && cfg.abi !== "android");
+  if (usePic) {
     if (!cfg.windows) picFlags.push("-fPIC");
   } else if (cfg.darwin) {
     picFlags.push("-fno-pic", "-fno-pie");
diff --git a/src/jsc/bindings/bindings.cpp b/src/jsc/bindings/bindings.cpp
index 0000000..0000000 100644
--- a/src/jsc/bindings/bindings.cpp
+++ b/src/jsc/bindings/bindings.cpp
@@ -6284,8 +6284,15 @@ extern "C" void Bun__JSValue__protect(JSC::EncodedJSValue encodedValue)
 #if ASSERT_ENABLED
 CPP_DECL const char* Bun__CallFrame__describeFrame(JSC::CallFrame* callFrame)
 {
     return callFrame->describeFrame();
 }
+#else
+// LIBBUN_RELEASE_CALLFRAME_DESCRIBE_SYMBOL: bun_jsc references this C ABI
+// symbol in release builds, where JSC::CallFrame::describeFrame is assert-only.
+CPP_DECL const char* Bun__CallFrame__describeFrame(JSC::CallFrame*)
+{
+    return "<CallFrame::describeFrame unavailable in release build>";
+}
 #endif

 extern "C" double Bun__JSC__operationMathPow(double x, double y)

[patched call-frame definitions and references]
  4340
  4341	    auto& vm = JSC::getVM(globalObject);
  4342	    JSC::JSObject* object = value.getObject();
  4343	    if (!object) [[unlikely]] {
  4344	        return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4345	    }
  4346
  4347	    // Since Identifier might not ref the string, we need to ensure it doesn't get deref'd until this function returns
  4348	    const auto propertyString = String(StringImpl::createWithoutCopying({ arg1, arg2 }));
  4349	    const auto identifier = JSC::Identifier::fromString(vm, propertyString);
  4350	    const auto property = JSC::PropertyName(identifier);
  4351
  4352	    return JSC::JSValue::encode(Bun::getIfPropertyExistsPrototypePollutionMitigationUnsafe(vm, globalObject, object, property));
  4353	}
  4354
  4355	// Returns empty for exception, returns deleted if not found.
  4356	// Be careful when handling the return value.
  4357	// Can handle numeric index property names safely. If you know that the property name is not an integer index, use JSC__JSValue__getIfPropertyExistsImpl instead.
  4358	JSC::EncodedJSValue JSC__JSValue__getPropertyValue(JSC::EncodedJSValue encodedValue,
  4359	    JSC::JSGlobalObject* globalObject,
  4360	    const unsigned char* propertyName, uint32_t propertyNameLength)
  4361	{
  4362
  4363	    ASSERT_NO_PENDING_EXCEPTION(globalObject);
  4364	    JSValue value = JSC::JSValue::decode(encodedValue);
  4365	    ASSERT_WITH_MESSAGE(!value.isEmpty(), "getPropertyValue() must not be called on empty value");
  4366
  4367	    auto& vm = JSC::getVM(globalObject);
  4368	    JSC::JSObject* object = value.getObject();
  4369	    if (!object) [[unlikely]] {
  4370	        return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4371	    }
  4372
  4373	    // Since Identifier might not ref the string, we need to ensure it doesn't get deref'd until this function returns
  4374	    const auto propertyString = String(StringImpl::createWithoutCopying({ propertyName, propertyNameLength }));
  4375	    const auto identifier = JSC::Identifier::fromString(vm, propertyString);
  4376	    const auto property = JSC::PropertyName(identifier);
  4377
  4378	    auto scope = DECLARE_THROW_SCOPE(vm);
  4379	    PropertySlot slot(object, PropertySlot::InternalMethodType::Get);
  4380	    if (!object->getPropertySlot(globalObject, property, slot)) {
  4381	        RETURN_IF_EXCEPTION(scope, {});
  4382	        return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4383	    }
  4384	    RETURN_IF_EXCEPTION(scope, {});
  4385
  4386	    JSValue result = slot.getValue(globalObject, property);
  4387	    RETURN_IF_EXCEPTION(scope, {});
  4388
  4389	    return JSValue::encode(result);
  4390	}
  4391
  4392	extern "C" JSC::EncodedJSValue JSC__JSValue__getOwn(JSC::EncodedJSValue JSValue0, JSC::JSGlobalObject* globalObject, BunString* propertyName)
  4393	{
  4394	    ASSERT_NO_PENDING_EXCEPTION(globalObject);
  4395

[native integration imports and call sites]
     1	//! Native Bun engine behind the one-shot prepared-export worker.
     2	//!
     3	//! This crate is intentionally separate from the stable facade crate because
     4	//! upstream Bun currently requires its pinned nightly toolchain and generated
     5	//! codegen inputs.
     6
     7	#[cfg(not(feature = "internal-adapter"))]
     8	compile_error!(
     9	    "`libbun-native` is an internal implementation crate. Build \
    10	     `libbun-runtime-native` instead of linking it into a host."
    11	);
    12
    13	use std::collections::BTreeMap;
    14	use std::io::Read;
    15	use std::path::Component;
    16	use std::path::Path;
    17	use std::path::PathBuf;
    18	use std::ptr::NonNull;
    19	use std::sync::Mutex;
    20	use std::sync::MutexGuard;
    21	use std::sync::OnceLock;
    22	use std::sync::TryLockError;
    23
    24	use bun_core::{String as BunString, ZigString};
    25	use bun_jsc::js_promise::{Status as PromiseStatus, UnwrapMode, Unwrapped};
    26	use bun_jsc::virtual_machine::{InitOptions, VirtualMachine};
    27	use bun_jsc::{
    28	    AnyPromise, BuiltinName, JSGlobalObject, JSInternalPromise, JSModuleLoader, JSPromise, JSType,
    29	    JSValue, ZigStringJsc,
    30	};
    31	use bun_platform as _;
    32	use bun_runtime as _;
    33	use libbun_prepared_export_wire::DriveRequest;
    34	use libbun_prepared_export_wire::WorkerFaultKind;
    35	use serde::Deserialize;
    36
    37	type LibbunResult<T> = Result<T, LibbunError>;
    38
    39	#[derive(Debug, thiserror::Error)]
    40	enum LibbunError {
    41	    #[error("runtime initialization failed: {message}")]
    42	    Initialize { message: String },
    43	    #[error("module load failed: {message}")]
    44	    ModuleLoad { message: String },
    45	    #[error("export call failed: {message}")]
    46	    ExportCall { message: String },
    47	    #[error("async handle `{handle}` is unknown")]
    48	    UnknownAsyncHandle { handle: String },
    49	}
    50
    51	impl LibbunError {
    52	    fn initialize(message: impl Into<String>) -> Self {
    53	        Self::Initialize {
    54	            message: message.into(),
    55	        }
    56	    }
    57
    58	    fn module_load(message: impl Into<String>) -> Self {
    59	        Self::ModuleLoad {
    60	            message: message.into(),
    61	        }
    62	    }
    63
    64	    fn export_call(message: impl Into<String>) -> Self {
    65	        Self::ExportCall {
    66	            message: message.into(),
    67	        }
    68	    }
    69	}
    70
   235	            while !diagnostic.is_char_boundary(boundary) {
   236	                boundary -= 1;
   237	            }
   238	            diagnostic.truncate(boundary);
   239	        }
   240	        Self { kind, diagnostic }
   241	    }
   242	}
   243
   244	pub fn drive_prepared_export(request: DriveRequest) -> Result<Vec<u8>, NativeDriveFailure> {
   245	    let input: serde_json::Value =
   246	        serde_json::from_slice(&request.opaque_invocation).map_err(|error| {
   247	            NativeDriveFailure::new(
   248	                WorkerFaultKind::InputLowering,
   249	                format!("opaque invocation lowering failed: {error}"),
   250	            )
   251	        })?;
   252	    let bundle = PreparedBundleV1::from_bytes(&request.prepared_artifact).map_err(|error| {
   253	        NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   254	    })?;
   255	    let bundle_id = bundle.bundle_id.clone();
   256	    let mut runtime =
   257	        NativeBunRuntime::initialize(BunRuntimeConfig::one_shot()).map_err(|error| {
   258	            NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   259	        })?;
   260	    let module = runtime
   261	        .load_module(BunModuleSpec::PreparedBundle {
   262	            bundle_id,
   263	            bytes: request.prepared_artifact,
   264	        })
   265	        .map_err(|error| {
   266	            NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   267	        })?;
   268	    let mut result = runtime
   269	        .call_export(&module, &request.selected_export, StructuralValue(input))
   270	        .map_err(|error| {
   271	            NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   272	        })?;
   273	    loop {
   274	        match result {
   275	            ExportCallResult::Ready(ProviderCallResult::Ok(value)) => {
   276	                return serde_json::to_vec(&value.0).map_err(|error| {
   277	                    NativeDriveFailure::new(
   278	                        WorkerFaultKind::CargoExtraction,
   279	                        format!("opaque cargo extraction failed: {error}"),
   280	                    )
   281	                });
   282	            }
   283	            ExportCallResult::Ready(ProviderCallResult::Err(error)) => {
   284	                return Err(NativeDriveFailure::new(
   285	                    WorkerFaultKind::JavaScriptRejection,
   286	                    error.message,
   287	                ));
   288	            }
   289	            ExportCallResult::Pending(handle) => {
   290	                runtime
   291	                    .pump_event_loop(PumpBudget { max_ticks: 256 })
   292	                    .map_err(|error| {
   293	                        NativeDriveFailure::new(WorkerFaultKind::CargoExtraction, error.to_string())
   294	                    })?;
   295	                if let Some(settled) = runtime.resolve_async(&handle).map_err(|error| {
   296	                    NativeDriveFailure::new(WorkerFaultKind::CargoExtraction, error.to_string())
   297	                })? {
   298	                    result = ExportCallResult::Ready(settled);
   299	                } else {
   300	                    result = ExportCallResult::Pending(handle);
   301	                }
   302	            }
   303	        }
   304	    }
   305	}
   306
   307	#[derive(Debug)]
   308	struct NativeBunRuntime {
   309	    vm: NonNull<VirtualMachine>,
   310	    modules: BTreeMap<String, JSValue>,
   311	    pending: BTreeMap<String, JSValue>,
   312	    stdout: OutputCapture,
   313	    stderr: OutputCapture,
   314	    log: OutputCapture,
   315	    prepared_bundle_tempdirs: Vec<tempfile::TempDir>,
   320
   321	#[derive(Debug)]
   322	struct OutputCapture {
   323	    write_file: std::fs::File,
   324	    read_file: std::fs::File,
   325	}
   326
   327	impl NativeBunRuntime {
   328	    fn vm(&self) -> &VirtualMachine {
   329	        // SAFETY: `vm` is initialized in `initialize` and remains live until
   330	        // `shutdown`, which consumes all public operations through the facade.
   331	        unsafe { self.vm.as_ref() }
   332	    }
   333
   334	    fn vm_mut(&mut self) -> &mut VirtualMachine {
   335	        // SAFETY: `NativeBunRuntime` is `&mut self`-borrowed for all VM-driving
   336	        // methods, matching Bun's single-JS-thread contract.
   337	        unsafe { self.vm.as_mut() }
   338	    }
   339
   340	    fn evaluate_json(&self, value: &StructuralValue) -> LibbunResult<JSValue> {
   341	        let json = serde_json::to_string(&value.0)
   342	            .map_err(|err| LibbunError::export_call(format!("input JSON encode failed: {err}")))?;
   343	        let json = ZigString::init(json.as_bytes());
   344	        self.vm().run_with_api_lock(|| {
   345	            let value = json.to_json_object(self.vm().global());
   346	            if value.is_empty() {
   347	                Err(LibbunError::export_call("input JSON parse failed"))
   348	            } else {
   349	                Ok(value)
   350	            }
   351	        })
   352	    }
   353
   354	    fn value_to_result(&self, value: JSValue) -> LibbunResult<ProviderCallResult> {
   355	        if value.is_undefined() || value.is_null() {
   356	            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
   357	        }
   358
   359	        let mut out = BunString::empty();
   360	        self.vm()
   361	            .run_with_api_lock(|| value.json_stringify_fast(self.vm().global(), &mut out))
   362	            .map_err(|_| LibbunError::export_call("JSON.stringify threw"))?;
   363	        let bytes = out.to_utf8_bytes();
   364	        out.deref();
   365
   366	        if bytes.is_empty() {
   367	            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
   368	        }
   369
   370	        let parsed = serde_json::from_slice(&bytes).map_err(|err| {
   371	            LibbunError::export_call(format!(
   372	                "provider result is not structurally serializable: {err}"
   373	            ))
   374	        })?;
   375	        Ok(ProviderCallResult::Ok(StructuralValue(parsed)))
   376	    }
   377
   378	    fn rejected_to_result(&self, value: JSValue) -> ProviderCallResult {
   379	        let value = value.to_error().unwrap_or(value);
   380	        ProviderCallResult::Err(ProviderError {
   381	            code: "provider_rejected".to_string(),
   382	            message: self.js_error_to_string(value, "provider promise rejected"),
   383	        })
   384	    }
   385
   386	    fn js_error_to_string(&self, value: JSValue, fallback: &str) -> String {
   387	        let global = self.vm().global();
   388	        if value.is_object() {
   389	            let stack = value
   390	                .get(global, "stack")
   391	                .ok()
   392	                .flatten()
   393	                .and_then(|stack| js_value_to_string(global, stack));
   394	            if let Some(stack) = stack.filter(|stack| !stack.trim().is_empty()) {
   395	                return bounded_js_diagnostic_text(stack);
   396	            }
   397
   398	            let message = value
   399	                .get(global, "message")
   400	                .ok()
   401	                .flatten()
   402	                .and_then(|message| js_value_to_string(global, message))
   403	                .or_else(|| {
   404	                    value
   405	                        .fast_get(global, BuiltinName::Message)
   406	                        .ok()
   407	                        .flatten()
   408	                        .and_then(|message| js_value_to_string(global, message))
   409	                });
   410	            let name = value
   411	                .fast_get(global, BuiltinName::name)
   412	                .ok()
   413	                .flatten()
   414	                .and_then(|name| js_value_to_string(global, name));
   415	            match (name, message) {
   416	                (Some(name), Some(message))
   417	                    if !name.trim().is_empty() && !message.trim().is_empty() =>
   418	                {
   419	                    return bounded_js_diagnostic_text(format!("{name}: {message}"));
   420	                }
   421	                (_, Some(message)) if !message.trim().is_empty() => {
   422	                    return bounded_js_diagnostic_text(message);
   423	                }
   424	                (Some(name), _) if !name.trim().is_empty() => {
   425	                    return bounded_js_diagnostic_text(name);
   426	                }
   427	                _ => {}
   428	            }
   429	            return js_value_to_string_lossy(global, value)
   430	                .map(bounded_js_diagnostic_text)
   431	                .unwrap_or_else(|| fallback.to_string());
   432	        }
   433
   434	        js_value_to_string_lossy(global, value)
   435	            .map(bounded_js_diagnostic_text)
   436	            .unwrap_or_else(|| fallback.to_string())
   437	    }
   438
   439	    fn import_module_specifier(&mut self, specifier: &str) -> LibbunResult<JSValue> {
   440	        let import_specifier = specifier.to_owned();
   441	        let specifier = BunString::from_bytes(specifier.as_bytes());
   442	        let promise = JSModuleLoader::import_ptr(self.vm().global, &specifier).map_err(|err| {
   443	            let exception = self.vm().global().take_exception(err);
   444	            let error = exception.to_error().unwrap_or(exception);
   445	            LibbunError::module_load(format!(
   446	                "module import threw for specifier `{import_specifier}`: {}",
   447	                self.js_error_to_string(error, "JavaScriptCore did not expose exception details")
   448	            ))
   449	        })?;
   450	        self.resolve_module_promise(
   451	            AnyPromise::Internal(promise.as_ptr()),
   452	            &format!("module import `{import_specifier}`"),
   453	        )
   454	    }
   455
   456	    fn resolve_module_promise(
   457	        &mut self,
   458	        promise: AnyPromise,
   459	        operation: &str,
   460	    ) -> LibbunResult<JSValue> {
   461	        self.vm_mut().wait_for_promise(promise);
   462
   463	        match promise.unwrap(unsafe { &*self.vm().jsc_vm }, UnwrapMode::MarkHandled) {
   464	            Unwrapped::Pending => Err(LibbunError::module_load(format!(
   465	                "{operation} remained pending after wait"
   466	            ))),
   467	            Unwrapped::Rejected(value) => {
   468	                let error = self.rejected_to_result(value);
   469	                match error {
   470	                    ProviderCallResult::Err(error) => Err(LibbunError::module_load(format!(
   471	                        "{}: {}",
   472	                        error.code, error.message
   473	                    ))),
   474	                    ProviderCallResult::Ok(_) => {
   475	                        Err(LibbunError::module_load(format!("{operation} rejected")))
   476	                    }
   477	                }
   478	            }
   479	            Unwrapped::Fulfilled(namespace) => Ok(namespace),
   480	        }
   481	    }
   482
   483	    fn promise_result(
   484	        &self,
   485	        promise: *mut JSInternalPromise,
   486	    ) -> LibbunResult<Option<ProviderCallResult>> {
   487	        let status = JSPromise::status_ptr(promise);
   488	        if status == PromiseStatus::Pending {
   489	            return Ok(None);
   490	        }
   491
   492	        let value = JSInternalPromise::opaque_mut(promise).result(unsafe { &*self.vm().jsc_vm });
   493	        if status == PromiseStatus::Rejected {
   494	            JSInternalPromise::opaque_mut(promise).set_handled();
   495	            return Ok(Some(self.rejected_to_result(value)));
   496	        }
   497	        Ok(Some(self.value_to_result(value)?))
   498	    }
   499
   500	    fn drain_output(&mut self) -> LibbunResult<()> {
   501	        bun_core::Output::flush();
   502	        self.stdout.drain()?;
   503	        self.stderr.drain()?;
   504	        self.log.drain()?;
   505	        Ok(())
   506	    }
   507
   508	    fn materialize_prepared_bundle(
   509	        &mut self,
   510	        module_id: &str,
   511	        bundle_id: &str,
   512	        bytes: &[u8],
   513	    ) -> LibbunResult<PathBuf> {
   514	        let bundle = PreparedBundleV1::from_bytes(bytes)?;
   515	        bundle.validate_for_current_runtime(bundle_id)?;
   516
   517	        let tempdir = tempfile::Builder::new()
   518	            .prefix("libbun-prepared-bundle-")
   519	            .tempdir()
   520	            .map_err(|err| {
   521	                LibbunError::module_load(format!("prepared bundle tempdir create failed: {err}"))
   522	            })?;
   523	        let bundle_dir = tempdir.path().join(format!("{module_id}.bundle"));
   524	        std::fs::create_dir_all(&bundle_dir).map_err(|err| {
   525	            LibbunError::module_load(format!("prepared bundle directory create failed: {err}"))
   526	        })?;
   527
   528	        for (module_path, module) in &bundle.modules {
   529	            let path = bundle_dir.join(module_path);
   530	            if let Some(parent) = path.parent() {
   531	                std::fs::create_dir_all(parent).map_err(|err| {
   532	                    LibbunError::module_load(format!(
   533	                        "prepared bundle module directory create failed: {err}"
   534	                    ))
   535	                })?;
   536	            }
   537	            std::fs::write(&path, module.source.as_bytes()).map_err(|err| {
   538	                LibbunError::module_load(format!(
   539	                    "prepared bundle module `{module_path}` write failed: {err}"
   540	                ))
   541	            })?;
   542	        }
   543
   544	        let entry_module = bundle_dir.join(bundle.entry_module);
   545	        self.prepared_bundle_tempdirs.push(tempdir);
   546	        Ok(entry_module)
   547	    }
   548	}
   549
   550	bun_core::declare_scope!(LibbunNative, visible);
   551
   552	impl OutputCapture {
   553	    fn create() -> LibbunResult<Self> {
   554	        let (read_file, write_file) = create_nonblocking_pipe_pair()?;
   555	        Ok(Self {
   556	            write_file,
   557	            read_file,
   558	        })
   559	    }
   560
   561	    fn bun_file(&self) -> bun_core::Output::File {
   562	        bun_core::Output::File(fd_from_file(&self.write_file))
   563	    }
   564
   565	    fn drain(&mut self) -> LibbunResult<()> {
   566	        let mut buffer = [0_u8; 8192];
   567	        loop {
   568	            match self.read_file.read(&mut buffer) {
   569	                Ok(0) => break,
   570	                Ok(_) => {}
   710	    if key.is_empty() || key.contains('=') || key.contains('\0') {
   711	        return Err(LibbunError::initialize(format!(
   712	            "invalid environment overlay key `{key}`"
   713	        )));
   714	    }
   715	    Ok(())
   716	}
   717
   718	impl NativeBunRuntime {
   719	    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
   720	        let runtime_guard = native_runtime_guard().try_lock().map_err(|err| match err {
   721	            TryLockError::WouldBlock => LibbunError::initialize(
   722	                "another native Bun runtime is already active in this process",
   723	            ),
   724	            TryLockError::Poisoned(_) => {
   725	                LibbunError::initialize("native Bun runtime guard is poisoned")
   726	            }
   727	        })?;
   728	        ensure_macos_compat_symbols();
   729	        bun_core::StackCheck::configure_thread();
   730
   731	        let stdout = OutputCapture::create()?;
   732	        let stderr = OutputCapture::create()?;
   733	        let log = OutputCapture::create()?;
   734	        bun_core::Output::Source::set_init(stdout.bun_file(), stderr.bun_file());
   735	        bun_core::Output::init_scoped_debug_writer_at_startup();
   736	        unsafe {
   737	            bun_core::Output::scoped_debug_writer::SCOPED_FILE_WRITER
   738	                .write(bun_core::Output::output_sink().quiet_writer_from_fd(log.bun_file().0));
   739	        }
   740
   741	        bun_jsc::initialize(false);
   742	        bun_ast::initialize_store();
   743
   744	        let vm = VirtualMachine::init(InitOptions {
   745	            is_main_thread: true,
   746	            mini_mode: false,
   747	            ..Default::default()
   748	        })
   749	        .map_err(|err| LibbunError::initialize(format!("{err:?}")))?;
   750	        let vm =
   751	            NonNull::new(vm).ok_or_else(|| LibbunError::initialize("Bun VM init returned null"))?;
   752	        apply_environment_overlay(
   753	            unsafe { vm.as_ptr().as_mut().expect("vm pointer checked") },
   754	            &config.environment,
   755	        )?;
   756	        // Bun's module loader/transpiler expects this VM-owned initialization
   757	        // before any provider import can reach source loading.
   758	        unsafe { vm.as_ptr().as_mut().expect("vm pointer checked") }
   759	            .load_extra_env_and_source_code_printer();
   760
   761	        Ok(Self {
   762	            vm,
   763	            modules: BTreeMap::new(),
   764	            pending: BTreeMap::new(),
   765	            stdout,
   766	            stderr,
   767	            log,
   768	            prepared_bundle_tempdirs: Vec::new(),
   769	            _runtime_guard: runtime_guard,
   770	            next_module: 1,
   771	            next_async: 1,
   772	        })
   773	    }
   774
   775	    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
   776	        let id = format!("module-{}", self.next_module);
   777	        self.next_module += 1;
   778	        bun_core::scoped_log!(LibbunNative, "loading module {}", id);
   779
   780	        let BunModuleSpec::PreparedBundle { bundle_id, bytes } = spec;
   781	        let specifier =
   782	            path_to_file_specifier(&self.materialize_prepared_bundle(&id, &bundle_id, &bytes)?)?;
   783	        let namespace = self.import_module_specifier(&specifier)?;
   784
   785	        self.vm().run_with_api_lock(|| namespace.protect());
   786	        self.modules.insert(id.clone(), namespace);
   787	        self.drain_output()?;
   788	        Ok(BunModuleHandle { id })
   789	    }
   790
   791	    fn call_export(
   792	        &mut self,
   793	        module: &BunModuleHandle,
   794	        export: &str,
   795	        input: StructuralValue,
   796	    ) -> LibbunResult<ExportCallResult> {
   797	        let namespace = *self
   798	            .modules
   799	            .get(&module.id)
   800	            .ok_or_else(|| LibbunError::module_load("unknown module handle"))?;
   801	        let function = namespace
   802	            .get(self.vm().global(), export)
   803	            .map_err(|_| LibbunError::export_call(format!("export lookup threw: {export}")))?
   804	            .ok_or_else(|| LibbunError::export_call(format!("missing export `{export}`")))?;
   805	        if !function.is_callable() {
   806	            return Err(LibbunError::export_call(format!(
   807	                "export `{export}` is not callable"
   808	            )));
   809	        }
   810
   811	        let arg = self.evaluate_json(&input)?;
   812	        let result = match self.vm().run_with_api_lock(|| {
   813	            match function.call(self.vm().global(), namespace, &[arg]) {
   814	                Ok(result) => Ok(result),
   815	                Err(error) => {
   816	                    let exception = self.vm().global().take_exception(error);
   817	                    Err(self.rejected_to_result(exception))
   818	                }
   819	            }
   820	        }) {
   821	            Ok(result) => result,
   822	            Err(error) => {
   823	                self.drain_output()?;
   824	                return Ok(ExportCallResult::Ready(error));
   825	            }
   826	        };
   827
   828	        if result.is_cell() && result.js_type() == JSType::JSPromise {
   829	            let id = format!("async-{}", self.next_async);
   830	            self.next_async += 1;
   831	            self.vm().run_with_api_lock(|| result.protect());
   832	            self.pending.insert(id.clone(), result);
   833	            self.drain_output()?;
   834	            return Ok(ExportCallResult::Pending(BunAsyncHandle { id }));
   835	        }
   836
   837	        let result = self.value_to_result(result)?;
   838	        self.drain_output()?;
   839	        Ok(ExportCallResult::Ready(result))
   840	    }
   841
   842	    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<()> {
   843	        for _ in 0..budget.max_ticks {
   844	            self.vm_mut().tick();
   845	            self.vm_mut().auto_tick();
   846	        }
   847	        self.drain_output()?;
   848	        Ok(())
   849	    }
   850
   851	    fn resolve_async(
   852	        &mut self,
   853	        handle: &BunAsyncHandle,
   854	    ) -> LibbunResult<Option<ProviderCallResult>> {
   855	        let value =
   856	            *self
   857	                .pending
   858	                .get(&handle.id)
   859	                .ok_or_else(|| LibbunError::UnknownAsyncHandle {
   860	                    handle: handle.id.clone(),
   861	                })?;
   862	        if !(value.is_cell() && value.js_type() == JSType::JSPromise) {
   863	            return Err(LibbunError::UnknownAsyncHandle {
   864	                handle: handle.id.clone(),
   865	            });
   866	        }
   867	        let promise =
   868	            value
   869	                .as_internal_promise()
   870	                .ok_or_else(|| LibbunError::UnknownAsyncHandle {
   871	                    handle: handle.id.clone(),
   872	                })?;
   873	        let result = self.promise_result(promise)?;
   874	        if result.is_some() {
   875	            self.vm().run_with_api_lock(|| value.unprotect());
   876	            self.pending.remove(&handle.id);
   877	        }
   878	        self.drain_output()?;
   879	        Ok(result)
   880	    }
   881	}
   882
   883	fn native_runtime_guard() -> &'static Mutex<()> {
   884	    static NATIVE_RUNTIME_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
   885	    NATIVE_RUNTIME_GUARD.get_or_init(|| Mutex::new(()))
   886	}
   887
   888	fn path_to_file_specifier(path: &Path) -> LibbunResult<String> {
   889	    // Avoid `std::fs::canonicalize` here. On Linux, Bun's linked mimalloc
   890	    // symbols can interpose the free path for libc `realpath` allocations,
   891	    // which makes canonicalize report a mimalloc invalid-pointer diagnostic.
   892	    let path = if path.is_absolute() {
   893	        path.to_path_buf()
   894	    } else {
   895	        std::env::current_dir()
   896	            .map_err(|err| LibbunError::module_load(format!("current_dir failed: {err}")))?
   897	            .join(path)
   898	    };
   899	    url::Url::from_file_path(&path)
   900	        .map(|url| url.to_string())
   901	        .map_err(|()| {
   902	            LibbunError::module_load(format!(
   903	                "path cannot be represented as a file URL: {}",
   904	                path.display()
   905	            ))
   906	        })
   907	}
   908
   909	#[cfg(target_os = "macos")]
   910	fn ensure_macos_compat_symbols() {
   911	    let symbol =
   912	        libbun_libcxx_hash_memory_compat as extern "C" fn(*const std::ffi::c_void, usize) -> usize;
   913	    std::hint::black_box(symbol);
   914	}
   915
   916	#[cfg(not(target_os = "macos"))]
   917	fn ensure_macos_compat_symbols() {}
   918
   919	#[cfg(target_os = "macos")]
   920	#[unsafe(export_name = "_ZNSt3__113__hash_memoryEPKvm")]

[build/link integration]
     1	use std::env;
     2	use std::fs;
     3	use std::path::Path;
     4	use std::path::PathBuf;
     5	use std::process::Command;
     6
     7	fn main() {
     8	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
     9	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    10	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");
    11
    12	    if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
    13	        return;
    14	    }
    15
    16	    let manifest = env::var_os("LIBBUN_NATIVE_LINK_MANIFEST")
    17	        .map(PathBuf::from)
    18	        .unwrap_or_else(default_manifest_path);
    19	    let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
    20	        panic!(
    21	            "failed to read native Bun link manifest at {}: {err}. Run scripts/prepare-native-bun-link.sh first.",
    22	            manifest.display()
    23	        )
    24	    });
    25	    let link_inputs = native_link_inputs_from_manifest(&manifest, &contents);
    26
    27	    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    28	    if target_os == "linux" {
    29	        println!("cargo:rustc-link-arg=-fuse-ld=lld");
    30	    }
    31
    32	    for path in link_inputs {
    33	        if target_os == "macos" {
    34	            println!("cargo:rustc-link-arg=-Wl,-force_load,{}", path.display());
    35	        } else {
    36	            println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    37	            println!("cargo:rustc-link-arg={}", path.display());
    38	            println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    39	        }
    40	    }
    41
    42	    if target_os == "macos" {
    43	        println!("cargo:rustc-link-arg=-fsanitize=null");
    44	        println!("cargo:rustc-link-arg=-Wl,-ld_new");
    45	        println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
    46	        println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1200000");
    47	        println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
    48	        if let Some(ubsan) = find_compiler_rt("libclang_rt.ubsan_osx_dynamic.dylib") {
    49	            let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
    50	            println!("cargo:rustc-link-arg={}", ubsan.display());
    51	            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
    52	        }
    53	        println!("cargo:rustc-link-lib=c++");
    54	        println!("cargo:rustc-link-lib=icucore");
    55	        println!("cargo:rustc-link-lib=resolv");
    56	    } else if target_os == "linux" {
    57	        println!("cargo:rustc-link-lib=stdc++");
    58	        println!("cargo:rustc-link-lib=dl");
    59	        println!("cargo:rustc-link-lib=pthread");
    60	        println!("cargo:rustc-link-lib=m");
    61	    }
    62	}
    63
    64	fn find_compiler_rt(library: &str) -> Option<PathBuf> {
    65	    let output = Command::new("clang")
    66	        .arg(format!("-print-file-name={library}"))
    67	        .output()
    68	        .ok()?;
    69	    if !output.status.success() {
    70	        return None;
    71	    }
    72	    let path = String::from_utf8(output.stdout).ok()?;
    73	    let path = PathBuf::from(path.trim());
    74	    path.exists().then_some(path)
    75	}
    76
    77	fn default_manifest_path() -> PathBuf {
    78	    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    79	    let repo_root = manifest_dir.parent().expect("native crate has repo parent");
    80	    env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
    81	        .map(PathBuf::from)
    82	        .map(|path| {
    83	            if path.is_absolute() {
    84	                path
    85	            } else {
    86	                repo_root.join(path)
    87	            }
    88	        })
    89	        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
    90	        .join("libbun_native_link_manifest.txt")
    91	}
    92
    93	fn native_link_inputs_from_manifest(manifest: &Path, contents: &str) -> Vec<PathBuf> {
    94	    let mut link_inputs = Vec::new();
    95	    for line in contents.lines() {
    96	        let Some((kind, raw_path)) = line.split_once('=') else {
    97	            continue;
    98	        };
    99	        if kind != "archive" && kind != "static" {
   100	            continue;
   101	        }
   102	        reject_debug_native_link_input(manifest, raw_path);
   103	        let path = PathBuf::from(raw_path);
   104	        let path = if path.is_absolute() {
   105	            path
   106	        } else {
   107	            manifest
   108	                .parent()
   109	                .expect("native link manifest has parent")
   110	                .join(path)
   111	        };
   112	        if !path.exists() {
   113	            panic!(
   114	                "native Bun link manifest {} references missing input {}. Regenerate it locally with scripts/prepare-native-bun-link.sh.",
   115	                manifest.display(),
   116	                path.display()
   117	            );
   118	        }
   119	        link_inputs.push(path);
   120	    }
   121
   122	    if link_inputs.is_empty() {
   123	        panic!(
   124	            "native Bun link manifest {} contains no archive/static inputs. Regenerate it locally with scripts/prepare-native-bun-link.sh.",
   125	            manifest.display()
   126	        );
   127	    }
   128	    link_inputs
   129	}
   130
   131	fn reject_debug_native_link_input(manifest: &Path, path: &str) {
   132	    if path.contains("/build/debug/")
   133	        || path.contains("\\build\\debug\\")
   134	        || path.contains("/bun-debug")
   135	        || path.contains("\\bun-debug")
   136	        || path.contains("-debug/")
   137	        || path.contains("-debug\\")
   138	    {
   139	        panic!(
   140	            "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
   141	            manifest.display(),
   142	            path
   143	        );
   144	    }
   145	}
     1	use std::env;
     2	use std::fs;
     3	use std::path::Path;
     4	use std::path::PathBuf;
     5	use std::process::Command;
     6
     7	fn main() {
     8	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
     9	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    10	    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");
    11
    12	    if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
    13	        println!(
    14	            "cargo:warning=building libbun-runtime-native without native Bun link inputs; set LIBBUN_NATIVE_LINK_BUN=1 for a runnable helper"
    15	        );
    16	        return;
    17	    }
    18
    19	    let manifest = env::var_os("LIBBUN_NATIVE_LINK_MANIFEST")
    20	        .map(PathBuf::from)
    21	        .unwrap_or_else(default_manifest_path);
    22	    let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
    23	        panic!(
    24	            "failed to read native Bun link manifest at {}: {err}. Run scripts/prepare-native-bun-link.sh first.",
    25	            manifest.display()
    26	        )
    27	    });
    28
    29	    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    30	    if target_os == "linux" {
    31	        println!("cargo:rustc-link-arg=-fuse-ld=lld");
    32	    }
    33
    34	    for line in contents.lines() {
    35	        let Some((kind, path)) = line.split_once('=') else {
    36	            continue;
    37	        };
    38	        if kind == "archive" || kind == "static" {
    39	            reject_debug_native_link_input(&manifest, path);
    40	            if target_os == "macos" {
    41	                println!("cargo:rustc-link-arg=-Wl,-force_load,{path}");
    42	            } else {
    43	                println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    44	                println!("cargo:rustc-link-arg={path}");
    45	                println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    46	            }
    47	        }
    48	    }
    49
    50	    if target_os == "macos" {
    51	        println!("cargo:rustc-link-arg=-fsanitize=null");
    52	        println!("cargo:rustc-link-arg=-Wl,-ld_new");
    53	        println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
    54	        println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1200000");
    55	        println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
    56	        if let Some(ubsan) = find_compiler_rt("libclang_rt.ubsan_osx_dynamic.dylib") {
    57	            let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
    58	            println!("cargo:rustc-link-arg={}", ubsan.display());
    59	            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
    60	        }
    61	        println!("cargo:rustc-link-lib=c++");
    62	        println!("cargo:rustc-link-lib=icucore");
    63	        println!("cargo:rustc-link-lib=resolv");
    64	    } else if target_os == "linux" {
    65	        link_compiler_rt_archive("libclang_rt.ubsan_standalone-aarch64.a");
    66	        link_compiler_rt_archive("libclang_rt.ubsan_standalone-x86_64.a");
    67	        link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-aarch64.a");
    68	        link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-x86_64.a");
    69	        println!("cargo:rustc-link-lib=stdc++");
    70	        println!("cargo:rustc-link-lib=dl");
    71	        println!("cargo:rustc-link-lib=pthread");
    72	        println!("cargo:rustc-link-lib=m");
    73	    }
    74	}
    75
    76	fn link_compiler_rt_archive(library: &str) {
    77	    let Some(path) = find_compiler_rt(library) else {
    78	        return;
    79	    };
    80	    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    81	    println!("cargo:rustc-link-arg={}", path.display());
    82	    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    83	}
    84
    85	fn find_compiler_rt(library: &str) -> Option<PathBuf> {
    86	    let output = Command::new("clang")
    87	        .arg(format!("-print-file-name={library}"))
    88	        .output()
    89	        .ok()?;
    90	    if !output.status.success() {
    91	        return None;
    92	    }
    93	    let path = String::from_utf8(output.stdout).ok()?;
    94	    let path = PathBuf::from(path.trim());
    95	    path.exists().then_some(path)
    96	}
    97
    98	fn default_manifest_path() -> PathBuf {
    99	    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
   100	    let repo_root = manifest_dir
   101	        .parent()
   102	        .expect("runtime crate has repo parent");
   103	    env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
   104	        .map(PathBuf::from)
   105	        .map(|path| {
   106	            if path.is_absolute() {
   107	                path
   108	            } else {
   109	                repo_root.join(path)
   110	            }
   111	        })
   112	        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
   113	        .join("libbun_native_link_manifest.txt")
   114	}
   115
   116	fn reject_debug_native_link_input(manifest: &Path, path: &str) {
   117	    if path.contains("/build/debug/")
   118	        || path.contains("\\build\\debug\\")
   119	        || path.contains("/bun-debug")
   120	        || path.contains("\\bun-debug")
   121	        || path.contains("-debug/")
   122	        || path.contains("-debug\\")
   123	    {
   124	        panic!(
   125	            "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
   126	            manifest.display(),
   127	            path
   128	        );
   129	    }
   130	}
     1	use std::io;
     2
     3	#[cfg(target_os = "linux")]
     4	use bun_platform as _;
     5
     6	fn main() {
     7	    if let Err(error) = run_one_drive() {
     8	        eprintln!("libbun one-shot worker failed: {error}");
     9	        std::process::exit(1);
    10	    }
    11	}
    12
    13	fn run_one_drive() -> io::Result<()> {
    14	    let request = match libbun_prepared_export_wire::read_drive_request(&mut io::stdin().lock()) {
    15	        Ok(request) => request,
    16	        Err(error) => {
    17	            return libbun_prepared_export_wire::write_fault(
    18	                &mut io::stdout().lock(),
    19	                libbun_prepared_export_wire::WorkerFaultKind::Internal,
    20	                &format!("private drive request admission failed: {error}"),
    21	            );
    22	        }
    23	    };
    24
    25	    match libbun_native::drive_prepared_export(request) {
    26	        Ok(cargo) => {
    27	            libbun_prepared_export_wire::write_cargo(&mut io::stdout().lock(), &cargo)?;
    28	        }
    29	        Err(failure) => {
    30	            libbun_prepared_export_wire::write_fault(
    31	                &mut io::stdout().lock(),
    32	                failure.kind(),
    33	                failure.diagnostic(),
    34	            )?;
    35	        }
    36	    }
    37	    Ok(())
    38	}
