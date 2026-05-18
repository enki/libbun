# ADR-2041: libbun Mature Publication and Downstream Consumption Shape

Status: Proposed
Date: 2026-05-18

`libbun` has the right architectural boundary: downstream applications should
depend on a small Rust facade and load Bun/JSC/WebKit native code only through a
replaceable dynamic plugin. The current implementation is not yet at the
publication shape a mature project should present to ordinary users.

The target state is intentionally boring for downstream hosts:

- `cargo add libbun` installs only the facade crate;
- GitHub Releases provide prebuilt native plugins for common platforms;
- every native plugin binary has matching source, notices, license inventory,
  source instructions, checksums, and eventually signatures;
- users can replace the plugin by path without relinking their host
  application;
- downstream distributors can pass through the upstream compliance bundle
  mechanically.

## Decision

`libbun` will publish only the facade crate to crates.io.

The crates.io package must not contain `vendor/bun`, `native/`, `plugin/`,
release archives, build caches, or any source path that causes Cargo users to
compile or statically link Bun/JSC/WebKit during normal dependency resolution.

The `libbun-native` and `libbun-plugin-native` crates remain internal release
inputs. They are not crates.io packages unless a later ADR defines a registry
shape that preserves the dynamic-plugin-only boundary and satisfies size,
source, and compliance constraints.

Official native plugin binaries are published only as GitHub Release assets.
The mature baseline release matrix is:

```text
libbun-plugin-native-vX.Y.Z-aarch64-apple-darwin.tar.zst
libbun-plugin-native-vX.Y.Z-x86_64-apple-darwin.tar.zst
libbun-plugin-native-vX.Y.Z-x86_64-unknown-linux-gnu.tar.zst
libbun-plugin-native-vX.Y.Z-aarch64-unknown-linux-gnu.tar.zst
```

The first broadly usable public release must include at least:

```text
libbun-plugin-native-vX.Y.Z-aarch64-apple-darwin.tar.zst
libbun-plugin-native-vX.Y.Z-x86_64-unknown-linux-gnu.tar.zst
libbun-plugin-native-vX.Y.Z-aarch64-unknown-linux-gnu.tar.zst
```

A macOS-only release is acceptable as an internal smoke release, but it is not
the target public consumption shape.

The facade crate version and native plugin release version should match until
there is a compelling reason to split version lines. The plugin ABI version is
separate and must remain explicitly discoverable through the plugin handshake
and release inventory.

## Cargo Publication Requirements

Before removing `publish = false` from the root crate, `libbun` must:

- add crates.io metadata: description, repository, readme, license, keywords,
  and categories;
- add an explicit `include = [...]` allowlist for the published facade package;
- prove `cargo package --list` does not include `vendor/`, `native/`,
  `plugin/`, `dist/`, generated Bun outputs, GitHub release archives, or local
  build products;
- run `cargo publish --dry-run`;
- keep the default feature set free of native/plugin dependencies;
- keep the optional `dynamic-loading` feature dependent only on ordinary Rust
  dynamic loading support such as `libloading`;
- document that the native plugin is acquired from GitHub Releases, not Cargo.

The published facade crate may include:

- `src/` facade code;
- facade integration tests that do not require the native plugin;
- README/license files;
- ADR excerpts or links only if they are useful to consumers and do not bloat
  the package.

## Native Plugin Release Requirements

The GitHub Actions release workflow must become a real multi-platform release
factory. For each platform job it must:

- install the exact native build prerequisites required by vendored Bun;
- prepare Bun native link inputs from the checked-out tag;
- build a dynamic library plugin, not a static library or host executable;
- run a dynamic-loader smoke test through `LIBBUN_PLUGIN_PATH`;
- verify the replacement build path;
- package the binary with the exact release version and target triple;
- attach the platform binary only if that platform's checks pass.

The release as a whole must also publish the same-version compliance bundle
from ADR-2039 and ADR-2040:

```text
libbun-plugin-native-vX.Y.Z-source.tar.zst
libbun-plugin-native-vX.Y.Z-NOTICE.txt
libbun-plugin-native-vX.Y.Z-licenses.json
libbun-plugin-native-vX.Y.Z-SOURCE.txt
libbun-plugin-native-vX.Y.Z-checksums.txt
```

Checksums should be signed once the unsigned release path is reliable. The
preferred signing shape is a keyless or maintainer-key signature for the
checksum file, plus provenance/attestation if GitHub Actions support is added.

## Documentation Requirements

The README must include a downstream usage path, not only maintainer release
commands. At minimum it must show:

- `cargo add libbun --features dynamic-loading`;
- how to choose and download the matching plugin asset for the host platform;
- how to unpack the asset;
- how to set `LIBBUN_PLUGIN_PATH`;
- a minimal Rust example that initializes `BunHost` or
  `DynamicBunRuntime`;
- what error to expect when the plugin is missing or ABI-incompatible;
- what downstream distributors must pass through if they redistribute the
  plugin binary.

The README must also explain the publication split in plain terms: Cargo
provides the Rust facade; GitHub Releases provide replaceable native plugins
and compliance assets.

## Migration Plan

1. Stabilize the current macOS release job until it completes and attaches all
   required assets for `aarch64-apple-darwin`.
2. Add Linux x86_64 release support and make it part of the minimum public
   release gate.
3. Add README downstream installation and runtime-loading examples.
4. Add a release asset verifier script that checks a GitHub Release contains
   every required binary and compliance asset before the release is announced.
5. Add Cargo package metadata and an explicit root crate `include` allowlist.
6. Run `cargo package --list` and `cargo publish --dry-run` in CI.
7. Publish `libbun` facade to crates.io only after the package list and dry run
   prove the crate does not include native/plugin/vendor material.
8. Add macOS x86_64 and Linux aarch64 plugin releases.
9. Add checksum signing and, if practical, GitHub provenance attestations.

## Non-Goals

This ADR does not require publishing `libbun-native` or `libbun-plugin-native`
to crates.io.

This ADR does not require Cargo to download native plugin binaries. Runtime
plugin acquisition remains explicit so hosts can control installation,
replacement, and redistribution.

This ADR does not require Windows support for the first public release. Windows
can be added after the Unix-like dynamic plugin path is reliable.

## Consequences

The facade crate becomes easy to consume from ordinary Rust projects without
surprising native builds or LGPL-bearing static links.

The native release workflow becomes the critical path for real adoption. A
GitHub Release without Linux x86_64 support is not enough for common server
deployment.

Downstream users get a simple support matrix: Cargo for the facade, GitHub
Release assets for native capability, and a documented environment/config path
for plugin replacement.

Maintainers must treat failed release workflows as release blockers. Moving a
tag after failed smoke attempts is acceptable before announcement, but a mature
release process should eventually publish from immutable tags only after
preflight and CI are reliable.

## Acceptance Criteria

This ADR can move to `docs/done/` when:

- `libbun` is publishable as a small facade-only crates.io package;
- `cargo package --list` proves the published crate excludes native/plugin/Bun
  vendor material;
- `cargo publish --dry-run` passes in CI;
- GitHub Releases produce at least macOS aarch64, Linux x86_64, and Linux
  aarch64 native plugin binaries;
- the release contains source, notice, license inventory, source instruction,
  checksum, and binary assets from the same tag;
- README downstream usage instructions are complete enough for a Rust host to
  install the crate, download a plugin, set `LIBBUN_PLUGIN_PATH`, and run a
  minimal provider.
