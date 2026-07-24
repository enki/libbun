# libbun W1-11/W1-12 Final Composition Review

Verdict: **REVISE**

Date: 2026-07-24

## Reviewed Candidate

- Candidate commit: `5b14f8d0599e40630f788cc3863d0b3d96116199`
- Candidate tree: `5ceb06fa11f19029e150140870c2043b635f1561`
- Remote candidate ref: `origin/codex/libbun-prepared-export-owner-20260723`
- Reviewed commits, oldest first: `af9d5e14`, `c17ebd45`, `5a3258ae`,
  `f1c450b0`, `23ad2419`, `d5d007e0`, `98bcb4a2`, `5b14f8d0`
- Review Lane owner: `libbun-w1112-final-composition-review-20260724`
- Review worktree:
  `/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-final-composition-review-20260724`
- Review `CARGO_TARGET_DIR`:
  `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-8`

The candidate commit and tree matched the remote candidate ref before review.
The worktree was clean and detached at the candidate. All acceptance commands
reported below were repeated through the claimed Lane wrapper so they used the
exact worktree and cargo slot above.

## Ruling

This candidate cannot compose as the W1-11/W1-12 implementation. It implements
the negative fresh-process adapter that the frozen lifecycle, containment, and
release contracts explicitly reject. Passing its local one-shot tests and
successfully producing a manually prepared linked executable do not satisfy the
retained-backend ownership or release product.

The determining contradictions are:

1. `src/prepared_export.rs` still publishes
   `install_prepared_export(Vec<u8>, String, Vec<u8>)`, and `src/lib.rs` still
   reexports it. This is the raw constructor that the frozen edit gate requires
   deleting first.
2. Every drive starts a new worker. The passing test
   `each_drive_uses_a_fresh_worker_process` proves the rejected behavior rather
   than retained same-worker/same-epoch reuse.
3. No `BunProviderBackend`, branded `SelectedProviderPackage`, branded
   `ProviderInvocation`, readiness proof, release proof, retirement proof,
   durable reaper, private quarantine custody, affine completion claim, or
   consuming shutdown exists in implementation source.
4. `wire` remains a public library crate with public `DriveRequest`, fault
   constants, and codec operations. `native` remains a public library crate
   with the public `internal-adapter` feature and public
   `drive_prepared_export` operation.
5. The linked executable is not stripped and contains global
   `libbun_native::drive_prepared_export` and
   `libbun_prepared_export_wire::decode_drive_material` symbols. This directly
   fails the binary-symbol gate.
6. Unix containment is process-group creation plus group `SIGKILL`, not Linux
   namespace containment. The frozen containment contract expressly rejects
   process-group fallback.
7. `DriveGuard::retire` performs blocking child wait and unconditional thread
   joins. `DriveGuard::drop` calls `std::process::abort()` when retirement
   fails. There is no nonblocking Drop adoption into a durable reaper.
8. JavaScript rejection is a `MechanicalFaultKind::JavaScriptRejection`.
   Undefined and null results are both minted as JSON null cargo, and output is
   flushed before opportunistic drain calls instead of using persistent bounded
   pumps and per-invocation barriers.
9. `runtime/build.rs` permits warning-only, unlinked builds. A manually linked
   build succeeds only after an explicit configure and native-link preparation
   sequence.
10. The worker package manifest contains the forbidden
    `"execution": "fresh-process-only"` and `"fallback": null` fields. The
    script can archive and launch the extracted binary, but it does not run the
    required complete extracted-package smoke protocol or ship the required
    compliance inventory.
11. The facade cannot be packaged because its private path dependency on
    `libbun-prepared-export-wire` has no version. The repository also retains
    separate root/native/runtime locks while `wire/Cargo.lock` is absent.
12. There is no worker-only immutable-tag release workflow. The only workflow
    is `.github/workflows/ci.yml`, and it does not implement the frozen release
    matrix and extracted-package gates.

## Required Replacement

### Highest owner and closed algebra

The highest semantic owner is the opaque, by-value `BunProviderBackend`.
Producer-minted branded `SelectedProviderPackage` plus branded
`ProviderInvocation` are the selected inputs. The backend owns the complete
offer, reservation, dispatch, completion, retirement, restart, and consuming
shutdown transition.

The closed authority algebra is:

- offer readiness is established by private `OfferCustody` and
  `OfferReadyProof`;
- pre-dispatch release is established only by sealed
  `ReservationReleaseProof`;
- fulfilled, rejected, and same-worker cooperative-cancel terminals are
  established only after `InvocationReadyProof`;
- forced cancellation, deadline, unwind, active-worker shutdown, and retired
  faults are established only after `RetirementProof`, or after by-value
  adoption of private `RetirementQuarantine<Purpose>` by `DurableReaper`;
- public quarantine output is bounded observation only; any recovery authority
  remains one private affine `QuarantineCompletionClaim<Purpose>`;
- Drop transfers custody silently to the preallocated durable queue and emits
  no terminal, observation, completion claim, or process spawn; and
- consuming shutdown cannot expose backend restart authority.

