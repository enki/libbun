# W1-11/W1-12 Retained Prepared-Export Positive Evidence

This is an internal libbun GREEN checkpoint, not W1-11/W1-12 composition
approval. The real SwarmScript path remains pending the process-contained
worker/package producer migration described below.

Dual-Path Evidence:
- Phase: GREEN (libbun internal path); downstream `.ss` GREEN pending.
- Source SHA: `7bdba3f099293bcdb1619d2e1ef83d23b7c94a6a`.
- Implemented mechanical law: non-generic opaque `BunProviderBackend` moves by value into one affine `PreparedExport`; `PreparedExport::drive(self, DriveControl)` returns the closed `Cargo | Cancelled | DeadlineElapsed | MechanicalFault` terminal after retained-runtime readiness, completed retirement, or durable adoption. Typed interrupt, deadline dominance, second invocation, unwind, fallible consuming shutdown, and prepared/terminal Drop custody are owner-tested.
- Rust fixture/test paths: `src/retained_backend.rs`, `tests/prepared_export_lifecycle.rs`, `tests/public_api_boundary.rs`, and `tests/fixtures/prepared_export_privacy/`.
- Focused parallel Nextest command; exit code; result: `cargo nextest run --locked --lib retained_backend::tests`; exit `0`; 11/11 tests passed.
- External surface/compile-fail command; exit code; result: included in `cargo nextest run --locked`; the external surface proof and two compile-fail privacy tests passed. Sibling code cannot construct or clone selected work, clone `PreparedExport`, or import the deleted lease family.
- Broad parallel Nextest command; exit code; result: `ulimit -n 65536; cargo nextest run --locked`; exit `0`; 40/40 tests passed across seven binaries.
- Owning checks: `cargo fmt --all -- --check`, `cargo check --locked`, and `cargo check --locked --features dynamic-loading`; all exited `0` at the source SHA.
- Old-family search: active `src/` and non-negative `tests/` are clean for `ProviderInvocationLease`, `SettledInvocationOutcome`, `ProviderInvocationDescriptor`, `FinishedInvocation`, `InvocationOutputLedger`, `InvocationProfileLedger`, `begin_invocation`, and `finish_invocation`.
- Native consumer check: `cargo check --locked --manifest-path native/Cargo.toml` compiled the changed root libbun crate and then stopped in vendored `bun_core` because generated `vendor/bun/build/release/codegen/build_options.rs` is absent; no diagnostic was attributable to the changed module graph.
- All-features check: stopped in the existing `download-plugin` release checksum gate because v0.2.3 has no committed x86_64 Linux plugin checksum. Default and `dynamic-loading` owning checks are green.
- `.ss` fixture path: Swarm `tests/conformance/ss/provider/libbun_provider_value_json_v1_quiescence.test.ss` remains the preserved RED path. This commit does not install a process-contained worker, package the linked runtime, or migrate the Swarm producer to mint the new opaque selected inputs, so it cannot lawfully claim a real `.ss` GREEN or W1-11/W1-12 composition.
- Post-SHA proof: the exact SHA printed immediately before formatter, both owning checks, focused Nextest, broad Nextest, diff check, and clean status was `7bdba3f099293bcdb1619d2e1ef83d23b7c94a6a`.

## Required successor

Move the retained runtime owner from the current dedicated in-process owner
thread into the reviewed exact-contained worker/package boundary, add the
producer operation that mints the opaque co-branded selected package and
invocation without RAW projection, pin that libbun candidate in Swarm, and run
the existing `.ss` fixture through the Lane-built `swarm` binary. W1-13 remains
outside libbun and must not be implemented by this successor.
