# ADR-2046: libbun Released Native Plugin Resolution

Status: Done
Date: 2026-05-19

ADR-2038 requires downstream hosts to consume Bun/JSC/WebKit native code only
through a replaceable dynamic plugin. ADR-2039 requires official plugin
binaries to be distributed as GitHub Release assets with matching source,
notice, license inventory, source instructions, and checksums.

The first crates.io publication completed the facade side of that split:
ordinary Rust projects can depend on `libbun = "0.1.2"` without vendoring Bun or
linking `libbun-native`. The remaining gap is runtime acquisition and discovery
of the native plugin. Downstream hosts should not hardcode a sibling checkout
such as `../libbun/plugin/target/...`, because that only works for maintainer
workspaces and fails for ordinary users, packaged binaries, CI environments,
and downstream distributors.

The release asset is the product boundary for native Bun support. The facade
crate must make that boundary easy to consume without making Cargo responsible
for building, vendoring, or installing the native runtime.

## Decision

`libbun` will provide first-class metadata and resolver APIs for official native
plugin release assets.

The crates.io `libbun` package remains a facade crate. It must not include
`vendor/bun`, native plugin binaries, helper binaries, release tarballs, or
source archives. It must not cause Cargo users to compile or statically link
Bun/JSC/WebKit as part of normal dependency resolution.

The facade crate may expose release metadata describing the native plugin
artifact that matches the crate version:

```text
facade crate version: 0.1.2
native release tag: v0.1.2
release repository: enki/libbun
supported host target triples:
  aarch64-apple-darwin
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
asset name:
  libbun-plugin-native-v0.1.2-<target>.tar.zst
plugin filename:
  liblibbun_plugin_native.dylib
  liblibbun_plugin_native.so
```

The facade crate will expose a resolver API that downstream hosts can call
instead of implementing ad hoc path discovery. The resolver order is:

1. Explicit plugin path from `LIBBUN_PLUGIN_PATH`.
2. Explicit cache/home override from a documented `libbun` environment variable
   or host-supplied resolver configuration.
3. The standard per-user release cache path for the current crate version and
   host target.
4. A clear error explaining the missing plugin, the expected release asset, and
   how to install it or set `LIBBUN_PLUGIN_PATH`.

The resolver must never probe sibling checkouts, repository-relative development
build directories, or `../libbun/plugin/target/...` as a production fallback.
Development build paths may remain in repository scripts and tests, but they
must not be part of the published downstream resolution contract.

`DynamicBunRuntime::initialize` uses the released-plugin resolver so the trait
initialization path honors `LIBBUN_PLUGIN_PATH` and the standard release cache.
Hosts that want explicit path control can still call `DynamicBunRuntime::load`.

## Installer Shape

`libbun` will provide either an optional installer API or a small companion
command that downstream hosts can wrap. The installer must:

- choose the correct target triple for the host;
- download the matching GitHub Release plugin tarball;
- download or otherwise obtain the matching checksum file from the same release;
- verify the downloaded asset against the checksum file before installation;
- extract the plugin into a stable per-user cache directory;
- preserve or install the release's `SOURCE.txt`, `NOTICE.txt`,
  `licenses.json`, and checksum metadata alongside the plugin when practical;
- return or print the resolved plugin path for host integration;
- avoid git clones, sibling checkouts, vendoring, or local plugin builds.

The default cache layout should be deterministic and versioned, for example:

```text
~/.cache/libbun/v0.1.2/aarch64-apple-darwin/
  liblibbun_plugin_native.dylib
  libbun-native-bundle.json
  SOURCE.txt
  NOTICE.txt
  licenses.json
  checksums.txt
```

Linux cache entries contain both the dynamic plugin and helper executable:

```text
~/.cache/libbun/v0.1.2/x86_64-unknown-linux-gnu/
  liblibbun_plugin_native.so
  libbun-runtime-native
  libbun-native-bundle.json
  SOURCE.txt
  NOTICE.txt
  licenses.json
  checksums.txt
```

Hosts still load the plugin path. The Linux helper remains an implementation
detail of the plugin bundle unless a host deliberately overrides
`LIBBUN_RUNTIME_NATIVE_PATH` for testing or replacement.

## Downstream Host Contract

Downstream hosts should depend on the facade crate from crates.io:

```toml
libbun = { version = "0.1.2", features = ["dynamic-loading"] }
```

They should resolve the plugin through the `libbun` resolver or an equivalent
host wrapper around the official release cache. A typical downstream order is:

