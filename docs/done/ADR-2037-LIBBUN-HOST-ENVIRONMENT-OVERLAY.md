# ADR-2037: libbun Host Environment Overlay

Status: Done
Date: 2026-05-18

`BunRuntimeConfig` includes an `environment` map so embedding hosts can provide
provider-visible configuration without mutating the process-global environment.

## Decision

`BunRuntimeConfig.environment` is an overlay, not a replacement environment.

The native adapter keeps Bun's normal process environment visible and applies
host-provided entries over it during VM initialization. Overlay entries are
written into Bun's per-VM environment map, so `process.env` and `Bun.env`
inside the provider can observe them without requiring `std::env::set_var` or
any other global process mutation.

The overlay is immutable after runtime initialization. Hosts that need a
different environment create a new runtime with a different
`BunRuntimeConfig`.

Environment keys are rejected when empty or when they contain `=` or NUL bytes.
Values are also rejected when they contain NUL bytes. These rules keep the
overlay compatible with platform environment conventions and Bun's key/value
storage.

The stable facade carries the same configuration for all runtimes. Runtimes
that do not expose provider-visible environment access may retain the map as
configuration metadata, but they must not silently mutate the host process to
simulate support.

## Consequences

Embedding hosts can provide tenant-specific or request-specific environment
values to Bun providers without leaking those values into sibling runtimes or
the host process.

The native implementation currently applies the overlay at VM creation. It does
not provide a mutable environment API for already-created runtimes.

## Evidence

- `src/lib.rs` defines `BunRuntimeConfig.environment` and
  `BunRuntimeConfig::with_environment_overlay`.
- `native/src/lib.rs` validates overlay keys/values and applies them to Bun's
  per-VM environment map during initialization.
- `native/tests/native_runtime.rs` proves provider code can read overlay values
  through `process.env` and `Bun.env`.
- `native/tests/native_runtime.rs` also proves the overlay does not require the
  value to exist in `std::env`.
