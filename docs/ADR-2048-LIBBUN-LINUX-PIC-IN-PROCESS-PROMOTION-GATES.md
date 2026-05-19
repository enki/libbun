# ADR-2048: Linux PIC In-Process Promotion Gates

Status: Proposed
Date: 2026-05-19

## Context

ADR-2038 defines the public downstream contract: hosts use `libbun` through a
replaceable dynamic plugin and do not link Bun/JSC/WebKit directly.

ADR-2044 and ADR-2045 allow Linux releases to use a helper-backed runtime
bundle while Linux shared-object-compatible Bun/WebKit inputs are not proven.
Those ADRs intentionally do not make helper IPC a permanent Linux API.

ADR-2042 was superseded after proving that the original Linux release failure
was caused by native inputs compiled for static executable linkage, especially
prebuilt WebKit/JSC/WTF archives with local-exec TLS relocations. It also added
useful guardrails and a reproducible PIC mode for Bun-owned/native dependency
inputs.

On 2026-05-19, a Linux arm64 experiment using PIC WebKit artifacts from the
`enki/WebKit` fork's `libbun-pic-5488984d` branch and workflow run
`26077123752` proved that the in-process path can work at least on one Linux
architecture:

- the rewritten native link manifest passed
  `scripts/inspect-linux-native-relocations.sh`;
- GNU `ld` was killed during the large final link, but `lld` completed it;
- the resulting `liblibbun_plugin_native.so` dynamically loaded under Ubuntu
  24.04 in `smolvm`;
- `tests/dynamic_plugin.rs::dynamic_plugin_provider_flow` passed through
  `LIBBUN_PLUGIN_PATH`;
- two missing link requirements were identified: Linux `libubsan` for PIC debug
  WebKit inputs and Bun's `bun_platform` shims such as `sys_epoll_pwait2`.

That is enough to reopen the in-process Linux strategy. It is not yet enough to
retire the helper bundle or publish Linux assets as generally supported,
because x86_64 is unproven, the WebKit PIC artifacts are currently Actions
artifacts rather than durable release inputs, and the passing arm64 smoke test
still emitted a shutdown-time `mimalloc` invalid-pointer diagnostic.

The intended WebKit source for this work is the `enki/WebKit` fork's
`libbun-pic-5488984d` branch, not a fresh ad hoc WebKit build inside the
libbun release workflow. The sibling checkout at `../WebKit` is large
enough that it should be treated as an upstream producer repository, not as
part of libbun's normal local or CI checkout. That fork already has GitHub
Actions capable of producing the PIC WebKit builds libbun needs. Those
workflow artifacts are the right channel for proving whether the PIC inputs
are usable, but they are not a release boundary because they expire and are
not addressable as stable dependency inputs. Once a WebKit PIC snapshot is
close to usable for libbun's Linux plugin release path, the WebKit workflow
must publish durable GitHub Release assets that libbun can fetch by tag,
filename, and checksum.

Phase 1 has started: the artifacts from run `26077123752` were promoted to the
durable `enki/WebKit` release tag
`libbun-webkit-pic-5488984d-20260519`, with archive checksums and metadata.
`scripts/fetch-webkit-pic-artifact.sh` consumes that release and rewrites the
native link manifest reproducibly. `.github/workflows/verify-linux-pic-plugin.yml`
is the non-publishing CI lane for proving both Linux targets before release
promotion.

Future WebKit PIC updates should follow the same producer/consumer split:
develop and build PIC WebKit in `enki/WebKit`; use Actions artifacts only for
trial validation; promote a proven artifact set to a WebKit GitHub Release;
then update libbun to pin that release tag, asset names, and checksums.

## Decision

