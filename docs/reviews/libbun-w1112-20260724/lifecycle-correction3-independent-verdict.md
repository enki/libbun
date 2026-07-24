PART BUNDLE REVISE

# W1-11/W1-12 Correction-3 Lifecycle Bundle Review

Date: 2026-07-24

## Bound review identity

- Review candidate commit: `c2ea016e4c9810fa86ddfd21bd4b30823746a9b9`
- Review candidate tree: `67bdbd8830930ed39d19e7f37be092c108de01f7`
- Exact product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Review Lane owner: `libbun-c3-lifecycle-review-20260724`
- Review worktree:
  `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c3-lifecycle-review-20260724`
- Review `CARGO_TARGET_DIR`:
  `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-13`
- Model launch: none; Oracle and Fable remain `NOT LAUNCHED`.

The assigned Lane resolved to the exact worktree and cargo slot above, was
clean and detached at the candidate, and matched the requested candidate SHA
before review. The correction-3 generator check and fail-closed verifier both
exit zero from this independent Lane. The verifier also successfully replays
all six generated reports in a differently named clean checkout. Product,
test, Cargo, vendor, and workflow source remain unchanged from the exact
product source.

Those mechanical gates close the two correction-2 defects, but the lifecycle
bundle is still not source-complete. Its vendored search stops at a runtime
hook declaration and omits the concrete process-exit -> worker-termination ->
worker-shutdown SCC needed to judge quiescence. It also replaces the prior
full retained-runtime Rust integration fixture with grep output. The bundle
therefore cannot authorize an implementation-model launch.

## Determining defect: the attached VM calls an unbundled shutdown SCC

The ordered plan attaches complete `VirtualMachine.rs`, `JSGlobalObject.rs`,
`VM.rs`, and `virtual_machine_exports.rs`, plus selected C++/Zig excerpts. That
is not the complete lifecycle/caller closure claimed by the prompt and
supplemental bundle.

At the exact product source, attached
`vendor/bun/src/jsc/VirtualMachine.rs:1511` defines `global_exit`. At line 1532
it calls `(hooks.terminate_all_workers_and_wait)(10_000)`. The attached file
contains only the `RuntimeHooks` function-pointer field. The concrete binding
and transition continue through files absent from `lifecycle-files.txt`:

1. `vendor/bun/src/runtime/jsc_hooks.rs:1525` implements
   `terminate_all_workers_and_wait` and forwards to
   `bun_jsc::web_worker::terminate_all_and_wait`.
2. `vendor/bun/src/jsc/web_worker.rs:228-396` owns the process-global live
   worker registry, registration/unregistration ordering, outstanding counter,
   repeated termination sweep, VM-lock handoff, wakeup, bounded futex wait,
   and timeout return.
3. `vendor/bun/src/jsc/web_worker.rs:673` is the exported per-worker
   `notifyNeedTermination` path, and its `spin`/checkpoint paths determine
   whether cooperative termination reaches shutdown.
4. `vendor/bun/src/jsc/web_worker.rs:1206` owns ordered worker shutdown:
   unpublish the VM, run exit handlers, tear down JSC, unregister from the live
   set, dispatch exit, and then free worker-thread resources. Its
   `WebWorker::exit` at line 1331 is the worker-side `process.exit` path.
5. `vendor/bun/src/runtime/node/node_process.rs:53` owns
   `Bun__Process__exit` and chooses worker `exit` versus main-VM
   `global_exit`. `vendor/bun/src/jsc/bindings/BunProcess.cpp` contains the
   public process entry and termination-request checks. Neither source is
   attached or excerpted as a complete item.

The checked-in vendored search cannot reveal this gap because its pathspec is
limited to seven files:

```text
vendor/bun/src/jsc/VirtualMachine.rs
vendor/bun/src/jsc/VM.rs
vendor/bun/src/jsc/JSGlobalObject.rs
vendor/bun/src/jsc/virtual_machine_exports.rs
vendor/bun/src/jsc/bindings/bindings.cpp
vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp
vendor/bun/src/jsc/VirtualMachine.zig
```

