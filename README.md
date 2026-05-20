# libbun

Rust facade for hosting JavaScript and TypeScript providers through a non-CLI
Bun embedding boundary.

This repository owns the stable facade, conformance tests, and a vendored Bun
source snapshot. It does not call Bun CLI `main`, `Cli::start`, or
process-global command dispatch.

Current Bun source target:

```text
9ecb985ad0f06fa12cbd8eede2404589992527d5
```

## Status

The initial crate defines the embedding ABI, provider-host receipts, structural
value carriers, prepared source bundle artifacts, explicit event-loop pumping,
output capture, deterministic shutdown, and Rust-substrate rejection.

The native adapter binds this facade to Bun/JSC internals and has a real linked
integration flow for source module load, prepared source bundle load,
synchronous export calls, async export parking/resolution, structured provider
errors, event-loop pumping, host environment overlays, dedicated internal log
capture, and shutdown. Downstream hosts consume the native implementation only
through the replaceable dynamic plugin described by ADR-2038; they should not
statically link `libbun-native`.

## Downstream Use

Downstream Rust applications depend on the facade crate and load the native Bun
implementation through a replaceable plugin. Product hosts should bundle that
plugin relative to their own binary. The download/cache helpers are development
and packaging conveniences, not the runtime contract for shipped hosts.

### Bundled Product Integration

Use this mode for applications that ship a native binary.

Depend on the facade without `download-plugin`:

```sh
cargo add libbun --features dynamic-loading
```

Bundle the verified native plugin beside the host binary, or in a deterministic
directory relative to it:

```text
bin/
  ss
  liblibbun_plugin_native.dylib      # macOS
  liblibbun_plugin_native.so         # Linux
```

Then load by exact path or binary-relative resolution:

```rust
use libbun::dynamic::DynamicBunRuntime;

let runtime = DynamicBunRuntime::initialize_with_bundled_plugin(config, host_binary_path)?;
```

`LIBBUN_PLUGIN_PATH` remains a user/admin replacement override. Product hosts
must not rely on `~/.cache/libbun`, `LIBBUN_HOME`, or build-output release
caches at runtime.

### Automatic Cargo Build Download

Use this mode for local development and experiments whose Cargo builds are
allowed to download verified release artifacts. It is not the product shipping
topology for native host binaries.

Add `libbun` with `dynamic-loading` and `download-plugin`:

```sh
cargo add libbun --features dynamic-loading,download-plugin
```

With `download-plugin`, `libbun`'s build script selects the Cargo `TARGET`,
downloads the matching native plugin release asset for the crate version,
verifies its committed checksum, and extracts it under Cargo's `OUT_DIR`.

`download-plugin` is intentionally opt-in because it makes Cargo builds depend
on network access unless an override is provided. Use these overrides when the
artifact is pre-fetched by CI, a package manager, or an app release process:

```text
LIBBUN_PLUGIN_PATH=/absolute/path/to/liblibbun_plugin_native.dylib
LIBBUN_PLUGIN_BUNDLE_DIR=/absolute/path/to/extracted/libbun/bundle
LIBBUN_PLUGIN_ARCHIVE=/absolute/path/to/libbun-plugin-native-vX.Y.Z-<target>.tar.zst
LIBBUN_DOWNLOAD_PLUGIN=0
```

`LIBBUN_PLUGIN_PATH` is also the user replacement path and always wins at
runtime.

### No-Download Packaging

Package managers, hermetic CI systems, and app release processes can fetch the
GitHub Release assets directly and place the extracted plugin into the host
bundle. The important rules are that the plugin remains dynamically loaded,
user-replaceable, and binary-relative for product hosts.

Download the plugin asset that matches the host platform from the native plugin
release tag selected by the `libbun` facade crate. Facade patch releases may
reuse an existing native plugin release when the native bytes do not change.
The selected tag is exposed by `libbun::release::RELEASE_TAG` and in missing
plugin errors. The supported native plugin release targets are:

```text
libbun-plugin-native-vX.Y.Z-aarch64-apple-darwin.tar.zst
libbun-plugin-native-vX.Y.Z-x86_64-unknown-linux-gnu.tar.zst
libbun-plugin-native-vX.Y.Z-aarch64-unknown-linux-gnu.tar.zst
```