Linux PIC in-process plugin support may replace the helper-backed Linux bundle
only after it passes explicit promotion gates on both mature Linux targets:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
```

Until those gates pass, the helper-backed bundle remains the conservative Linux
release strategy from ADR-2044 and ADR-2045.

The downstream contract must not change during promotion. Consumers still use:

```text
libbun crate -> dynamic loader -> LIBBUN_PLUGIN_PATH -> liblibbun_plugin_native.so
```

The implementation behind that plugin may change from:

```text
plugin -> helper process -> Bun/JSC/WebKit
```

to:

```text
plugin -> in-process Bun/JSC/WebKit
```

without changing the facade API, plugin filename, target tarball names, Cargo
feature shape, or normal downstream setup instructions.

## Promotion Gates

The in-process Linux path is release-eligible only when every gate below passes
for both Linux targets.

### Durable WebKit Inputs

PIC-compatible WebKit/JSC/WTF artifacts must be durable release inputs from
the `enki/WebKit` fork's `libbun-pic-5488984d` branch, not expiring Actions
artifacts.

The `enki/WebKit` workflow has two distinct responsibilities:

- proof builds: produce Actions artifacts for local, `smolvm`, or
  non-publishing libbun CI validation;
- release builds: attach the same target-specific artifact shape to a stable
  WebKit GitHub Release once a snapshot is selected for libbun release
  consumption.

libbun may consume proof-build artifacts only in explicit experiments. Any
published libbun release, replacement-build check, or compliance source bundle
must consume a WebKit GitHub Release asset or another durable publication
channel with equivalent immutability and checksums.

Each WebKit PIC artifact must have:

- a stable release tag or content-addressed publication channel;
- target-specific archive names;
- SHA-256 checksums;
- source commit and build configuration metadata;
- enough source/build instructions to satisfy the same source-rebuild and
  replacement expectations as the rest of the native plugin release.

The libbun release workflow must pin those artifact names and checksums. It
must not consume mutable "latest successful workflow" artifacts.

### Native Input Verification

For each Linux target, the release job must generate the final native link
manifest and run:

```text
scripts/inspect-linux-native-relocations.sh
```

The scan must cover every `archive=` and `static=` entry that will be linked
into the plugin. The job must fail before the expensive final link if known
shared-object-hostile relocations are present.

### Final Shared Object Link

For each Linux target, CI must produce:

```text
liblibbun_plugin_native.so
```

The Linux in-process link should use `lld` unless a target proves that GNU `ld`
can handle the link reliably within CI memory limits. The workflow must encode
the linker choice explicitly rather than relying on whatever the runner happens
to select.

The produced plugin must be a normal dynamically loadable ELF shared object.
It must not require text relocations or linker flags that turn invalid native
inputs into a loadable-but-fragile artifact.

### Runtime Loader Smoke

For each Linux target, CI must set `LIBBUN_PLUGIN_PATH` to the produced `.so`
and run the dynamic provider smoke test:

```text
cargo test --features dynamic-loading dynamic_plugin_provider_flow -- --exact --nocapture
```

The test must load the plugin through the public dynamic loader path. It must
not use sibling checkout discovery, helper-process fallback, or host-specific
native path overrides. CI must also fail the lane if the smoke test emits a
`mimalloc: error` diagnostic even when the Rust test exits successfully.

### Facade Conformance

For each Linux target, the in-process plugin must pass the same behavioral
surface required from the native runtime:

- module loading from source;
- prepared bundle loading;
- synchronous export calls;
- async export parking and explicit pump resolution;
- structured provider errors;
- output/log capture;
- host environment overlays;
- Rust-substrate provider rejection;
- deterministic shutdown.

The test suite may share the same facade-level tests used by macOS and helper
runtime paths, but the Linux lane must run them against the in-process `.so`.
The concrete dynamic conformance entrypoint is:

```text
cargo test --features dynamic-loading dynamic_plugin_facade_conformance -- --exact --nocapture
```

Like the smoke test, CI must run it through `LIBBUN_PLUGIN_PATH` and fail if
the log contains `mimalloc: error`.

### Shutdown Cleanliness

The arm64 proof emitted:

```text
mimalloc: error: mi_free: invalid pointer
```

That diagnostic was traced to `std::fs::canonicalize` in libbun's module
specifier path. On Linux, the embedded Bun/JSC/WebKit image brings mimalloc
symbols into the process; Rust's Unix canonicalization path called libc
`realpath` and then freed that libc-owned allocation through the interposed
mimalloc free path. libbun must not use libc APIs with malloc/free ownership
ambiguity in the dynamic plugin hot path. The fixed path converts module paths
to absolute file URLs without calling `realpath`, and the Linux smoke lane must
grep for `mimalloc: error` so a diagnostic cannot pass unnoticed.

The default expectation is clean shutdown with no allocator diagnostics,
crashes, sanitizer failures, or leaked helper fallback processes.

### Replacement Build Check

The release lane must prove user replaceability for the in-process Linux
plugin. At minimum, each Linux target must support:

- building a replacement plugin from the released source bundle and pinned
  WebKit PIC inputs;
- loading that replacement through `LIBBUN_PLUGIN_PATH`;
- running the same dynamic provider smoke test against the replacement.

This gate is part of the LGPL-oriented dynamic plugin boundary. The project
must not publish a Linux in-process asset that downstream users cannot
realistically rebuild or replace.

### Packaging and Compliance

Each Linux target tarball may shrink to a single plugin file plus metadata once
the helper is retired:

```text
liblibbun_plugin_native.so
libbun-native-bundle.json
```

The release must still publish shared compliance assets:

```text
libbun-plugin-native-vX.Y.Z-source.tar.zst
libbun-plugin-native-vX.Y.Z-NOTICE.txt
libbun-plugin-native-vX.Y.Z-licenses.json
libbun-plugin-native-vX.Y.Z-SOURCE.txt
libbun-plugin-native-vX.Y.Z-checksums.txt
```

The checksum file must include the Linux plugin tarballs, WebKit PIC input
references, and all shared compliance assets.

`libbun-native-bundle.json` must record whether the Linux bundle is
`in-process` or `helper-process`, plus the target triple, libbun version, Bun
source commit, plugin checksum, WebKit PIC artifact identity, and compliance
asset filenames.

### Documentation

README and release documentation must be updated before promotion to explain:

- Linux x86_64 and Linux arm64 are supported;
- Linux now uses an in-process plugin if that is the promoted implementation;
- older helper-backed bundles, if any exist, remain an implementation detail of
  those releases;
- downstream consumers still dynamically load the plugin and do not link
  Bun/JSC/WebKit;
- how to override `LIBBUN_PLUGIN_PATH`;
- how to use the no-download and `download-plugin` Cargo flows;
- where source, notices, license inventory, checksums, and replacement build
  instructions live.

## CI Shape

The release workflow should model Linux runtime mode explicitly:

```text
runtime_mode: in-process
target: x86_64-unknown-linux-gnu
target: aarch64-unknown-linux-gnu
```

During transition, it is acceptable to keep helper-backed and in-process jobs
side by side. The release workflow must publish only one default Linux asset
per target for a given release. Experimental in-process artifacts may be
uploaded as CI artifacts, but they must not be attached to a public GitHub
Release under the normal target tarball names until all promotion gates pass.

## Execution Plan

The current repository is close to the right shape, but several pieces still
encode helper-backed Linux as the only releasable Linux mode:

- `.github/workflows/release-native-plugin.yml` sets both Linux matrix rows to
  `runtime_mode: helper-process`;
- `scripts/package-native-plugin-release.sh` treats every `*-linux-gnu`
  package as `helper-process` and requires `LIBBUN_NATIVE_HELPER_BINARY`;
- `scripts/verify-release-assets.sh` already expects Linux target tarballs by
  default, but it does not distinguish helper-backed from in-process bundle
  contents;
- the source/compliance package records the native link manifest, but does not
  yet record WebKit PIC artifact identity as a first class input.

Move from the current state to an in-process Linux release in the following
phases.

### Phase 1: Make WebKit PIC Artifacts Durable

Convert the working `enki/WebKit` PIC outputs from workflow artifacts into
durable release inputs.

Required work:

- use the existing `enki/WebKit` `libbun-pic-5488984d` branch/workflow as the
  producer for these artifacts;
- keep the WebKit checkout and build logic in the WebKit fork; libbun CI must
  not clone the full WebKit repository just to assemble a plugin release;
- use WebKit Actions artifacts only for trial validation while deciding
  whether a snapshot is usable for libbun;
- add or run a WebKit release publication job in that fork that attaches
  `bun-webkit-linux-amd64-pic-debug` and
  `bun-webkit-linux-arm64-pic-debug` archives to a stable tag;
- publish SHA-256 checksums for each archive;
- record the WebKit source commit, build configuration, target triple, and PIC
  mode in a small metadata file next to the archives;
- decide the naming convention libbun will pin, for example:

```text
webkit-pic-<webkit-source-commit>
  bun-webkit-linux-amd64-pic-debug.tar.gz
  bun-webkit-linux-arm64-pic-debug.tar.gz
  checksums.txt
  metadata.json
