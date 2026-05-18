# ADR-2034: libbun Prepared Bundle Artifact Contract

Status: Proposed
Date: 2026-05-18

`BunModuleSpec::PreparedBundle` currently names an embedding capability but does
not define a portable artifact format. Treating arbitrary bytes as source text
or as a Bun CLI compile artifact would make `libbun` appear reusable while
leaking unstable implementation details to downstream hosts.

Before the native adapter accepts prepared bundles, `libbun` needs a documented
artifact contract:

- bundle format identifier and version;
- Bun source revision and `libbun` ABI compatibility metadata;
- entry module identity and export table expectations;
- integrity/fingerprint fields suitable for receipts and cache keys;
- host-independent path, environment, and external transport assumptions;
- validation behavior and structured errors for incompatible artifacts;
- tests that produce an artifact through the supported producer and load it
  through `BunModuleSpec::PreparedBundle` without invoking Bun CLI dispatch.

Until this ADR is resolved, the native adapter must keep returning a structured
module-load error for `PreparedBundle` rather than accepting ambiguous bytes.
