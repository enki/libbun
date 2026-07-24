PART BUNDLE REVISE

# W1-11/W1-12 Correction-4 Lifecycle Independent Verdict

Date: 2026-07-24

## Bound identity

- Correction-4 candidate: `4dd3395129a221d8c1fb2d1dbbdae509b2331f0e`
- Candidate tree: `fbd9f82cfae0554abe87623f080f0ce4eb1c6b91`
- Exact libbun product: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Review Lane owner: `libbun-c4-lifecycle-review-20260724`
- Worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c4-lifecycle-review-20260724`
- `CARGO_TARGET_DIR`: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-17`
- Lifecycle manifest SHA-256: `a11b05b7a4f90bc9cd81fe44fd31dbf84685fe0de7c4dec8a9da16d6200df515`
- Lifecycle ordered plan SHA-256: `00689c94b1b2e86a91523146bb63c618a3cfd8750df8bb139b44610f98bb49c8`
- Lifecycle prompt SHA-256: `db33f0c83d35142167661c69d3eb55118a0c333753ba4d9f76073519051e230b`
- Lifecycle dry-run SHA-256: `c6cb5f387ea1b4b66971f0474c862793a5fd7cd07e20c793d68104d1b3388f4e`
- Generator SHA-256: `f4158b8bf8d9862de47105b87957bf961d7d87f8ec1ba7096de5156c5fa31309`
- Verifier SHA-256: `52c35938408a84107acb43282f27761844369494fd6fcb6b3ae1ce6a9e54b749`
- Oracle and Fable: `NOT LAUNCHED`

The Lane resolved to the assigned worktree, cargo slot, detached candidate
HEAD, and a clean tree before review. Correction 4 closes the correction-3
omissions for the core process-exit -> RuntimeHooks -> live WebWorker
terminate/wait -> ordered worker shutdown path and restores the full retained
runtime fixture and pool sources. The generator and verifier both exit zero;
the verifier also replays from a differently named clean checkout. Those
mechanical results are valid.

## Determining defect: an active termination-reset owner is outside the bundle

The lifecycle source bundle is rendered from the fixed eight-path
`LIFECYCLE_SOURCE_PATHS` tuple. A fresh exact-tree search of the active Rust
and C++ JSC/runtime sources finds another semantic reset owner:

```text
vendor/bun/src/jsc/bindings/NodeVMModule.cpp:132:
    vm.clearHasTerminationRequest();
```

`NodeVMModule::evaluate` drains microtasks, clears the exception, clears the
VM termination request, and then converts the event into a SIGINT or timeout
error. This is the same reusable-VM termination state transition that the
bundle already exposes for `NodeVMScript.cpp:287` and
`ZigGlobalObject.cpp:3140`. It directly determines whether a cooperative
interrupt remains pending, whether selected code can consume/reset it, and
whether `InvocationReadyProof` may authorize same-epoch reuse.

`NodeVMModule.cpp` is absent from `lifecycle-files.txt`,
`lifecycle-process-worker-source-bundle.md`, and
`lifecycle-vendored-jsc-source-bundle.md`. The lifecycle ordered model input
therefore contains neither this definition nor its owning `evaluate` item.
The generated process/Drop report can discover the match, but that report is
not a lifecycle attachment.

The same fixed excerpt selection also drops one live process-exit caller from
the included `BunProcess.cpp` file. Exact source line 1238 invokes
`Bun__Process__exit(lexicalGlobalObject, 1)` when an exception is thrown by an
uncaught-exception handler. The lifecycle bundle includes only lines 280-304
and 3245-3263 from that file, so the model sees the public `process.exit`
bridge but not this second process-exit route. Process-exit selection and
unwind/exception custody are explicit prompt requirements.

The verifier cannot detect either omission. It checks required strings inside
the already selected bundle and performs a text-replacement self-check. It
does not compare a repository-wide active termination-reset/process-exit
discovery result with the rendered source inventory. Its literal PASS is thus
mechanical replay evidence, not a full lifecycle SCC pass.

## Clean correction

Keep the correction-4 core worker bundle, complete Rust VM attachments,
retained-host fixtures, stable repository labels, zero-product-delta gate,
and `NOT LAUNCHED` state. Correct only review evidence:

1. Make the first generator edit a precise exact-SHA repository-wide discovery
   of active Rust/C++ definitions and callers for process exit, termination
   request, termination reset, worker notification, live-worker wait, and
   shutdown. Classify declarations, comments, and inactive legacy mirrors
   explicitly; bind every active semantic item.
2. Add a complete owning-item excerpt for `NodeVMModule::evaluate` from
   `vendor/bun/src/jsc/bindings/NodeVMModule.cpp`, with blob, full-file
   SHA-256/bytes, exact line span, and excerpt SHA-256.
3. Add the complete `Bun__handleUncaughtException` item containing the line
   1238 `Bun__Process__exit` route, or include the complete containing source
   file if that is smaller than maintaining another partial inventory.
4. Extend `LIFECYCLE_SOURCE_PATHS`, the lifecycle required attachments and
   markers, and the prompt so the implementation must preserve timeout,
   SIGINT, exception, and process-exit custody across every reset/exit route.
5. Replace the current string-deletion self-check with a negative discovery
   gate: removing any discovered active path/item from the rendered inventory
   must make verification fail. It must catch an omitted path before bundle
   text exists.
6. Regenerate the bundle, ordered plan, Oracle dry run, Fable plan, manifest,
   hashes, byte totals, and synthesis inputs. The current lifecycle estimate
   is `268280`, leaving bounded room for these two complete items; preserve the
   272000-token cap.
7. Replay the generator and verifier from a differently named clean checkout,
   keep product/test/Cargo/vendor/workflow/release sources unchanged, and
   obtain a fresh independent literal lifecycle verdict before model launch.

## Preserved lifecycle law

`DriveCustody` remains private to `BunProviderBackend`.
`InvocationReadyProof` alone authorizes fulfilled, rejected, or cooperatively
cancelled same-worker reuse after invocation cargo, microtasks, diagnostics,
output barriers, finalization, and all child/nested workers are proven drained.
Timeout, surviving workers, exception-path ambiguity, deadline, unwind,
forced cancellation, join/output failure, and every future/final `Drop` retain
custody or move intact `RetirementQuarantine<Purpose>` to the preallocated
`DurableReaper` before a bounded typed fault. Shutdown consumes the backend
from every state, and shutdown-origin custody never returns Ready or
Restartable.

## Executed evidence

```text
LIBBUN_REPO=<assigned-worktree> SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0: all eight generated reports OK

SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0: independent clean-checkout replay PASS; lifecycle 37 files, 268280 tokens

git grep -n -E \
  'clearHasTerminationRequest|clear_termination|clearTerminationException|request_termination|requestTermination|notifyNeedTermination' \
  6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- vendor/bun/src/jsc vendor/bun/src/runtime
exit 0: includes the unbundled NodeVMModule.cpp:132 reset owner

git grep -n -E \
  'Bun__Process__exit|global_exit|terminate_all_workers_and_wait|terminate_all_and_wait' \
  6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- vendor/bun/src
exit 0: includes the unrendered BunProcess.cpp:1238 process-exit caller
```

Final disposition: `PART BUNDLE REVISE`. No lifecycle implementation-model
launch is authorized until the active reset and process-exit caller closure is
source-bound and independently passes.
