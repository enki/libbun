# libbun Current Decision And Handoff Index

Status: frozen implementation contract

Date: 2026-07-23

Rejected implementation: `f1c450b042e4aa2c0c7abe05f9e95c86b8c1e697`

This index is the current authority for the libbun retained-backend and
worker-only prepared-export repair. The implementation at `f1c450` is rejected
and must not be released or treated as a compatibility baseline.

The frozen decision reconciles:

- Oracle Pro/xhigh response
  `resp_0b95d5d9288126a6006a62546d89cc8198a27589b81945f3c9`;
- Fable session `de790485-7219-45f8-8a3c-0cd3ebac5622`; and
- the independent full-SCC review of `2022bafe..f1c450`.

## Authoritative Documents

Read these as one contract:

1. [Retained backend and prepared-export lifecycle](LIBBUN-LIFECYCLE-CONTRACT.md)
   fixes the highest owner, sealed inputs, closed terminal algebra, refusal,
   retry, backend reuse, authored settlement, consuming shutdown, and Drop.
2. [Worker containment, retirement, and quarantine](LIBBUN-WORKER-CONTAINMENT-CONTRACT.md)
   fixes exact process custody, bounded quiescence, output pumps, platform
   containment, retirement proof, and the durable reaper.
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

An admitted invocation becomes an affine `PreparedExport`. After worker spawn,
private `DriveCustody` owns the exact containment, child, request and terminal
pipes, output pumps, completion channels, join handles, provisional candidate,
deadline, cancellation observation, and retained-backend continuation.

`DriveCustody` can be consumed only into:

- `RetirementProof`, after exact bounded quiescence; or
- `RetirementQuarantine`, which transfers the intact custody to the durable
  reaper and produces a typed mechanical fault.

Cargo, cancellation, and deadline evidence are minted only from
`RetirementProof`. A retirement, containment, wait, EOF, channel, or join fault
dominates every provisional cargo/cancel/deadline choice.

Terminal products keep the backend continuation sealed and expose finite
consuming operations such as retry, reuse for the next invocation, or shutdown.
They do not expose `into_parts`, raw backend/session handles, process ids,
worker paths, selected export names, wire frames, or callbacks.

## Rejection Ledger For `f1c450`

The rejected tree:

- deletes the required retained `BunProviderBackend` lifecycle;
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
- deletes the release factory without replacing it with a tested worker-only
  release path.

None of these shapes may be preserved behind a compatibility alias or fixed
error.

## Edit Gate

- Bucket: `POISON`, followed by the positive owner move.
- Exact first source edit: delete `install_prepared_export` from
  `src/prepared_export.rs` and its re-export from `src/lib.rs`.
- Owner boundary: `BunProviderBackend`, its by-value admission operation, and
  private `DriveCustody`/retirement owner.
- First stale caller: every caller passing artifact bytes, export text, and
  invocation bytes separately.
- Replacement input: producer-minted branded `SelectedProviderPackage` plus
  branded `ProviderInvocation`.
- Tripwire terms: `install_prepared_export`, `DriveRequest`,
  `internal-adapter`, `drive_prepared_export`, `libbun-native`,
  `libbun-prepared-export-wire`, `Child::wait`, `process::abort`,
  `JavaScriptRejection` as a mechanical fault, plugin, dynamic loading,
  fallback, `into_parts`, public raw selectors, callback proof, and unsafe
  `Send`/`Sync`.

The first edit is a poison cut. It does not add a fixed error, empty terminal,
default cargo, fallback, or compatibility route. Compiler fallout identifies
the producer callers that must move to the branded admission operation.

## Fifteen-Step Hard-Cut Order

This order is locked. A later step may not be used to patch around a missing
earlier owner product.

1. Poison `install_prepared_export` and its root re-export. Add compile-fail
   tripwires proving the raw constructor is absent.
2. Create the opaque, by-value `BunProviderBackend` owner and the private state
   needed for Ready, Admitted, Poisoned, Quarantined, and ShuttingDown custody.
3. Move producer admission to generatively branded `SelectedProviderPackage`
   and `ProviderInvocation`; delete every raw constructor, selector, serde,
   clone, parts, and path/export projection.
4. Define the closed retained admission/refusal/retry and terminal continuation
   products. Refusal and retry consume the same backend, session epoch, package,
   and invocation.
5. Rewrite `PreparedExport` around private `DriveCustody`, provisional
   selection, `RetirementProof`, and intact `RetirementQuarantine`.
6. Add the durable process-wide reaper. Drop paths enqueue intact custody only;
   reaper creation, wake, panic, and retry failures retain the queue item.
7. Move the wire codec into private modules compiled only by the facade owner
   and binary. Delete public protocol constants, frames, and request parts.
8. Move the native engine into the binary-only runtime crate. Delete the
   `native` and `wire` library crates, public internal feature, and public
   in-process drive entry point.
9. Implement exact platform containment before selected work is sent: Linux
   namespace containment, macOS spawn-denying sandboxing, and atomic Windows
   job assignment. Unsupported platforms fail admission; there is no process
   group fallback.
10. Replace blocking output with pumps that start before Bun receives its write
    descriptors, retain bounded bytes, discard after overflow while continuing
    to drain, and prove barriers/EOF.
11. Implement retained worker offer/reserve/refuse/drive/cancel/ready/shutdown
    protocol and consuming backend shutdown. Restore no generic runtime trait,
    callback, module handle, promise handle, or public event-loop control.
12. Migrate the source producer and downstream semantic consumer to the branded
    inputs and finite terminal continuation operations. Compiler fallout is the
    caller map; no adapter source or raw export selection survives.
13. Require an actually linked binary, one workspace lock, current manifests,
    and a packageable facade with no unpublished library dependency.
14. Build the worker-only package/release factory and run the full hostile,
    privacy, real-worker, extracted-package, compliance, and target gates.
15. Run completion searches, link validation, repeat lock/package generation,
    current-tree evidence capture, and independent full-SCC review.

Tests are added with the owner edit they guard. They do not precede or replace
the first owner-boundary source edit.

## Handoff

The implementation lane starts at step 1 only. It must reference these frozen
documents in its edit gate and repair contract. Discovery is complete unless a
proposed edit changes the highest owner, sealed input, public terminal algebra,
fault settlement, containment primitive, or release boundary. Such a change
requires a new independent contract review before source edits continue.

The rejected commit remains useful only as a negative test fixture and compiler
fallout map. It is not a base to preserve.
