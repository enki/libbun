BUNDLE REVISE

# W1-11/W1-12 Correction-6 Independent Full-SCC Verdict

Date: 2026-07-24

## Literal part and union verdicts

- Owner/correspondence: `PART BUNDLE REVISE`
- Lifecycle custody: `PART BUNDLE PASS` (preserved from correction 5)
- Atomic deletion/tests and containment/release: `PART BUNDLE PASS` (preserved from correction 5)
- Three-part union: `BUNDLE REVISE`

Synthesis remains blocked. Oracle and Fable remain `NOT LAUNCHED`.

## Bound identity

- Correction-6 candidate: `f9c0a3a35c182364efadf14548a142f8ce0fb772`
- Candidate tree: `407a91aceadf57564716e9a1e9c8655449593269`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Lane owner: `libbun-c6-independent-review-20260724`
- Lane worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c6-independent-review-20260724`
- Lane Cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-16`

The Lane resolved to the exact assigned worktree and Cargo target, was detached
at the candidate commit, and was clean before review.

## Mechanical evidence passes

Correction 6 is an evidence-only delta: all 41 candidate-parent changes are in
the review bundle, its generated exact-SHA snapshots, or the two bundle scripts.
There is no product, test, Cargo, vendor, runtime, native, or wire delta from the
frozen libbun product SHA.

The generator passes all ten deterministic reports. The independent verifier
passes its clean-checkout replay, snapshot/blob parity, ordered-list parity,
zero-product-delta check, launch-state gate, and token bounds. The independent
repository searches reproduce the ordered 27-path semantic-owner set and the
ordered 16-path `ProviderHostExecutionSession` set exactly.

Fresh wrapper dry runs reproduce the prepared requests:

| Request | Files | Attachment tokens | Call tokens | State |
| --- | ---: | ---: | ---: | --- |
| owner-generative | 33 | 235,976 | 237,238 | `NOT LAUNCHED` |
| lifecycle | 37 | 270,584 | 271,523 | `NOT LAUNCHED` |
| containment-release | 56 | 252,679 | 253,640 | `NOT LAUNCHED` |
| synthesis | 32 | 271,369 | 272,217 | `NOT LAUNCHED`; explicitly blocked |

The checked-in dry-run report SHA-256 values are respectively
`3f2c3f29be3ba5fca27b8e3fb2ca70c0649e1b1150d34dd2ae1b2330d165cfdd`,
`bb57b46f980145502bc42c22584206902118a2b10888ca173db4bb2ed5d5b512`,
`50365e8a5bfdf28d7fc7b8b1841033034e9ee209d0286bf2a7a6163bf5bf19f7`,
and `d8e4b1b21332adee734510e04488ea5bde02cdca94d6d84ca4489a3647aea4a5`.

Those mechanical passes do not establish source closure.

## Owner/correspondence: PART BUNDLE REVISE

Correction 6 does attach every path returned by its 27-path semantic search and
every path returned by its 16-path execution-session search. It also attaches
both real session mints, the by-value prepared-runtime admission and terminal
drive, the borrowed route carriers, the complete private finalization-selection
seal with both Drop guards, the reissue/refusal tests, and the named
`process_session_provider_commit_unwind_retains_exact_custody_through_retry_publication`
test.

The requested full semantic SCC is nevertheless not source-closed. An
independent exact-source search for the release authority, transfer set,
session pending custody, release fault carriers, and unwind hook returns 23
active paths. Fifteen are wholly absent from the owner plan, source bundle, and
snapshot inventory:

```text
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap/materialized_payloads_and_allocation.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap/projection_and_receiver_types.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap/runtime_value_projection/boundary_value_admission.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap/runtime_value_projection/boundary_value_conversion.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_state/frame_plans_and_state_types.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_state/register_resource_obligation_transitions.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_state/terminal_settlement_and_faults.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/observable_effect_coverage.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/open.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scoped_frame_lifecycle.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/state.rs
crates/swarm-rust-sdk-static-provider-host/src/builtin_operation_admission.rs
```

