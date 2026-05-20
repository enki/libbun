# ADR-2047: libbun download-plugin Build Feature

Status: Done
Date: 2026-05-19

Updated: 2026-05-20 by ADR-2049. `download-plugin` remains a convenience for
crate development and experiments, but it is not the final product topology for
hosts that ship a native binary. Product hosts must bundle the verified plugin
relative to the shipped binary, and development hosts must resolve the
development plugin relative to the development binary or target artifact rather
than relying on persistent runtime cache state.

`libbun` should be usable as a Cargo dependency without requiring downstream
users to keep a sibling `../libbun` checkout, manually download GitHub Release
assets, or learn the native plugin package layout. The crate called `libbun`
should be able to make the matching native Bun plugin available for local use.

The dynamic plugin boundary from ADR-2038 remains correct: downstream host
executables must not statically link Bun/JSC/WebKit. The missing piece is
artifact acquisition. ADR-2046 added release metadata, cache resolution, and a
runtime installer, but its primary path still required a separate install step
or a pre-populated cache. That is not the desired Cargo experience.

Publishing prebuilt plugin bytes directly in target-specific crates would be
conceptually clean, but the current release bundles are too large for ordinary
crates.io publication. The v0.1.1 compressed plugin bundles are about 30 MiB on
macOS arm64 and 45-47 MiB on Linux, while crates.io package size limits are far
smaller than that. Source-building Bun/JSC/WebKit as the default dependency
path would make every downstream build inherit the full native build stack and
is not acceptable as the normal path.

## Decision

`libbun` will add an opt-in Cargo feature named `download-plugin`.

When `download-plugin` is enabled, `libbun`'s build script downloads the exact
official native plugin release asset for the Cargo target, verifies it against
a known checksum, extracts it into Cargo's `OUT_DIR`, and emits compile-time
environment variables that let the runtime resolver find the plugin.

The intended downstream dependency for applications that want automatic native
plugin availability is:

```toml
libbun = { version = "0.1.3", features = ["dynamic-loading", "download-plugin"] }
```

The default feature set should remain conservative until the networked build
path has enough operational history. Hosts that cannot allow network access
during Cargo builds can depend on `libbun` without `download-plugin`, install
the plugin through their own package manager/release process, or set
`LIBBUN_PLUGIN_PATH`.

## Upstream Consumption Modes

`libbun` supports two upstream consumption modes.

### Download Mode

Download mode is for upstream crates whose Cargo builds are allowed to fetch
verified release artifacts. They depend on:

```toml
libbun = { version = "0.1.3", features = ["dynamic-loading", "download-plugin"] }
```

In this mode, `libbun` owns target selection, release asset naming, checksum
verification, extraction, and runtime path emission. The upstream crate should
not know the `libbun` repository layout or probe development build
directories. If the network is unavailable, the upstream build can provide
`LIBBUN_PLUGIN_ARCHIVE`, `LIBBUN_PLUGIN_BUNDLE_DIR`, or `LIBBUN_PLUGIN_PATH`
instead.

### No-Download Mode

No-download mode is for upstream crates, package managers, hermetic CI systems,
or app release processes that fetch native artifacts outside Cargo. They depend
on:

```toml
libbun = { version = "0.1.3", features = ["dynamic-loading"] }
```

In this mode, `libbun` never downloads during the Cargo build. The upstream
package must arrange for one of the documented runtime paths:

```text
LIBBUN_PLUGIN_PATH
LIBBUN_HOME/vX.Y.Z/<target>/
~/.cache/libbun/vX.Y.Z/<target>/
```

This mode is the right fit when a package manager wants all downloads declared
with external hashes, when an application bundles the plugin beside its own
binary, or when CI installs the native plugin as a separate setup step.

Both modes share the same runtime resolver and both preserve
`LIBBUN_PLUGIN_PATH` as the first-priority user replacement override.

## Build-Time Behavior

The `download-plugin` build path must:

- select the plugin asset by Cargo `TARGET`, not by build host;
- use the `libbun` crate version to choose the matching GitHub Release tag;
- know the expected SHA-256 checksum for each supported target asset;
- download only when the extracted plugin is missing or the downloaded archive
  checksum does not match the expected checksum;
