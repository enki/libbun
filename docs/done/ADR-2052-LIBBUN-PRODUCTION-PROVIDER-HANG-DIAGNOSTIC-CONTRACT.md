# ADR-2052: libbun Production Provider Hang Diagnostic Contract

Status: Done
Date: 2026-05-21

Related: ADR-2035, ADR-2036, ADR-2038, ADR-2050, ADR-2051

## Context

`libbun` is a library boundary, not the downstream application host. It cannot
decide whether a production host runs providers in-process, in a worker process,
or behind a larger supervisor. It also cannot guarantee hard preemption while
the host thread is executing arbitrary blocking Bun, JSC, WebKit, or native I/O
code through an FFI boundary.

The settled provider receipts introduced by ADR-2050 and strengthened by
ADR-2051 are useful after a call returns. They are not sufficient for diagnosing
a production hang where the call never returns. If a provider wedges inside
module load, export invocation, async resolution, event-loop pumping, output
drain, or runtime shutdown, the host needs live evidence from before the blocked
boundary was entered.

The correct responsibility split is:

- `libbun` owns production observability for provider execution;
- the host owns any supervision policy, user-facing reporting, and escalation.

This ADR defines the `libbun` diagnostic contract needed by a host such as
`ss` to diagnose production provider hangs without relying on CI logs, stdout
diagnostics, debug builds, or ad hoc native debugger sessions.

## Decision

`libbun` must expose a host-consumable diagnostic contract for settled provider
calls. The contract must be stable enough for hosts to record, stream, and
postprocess without depending on internal Bun or plugin implementation details.

Each settled provider call must have a stable call identity:

```rust
pub struct ProviderCallId(pub String);
```

The call identity must be included in every live diagnostic event and in every
returned receipt or failure trace. The identity may be host-supplied or generated
by `libbun`, but it must be unique within the hosting process for the lifetime of
the runtime.

`libbun` must provide a live event sink:

```rust
pub trait ProviderDiagnosticSink {
    fn provider_event(&self, event: ProviderDiagnosticEvent);
}
```

The sink is a library callback. It must not write directly to host stdout or
stderr. Hosts may adapt the sink to JSONL, structured logs, telemetry, IPC, or
an in-memory ring buffer.

The sink contract is infallible from `libbun`'s perspective. A sink that cannot
persist or forward an event must handle that internally, for example by dropping
events and incrementing host-owned counters. Provider execution must not fail
because a diagnostic consumer is unavailable.

The sink must not be invoked while holding locks required by normal provider
execution. A sink implementation must be treated as potentially slow or
reentrant, so `libbun` should update its Rust-owned diagnostic state first, drop
internal locks, and then notify the sink with a cloned event.

The diagnostic event shape must include:

- call id;
- monotonic sequence number;
- schema version;
- span id for enter/exit pairing;
- optional parent span id;
- event timestamp or elapsed milliseconds from call start;
- event kind;
- settlement phase;
- operation;
- contract identity;
- provider domain class;
- module specifier or URL;
- export name;
- configured deadline;
- pending async task count when known;
- captured output record count when known;
- libbun ABI/version and Bun revision;
- dynamic plugin path and plugin fingerprint when applicable;
- process id and thread id where cheaply available;
- runtime instance id;
- optional detail string for errors or implementation-specific context.

The event kind must distinguish at least:

```text
call_start
phase_enter
phase_exit
deadline_elapsed
call_complete
call_failed
output_captured
runtime_snapshot
```

`phase_enter` must be emitted before crossing any boundary that can block or
enter native code. `phase_exit` must be emitted only after control returns from
that boundary. This asymmetry is intentional: a hang is diagnosed by the latest
`phase_enter` whose span id has no matching `phase_exit`.

The minimum phase set is:

```text
runtime_initialize
module_load
call_export
resolve_async
pump_event_loop
drain_output
shutdown
deadline_elapsed
complete
```

The existing settlement trace on `SettledProviderReceipt` remains required, but
it is post-return diagnostic data. It must not be described as the production
hang solution by itself.

