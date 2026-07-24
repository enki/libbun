# libbun W1-11/W1-12 vendored Bun boundary report (correction 4)

Generated from exact product SHA 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb, tree cb964de8ab8162449fbe95959bf34d231570aa5c.

Generator command: python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --emit vendored-bun-boundary-report.md

No shell pipeline or awk transformation participates in identity generation.

## Full-file identities

| Path | Git blob | SHA-256 | Bytes |
| --- | --- | --- | ---: |
| patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | 29b85eb51d9b35735369831c50c1b6a3df3f2a17 | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | d263927c2a5209a8cebabd6f1cc5dc982536b142 | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| native/build.rs | b2d755d810a45a4c972953cdf5521174bcd10925 | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| runtime/build.rs | b889b6059332d8adb8c40dd70177084f93e52b77 | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| scripts/prepare-native-bun-link.sh | 2b44b7e3ec653f663ee8857bc7fd188b5cff113b | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| BUN_SOURCE_COMMIT | a895f3402c48a07fefc650ee455e79ebb34de770 | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |
| vendor/bun/src/jsc/bindings/bindings.cpp | b08737cc9ba97c08fccd8d291f73cc03275031d0 | e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff | 255398 |

## Full patch: patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch

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

## Full patch: patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch

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

## vendor/bun/src/jsc/bindings/bindings.cpp:4340-4395