- verify the compressed plugin tarball before extraction;
- extract into `OUT_DIR/libbun-plugin-native/<version>/<target>/`;
- include the Linux helper executable when the target bundle contains one;
- also install `SOURCE.txt`, `NOTICE.txt`, `licenses.json`, `checksums.txt`,
  and `libbun-native-bundle.json` next to the plugin when available;
- emit compile-time environment variables such as:

```text
LIBBUN_BUILD_PLUGIN_DIR=<OUT_DIR>/libbun-plugin-native/vX.Y.Z/<target>
LIBBUN_BUILD_PLUGIN_PATH=<...>/liblibbun_plugin_native.dylib
LIBBUN_BUILD_PLUGIN_TARGET=<target>
LIBBUN_BUILD_PLUGIN_VERSION=vX.Y.Z
```

The build script must fail loudly if the release asset for the crate version
does not exist, the target is unsupported, the checksum is missing, the checksum
does not match, or the extracted bundle does not contain the expected plugin
file.

## Runtime Resolver Order

With or without `download-plugin`, the runtime resolver order is:

1. `LIBBUN_PLUGIN_PATH`, for explicit user replacement.
2. Build-time downloaded plugin path emitted by the `download-plugin` feature.
3. Explicit `LIBBUN_HOME` or API-provided cache root.
4. Default per-user cache path.
5. Clear missing-plugin error naming the crate version, target triple, expected
   asset, and supported remedies.

`DynamicBunRuntime::initialize` must remain non-downloading. Downloading is a
build-time action only when `download-plugin` is enabled. Runtime loading may
extract or copy from already-built artifacts only if a later ADR decides that
embedding is required; this ADR does not require runtime network access.

## Overrides and Offline Builds

The build script must support explicit offline or pre-fetched inputs:

- `LIBBUN_PLUGIN_PATH` may point to a user-provided replacement plugin and
  should let builds skip network download when the host wants that behavior.
- `LIBBUN_PLUGIN_BUNDLE_DIR` may point to an already extracted bundle
  containing the plugin, metadata, and optional Linux helper.
- `LIBBUN_PLUGIN_ARCHIVE` may point to a pre-downloaded official release
  archive to verify and extract.
- `LIBBUN_DOWNLOAD_PLUGIN=0` disables network download and fails with an
  actionable message if no override/pre-fetched bundle is available.

These overrides are required so package managers, CI systems, and offline
builders can make all fetches explicit while still using the same `libbun`
runtime resolver.

## Checksum Source

The checksum table for official release assets must be committed to the
`libbun` crate source for each published version that supports
`download-plugin`.

The build script may additionally download the GitHub Release checksum file for
metadata installation, but trust for the plugin archive must come from the
crate's committed checksum table. This prevents a build from trusting whatever
checksum file happens to be served by the network at build time.

When cutting a new release, release automation must update the checksum table
after the native plugin assets are produced and before publishing the matching
crate to crates.io. If that requires a two-phase release, the tag used for the
published crate must point at the commit containing the final checksum table.

## Release Sequence

The correct release sequence is:

1. Build and publish draft/native plugin assets for the release candidate.
2. Record the exact target asset checksums in the `libbun` source tree.
3. Verify `download-plugin` builds from a clean checkout for each supported
   target, using the committed checksums.
4. Publish the GitHub Release with plugin assets and compliance files.
5. Publish the matching `libbun` crate version to crates.io.

The crate must not be published before the matching release assets exist and
the checksum table is present.

## Non-Goals

This ADR does not make Bun/JSC/WebKit part of the downstream host executable.
The plugin remains a replaceable dynamic library loaded at runtime.

This ADR does not publish oversized native plugin bytes inside the `libbun`
crate.

This ADR does not make runtime network access acceptable. Network access, when
enabled, occurs during Cargo build through the explicit `download-plugin`
feature.

This ADR does not remove manual plugin replacement. `LIBBUN_PLUGIN_PATH`
remains the first resolver entry.

## Consequences

Applications that choose `download-plugin` get the desired Cargo experience:
depending on `libbun` can make the matching native plugin available without a
sibling checkout or a separate user install command.

