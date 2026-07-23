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
  mechanical fault.
- Private phases: package correspondence, worker offer/reservation, request
  framing, execution, output capture, cancellation, retirement, quarantine,
  and session-ready proof.
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

`BunProviderBackend` owns one retained worker session or its poison/quarantine
continuation. Its fields and variants are private.

```rust
pub struct BunProviderBackend {
    state: private::BackendState,
}

enum BackendState {
    Ready(RetainedCustody),
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

The retained worker first receives a bounded offer containing only its private
framed request. It answers with the same session epoch:

- `Refused`: the worker remains exactly Ready; the refusal product owns the
  same backend, package, and invocation and exposes `retry(self, control)` and
  `shutdown(self, control)` finite operations.
- `Reserved`: this is the exact admission point. The resulting
  `PreparedExport` owns the backend continuation, session reservation, package,
  and invocation.
- transport ambiguity or malformed correspondence: typed admission fault. It
  is never a refusal. The terminal owns the backend continuation and exposes
  only lawful recovery or shutdown operations.

No terminal exposes separate backend/session/request parts. A caller cannot
mix a backend from one terminal with an invocation from another.

## Prepared Export

`PreparedExport` is affine and non-serializable. It can be driven once, retried
only through a valid refusal product, cancelled before spawn by consuming it,
or dropped into quarantine.

```rust
pub struct PreparedExport<Brand> {
    custody: Option<private::PreparedCustody<Brand>>,
}

impl<Brand> PreparedExport<Brand> {
    pub fn drive(self, control: DriveControl) -> MechanicalTerminal<Brand>;
    pub fn cancel_before_spawn(self) -> MechanicalTerminal<Brand>;
}
```

After worker spawn, private `DriveCustody` owns all lifecycle facts. A public
terminal can be minted only after it is consumed into a `RetirementProof` or
transferred intact into `RetirementQuarantine`.

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

- successful cargo: observe final authored cargo, then reuse the same Ready
  backend for the next branded admission or consume it into shutdown;
- cancellation/deadline after exact retirement: continue only with the backend
  state proved by retirement, or consume it into shutdown;
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

1. Cargo requires a complete nonempty authored frame, successful worker exit,
   exact empty containment, leader reap, request completion, protocol EOF,
   diagnostic/output completion, closed completion channels, and joined pumps.
2. Cancellation and deadline require the same retirement proof.
3. Any later retirement failure dominates provisional cargo, cancellation, or
   deadline and returns a typed fault terminal.
4. Foreground retirement timeout transfers intact custody to quarantine and
   returns a typed quarantine fault. It never mints cancellation or deadline.
5. A retirement failure that later recovers remains observable as a typed
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
races have a documented deterministic tie rule. Their provisional choice is
settled only after retirement proof. A wait, containment, EOF, channel, join,
or quarantine failure returns `Fault`, with the original trigger recorded.

## Retained Reuse And Shutdown

A retained worker returns to Ready only after it proves:

- the admitted invocation is settled;
- no pending promise/module execution authority remains;
- the invocation output barrier is complete;
- no late output entered idle capture;
- the protocol sequence and session epoch match; and
- no cancellation or teardown work remains.

Late output, correspondence ambiguity, or incomplete cancellation poisons the
session. It is not reused.

Shutdown consumes `BunProviderBackend`. Success proves cooperative shutdown or
forced exact retirement, empty containment, reap, EOF, channel closure, and
joined pumps. Failure returns a typed terminal whose quarantine continuation
owns the exact remaining custody. There is no post-shutdown backend value.

## Drop

Drop never waits, joins, blocks, calls user code, fabricates a terminal, ignores
a shutdown error, or aborts the host. Drop takes any still-owned custody once
and submits it to the durable quarantine queue. Queue/reaper failure retains
the item for retry.

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