- Full-file Git blob: b08737cc9ba97c08fccd8d291f73cc03275031d0
- Full-file SHA-256: e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff
- Full-file bytes: 255398
- Excerpt line span: 4340-4395
- Excerpt SHA-256: 0161434f76fa2e74a145cbdfd58635dd08e7cfd873080f1fd7d8a60809c05ff6

  4340
  4341      auto& vm = JSC::getVM(globalObject);
  4342      JSC::JSObject* object = value.getObject();
  4343      if (!object) [[unlikely]] {
  4344          return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4345      }
  4346
  4347      // Since Identifier might not ref the string, we need to ensure it doesn't get deref'd until this function returns
  4348      const auto propertyString = String(StringImpl::createWithoutCopying({ arg1, arg2 }));
  4349      const auto identifier = JSC::Identifier::fromString(vm, propertyString);
  4350      const auto property = JSC::PropertyName(identifier);
  4351
  4352      return JSC::JSValue::encode(Bun::getIfPropertyExistsPrototypePollutionMitigationUnsafe(vm, globalObject, object, property));
  4353  }
  4354
  4355  // Returns empty for exception, returns deleted if not found.
  4356  // Be careful when handling the return value.
  4357  // Can handle numeric index property names safely. If you know that the property name is not an integer index, use JSC__JSValue__getIfPropertyExistsImpl instead.
  4358  JSC::EncodedJSValue JSC__JSValue__getPropertyValue(JSC::EncodedJSValue encodedValue,
  4359      JSC::JSGlobalObject* globalObject,
  4360      const unsigned char* propertyName, uint32_t propertyNameLength)
  4361  {
  4362
  4363      ASSERT_NO_PENDING_EXCEPTION(globalObject);
  4364      JSValue value = JSC::JSValue::decode(encodedValue);
  4365      ASSERT_WITH_MESSAGE(!value.isEmpty(), "getPropertyValue() must not be called on empty value");
  4366
  4367      auto& vm = JSC::getVM(globalObject);
  4368      JSC::JSObject* object = value.getObject();
  4369      if (!object) [[unlikely]] {
  4370          return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4371      }
  4372
  4373      // Since Identifier might not ref the string, we need to ensure it doesn't get deref'd until this function returns
  4374      const auto propertyString = String(StringImpl::createWithoutCopying({ propertyName, propertyNameLength }));
  4375      const auto identifier = JSC::Identifier::fromString(vm, propertyString);
  4376      const auto property = JSC::PropertyName(identifier);
  4377
  4378      auto scope = DECLARE_THROW_SCOPE(vm);
  4379      PropertySlot slot(object, PropertySlot::InternalMethodType::Get);
  4380      if (!object->getPropertySlot(globalObject, property, slot)) {
  4381          RETURN_IF_EXCEPTION(scope, {});
  4382          return JSValue::encode(JSValue::decode(JSC::JSValue::ValueDeleted));
  4383      }
  4384      RETURN_IF_EXCEPTION(scope, {});
  4385
  4386      JSValue result = slot.getValue(globalObject, property);
  4387      RETURN_IF_EXCEPTION(scope, {});
  4388
  4389      return JSValue::encode(result);
  4390  }
  4391
  4392  extern "C" JSC::EncodedJSValue JSC__JSValue__getOwn(JSC::EncodedJSValue JSValue0, JSC::JSGlobalObject* globalObject, BunString* propertyName)
  4393  {
  4394      ASSERT_NO_PENDING_EXCEPTION(globalObject);
  4395

## native/src/lib.rs:235-315

- Full-file Git blob: abb0dbdfc3c3832b85071c8d2fcf2557ff00d82d
- Full-file SHA-256: 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a
- Full-file bytes: 32133
- Excerpt line span: 235-315
- Excerpt SHA-256: 9c4375be3fa5c5d759e1b73cb306945ff30fc81ec025dfdbc7ad226a62bba9cf

   235              while !diagnostic.is_char_boundary(boundary) {
   236                  boundary -= 1;
   237              }
   238              diagnostic.truncate(boundary);
   239          }
   240          Self { kind, diagnostic }
   241      }
   242  }
   243
   244  pub fn drive_prepared_export(request: DriveRequest) -> Result<Vec<u8>, NativeDriveFailure> {
   245      let input: serde_json::Value =
   246          serde_json::from_slice(&request.opaque_invocation).map_err(|error| {
   247              NativeDriveFailure::new(
   248                  WorkerFaultKind::InputLowering,
   249                  format!("opaque invocation lowering failed: {error}"),
   250              )
   251          })?;
   252      let bundle = PreparedBundleV1::from_bytes(&request.prepared_artifact).map_err(|error| {
   253          NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   254      })?;
   255      let bundle_id = bundle.bundle_id.clone();
   256      let mut runtime =
   257          NativeBunRuntime::initialize(BunRuntimeConfig::one_shot()).map_err(|error| {
   258              NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   259          })?;
   260      let module = runtime
   261          .load_module(BunModuleSpec::PreparedBundle {
   262              bundle_id,
   263              bytes: request.prepared_artifact,
   264          })
   265          .map_err(|error| {
   266              NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   267          })?;
   268      let mut result = runtime
   269          .call_export(&module, &request.selected_export, StructuralValue(input))
   270          .map_err(|error| {
   271              NativeDriveFailure::new(WorkerFaultKind::Preparation, error.to_string())
   272          })?;
   273      loop {
   274          match result {
   275              ExportCallResult::Ready(ProviderCallResult::Ok(value)) => {
   276                  return serde_json::to_vec(&value.0).map_err(|error| {
   277                      NativeDriveFailure::new(
   278                          WorkerFaultKind::CargoExtraction,
   279                          format!("opaque cargo extraction failed: {error}"),
   280                      )
   281                  });
   282              }
   283              ExportCallResult::Ready(ProviderCallResult::Err(error)) => {
   284                  return Err(NativeDriveFailure::new(
   285                      WorkerFaultKind::JavaScriptRejection,
   286                      error.message,
   287                  ));
   288              }
   289              ExportCallResult::Pending(handle) => {
   290                  runtime
   291                      .pump_event_loop(PumpBudget { max_ticks: 256 })
   292                      .map_err(|error| {
   293                          NativeDriveFailure::new(WorkerFaultKind::CargoExtraction, error.to_string())
   294                      })?;
   295                  if let Some(settled) = runtime.resolve_async(&handle).map_err(|error| {
   296                      NativeDriveFailure::new(WorkerFaultKind::CargoExtraction, error.to_string())
   297                  })? {
   298                      result = ExportCallResult::Ready(settled);
   299                  } else {
   300                      result = ExportCallResult::Pending(handle);
   301                  }
   302              }
   303          }
   304      }
   305  }
   306
   307  #[derive(Debug)]
   308  struct NativeBunRuntime {
   309      vm: NonNull<VirtualMachine>,
   310      modules: BTreeMap<String, JSValue>,
   311      pending: BTreeMap<String, JSValue>,
   312      stdout: OutputCapture,
   313      stderr: OutputCapture,
   314      log: OutputCapture,
   315      prepared_bundle_tempdirs: Vec<tempfile::TempDir>,

## native/build.rs:1-145

- Full-file Git blob: b2d755d810a45a4c972953cdf5521174bcd10925
- Full-file SHA-256: 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d
- Full-file bytes: 5195
- Excerpt line span: 1-145
- Excerpt SHA-256: 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d

     1  use std::env;
     2  use std::fs;
     3  use std::path::Path;
     4  use std::path::PathBuf;
     5  use std::process::Command;
     6
     7  fn main() {
     8      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
     9      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    10      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");
    11
    12      if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
    13          return;
    14      }
    15
    16      let manifest = env::var_os("LIBBUN_NATIVE_LINK_MANIFEST")
    17          .map(PathBuf::from)
    18          .unwrap_or_else(default_manifest_path);
    19      let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
    20          panic!(
    21              "failed to read native Bun link manifest at {}: {err}. Run scripts/prepare-native-bun-link.sh first.",
    22              manifest.display()
    23          )
    24      });
    25      let link_inputs = native_link_inputs_from_manifest(&manifest, &contents);
    26
    27      let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    28      if target_os == "linux" {
    29          println!("cargo:rustc-link-arg=-fuse-ld=lld");
    30      }
    31
    32      for path in link_inputs {
    33          if target_os == "macos" {
    34              println!("cargo:rustc-link-arg=-Wl,-force_load,{}", path.display());
    35          } else {
    36              println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    37              println!("cargo:rustc-link-arg={}", path.display());
    38              println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    39          }
    40      }
    41
    42      if target_os == "macos" {
    43          println!("cargo:rustc-link-arg=-fsanitize=null");
    44          println!("cargo:rustc-link-arg=-Wl,-ld_new");
    45          println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
    46          println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1200000");
    47          println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
    48          if let Some(ubsan) = find_compiler_rt("libclang_rt.ubsan_osx_dynamic.dylib") {
    49              let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
    50              println!("cargo:rustc-link-arg={}", ubsan.display());
    51              println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
    52          }
    53          println!("cargo:rustc-link-lib=c++");
    54          println!("cargo:rustc-link-lib=icucore");
    55          println!("cargo:rustc-link-lib=resolv");
    56      } else if target_os == "linux" {
    57          println!("cargo:rustc-link-lib=stdc++");
    58          println!("cargo:rustc-link-lib=dl");
    59          println!("cargo:rustc-link-lib=pthread");
    60          println!("cargo:rustc-link-lib=m");
    61      }
    62  }
    63
    64  fn find_compiler_rt(library: &str) -> Option<PathBuf> {
    65      let output = Command::new("clang")
    66          .arg(format!("-print-file-name={library}"))
    67          .output()
    68          .ok()?;
    69      if !output.status.success() {
    70          return None;
    71      }
    72      let path = String::from_utf8(output.stdout).ok()?;
    73      let path = PathBuf::from(path.trim());
    74      path.exists().then_some(path)
    75  }
    76
    77  fn default_manifest_path() -> PathBuf {
    78      let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    79      let repo_root = manifest_dir.parent().expect("native crate has repo parent");
    80      env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
    81          .map(PathBuf::from)
    82          .map(|path| {
    83              if path.is_absolute() {
    84                  path
    85              } else {
    86                  repo_root.join(path)
    87              }
    88          })
    89          .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
    90          .join("libbun_native_link_manifest.txt")
    91  }
    92
    93  fn native_link_inputs_from_manifest(manifest: &Path, contents: &str) -> Vec<PathBuf> {
    94      let mut link_inputs = Vec::new();
    95      for line in contents.lines() {
    96          let Some((kind, raw_path)) = line.split_once('=') else {
    97              continue;
    98          };
    99          if kind != "archive" && kind != "static" {
   100              continue;
   101          }
   102          reject_debug_native_link_input(manifest, raw_path);
   103          let path = PathBuf::from(raw_path);
   104          let path = if path.is_absolute() {
   105              path
   106          } else {
   107              manifest
   108                  .parent()
   109                  .expect("native link manifest has parent")
   110                  .join(path)
   111          };
   112          if !path.exists() {
   113              panic!(
   114                  "native Bun link manifest {} references missing input {}. Regenerate it locally with scripts/prepare-native-bun-link.sh.",
   115                  manifest.display(),
   116                  path.display()
   117              );
   118          }
   119          link_inputs.push(path);
   120      }
   121
   122      if link_inputs.is_empty() {
   123          panic!(
   124              "native Bun link manifest {} contains no archive/static inputs. Regenerate it locally with scripts/prepare-native-bun-link.sh.",
   125              manifest.display()
   126          );
   127      }
   128      link_inputs
   129  }
   130
   131  fn reject_debug_native_link_input(manifest: &Path, path: &str) {
   132      if path.contains("/build/debug/")
   133          || path.contains("\\build\\debug\\")
   134          || path.contains("/bun-debug")
   135          || path.contains("\\bun-debug")
   136          || path.contains("-debug/")
   137          || path.contains("-debug\\")
   138      {
   139          panic!(
   140              "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
   141              manifest.display(),
   142              path
   143          );
   144      }
   145  }