The consumer contract is the same on every platform:

```text
consumer app -> bundled plugin path or LIBBUN_PLUGIN_PATH -> native plugin
```

The implementation behind that plugin is recorded in
`libbun-native-bundle.json`:

```text
macOS:
consumer app -> dynamically loaded .dylib -> in-process Bun/JSC/WebKit

Linux in-process releases:
consumer app -> dynamically loaded .so -> in-process Bun/JSC/WebKit

Older Linux helper-backed releases:
consumer app -> dynamically loaded .so -> helper process -> Bun/JSC/WebKit
```

Linux in-process tarballs contain `liblibbun_plugin_native.so` plus
`libbun-native-bundle.json`; helper-backed tarballs also contain
`libbun-runtime-native`. Hosts always point `LIBBUN_PLUGIN_PATH` at the `.so`.
For older helper-backed bundles, set `LIBBUN_RUNTIME_NATIVE_PATH` only when
testing or replacing a modified helper build. The helper process is an
implementation detail of those releases, not a downstream API commitment.

Hosts should prefer `DynamicBunRuntime::load(...)` with an exact bundled path,
`DynamicBunRuntime::initialize_with_bundled_plugin(...)`, or
`DynamicBunRuntime::initialize_with_plugin_dir(...)`. These APIs honor
`LIBBUN_PLUGIN_PATH` as the replacement override and do not inspect runtime
plugin caches.

Manual macOS bundling example when `download-plugin` is not used:

```sh
native_version=v0.1.5
target=aarch64-apple-darwin
curl -LO "https://github.com/enki/libbun/releases/download/${native_version}/libbun-plugin-native-${native_version}-${target}.tar.zst"
mkdir -p dist/bin
tar --zstd -xf "libbun-plugin-native-${native_version}-${target}.tar.zst" -C dist/bin
```

Linux setup is the same except for the target name and `.so` filename:

```sh
native_version=v0.1.5
target=aarch64-unknown-linux-gnu
curl -LO "https://github.com/enki/libbun/releases/download/${native_version}/libbun-plugin-native-${native_version}-${target}.tar.zst"
mkdir -p dist/bin
tar --zstd -xf "libbun-plugin-native-${native_version}-${target}.tar.zst" -C dist/bin
```

Minimal dynamic-loading example:

```rust
use libbun::dynamic::DynamicBunRuntime;
use libbun::{
    BunEmbeddingRuntime, BunModuleSpec, BunRuntimeConfig, ExportCallResult,
    ProviderCallResult, StructuralValue,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_binary_path = std::env::current_exe()?;
    let config = BunRuntimeConfig::new("example-host", std::env::current_dir()?);
    let mut runtime =
        DynamicBunRuntime::initialize_with_bundled_plugin(config, host_binary_path)?;

    let module = runtime.load_module(BunModuleSpec::Source {
        module_id: "provider".to_string(),
        source: r#"
            export function run(input) {
                return { ok: true, input };
            }
        "#
        .to_string(),
    })?;

    let result = runtime.call_export(
        &module,
        "run",
        StructuralValue(json!({ "value": 7 })),
    )?;

    assert_eq!(
        result,
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(json!({
            "ok": true,
            "input": { "value": 7 }
        }))))
    );

    runtime.shutdown()?;
    Ok(())
}
```

If `LIBBUN_PLUGIN_PATH` is unset and no bundled plugin exists next to the host
binary or in the configured plugin directory, initialization fails with an error
naming the expected plugin filename and directory. If the plugin ABI does not
match the facade ABI, initialization fails before a runtime is created.

If you redistribute the native plugin binary, pass through the matching
`SOURCE.txt`, `NOTICE.txt`, `licenses.json`, source archive, and checksum file
from the same GitHub Release. Keep the plugin replaceable by user-controlled
path or configuration.

## Vendored Bun

Bun source is tracked at `vendor/bun`. The snapshot is created from upstream
Git history with `git archive`, so it excludes nested `.git` metadata and local
build artifacts. Bun build-time source dependencies needed by the Rust crates,
including `lolhtml`, are vendored under `vendor/bun/vendor`.

Update to a new upstream ref:

```sh
scripts/update-vendored-bun.sh <ref>
```

Verify the vendored snapshot:

