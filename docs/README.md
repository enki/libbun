# libbun Current Decision And Handoff Index

Status: frozen implementation contract

Date: 2026-07-23

Rejected implementation:
`d5d007e09b89eb3d7d23ba8380eb723b7bd6948d`

This index is the current authority for the libbun retained-backend and
worker-only prepared-export repair. The implementation at `d5d007e` is rejected
and must not be released or treated as a compatibility baseline. The earlier
`f1c450` tree remains historical negative evidence only.

The frozen decision reconciles:

- Oracle Pro/xhigh responses
  `resp_0b95d5d9288126a6006a62546d89cc8198a27589b81945f3c9`,
  `resp_0c819c36326a1bdb006a62697111bc819b9bb9f7c6fd76f1a7`, and
  `resp_0da9cbece71205d8006a62697111bc819b9bb9f7c6fd76f1a7`;
- independent Fable-max sessions
  `ddf50aed-f3b9-4125-9ea9-b08179d856db` and
  `13e7e19a-d071-4b7d-a606-48900e7c37e6`; and
- the narrow Oracle Pro arbitration fixing the retained-worker quarantine
  terminal, completion-claim, race, abandonment, shutdown, and recovery shape.

## Authoritative Documents

Read these as one contract:

1. [Retained backend and prepared-export lifecycle](LIBBUN-LIFECYCLE-CONTRACT.md)
   fixes the highest owner, sealed inputs, closed terminal algebra, refusal,
   retry, backend reuse, authored settlement, consuming shutdown, and Drop.
2. [Worker invocation readiness, retirement, and quarantine](LIBBUN-WORKER-CONTAINMENT-CONTRACT.md)
   fixes exact process custody, `ReservationReleaseProof`,
   `InvocationReadyProof`, `RetirementProof`, queue-exclusive quarantine,
   completion claims, output pumps, platform containment, and the durable
   reaper.
3. [Worker build, package, and release](LIBBUN-WORKER-RELEASE-CONTRACT.md)
   fixes the Rust privacy boundary, linked binary, package inventory, CI,
   release, hostile tests, and stale-shape gates.

When wording elsewhere conflicts, these documents win. Historical plugin,
dynamic-loading, callback, path-fed, raw-runtime, and fallback decisions remain
deleted and provide no compatibility obligation.

## Frozen Decision

The semantic unit is one retained Bun provider backend and its admitted
invocations. `BunProviderBackend` is the highest owner. It consumes
producer-minted, generatively branded `SelectedProviderPackage` and
`ProviderInvocation` products. Neither product has a public raw constructor,
selector getter, parts projection, clone, serde implementation, or replay path.

Backend startup creates the retained worker before it receives a bounded offer.
Private `OfferCustody` owns the pre-reservation exchange. An accepted offer
becomes private `ReservedCustody`, which owns the exact reservation, selected
inputs, unconsumed dispatch permit, retained-worker continuation, and
preallocated queue node. No invocation request, terminal task, output
generation, or per-invocation join authority exists before dispatch.

`PreparedExport` seals `ReservedCustody`. `cancel_before_dispatch` may return
the same worker only by consuming `ReservationReleaseProof`, which proves that
the exact reservation is closed and unreplayable, no selected package or
invocation was transmitted, and the same worker remains Ready at the same
epoch. Consuming `drive` consumes the dispatch permit and creates private
`DriveCustody`; after that point `ReservationReleaseProof` is impossible.

`DriveCustody` may produce `InvocationReadyProof` only after one dispatched
invocation is settled, drained, barrier-complete, and free of pending
invocation authority while the same worker remains Ready. Retirement may
produce `RetirementProof` only after exact worker death and complete discharge
of containment, reap, EOF, channel, pipe, receiver, pump, and join custody.

If foreground retirement cannot produce `RetirementProof`, private
`RetirementQuarantine<Purpose>` owns all unresolved custody until
`DurableReaper::adopt` consumes it by value exactly once. Adoption publishes
the preallocated node before any public fault terminal exists. After adoption
only the durable queue/reaper owns or polls unresolved OS custody.