## runtime/build.rs:1-130

- Full-file Git blob: b889b6059332d8adb8c40dd70177084f93e52b77
- Full-file SHA-256: c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0
- Full-file bytes: 4902
- Excerpt line span: 1-130
- Excerpt SHA-256: c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0

     1  use std::env;
     2  use std::fs;
     3  use std::path::Path;
     4  use std::path::PathBuf;
     5  use std::process::Command;
     6
     7  fn main() {
     8      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
     9      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    10      println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");
    11
    12      if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
    13          println!(
    14              "cargo:warning=building libbun-runtime-native without native Bun link inputs; set LIBBUN_NATIVE_LINK_BUN=1 for a runnable helper"
    15          );
    16          return;
    17      }
    18
    19      let manifest = env::var_os("LIBBUN_NATIVE_LINK_MANIFEST")
    20          .map(PathBuf::from)
    21          .unwrap_or_else(default_manifest_path);
    22      let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
    23          panic!(
    24              "failed to read native Bun link manifest at {}: {err}. Run scripts/prepare-native-bun-link.sh first.",
    25              manifest.display()
    26          )
    27      });
    28
    29      let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    30      if target_os == "linux" {
    31          println!("cargo:rustc-link-arg=-fuse-ld=lld");
    32      }
    33
    34      for line in contents.lines() {
    35          let Some((kind, path)) = line.split_once('=') else {
    36              continue;
    37          };
    38          if kind == "archive" || kind == "static" {
    39              reject_debug_native_link_input(&manifest, path);
    40              if target_os == "macos" {
    41                  println!("cargo:rustc-link-arg=-Wl,-force_load,{path}");
    42              } else {
    43                  println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    44                  println!("cargo:rustc-link-arg={path}");
    45                  println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    46              }
    47          }
    48      }
    49
    50      if target_os == "macos" {
    51          println!("cargo:rustc-link-arg=-fsanitize=null");
    52          println!("cargo:rustc-link-arg=-Wl,-ld_new");
    53          println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
    54          println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1200000");
    55          println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
    56          if let Some(ubsan) = find_compiler_rt("libclang_rt.ubsan_osx_dynamic.dylib") {
    57              let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
    58              println!("cargo:rustc-link-arg={}", ubsan.display());
    59              println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
    60          }
    61          println!("cargo:rustc-link-lib=c++");
    62          println!("cargo:rustc-link-lib=icucore");
    63          println!("cargo:rustc-link-lib=resolv");
    64      } else if target_os == "linux" {
    65          link_compiler_rt_archive("libclang_rt.ubsan_standalone-aarch64.a");
    66          link_compiler_rt_archive("libclang_rt.ubsan_standalone-x86_64.a");
    67          link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-aarch64.a");
    68          link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-x86_64.a");
    69          println!("cargo:rustc-link-lib=stdc++");
    70          println!("cargo:rustc-link-lib=dl");
    71          println!("cargo:rustc-link-lib=pthread");
    72          println!("cargo:rustc-link-lib=m");
    73      }
    74  }
    75
    76  fn link_compiler_rt_archive(library: &str) {
    77      let Some(path) = find_compiler_rt(library) else {
    78          return;
    79      };
    80      println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    81      println!("cargo:rustc-link-arg={}", path.display());
    82      println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    83  }
    84
    85  fn find_compiler_rt(library: &str) -> Option<PathBuf> {
    86      let output = Command::new("clang")
    87          .arg(format!("-print-file-name={library}"))
    88          .output()
    89          .ok()?;
    90      if !output.status.success() {
    91          return None;
    92      }
    93      let path = String::from_utf8(output.stdout).ok()?;
    94      let path = PathBuf::from(path.trim());
    95      path.exists().then_some(path)
    96  }
    97
    98  fn default_manifest_path() -> PathBuf {
    99      let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
   100      let repo_root = manifest_dir
   101          .parent()
   102          .expect("runtime crate has repo parent");
   103      env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
   104          .map(PathBuf::from)
   105          .map(|path| {
   106              if path.is_absolute() {
   107                  path
   108              } else {
   109                  repo_root.join(path)
   110              }
   111          })
   112          .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
   113          .join("libbun_native_link_manifest.txt")
   114  }
   115
   116  fn reject_debug_native_link_input(manifest: &Path, path: &str) {
   117      if path.contains("/build/debug/")
   118          || path.contains("\\build\\debug\\")
   119          || path.contains("/bun-debug")
   120          || path.contains("\\bun-debug")
   121          || path.contains("-debug/")
   122          || path.contains("-debug\\")
   123      {
   124          panic!(
   125              "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
   126              manifest.display(),
   127              path
   128          );
   129      }
   130  }

