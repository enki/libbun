# ADR-2044: Linux Helper-Backed Native Runtime

Status: Superseded by ADR-2048
Date: 2026-05-18

Superseded: 2026-05-19

ADR-2048 promoted Linux PIC in-process plugin releases for the default Linux
release path. This ADR is retained as the historic helper-backed fallback
design and as context for older helper-backed bundles.

## Context

ADR-2038 established the downstream contract: hosts consume Bun through a
replaceable dynamic plugin and do not statically link Bun/JSC/WebKit.

ADR-2042 tried to make the Linux plugin a single in-process `.so` by producing
shared-object-compatible native inputs. That work added useful guardrails, but
it also identified the real blocker: the default upstream WebKit/JSC/WTF
prebuilt static archives contain local-exec TLS relocations that cannot be
linked into a normal Linux shared object. Rebuilding all WebKit inputs for
shared-object embedding may still be possible, but Linux support should not
depend on that being the only path.

The product requirement is broader than PIC: Linux support must work for
downstream hosts while preserving the dynamic plugin boundary.

## Decision

Linux native support may use a helper-backed runtime:

- downstream hosts still load `liblibbun_plugin_native.so` dynamically;
- the Linux plugin is allowed to be a thin adapter that starts and supervises a
  companion native runtime executable;
- the helper executable may link Bun/JSC/WebKit using Bun's normal Linux
  executable-oriented native inputs;
- host applications must not link Bun/JSC/WebKit directly;
- users must be able to replace both the plugin and the helper executable with
  interface-compatible modified builds.

The Linux release artifact therefore becomes a small runtime bundle rather than
only a single `.so`:

```text
liblibbun_plugin_native.so
libbun-runtime-native
```

The plugin owns helper process lifecycle, protocol negotiation, request routing,
stdout/stderr/log forwarding, shutdown, and crash/error translation into the
existing `libbun` facade error model.

The public consumer contract stays the same across platforms:

```text
consumer app -> LIBBUN_PLUGIN_PATH -> liblibbun_plugin_native.{dylib,so}
```

The native implementation may differ by platform:

```text
macOS:
consumer app -> dynamically loaded plugin -> in-process Bun/JSC/WebKit

Linux:
consumer app -> dynamically loaded plugin -> helper process -> Bun/JSC/WebKit
```

If Linux later gains shared-object-compatible WebKit/JSC/WTF artifacts, the
Linux plugin may move back to the macOS-style in-process implementation without
changing the downstream dynamic-loading contract.

The helper-backed Linux runtime is therefore an implementation strategy, not a
new public API commitment. The plugin ABI must keep the runtime implementation
behind an internal provider boundary so Linux can support either:

```text
plugin -> helper process -> Bun/JSC/WebKit
plugin -> in-process Bun/JSC/WebKit
```

without changing how downstream hosts load or configure `libbun`.

## Required Technical Shape

The helper protocol must preserve the existing facade semantics:

- module loading from source and prepared bundles;
- synchronous export calls;
- async export parking/resolution through explicit host pumping;
- structured provider errors;
- output capture and log sink delivery;
- host environment overlays;
- deterministic shutdown;
- Rust-substrate rejection before provider execution.

The helper boundary must be versioned. The plugin and helper must negotiate at
startup using at least:

- plugin ABI version;
- helper protocol version;
- target triple;
- libbun release version;
- Bun source commit;
- helper binary checksum when available.

The helper executable must be treated as a replaceable native runtime component.
It must be discoverable by:

- an explicit environment override;
- a path adjacent to the loaded plugin;
- release-bundle metadata.

The plugin must fail closed when the helper is missing, has an incompatible
protocol version, exits unexpectedly, or reports a Bun initialization failure.

The plugin implementation should introduce an internal runtime transport
boundary with at least two possible implementations:

- `in_process`, used by the current macOS plugin and by any future Linux PIC
  plugin;
- `helper_process`, used by Linux while WebKit/JSC/WTF inputs are not suitable
  for in-process shared-object linking.

Tests should target the shared transport behavior where possible, not only the
helper process implementation, so future Linux PIC work can reuse the same
conformance suite.

## Compliance Shape

The Linux helper may include statically linked LGPL-covered WebKit/JSC objects.
That is acceptable for this repository because the helper is part of the open
source `libbun` release bundle and the same GitHub Release must provide the
corresponding source, notices, license inventory, build instructions, and
checksums.

Downstream consumers still satisfy the intended boundary by dynamically loading
the plugin and keeping the native runtime bundle replaceable. They should not
need to build or relink Bun/JSC/WebKit themselves unless they choose to replace
the native runtime.

## Non-Goals

This ADR does not require abandoning a future in-process Linux `.so` if
PIC-compatible WebKit artifacts become available.

This ADR does not declare helper IPC to be the permanent Linux architecture.

This ADR does not change the stable facade crate API.

This ADR does not require publishing `libbun-native` or
`libbun-plugin-native` to crates.io.

This ADR does not add Windows support.

## Consequences

Linux support no longer depends exclusively on converting every Bun/WebKit
native input into shared-object-compatible form.

The runtime bundle is more complex than a single plugin file because it has a
process boundary and IPC protocol. That complexity is acceptable because it
keeps the downstream dynamic-loading contract intact and avoids forcing
upstream consumers to handle WebKit/JSC native-link details.

macOS may continue using the existing in-process dynamic plugin path. Linux can
move independently through the helper-backed path until an in-process Linux
plugin is proven practical.

Trying the helper-backed path has limited downside because it should add a
runtime transport abstraction that the project needs anyway for process
isolation, diagnostics, and replacement testing. If PIC-compatible WebKit
artifacts become practical later, Linux can retire the helper process while
keeping the same plugin filename, release target names, facade API, and
downstream loading instructions.

README and release documentation must explain this platform difference
directly: consumers dynamically load the plugin on both platforms, macOS runs
the native runtime in-process today, and Linux may use a helper executable
behind the plugin. The helper must be described as an implementation detail of
the replaceable native bundle, not as a static-linking requirement for
downstream hosts.

## Acceptance Criteria

This ADR can move to `docs/done/` when:

- a Linux helper executable crate or target exists;
- the Linux plugin can discover, start, and stop the helper;
- plugin/helper protocol negotiation rejects incompatible versions;
- plugin internals keep helper-process transport separate from the native
  runtime facade so an in-process Linux transport can be added later without
  changing the public plugin ABI;
- the helper path passes the same facade conformance behaviors currently
  required from the native adapter;
- `LIBBUN_PLUGIN_PATH` dynamic loading works with the helper-backed plugin;
- users can replace the helper path through a documented override;
- helper crashes and startup failures become structured `libbun` errors;
- the release package includes both the Linux `.so` and helper executable;
- README documents the macOS in-process plugin path, the Linux helper-backed
  plugin path, and the fact that both preserve the same dynamic-loading
  consumer contract;
- source, notice, license inventory, and checksum assets cover the helper.
