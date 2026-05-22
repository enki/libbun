# ADR-2053: libbun Retained Backend and Affine Invocation Leases

Status: Done
Date: 2026-05-22

Related: ADR-2035, ADR-2049, ADR-2050, ADR-2052

## Context

ADR-2049 establishes the current runtime-concurrency law: a process may hold at
most one live in-process libbun runtime lease unless a future adapter proves
multiple isolated Bun runtimes. Hosts that need multiple provider calls should
load and dispatch them through the same runtime.

ADR-2050 establishes the normal provider-call API as settled and terminal:
libbun owns module import, provider invocation, promise settlement, deadline
enforcement, output capture, and diagnostics. Hosts should not drive raw async
handles or event-loop ticks for ordinary provider calls.

The missing piece is retained backend reuse with per-invocation isolation. The
current `BunHost` API supports:

- host-wide output history through `drain_captured_output`;
- optional diagnostic callback delivery;
- settled provider receipts that include output collected during the call.

That is enough for simple one-call hosts. It is not the clean shape for a native
runner that keeps one libbun runtime hot while executing many independent
provider invocations. The output handler is fixed at runtime initialization, but
runner integrations need a fresh output/capture scope for each invocation. If
the downstream host builds its own active-invocation router around a retained
`BunHost`, the downstream host becomes responsible for libbun lifecycle
invariants:

- which invocation owns output;
- whether late output is legal;
- whether an active invocation remains after settlement;
- whether a failed capture finalization poisons the backend;
- whether the backend can be reused after a runtime or output-drain failure;
- when shutdown is mandatory.

Those are libbun embedding concerns. Downstream hosts should supply invocation
identity, request, deadline, and output policy. libbun should own the Bun runtime
backend, invocation lease, output ledger, diagnostics, and poison state.

## Decision

libbun will expose a retained backend API with affine per-invocation leases.
This is a hard cut for retained provider execution. Retained hosts must use this
API directly; libbun must not support a parallel normal path where downstream
integrations keep a raw `BunHost` hot and reconstruct invocation ownership with
callbacks, drained output, mutexes, or string ids.

```rust
let mut backend = BunProviderBackend::<R>::open(config)?;

let lease = backend.begin_invocation(ProviderInvocationDescriptor {
    invocation_id,
    output_policy,
    diagnostics_policy,
})?;

let outcome = lease.settle_provider(request, options)?;
let finished = outcome.finish()?;

backend.shutdown()?;
```

Names may change, but the ownership must not:

- `BunProviderBackend` owns one live Bun runtime lease and backend state;
- `ProviderInvocationLease` owns one active provider invocation;
- `SettledInvocationOutcome` owns the terminal provider receipt plus invocation
  output ledger until finished;
- `finish()` consumes the invocation outcome and returns finished output,
  diagnostics, and invocation profile projections;
- the backend is either reusable or poisoned after the invocation is consumed.

This is not a framework for downstream application authority. libbun remains a
generic Bun embedding facade. The lease API gives generic provider hosts a safe
way to retain a Bun backend while keeping each call isolated.

## Authority Discipline

The retained backend is a process-local runtime lease. It is not cloneable,
copyable, or shareable as semantic authority.

```text
BunProviderBackend
  state: Ready | Active | Poisoned | Shutdown

ProviderInvocationLease
  affine; consumed by settle, fail, cancel, or poison

SettledInvocationOutcome
  affine; consumed by finish

InvocationOutputLedger
  evidence; belongs to exactly one invocation

InvocationProfileLedger
  evidence; belongs to exactly one invocation
```

Rules:

- only a `Ready` backend can begin an invocation;
- beginning an invocation moves the backend to `Active`;
- a second invocation while `Active` fails immediately with a structured error;
- `ProviderInvocationLease` must be consumed exactly once;
- `finish()` must prove no active capture remains;
- runtime initialization, module import, call, promise settlement, output drain,
  or shutdown failures that leave the backend state uncertain poison the backend;
- provider rejection is a terminal provider receipt, not backend poison by
  itself;
- a `Poisoned` backend cannot execute another invocation and may only project
  diagnostics or shut down;
- `Shutdown` is terminal.

The implementation may approximate linear/affine ownership with Rust private
fields, sealed constructors, consuming methods, and explicit backend state
transitions.

