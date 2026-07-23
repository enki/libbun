# libbun Retained Backend And Prepared-Export Lifecycle Contract

Status: frozen implementation contract

Date: 2026-07-23

## Scope

This document owns the semantic lifecycle of one retained Bun provider backend
and every one-shot prepared export admitted through it. Worker containment and
release proof are specified separately but are part of the same completion
gate.

## Semantic Abstraction Gate

- Unit: one retained Bun provider backend, its exact worker session, and an
  invocation selected by the source producer.
- Highest owner: opaque `BunProviderBackend` by value.
- Selected inputs: producer-minted `SelectedProviderPackage` and
  `ProviderInvocation`, each generatively branded to the same selection.
- Receipt/fault: a closed terminal product carrying a sealed backend
  continuation plus authored cargo, cancellation/deadline evidence, or a typed
  mechanical fault. A live-worker continuation is proved separately from a
  retired-worker continuation.
- Private phases: package correspondence, worker offer/reservation, request
  framing, execution, output capture, cancellation, retirement, quarantine,
  and invocation-ready proof.
- Too low: wire frames, export strings, artifact bytes, invocation bytes,
  process ids, job/group handles, pipe parts, callbacks, or receipts supplied by
  callers.
- Too high: a generic embedding runtime, generic workflow/session framework,
  public module/promise handles, or public event-loop operations.

## Sealed Producer Inputs

The source/package owner mints both products after it has selected and checked
the package, export, and invocation. Their brand proves generative
correspondence; it is not a public string, digest getter, id, or boolean.

```rust
pub struct SelectedProviderPackage<Brand> {
    sealed: private::SelectedPackageCustody<Brand>,
}

pub struct ProviderInvocation<Brand> {
    sealed: private::InvocationCustody<Brand>,
}
```

The concrete representation and mint operation must be co-located with the
producer owner or moved with that ownership. There is no public:

- `admit(Vec<u8>)`, `new(Vec<u8>, String, Vec<u8>)`, or equivalent raw mint;
- export/path/name/fingerprint selector getter;
- `from_parts` or `into_parts`;
- `Clone`, `Copy`, `Default`, serde, or replay operation; or
- borrowed method that can mint another authority product.

Libbun consumes the sealed products in one finite owner operation. It does not
ask the caller to unpack them or reconstruct their correspondence.

## Backend State And Admission

`BunProviderBackend` owns one retained worker session, a restartable continuation
whose prior worker is proved dead, or its poison/quarantine continuation. Its
fields and variants are private.

```rust
pub struct BunProviderBackend {
    state: private::BackendState,
}

enum BackendState {
    Ready(RetainedCustody),
    Restartable(RetiredCustody),
    Poisoned(PoisonedBackend),
    Quarantined(QuarantinedBackend),
    ShuttingDown(ShutdownCustody),
}
```

Admission consumes the backend, package, and invocation:

```rust
impl BunProviderBackend {
    pub fn prepare<Brand>(
        self,
        package: SelectedProviderPackage<Brand>,
        invocation: ProviderInvocation<Brand>,
        control: AdmissionControl,
    ) -> PreparedAdmission<Brand>;

    pub fn shutdown(self, control: ShutdownControl) -> BackendShutdownTerminal;
}
```

The public algebra is closed. Payload fields remain private.

```rust
pub enum PreparedAdmission<Brand> {
    Admitted(PreparedExport<Brand>),
    Refused(PreparedRefusal<Brand>),
    Fault(AdmissionFaultTerminal<Brand>),
}
```

The retained worker first receives a bounded private admission envelope. The
offer contains no package bytes, export selector, invocation cargo, or selected
work authority. It answers with the same session epoch:

- `Refused`: a sealed `OfferReadyProof` proves that no reservation or
  invocation authority moved to the worker and that the same live session
  remains Ready. The refusal product owns that proof, the same backend,
  package, and invocation and exposes `retry(self, control)` and
  `shutdown(self, control)` finite operations.
- `Reserved`: this is the exact admission point. The resulting
  `PreparedExport` owns the backend continuation, session reservation, package,
  and invocation. Reservation does not dispatch those selected inputs; the
  consuming `drive` operation does.
- transport ambiguity or malformed correspondence: typed admission fault. It
  is never a refusal. The terminal owns the backend continuation and exposes
  only lawful recovery or shutdown operations.

No terminal exposes separate backend/session/request parts. A caller cannot
mix a backend from one terminal with an invocation from another.

## Prepared Export

`PreparedExport` is affine and non-serializable. It exists only after a retained
worker has accepted the offer and reserved the exact invocation. It can be
driven once, retried only through a valid refusal product, have its reservation
cancelled before invocation dispatch by consuming it, or be dropped into
quarantine.

