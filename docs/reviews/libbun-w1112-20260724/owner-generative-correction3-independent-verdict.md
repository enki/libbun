PART BUNDLE REVISE

# W1-11/W1-12 Correction-3 Owner/Generative Independent Verdict

Date: 2026-07-24

## Reviewed identity

- Exact correction-3 candidate: `c2ea016e4c9810fa86ddfd21bd4b30823746a9b9`
- Candidate tree: `67bdbd8830930ed39d19e7f37be092c108de01f7`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Correction-2 base: `51b0118428d7881f39f32df396ef32349a5a52ab`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Lane owner: `libbun-c3-owner-review-20260724`
- Lane worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c3-owner-review-20260724`
- Lane cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-12`
- Review scope: independent source and bundle review only; no Oracle/Fable
  launch and no product, test, fixture, Cargo, vendor, workflow, or package
  source change.

The correction-3 generator and fail-closed verifier both pass from the assigned
Lane. The verifier also completes its differently named clean-checkout replay,
proves zero product delta, validates every attachment identity and order, keeps
all model states `NOT LAUNCHED`, and reports the owner plan at 49 files and
254259 tokens. Correction 3 therefore fixes the Lane-path, W1-10, retained-host
pool, and correction-2 evidence defects it names.

It still cannot receive `PART BUNDLE PASS`. A repository-wide exact-SHA search,
rather than the generator's attachment-limited search, finds an omitted
generative-correspondence producer and the omitted source-compiler carrier and
consumer SCC. Those files own the exact selected-output mint, mismatch fault,
host-input carrier, two real execution consumers, typed fault custody, and
final output commit. Without them the requested owner move and commit-grade
cross-repository patch cannot be reviewed.

## Determining omission

### 1. The real generative output-correspondence mint is not attached

The owner plan attaches the W1-10 `ProviderValue`, exact manifest route,
`DurableExternalProviderInvocationAuthority`, external trait callback, and
`SsExternalCapabilityProviderHost`. It does not attach the source that mints
and joins the existing generative selected-output correspondence:

```text
crates/swarm-capability-model/src/provider_boundary_correspondence.rs
crates/swarm-capability-model/src/lib.rs
crates/swarm-capability-model/Cargo.toml
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  work_runtime/work_runtime_stores_impl.rs
```

At the bound Swarm SHA,
`mint_provider_boundary_output_correspondence_v1` creates one shared private
seal and returns the move-only pair
`PendingProviderBoundaryOutputCommitAuthority` and
`SelectedProviderBoundaryOutputAuthority`. The former stays with the exact
pending session application; the latter moves with the selected host request.
`consume_corresponded_ready_output_for_provider_boundary_owner_v1` is the sole
join and returns the typed
`ProviderBoundaryOutputCorrespondenceFault::ReadyOutputDoesNotMatchPendingBoundary`
on a cross-pair splice.

This existing mint is part of the same generative law as the proposed
`SelectedProviderPackage<Brand>` / `ProviderInvocation<Brand>` pair. An
implementation review must either extend one occurrence brand through this
existing pair or replace the pair in one atomic owner move. It must not create
a second independent brand that can correspond to the selected package while
the pending output still corresponds to a different invocation. The current
prompt and attachments do not expose that obligation to the model.

### 2. The actual selected request carriers and both final consumers are absent

Repository-wide search finds the selected external request entering and
leaving the attached host-set files through the omitted
`ss-runtime-source-compiler-owner` crate:

```text
crates/ss-runtime-source-compiler-owner/Cargo.toml
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  work_runtime/work_runtime_stores_impl.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  work_runtime/work_store/types.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/
  live_process_session_registry.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/
  process_session_public_aperture.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/
  process_session_public_aperture/provider_resume_lifecycle.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/
  process_session_public_aperture/process_child_lifecycle.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/
  process_session_public_aperture/public_aperture_drive.rs
crates/ss-runtime-source-compiler-owner/src/direct_run/event/mod.rs
crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  execution_kernel/executable_image/plan/operation_algebra/
  boundary_and_work_selection.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  execution_kernel/executable_value/process_carriers.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/root.inc.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  scheduler/phase_machine_drive_entrypoints.inc.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  work_runtime/mod.rs
crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/
  work_runtime/work_store.rs
```