## Invocation Output

Output capture must be invocation-owned.

`initialize_diagnostic_with_output_handler(...)` is a low-level diagnostic API.
Its name is intentionally not the normal retained-backend path, and downstream
hosts must not use it to build invocation routers. The normal retained-backend
API does not require a handler fixed at runtime initialization. Each invocation
supplies its own output policy and receives a finished output ledger.

The output ledger should include:

```text
invocation_id
records[]
record_count
late_output_policy
late_output_count
drain_failures[]
diagnostics_snapshot
```

Late output must be explicit. The default policy is poison: if output appears
after the invocation has finished and before another invocation begins, libbun
must mark the backend poisoned or return a structured late-output fault before
reuse. Future policies may quarantine or attach late output, but they must be
explicit and visible in diagnostics.

If single-call APIs expose provider-call output, that output is a projection of
the same invocation ledger. It is not a second authority surface. The
retained-backend API's authoritative output boundary is the invocation output
ledger.

## Invocation Profiling

Retained provider profiling is libbun-owned. Downstream hosts must not wrap
libbun calls with their own timers and present those timings as libbun phase
facts. libbun owns module import, provider export lookup, call dispatch, async
settlement, event-loop pumping, output capture, diagnostics snapshot, backend
finish, and poison transitions, so libbun must emit the profile spans for those
phases.

`FinishedInvocation` carries:

```rust
pub struct InvocationProfileLedger {
    pub schema: String,
    pub invocation_id: String,
    pub spans: Vec<InvocationProfileSpan>,
}
```

The initial span set is:

```text
backend_begin_invocation
provider_module_load
provider_export_lookup
provider_call_dispatch
provider_settlement
provider_event_loop_pump
provider_call_settlement
output_ledger_finish
diagnostics_snapshot
backend_finish_invocation
backend_poison
```

Some spans are absent when the corresponding phase does not occur. Absence is a
fact of the retained backend's execution path, not a license for downstream
hosts to synthesize the missing phase. Downstream runners may attach their own
test/body context to these spans, but they may not invent libbun-internal timing.

## API Shape

Target public surface:

```rust
pub struct BunProviderBackend<R: BunEmbeddingRuntime> { ... }

pub struct ProviderInvocationDescriptor {
    pub invocation_id: String,
    pub output_policy: InvocationOutputPolicy,
    pub diagnostics_policy: InvocationDiagnosticsPolicy,
}

pub struct ProviderInvocationLease<'a, R: BunEmbeddingRuntime> { ... }

pub struct SettledInvocationOutcome<'a, R: BunEmbeddingRuntime> { ... }

pub struct FinishedInvocation {
    pub invocation_id: String,
    pub receipt: SettledProviderReceipt,
    pub output: InvocationOutputLedger,
    pub diagnostics: ProviderRuntimeSnapshot,
    pub profile: InvocationProfileLedger,
}
```

Essential methods:

```rust
impl<R: BunEmbeddingRuntime> BunProviderBackend<R> {
    pub fn open(config: BunRuntimeConfig) -> LibbunResult<Self>;
    pub fn begin_invocation(
        &mut self,
        descriptor: ProviderInvocationDescriptor,
    ) -> LibbunResult<ProviderInvocationLease<'_, R>>;
    pub fn state(&self) -> BackendState;
    pub fn diagnostics_snapshot(&self) -> ProviderRuntimeSnapshot;
    pub fn shutdown(&mut self) -> LibbunResult<()>;
}

impl<R: BunEmbeddingRuntime> ProviderInvocationLease<'_, R> {
    pub fn settle_provider(
        self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledInvocationOutcome<'_, R>>;
}

impl<R: BunEmbeddingRuntime> SettledInvocationOutcome<'_, R> {
    pub fn finish(self) -> LibbunResult<FinishedInvocation>;
}
```

The exact lifetime encoding may change if Rust borrow constraints require a
different typestate shape, but the public API must keep the same authority:
backend state is retained, invocation state is affine, and output is
invocation-owned.

## Poison

The following are forbidden:

- downstream hosts manually routing output records from a retained `BunHost`
  through application-specific active-invocation state for the normal provider
  path;
