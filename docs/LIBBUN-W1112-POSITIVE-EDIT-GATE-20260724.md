# W1-11/W1-12 Retained Prepared-Export Positive Edit Gate

Status: implementation gate

Source basis: `2022bafe5fbc37c52293831db3f87aff55b1a399`

The frozen contracts are `LIBBUN-LIFECYCLE-CONTRACT.md`,
`LIBBUN-WORKER-CONTAINMENT-CONTRACT.md`, and
`LIBBUN-WORKER-RELEASE-CONTRACT.md` on the reviewed W1-11/W1-12 contract ref.
This tranche preserves their owner and lifecycle boundary while evolving the
retained backend on current libbun main. Libbun produces one mechanical drive
terminal. Swarm remains the sole owner of authored provider settlement.

## Edit Gate

- Bucket: W1-11/W1-12 positive retained prepared-export owner tranche.
- First source edit: replace the borrow-based invocation lease in
  `src/retained_backend.rs` with an affine `PreparedExport` that consumes the
  retained `BunProviderBackend` owner and one sealed selected invocation.
- Owner boundary: opaque `BunProviderBackend` by value; the runtime, selected
  request, interrupt state, readiness state, and shutdown custody remain
  private.
- Selected input: one non-cloneable selected package/export and one
  non-cloneable invocation payload, kept opaque across the libbun boundary.
- Consuming owner: `BunProviderBackend` admits the selected input and moves
  itself into `PreparedExport`; `PreparedExport::drive(self, DriveControl)` is
  the sole dispatch operation.
- Output product/state: exactly one closed `MechanicalTerminal` in the
  `Cargo | Cancelled | DeadlineElapsed | MechanicalFault` algebra. Cargo is
  authored bytes only; libbun does not interpret or settle provider semantics.
- Final observation owner: the downstream Swarm execution owner consumes the
  authored cargo. Mechanical lifecycle observations remain typed libbun
  terminals.
- One-shot proof: `PreparedExport` and every terminal continuation are
  non-cloneable and expose consuming operations only. Dispatch consumes the
  sole prepared value.
- Existing typed fault: `MechanicalFault` is a closed structured libbun fault;
  JavaScript rejection remains authored cargo and never becomes this fault.
- Forbidden old shape removed: public borrow-based
  `ProviderInvocationDescriptor`, `ProviderInvocationLease`,
  `SettledInvocationOutcome`, `FinishedInvocation`, output/profile ledgers,
  backend state selectors, semantic `ProviderRequest` settlement at the
  retained-owner boundary, and non-consuming idempotent shutdown.
- First stale caller now: retained-backend conformance tests using
  `begin_invocation(...).settle_provider(...).finish()`.
- Tripwire terms: `ProviderInvocationLease`, `SettledInvocationOutcome`,
  `begin_invocation`, `finish_invocation`, public `BackendState`, raw prepared
  parts/selectors, `Clone`/serde on authority products, borrowed shutdown,
  `process::abort`, `Child::wait`, callback proof, placeholder faults, and
  mechanical JavaScript rejection.

## Repair Contract

- Selected input: one sealed selected provider request whose package/export
  correspondence and invocation payload cannot be separately replayed.
- Consuming owner: `BunProviderBackend` owns admission, dispatch, readiness,
  retirement, restart, and shutdown.
- Prepared product: opaque affine `PreparedExport` bound to exactly that
  backend and selected invocation.
- Interrupt/control: `DriveControl` owns the deadline and a shared typed
  interrupt signal; `DriveInterrupt` may request cancellation but cannot mint
  a terminal or recover custody.
- Receipt/Fault: `MechanicalTerminal` is closed over authored cargo,
  cancellation, deadline, and `MechanicalFault`; all variants are constructed
  only after the drive is quiescent or the runtime is terminally shut down.
- Cancellation/deadline: deadline construction is checked; deadline wins when
  deadline and interrupt are observed together. Cancellation or deadline that
  cannot prove same-runtime readiness consumes shutdown/retirement before its
  terminal is published.
- Retry: only a terminal that privately owns a proved ready or restartable
  continuation may admit the next sealed invocation. No terminal returns raw
  backend or runtime parts.
- Unwind: dispatch is wrapped by an owner guard. Unwind invalidates readiness
  and transfers the backend to the same terminal shutdown/disposal path before
  unwind resumes.
- Drop: dropping prepared, active, or terminal continuation custody performs
  only its already-staged infallible disposal action. Drop emits no terminal,
  observation, semantic receipt, callback, or restart authority.
- Shutdown: shutdown consumes its owner and returns one typed
  `BackendShutdownTerminal`; it is fallible and cannot be retried from a
  consumed backend.
- Private phases: runtime loading/call/pump, interrupt observation, output
  drain, readiness proof, retirement, and disposal stay inside the owner.
- Deletion order: introduce the complete consuming owner and hostile tests;
  migrate retained conformance callers; delete borrow leases and public
  semantic ledgers/selectors; then run stale-shape and privacy searches.

## Hostile Acceptance Evidence

Focused default-parallel tests must cover affine dispatch, fulfilled and
rejected authored cargo, cancellation, deadline dominance, late-output fault,
shutdown failure, second invocation through a sealed continuation, dropped
undispatched prepared work, dropped terminal continuation, and unwind custody.
The owning crate must also pass `cargo check --locked`, workspace Nextest with
default parallelism, `git diff --check`, and stale public-API searches.
