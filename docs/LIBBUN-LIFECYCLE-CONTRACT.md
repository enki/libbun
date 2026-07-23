# libbun Retained Backend And Prepared-Export Lifecycle Contract

Status: frozen implementation contract

Date: 2026-07-23

## Scope

This document owns the semantic lifecycle of one retained Bun provider backend
and every one-shot prepared export admitted through it. Worker containment and
release proof are specified separately but are part of the same completion
gate.

## Semantic Abstraction Gate

- Unit: one retained Bun provider backend, its exact contained worker session,
  and a sequence of one-shot branded provider invocations.
- Highest owner: opaque `BunProviderBackend` by value.
- Selected inputs: producer-minted, generatively co-branded
  `SelectedProviderPackage` and `ProviderInvocation`.
- Pre-reservation owner: private `OfferCustody`.
- Reserved-but-undispatched owner: private `ReservedCustody`; no request,
  terminal, output-generation, cancellation, or per-invocation join authority
  exists yet.
- Dispatched owner: private `DriveCustody`.
- Retirement owner: private `RetirementCustody`.
- Quarantine owner before adoption: private `RetirementQuarantine<Purpose>`.
- Quarantine owner after adoption: only the durable queue/reaper.
- Public quarantine view: bounded non-authoritative
  `QuarantineObservation`; a concrete recoverable terminal may privately seal
  one opaque affine `QuarantineCompletionClaim<Purpose>`.
- Too low: wire frames, raw bytes or strings, worker paths, process ids,
  session epochs, containment handles, pipe parts, quarantine ids, numbers,
  UUIDs, lookup keys, raw receipts, callbacks, or caller-supplied proofs.
- Too high: a generic embedding runtime, workflow/session framework, public
  module or promise handles, or public event-loop operations.

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

A public `BunProviderBackend` owns either one live Ready session or one
no-live-worker Restartable continuation. Active offer, reservation, drive,
retirement, shutdown, and quarantine are private custody or terminal states,
not publicly projectable backend variants.

```rust
pub struct BunProviderBackend {
    state: private::BackendState,
}

enum BackendState {
    Ready(ReadyCustody),
    Restartable(RestartableCustody),
}
```

There is no `BackendState::Quarantined` observation-only husk. A quarantine
result is a concrete typed fault terminal, not a backend.

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

The retained worker first receives a bounded private admission envelope. The
offer contains no selected package, export, invocation, or dispatch authority.

- `Refused`: private `OfferReadyProof` proves that no reservation or selected
  authority moved and that the same worker and epoch remain Ready. The refusal
  owns the unchanged branded selection and exposes only retry or shutdown.
- `Reserved`: private `ReservedCustody` owns the exact reservation, selected
  inputs, unconsumed dispatch permit, retained-worker continuation, and
  preallocated queue node. Reservation transmits no selected package or
  invocation.
- transport ambiguity or malformed correspondence: a typed admission fault. It
  is never a refusal and may preserve a live worker only through the exact proof
  required by its actual private state.

No terminal exposes separate backend, session, reservation, or selected-input
parts.

## Prepared Export

`PreparedExport` is affine and non-serializable. It seals exactly one
`ReservedCustody`.

```rust
pub struct PreparedExport<Brand> {
    custody: private::ReservedCustody<Brand>,
}

impl<Brand> PreparedExport<Brand> {
    pub fn drive(self, control: DriveControl) -> DriveTerminal<Brand>;

    pub fn cancel_before_dispatch(
        self,
        control: CancelControl,
    ) -> PreDispatchCancelTerminal<Brand>;

    pub fn shutdown(self, control: ShutdownControl) -> ShutdownTerminal;
}
```

`cancel_before_dispatch` can return the same worker only after
`ReservationReleaseProof`. Consuming `drive` consumes the dispatch permit and
selected inputs into private `DriveCustody`; after that point
`ReservationReleaseProof` is impossible.

## Closed Readiness, Release, Retirement, And Quarantine Products

All proofs and authority products are sealed, affine, and private. They have no
public constructors, fields, parts projections, clone, serde, selector getter,
or borrowed authority mint.

- `OfferReadyProof<Brand>` proves no reservation was created, no selected work
  moved, and the same worker and epoch remain Ready. It alone permits unchanged
  refusal retry.
- `ReservationReleaseProof<Brand>` proves that one exact reservation existed
  and is now closed and unreplayable before dispatch; no selected package or
  invocation was enqueued or transmitted; no invocation request, terminal,
  output generation, cancellation, or per-invocation task exists; and the same
  worker and epoch remain Ready. It alone permits reuse after pre-dispatch
  release.