```

Exit criteria:

- libbun can fetch a WebKit PIC artifact by tag/name/checksum;
- no libbun workflow depends on expiring Actions artifact IDs;
- libbun release jobs do not clone or build the full WebKit repository.

### Phase 2: Add a libbun WebKit PIC Input Resolver

Add a small script or release-step helper that installs the pinned WebKit PIC
artifact into the location expected by the native link manifest rewrite.

Required work:

- add a script such as `scripts/fetch-webkit-pic-artifact.sh`;
- require explicit inputs for WebKit tag, target, archive name, and checksum;
- verify the archive checksum before extraction;
- extract into a deterministic path under the build directory;
- rewrite or generate the native link manifest so WebKit/JSC/WTF static
  entries point at the PIC archive paths;
- record the WebKit artifact identity in a checked or generated metadata file
  that packaging can include in `libbun-native-bundle.json` and
  `licenses.json`.

Exit criteria:

- the current arm64 `smolvm` manifest rewrite can be reproduced by one script;
- the same script can run in CI for both Linux targets.

### Phase 3: Prove x86_64 Locally or in a Non-Publishing CI Lane

Repeat the arm64 experiment for `x86_64-unknown-linux-gnu` before changing the
public release matrix.

Required work:

- prepare or reuse an Ubuntu x86_64 environment;
- fetch the durable x86_64 WebKit PIC artifact;
- run `scripts/prepare-native-bun-link.sh`;
- rewrite the native link manifest to the PIC WebKit archive paths;
- run `scripts/inspect-linux-native-relocations.sh`;
- build the plugin with:

```text
LIBBUN_NATIVE_LINK_BUN=1
RUSTFLAGS="-C link-arg=-fuse-ld=lld"
cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml --features linux-in-process
```

- run the dynamic loader smoke test through `LIBBUN_PLUGIN_PATH`.

Exit criteria:

- x86_64 reaches the same proof level as the current arm64 result;
- any x86_64-only linker/runtime failures are captured in
  `docs/LIBBUN-LINUX-NATIVE-RELOCATION-DIAGNOSTICS.md`.

### Phase 4: Add Experimental In-Process Linux CI Lanes

Add non-publishing Linux in-process lanes beside the helper-backed lanes.

Required work in `.github/workflows/release-native-plugin.yml`:

- add matrix rows with `runtime_mode: in-process` for both Linux targets;
- install `lld` explicitly and set the Linux in-process linker flags
  deterministically;
- fetch and verify the pinned WebKit PIC artifact;
- run the native relocation scanner after manifest generation and before the
  final plugin link;
- build `plugin/Cargo.toml` with `--features linux-in-process`;
- do not build `runtime/Cargo.toml` for in-process rows;
- run dynamic loading tests with `LIBBUN_PLUGIN_PATH` only and
  `LIBBUN_RUNTIME_NATIVE_PATH` unset;
- fail the lane if the dynamic loading test emits `mimalloc: error`;
- upload these as workflow artifacts only, not GitHub Release assets.

Exit criteria:

- both Linux in-process CI rows are green;
- helper-backed rows remain available until the promotion decision is made.

### Phase 5: Expand Runtime and Replacement Verification

The smoke test proves the loader path, not the whole runtime contract. Promote
only after the in-process `.so` passes the broader facade behavior.

Required work:

- keep `tests/dynamic_conformance.rs::dynamic_plugin_facade_conformance`
  running on both Linux targets with `LIBBUN_PLUGIN_PATH`;
- keep the fixed `mimalloc` diagnostic covered by CI log checks;
- add a replacement-build check that rebuilds the plugin from the source bundle
  plus pinned WebKit PIC inputs, then loads that replacement through
  `LIBBUN_PLUGIN_PATH`.

Exit criteria:

- both Linux targets pass smoke, conformance, shutdown, and replacement checks;
- the result is not dependent on sibling checkouts or local-only artifact
  paths.

### Phase 6: Update Packaging for In-Process Linux

Teach release packaging that Linux can be either helper-backed or in-process.

Required work in `scripts/package-native-plugin-release.sh`:

- accept an explicit runtime mode instead of deriving helper mode solely from
  `*-linux-gnu`;
- require a helper binary only when `runtime_mode=helper-process`;
- package only `liblibbun_plugin_native.so` and
  `libbun-native-bundle.json` for `runtime_mode=in-process`;
- write the WebKit PIC artifact tag/name/checksum into
  `libbun-native-bundle.json`;
- include WebKit PIC artifact identity in `licenses.json` and `SOURCE.txt`;
- keep target tarball names unchanged:

```text
libbun-plugin-native-vX.Y.Z-x86_64-unknown-linux-gnu.tar.zst
libbun-plugin-native-vX.Y.Z-aarch64-unknown-linux-gnu.tar.zst
```

Required work in `scripts/verify-release-assets.sh`:

- verify that each Linux tarball contains bundle metadata;
- verify that helper-backed tarballs include the helper and in-process tarballs
  do not require one;
- keep both Linux target tarballs required by default only after the
  in-process lanes are release-eligible.

Exit criteria:

- local packaging can produce a Linux in-process tarball;
- the release verifier rejects missing metadata, missing checksums, or an
  unexpected helper/runtime-mode mismatch.

### Phase 7: Promote and Retire Helper as the Default

Only after phases 1 through 6 pass should the release workflow switch the
normal Linux target rows from `helper-process` to `in-process`.

Required work:

- update `.github/workflows/release-native-plugin.yml` so the published Linux
  matrix rows use `runtime_mode: in-process`;
- stop building `runtime/Cargo.toml` in the default Linux release rows;
- attach Linux in-process target tarballs to GitHub Releases;
- update README, release instructions, and Cargo feature documentation;
- update or supersede ADR-2044 and ADR-2045 to say helper-backed Linux is no
  longer the default release implementation;
- keep helper-backed code only if there is an explicit maintenance reason, such
  as fallback diagnostics or older-release support.

Exit criteria:

- a tagged release publishes macOS, Linux x86_64, and Linux arm64 assets;
- `scripts/verify-release-assets.sh --version <tag>` passes against the GitHub
  Release;
- downstream users can consume Linux through the same `libbun` facade and
  `LIBBUN_PLUGIN_PATH` flow as macOS.

Recommended sequence:

1. Prove Linux x86_64 with the same PIC WebKit artifact strategy used for
   arm64.
2. Convert WebKit PIC artifacts from expiring Actions artifacts to durable
   release inputs with checksums.
3. Add explicit `lld` Linux plugin links in CI.
4. Run relocation scan, final link, dynamic loader smoke, and conformance tests
   on both Linux targets.
5. Resolve or formally disposition the `mimalloc` shutdown diagnostic.
6. Add replacement build verification from the source/compliance bundle.
7. Update packaging, release verifier, README, and release instructions.
8. Only then retire the helper-backed Linux bundle as the default release
   implementation.

## Non-Goals

This ADR does not remove the helper-backed implementation immediately.

This ADR does not require publishing `libbun-native` or
`libbun-plugin-native` to crates.io.

This ADR does not change the stable facade API.

This ADR does not require Windows support.

This ADR does not require cross-compiling Linux releases from macOS.

## Consequences

The project gets a clear bar for switching Linux to the simpler macOS-style
runtime shape. The helper bundle remains available as a fallback strategy until
the in-process path is proven on the full Linux matrix.

The release process becomes stricter because WebKit PIC inputs become first
class release dependencies with checksums and rebuild expectations. That is the
right tradeoff if the project wants Linux to ship as a single dynamically
loaded plugin rather than a plugin-plus-helper bundle.

Promotion should reduce runtime complexity for Linux consumers by removing the
helper process and IPC layer. It may increase build/release complexity because
the release lane must own PIC WebKit artifacts explicitly.

## Acceptance Criteria

This ADR can move to `docs/done/` when:

- durable PIC WebKit artifacts exist for Linux x86_64 and Linux arm64;
- those artifacts are pinned by version/name/checksum in the libbun release
  workflow;
- both Linux targets pass native relocation inspection;
- both Linux targets link `liblibbun_plugin_native.so` using an explicit
  linker choice;
- both Linux targets pass `LIBBUN_PLUGIN_PATH` dynamic loader smoke tests;
- both Linux targets pass facade conformance tests against the in-process
  plugin;
- the `mimalloc` shutdown diagnostic is fixed or explicitly dispositioned;
- replacement build checks pass from the release source/compliance bundle;
- release packages and `libbun-native-bundle.json` support the in-process
  Linux bundle shape;
- `scripts/verify-release-assets.sh` requires both Linux target tarballs by
  default;
- README documents Linux x86_64 and Linux arm64 in-process plugin usage and
  the shared dynamic-loading downstream contract;
- ADR-2044 and ADR-2045 are updated or superseded to state that helper-backed
  Linux is no longer the default release implementation.