```rust
pub struct PreparedExport<Brand> {
    custody: Option<private::PreparedCustody<Brand>>,
}

impl<Brand> PreparedExport<Brand> {
    pub fn drive(self, control: DriveControl) -> MechanicalTerminal<Brand>;
    pub fn cancel_before_dispatch(self, control: CancelControl)
        -> MechanicalTerminal<Brand>;
}
```

Worker creation belongs to backend startup, before the bounded offer. After
reservation, private `DriveCustody` owns all invocation lifecycle facts and the
sealed retained-worker continuation. A public terminal can be minted only
after that custody is consumed into the proof required by the transition below
or transferred intact into `RetirementQuarantine`.

## Closed Readiness And Retirement Proofs

The proof products are private, sealed, affine, and generatively tied to the
exact backend session and invocation. They have no public constructors, fields,
parts projections, serde, clone, selector getters, or borrowed authority mints.

- `OfferReadyProof<Brand>` proves that a refused offer transferred no selected
  work or reservation and that the same worker and session epoch remain Ready.
  It is the only proof from which retry may be offered.
- `InvocationReadyProof<Brand>` proves that one exact reserved invocation is
  settled and drained, its reservation is closed, no promise/module/cancel
  authority remains, its output barrier and ledger are complete, no late output
  entered idle capture, and the same worker is alive and Ready at the same
  epoch. It is the only proof that permits reuse of that worker.
- `RetirementProof` proves that the worker is dead, exact containment is empty,
  the leader is reaped, worker protocol and diagnostics reached EOF, channels
  are closed, persistent pumps are at EOF and joined, and no process, pipe,
  receiver, containment, or join custody remains. It can return only a
  `Restartable` backend continuation with no live session.
- `RetirementQuarantine` owns unresolved retirement custody intact. It proves
  neither Ready nor retired and permits no reuse or retry.

An `InvocationReadyProof` and a `RetirementProof` are mutually exclusive for
one session transition: the former proves the worker remains alive; the latter
proves it is dead. Neither can be reconstructed from protocol frames, exit
status, ids, booleans, or observation data.

The transition evidence is frozen as follows:

| Transition | Required sealed evidence | Backend continuation |
| --- | --- | --- |
| Retry a refused offer | `OfferReadyProof` | Same live Ready worker, same epoch, unchanged package/invocation |
| Fulfilled cargo | `InvocationReadyProof` | Same live Ready worker and epoch |
| Rejected cargo | `InvocationReadyProof` | Same live Ready worker and epoch |
| Cancel reserved work before dispatch | `InvocationReadyProof` | Same live Ready worker and epoch |
| Cooperative in-drive cancellation | `InvocationReadyProof` | Same live Ready worker and epoch |
| Forced cancellation | `RetirementProof` | `Restartable`, with no live worker |
| Deadline | `RetirementProof` | `Restartable`, with no live worker |
| Supervisor unwind | `RetirementProof` or `RetirementQuarantine` | `Restartable` after proof; otherwise quarantined |
| Active-worker shutdown | `RetirementProof` or `RetirementQuarantine` | No post-shutdown backend after proof; otherwise quarantined |
| Shutdown of `Restartable` | Prior `RetirementProof` already sealed in the continuation | No post-shutdown backend |
| Drop of live custody | No proof may be fabricated by Drop; durable reaper must later produce `RetirementProof` before deletion | No public continuation; intact custody remains queued until proof |

Failure to prove Ready forces the typed-fault retirement path. Failure to prove
retirement transfers intact custody to `RetirementQuarantine` and returns only
the typed quarantine fault. Neither failure may substitute cargo, cancellation,
deadline, retry, or shutdown success from another row of this table.

## Closed Terminal Algebra

```rust
pub enum MechanicalTerminal<Brand> {
    Cargo(CargoTerminal<Brand>),
    CancelObserved(CancelTerminal<Brand>),
    DeadlineExpired(DeadlineTerminal<Brand>),
    Fault(FaultTerminal<Brand>),
}
```

Each terminal privately owns exactly one backend continuation. It offers finite
consuming operations appropriate to its state:

- fulfilled or rejected cargo: privately consume `InvocationReadyProof`,
  observe final authored cargo, then reuse the same live Ready worker for the
  next branded admission or consume it into shutdown;
- cooperative cancellation, including reservation cancellation before
  dispatch: privately consume `InvocationReadyProof`, then reuse the same live
  Ready worker or consume it into shutdown;
- forced cancellation or deadline: privately consume `RetirementProof`, then
  continue only with a `Restartable` backend that owns no live worker, or
  consume that continuation into shutdown;
- refusal: retry the same package/invocation/backend or shut it down;
- poisoned/quarantined fault: query bounded final diagnostics and invoke the
  finite recovery/shutdown operation allowed by that state.

