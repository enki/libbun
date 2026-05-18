# ADR-2035: libbun Host-Owned Output and Log Sinks

Status: Done
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
- `SinkPolicy::Capture` and `SinkPolicy::Drop` are enforced by `BunHost` for
  stdout, stderr, and log records from every runtime;
- the native adapter also applies stdout/stderr policies before records leave
  the adapter, so dropped provider output is not retained by the native runtime.

`OutputStream::Log` is a first-class facade stream. ADR-2036 completes the
native Bun scoped/debug logger integration so Bun internal logs can be captured
or dropped independently from provider stderr.

Evidence:

- `tests/conformance.rs` covers host callback delivery and host-side draining.
- `tests/conformance.rs` covers host-side log drop policy enforcement.
- `native/tests/native_runtime.rs` covers native stdout/stderr/log capture.
- `native/tests/native_output_policy.rs` covers native stdout/stderr/log drop
  policy.
