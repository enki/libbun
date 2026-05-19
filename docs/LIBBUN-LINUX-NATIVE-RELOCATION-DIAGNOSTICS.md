# Linux Native Plugin Relocation Diagnostics

Date: 2026-05-18

`libbun` currently publishes the native Bun/JSC runtime as a replaceable
dynamic plugin. The macOS arm64 plugin links successfully. Linux plugin lanes
were removed from the release matrix after the Linux `.so` link reached the
final `cdylib` stage and failed on non-shared-library-compatible relocations
inside vendored Bun/WebKit/native static inputs.

## Observed x86_64 Failure

GitHub Actions release run `26063623639` on commit `297ef51` reached the
`x86_64-unknown-linux-gnu` plugin link and failed in `rust-lld` with TLS
local-exec relocations from static archives:

```text
relocation R_X86_64_TPOFF32 against WTF::disableMallocRestrictionScopeCount cannot be used with -shared
.../.bun/build-cache/webkit-5488984d20e0dbfe-debug/lib/libWTF.a(MallocCommon.cpp.o)

relocation R_X86_64_TPOFF32 against WTF::forbidMallocUseScopeCount cannot be used with -shared
.../.bun/build-cache/webkit-5488984d20e0dbfe-debug/lib/libWTF.a(MallocCommon.cpp.o)

relocation R_X86_64_TPOFF32 against errStr cannot be used with -shared
libbun_native_objects.a(turbojpeg.c.o)
```

The first two inputs are WebKit/WTF static archive members. The `errStr`
failure comes from a native dependency object collected into
`libbun_native_objects.a`.

## Observed AArch64 Failure

GitHub Actions release run `26060921681` reached the
`aarch64-unknown-linux-gnu` plugin link and failed in `ld.lld` with AArch64
local-exec TLS relocations:

```text
relocation R_AARCH64_TLSLE_ADD_TPREL_HI12 against pas_thread_local_cache_is_exiting cannot be used with -shared
relocation R_AARCH64_TLSLE_ADD_TPREL_LO12_NC against pas_thread_local_cache_is_exiting cannot be used with -shared
relocation R_AARCH64_TLSLE_ADD_TPREL_HI12 against pas_thread_local_cache_pointer cannot be used with -shared
relocation R_AARCH64_TLSLE_ADD_TPREL_HI12 against WTF::disableMallocRestrictionScopeCount cannot be used with -shared
relocation R_AARCH64_TLSLE_ADD_TPREL_HI12 against simd_support cannot be used with -shared
```

These relocations indicate that at least some Bun/WebKit/native dependency
objects were compiled for a static executable TLS model. They cannot simply be
forced into a normal Linux shared object.

## Current Guardrail

`scripts/inspect-linux-native-relocations.sh` inspects the generated native
link manifest before the expensive Rust `cdylib` link. On Linux it walks the
`archive=` and `static=` entries and fails if any ELF object contains the known
hostile TLS relocation families observed above:

```text
R_X86_64_TPOFF32
R_X86_64_TPOFF64
R_AARCH64_TLSLE_*
```

This scanner is a guardrail, not the final fix. Passing it only proves that the
known local-exec TLS blockers are absent from the link inputs. The Linux plugin
release remains blocked until the full `.so` link, dynamic loader smoke test,
replacement build check, packaging, and release verifier all pass.

## PIC Build Mode Under Test

`patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch`
adds a reproducible vendored Bun patch that is replayed by
`scripts/apply-vendored-bun-patches.sh` after re-vendoring.
`scripts/prepare-native-bun-link.sh` now exports `LIBBUN_NATIVE_PLUGIN_PIC=1`
by default on Linux. That mode keeps Bun's normal executable build defaults
unchanged, but asks the vendored build scripts to produce plugin inputs
suitable for a Linux shared object:

- Bun C/C++ global flags use `-fPIC` instead of `-fno-pic -fno-pie`;
- WebKit/JSC/WTF CMake flags use `-fPIC` and
  `CMAKE_POSITION_INDEPENDENT_CODE=ON` instead of the static-executable
  `-fno-pic`/`CMAKE_POSITION_INDEPENDENT_CODE=OFF` path;
- direct and nested native dependency builds receive `-fPIC` when the plugin
  PIC mode is enabled;
- mimalloc uses `-ftls-model=local-dynamic` instead of the
  static-executable-oriented `initial-exec` TLS model.

This mode still must be proven by an actual Linux plugin build. If the
relocation scanner or final `.so` link continues to fail, the failing object
owners should be added to this diagnostic note before the next patch.

## 2026-05-18 Follow-up Findings

Linux arm64 preparation now avoids linking Bun's final executable when it only
needs plugin link inputs. That sidesteps the non-PIC `libbun_rust.a` failure
from Bun's executable link, which is not part of the plugin manifest.

