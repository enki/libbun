# Wave-0/Wave-1 Semantic Closure And Oracle/Fable Decision Index

This is the canonical Oracle/Fable decision index for Wave 0 and Wave 1. A
superseded row identity is historical provenance only and is not independently
claimable or schedulable.

This reconciliation is source-grounded at
`b84269fa9230bf540d25352b815e3b8f51c89142`. Structural ownership splits
preserve semantics but do not advance a semantic closure row by themselves.
Actor W1-01 has a bounded combined-family refusal-custody checkpoint composed
through `4045bb048926e659008a0c0cf50ad8b22a6ac7c6`; independent review rejects
only its lossy generic diagnostic and the correction is active. W1-02/W1-03
remain unlanded. Provider-lineage and protocol-C0 Oracle correction rounds
remain evidence until reconciled here; an individual response does not
silently supersede a frozen row.

| order | wave | family | exact owner | sealed product/operation | direct dependency row ids | status | law link | source commit | documentation commit | Oracle/Fable session ids |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| W0-01 | Wave 0 | compiler transaction root production | private `SourceEntrypointCompilerTransactionV1` | closed `PreparedRoot` forest plus branded `CompilerTransactionTripleSuccessProofV1` into one infallible registry commit, compiler-host finish, and one-shot atomic-manifest cache OBS | — | reconciled contract frozen; source implementation WAIT | [law](working/SOURCE_ENTRYPOINT_COMPILER_TRANSACTION_ROOT_LAW.md) | reviewed candidate `c5b8ebffc8ac39b177b3e2018fc1d25e97df54a3` is REVISE | this freeze | `resp_080ba73c47ce6f81006a624e7573f481998e0cfbd25a2508d1`; `resp_080ba73c47ce6f81006a625302fbf8819984e4d07c73934d86`; `3f59c42e-20a7-4aed-a395-e66327027623` |
| W0-02 | Wave 0 | package A — authored occurrence custody | `SourceEntrypointCompilerAdmissionSession` | `SourceModuleAuthoredImportOccurrenceBatchForBoundImportClosureOwnerV1` | W0-01 | A done | [law](SOURCE_ENTRYPOINT_PACKAGE_RESOLUTION_LAW.md) | — | `8ae7c1ed47506e93ee8dfdc67f419a70c38179d9` | `package-import-occurrence-family-a`; `6056491e-74e2-4114-9a00-0acdb47de1bb` |
| W0-03 | Wave 0 | package B — resolution and role staging | `ss-runtime-source-compiler-owner::program_assembly::bound_import_closure_admission` | `SourceEntrypointRoleValidatedImportClosurePreparedForCompilerOwnerV1` | W0-02 | B done | [law](SOURCE_ENTRYPOINT_PACKAGE_RESOLUTION_LAW.md) | — | `8ae7c1ed47506e93ee8dfdc67f419a70c38179d9` | `package-import-resolution-staging-b`; `6056491e-74e2-4114-9a00-0acdb47de1bb` |
| W0-04 | Wave 0 | package C — atomic close | `ss-runtime-source-compiler-owner::program_assembly` | infallible one-shot `SourceEntrypointBoundImportClosureReceipt` seal | W0-03 | Oracle C completed; Fable running | [law](SOURCE_ENTRYPOINT_PACKAGE_RESOLUTION_LAW.md) | — | `8ae7c1ed47506e93ee8dfdc67f419a70c38179d9` | `package-import-atomic-close-c`; `6056491e-74e2-4114-9a00-0acdb47de1bb` |
| W0-05 | Wave 0 | installed exact-operation cohort | `ss-runtime-source-compiler-owner::installed_capability_implementation_owner` | complete exact-operation cohort consumption into one installed capability implementation | W0-04 | PASS; source `5849c73d6` | [law](PROVIDER_EXECUTION_AND_SDK_LAW.md) | `5849c73d60f4e130152372a9990ea7722c772a6f` | `76228fd2eb85339d61d6e171bca8a1d7fb70972e` | `installed-operation-owner-verdict`; `51341816-b33b-4c6c-bf82-a5a37c5c00b1` |
| W0-06 | Wave 0 | exact provider occurrence image-open close | executable-image owner in `plan::image_open` | `CompilerCommittedExactProviderOccurrenceFinalizationV1` | W0-05 | editor `364bb454b`; review active | [law](PROVIDER_EXECUTION_AND_SDK_LAW.md) | `364bb454b31e68691b476c2ca020717c0774c9ac` | `76228fd2eb85339d61d6e171bca8a1d7fb70972e` | `installed-operation-owner-verdict`; `51341816-b33b-4c6c-bf82-a5a37c5c00b1` |
| W1-01 | Wave 1 | actor A — exact canonical handler/member provenance | `PreparedSyntaxModuleArtifactSiblingProductsForCheckerAuthoredLoweringOwnerV1::install_parser_authored_syntax_for_checker_authored_lowering_owner_v1`, with first source edit at `module_scoped_actor_definition_attachment_algebra.rs:548`, then type-scope and exact-member custody | consuming `SelectedCombinedAttachmentFamily -> PreparedCombinedAttachmentFamily -> CommittedCombinedAttachmentFamily`; each reusable generative arm contract contains the existing canonical actor-handler source-body identity; exact occurrence-owned `CheckedNativeActorInteraction::{Request, Delivery, Close}`; static Drop retains `Abandoned` for typed owner-close cancellation | W0-04 | **GO frozen corrected contract; implementation unlanded. `e5e6726936de78bfdc37cf7223dac8f4f2c42114` and `6b03daef6f55d9af45e7e68f012920ed8d712ffc` REJECTED AND SUPERSEDED; `6b03` is not composition-eligible. Oracle pre-seal/fresh-identity and Fable borrowed per-row issuance proposals are explicitly superseded by whole-family consuming preparation/commit.** | [law](SWARM_ACTOR_ISOLATION.md) | `e5e6726936de78bfdc37cf7223dac8f4f2c42114` rejected; `6b03daef6f55d9af45e7e68f012920ed8d712ffc` rejected | `87805207ddcf3cb6b2e7dec7830d1d05f0e83ce1`; corrected correspondence law in this revision | `actor-compiler-provenance-cut`; Oracle `actor-member-complete-scc-v2` / `resp_01a446b861320c91006a625046b19c8199b1eecbe8cfe87bcb`; Fable `actor-member-static-arbitration-v2` / `9b2930ac-b56b-40fa-8bad-637b83e94544`; exact-source reconciliation `actor-handler-correspondence-oracle-20260723` + `actor-handler-correspondence-fable-20260723` |
| W1-02 | Wave 1 | actor B — sealed image/cell installation | executable-image finalization plus `KernelTransitionOwner::CommitActorStart` at `commit_actor_start` | nominal checked-arm dispatch carrying the committed canonical handler correspondence into a private image/Session-LIR-bound target, then `InstallActorCell -> InstalledActorCellCommitReceipt \| InstallActorCellTeardownReceipt` | W0-06; W1-01 | **GO frozen contract; waits on corrected W1-01 whole-family implementation, not rejected `e5e672` or `6b03`** | [law](SWARM_ACTOR_ISOLATION.md) | — | `87805207ddcf3cb6b2e7dec7830d1d05f0e83ce1`; corrected correspondence law in this revision | `actor-install-cell-cut`; Oracle `actor-member-complete-scc-v2`; Fable `9b2930ac-b56b-40fa-8bad-637b83e94544` |
| W1-03 | Wave 1 | actor C — mixed FIFO, complete turn, and lifecycle | `ProcessSessionV0` / private `ProcessSessionActorTurnMachineV1` | one issue-sequence mixed-arm FIFO; cell-owned Selected custody; `SelectedActorTurn -> ActorTurnReceipt \| ActorTurnFault`; atomic Close reply/Stop/tail drain | W1-02 | **GO frozen contract; implementation unlanded and ordered after W1-02** | [law](SWARM_ACTOR_ISOLATION.md) | — | `87805207ddcf3cb6b2e7dec7830d1d05f0e83ce1` | `actor-mailbox-lifecycle-cut`; Oracle `actor-member-complete-scc-v2` / `resp_01a446b861320c91006a625046b19c8199b1eecbe8cfe87bcb`; Fable `actor-member-static-arbitration-v2` / `9b2930ac-b56b-40fa-8bad-637b83e94544` focused correction |
| W1-04 | Wave 1 | protocol A — placement-bound checked occurrence | direct checked-protocol settlement producer in `protocol_declaration_settlement.rs` | complete move-only placement-bound checked protocol occurrence retaining installed obligation, exact declaration, callable occurrences, checked body, and typed refusal/destruction custody | W0-04 | **GO — corrected contract frozen; implementation incomplete; repaired as direct fallout of the W1-05 C0 wall** | [law](PROTOCOLS.md#compiler-c0-placement-bound-installed-definition) | — | this revision | Oracle `protocol-owner-conflict-correction` / `resp_07c72f6559696564006a6268d92c1c8199873c4347f7d8d700`; Fable max `794dcd91-156a-443a-ac61-e9b215603e22` |
| W1-05 | Wave 1 | protocol B — installed definition/image close | `protocol_declaration_authority` table owner through `CompilerProtocolSettlementTransaction`, then image installation owner | C0 consumes the W1-04 product into one generative sealed table-owned `InstalledProtocolDefinition` while atomically deleting the occurrence-free table, content-key fallback, action-witness producer, and event-stream reconstruction; sealed installed identity plus table-owned ordinal then closes into the image with no name/content/alias rematch | W1-04 | **GO — corrected contract frozen; implementation incomplete; C0 is the first compiler source wall and W1-04 is its intentional direct fallout** | [law](PROTOCOLS.md#compiler-c0-placement-bound-installed-definition) | — | this revision | Oracle `protocol-owner-conflict-correction` / `resp_07c72f6559696564006a6268d92c1c8199873c4347f7d8d700`; Fable max `794dcd91-156a-443a-ac61-e9b215603e22` |
| W1-06 | Wave 1 | protocol C — registered session lifecycle/full SCC | `DirectRunRuntimeAuthorityOwner`, with nine registered-slot transitions paired one-to-one with nine private consuming `ProcessSessionV0` operations, plus the owned child-tree unwind operation | private sealed `DirectRunLeasedRegisteredProcessSessionAttemptV1<F>`; `Resident \| Leased \| Refused \| Recovery \| Terminal`; one iterative full-session unwind settlement reached from exactly three custody-tier catch sites; typed shutdown/checkpoint/terminal receipts | W1-05 | **GO — corrected contract frozen; implementation incomplete and ordered after W1-05** | [law](PROTOCOLS.md#runtime-highest-owner-and-sealed-family-correspondence) | — | this revision | Oracle `protocol-owner-conflict-correction` / `resp_07c72f6559696564006a6268d92c1c8199873c4347f7d8d700`; Fable max `794dcd91-156a-443a-ac61-e9b215603e22` |
| W1-07 | Wave 1 | primitive `Operation<Event, Output>` whole-body custody | current checked operation-body owner through its sole positional image-emission consumer | `CheckedOperationBodyClosure = NotOperationBody \| Operation(SealedOperationBodyEmissionPlan)` privately retains checked attachment, lowering, terminal custody, retry/cancel/Drop teardown, finish check, and one infallible image commit | W0-04 | independently PASSed; awaiting implementation; W1-08/W1-09 superseded | [law](OPERATION_EXECUTION_DESIGN.md#canonical-whole-body-custody) | base `6d130cd3a0c890347a259a77b1204e63ad00abc4`; rejected `d6fab2e8ad461737bdaa0c1f6338e8caacf4f729`, `af1dc5312334d0648cf2295d94d00c83660b09ad` | — | Fable max `33204760`; Oracle `primitive-yield-core-arbitratio-v2`; response `resp_06ef5e54f177410c006a624f7c8724819a9d4de8c05456802d` |
| W1-10 | Wave 1 | ProviderValue JSON V1 | `swarm-provider-value-model` | fail-closed canonical ProviderValue JSON V1 encode/decode | W0-05 | source `8a4773512`; law corrected through `cd564b9fc` | [law](PROVIDER_VALUE_JSON_WIRE_V1.md) | `8a4773512daf5636ee5aab81ec53df50c150de4c` | `cd564b9fcc21e8b8eec507f009fe5537c0585362` | `provider-wire-law-correction` |
| W1-11 | Wave 1 | libbun A — mechanical prepared export drive | libbun `Quiescent Prepared Export Drive` owner | `PreparedExport -> MechanicalTerminal<UntrustedCargo \| CancelObserved \| DeadlineExpired \| Fault>` | W1-10 | Oracle/Fable merged; existing libbun backend mechanical work is current; no general TypeScript SDK or Python/Go/WASM host implementation | [law](PROVIDER_EXECUTION_AND_SDK_LAW.md) | — | `76228fd2eb85339d61d6e171bca8a1d7fb70972e` | `libbun-mechanical-api-cut`; `e152ae21-4fd7-4095-ae8a-19576c48fb57` |
| W1-12 | Wave 1 | libbun B — interrupt and quiescence | libbun retained-backend `BunProviderBackend` lifecycle owner | affine `AdmittedDrive + InterruptHandle -> MechanicalOutcome` after quiescence | W1-11 | Oracle/Fable merged; existing libbun backend lifecycle work is current; no general TypeScript SDK or Python/Go/WASM host implementation | [law](PROVIDER_EXECUTION_AND_SDK_LAW.md) | — | `76228fd2eb85339d61d6e171bca8a1d7fb70972e` | `libbun-interrupt-quiescence-cut`; `e152ae21-4fd7-4095-ae8a-19576c48fb57` |
| W1-13 | Wave 1 | libbun C — Swarm semantic settlement | `ss-runtime-source-compiler-owner::installed_capability_implementation_owner` | selected invocation plus mechanical outcome into exact typed settlement | W0-05; W1-10; W1-12 | Oracle/Fable merged; current existing-libbun settlement waits on full W0-05; no general TypeScript SDK or Python/Go/WASM host implementation | [law](PROVIDER_EXECUTION_AND_SDK_LAW.md) | — | `76228fd2eb85339d61d6e171bca8a1d7fb70972e` | `swarm-libbun-owner-migration-cut`; `e152ae21-4fd7-4095-ae8a-19576c48fb57` |
| W1-14 | Wave 1 | executable artifact A — compiler product/publication | semantic compiler-product owner through `DirectRunPreparedRuntimeAuthorityOwner::install_compiler_prepared_source_program_image_v1` | one deterministic complete serialized closure into canonical Merkle envelopes, clean compiler-custody close, atomic publication, read-back owner admission, and one sealed admitted complete-artifact receipt; no compiler-worker authority | W0-01; W0-05; W1-02; W1-05; W1-07; W1-13 | reviewed hard law; implementation unlanded at `b84269fa9230bf540d25352b815e3b8f51c89142` | [law](EXECUTABLE_ARTIFACT_LIFECYCLE_DESIGN.md) | — | this reconciliation | `executable-artifact-worker-cutover` |
| W1-15 | Wave 1 | executable artifact B — fresh execution worker | runtime launch owner | sealed admitted complete-artifact receipt plus launch-environment authority into one fresh execution-worker launch; no source/path/cache/compiler input, compilation, rederivation, or remint | W1-14 | reviewed hard law; implementation unlanded at `b84269fa9230bf540d25352b815e3b8f51c89142` | [law](EXECUTABLE_ARTIFACT_LIFECYCLE_DESIGN.md) | — | this reconciliation | `executable-artifact-worker-cutover` |

Primitive supersession: old independently schedulable W1-08 lowering custody
and W1-09 terminal emission are subsumed by unified W1-07. Their phase
obligations remain mandatory inside `CheckedOperationBodyClosure`; their row
identities and direct dependency edges are superseded. W1-14 therefore depends
directly on unified W1-07.

Installed-capability supersession: W0-05 is one ordered SCC, not three parallel
roles. Phase A moves
`ProviderLaneBindingForRouteResolutionOwnerV1.selected_implementation_target`
by value immediately before the production `target: Arc::new(target)` at
canonical line 2510 in `static_capability_route_resolution.rs`; this is the
exact first source/code edit. No test, fixture, model, stale caller, or cohort
eliminator precedes it; external negative-construction and retaining package
fault proof follows immediately after A.
Phase B may begin only from A's retaining package-installation transaction and
must close generative occurrence/`ImportId`, complete child-forest, callable,
and root/worker drain custody. Phase C may begin only from B's complete private
precommit product and performs the sole zero-work commit after
`executable_image/projection.rs:1359-1365` and before the current line 1366.
W0-06 is therefore not claimable independently. No phase permits TLS, string
faults, empty operations, DTO/parts/raw `ImportId`, precommit
`Arc`/clone/`OnceLock`, fallback/default/`None`, separately selectable
family/site authority, or a second commit.

### Frozen compiler-root production contract — 2026-07-23

This block is the byte-identical cross-document contract freeze. A later
document may supersede it only by naming a new reviewed owner contract and
explicitly replacing this block.

1. **Highest owner.** One private
   `SourceEntrypointCompilerTransactionV1` is the highest semantic owner. It
   owns one generative brand; complete selected plan, admitted configuration,
   and compiler-only provider-host custody; custody epoch, scope, and families;
   package/import and compiler phase products; every root/static-child forest
   frame and permit; prepared-runtime custody; typed faults; and unpublished
   cache observation work. No parallel root, forest carrier, or lower owner may
   hold part of that invariant.
2. **Complete selected ingress.** Ordinary source-entrypoint and executable
   `ss test` each use the same sole private constructor at their real selected
   ingress, before `CustodyEpoch::new`, `scope()`, either `family()`, or any
   other fallible phase. Each lane supplies one closed selected-input variant
   containing its complete plan/config/provider-host cargo. The ss-test ingress
   is
   `ss_source_work_set_mint_selected_prepared_test_file_product_for_ss_test_execution_owner_v1`,
   extended to consume the complete lane input; no root may be minted after an
   executable front pass.
3. **Total consuming phases.** Every semantic phase consumes the transaction
   and returns the next transaction or an existing typed fault that owns the
   complete transaction at the exact phase. To retain custody across arbitrary
   unwind, fallible work is a borrow-only preflight while the transaction
   remains owned, followed by a mechanically infallible, non-panicking by-value
   commit. A `catch_unwind` around a consuming call, `Option::take`, sentinel,
   placeholder, or injection before the hazardous consume is not proof.
4. **Closed forest.** The transaction owns one closed forest algebra ending
   only in `PreparedRoot`. Its states cover root selection/staging, active work,
   child staging, spawn, acceptance, retaining refusal/cancellation, and
   `PreparedRoot { root_permit, authored_order_children_complete,
   prepared_program }`. An awaiting, empty, active, refused, or synthetic
   forest cannot satisfy success. Child staging and acceptance retain the
   parent, child semantic state, authored role, pending permit, return permit,
   and ancestor stack in every refusal or unwind.
5. **Consumptive forest proof.** The only prepared-forest operation consumes
   `PreparedRoot`, verifies exact brand correspondence and an empty ancestor
   stack, and returns the prepared program plus one branded
   `CompilerTransactionForestConsumptionReceiptV1`. It never accepts forest
   emptiness, `Rc::strong_count`, a dropped root permit, or a borrowed proof.
6. **Branded terminal proof.** Exact-image settlement, authored-family
   settlement, and custody-epoch close return sealed move-only success receipts
   from their actual owners. The compiler terminal consumes those receipts with
   the forest-consumption receipt into one branded
   `CompilerTransactionTripleSuccessProofV1`. The proof has no getter, borrowed
   consumer, `Clone`, serde, public variant, or alternate constructor.
7. **One authority commit.** One by-value final-success aggregate owns the
   triple-success proof, preflighted pending image-registry installation,
   complete prepared-runtime custody, compiler-only provider-host set, runtime
   observations, and the complete pending module-cache observation batch. Its
   authority commit is a pure infallible move that publishes all runtime/image
   registries exactly once, explicitly finishes the compiler-only provider-host
   set exactly once, and returns the prepared runtime. No refusal,
   cancellation, unwind, `Drop`, or shutdown before this operation publishes
   runtime authority.
8. **Cache is final OBS.** Module-interface cache work remains non-authoritative
   and outside semantic finalization. The terminal consumes the pending batch
   exactly once after the branded success proof. Immutable artifact and lookup
   writes are inert until one complete program-level active manifest is
   atomically renamed; per-module partial activation is forbidden. Cache I/O,
   collision, serialization, or rename failure becomes a bounded explicit
   observation and cannot refuse, roll back, select, mint, or invalidate the
   already valid prepared runtime. A published observation is a different
   type and cannot re-enter publication.
9. **One ss-test root.** The sole ss-test transaction is threaded privately
   from selected ingress through selected provider requirements and lineage,
   `SourceEntrypointExecutableProduct`, executable-front-pass and checked
   post-provider inputs, every retaining refusal, the forest, pending image,
   cache observation, and final dispatch settlement. There is no late, second,
   optional, child-local, retry, fixture, or compatibility mint.
10. **Typed lifecycle and stack safety.** Nested-child refusal remains typed;
    it is never converted to a string. Retry consumes the retained fault back
    into the same phase. Cancellation, explicit shutdown, panic settlement,
    and ordinary `Drop` use one private iterative heap teardown covering
    frames, permits, pending publication, and nested prepared children; authored
    depth never controls the Rust stack.
11. **Forbidden production shapes.** Delete and keep absent the optional forest
    shuttle, root/forest back-borrow, `selected.take()`, `active.take()`,
    `in_flight.take()`, successful `drop(forest)`, late ss-test mint, zero-field
    or borrowed success marker, repeatable cache observer, per-module partial
    activation, RAW selector or DTO/parts bridge, callback proof, borrowed AUTH,
    fallback/default/empty success, stringified authority fault, and
    compatibility entrypoint.
12. **First source cut.** In
    `admit_source_entrypoint_executable_closure_with_custody_configuration_for_compiler_owner_v1`,
    construct `SourceEntrypointCompilerTransactionV1` from the joined
    `(plan, admitted_config, provider_host_set)` as the first statement and move
    epoch/scope/family acquisition into its consuming custody transition. In
    the same owner-move bucket, extend the exact ss-test selected-ingress
    constructor to mint the identical transaction shape, thread it through the
    closed lane variants, then delete
    `mint_from_ss_test_prepared_front_pass_for_compiler_custody_owner_v1` and
    its late caller. No source implementation begins until this documentation
    freeze is independently reviewed.

Review provenance:

- Oracle initial response:
  `resp_080ba73c47ce6f81006a624e7573f481998e0cfbd25a2508d1`.
- Oracle correction response:
  `resp_080ba73c47ce6f81006a625302fbf8819984e4d07c73934d86`.
- Fable session: `3f59c42e-20a7-4aed-a395-e66327027623`.
- Fable transcript: `/tmp/compiler-root-production-v3-fable.md`, SHA-256
  `a77f3eb726351be28c56b949d375c1cfcf971da5cd1c0af3cf123ec7d3f83bc5`.
