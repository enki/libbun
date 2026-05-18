# ADR-2036: libbun Dedicated Bun Internal Log Stream

Status: Proposed
Date: 2026-05-18

`libbun` exposes `OutputStream::Log` so embedding hosts can receive diagnostic
records that are not provider stdout or stderr. The stable facade and fake
conformance runtime can produce those records, but the native Bun adapter does
not yet have a dedicated hook for Bun's internal scoped/debug logger.

Current native Bun internals route many diagnostics through `bun_core::Output`
and scoped log helpers that ultimately write to the configured output streams.
That means the embedding host can capture them as stdout/stderr bytes, but it
cannot reliably distinguish provider stderr from Bun internal logs or attach
native severity/scope metadata.

This ADR should define and implement:

- whether native Bun internal logs become `OutputStream::Log` records or
  structured stderr records with severity and scope;
- how to hook Bun scoped/debug logging without calling CLI dispatch;
- ordering guarantees relative to provider stdout/stderr records;
- runtime configuration for enabling/disabling Bun debug scopes;
- tests that prove native internal log capture is separate from provider
  stderr.

Until this ADR is resolved, `OutputStream::Log` is a facade capability and
host-adapter extension point, while the native adapter only promises stdout and
stderr capture/drop semantics.
