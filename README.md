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
implementation from a GitHub Release plugin asset.

Add the facade:

```sh
cargo add libbun --features dynamic-loading
```

Download the plugin asset that matches the host platform from the same
`libbun` GitHub Release as the facade version. The currently supported native
plugin release target is:

```text
libbun-plugin-native-vX.Y.Z-aarch64-apple-darwin.tar.zst
```

Linux plugin binaries are not published yet. Current upstream WebKit/JSC/WTF
prebuilt static archives contain TLS relocations that cannot be linked into a
Linux shared object. Linux plugin releases require PIC-compatible WebKit
artifacts built from source or provided by an upstream PIC release.

For example:

```sh
version=v0.1.0
target=aarch64-apple-darwin
curl -LO "https://github.com/enki/libbun/releases/download/${version}/libbun-plugin-native-${version}-${target}.tar.zst"
tar --zstd -xf "libbun-plugin-native-${version}-${target}.tar.zst"
export LIBBUN_PLUGIN_PATH="$PWD/liblibbun_plugin_native.dylib"
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
    let mut runtime = DynamicBunRuntime::initialize(BunRuntimeConfig::new(
        "example-host",
        std::env::current_dir()?,
    ))?;

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

If `LIBBUN_PLUGIN_PATH` is unset, initialization fails with an error naming that
environment variable. If the plugin ABI does not match the facade ABI,
initialization fails before a runtime is created.

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

The native link manifest intentionally records Bun's C/C++ object archive and
prebuilt WebKit/JSC static libraries, but not Bun's Rust staticlib. The adapter
depends on the vendored Rust crates directly so Rust global state is not linked
twice into the test host.

## Dynamic Plugin

`plugin/` builds `libbun-plugin-native` as a `cdylib`. This is the only supported
way for downstream applications to use the native Bun/JSC implementation.

Build the plugin after preparing the native link manifest:

```sh
scripts/prepare-native-bun-link.sh
LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
```

Use `LIBBUN_NATIVE_BUN_BUILD_DIR=vendor/bun/build/native-$(uname -m)-$(uname -s)`
to keep platform-specific Bun native build products outside the default
`vendor/bun/build/debug` directory.

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
scripts/preflight-native-plugin-release.sh v0.1.0
```

After the preflight passes, commit the release changes and push the annotated
release tag:

```sh
git add .
git commit -m "Prepare native plugin release"
scripts/create-native-plugin-release.sh v0.1.0
```

Pushing the tag triggers `.github/workflows/release-native-plugin.yml`. Inspect
the completed workflow and GitHub Release assets before announcing the release.
