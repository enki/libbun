# ADR-2037: libbun Host Environment Overlay

Status: Proposed
Date: 2026-05-18

An earlier `BunRuntimeConfig` shape exposed an `environment` map, but the native
adapter did not apply it. That field has been removed from the active facade
until the embedding semantics are defined. Leaving it in place unused would be
misleading for embedding hosts because providers may observe environment
variables through Bun APIs, process shims, or package/runtime code.

A naive implementation that mutates process-global environment variables during
VM initialization would not be a strong substrate: it would leak across host
tenants, race with other runtimes, and make future native worker-thread or WASM
hosting semantics unclear.

This ADR should define and implement:

- whether `environment` is an overlay, a replacement environment, or only a
  provider-visible configuration map;
- how Bun/JSC environment reads are routed without CLI dispatch;
- how host-provided variables interact with the real process environment;
- lifecycle and thread-safety rules for native runtimes;
- WASM host behavior where the host may provide environment access;
- tests proving provider-visible environment values come from the configured
  host overlay and do not require mutating global process state.

Until this ADR is resolved, host environment overlays are deliberately absent
from the active facade and the native adapter should not be described as
supporting them.