```sh
scripts/verify-vendored-bun.sh
```

Prepare Bun's generated Rust inputs and check the reusable Rust runtime crates:

```sh
scripts/check-vendored-bun-rust.sh
```

That script runs Bun configure/codegen inside `vendor/bun`, rewrites generated
artifact identity to the pinned `BUN_SOURCE_COMMIT`, checks `bun_jsc` plus
`bun_runtime`, and type-checks the `native/` adapter with Bun's pinned nightly
toolchain.

## Native Adapter

`native/` contains the nightly-only adapter that implements `BunEmbeddingRuntime`
over vendored Bun/JSC crates. It is kept out of the default crate so downstream
users can depend on the stable facade without pulling Bun's build toolchain into
their normal Rust build. It is an internal implementation crate for the dynamic
plugin, not a downstream dependency surface.

Run native adapter integration tests against Bun's C++/JSC objects:

```sh
scripts/prepare-native-bun-link.sh
LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 test --manifest-path native/Cargo.toml --features internal-adapter
```

The native link manifest is prepared from Bun's release profile only so internal
JS builtins are embedded in the linked plugin instead of loaded from a developer
build directory at runtime. Debug-profile manifests, `bun-debug`, `build/debug`,
and debug WebKit/JSC inputs are rejected by the preparation script and Cargo
build scripts. The manifest intentionally records Bun's C/C++ object archive and
prebuilt WebKit/JSC static libraries, but not Bun's Rust staticlib. The adapter
depends on the vendored Rust crates directly so Rust global state is not linked
twice into the test host.

## Dynamic Plugin

`plugin/` builds `libbun-plugin-native` as a `cdylib`. This is the only supported
way for downstream applications to use the native Bun/JSC implementation.

Build the macOS in-process plugin after preparing the native link manifest:

```sh
scripts/prepare-native-bun-link.sh
LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
```

Build the Linux in-process plugin with PIC WebKit inputs:

```sh
scripts/prepare-native-bun-link.sh
scripts/fetch-webkit-pic-artifact.sh --target x86_64-unknown-linux-gnu \
  --manifest vendor/bun/build/release/libbun_native_link_manifest.txt \
  --out vendor/bun/build/release/libbun_native_link_manifest.pic.txt
LIBBUN_NATIVE_LINK_MANIFEST=vendor/bun/build/release/libbun_native_link_manifest.pic.txt \
  LIBBUN_NATIVE_LINK_BUN=1 \
  RUSTFLAGS="-C link-arg=-fuse-ld=lld" \
  cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml --features linux-in-process
```

Older helper-backed Linux bundles can still be built after preparing the same
native link manifest:

```sh
scripts/prepare-native-bun-link.sh
cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path runtime/Cargo.toml
```

Use `LIBBUN_NATIVE_BUN_BUILD_DIR=vendor/bun/build/native-$(uname -m)-$(uname -s)`
to keep platform-specific Bun native build products outside the default
`vendor/bun/build/release` directory. The native plugin link path is release
profile only; do not use debug Bun profiles for libbun plugin artifacts.

Rust hosts can enable the facade's `dynamic-loading` feature and load the plugin
at runtime with `libbun::dynamic::DynamicBunRuntime`. `BunHost` initialization
through the trait reads `LIBBUN_PLUGIN_PATH`; hosts that want explicit path
control can call `DynamicBunRuntime::load(path, config)` directly.

## Native Plugin Releases

Official native plugin binaries are produced by GitHub Actions and published as
GitHub Release assets with matching source, notice, license inventory, source
instructions, and checksum files.

Before creating a release tag, run the local preflight:

```sh
scripts/preflight-native-plugin-release.sh v0.1.5
```

On Linux, set `LIBBUN_NATIVE_RUNTIME_MODE=in-process` to preflight the PIC
single-plugin release path. Without that override, the preflight keeps the
older helper-backed path available for diagnostics.

After the preflight passes, commit the release changes and push the annotated
release tag:

```sh
git add .
git commit -m "Prepare native plugin release"
scripts/create-native-plugin-release.sh v0.1.5
```

Pushing the tag triggers `.github/workflows/release-native-plugin.yml`. Inspect
the completed workflow and GitHub Release assets before announcing the release.