1. Use an explicit user/admin configured plugin path.
2. Use the `libbun` release cache for the crate-pinned version and host target.
3. If missing, fail with instructions to run the host's install command or set
   `LIBBUN_PLUGIN_PATH`.

Downstream hosts must not require ordinary users to clone the `libbun`
repository, build `./plugin`, or keep a sibling checkout next to the host
workspace. Direct checkout builds are maintainer and contributor workflows, not
the published consumption path.

Downstream hosts that redistribute the plugin binary must pass through the
matching compliance artifacts from the same GitHub Release, as required by
ADR-2039 and ADR-2040. Hosts that only point users at the upstream installer or
release asset should still keep the plugin replaceable by user-controlled path
or configuration.

## Non-Goals

This ADR does not publish `libbun-native` or `libbun-plugin-native` to
crates.io.

This ADR does not make Cargo download or install native plugin binaries during
`cargo build`. Native plugin acquisition remains an explicit runtime
installation step so hosts can control replacement, redistribution, and
compliance handling.

This ADR does not require downstream hosts to redistribute the plugin. A host
may require users to install the plugin separately or run a host-provided
installer command.

This ADR does not remove maintainer development workflows that build the plugin
from a checkout. It removes those paths from the downstream user contract.

## Consequences

The crates.io facade becomes enough for source-level integration, and GitHub
Release assets become enough for native runtime installation.

Downstream hosts no longer need to know the `libbun` repository layout, Cargo
target directory structure, or maintainer workspace conventions.

The native plugin release metadata becomes public API. Asset naming, supported
targets, checksum files, and cache layout need compatibility discipline.

The installer/resolver layer becomes the right place to add future checksum
signatures, GitHub provenance verification, mirror support, offline
installation, and enterprise policy hooks.

## Implementation Plan

1. Add a `libbun::release` or `libbun::plugin_asset` module behind an optional
   feature if new dependencies are needed.
2. Expose typed release metadata for the crate version, release tag, repository,
   target triple, asset name, plugin filename, and standard cache location.
3. Expose a resolver that honors `LIBBUN_PLUGIN_PATH`, cache overrides, and the
   standard release cache before returning an actionable missing-plugin error.
4. Add an installer API or companion CLI that downloads the release asset,
   verifies checksums, extracts to the cache, and returns the plugin path.
5. Update README downstream instructions to prefer the resolver/installer flow
   over manual `curl` commands.
6. Add tests for target mapping, asset naming, cache layout, missing-plugin
   errors, and `LIBBUN_PLUGIN_PATH` precedence.
7. Update downstream examples to remove sibling checkout probing and use the
   released-plugin resolver/cache path.
8. Keep direct `./plugin` build instructions only in maintainer documentation.

## Implementation

This ADR is implemented by:

- `libbun::release`, which exposes native plugin release metadata, current
  target mapping, asset naming, GitHub Release URLs, cache layout, checksum
  helpers, and a resolver API;
- `NativePluginResolver`, which honors explicit plugin paths, `LIBBUN_HOME`,
  the default `~/.cache/libbun/<version>/<target>/` cache, and actionable
  missing-plugin errors;
- the optional `plugin-installer` feature, which adds
  `NativePluginInstall`/`install_native_plugin` for downloading the official
  release asset, verifying it against the release checksum file, extracting it
  into the versioned cache, and installing `SOURCE.txt`, `NOTICE.txt`,
  `licenses.json`, and `checksums.txt`;
- `DynamicBunRuntime::initialize`, which resolves the plugin through
  `libbun::release::resolve_native_plugin()` instead of requiring only
  `LIBBUN_PLUGIN_PATH`;
- README downstream instructions for the release cache, `LIBBUN_HOME`,
  `LIBBUN_PLUGIN_PATH`, manual cache installation, and optional installer API;
- `tests/release.rs`, which covers target metadata, asset naming, cache
  resolution, explicit path precedence, missing-plugin diagnostics, checksum
  parsing/verification, and an ignored network test for the real release
  installer.

The first published version with this resolver and installer API is `0.1.2`.

## Acceptance Criteria

- A fresh downstream project can use `libbun = "0.1.2"` or later without a
  sibling `libbun` checkout.
- The downstream project can install or resolve the matching native plugin from
  official GitHub Release assets.
- The plugin path loaded at runtime is explicit, cached, or user configured,
  never inferred from `../libbun`.
- Missing plugin errors name the expected version, target, release asset, and
  installation remedy.
- The facade crate package remains small and does not include vendored Bun or
  native build inputs.
