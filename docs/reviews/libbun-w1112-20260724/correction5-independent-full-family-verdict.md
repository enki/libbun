BUNDLE REVISE

# W1-11/W1-12 Correction-5 Independent Full-Family Verdict

Date: 2026-07-24

## Literal part verdicts

- Owner/correspondence: `PART BUNDLE REVISE`
- Lifecycle custody: `PART BUNDLE PASS`
- Atomic deletion/tests and containment/release: `PART BUNDLE PASS`
- Overall: `BUNDLE REVISE`

Synthesis remains blocked. Oracle and Fable remain `NOT LAUNCHED`.

## Bound identity

- Correction-5 candidate: `bc065ac24778d2283dc6915b7e888cd72dda1170`
- Candidate tree: `f6065b359a4fdf72722154a3f9ab147dbc64ccd4`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Lane owner: `libbun-c5-independent-review-20260724`
- Lane worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c5-independent-review-20260724`
- Lane Cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-16`

The Lane resolved to the exact assigned worktree and Cargo target, was detached
at the candidate commit, and was clean before review. The correction-5
generator passed all nine deterministic reports. The verifier passed its
differently named clean-checkout replay, exact snapshot and ordered-list
parity, zero-product-delta gate, launch-state gate, and token bounds.

| Part | Ordered files | Oracle total tokens | State |
| --- | ---: | ---: | --- |
| owner-generative | 32 | 197557 | `NOT LAUNCHED` |
| lifecycle | 37 | 269587 | `NOT LAUNCHED` |
| containment-release | 56 | 251954 | `NOT LAUNCHED` |
| synthesis | 35 | 241995 | `NOT LAUNCHED`; explicitly blocked |

The Oracle and Fable ordered rows have exact count and digest parity for every
part. Every total is below the 272000-token cap. The synthesis manifest remains
`CORRECTION 5 SYNTHESIS BLOCKED; THREE FRESH LITERAL PART BUNDLE PASS VERDICTS
PENDING`.

## Owner/correspondence: PART BUNDLE REVISE

Correction 5 repairs the correction-4 named omissions for the final output
correspondence join, `ProviderBoundaryIngressFault::OutputCorrespondence`, the
selected `ProviderHostExecutionSession` fields/constructor/reexport/Drop, its
durable-loader caller, and the two previously identified drive/fault carriers.
Those source items and the capability-model correspondence tests are genuinely
attached.

The owner-family source closure is nevertheless incomplete. The correction-5
repository-wide search titled “Adjacent final correspondence, typed-fault,
finite-owner, and caller closure” returns 27 active paths. Ten of those paths
are absent from `OWNER_SOURCE_PATHS`, the compact owner source bundle, the
adjacent snapshots, and the 32-file owner plan:

```text
crates/ss-runtime-source-compiler-owner/src/direct_run.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/public_aperture_entrypoint/trusted_step.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/session_route_lifecycle.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_result_route.rs
crates/ss-runtime-source-compiler-owner/src/lib.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_direct_run_prepared_runtime.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/final_observation/host_resource_finalization.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine.rs
```

This is not a filename-only defect. The omitted owning items contain:

- `DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1`
  and `AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1`,
  which carry `ProviderHostExecutionSession` by value through host admission and
  terminal drive;
- the two current source-entrypoint constructors that call
  `begin_provider_execution_session_v1`, plus their typed refusal and
  `DirectRunProcessSessionDriveFaultV1` cancellation/custody handling;
- the start, reawaken, and provider-resume host-resource-finalization routes and
  the process-session result carriers that borrow the same execution session;
- `commit_exact_provider_release_for_session_execution_kernel_owner_v1`, the
  selected release correspondence, and its final publication path; and
- the actual hostile selection/release tests for foreign cross-splice,
  twenty-thousand-drop reissue, unwind reissue, refusal/retry, and exact
  publication. In particular,
  `process_session_provider_commit_unwind_retains_exact_custody_through_retry_publication`
  is absent from every owner model input.

The generated exact-source search report prints match lines for these files,
but that report is itself absent from the owner file plan. Even if it were
present, grep output would not replace complete owning source items. The
verifier misses the defect because it compares only the old 24-path lexical
search to `GENERATIVE_PATHS`; it never compares the new 27-path semantic search
result to `OWNER_SOURCE_PATHS`. Its per-path deletion checks therefore operate
only on a preselected incomplete set.

Without these sources, a model cannot trace the selected finite owner from
both real session constructors through admission, route driving, exact host
resource release, refusal/retry/cancellation, Drop reissue, and the final
publication consumer. The bundle cannot authorize the requested full-SCC
owner move or prove that libbun remains a purely mechanical drive/result/fault
owner while authored settlement stays in the outer Swarm owner.

### Exact correction required

1. Bind the repository-wide 27-path semantic search result as an exact ordered
   discovery set. Compare the observed result to that set in both generator and
   verifier before rendering. Removing any discovered path must fail.
2. Add exact-SHA snapshots and compact complete-owning-item excerpts for all ten
   omitted paths. Small module/reexport files may use narrow complete spans;
   the seven live carrier/consumer files must include their complete relevant
   types and operations, not isolated match lines.
