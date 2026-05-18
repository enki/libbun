# ADR-2035: libbun Host-Owned Output and Log Sinks

Status: In Progress
Date: 2026-05-18

The native adapter can capture Bun stdout/stderr by initializing Bun's output
streams with host-owned files before VM creation and draining them into
`OutputRecord`s. This covers JavaScript console output and Bun output paths that
write through `bun_core::Output`.

`libbun` now also gives hosts an ownership boundary above the runtime adapter:

- `BunEmbeddingRuntime::drain_captured_output` transfers pending records out of
  the runtime instead of requiring unbounded adapter-side accumulation;
- `BunHost` keeps host-owned output history and exposes
  `drain_captured_output`;
- `BunHost::initialize_with_output_handler` delivers newly captured records to a
  host callback after initialization, module load, export calls, event-loop
  pumps, async resolution, and shutdown;
- `SinkPolicy::Capture` and `SinkPolicy::Drop` are independently honored by the
  native stdout/stderr capture path.

Remaining weakness:

- `OutputStream::Log` is a first-class facade stream for adapters that can
  produce host logs, but the native Bun scoped/debug logger does not yet have a
  dedicated hook separate from stderr;
- ordering guarantees between stdout, stderr, and internal log records;
- backpressure and failure semantics when host callbacks reject writes are not
  modeled because callbacks are currently infallible `FnMut(OutputRecord)`;
- redaction/ANSI policy and terminal-color behavior are not yet configurable;
- future native worker-thread and WASM host transports need explicit
  cross-thread delivery constraints.

Evidence:

- `tests/conformance.rs` covers host callback delivery and host-side draining.
- `native/tests/native_runtime.rs` covers native stdout/stderr capture.
- `native/tests/native_output_policy.rs` covers native stdout/stderr drop
  policy.

Until the remaining weakness is resolved, `libbun` should describe native output
capture as stdout/stderr capture plus host-owned delivery, not as a complete
dedicated Bun internal logging substrate.
