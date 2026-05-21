# ADR-2051: libbun Hang Diagnostics and Release Artifact Debuggability

Status: Done
Date: 2026-05-21

Related: ADR-2035, ADR-2038, ADR-2049, ADR-2050

## Context

`libbun` release plugins are loaded dynamically by downstream hosts. A release
plugin that only works in the GitHub Actions build directory is invalid even if
it passes a local smoke test. Bun debug-profile artifacts can embed absolute
paths to generated built-in JavaScript files, emit internal loop/I/O diagnostics
to process stdout/stderr, and fail or hang after the plugin is moved.

The settled provider API from ADR-2050 also changed the debugging obligation.
If `call_provider_until_settled` hangs, the host should not need to reconstruct
the parked-handle loop by reading native code or attaching a debugger. The
settled call must expose where it is waiting and must enforce deadline behavior
from the facade-owned boundary whenever possible.

## Decision

Release native plugins must be built and packaged from Cargo release-profile
artifacts only.

The release path must reject:

- plugin or helper binaries selected from `target/debug`;
- binaries containing Bun debug relocation markers such as `bun-debug`,
  `BUN_DYNAMIC_JS_LOAD_PATH`, `build/debug`, or generated built-in JavaScript
  absolute paths;
- release archives containing static-linkable files such as `.a`, `.o`,
  `.rlib`, `.lib`, `.lo`, or `.bc`;
- dynamic provider smoke or conformance runs that leak internal Bun diagnostics
  such as `[loop]` or `[filesink]` to host stdout/stderr.

The normal Rust dynamic facade must own settled provider control flow. It may
still use the plugin ABI for module load, export call, event-loop pump, async
resolution, output drain, and shutdown, but it must not delegate the entire
settled call to one opaque plugin FFI call. This keeps deadline checks and
diagnostic collection in the facade between observable phases.

This is necessary but not sufficient for perfect hang handling. The architectural
target is stricter:

- no FFI call used by the settled path may perform an unbounded wait;
- every native pump or I/O wait must accept a bounded budget or expose a
  nonblocking poll shape;
- tests that prove hang behavior should run the plugin behind a diagnostic
  process boundary so CI can terminate a stuck native runtime and preserve
  artifacts;
- production in-process hosting may remain the fast path, but it cannot promise
  hard preemption while executing arbitrary blocking native code on the host
  thread.

Settled provider receipts and failures must carry a bounded structured trace of
the phases observed by the facade:

```text
start
module_load
call_export
resolve_async
pump_event_loop
deadline_elapsed
complete
```

The trace is diagnostic data, not host stdout/stderr. Release builds must remain
silent except for provider output that is intentionally captured by `libbun`'s
configured output policy.

## Consequences

The GitHub release workflow, local preflight, and packaging script must agree on
the same release-profile artifact paths. A workflow using `target/debug` for
release assets is a release-blocking bug.

Debug-profile and relocation checks are intentionally string-based because the
failure mode is encoded into the native image. The checks are conservative:
false positives should fail the release and force review rather than allowing a
non-relocatable plugin onto GitHub or crates.io.

The plugin ABI keeps the full settled-call export for compatibility and
diagnostic consumers, but the Rust facade does not depend on it for normal
dynamic provider hosting.

This ADR does not claim every native blocking point is interruptible. If a Bun
or JSC FFI call blocks inside one phase, the trace identifies the last entered
phase. Future work may add an opt-in watchdog that writes a repro bundle with
thread stacks and the last structured trace events before aborting a stuck test
process.