Builds using `download-plugin` are networked by default and therefore less
friendly to some package managers and hermetic CI systems. The override and
offline paths are mandatory, not nice-to-have.

The release process becomes two-phase: native assets must exist before the
published crate can contain their checksums. This is operationally more complex
than a facade-only crate, but it keeps the dynamic plugin boundary intact while
making the developer experience match the crate name.

## Implementation Plan

1. Remove the target-specific platform crate experiment.
2. Keep `libbun::release` metadata and resolver APIs, but add build-time plugin
   path support to the resolver.
3. Add a `download-plugin` feature to `Cargo.toml`.
4. Extend `build.rs` to select target assets, verify committed checksums,
   download or use override inputs, extract into `OUT_DIR`, and emit plugin
   path environment variables.
5. Add a committed checksum table for supported release assets.
6. Add tests for resolver precedence, build-time plugin path handling, missing
   checksum errors, and offline override behavior.
7. Update README to show `libbun = { features = ["dynamic-loading",
   "download-plugin"] }` as the automatic local-use path.
8. Update release scripts to enforce the native-assets-before-crate-publish
   sequence.

## Implementation

This ADR is implemented by:

- the `download-plugin` feature in `Cargo.toml`;
- `build.rs`, which selects the plugin by Cargo `TARGET`, accepts
  `LIBBUN_PLUGIN_PATH`, `LIBBUN_PLUGIN_BUNDLE_DIR`,
  `LIBBUN_PLUGIN_ARCHIVE`, and `LIBBUN_DOWNLOAD_PLUGIN=0` overrides,
  verifies release archives against committed checksums, extracts the plugin
  bundle into `OUT_DIR`, downloads/verifies release metadata, and emits
  `LIBBUN_BUILD_PLUGIN_*` environment variables;
- `src/plugin_checksums.rs` and `src/plugin_checksums_table.in`, which provide
  the committed checksum table used by the build script;
- `libbun::release::build_time_plugin_path`,
  `libbun::release::build_time_plugin_dir`, and resolver support for the
  `BuildTime` source after `LIBBUN_PLUGIN_PATH` and before cache lookup;
- README documentation for the automatic download mode and no-download mode;
- `scripts/update-plugin-checksums.sh`, which records release asset checksums
  from the GitHub Release checksum file before crate publication.

Release `v0.1.2` was produced in two phases:

1. native plugin assets were built by GitHub Actions from commit `5a1cce5`;
2. checksums were recorded in commit `d002399`;
3. the `v0.1.2` tag was force-moved to `d002399`;
4. the final GitHub Actions release workflow completed successfully from
   `d002399`;
5. `libbun 0.1.2` was published to crates.io from the same commit.

Verification performed:

- `cargo test`;
- `cargo test --features dynamic-loading,plugin-installer`;
- `cargo test --features download-plugin --test release` against the real
  `v0.1.2` GitHub Release asset;
- `cargo publish --dry-run`;
- `scripts/verify-release-assets.sh --version v0.1.2`;
- `git diff --check`;
- successful crates.io publication of `libbun 0.1.2`.

Follow-up correction:

`libbun 0.1.2` was published with checksums from an earlier `v0.1.2` native
release run. The final `v0.1.2` GitHub Release assets were regenerated after
the checksum table commit, so the published crate's `download-plugin` feature
rejected the final macOS arm64 asset. Because crates.io packages are immutable,
`libbun 0.1.3` is a facade-only patch release that points at the already
published `v0.1.2` native plugin release and records the final release asset
checksums.

## Acceptance Criteria

- A downstream crate can depend on `libbun` with `dynamic-loading` and
  `download-plugin` and run without a sibling checkout or manual plugin
  installation.
- `DynamicBunRuntime::initialize` loads the build-time downloaded plugin when
  no `LIBBUN_PLUGIN_PATH` override is set.
- `LIBBUN_PLUGIN_PATH` always wins over the build-time plugin path.
- The build script verifies the plugin archive against a committed checksum
  before extraction.
- The build script supports an offline/pre-fetched bundle path.
- Missing or unsupported targets fail with actionable errors.
- The published facade crate still does not contain vendored Bun source,
  native build products, or oversized plugin binaries.