A fresh exact-tree search for termination request/reset and worker shutdown
finds the omitted owners above, as well as the C++ worker bridge and concrete
callers in `bindings/webcore/Worker.cpp`, `bindings/vm/SigintWatcher.cpp`,
`NodeVM.cpp`, and `NodeVMScript.cpp`. The verifier repeats the same bounded
pathspec and hard-codes only the already attached four Rust files, so its PASS
does not establish the missing SCC.

This omission determines the verdict. `InvocationReadyProof` may authorize
same-epoch reuse only after the exact invocation, its microtasks, persistent
output, diagnostics, finalization, and child-worker activity are quiescent.
`RetirementProof` may authorize restart only after worker death and complete
drain. Without the actual live-worker registry, termination wait, timeout,
unregister, process-exit selection, and resource-freeing order, a model cannot
prove whether a child worker survives a cooperative reset, whether a timeout
is ambiguous and must retire, or whether process exit/final shutdown has
released the custody needed for reuse or restart. The empty
`JSC__VM__deinit` body does not fill that gap.

## Hostile fixture regression

Correction 2 directly attached the full adjacent Rust fixture
`crates/ss/tests/external_capability_provider.rs`, including
`ss_reuses_one_libbun_runtime_for_multiple_capability_imports`, its real linked
plugin setup, and the pool-child package-root fixture. Its independent verdict
explicitly required those attached sources and fixtures to remain in the
corrected source-aware bundle.

Correction 3 removes that full source from the 39-file lifecycle plan. The
process/Drop report contains only matching lines from it. The same correction
also removes the adjacent `crates/ss/src/product.rs`, `crates/ss/Cargo.toml`,
root `Cargo.toml`, and exact-route producer snapshot that were in the
correction-2 lifecycle plan. The newly attached `ss-test` pool implementation
is valuable and must remain, but it does not replace the real end-to-end Rust
fixture or its construction helpers.

The lifecycle prompt requests commit-grade hostile tests for same-worker
reuse, cancellation, deadline, unwind, worker replacement, output barriers,
reaper races, working-directory replacement, and final shutdown. Grep excerpts
cannot serve as editable source or show the complete existing harness. A
source-aware implementation model would have to invent the test owner,
package builder, linked-plugin preparation, and invocation commands.

## Exact correction

Keep product, test, Cargo, vendor, and workflow source unchanged, then make the
following review-evidence correction.

1. In
   `scripts/generate-libbun-w1112-review-evidence-20260724.py`, make the first
   evidence edit a repository-wide vendored lifecycle inventory for
   `Bun__Process__exit`, `process_exit`, `global_exit`,
   `terminate_all_workers_and_wait`, `terminate_all_and_wait`, live-worker
   register/unregister, `notifyNeedTermination`, termination request/reset,
   `spin`, `shutdown`, `destroy`, microtask drain, and every matching caller.
   Record the stable repository label, exact SHA, pattern, pathspec, exit, and
   output as the existing deterministic reports do.
2. Generate a separately ordered and hashed lifecycle source sub-bundle that
   supplies complete source items, not filename mentions, for:
   - the `Bun__Process__exit` owner in
     `vendor/bun/src/runtime/node/node_process.rs`;
   - the `process_exit` and `terminate_all_workers_and_wait` bindings in
     `vendor/bun/src/runtime/jsc_hooks.rs`;
   - the live-worker registry, `terminate_all_and_wait`, exported
     notification, checkpoint/spin transition, full `shutdown`, and
     worker-side `exit` items in `vendor/bun/src/jsc/web_worker.rs`;
   - the exact public process and C++ worker bridge items in
     `vendor/bun/src/jsc/bindings/BunProcess.cpp` and
     `vendor/bun/src/jsc/bindings/webcore/Worker.cpp`; and
   - every additional repository-wide match that changes termination clearing,
     process-exit, child-worker drain, or shutdown semantics.