There is no public `into_parts`, backend getter, session getter, raw handle,
generic callback, or caller-supplied proof. Observation methods cannot feed
authority minting.

## Provisional Selection And Final Proof

Candidate cargo, cancellation, deadline, worker fault, protocol fault, and
supervisor unwind are private provisional facts. None is public evidence.

Finalization rules:

1. Fulfilled or rejected cargo requires a complete nonempty authored frame and
   `InvocationReadyProof`. Worker exit or retirement can never prove cargo.
2. Cooperative cancellation requires an exact cancellation/reservation-release
   acknowledgement followed by `InvocationReadyProof`.
3. Forced cancellation or deadline requires `RetirementProof`; it returns a
   `Restartable` continuation and never claims that the retired session is
   Ready.
4. A mechanical fault may preserve the same worker only when the invocation
   state still reaches `InvocationReadyProof`. Otherwise it forces retirement;
   its fault terminal is minted only after `RetirementProof`, or it returns a
   quarantine fault holding `RetirementQuarantine`.
5. Supervisor unwind always forces retirement. It produces a typed unwind fault
   after `RetirementProof`, or a typed quarantine fault; it never produces
   `InvocationReadyProof`.
6. Foreground retirement timeout transfers intact custody to quarantine. It
   never mints cargo, cancellation, deadline, Ready, or retired evidence.
7. A retirement failure that later recovers remains observable as a typed
   retirement fault; success is not retroactively fabricated.

## Authored Settlement And Mechanical Faults

JavaScript fulfillment and rejection are authored settlement cargo:

```rust
pub enum AuthoredSettlementCargo {
    Fulfilled(FulfilledCargo),
    Rejected(RejectedCargo),
}
```

The encoding is a bounded, nonempty, closed authored format. It is not public
JSON, `serde_json::Value`, an empty buffer, a default value, or a string status.
The semantic consumer interprets this final cargo; libbun does not convert it
into provider semantics.

Mechanical faults are structured typed faults. Required distinct classes
include:

- package/export correspondence and preparation;
- missing or non-callable selected export;
- invocation lowering;
- undefined or unserializable result/cargo extraction;
- worker admission and containment;
- request/terminal/output protocol;
- worker termination;
- wait/reap, EOF, channel, join, output overflow, and retirement;
- quarantine handoff; and
- supervisor unwind.

A JavaScript throw or rejected promise produces `Rejected` authored cargo, not
a mechanical fault. JavaScript `undefined`, non-serializable output, missing
export, and non-callable export are typed faults. They never become JSON null,
empty cargo, or a placeholder rejection string.

## Cancellation And Deadline

Deadline construction is fallible. `Instant::checked_add` failure is a typed
construction fault and never converts to unbounded execution.

Cancellation records its monotonic observation. Cancellation and deadline
races have a documented deterministic tie rule. A cooperative cancellation is
settled only by `InvocationReadyProof`; forced cancellation and deadline are
settled only by `RetirementProof`. A wait, containment, EOF, channel, join, or
quarantine failure returns `Fault`, with the original trigger recorded.

## Retained Reuse And Shutdown

A retained worker returns to Ready only through `InvocationReadyProof`, which
proves:

- the admitted invocation is settled;
- no pending promise/module execution authority remains;
- the invocation output barrier is complete;
- no late output entered idle capture;
- the protocol sequence and session epoch match; and
- no cancellation or teardown work remains.

Late output, correspondence ambiguity, or incomplete cancellation poisons the
session and forces retirement or quarantine. It is not reused.

Shutdown consumes `BunProviderBackend`. From `Ready`, `Poisoned`, or an active
invocation continuation, success requires `RetirementProof`. From
`Restartable`, the prior `RetirementProof` already seals the absence of a live
worker, so shutdown consumes that continuation without starting another one.
Failure returns a typed terminal whose `RetirementQuarantine` owns the exact
remaining custody. There is no post-shutdown backend value and shutdown never
returns `InvocationReadyProof`.

## Drop

Drop never waits, joins, blocks, calls user code, fabricates a proof or terminal,
ignores a shutdown error, or aborts the host. Drop takes any still-owned custody
once and submits it to the durable quarantine queue. Queue/reaper failure
retains the item for retry. The reaper may delete active-worker custody only
after obtaining `RetirementProof`; Drop itself produces no public continuation.

## Negative Construction Proof

External compile-fail tests must prove that sibling crates cannot:

- construct or clone package, invocation, backend, prepared export, proof, or
  terminal values;
- obtain raw package bytes, export names, worker paths, ids, session epochs,
  pipe/process handles, or wire frames;
- call an in-process Bun drive operation;
- deserialize/replay selected work;
- call a borrowed authority mint; or
- select a callback or return a caller-chosen receipt.