- `InvocationReadyProof<Brand>` proves that one dispatched invocation settled
  and drained, its reservation and output ledger are closed, no pending
  invocation authority or late output remains, and the same worker and epoch
  remain Ready.
- `RetirementProof` proves exact worker death, containment emptiness, leader
  reap, protocol and diagnostic EOF, channel closure, final pump barriers and
  joins, and absence of all child, containment, pipe, receiver, channel, pump,
  and join custody.
- `RetirementQuarantine<Purpose>` privately owns all unresolved retirement
  custody until `DurableReaper::adopt` consumes it by value exactly once.
  Adoption publishes its preallocated node before any public fault terminal
  exists.
- `QuarantineObservation` is bounded public data only. It has private fields,
  no clone, serde, or parts projection, and carries no id, epoch, process id,
  path, number, UUID, key, receipt, handle, or completion authority.
- `QuarantineCompletionClaim<Purpose>` is private, generative, affine,
  purpose-typed, non-cloneable, and usable only through finite consuming
  operations of its concrete terminal.

| Transition | Required sealed evidence or movement | Continuation |
| --- | --- | --- |
| Retry refused offer | `OfferReadyProof` | Same Ready worker and epoch; unchanged branded inputs |
| Release reservation before dispatch | `ReservationReleaseProof` | Same Ready worker and epoch |
| Fulfilled or rejected cargo | `InvocationReadyProof` | Same Ready worker and epoch |
| Cooperative in-drive cancellation | `InvocationReadyProof` | Same Ready worker and epoch |
| Forced cancellation or clean deadline | `RetirementProof` | One Restartable continuation; no live worker |
| Supervisor unwind or retired mechanical fault | `RetirementProof` | One Restartable continuation; no live worker |
| Recoverable retirement unresolved | `RetirementQuarantine<RecoverBackend>` consumed by `DurableReaper::adopt` | Concrete quarantine fault with observation and one private recovery claim |
| Active-worker shutdown complete | `RetirementProof` | No backend |
| Active-worker shutdown unresolved | `RetirementQuarantine<CompleteShutdown>` consumed by `DurableReaper::adopt` | Shutdown quarantine fault with one private shutdown-only claim |
| Shutdown of Restartable | Prior `RetirementProof` sealed in continuation | No backend and no spawn |
| Drop of live custody | Silent adoption of `RetirementQuarantine<DisposeOnly>` | No public terminal, observation, claim, or continuation |
| Drop of an already-recovered terminal | Its sole terminal-owned `RestartableCustody` is silently consumed or adopted as `RetiredDisposal` | No spawn, observation, completion claim, or public continuation |
| Drop of a shutdown-only claim | Only completion observation is abandoned; queue-owned retirement or disposal continues | No public continuation and no custody transfer back to the caller |

Failure to prove Ready forces retirement. Failure to prove retirement forces
adoption. An adopted quarantine fault dominates every provisional cargo,
cancellation, deadline, unwind continuation, or shutdown success.

## Closed Terminal Algebra

Each non-quarantine terminal privately owns the exact continuation proved for
its transition. A quarantine terminal owns no unresolved OS custody and is not
a backend husk. It owns bounded observation and, where completion recovery is
meaningful, exactly one private completion claim.

A concrete recoverable quarantine terminal exposes only:

- `observation(&self) -> &QuarantineObservation`;
- nonblocking `poll(self)`, returning either the same terminal with the same
  moved claim or a completed terminal sealing exactly one
  `RestartableCustody`; and
- consuming `shutdown(self, control)`.

A shutdown-origin quarantine terminal exposes only observation and consuming
poll. It has no restart or backend-recovery operation.

A completed recoverable terminal exposes only observation, consuming restart,
and consuming shutdown. It has no `into_parts`, raw Restartable getter, backend
getter, session getter, generic callback, or selector operation. If it is
dropped instead, its sole terminal-owned `RestartableCustody` is silently
consumed or adopted as `RetiredDisposal`; Drop never spawns and creates no
observation or completion claim.

No operation takes `QuarantineObservation`, an id, path, number, UUID, process
id, epoch, lookup key, raw receipt, or caller-supplied proof as authority.

## Provisional Selection And Final Proof

Cargo, rejection, cancellation, deadline, worker fault, protocol fault,
shutdown acknowledgement, and supervisor unwind are private provisional facts.