- retaining a `BunHost` and swapping global output handlers between calls;
- treating callback delivery, drained output history, string ids, or downstream
  mutex state as invocation authority;
- silently dropping late output after an invocation has finished;
- allowing a backend to execute after output-drain failure, uncertain runtime
  state, unfinished invocation, or failed shutdown transition;
- blocking on a second invocation when one is already active;
- representing invocation ownership as an unsealed JSON object, string id, or
  host convention;
- downstream hosts synthesizing libbun-internal profile spans with outside
  timers;
- keeping raw-host retained execution available as a compatibility path after the
  leased backend exists.

## Implementation Plan

2026-05-22 substrate weakness ledger:

- Existing `BunHost::call_provider_until_settled` returns a terminal receipt, but
  retained callers still have to reason about backend readiness, active
  invocation state, late output, and poison by convention.
- The first retained-backend implementation must not prove invocation ownership by
  comparing terminal receipt output with a later host-wide drain. The retained
  backend must call a libbun-internal settlement path where the active invocation
  ledger is the source of truth and any post-settlement host output is immediate
  poison.
- Diagnostic callback delivery is host-wide and cannot represent a fresh
  invocation lease.
- Downstream hosts can therefore accidentally build active-call routers around a
  retained `BunHost`. That is the bad state ADR-2053 removes.
- Downstream hosts can also accidentally wrap libbun as a black box and label the
  elapsed time as a libbun invocation profile. That is not admitted telemetry; the
  retained backend must emit its own invocation phase ledger.
- The retained dynamic backend reintroduced a severe runtime-startup regression by
  computing the native plugin SHA-256 inside `DynamicPlugin::load`. That made the
  first provider call in every fresh host process read and hash the full plugin
  binary. This was diagnostic ceremony, not execution authority. Artifact
  verification belongs to release/install/build workflows. Runtime provider startup
  must not compute, read back, or reconstruct plugin fingerprint metadata.

The implementation must hard-cut retained provider execution to the new
retained-backend module rather than expanding or preserving the raw host callback
path as a normal integration surface.

1. Add `BackendState`, `ProviderInvocationDescriptor`,
   `InvocationOutputPolicy`, `InvocationOutputLedger`, and structured backend
   poison errors.
2. Implement `BunProviderBackend<R>` as the normal retained-runtime owner above
   `BunEmbeddingRuntime`.
3. Move settled provider execution behind `ProviderInvocationLease`.
4. Route all runtime output collection during module load, export call, async
   settlement, event-loop pump, failures, and shutdown into the active invocation
   ledger. `SettledProviderReceipt::output` is projected from that ledger for
   legacy single-call surfaces; it is not a second source of authority.
5. Add `InvocationProfileLedger` to `FinishedInvocation` and emit libbun-owned
   spans from retained backend phase transitions. Downstream wrappers must be
   unable to produce normal-path libbun phase timings.
6. Make late output before the next invocation poison the backend by default.
7. Remove native plugin fingerprint diagnostics from the runtime load path.
   `DynamicPlugin::load` must not read the plugin binary to populate diagnostics,
   and retained backend startup must not replace that with JSON metadata reads or
   resolver readback on the provider-call path. Release/install checksum
   verification remains in the release/install workflows that already own artifact
   trust.
8. Quarantine `BunHost` and `LowLevelBunHost` as low-level diagnostic/internal
   APIs for raw embedding. The leased backend is the normal retained provider
   API; downstream retained-runner integrations must not build output routers
   around raw host callbacks.
9. Add conformance tests:
   - one backend runs multiple provider calls serially;
   - a second active invocation fails immediately;
   - output from each call belongs only to that invocation;
   - invocation profile spans are emitted by libbun and attached to
     `FinishedInvocation`;
   - dynamic plugin load does not compute or read plugin fingerprint metadata on
     the provider-call path;
   - provider rejection does not poison the backend;
   - output-drain/runtime failure poisons the backend;
   - poisoned backend cannot execute another call;
   - shutdown consumes the backend cleanly.

## Implementation Progress

2026-05-22:

- Added `src/retained_backend.rs` with `BunProviderBackend`,
  `ProviderInvocationDescriptor`, affine `ProviderInvocationLease`,
  `SettledInvocationOutcome`, `FinishedInvocation`, `InvocationOutputLedger`,
  backend state, and structured poison diagnostics.