After regenerating the build, direct/native dependency objects such as
`libjpeg-turbo` are compiled with `-fPIC`; `turbojpeg.c.o` is no longer the
first relocation scanner failure.

The remaining scanner failures come from upstream prebuilt WebKit/JSC/WTF
archives under Bun's shared build cache, for example:

```text
libWTF.a(MallocCommon.cpp.o): R_AARCH64_TLSLE_ADD_TPREL_HI12
libbmalloc.a(pas_thread_local_cache.c.o): R_AARCH64_TLSLE_ADD_TPREL_HI12
```

Those archives are downloaded prebuilt artifacts, not outputs of the patched
local WebKit CMake path. Linux plugin publication therefore requires
PIC-compatible WebKit artifacts: either build WebKit from source in the plugin
PIC mode, or consume an upstream WebKit prebuilt release that is explicitly
compatible with shared-object embedding.

## 2026-05-19 PIC Artifact Verification

GitHub Actions artifacts from `enki/WebKit` run `26077123752` produced
PIC-compatible debug WebKit bundles for:

- `bun-webkit-linux-amd64-pic-debug`;
- `bun-webkit-linux-arm64-pic-debug`.

Those artifacts were promoted to durable `enki/WebKit` GitHub Release assets:

```text
https://github.com/enki/WebKit/releases/tag/libbun-webkit-pic-5488984d-20260519
```

The release targets the `libbun-pic-5488984d` branch and includes the amd64 and
arm64 PIC WebKit archives, `checksums.txt`, and `metadata.json`.

The arm64 bundle was tested locally inside an Ubuntu 24.04 `smolvm` machine
using the mounted libbun checkout and the existing Bun native object archive.
The native link manifest was rewritten to use the extracted PIC WebKit
archives, then `scripts/inspect-linux-native-relocations.sh` passed with no
known shared-object-hostile TLS relocations.

The first full plugin link with GNU `ld` was killed by the kernel during the
large final link. Re-running the same link with `lld` succeeded and produced a
normal dynamically loadable Linux arm64 plugin:

```text
/work/target-smolvm-plugin-pic/debug/liblibbun_plugin_native.so:
ELF 64-bit LSB shared object, ARM aarch64, dynamically linked
```

Two runtime-loader issues were found and fixed in the libbun plugin/native
crates:

- the PIC debug WebKit inputs reference UBSan handlers, so Linux plugin builds
  must link the toolchain `libubsan` runtime when it is available;
- Bun's Linux `sys_epoll_pwait2` platform shim must be force-linked into the
  native adapter, matching Bun's own binary crate behavior.

With those fixes and `RUSTFLAGS="-C link-arg=-fuse-ld=lld"`, the dynamic
loader smoke test passed on Linux arm64:

```text
LIBBUN_PLUGIN_PATH=/work/target-smolvm-plugin-pic/debug/liblibbun_plugin_native.so
cargo +nightly-2026-05-06 test --features dynamic-loading dynamic_plugin_provider_flow -- --exact --nocapture
```

The first version completed successfully, but emitted one allocator diagnostic:

```text
mimalloc: error: mi_free: invalid pointer: 0xFFFF94003600
```

The diagnostic was traced with GDB to `std::fs::canonicalize` inside
`path_to_file_specifier`: Rust's Unix canonicalization path called libc
`realpath`, then the process-local Bun mimalloc symbols interposed the free
path for that libc allocation. libbun no longer canonicalizes module paths on
that hot path; it converts them to absolute file URLs without calling
`realpath`. The Linux PIC smoke workflow now greps test output and fails if a
`mimalloc: error` diagnostic is emitted.

The workflow also runs
`tests/dynamic_conformance.rs::dynamic_plugin_facade_conformance` through
`LIBBUN_PLUGIN_PATH`. That single-process dynamic conformance test covers
source module loading, prepared bundle loading, sync and async exports,
structured provider errors, captured output/log handling, host environment
overlays, Rust-substrate provider rejection, and deterministic shutdown.

This means PIC WebKit artifacts can make the Linux in-process dynamic plugin
viable at least on arm64. Linux release publication still needs the full
promotion gate set, including replacement-build verification and
packaging/release promotion before replacing the helper-backed Linux bundle as
the default release shape.

`scripts/fetch-webkit-pic-artifact.sh` now makes the WebKit PIC input step
reproducible: it downloads the pinned `enki/WebKit` release asset, verifies the
published checksum, extracts it, and rewrites a libbun native link manifest to
point WebKit/JSC/WTF static entries at the PIC archives. The non-publishing
`.github/workflows/verify-linux-pic-plugin.yml` workflow uses that script to
verify both Linux targets before any release-matrix promotion.