3. At minimum, bind these transitions end to end:

   ```text
   source_entrypoint_direct_run_prepared_runtime
     -> begin_provider_execution_session_v1
     -> prepared-runtime process-start admission input/refusal
     -> admitted terminal drive
     -> session route/result carriers
     -> exact host-resource release selection/commit
     -> final publication or typed custody-retaining refusal/fault
   ```

4. Attach the complete selection seal, both Drop implementations, reissue and
   consume operations, exact provider-release commit, and all hostile tests in
   `host_resource_finalization.rs`, including the final process-session unwind
   test.
5. Add per-path required-item bindings for both source-entrypoint session mints,
   all by-value and borrowed `ProviderHostExecutionSession` carriers, the exact
   release operation, and the hostile mismatch/Drop/refusal/unwind tests.
6. Regenerate owner and synthesis plans, dry runs, Fable plans, manifests,
   digests, and token totals. Preserve identical Oracle/Fable order, the
   272000-token cap, zero product/test/Cargo/vendor/workflow delta,
   `NOT LAUNCHED`, and clean-checkout replay.

## Lifecycle custody: PART BUNDLE PASS

The lifecycle correction source-binds both active `BunProcess.cpp` exit
callers, including `Bun__handleUncaughtException` and its
`Bun__Process__exit(lexicalGlobalObject, 1)` branch. It also binds the complete
`NodeVMModule::evaluate` item containing microtask drain, exception clearing,
`clearHasTerminationRequest`, SIGINT translation, and timeout translation.
Independent repository searches confirm these routes beside NodeVMScript,
RuntimeHooks, WebWorker termination/wait, live-worker registration and
unregistration, and ordered shutdown.

The lifecycle prompt preserves the correct law: termination reset is not
quiescence; reuse requires complete invocation, microtask, diagnostic,
output-barrier, finalization, and child/nested-worker drain. Cancellation,
timeout, exception unwind, future/final Drop, reaper races, and shutdown retain
or durably transfer typed custody. Shutdown-origin custody never returns Ready
or Restartable. The full retained-runtime fixture and required hostile
implementation tests remain bound. No determining lifecycle omission remains.

## Atomic deletion/tests and containment/release: PART BUNDLE PASS

The repository-wide final-close search returns exactly the nine bound paths.
The bundle includes the retained-runtime inner shutdown chain, both outer
`close_for_execution_graph_owner` carriers, their privacy/module boundary, and
`produce_graph_close_receipt_for_execution_graph_owner`, the sole final
success/fault consumer. The final consumer's runtime-file reap, graph/feed
consumption, success settlement, failed-closeout settlement, and runtime-file
session restoration are source-bound.

The prompt correctly requires exact-once retained-runtime shutdown, forbids
retry from a consumed backend, and preserves refusal/retry, cancellation,
unwind, Drop, process-exit, and final-shutdown custody. It also binds the real
worker fixture, stale plugin callers, package/link/reproducibility scripts,
freshly extracted smoke, locks, privacy harness, and compliance inventory. No
determining containment/release omission remains.

## Executed evidence

```text
lane run --owner libbun-c5-independent-review-20260724 --repo /home/ubuntu/libbun --
  sh -lc 'pwd; printf "%s\n" "$CARGO_TARGET_DIR"; git rev-parse HEAD; git status --short --branch'
exit 0; exact assigned worktree, Cargo target, candidate OID, detached clean state

LIBBUN_REPO=<assigned-worktree> SWARM_REPO=/home/ubuntu/swarm
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0; all nine reports OK

SWARM_REPO=/home/ubuntu/swarm
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0; clean-checkout replay PASS; 32/37/56/35 ordered files;
197557/269587/251954/241995 tokens

git -C /home/ubuntu/swarm grep -l -E
  'ProviderBoundaryOutputCorrespondenceFault|consume_corresponded_ready_output_for_provider_boundary_owner_v1|ProviderBoundaryIngressFault|DirectRunProcessChildProviderFaultV1|DirectRunProcessSessionDriveFaultV1|ProviderDriveSessionExecutionCommitFault|ProviderHostExecutionSession|begin_provider_execution_session_v1|cross_boundary_swap_is_a_typed_fault|nominal_join_preserves_both_halves_on_mismatch'
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0; 27 paths; ten are not source-bound by the owner plan

git -C /home/ubuntu/libbun grep -l -E
  'Bun__Process__exit|global_exit|terminate_all_workers_and_wait|terminate_all_and_wait|WebWorker__notifyNeedTermination|clearHasTerminationRequest|requestTermination|notifyNeedTermination|clearTerminationException'
  6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- vendor/bun/src/jsc vendor/bun/src/runtime
exit 0; active lifecycle routes independently inspected

git -C /home/ubuntu/swarm grep -l -E
  'close_for_execution_graph_owner|shutdown_runtime_execution_domain_owner|SsRuntimeExecutionDomainOwner|ProviderSettlementLane|ExternalCapabilityProviderPool'
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates/ss-runtime-test-execution-owner/src
exit 0; exact nine-path final-close set
```

This review changes only this verdict document. It does not modify models,
product source, vendor source, tests, fixtures, workflows, release source,
Cargo files, prepared manifests, prompts, file plans, dry runs, generator, or
verifier.