`libbun` must also expose a cloneable diagnostics handle that is separate from
the provider execution call path:

```rust
pub struct ProviderDiagnosticsHandle { /* opaque */ }

impl ProviderDiagnosticsHandle {
    pub fn snapshot(&self) -> ProviderRuntimeSnapshot;
}
```

The handle must be backed by Rust-owned state that is updated before and after
boundary calls. A host must be able to keep the handle on a supervising thread
and call `snapshot` without entering Bun, JSC, WebKit, or the dynamic plugin.

The snapshot type is:

```rust
pub struct ProviderRuntimeSnapshot {
    pub active_call: Option<ProviderCallSnapshot>,
    pub recent_events: Vec<ProviderDiagnosticEvent>,
    pub captured_output_count: usize,
    pub runtime_state: ProviderRuntimeState,
}
```

The snapshot is best-effort diagnostic state. It must be cheap to capture and
must not require calling into Bun or JSC. It exists so a supervising host thread
or worker supervisor can ask `libbun` what it last attempted before deciding how
to report or escalate the stuck provider call.

The snapshot must be useful even when the runtime thread is blocked inside a
native call. It therefore cannot require `&mut BunHost`, `&mut LowLevelBunHost`,
or any lock that is held while crossing the blocking boundary.

Diagnostic payloads must avoid leaking provider input, source text, environment
values, or captured output text by default. Events may include identifiers,
counts, checksums, module specifiers, and explicit error details. Hosts that need
full request or output capture must opt in outside the default diagnostic event
contract.

The intended host integration is:

1. The host enables a diagnostic sink when creating the `libbun` host/runtime.
2. The host keeps a `ProviderDiagnosticsHandle` on a supervising thread or in a
   worker supervisor.
3. The host records live events durably enough for postmortem reporting.
4. The host starts a provider call with a call id and deadline.
5. The host enforces any hard deadline outside the in-process call path.
6. If the call returns, the host records the receipt trace and final events.
7. If the call hangs, the host asks the diagnostics handle for a snapshot and
   reports the latest unmatched `phase_enter`,
   plugin/runtime identity, request metadata, captured output count, and any
   snapshot collected by the host.

## Non-Goals

This ADR does not require `libbun` to kill threads or processes. Rust cannot
safely kill an arbitrary thread executing native code inside the same process.

This ADR does not require every host to run providers out of process. It requires
`libbun` to expose diagnostics that remain useful whether the host runs
providers in-process or behind an external supervisor.

This ADR does not make CI diagnostics the primary goal. CI may use the same
events and snapshots, but the contract is designed for production host
observability.

This ADR does not permit release builds to emit internal Bun diagnostics to host
stdout or stderr. Diagnostic events are structured library data, not leaked
process output.

## Consequences

`ProviderSettleOptions` needs an extension point for call identity and
diagnostic configuration, or `libbun` needs a separate provider-call context
argument that can be introduced without making deadlines harder to use. Existing
callers that only supply a deadline must remain source compatible where
practical.

`BunHost`, `LowLevelBunHost`, and dynamic plugin hosting must propagate
diagnostic events consistently. Events emitted by the Rust facade must happen
before and after each risky boundary, including dynamic plugin FFI calls.

Dynamic plugin metadata must become part of the diagnostic surface. A host
debugging a production hang needs to know which `.dylib` or `.so` was loaded,
which checksum it had, and which Bun revision/libbun ABI it represented.

The snapshot API must not call into Bun or depend on the blocked native runtime
making progress. It should be backed by Rust-owned state updated before and
after boundary calls.

The diagnostic schema must be versioned. Hosts must be able to persist diagnostic
events from one `libbun` version and parse them after an upgrade. New optional
fields may be added, but existing meanings must remain stable within a major
schema version.

The live event sink must be careful about reentrancy and latency. `libbun` must
document that sink implementations should avoid recursively calling back into the
same runtime, and `libbun` must not rely on sink callbacks for internal state
correctness.

The release hygiene requirements from ADR-2051 remain in force. Better
diagnostics are not a substitute for preventing debug-profile or non-relocatable
artifacts from reaching production.
