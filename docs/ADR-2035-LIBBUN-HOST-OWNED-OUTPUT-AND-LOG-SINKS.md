# ADR-2035: libbun Host-Owned Output and Log Sinks

Status: Proposed
Date: 2026-05-18

The native adapter can capture Bun stdout/stderr by initializing Bun's output
streams with host-owned files before VM creation and draining them into
`OutputRecord`s. This covers JavaScript console output and Bun output paths that
write through `bun_core::Output`.

The current facade is still weak for reusable host integration because
`BunRuntimeConfig` only supports `Capture` and `Drop`, and it models `log` as a
third sink without a dedicated Bun internal-log hook. Strong substrate hosts need
more explicit output ownership.

This ADR should define:

- whether `log` is a first-class stream distinct from stderr, or a severity tag
  on stderr records;
- host callback/writer APIs for streaming output without polling
  `captured_output`;
- ordering guarantees between stdout, stderr, and internal log records;
- backpressure and failure semantics when host sinks reject writes;
- redaction/ANSI policy and terminal-color behavior;
- thread-safety constraints for native Bun worker threads and future WASM host
  transports;
- conformance tests that verify capture/drop/callback behavior for stdout,
  stderr, and log independently.

Until this ADR is resolved, `libbun` should describe native output capture as
stdout/stderr capture, not as a complete host-owned logging substrate.