## runtime/src/main.rs:1-38

- Full-file Git blob: ad00021b745eec81d53c813d5368f08da6e18e63
- Full-file SHA-256: 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee
- Full-file bytes: 1117
- Excerpt line span: 1-38
- Excerpt SHA-256: 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee

     1  use std::io;
     2
     3  #[cfg(target_os = "linux")]
     4  use bun_platform as _;
     5
     6  fn main() {
     7      if let Err(error) = run_one_drive() {
     8          eprintln!("libbun one-shot worker failed: {error}");
     9          std::process::exit(1);
    10      }
    11  }
    12
    13  fn run_one_drive() -> io::Result<()> {
    14      let request = match libbun_prepared_export_wire::read_drive_request(&mut io::stdin().lock()) {
    15          Ok(request) => request,
    16          Err(error) => {
    17              return libbun_prepared_export_wire::write_fault(
    18                  &mut io::stdout().lock(),
    19                  libbun_prepared_export_wire::WorkerFaultKind::Internal,
    20                  &format!("private drive request admission failed: {error}"),
    21              );
    22          }
    23      };
    24
    25      match libbun_native::drive_prepared_export(request) {
    26          Ok(cargo) => {
    27              libbun_prepared_export_wire::write_cargo(&mut io::stdout().lock(), &cargo)?;
    28          }
    29          Err(failure) => {
    30              libbun_prepared_export_wire::write_fault(
    31                  &mut io::stdout().lock(),
    32                  failure.kind(),
    33                  failure.diagnostic(),
    34              )?;
    35          }
    36      }
    37      Ok(())
    38  }
