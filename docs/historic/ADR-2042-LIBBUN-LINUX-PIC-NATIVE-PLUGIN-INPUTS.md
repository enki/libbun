# ADR-2042: Linux PIC Native Plugin Inputs

Status: Superseded
Date: 2026-05-18

ADR-2041 established the desired publication shape for `libbun`: a small
facade crate on Cargo and replaceable native plugins on GitHub Releases. The
macOS arm64 plugin release path works. Linux plugin releases do not.

The failed Linux release runs reached the final `cdylib` link and then failed
because the current vendored Bun/WebKit/native static archives contain TLS
relocations that cannot be linked into a shared object:

- `x86_64-unknown-linux-gnu`: `rust-lld` rejects `R_X86_64_TPOFF32`
  relocations from static objects in `libWTF.a` and native objects such as
  `turbojpeg.c.o`;
- `aarch64-unknown-linux-gnu`: `ld.lld` rejects AArch64 TLS relocations such as
  `R_AARCH64_TLSLE_ADD_TPREL_HI12` from static WebKit/Bun inputs.

This is not a GitHub Actions packaging problem. It is a native input format
problem. Linux `.so` plugin releases require Bun/WebKit/native dependency
objects that are compiled as position-independent code and use a TLS model
compatible with shared libraries.

Supersession note, 2026-05-18: this ADR proved that libbun can inspect and
reject Linux shared-object-hostile native inputs, and it added a reproducible
PIC mode for Bun-owned C/C++ objects and direct/native dependency builds.
However, the default Bun build consumes upstream prebuilt WebKit/JSC/WTF
archives from `~/.bun/build-cache`, and those archives still contain
local-exec TLS relocations. Linux release publication therefore cannot be made
complete by release-matrix plumbing or direct-dependency flag patches alone.
The next Linux publication attempt must start from PIC-compatible WebKit
artifacts: either a local WebKit source build configured for the plugin PIC
mode, or an upstream WebKit prebuilt release that is explicitly built for
shared-object embedding.

## Decision

`libbun` will not publish Linux native plugin binaries until the Linux native
inputs are proven shared-object compatible.

The Linux release path must produce or obtain PIC-compatible native inputs for:

- Bun C/C++ objects collected into `libbun_native_objects.a`;
- WebKit/WTF/JavaScriptCore static libraries used by Bun;
- bundled native dependency objects that are pulled into the plugin, including
  libjpeg-turbo and similar third-party C/C++ objects.

The release workflow may keep macOS plugin publication active while Linux work
is in progress, but README and release verification must not advertise Linux
plugin assets as currently supported until Linux artifacts pass the linker and
dynamic loading tests.

## Required Technical Shape

The Linux plugin build must link a real shared library:

```text
liblibbun_plugin_native.so
```

That shared library must be built from native inputs compiled with the
appropriate Linux shared-object settings. At minimum, the investigation must
prove the effective compiler/linker flags for all native inputs include the
equivalent of:

```text
-fPIC
```

and must avoid local-exec TLS relocations in any object that will be linked
into the plugin `.so`.

The acceptable fix is one of:

- configure vendored Bun/WebKit builds to emit PIC-compatible static archives
  for Linux plugin release jobs;
- build or obtain shared-library-compatible WebKit/JSC/WTF/native dependency
  artifacts and link the plugin against those;
- split the Linux native runtime boundary so the plugin links only to a
  separately replaceable shared native runtime library, while preserving the
  downstream dynamic-plugin-only contract.

The unacceptable fix is to force the linker to accept text relocations,
non-PIC static objects, or a Linux binary that is not a normal replaceable
shared library.

## Investigation Plan

1. Capture the exact failing Linux linker command and all rejected relocation
   sources into a checked-in diagnostic note under `release/` or `docs/`.
   Current diagnostic note:
   `docs/LIBBUN-LINUX-NATIVE-RELOCATION-DIAGNOSTICS.md`.
2. Add a script that inspects Linux native archives for shared-object-hostile
   relocations before the expensive plugin link. Current guardrail:
   `scripts/inspect-linux-native-relocations.sh`.
3. Identify which build system owns each failing object:
   `vendor/bun` CMake/Ninja output, Bun Rust crates, WebKit cache artifacts, or
   third-party native dependency builds.
4. Determine whether Bun's existing build configuration has a PIC/shared
   library mode for Linux artifacts.
5. If Bun/WebKit already supports PIC-compatible artifacts, add release-script
   configuration for that mode and verify both x86_64 and arm64 Linux.
6. If not, add a minimal patch set to the vendored build that enables PIC for
   only the Linux plugin release profile. Current Linux preparation mode:
   `scripts/prepare-native-bun-link.sh` exports `LIBBUN_NATIVE_PLUGIN_PIC=1`,
   which switches Bun/WebKit/native dependency flags away from the default
   static-executable PIC/TLS assumptions for the plugin input build. The
   vendored Bun changes live in
   `patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch`
   and are replayed by `scripts/apply-vendored-bun-patches.sh` after
   re-vendoring.
7. Re-run Linux x86_64 first, then Linux arm64.
8. Restore Linux targets to the release matrix and release asset verifier only
   after the link, dynamic loader smoke test, replacement build check, package,
   and compliance bundle all pass.

## Release Workflow Requirements

Linux plugin release jobs must fail before packaging if any archive still
contains forbidden relocations.

The workflow must keep the same downstream contract as ADR-2038:

- the host application loads the plugin dynamically;
- users can replace the plugin by path;
- hosts do not statically link Bun/JSC/WebKit;
- compliance assets remain attached to the same GitHub Release as the binary.

The Linux release matrix should be restored in this order:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
```

Linux x86_64 is the first blocker because it is the common server deployment
target. Linux arm64 follows because it is common for ARM server fleets and
should be part of the mature matrix.

## Documentation Requirements

Until this ADR is complete, README must clearly state that Linux native plugin
assets are not published yet.

When complete, README must be updated to include Linux download examples and
the release verifier default target list must again include Linux x86_64.
Linux arm64 should be added to the default verifier list once it is green in
the release workflow.

## Non-Goals

This ADR does not change the facade crate publication shape.

This ADR does not allow downstream hosts to statically link `libbun-native`.

This ADR does not require publishing `libbun-native` or
`libbun-plugin-native` to crates.io.

This ADR does not require Windows support.

## Consequences

Linux release work moves from "add a CI lane" to "produce valid shared-library
native inputs." That is a deeper native build task, but it is the only honest
route to a Linux `.so` plugin that preserves the LGPL-oriented dynamic boundary.

The project can continue publishing macOS plugin releases while Linux is
blocked, provided documentation is explicit about the platform limit.

## Acceptance Criteria

This ADR was superseded before completion. The partial implementation remains
useful, but Linux release publication is still blocked. The original completion
criteria were:

- a Linux native input inspection script exists and fails on shared-object-
  hostile TLS relocations;
- the Linux x86_64 release job builds `liblibbun_plugin_native.so`;
- the Linux x86_64 release job passes the dynamic loader smoke test through
  `LIBBUN_PLUGIN_PATH`;
- the Linux x86_64 release job passes the replacement build check;
- the release workflow packages and attaches the Linux x86_64 binary plus the
  matching SOURCE, NOTICE, license inventory, source archive, and checksum
  assets;
- `scripts/verify-release-assets.sh` requires the Linux x86_64 asset by
  default;
- README documents Linux x86_64 plugin installation and no longer says Linux
  plugin assets are unavailable;
- Linux arm64 is either green and added to the default release verifier, or a
  follow-up ADR exists for any remaining arm64-only blocker.