The structural include owner
`crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/runtime_heap/runtime_value_projection.rs`
is absent as well. It must accompany the two selected projection implementation
files so their effective module/privacy boundary is visible.

Four paths already selected by the bundle contain incomplete owning items:

- `crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs`
  begins its release excerpt at line 680, midway through an `Err` arm. It omits
  the definitions and complete implementations of
  `SelectedProviderHostResourceReleaseV1`, its closed inner algebra,
  `ProviderHostResourceReleaseReceiptV1`,
  `ProviderHostResourceReleaseRefusalV1`,
  `ProviderHostResourceReleaseTransferSetV1`, the exact carrier consumption and
  finish checks, and the borrowed release commit.
- `crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs` includes
  the test fixture entry but omits the finite host-set operations that dispatch
  consuming and borrowed release commit to the static owner.
- `crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs`
  includes the obligation and its final provider call but omits the actual
  binding merge, the one-take `ProviderHostResourceReleaseCustodyV1`, the shared
  carrier duplication law, the exact authority take, the runtime-family entry,
  and the entry-to-finalization-obligation conversion.
- `crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs`
  includes the final release commit but omits the earlier corresponded ready
  output plus `ProviderHostResourceReleaseTransferSetV1` admission into the
  runtime heap.

These omissions cut the source trace at every determining release boundary:

```text
static provider owner mints SelectedProviderHostResourceReleaseV1
  -> ProviderHostResourceReleaseTransferSetV1 accompanies exact ready output
  -> transactionally corresponded runtime-heap admission consumes each release
     only for its matching HostResourceHandleCarrier
  -> the one-take carrier is merged into HostResourceBindingValue
  -> scoped-frame exit creates OneShotHostResourceFinalizationObligation
  -> ProcessSessionV0 installs pending exact-selection custody
  -> ProviderHostExecutionSession performs the real borrowed owner commit
  -> consumed custody remains alive through sole continuation publication
```

In particular, the named hostile test calls the omitted
`arm_next_static_provider_host_resource_release_commit_unwind_for_test_support_v1`
and omitted real static-owner borrowed commit. The attached call site cannot
prove that the unwind occurs after all fallible validation and before either
resource-state or selected-authority mutation. It also calls the omitted
`ProcessSessionV0::try_reissue_cancelled_host_resource_finalization_selection_for_session_runtime_owner_v1`.
The test text therefore does not prove unwind/retry custody without those
owners.

The missing `observable_effect_coverage.rs`, `state.rs`, `open.rs`, and
`execution_kernel.rs` items also prevent verification that selected custody is
installed once, blocks an unrelated drive/root-terminal transition, survives
fault/unwind, and is reissuable only from the cancelled state. The omitted heap
and execution-state items prevent verification that mismatch and transfer
faults restore their source values instead of discarding release authority.

The 27/16 searches miss this because their patterns are centered on
`ProviderHostExecutionSession`, output-correspondence faults, and the named
tests; none binds the complete release-transfer and pending-session-custody
families. Per-path term checks over that preselected set cannot detect an
unselected producer, carrier, or consumer, and a full-file hash printed beside
a partial excerpt does not supply the omitted source bytes.

### Exact minimal correction

1. Add an independently fixed ordered exact-release SCC discovery to both
   generator and verifier. It must cover the 23 active paths found from
   `SelectedProviderHostResourceReleaseV1`,
   `ProviderHostResourceReleaseTransferSetV1`,
   `PendingSelectedHostResourceFinalizationBoundaryV1`,
   `pending_selected_host_resource_finalization`,
   `RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1`, release transfer
   faults, exact commit operations, the unwind arm, and the named hostile test.
   Keep the existing 27-path and 16-path checks independently fixed.
2. Add exact-SHA snapshots and complete owning-item excerpts for the fifteen
   absent paths above plus the `runtime_value_projection.rs` include owner.
3. Extend the four incomplete existing paths through the complete release
   algebra/transfer-set operations, host-set dispatch, exact heap/binding
   correspondence, scoped-lifecycle conversion, and session ready-output
   admission. Bind the exact owner operations and typed restoration paths as
   required items in both scripts.