A concrete public quarantine fault exposes only bounded
`QuarantineObservation`. A recoverable fault may privately contain one opaque,
affine, purpose-typed `QuarantineCompletionClaim<Purpose>`. Pending polling
moves that same claim; completed polling may claim exactly one
`RestartableCustody` after `RetirementProof`. A shutdown-origin claim can never
recover a backend. Dropping or converting a recovery claim disposes any
eventual or completed restartable continuation without spawning. Dropping an
already-recovered terminal silently consumes or adopts its sole terminal-owned
`RestartableCustody` as `RetiredDisposal`, with no spawn, observation, or
completion claim. Dropping a shutdown-only claim abandons only completion
observation while queue-owned retirement or disposal continues.

There is no `BackendState::Quarantined` husk, public quarantine id, id view,
number, UUID, path, process id, epoch, selector, registry, lookup operation,
raw receipt, callback, clone, serde, parts projection, or caller-supplied
proof. Observation data cannot select, poll, claim, restart, or shut down
queue custody.

Terminal products expose only finite consuming owner operations appropriate to
their state. They never expose separate backend/session/request parts.

## Rejection Ledger For `d5d007e`

The rejected tree:

- lacks the required retained `BunProviderBackend` lifecycle;
- exposes `install_prepared_export(Vec<u8>, String, Vec<u8>)`;
- exposes a public `internal-adapter` feature, raw public `DriveRequest`, and
  public in-process `drive_prepared_export`;
- treats a process group as exact containment even though `setsid` can escape;
- selects deadline/cancellation and then uses unbounded `Child::wait` and
  unconditional joins;
- consumes child and join custody on cleanup faults;
- aborts the host from `Drop`;
- flushes into blocking native output pipes before reading them;
- treats JavaScript rejection as a mechanical fault and can fabricate JSON
  `null` for undefined or unserializable output;
- permits an unlinked, non-runnable worker check to stand in for a worker build;
- carries stale locks and an unpackagable private path dependency; and
- lacks a tested retained worker-only release path.

None of these shapes may be preserved behind a compatibility alias or fixed
error.

## Edit Gate

- Bucket: `POISON`, followed by the positive owner move.
- Exact first source edit: delete `install_prepared_export` from
  `src/prepared_export.rs` and its re-export from `src/lib.rs`; add external
  compile-fail tripwires proving the raw constructor is absent.
- Owner boundary: `BunProviderBackend`, its by-value admission operation, and
  private reservation/invocation-readiness/retirement custody owners.
- First stale caller: every caller passing artifact bytes, export text, and
  invocation bytes separately.
- Replacement input: producer-minted branded `SelectedProviderPackage` plus
  branded `ProviderInvocation`.
- Tripwire terms: `install_prepared_export`, `DriveRequest`,
  `internal-adapter`, `drive_prepared_export`, `libbun-native`,
  `libbun-prepared-export-wire`, `Child::wait`, `process::abort`,
  `JavaScriptRejection` as a mechanical fault, plugin, dynamic loading,
  fallback, `cancel_before_spawn`, cargo from `RetirementProof`,
  pre-dispatch `InvocationReadyProof`, public `RetirementQuarantine`,
  `QuarantineId`, `QuarantineIdView`, `quarantine_reference`,
  `BackendState::Quarantined`, public raw completion receipts, `into_parts`,
  public raw selectors, callback proof, and unsafe `Send`/`Sync`.

The first edit is a poison cut. It does not add a fixed error, empty terminal,
default cargo, fallback, or compatibility route. Compiler fallout identifies
the producer callers that must move to the branded admission operation.

## Fifteen-Step Hard-Cut Order

This order is locked. A later step may not be used to patch around a missing
earlier owner product.

1. Poison `install_prepared_export` and its root re-export. Add compile-fail
   tripwires proving the raw constructor is absent.