`work_runtime_stores_impl.rs` mints and stores the pending/selected
correspondence. `work_store/types.rs` combines the selected half with
`ProviderValue`, exact Contract-TSON, invocation authority, and exact static
child use, then calls
`admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1`.
`process_child_lifecycle.rs` and `public_aperture_drive.rs` are two direct
consumers of
`invoke_selected_provider_boundary_request_for_direct_run_owner_v1`.
`direct_run/event/mod.rs` is the sibling product-session consumer.
`provider_drive_result.rs` and the execution-kernel carriers return the ready
half to the pending session commit.

The source-compiler faults retain the session frame across host admission,
host execution, result admission, and provider-drive commit. These are the
typed custody paths that determine refusal, retry, cancellation, unwind, and
drop behavior when the current callback/request ladder is deleted. The
owner plan instead jumps from the attached host set to the `ss-test` retained
host pool. That pool proves host replacement and final shutdown, but it does
not replace the omitted invocation carrier and final-consumer topology.

### 3. Direct sibling consumers inside the attached SDK crate are also absent

The plan attaches only
`swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs` from
that crate. Repository-wide search finds the public request and result types
also consumed by:

```text
crates/swarm-rust-sdk-static-provider-host/src/lib.rs
crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs
crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs
```

`host_set.rs` is the sibling consumer of the Rust-SDK branch of
`SelectedProviderBoundaryHostRequest`; changing or moving the external branch
without its match and reexports is not an exact-SHA-applicable migration.
`tests.rs` contains the current correspondence mint and mismatch/replay tests
that must be preserved or migrated. These omissions prevent the requested
hostile differently branded, replay, and exact-correspondence evidence from
being designed against all current consumers.

### 4. The advertised source report is attachment-wide, not repository-wide

The generated section named `Adjacent sole consumer, transport, retained-host
pool, and shutdown graph` searches a fixed tuple of attached paths. It does not
search `crates/ss-runtime-source-compiler-owner` or
`crates/swarm-capability-model`. The generated package-direction section also
omits both crates' Cargo manifests. The verifier requires only the same fixed
`ADJACENT_PATHS`, so generator replay and verifier success cannot detect this
omission.

The decisive independent search was:

```text
git -C /home/ubuntu/swarm grep -l -E \
  'DurableExternalProviderInvocationAuthority|SelectedProviderResumeHostInputForDirectRunOwnerV1|SelectedProviderBoundaryHostRequest|SelectedProviderBoundaryExecutionResultForProviderHostOwner|mint_provider_boundary_output_correspondence_v1|PendingProviderBoundaryOutputCommitAuthority|invoke_selected_provider_boundary_request_for_direct_run_owner_v1|admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1' \
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
```

It returns 24 implementation files. The owner plan contains only four of
them. The omitted files include every source-compiler match above, the
capability-model mint and join, and the static-host sibling consumer/tests.

## Highest-owner and generative replacement

The preserved runtime product remains opaque by-value
`libbun::BunProviderBackend`. The finite semantic owner of external provider
selection and execution must be
`swarm_provider_host_set::ProviderHostExecutionSession`: it already owns the
prepared-runtime exact-route admission set, receives the sealed
`ProviderValue` plus selected-output half, chooses the external domain, and
retains the external execution session across calls.

Move the libbun external-domain operation and the external invocation product
into that owner boundary. The operation consumes, without public parts:

```text
ManifestResolvedExternalProviderCallAdmission
+ ProviderValue
+ SelectedProviderBoundaryOutputAuthority
+ BunProviderBackend
-> one generatively branded prepared external invocation
-> PreparedExport::drive under private reservation/drive custody
-> SelectedProviderBoundaryExecutionResultForProviderHostOwner
   or a typed custody-retaining refusal/fault/retirement/shutdown terminal
```