4. Preserve the complete finalization seal, both Drop guards, twenty-thousand
   reissue, refusal/retry, foreign splice, presented-unwind, and real
   unwind-through-publication test already attached by correction 6.
5. Regenerate owner and synthesis plans, snapshots, source bundle, search
   reports, Oracle dry runs, Fable plans, manifests, digests, and token totals.
   Preserve exact Oracle/Fable order, the 272,000-token cap, zero product delta,
   `NOT LAUNCHED`, and blocked synthesis until a fresh literal owner
   `PART BUNDLE PASS` exists.
6. Preserve the semantic ruling: libbun owns only the one-shot mechanical drive
   outcome; installed-capability/outer Swarm remains the authored settlement
   owner. The implementation request must continue to reject RAW selectors or
   parts, callback proof, borrowed authority mint, fallback, placeholder faults,
   and semantic settlement in libbun.

No product implementation or model launch is authorized by this verdict.

## Preserved part passes

Correction 6 does not change the product source or the determining evidence for
the correction-5 lifecycle and containment/release families. Their literal
`PART BUNDLE PASS` verdicts remain controlling. The lifecycle bundle continues
to bind process exit, termination/reset, worker drain, cancellation/unwind,
retirement/quarantine, Drop, and shutdown custody. The containment/release
bundle continues to bind the final close owner, both outer carriers, sole final
success/fault consumer, packaging/link/reproducibility inputs, privacy harness,
and fresh extracted smoke.

## Executed evidence

```text
lane run --owner libbun-c6-independent-review-20260724 --repo /home/ubuntu/libbun --
  sh -lc 'pwd; printf "CARGO_TARGET_DIR=%s\n" "$CARGO_TARGET_DIR"; git rev-parse HEAD; git status --short --branch'
exit 0; exact assigned worktree, Cargo target, candidate OID, detached clean state

LIBBUN_REPO=<assigned-worktree> SWARM_REPO=/home/ubuntu/swarm
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0; all ten deterministic reports OK

SWARM_REPO=/home/ubuntu/swarm
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0; clean-checkout replay PASS; 33/37/56/32 ordered files;
235976/270584/252679/271369 attachment tokens

oracle --provider openai --engine api --model gpt-5.6-sol --reasoning-mode pro
  --dry-run summary --files-report --prompt <part-prompt> --file <ordered-part-files>
exit 0 for owner, lifecycle, containment-release, and blocked synthesis;
33/37/56/32 files; no model launched

git -C /home/ubuntu/swarm grep -l -E <correction-6 semantic-owner pattern>
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0; exact ordered 27-path set

git -C /home/ubuntu/swarm grep -l -E
  'ProviderHostExecutionSession|begin_provider_execution_session_v1'
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0; exact ordered 16-path set

git -C /home/ubuntu/swarm grep -l -E
  'SelectedProviderHostResourceReleaseV1|ProviderHostResourceReleaseTransferSetV1|PendingSelectedHostResourceFinalizationBoundaryV1|pending_selected_host_resource_finalization|RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1|into_finalization_obligation_for_one_shot_lifecycle_owner_v1|HostResourceReleaseAuthorityUnavailable|HostResourceReleaseTransfer|selected_provider_release|selected_release|commit_exact_provider_release_for_session_execution_kernel_owner_v1|commit_selected_host_resource_release_borrowed_for_session_execution_kernel_owner_v1|arm_next_static_provider_host_resource_release_commit_unwind_for_test_support_v1|process_session_provider_commit_unwind_retains_exact_custody_through_retry_publication'
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0; 23 active paths; fifteen absent and four incompletely excerpted

git diff --check 5e74c14a0125c1670be7e37cc31675ebedcd538d
exit 0
```

This review changes only this verdict document. It does not modify product
source, tests, fixtures, manifests, prompts, prepared dry runs, generator,
verifier, Cargo files, vendor source, workflows, or release source.
