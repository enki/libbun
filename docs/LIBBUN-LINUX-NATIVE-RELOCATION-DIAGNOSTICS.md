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
- mimalloc uses `-ftls-model=local-dynamic` instead of the
  static-executable-oriented `initial-exec` TLS model.

This mode still must be proven by an actual Linux plugin build. If the
relocation scanner or final `.so` link continues to fail, the failing object
owners should be added to this diagnostic note before the next patch.