The native engine, protocol, containment, output pumps, and lifecycle custody
remain private implementation phases of that owner. No caller receives raw
artifact/export/invocation parts, worker paths, protocol frames, native drive
operations, completion receipts, or quarantine selectors.

### Exact first source edit

Delete `install_prepared_export` from `src/prepared_export.rs` and its reexport
from `src/lib.rs`. In the same poison cut, add external compile-fail tripwires
proving that a sibling crate cannot import or call that raw constructor. Do not
replace it with a fixed error, compatibility alias, fallback, empty terminal,
or default cargo. Compiler fallout identifies the producer callers that must
move to branded admission.

### Locked migration and deletion order

The implementation must follow the Fifteen-Step Hard-Cut Order frozen in
`docs/README.md`:

1. Poison the raw installer and add external compile-fail tripwires.
2. Establish opaque `BunProviderBackend` and the process-wide preallocated
   durable queue before a live backend can be constructed.
3. Move admission to branded `SelectedProviderPackage` and
   `ProviderInvocation`; delete raw constructors, selectors, serde, clone,
   parts, replay, and path/export projections.
4. Add private offer custody/readiness, closed refusal, unchanged retry,
   admission faults, and consuming shutdown.
5. Add private reserved custody, unconsumed dispatch permit, affine
   `PreparedExport`, and sealed release proof.
6. Add the proof-bound pre-dispatch release terminal; ambiguous release must
   retire or transfer to the durable queue.
7. Add private drive custody, provisional selection, invocation-readiness and
   retirement proofs, and the closed authored/mechanical terminal algebra.
8. Complete private quarantine, durable adoption, bounded observation, affine
   completion claims, polling, one recovery, abandonment, disposal Drop,
   shutdown conversion, panic requeue, and silent Drop adoption.
9. Move the wire codec to private modules compiled only by the facade owner and
   binary; delete the public protocol crate and raw frames.
10. Move the native engine into the binary-only runtime; delete `native` and
    `wire` as libraries, `internal-adapter`, and the Rust drive entry point.
11. Implement exact pre-dispatch platform containment with no process-group
    fallback and refuse unsupported platforms at admission.
12. Install persistent bounded output pumps before descriptors reach Bun;
    prove overflow drain, barriers, and retirement-only EOF/join.
13. Implement retained offer/reserve/release/dispatch/cancel/ready/shutdown,
    bounded finalization, unwind, restart, second invocation, fault dominance,
    nonblocking Drop, and durable reaping.
14. Migrate producers and consumers, build the linked worker package, and run
    all hostile, privacy, real-worker, package, compliance, and containment
    gates.
15. Run completion searches, link validation, contradiction checks, repeated
    lock/package generation, exact-tree evidence capture, and independent
    full-SCC review.

Steps 2 through 8 are one ownership tranche. No intermediate state may mint
live custody without silent durable queue transfer, and no public quarantine
result may precede successful queue adoption.

## Required Hostile Evidence

The replacement is not composition-eligible until default-parallel evidence
covers all of the following against the linked, retained worker:

- external compile-fail privacy for installer, protocol, native engine, raw
  selected-work construction, selectors, parts, serde, clone, and callbacks;
- offer refusal and unchanged retry, pre-dispatch release, fulfilled/rejected/
  cooperative-cancel same-worker same-epoch reuse, and a second invocation;
- forced cancel, deadline, unwind, active-worker shutdown, retirement proof,
  replacement epoch, fault dominance, and adoption-before-terminal;
- single-claim poll/completion races, pending poll identity, claim abandonment,
  single recovery, recovered-terminal disposal Drop, shutdown-only-claim Drop,
  shutdown conversion, and shutdown-origin non-recovery;
- reaper spawn, wake, panic, retry, queue ownership, terminal/claim Drop, and
  silent nonblocking Drop paths;
- Linux namespace containment, macOS spawn-denying sandboxing, and atomic
  Windows job assignment on their supported release runners;
- output saturation, bounded overflow with continued drain, barrier ordering,
  EOF, channel, wait, and retirement-only join behavior;
- real fulfilled, rejected, malformed, cancellation, deadline, output,
  shutdown, containment, and retained-reuse execution;
- vendored Bun reproducibility and code generation, locked release link
  preparation, a nonzero linked runtime/native test target, facade packaging,
  repeat lock generation with no diff, compliance inventory, worker package
  creation, and complete execution from a freshly extracted package; and
- source and binary stale-shape searches followed by another independent
  full-SCC review at the exact resulting tree.

## Executed Gates

All commands in this table ran at the candidate SHA unless the row is explicitly
an identity or search operation. Exit codes are process exit codes; the wire
Nextest row also reports Cargo's nested exit.

