# ADR-2038: libbun Dynamic Plugin Only Downstream Consumption

Status: Done
Date: 2026-05-18

`libbun` embeds Bun internals, and Bun's native runtime path includes
JavaScriptCore/WebKit components whose redistribution requirements are not the
same as ordinary Rust library dependencies. Bun's own license notes describe
Bun as MIT-licensed while also documenting that Bun statically links
JavaScriptCore/WebKit and provides a relinking path for modified WebKit/JSC
builds.

That compliance posture is acceptable for `libbun` itself: `libbun` is open
source, owns the Bun embedding integration, and can document or ship whatever
native relinking materials are required for its own artifacts.

The boundary that must remain clean is downstream host applications. A
downstream host must not accidentally become the final statically linked
executable that contains Bun/JSC/WebKit objects merely because it enabled an
optional `libbun` feature or depended on a Rust crate.

## Decision

Any `libbun` implementation that carries Bun native runtime code, JSC/WebKit
objects, or Bun's native link manifest must be consumed by downstream
applications only as a replaceable dynamic plugin/shared library.

Downstream applications must not link `libbun-native` as a normal Rust
dependency that is statically folded into the host executable.

The required downstream shape is:

```text
host application
  loads at runtime through dlopen/libloading/LoadLibrary
replaceable libbun plugin dynamic library
  owns Bun embedding adapter and Bun/JSC/WebKit native integration
```

The plugin boundary must be a C-compatible ABI, not Rust's unstable native ABI.
The ABI must include an explicit version handshake before any provider runtime
is created.

The host application must be able to start and operate without the plugin
present. Missing, incompatible, or unloadable `libbun` plugins are reported as
an unavailable optional capability, not as host process startup failures.

## Scope

This decision applies to the native Bun/JSC/WebKit-carrying implementation.

The stable Rust facade crate may continue to define protocol types, structural
values, receipts, conformance runtimes, tests, and shared source-level
contracts, as long as depending on it does not pull Bun/JSC/WebKit native code
into a downstream host executable.

The `native/` crate is an implementation input for building and testing the
dynamic plugin. It is not a downstream application dependency surface.

## Required Plugin Contract

The dynamic plugin must provide:

- a stable C-compatible exported symbol set;
- an ABI version function, for example `libbun_plugin_abi_version`;
- explicit create/destroy lifecycle functions for plugin-owned runtime handles;
- plugin-owned allocation/free functions for strings, byte buffers, structured
  results, and error payloads crossing the ABI;
- a way to report the Bun source revision and `libbun` ABI revision used by the
  plugin;
- documented threading and reentrancy rules;
- documented shutdown behavior that does not require host process termination;
- a replaceable file path and loading strategy controlled by the host or user.

The host side should load the plugin with a runtime loader such as `libloading`
on Rust hosts, `dlopen` on Unix-like systems, or `LoadLibrary` on Windows.

## Prohibited Shapes

Downstream host applications must not ship these shapes:

- one host executable statically linking `libbun-native`;
- a Cargo feature that causes Bun/JSC/WebKit objects to be linked into the host
  binary;
- a public `libbun-native` crate workflow where ordinary downstream users build
  the native adapter directly into their application;
- a plugin ABI that requires the host application to be relinked when the
  plugin is rebuilt with modified Bun/JSC/WebKit objects.

## Consequences

Downstream hosts can keep `libbun` support optional without making the core
host binary part of the Bun/JSC/WebKit native distribution.

`libbun` carries the native runtime compliance and replacement story. The host
binary carries only the dynamic loading code and optional provider integration.

The plugin ABI becomes a real product boundary. It must be versioned,
documented, tested for compatibility, and kept small enough that replacement is
practical.

Rust-only convenience APIs for downstream hosts should wrap the dynamic plugin
loader. They must not bypass it by re-exporting or statically depending on the
native adapter.

## Implementation

This ADR is implemented by:

- `plugin/`, a `cdylib` crate that wraps `NativeBunRuntime` behind a C ABI;
- `libbun::plugin_abi`, which defines the shared ABI version, status, and
  plugin-owned buffer types;
- `libbun::dynamic::DynamicBunRuntime`, enabled by the `dynamic-loading`
  feature, which uses runtime dynamic loading and implements the stable
  `BunEmbeddingRuntime` facade without depending on `libbun-native`;
- a runtime-loaded dynamic plugin flow test in `tests/dynamic_plugin.rs`;
- an `internal-adapter` feature gate on `native/` so ordinary direct checks or
  dependency use fail with guidance to build the dynamic plugin instead;
- README guidance describing the dynamic plugin as the only downstream native
  consumption path.

The native adapter remains unpublished and is still used directly by repository
integration tests. That direct use is internal verification only; downstream
applications use the dynamic plugin boundary.

Release compliance for distributed native plugin binaries is tracked separately
by ADR-2039 and ADR-2040. Official native plugin binaries must be produced by
the GitHub Actions release workflow described there, so downstream hosts consume
versioned release assets rather than building the native plugin from a local
checkout by default.
