# ADR-2050: libbun Settled Provider Calls and Quarantined Async Embedding

Status: Proposed
Date: 2026-05-21

Related: ADR-2033, ADR-2035, ADR-2038, ADR-2049

## Context

`libbun` is a Rust facade for hosting JavaScript and TypeScript provider code
through a replaceable Bun native plugin. The facade currently exposes low-level
async pieces such as parked handles, event-loop pumping, and async resolution.
Those pieces are useful for experiments and future streaming embeddings, but
they are the wrong surface for ordinary provider calls.

An ordinary provider call has a terminal contract from the host's point of view:

```text
request provider
  -> provider returns a value
  -> provider rejects/throws
  -> provider times out
  -> runtime/module loading fails
```

The host should not need to know how many Bun/JSC ticks are required, when to
sleep, whether a promise is represented as an internal promise, or how to poll a
parked handle. Those details belong inside the component that owns the Bun VM.

The same boundary applies to diagnostics. A failure such as:

```text
module import threw
async result did not resolve
```

is not a sufficient embedding-boundary error. `libbun` owns the Bun-facing facts
and must report them structurally.

## Decision

`libbun` will expose a settled provider-call API as the normal provider-hosting
surface.

The normal provider flow is:

```text
ProviderRequest
  -> libbun imports the selected module
  -> libbun resolves the selected export
  -> libbun invokes the provider
  -> libbun settles the returned value or promise
  -> libbun returns one terminal receipt or one structured failure
```

The host-facing API should have a shape equivalent to:

```rust
pub struct ProviderSettleOptions {
    pub deadline: ProviderDeadline,
}

pub enum SettledProviderReceipt {
    Ready {
        artifact: BunArtifactFingerprint,
        result: ProviderCallResult,
        output: Vec<OutputRecord>,
        settlement: ProviderSettlementDiagnostics,
    },
    Failed(ProviderExecutionFailure),
}

impl BunHost {
    pub fn call_provider_until_settled(
        &mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledProviderReceipt>;
}
```

Exact names may change, but the ownership must not: `libbun` owns module import,
provider invocation, JavaScript promise settlement, deadline enforcement, output
capture, and Bun/JSC/provider-host diagnostics.

## API Separation

The normal provider-host interface must be terminal by construction. It must not
expose:

- `pump_event_loop`;
- `resolve_async`;
- raw parked handles;
- host-side tick counts;
- host-side async polling loops.

Those controls may exist only in a separately named low-level embedding
interface. The low-level interface is for diagnostic tools, future streaming
experiments, or hosts that intentionally take responsibility for Bun event-loop
driving. It is not the ordinary provider-call API.

This is an API boundary, not a documentation preference. A downstream host using
the normal provider-host interface should be unable to accidentally write a
provider completion loop around parked handles.

## Deadline Authority

Settled provider calls require explicit deadline authority.

`libbun` must not hide an unbounded "wait forever" default or silently spin
until a promise settles. If the deadline expires, `libbun` returns a structured
deadline failure with enough diagnostic information to identify the pending
operation.

Deadline diagnostics should include:

```text
operation
provider module specifier or URL
provider export
elapsed_ms
deadline_ms
pending_async_task_count
output_record_count
```

## Diagnostics

`libbun` failures on the settled provider path must be structured enough for a
host to identify the failing layer without reverse engineering the native
adapter.

Required diagnostic dimensions include:

```text
operation:
  runtime_initialize
  adapter_module_load
  provider_module_import
  provider_export_lookup
  provider_factory_validate
  provider_factory_invoke
  provider_callable_validate
  provider_callable_invoke
  provider_promise_settle
  provider_deadline_elapsed

module_specifier_or_url
export_name
provider_domain_class
js_error_name
js_error_message
js_error_stack
pending_async_task_count
elapsed_ms
deadline_ms
output_record_count
```

When Bun/JSC exposes a JavaScript exception, `libbun` preserves the message and
stack. When Bun/JSC does not expose exception details, `libbun` must self-accuse
by reporting that detail extraction failed while still including the operation
and module/export identity that were being evaluated.

Context-free strings such as `module import threw`, `module load failed`, or
`async result did not resolve` are structurally invalid on this path.

## Output Capture

Settled provider calls collect stdout, stderr, and internal log records across:

- module import;
- provider export lookup;
- provider factory invocation;
- provider callable invocation;
- promise settlement;
- provider rejection;
- deadline expiry.

The returned receipt or failure carries the captured output records or an
explicit output-drain failure. Output capture remains host-owned as defined by
ADR-2035 and ADR-2036, but settled provider execution must not lose records
because the call crossed an async boundary.

## Non-Goals

This ADR does not remove the dynamic plugin boundary.

This ADR does not require multiple concurrent Bun runtimes. ADR-2049 remains the
runtime-concurrency law.

This ADR does not define a streaming provider protocol. Streaming may use a
future explicitly low-level or stream-specific interface, but ordinary provider
calls are terminal.

This ADR does not put any downstream application's authority model into
`libbun`. `libbun` remains a generic Bun embedding facade.

## Consequences

Ordinary hosts get a smaller, safer provider API: call once, receive one
terminal receipt or one structured failure.

The current parked-handle API becomes quarantined. It can remain available for
deliberate low-level integrations, but it is not the normal provider-hosting
surface and must not be needed for ordinary provider calls.

The dynamic plugin ABI must grow to support settled provider calls so the Bun VM
owner settles JavaScript promises inside the native runtime boundary.

Diagnostics become a compatibility surface. Once hosts depend on structured
provider failures, `libbun` must preserve the schema or version it explicitly.

## Implementation Plan

1. Add facade types for provider deadlines, settlement diagnostics, settled
   receipts, and structured provider execution failures.
2. Split the normal provider-host interface from the low-level embedding
   interface. The normal provider-host interface exposes settled calls only.
3. Add dynamic plugin ABI functions for settled provider calls.
4. Implement native settled provider execution inside the Bun-owned runtime
   boundary.
5. Preserve output records across synchronous and asynchronous provider phases.
6. Preserve JavaScript error message and stack for module import failures,
   provider factory failures, provider callable failures, and promise
   rejections.
7. Return structured deadline failures for pending promises/tasks.
8. Keep parked handles, event-loop pumping, and raw async resolution only on the
   explicitly low-level embedding interface.
9. Add conformance tests for:
   - synchronous provider success;
   - async provider success;
   - provider promise rejection with stack;
   - module import failure with stack;
   - missing provider export;
   - invalid provider factory;
   - invalid provider callable;
   - deadline expiry with pending async diagnostics;
   - stdout/stderr/log capture before and after awaits.
10. Update public docs so ordinary provider hosts use settled provider calls and
    low-level async controls are documented as advanced/quarantined APIs.

## Acceptance Criteria

- Ordinary provider calls can complete async JavaScript providers without host
  code manually pumping the event loop or polling parked handles.
- The normal provider-host interface does not expose `pump_event_loop`,
  `resolve_async`, or raw parked handles.
- Low-level async controls, if retained, live behind a separately named
  interface and are not required for ordinary provider hosting.
- Dynamic plugin and native adapter tests prove settled async provider success.
- Provider promise rejection returns a structured failure with JavaScript
  message/stack when available.
- Module import failure returns a structured failure with module identity and
  JavaScript message/stack when available.
- Deadline expiry returns a structured failure with elapsed time, deadline, and
  pending async diagnostics.
- Output records are preserved across async provider execution.
- Context-free errors such as `module import threw` and `async result did not
  resolve` are absent from the settled provider-call API.
