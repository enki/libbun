PART BUNDLE REVISE

# W1-11/W1-12 Correction-4 Owner/Generative Independent Verdict

Date: 2026-07-24

## Reviewed identity

- Correction-4 commit: `4dd3395129a221d8c1fb2d1dbbdae509b2331f0e`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Lane owner: `libbun-c4-owner-review-20260724`
- Lane worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c4-owner-review-20260724`
- Lane Cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-16`

The Lane resolved to the assigned worktree and Cargo target, was detached at the
exact correction-4 commit, and was clean before review. The correction-4
generator and verifier both pass. The verifier also passes its differently
named clean-checkout replay and reports the owner plan at 32 files and 173462
tokens. Independent Git searches confirm that the fixed lexical pattern returns
the advertised 24 paths.

The owner/generative part still cannot receive `PART BUNDLE PASS`. The compact
bundle lists the 24 path names, but it does not include the complete source of
the final correspondence join, the typed fault carriers used by the two drive
consumers, or the selected `ProviderHostExecutionSession` owner lifecycle. The
prompt consequently makes two source claims that the attachments do not prove.

## Determining defects

### 1. The final output join is indexed but not attached

The bundle inventory names:

```text
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  execution_kernel/executable_image/plan/operation_algebra/
  boundary_and_work_selection.rs
```

but correction 4 emits only lines `1-40`. The actual final output commit is the
complete `PendingExecutableProviderBoundary::consume_corresponded_ready_output`
item at exact-SHA lines `148-197`. Its decisive join is:

```rust
let provider_output = pending_output_authority
    .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready_output)
    .map_err(crate::ProviderBoundaryIngressFault::from)?;
```

That source is absent from the attachment. The typed wrapper it invokes is also
absent because this file is not in the 24-path discovery result:

```text
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  errors.rs:215-238
```

`ProviderBoundaryIngressFault::OutputCorrespondence` owns the conversion from
`ProviderBoundaryOutputCorrespondenceFault`. Without both complete items, the
model cannot produce an exact-SHA-applicable migration of the sole final join or
prove that mismatched branded output reaches the existing typed fault.

This omission passes verification because `verify_source_closure` requires only
the inventory row for each path and global marker strings. It does not require
the final-join section or `ProviderBoundaryIngressFault`. Its negative self-check
only removes an already required marker from generated text and confirms that a
substring checker notices the removed substring; it does not prove that the
owning source item was emitted.

### 2. The selected owner is present only as partial method excerpts

The prompt selects `swarm_provider_host_set::ProviderHostExecutionSession` as
the finite semantic owner. The bundle omits its complete current lifecycle:

```text
crates/swarm-provider-host-set/src/provider_host_set.rs:42-54
    ProviderHostExecutionSession and its retaining admission refusal
crates/swarm-provider-host-set/src/provider_host_set.rs:267-276
    ProviderHostSet::begin_provider_execution_session_v1
crates/swarm-provider-host-set/src/provider_host_set.rs:839-1038
    complete owner impl, external branch invocation, and Drop shutdown
crates/swarm-provider-host-set/src/lib.rs:1-22
    module privacy and public reexports
crates/durable-native-provider-loader/src/lib.rs:372-374
    current execution-session mint/caller
crates/durable-native-provider-loader/Cargo.toml
    constructor-side dependency direction
```

The current bundle emits `provider_host_set.rs` lines `1-30`, `650-730`,
`870-940`, and `980-1030`. Those spans exclude the owner struct, its constructor,
the start of its impl, and `Drop`. The root reexport and the loader mint are not
in the adjacent snapshot set at all. A repository-wide search for
`ProviderHostExecutionSession|begin_provider_execution_session_v1` finds those
definitions and callers in addition to the excerpted consumers.

The selected owner must retain `BunProviderBackend`, branded invocation cargo,
reservation/dispatch authority, and shutdown custody. A review attachment that
omits the owner's fields, mint, public boundary, and current destructor cannot
authorize that move.

### 3. The two drive consumers are detached from their typed custody algebra

Correction 4 includes the process-child consumer at
`process_child_lifecycle.rs:1586-1940`, but it omits the products that own every
failure branch:

```text
process_child_lifecycle.rs:91-274
    DirectRunProcessChildProviderTransitionV1
    DirectRunProcessChildProviderFaultPhaseV1
    DirectRunProcessChildProviderFaultV1
    DirectRunProcessChildProviderRefusalV1
process_child_lifecycle.rs:399-428
    DirectRunProcessChildOwnedRefusalV1
    DirectRunProcessChildDriveRefusalV1
    DirectRunProcessChildDriveOutcomeV1
```