2. Establish the opaque by-value `BunProviderBackend` owner and the process-wide
   preallocated durable queue substrate before any live backend constructor is
   usable. No temporary blocking, aborting, leaking, or observation-only
   quarantine state is permitted.
3. Move producer admission to generatively branded `SelectedProviderPackage`
   and `ProviderInvocation`; delete every raw constructor, selector, serde,
   clone, parts, replay, and path/export projection.
4. Define private `OfferCustody`, `OfferReadyProof`, closed refusal, unchanged
   retry, admission faults, and consuming shutdown.
5. Define private `ReservedCustody`, its unconsumed dispatch permit, affine
   `PreparedExport`, and sealed `ReservationReleaseProof`.
6. Define the pre-dispatch release terminal. Successful release uses only
   `ReservationReleaseProof`; failed or ambiguous release forces retirement or
   queue adoption.
7. Define private `DriveCustody`, provisional selection,
   `InvocationReadyProof`, `RetirementProof`, and closed authored/mechanical
   terminals. Dispatch permanently disables `ReservationReleaseProof`.
8. Complete quarantine ownership: private queue-owned
   `RetirementQuarantine<Purpose>`, by-value `DurableReaper::adopt`,
   preallocated publication before public fault construction, bounded
   `QuarantineObservation`, private purpose-typed
   `QuarantineCompletionClaim<Purpose>`, pending polling, single recovery,
   abandonment, recovered-terminal `RetiredDisposal` Drop,
   shutdown-only-claim Drop, shutdown conversion, panic requeue, and silent
   Drop adoption.
9. Move the wire codec into private modules compiled only by the facade owner
   and binary. Delete public protocol constants, frames, and request parts.
10. Move the native engine into the binary-only runtime crate. Delete the
    `native` and `wire` library crates, public internal feature, and public
    in-process drive entry point.
11. Implement exact platform containment before selected work is sent: Linux
    namespace containment, macOS spawn-denying sandboxing, and atomic Windows
    job assignment. Unsupported platforms fail admission; there is no process
    group fallback.
12. Replace blocking output with persistent pumps that start before Bun
    receives its write descriptors, retain bounded bytes, discard after
    overflow while continuing to drain, prove per-invocation barriers without
    stopping, and prove EOF/join only during retirement.
13. Implement retained offer/reserve/release/dispatch/cancel/ready/shutdown,
    bounded finalization and retirement, unwind, restart, second invocation,
    consuming shutdown, fault dominance, nonblocking Drop, and durable
    reaping.
14. Migrate producers and consumers to branded inputs and finite terminal owner
    operations; build the linked worker package and run hostile, privacy,
    real-worker, extracted-package, compliance, containment, proof-boundary,
    completion-race, abandonment, and new-epoch restart gates.
15. Run completion searches, link validation, contradiction checks, repeat
    lock/package generation, current-tree evidence capture, and independent
    full-SCC review.

Steps 2 through 8 form one ownership tranche. No intermediate source state may
mint live custody without a working silent queue transfer, and no public
quarantine result may exist before queue adoption.

Tests are added with the owner edit they guard. They do not precede or replace
the first owner-boundary source edit.

## Handoff

The implementation lane starts at Step 1 only. It must reference these frozen
documents in its edit gate and repair contract.

`OfferReadyProof`, `ReservationReleaseProof`, `InvocationReadyProof`, and
`RetirementProof` are fixed, distinct evidence products.
`RetirementQuarantine<Purpose>`, private
`QuarantineCompletionClaim<Purpose>`, and public `QuarantineObservation` are
fixed, distinct ownership/observation products. A source editor may not merge,
substitute, publicly project, or reconstruct their evidence or authority.

After this documentation correction, no further quarantine-shape arbitration
is required for locked Step 1. Any proposed deviation in highest owner, proof
boundary, adoption order, completion-claim race, public terminal algebra,
fault dominance, containment primitive, or release boundary requires a new
independent contract review before source edits continue.

The rejected commit remains useful only as a negative test fixture and compiler
fallout map. It is not a base to preserve.