| Command | Exit | Result |
| --- | ---: | --- |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- bash -lc 'ulimit -n 65536; cargo nextest run --workspace'` | 0 | 14/14 tests passed across one facade binary. `each_drive_uses_a_fresh_worker_process` passed. No retained-backend lifecycle tests ran. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- bash -lc 'ulimit -n 65536; cargo nextest run --locked --manifest-path wire/Cargo.toml'` | 102 | Test discovery failed because `wire/Cargo.lock` is absent; nested Cargo metadata exited 101. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- scripts/verify-vendored-bun-reproducible.sh` | 1 | Patch replay failed because `src/jsc/bindings/bindings.cpp` already exists; one hunk was skipped. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- scripts/configure-vendored-bun.sh` | 0 | Pinned Bun SHA `9ecb985ad0f06fa12cbd8eede2404589992527d5` configured and code generation completed. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- scripts/prepare-native-bun-link.sh` | 0 | Release `bun-profile` inputs and `libbun_native_link_manifest.txt` were produced. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- bash -lc 'LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --release --manifest-path runtime/Cargo.toml'` | 0 | Linked release executable built in the exact Lane cargo slot. |
| `sha256sum "$CARGO_TARGET_DIR/release/libbun-runtime-native"` | 0 | `1fc8e7f6d679c521b2ec703f67cee5b287e8b388ede65b12fe8e12ca79d527c8`. ELF PIE, dynamically linked, not stripped. |
| `nm -C "$CARGO_TARGET_DIR/release/libbun-runtime-native"` filtered for forbidden native/wire symbols | 0 | Found global `libbun_native::drive_prepared_export`, global `libbun_prepared_export_wire::decode_drive_material`, `DriveRequest`, and many native implementation symbols. |
| `lane run --owner libbun-w1112-final-composition-review-20260724 -- cargo package --locked` | 101 | Facade packaging failed: `libbun-prepared-export-wire` path dependency has no version requirement. |
| `scripts/package-prepared-export-worker-release.sh 0.2.3 "$CARGO_TARGET_DIR/release/libbun-runtime-native" <temporary-output>` followed by extraction and launch with empty stdin | 0 | Archive created; extracted worker exited 0 and emitted 78 protocol bytes. Manifest still contained `fresh-process-only` and `fallback: null`; this was not the required complete extracted-package smoke protocol. |
| Locked `cargo metadata --no-deps --format-version 1` for root, native, runtime, and wire manifests | 0 each | Manifest metadata parsed. This does not repair the missing wire lock or facade package failure. |
| Root, wire, native, and runtime `cargo fmt ... -- --check` | 0 each | Formatting passed. |
| `git diff --check origin/master..HEAD` | 0 | Candidate diff was whitespace-clean. |
| `git diff --exit-code -- Cargo.lock native/Cargo.lock runtime/Cargo.lock` | 0 | Executed gates did not rewrite the three checked-in locks. `wire/Cargo.lock` remained absent. |

Earlier locked owning-crate checks at the same candidate established the
following additional release-state facts: root `cargo check --locked
--workspace` passed; the standalone native locked check required a lock update;
the standalone wire locked check failed for its missing lock; and the standalone
runtime check could not compile before code generation because required Bun
generated files were absent while its build script also advertised warning-only
unlinked mode. The successful linked build above is the stronger current build
evidence and does not remove those lock and default-build defects.

## Search Evidence

Implementation-only searches excluded `docs/`, `vendor/`, build targets, and
`.git`.

- Required owner search for `BunProviderBackend`, `SelectedProviderPackage`,
  `ProviderInvocation`, `OfferReadyProof`, `ReservationReleaseProof`,
  `InvocationReadyProof`, `RetirementProof`, `DurableReaper`,
  `RetirementQuarantine`, `QuarantineCompletionClaim`, `RestartableCustody`,
  and `RetiredDisposal`: no matches, `rg` exit 1.
- Forbidden search found the raw installer/reexport, public `DriveRequest`,
  mechanical `JavaScriptRejection`, `libbun-native`, `internal-adapter`, public
  `drive_prepared_export`, public wire dependency, `process::abort`, `setpgid`,
  process-group `kill`, blocking child waits, unconditional joins,
  `fresh-process-only`, and `fallback` in active implementation/configuration.
- Test-attribute search found 14 facade tests and one wire codec unit test. It
  found no native/runtime tests and no retained-backend, readiness-proof,
  retirement, quarantine, reaper, completion-claim, or consuming-shutdown
  tests.
- Workflow search found only `.github/workflows/ci.yml`; no release workflow
  exists.

## Omissions That Prevent Composition

The candidate has no exact owner product or lifecycle proof algebra, no retained
worker, no durable queue/reaper, no lawful quarantine completion ownership, no
consuming shutdown, no exact Linux/macOS containment, no persistent output
pumps, no private wire/native boundary, no compile-fail privacy suite, no
nonzero linked engine test target, no reproducible vendor replay, no packageable
facade, no compliant worker package, and no immutable-tag release workflow.

These are implementation omissions in the owned W1-11/W1-12 product, not
post-composition cleanup. W1-13 remains dependent on a corrected W1-11/W1-12
implementation and a new exact-tree composition review; this candidate does
not clear that dependency.

## Final Disposition

**REVISE.** Preserve this candidate only as rejected negative evidence. Apply
the exact installer poison cut first, then implement the frozen owner move and
fifteen-step hard cut. Do not merge or publish this candidate as the retained
libbun lifecycle or worker release product.
