# ADR-2036: libbun Dedicated Bun Internal Log Stream

Status: Done
Date: 2026-05-18

`libbun` exposes `OutputStream::Log` so embedding hosts can receive diagnostic
records that are not provider stdout or stderr. The native Bun adapter now
routes Bun internal scoped/debug output to that stream instead of mixing it into
provider stderr.

## Decision

Native Bun internal logs are emitted as `OutputStream::Log` records.

The native adapter initializes Bun stdout and stderr with host-owned capture
files before VM creation, then installs a dedicated scoped debug writer backed
by a separate host-owned log capture file. Bun scoped logging therefore remains
inside Bun's normal `bun_core::Output` machinery while giving `libbun` a stream
boundary that hosts can capture or drop independently.

`SinkPolicy::Capture` and `SinkPolicy::Drop` apply to the log stream the same
way they apply to stdout and stderr. Dropped log records are not retained by the
native runtime after draining.

The adapter drains streams in a stable order: stdout, stderr, then log. This is
a drain-order guarantee, not a cross-stream byte-ordering guarantee. Hosts that
need causal ordering should treat records from different streams as separate
channels unless a future adapter adds explicit timestamps or sequence numbers.

`libbun` does not add a new CLI-dispatch path for enabling Bun debug scopes.
Scope visibility remains controlled by Bun's own compile-time and runtime debug
mechanisms. The adapter only supplies the destination for records that Bun
emits.

## Consequences

Provider stderr and Bun internal diagnostics are distinguishable in the public
facade. Hosts can store, redact, display, or drop Bun internals without
changing provider output policy.

The current log stream is byte-oriented text output. It does not expose native
Bun severity, file, line, or structured scope metadata beyond the emitted log
message.

## Evidence

- `native/src/lib.rs` adds a third native `OutputCapture` for log records and
  wires Bun's scoped debug writer to that file.
- `native/src/lib.rs` drains stdout, stderr, and log records into distinct
  `OutputRecord` streams.
- `native/tests/native_runtime.rs` proves a native module load emits an
  `OutputStream::Log` record separate from provider stdout/stderr.
- `native/tests/native_output_policy.rs` proves log records honor
  `SinkPolicy::Drop`.