Finalization rules:

1. Fulfilled or rejected cargo requires one complete authored terminal and
   `InvocationReadyProof`.
2. Pre-dispatch release requires an exact release acknowledgement and
   `ReservationReleaseProof`. It never produces `InvocationReadyProof`.
3. Cooperative in-drive cancellation requires an exact acknowledgement of the
   dispatched invocation and `InvocationReadyProof`.
4. Forced cancellation and a clean deadline require `RetirementProof`.
5. A mechanical fault may preserve the same worker only through
   `InvocationReadyProof`; otherwise it forces retirement.
6. Supervisor unwind always forces retirement and can never produce a
   Ready-family proof.
7. A retirement fault followed by eventual foreground `RetirementProof`
   remains a typed retired fault; it does not retroactively become cargo,
   cancellation, deadline, or clean shutdown.
8. Foreground failure to obtain `RetirementProof` transfers all unresolved
   custody into `RetirementQuarantine<Purpose>`, which `DurableReaper::adopt`
   consumes before the concrete public quarantine fault is constructed.
9. Reaper completion cannot retroactively fabricate the displaced provisional
   terminal. Recoverable completion yields only one Restartable continuation;
   shutdown completion yields only fault-complete shutdown.

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
construction fault and never becomes unbounded execution.

Pre-dispatch release is settled only by `ReservationReleaseProof`.
Cooperative in-drive cancellation is settled only by `InvocationReadyProof`.
Failed or ambiguous cooperative cancellation, forced cancellation, and every
deadline force retirement. When cancellation and deadline are observed at the
same private poll point, deadline wins. Any finalization or retirement fault
still dominates the selected trigger.

## Retained Reuse And Shutdown

Same-worker reuse after a reservation that never dispatched requires
`ReservationReleaseProof`. Same-worker reuse after a dispatched invocation
requires `InvocationReadyProof`. Late output, correspondence ambiguity,
incomplete cancellation, or incomplete release forces retirement or adoption.

Shutdown consumes its owner:

- from a live-worker state, clean completion requires `RetirementProof`;
- from Restartable, shutdown consumes the prior no-worker continuation without
  spawning;
- from a pending recoverable quarantine fault, shutdown atomically converts the
  sole recovery claim into a shutdown-only claim;
- from a completed recoverable quarantine fault, shutdown atomically consumes
  the queue-held `RestartableCustody` without spawning;
- from shutdown-origin quarantine, polling can yield only pending or
  `CompleteWithFault`.

No shutdown path returns `ReservationReleaseProof`, `InvocationReadyProof`, a
backend husk, or a recovered backend. A shutdown-origin quarantine can never
recover a backend.

## Drop

Drop never allocates, waits, joins, blocks, calls user code, constructs a public
terminal or observation, fabricates a proof, aborts, or panics.

Drop of an owner that still contains live or unresolved custody moves all of
that custody into its preallocated node and silently calls
`DurableReaper::adopt` with disposal purpose. Publication precedes best-effort
wake or spawn and creates no completion claim.

Drop of a quarantine terminal does not resubmit custody because the queue
already owns it. It abandons only the terminal's private claim. If recovery is
abandoned before completion, the reaper disposes the continuation after
`RetirementProof`. If recovery is abandoned after completion, the queue
consumes the stored `RestartableCustody` without spawning.

Drop of an already-recovered terminal silently consumes or adopts its sole
terminal-owned `RestartableCustody` as `RetiredDisposal`. It never spawns and
creates no observation or completion claim. Drop of a shutdown-only claim
abandons only completion observation; queue-owned retirement and disposal
continue unchanged until the entry closes.

The reaper may delete or transform active-worker custody only after exact
`RetirementProof`.

## Negative Construction Proof

External compile-fail tests must prove that sibling crates cannot:

- construct or clone package, invocation, backend, prepared export, proof, or
  terminal values;
- obtain raw package bytes, export names, worker paths, ids, session epochs,
  pipe/process handles, wire frames, or a quarantine identity;
- name, construct, clone, serialize, or project
  `QuarantineCompletionClaim<Purpose>`, `RetirementQuarantine<Purpose>`, a
  queue entry, or a purpose marker;
- use `QuarantineObservation` to select, poll, claim, restart, shut down, or
  otherwise feed authority;
- call an in-process Bun drive operation;
- deserialize/replay selected work;
- call a borrowed authority mint;
- select a callback or return a caller-chosen receipt; or
- poll, shut down, or restart a terminal after its consuming operation.
