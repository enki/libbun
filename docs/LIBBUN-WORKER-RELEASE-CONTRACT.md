# libbun Worker Build, Package, And Release Contract

Status: frozen implementation contract

Date: 2026-07-23

## Release Product

The only executable Bun authority is `libbun-runtime-native`, a linked,
worker-process-only binary. There is no native library, plugin ABI, dynamic
loader, helper fallback, in-process feature, or callable Rust drive entry point.

The root facade contains supervisor/model code only. The runtime package has a
binary target only. Native engine code is a private module of that binary.
Protocol code is private source compiled into the facade and binary; it is not
an independently depend-able crate and exposes no public frames or raw parts.

Required deletion set:

- `native/` as a library crate;
- `wire/` as a library crate;
- `internal-adapter` or any equivalent public feature;
- public `DriveRequest`, protocol constants, and codec functions;
- public `drive_prepared_export`;
- plugin/dynamic-loading packages, features, checksums, paths, installers,
  caches, aliases, and workflows; and
- public raw artifact/export/invocation constructors.

Rust has no friend crates. `publish = false`, a feature named internal, an
owner suffix, or a workspace-only path does not enforce this boundary.

## Build Gate

The worker build script fails immediately unless it has an admitted, nonempty,
release-profile Bun link manifest and every referenced archive/static input
exists. Warning-only unlinked mode is forbidden.

The acceptance build is an actual linked binary:

```sh
LIBBUN_NATIVE_LINK_BUN=1 \
cargo +nightly-2026-05-06 build \
  --release \
  --manifest-path runtime/Cargo.toml
```

`cargo check` is useful but cannot satisfy the worker gate. The linked binary
must execute real fulfilled, rejected, malformed, cancellation, deadline,
output, shutdown, containment, and retained-reuse cases.

The repository uses one current lockfile unless a reviewed release reason
requires otherwise. A locked build must not rewrite manifests or locks. The
facade must package without an unpublished path library dependency.

## Worker Package

Each target package contains:

- the exact release-profile `libbun-runtime-native` executable;
- a versioned manifest with target, wire/protocol version, build identity,
  Bun source revision, checksums, supported exact-containment mode, retained
  mode, one-shot mode, and output limits;
- required runtime native libraries, if any;
- source/build instructions sufficient to reproduce the binary;
- notices, license texts, and dependency inventory; and
- package checksums.

The manifest contains no fallback key, plugin ABI, shared-library runtime mode,
raw worker path override, or compatibility alias. Unsupported containment
targets are not published.

The package test extracts into a new directory, places the worker at the
documented sibling location, and runs the complete smoke protocol against the
extracted binary. Testing the build-tree executable is not package proof.

## CI Gate

Default-parallel CI must run:

1. root all-feature tests;
2. compile-fail privacy tests from an external fixture crate;
3. vendored Bun reproducibility and code generation;
4. locked release Bun link preparation;
5. actual linked worker build;
6. nonzero linked runtime/native engine tests;
7. retained-backend `OfferReadyProof` refusal/retry,
   `ReservationReleaseProof` pre-dispatch release, and
   `InvocationReadyProof` fulfilled/rejected/cooperative-cancel same-worker
   same-epoch reuse tests;
8. forced-cancel/deadline/unwind/shutdown `RetirementProof`, replacement-epoch,
   fault-dominance, adoption-before-terminal, single-claim poll/completion race,
   claim-abandonment, recovered-terminal `RetiredDisposal` Drop,
   shutdown-only-claim Drop, shutdown-conversion, silent Drop, queue-disposal,
   and reaper spawn/wake/panic/retry tests;
9. target-specific containment hostile tests;
10. output saturation, overflow, barrier, EOF, channel, wait, and join tests;
11. worker package creation;
12. extracted-package execution;
13. source/notice/license/compliance verification;
14. facade `cargo package` and repeat lock generation with no diff;
15. symbol, dependency, and stale-shape searches; and
16. format and diff checks.

A linked test target reporting zero tests fails the gate. CI cannot skip real
execution because a local plugin-era gate once passed.

## Release Workflow