- Added `LibbunError::BackendState` for self-accusing retained-backend faults.
- Retained backend calls a libbun-internal settlement path and treats
  post-settlement host output as poison. It no longer proves ownership by
  comparing terminal receipt output with a later host-wide drain.
- Public `BunHost::call_provider_until_settled` and `LowLevelBunHost` provider
  calls now also fail closed when output appears after the terminal receipt
  ledger has settled. They do not silently retain post-settlement output as a
  second authority surface.
- `InvocationOutputPolicy::Drop` hides public output record bodies without
  erasing `record_count`, so diagnostics can still prove output existed.
- Added conformance tests for module-load output, sync call output, async
  settlement output, provider rejection output, dropped leases, dropped settled
  outcomes, drop-output projection, shutdown, public host late-output rejection,
  and late/post-settlement output poison.
- Added `InvocationProfileLedger` and `InvocationProfileSpan` to the retained
  backend API. `FinishedInvocation` now carries libbun-owned invocation spans,
  including backend begin/finish, provider diagnostic phases, provider
  settlement, output ledger finish, and poison when the backend becomes unsafe.
- `BackendPoisonDiagnostic` retains the invocation profile when poison happens
  after an invocation lease exists. Failure paths therefore keep libbun-owned
  phase evidence instead of forcing downstream hosts to reconstruct timing.
- Removed runtime plugin fingerprint diagnostics from `DynamicPlugin::load`.
  Runtime startup no longer reads and hashes the native plugin, and it does not
  replace that with release metadata JSON reads. Plugin checksums remain release
  artifact verification, not provider-call diagnostics.
- Renamed the raw callback surface to
  `initialize_diagnostic_with_output_handler(...)` for `BunHost` and
  `LowLevelBunHost`. The old normal-looking
  `initialize_with_output_handler(...)` API is gone, so downstream retained
  hosts cannot accidentally choose callback routing as the ordinary retained
  execution shape.
- Downstream Swarm/ss profile proof now preserves full libbun
  `InvocationProfileLedger` values instead of lossy bare spans. A Proving
  `ss test --profile --reporter json` run passed 4 tests and projected two
  resource-release invocation ids,
  `foreign-resource-release:ss-libbun-external-scope-release-1` and
  `foreign-resource-release:ss-libbun-external-scope-release-2`, with no
  `libbun_invocation` span missing its `libbunInvocationId`.
- Added conformance assertions that successful retained invocations carry
  non-empty libbun profile spans and that post-settlement output poison retains
  the matching invocation profile.
- A second active invocation is unrepresentable in normal safe Rust because
  `ProviderInvocationLease` holds the mutable backend borrow until consumed. The
  retained backend still carries an explicit `Active` state poison for internal
  state drift and future non-Rust FFI surfaces.
- Verified `cargo test --manifest-path Cargo.toml --test conformance --
  --nocapture`: 27 tests passed.

Closure verification:

- `cargo test --manifest-path ../libbun/Cargo.toml --quiet` passed: retained
  backend conformance and release verification remain green.
- Downstream Swarm/ss profile proof passed in Proving: `ss test --profile
  --reporter json` passed 4 tests and projected two
  `foreign-resource-release:*` libbun invocation ledgers with first-class
  invocation ids.
- The CLI fast path remains clean: a fresh `ss ../proving/iotest.ss` process
  completed in roughly 67-70 ms locally after runtime plugin hashing and metadata
  readback were removed.

## Acceptance Criteria

- A host can keep one libbun backend open and execute multiple settled provider
  calls through distinct affine invocation leases.
- Each invocation returns a finished output ledger containing only its own output.
- Starting a second invocation while one is active fails with a structured error.
- Late output after invocation finish is structurally diagnosed and prevents
  silent backend reuse.
- Backend poison state is explicit and prevents further provider execution.
- The existing settled provider diagnostics remain available.
- The API contains no downstream application-specific concepts.

## Consequences

Downstream hosts can keep libbun hot without owning libbun lifecycle internals.
They no longer need to build output routers, active-call mutexes, late-output
policies, or backend poison handling around a raw `BunHost`.

This improves performance-sensitive native runners while preserving ADR-2049's
single-runtime law and ADR-2050's settled provider-call boundary.