The omitted `DirectRunProcessChildProviderFaultV1` variants retain the complete
session frame across `HostAdmission`, `HostExecution`, `HostResultAdmission`,
and `ProviderDriveCommit`. These are the current custody facts that a replacement
typed refusal/fault must preserve.

The second direct-run consumer uses `DirectRunProcessSessionDriveFaultV1`, whose
definition, cancellation operation, and module boundary live in the entirely
omitted file:

```text
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs:1-123
```

The bundle also omits `ProviderDriveSessionExecutionCommitFault` at
`provider_drive_result.rs:54-88` and the `CapabilitySdkError` algebra at
`swarm-capability-model/src/lib.rs:71-227`; it emits only lines `1-30` of the
latter. Both concrete drive-to-final-output carriers are missing too:

```text
process_child_lifecycle.rs:1387-1410
    commit_process_child_provider_drive_result_for_process_kernel_owner_v1
live_process_session_registry.rs:869-902
    apply_provider_drive_ready_result_for_live_process_session
provider_drive_result.rs:491-519
    commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1
```

The current live-registry excerpt ends at line `831`, before its final-output
carrier. Therefore the two consumer bodies are not joined to the typed host,
result-admission, final-commit, cancellation, and shutdown terminals required by
the prompt.

### 4. The prompt overstates the attached hostile-test evidence

The prompt says the 24-file closure includes “static-host mismatch/replay
tests.” The static-host excerpt is `lib_parts/tests.rs:400-530`; its three
complete boundary tests cover nominal accepted, rejected, and authored cargo.
Repository-wide search finds no static-host replay test and no static-host
cross-boundary mismatch test at the bound SHA.

The complete capability-model file does contain useful hostile evidence:

- `cross_boundary_swap_is_a_typed_fault`;
- `cross_boundary_swap_preserves_typed_settlement_refusal`; and
- `nominal_join_preserves_both_halves_on_mismatch`.

Those are capability-model mismatch/custody-preservation tests, not static-host
mismatch/replay tests. The corrected prompt must state that distinction and
continue to require new static-host and retained-libbun replay tests in the
implementation deliverable.

## Preserved owner ruling

The correction-3 owner ruling remains the clean replacement. Preserve opaque
by-value `libbun::BunProviderBackend` as the highest runtime product. Move the
external-domain prepared-export operation into the finite
`swarm_provider_host_set::ProviderHostExecutionSession` owner so one occurrence
brand covers:

```text
ManifestResolvedExternalProviderCallAdmission
+ ProviderValue
+ SelectedProviderBoundaryOutputAuthority
+ BunProviderBackend
-> ProviderInvocation<Brand>
-> PreparedExport::drive under private reservation/drive custody
-> SelectedProviderBoundaryExecutionResultForProviderHostOwner
   | typed custody-retaining refusal/fault/retirement/shutdown terminal
```

Extend or atomically replace the existing
`mint_provider_boundary_output_correspondence_v1` seal. Do not add a parallel
package/invocation seal beside the pending/selected output seal. Keep
`PreparedExport::drive` as the sole mechanical drive, preserve W1-10
`ProviderValue` as the sole by-value input cargo, and delete the callback proof,
raw splitters, route match aperture, adapter-source reconstruction, compatibility
callers, caller-minted receipts, and backend-husk terminals in the same migration.

## Exact correction-5 bundle patch

The largest valid evidence-only patch is bounded and mechanical:

1. Preserve the current 24-path lexical discovery as a regression gate, but
   rename it as the 24-path lexical hit set rather than a complete SCC.
2. Add repository-wide searches for the actual correspondence/fault and owner
   families, including at least:

   ```text
   ProviderBoundaryOutputCorrespondenceFault
   consume_corresponded_ready_output_for_provider_boundary_owner_v1
   ProviderBoundaryIngressFault
   DirectRunProcessChildProviderFaultV1
   DirectRunProcessSessionDriveFaultV1
   ProviderDriveSessionExecutionCommitFault
   ProviderHostExecutionSession
   begin_provider_execution_session_v1
   cross_boundary_swap_is_a_typed_fault
   nominal_join_preserves_both_halves_on_mismatch
   ```

3. Add exact-SHA snapshots and Cargo/privacy search coverage for:

   ```text
   crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs
   crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs
   crates/swarm-provider-host-set/src/lib.rs
   crates/durable-native-provider-loader/src/lib.rs
   crates/durable-native-provider-loader/Cargo.toml
   ```