The worker-only release workflow builds from an immutable tag, repeats every CI
gate, packages each supported target, verifies extracted packages, publishes
binary and compliance assets, and verifies the release inventory after upload.

No old plugin workflow is restored. No platform enters the release matrix until
its exact containment and hostile tests pass on the release runner.

## Privacy And Stale Searches

External compile-fail fixtures must prove a sibling crate cannot import or call
protocol/native engine code or construct selected work from raw material.

Completion searches reject at least:

```text
install_prepared_export
DriveRequest
internal-adapter
drive_prepared_export
libbun-native
libbun-prepared-export-wire
BunEmbeddingRuntime
LowLevelBunHost
dynamic-loading
libbun_plugin
plugin installer/cache/path/checksum
fallback
into_parts / from_parts on authority products
public artifact/export/path/id selectors
callback diagnostics or output handlers
unsafe impl Send / unsafe impl Sync
Child::wait
process::abort
mechanical JavaScriptRejection
cancel_before_spawn
fulfilled/rejected cargo minted from RetirementProof
pre-dispatch cancellation minted from InvocationReadyProof
ReservationReleaseProof minted after dispatch
public RetirementQuarantine
public QuarantineReceipt or raw completion receipt
QuarantineId
QuarantineIdView
quarantine_reference
quarantine id/path/number/UUID/PID/epoch getter
quarantine registry/lookup/selector operation
BackendState::Quarantined
observation accepted as poll/claim/restart/shutdown authority
public completion claim or completion-claim parts
quarantine terminal Clone/serde/into_parts
shutdown-origin quarantine exposing restart/backend recovery
```

Binary symbol scans must show no Rust drive entry point, plugin ABI, shared
library export surface, or raw protocol API.

## Release PASS

Release is permitted only when:

- the retained backend and worker lifecycle contracts pass current hostile
  evidence;
- every pre-dispatch release terminal is post-`ReservationReleaseProof`;
- every fulfilled/rejected cargo or same-worker cooperative-cancel terminal is
  post-`InvocationReadyProof`;
- every forced-cancel, deadline, unwind, active-worker shutdown, or retired
  fault terminal is post-`RetirementProof`, or is a concrete typed quarantine
  fault constructed only after `DurableReaper::adopt` consumed the private
  `RetirementQuarantine<Purpose>`;
- every public quarantine view is limited to bounded
  `QuarantineObservation`;
- every recoverable quarantine terminal privately contains at most one opaque
  affine purpose-typed `QuarantineCompletionClaim<Purpose>`;
- no public quarantine id, id view, number, UUID, path, PID, epoch, lookup key,
  registry, raw receipt, selector, parts projection, clone, serde, callback, or
  caller-supplied proof exists;
- no `BackendState::Quarantined` husk exists;
- pending poll moves the same sole claim, completed poll yields at most one
  `RestartableCustody`, and every abandonment or shutdown conversion disposes
  any eventual or completed continuation without spawning;
- shutdown-origin quarantine can never recover a backend;
- Drop adoption produces no terminal, observation, or completion claim;
- Drop of an already-recovered terminal consumes or silently adopts its sole
  terminal-owned `RestartableCustody` as `RetiredDisposal`, never spawns, and
  produces no observation or completion claim;
- Drop of a shutdown-only claim abandons only completion observation while
  queue-owned retirement or disposal continues;
- reaper spawn, wake, panic, retry, and terminal/claim Drop cannot lose or
  return queue custody;
- active-worker custody is deleted or transformed only after exact
  `RetirementProof`;
- no `RetirementProof` path claims a live Ready worker, no
  `InvocationReadyProof` path claims worker death, and no
  `ReservationReleaseProof` path claims dispatched work;
- the runtime binary is actually linked and executed;
- the extracted package is executed;
- locks and package metadata are stable;
- compliance assets match the binary inputs;
- stale and privacy searches are clean; and
- an independent full-SCC reviewer approves the current tree.

Evidence from `d5d007e` is rejected because its source still exposes the raw
installer, uses fresh-worker semantics, checks an unlinked worker, lacks the
retained hostile gates above, and is not release-ready.