3. Restore the full exact-Swarm Rust fixture
   `crates/ss/tests/external_capability_provider.rs` to the lifecycle ordered
   plan. Restore the top-level adjacent install/manifest and exact-route
   producer sources removed from the correction-2 lifecycle plan, or attach a
   smaller complete-item sub-bundle that preserves their constructors,
   dependency edges, real-binary commands, and shutdown ownership without
   grep-only reconstruction. Keep the correction-3 retained-host pool files.
4. Rebudget below 272000 tokens by replacing the current 25494-token raw
   process/Drop match dump with a concise deterministic inventory plus the
   complete-item source bundle. Do not remove the newly attached complete
   `VirtualMachine.rs`, `JSGlobalObject.rs`, retained-host checkout/replacement
   chain, sole consumer, typed fault paths, or `.ss` fixtures.
5. Extend `REQUIRED_ATTACHMENTS["lifecycle"]` and report checks in
   `scripts/verify-libbun-w1112-review-bundle-20260724.py` to require the new
   process-exit/worker-shutdown sub-bundle, the full Rust integration fixture,
   and the restored source closure. Add a negative verifier self-check proving
   that deletion of any one process-exit, hook, live-worker, shutdown, or
   fixture item fails verification.
6. Align `lifecycle-prompt.md` with the exact new inputs. It must explicitly
   require child-worker and nested-worker custody, cooperative-termination
   timeout ambiguity, process-exit selection, future/final terminal `Drop`,
   and shutdown-origin non-recovery in the implementation and default-parallel
   hostile tests.
7. Regenerate the report, ordered files, Fable plan, Oracle dry run, manifest,
   attachment hashes, byte totals, and generator/verifier hashes. Keep Oracle
   and Fable `NOT LAUNCHED`.
8. From a newly claimed independent Lane at the corrected review SHA, run the
   generator check and verifier. Both must exit zero with no `DRIFT`; the
   verifier must replay from a differently named clean checkout and the
   lifecycle dry run must remain below 272000 tokens.
9. Obtain a fresh independent lifecycle part review. Launch remains forbidden
   until that review commits a literal `PART BUNDLE PASS`.

## Preserved lifecycle law

The correction must not weaken the frozen algebra. Private `DriveCustody` and
`InvocationReadyProof` alone authorize fulfilled, rejected, or cooperatively
cancelled same-worker reuse after complete drain. `RetirementProof` alone
authorizes restart. Every forced cancellation, deadline, unwind, ambiguous
finalization, active-worker shutdown, reaper failure, future `Drop`, and final
terminal `Drop` must retain custody or transfer intact
`RetirementQuarantine<Purpose>` by value to the preallocated `DurableReaper`
before a bounded typed fault exists. Recovery remains one private affine
`QuarantineCompletionClaim<Purpose>`; shutdown consumes the backend from every
state and shutdown-origin custody never returns to Ready or Restartable.

## Executed evidence

```text
LIBBUN_REPO=<assigned-worktree> SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0: all six generated reports OK

SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0: independent-checkout replay PASS; lifecycle 39 files, 268426 tokens

git grep -l -E \
  'request[Tt]ermination|clear[Tt]ermination|notify[Nn]eed[Tt]ermination|has[Tt]ermination[Rr]equest|JSC__VM__deinit|terminate_all_workers_and_wait' \
  6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- vendor/bun/src
exit 0: finds the attached files plus unbundled process, hook, worker,
NodeVM, SigintWatcher, REPL, test-runner, and WebWorker owners/callers

git diff --name-only \
  <correction-2 lifecycle-files.txt> \
  docs/reviews/libbun-w1112-20260724/lifecycle-files.txt
exit 1 as expected: confirms removal of the full adjacent Rust fixture,
top-level install/manifest sources, exact-route producer, and exact-source
search report while adding the correction-3 pool and full Rust VM sources
```

## Final disposition

`PART BUNDLE REVISE`. Preserve the correction-3 Lane-independent replay and
full Rust VM attachments. Add the missing concrete process-exit/WebWorker
shutdown SCC and restore the full hostile fixture closure before any Oracle or
Fable launch.