The same occurrence brand must cover the exact selected package/call, the
invocation cargo, and the existing pending/selected output-correspondence
pair. Differently branded package, invocation, or output halves cannot compose.
`ProviderHostExecutionSession` keeps the branch private and returns only the
existing sealed final execution result to the source-compiler commit.

Delete, in the same migration:

- `DurableExternalCapabilityProvider` and
  `DurableExternalCapabilityProviderFactory` callback proof;
- `DurableExternalProviderInvocationAuthority::into_call_input_and_output_settlement_for_durable_external_provider_owner_v1`;
- `ManifestResolvedExternalProviderCallAuthority::into_contract_and_module_for_durable_external_provider_owner_v1`;
- the public `SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1`
  external match aperture;
- adapter-source and raw `ProviderRequest` reconstruction in
  `SsExternalCapabilityProviderHost`;
- dynamic-loading/plugin compatibility callers after the retained owner is
  linked; and
- every caller-minted receipt, selector, parts, or backend-husk terminal.

The `ss-runtime-external-capability-provider-owner` package may supply the
bounded installation configuration during the migration, but it must no
longer implement a caller-controlled execution callback. If moving the
libbun dependency and retained operation into `swarm-provider-host-set` is
rejected as an ownership boundary, the decomposition is invalid: move the
whole `ProviderHostExecutionSession` external-domain owner rather than adding
another bridge.

## Exact source-complete correction

Keep product/test/vendor/workflow source and model state unchanged, then:

1. Add exact-SHA snapshots for every missing file named above, plus
   `crates/swarm-capability-model/Cargo.toml`,
   `crates/swarm-rust-sdk-static-provider-host/src/lib.rs`, and the complete
   direct module/reexport files needed to compile those sources.
2. Extend `ADJACENT_PATHS`, `REQUIRED_ATTACHMENTS["owner-generative"]`, the
   adjacent index, owner file plan, manifest, Fable plan, and synthesis inputs
   to bind them. Require the source-compiler and capability-model Cargo edges.
3. Replace the attachment-limited owner searches with a repository-wide
   exact-SHA search first, then emit compact complete-item reports for the
   discovered producer, mint, carrier, consumer, typed-fault, cancellation,
   unwind, drop, shutdown, and tests closure. A checked fixed path list may
   verify the discovered closure only after the repository-wide search proves
   no additional match.
4. Rebudget the owner plan below 272000 tokens. Prefer full defining files and
   exact complete items with file blob, full-file hash/bytes, exact line span,
   and excerpt hash for very large source-compiler files. Remove broad repeated
   grep output and duplicated roadmap material before omitting a direct
   constructor, carrier, consumer, or typed fault.
5. Amend the owner prompt so the new brand extends or atomically replaces
   `mint_provider_boundary_output_correspondence_v1`; explicitly forbid a
   parallel package/invocation brand beside the existing pending/selected
   output seal.
6. Regenerate all six reports and all four manifest/dry-run/Fable closures,
   then run generator and verifier from another differently named clean Lane.
   Keep Oracle and Fable `NOT LAUNCHED`.
7. Request a fresh independent owner/generative part review only after the
   corrected repository-wide search, exact attachment closure, and token gate
   pass.

## Commands and observed results

```text
LIBBUN_REPO=$PWD SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0; six OK reports

SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0; independent checkout replay passed
owner-generative: 49 files; 254259 tokens
```

Independent libbun source-wide search confirms the poisoned candidate still
has no positive `BunProviderBackend`, brand, offer/reservation proof, or
durable-reaper implementation, and that `PreparedExport::drive` remains the
sole current mechanical drive. Independent vendored Bun/JSC search confirms
the bound termination/reset/deinit source family and the empty
`JSC__VM__deinit` fact; no alternate VM owner repairs the missing Swarm
generative chain. The source-wide Swarm search above is what reveals the
omitted 20 implementation files.

Oracle and Fable remain `NOT LAUNCHED`. No product, test, fixture, Cargo,
vendor, workflow, package source, manifest, prompt, dry-run, or generated
report was modified by this review.