4. Expand `GENERATIVE_EXCERPT_SPANS` with complete owning items:

   ```text
   boundary_and_work_selection.rs:148-197
   errors.rs:215-238
   provider_drive_result.rs:45-119 and 491-519
   process_child_lifecycle.rs:85-445, 1387-1410, and 1586-1940
   live_process_session_registry.rs:822-902
   public_aperture_drive.rs:16-530
   direct_run/runtime_authority/mod.rs:1-123
   swarm-capability-model/src/lib.rs:71-227
   provider_host_set.rs:42-54, 267-276, and 839-1038
   swarm-provider-host-set/src/lib.rs:1-22
   durable-native-provider-loader/src/lib.rs:340-374
   ```

   Keep the existing complete correspondence mint/join file, selected request
   carriers, event consumer, static-host sibling consumer, and capability-model
   hostile tests.

5. Replace global `required_terms` with per-path required items. At minimum the
   generator and verifier must bind these exact source pairs:

   ```text
   boundary_and_work_selection.rs
     -> consume_corresponded_ready_output_for_provider_boundary_owner_v1
   errors.rs
     -> ProviderBoundaryIngressFault::OutputCorrespondence source
   process_child_lifecycle.rs
     -> DirectRunProcessChildProviderFaultV1 + both host invocation branches
   direct_run/runtime_authority/mod.rs
     -> DirectRunProcessSessionDriveFaultV1 + cancellation operation
   provider_host_set.rs
     -> owner struct + constructor + selected invoke + Drop
   swarm-provider-host-set/src/lib.rs
     -> owner reexport
   provider_boundary_correspondence.rs
     -> mint + join + typed mismatch + hostile mismatch/preservation tests
   lib_parts/host_set.rs
     -> static-host selected request consumer
   lib_parts/tests.rs
     -> nominal accepted/rejected/authored correspondence tests
   ```

6. Make verifier deletion checks remove a complete required source section or
   its per-path item binding. Do not let an inventory row, introduction, prior
   verdict, or prompt string satisfy a source marker.
7. Correct the prompt to say “capability-model mismatch and custody-preservation
   tests plus static-host nominal correspondence tests.” Keep new mismatched
   brand/output-half, replay, refusal/retry, cancellation/unwind, Drop, shutdown,
   and retained-host replacement tests as mandatory implementation output.
8. Regenerate the adjacent index, compact bundle, search reports, all affected
   file plans/manifests/Fable plans/dry-runs, and synthesis bindings. Keep Oracle
   and Fable `NOT LAUNCHED`, remain below 272000 tokens, and replay from another
   clean Lane.

## Required correction evidence

Run and record:

```text
LIBBUN_REPO=<correction-5-libbun-checkout> \
SWARM_REPO=<checkout-containing-95323ff17cb29928e31467f651ef03bae2099c14> \
python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check

SWARM_REPO=<checkout-containing-95323ff17cb29928e31467f651ef03bae2099c14> \
python3 scripts/verify-libbun-w1112-review-bundle-20260724.py

git -C <swarm-checkout> grep -n -E \
  'ProviderBoundaryOutputCorrespondenceFault|consume_corresponded_ready_output_for_provider_boundary_owner_v1|ProviderBoundaryIngressFault|DirectRunProcessChildProviderFaultV1|DirectRunProcessSessionDriveFaultV1|ProviderDriveSessionExecutionCommitFault|ProviderHostExecutionSession|begin_provider_execution_session_v1' \
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
```

Require literal exit 0 for both deterministic checks, exact per-path source-item
bindings for every item above, zero product/test/Cargo/vendor/workflow delta,
and a fresh independent owner/generative verdict.

## Observed commands

```text
LIBBUN_REPO=/home/ubuntu/bridge-ops/dev-worktrees/libbun-c4-owner-review-20260724 \
SWARM_REPO=/home/ubuntu/swarm \
python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0; eight generated reports matched

SWARM_REPO=/home/ubuntu/swarm \
python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0; independent clean-checkout replay passed;
owner-generative: 32 files; 173462 tokens

git -C /home/ubuntu/swarm grep -l -E \
  'DurableExternalProviderInvocationAuthority|SelectedProviderResumeHostInputForDirectRunOwnerV1|SelectedProviderBoundaryHostRequest|SelectedProviderBoundaryExecutionResultForProviderHostOwner|mint_provider_boundary_output_correspondence_v1|PendingProviderBoundaryOutputCommitAuthority|invoke_selected_provider_boundary_request_for_direct_run_owner_v1|admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1' \
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0; 24 paths
```

Oracle and Fable remain `NOT LAUNCHED`. This review changes only this verdict
record; no product, test, fixture, Cargo, vendor, workflow, release source,
manifest, prompt, dry-run, generator, or verifier is modified.
