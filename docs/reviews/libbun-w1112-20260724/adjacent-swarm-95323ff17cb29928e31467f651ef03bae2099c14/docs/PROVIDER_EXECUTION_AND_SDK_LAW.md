# Provider Execution And SDK Law

Status: canonical current law for provider installation, execution, settlement,
and language SDK adapters.

This document owns the end-state provider execution architecture. Historical
ADRs, completed repair notes, crate comments, and tests are evidence about the
implementation that existed when they were written. They are not authority for
provider execution when they conflict with this law.

This law explicitly supersedes directions that prescribe any of the following:

- contract-family or package-family executor admission;
- provider-name, package-name, export-name, or host-kind dispatch tables;
- separate runtime route variants for Rust SDK, built-in, static test, loaded
  native, libbun, or another implementation host;
- public provider request/result DTOs, selector getters, or reconstruction from
  manifests, prepared images, JSON, paths, ids, or descriptors;
- a provider callback that receives authority-bearing selectors or returns a
  caller-minted receipt, ticket, proof, or settlement authority; or
- generated bindings, manifests, catalogues, or SDK constants acting as
  contract, selection, installation, or execution authority.

In particular, ADR-2034, ADR-2037, ADR-2046, ADR-2049, and ADR-2050 are
historical implementation evidence, not competing current provider-execution
law. Their exact-contract, TSON, Rust-ownership, and transport-only findings
remain useful where they agree with this document. Their family-list,
host-kind, callback, DTO, or callback-returned result/proof shapes are
superseded.

The package and import side of this law is defined by
[Source Entrypoint Package Resolution And Import Binding Law](SOURCE_ENTRYPOINT_PACKAGE_RESOLUTION_LAW.md).
The contract side is defined by [TSON](TSON.md). This document begins after the
source compiler has retained their sealed products and owns the transition from
provider preparation through terminal runtime settlement.

This law is indexed by the
[SwarmScript Durable Roadmap](SWARMSCRIPT_ROADMAP.md#runtime-and-artifact-roadmap).
That index points here for the decision; older provider ADRs remain historical
evidence rather than alternate execution designs.

## Completed Oracle Verdicts And Current Distance

The repository has useful exact-contract, typed-request, selected-output, and
resource-release pieces, but it does not yet have one host-neutral installed
operation. The provider-install and installed-operation Oracle and Fable
reviews are reconciled at source audit head
`ecbe8f791e122fc8d05a8ebb8639c7522b1de730`. They approve the private owner and
the first compile-coherent source bucket below and reject the previous crate
root. This is a `GO` source verdict; it does not approve an implementation,
establish a completion percentage, or provide compiler or suite evidence.

Present substrate includes:

- `SsPackageUniverseAdmission` and Bun-like package export selection;
- a capability-role validator and compiler-private role-validated route lanes;
- sealed exact checked-operation cohort members and complete nonempty
  pending-execution cohort formation that retains unmatched rows;
- exact checked-call coverage joins, install-admission halves, adapter
  preparation requests, and a selected-provider lineage product;
- Contract-TSON typed request admission;
- selected output correspondence and parts of effect/resource custody;
- Rust static executor installation, loaded-native identity admission, and a
  libbun external execution session; and
- owner-local exact-operation dispatch tests plus complete Mesh cancellation
  teardown custody through router, session, and native transport.

### Provider-install Oracle: approved transition

The first source transition is approved as one compile-coherent bucket. In
order, it adds a private consuming eliminator for
`CompilerCheckedCapabilityExactOperationSpecializationForTypecheckOwnerV1`, a
private consuming eliminator for
`CompilerCheckedCapabilityExactOperationCohortMemberForTypecheckOwnerV1`, and
then consumes one complete exact-operation cohort through one provider census,
Contract-TSON admission, adapter admission, and install decision while
retaining every occurrence, typed refusal, and unmatched row.

The specialization eliminator begins in
`source_work_set/syntax_type_scope_owner/boundary_contract_index/index_model/type_scope_and_modules.rs`.
The cohort-member eliminator begins in
`source_entrypoint_compiler_admission_session/static_capability_route_resolution.rs`.
Only after both eliminators exist may
`source_entrypoint_typecheck_product_owner/provider_call_admission_owner/selected_requirements_and_batch_consumption.rs`
make `consume_into_selected_provider_requirements_for_source_work_set_owner_v1`
consume the existing
`CompilerPendingProviderCommandExecutionExactOperationCohortBatchFormationForSourceWorkSetOwnerV1`
instead of destructuring and scanning each pending row independently. Both
eliminators are private finite consuming owner operations. They are not
getters, callbacks, traits implemented by callers, parts conversions, or raw
specialization/member projections.

### Installed-operation Oracle: approved owner

The correct owner is the private
`ss-runtime-source-compiler-owner::installed_capability_implementation_owner`
module. It owns the complete install, exact-operation selection, invocation,
settlement, storage, and shutdown semantic SCC. Storage in that sentence means
storage of opaque installed operations, parked invocations, live stream state,
deadlines, cancellation custody, and shutdown obligations. No sibling crate
receives their selectors or parts.

The corrected owner-root verdict is now `GO` and records this module plus the
specialization eliminator, cohort-member eliminator, and cohort fold as one
compile-coherent source bucket. Implementation is still pending. No common
owner module, eliminator, cohort consumer, complete host migration,
compiler-green result, or same-SHA acceptance matrix is claimed by this
document update.

The missing common unit remains the installation event that generatively binds
those pieces. Runtime still stores and chooses host families. Built-ins and
`@swarm/test` still carry catalogue/facade vocabularies. Loaded-native and
libbun paths still use separate activation and callback shapes. Current
package-provider discovery still classifies `.ts`, `.js`, and native-manifest
targets through suffix-oriented host-environment paths.

## Highest Semantic Unit

The highest concrete unit is an **Installed Capability Implementation**.

```text
Semantic Abstraction Gate:
- Unit: Installed Capability Implementation
- Selected input: complete exact-operation cohort + ProviderLaneBinding
  + exact admitted Contract-TSON operation + checked provider-effect coverage
  + installed adapter admission
- Receipt/Fault: InstalledCapabilityOperation or typed installation fault that
  retains the complete cohort, unmatched rows, and adapter resources or
  terminally settles every input
- Private phases: package/export decoding, generated binding mechanics, native
  symbol lookup, libbun isolate lookup, input/output encoding, storage, and
  host-specific teardown
- Too low: provider ids, host kinds, executor traits, callbacks, manifests,
  descriptors, family registrations, and route enums
- Too high: a generic workflow/runtime/plugin framework
- First source edit: in one compile-coherent bucket, add the private consuming
  specialization eliminator, then the private consuming cohort-member
  eliminator, then fold the complete cohort in
  `selected_requirements_and_batch_consumption.rs` and let the private
  `installed_capability_implementation_owner` perform one census,
  Contract-TSON admission, and adapter install into one opaque exact-operation
  set
```

The common semantic owner is the private complete checked provider-install
co-location owner named above. The completed topology review supersedes the
earlier assumption that this owner can remain wholly inside
`swarm-provider-host-set/src/capability_invocation.rs`: that crate cannot name
the compiler-private complete provider lineage. The install/select/invoke/
settle/storage/shutdown SCC must move into
`installed_capability_implementation_owner` so one finite operation consumes
the lineage plus installed adapter admission without a public getter, DTO,
callback, facade, or successor carrier. `swarm-provider-host-set` may retain
mechanical adapter code during migration, but it does not remain a semantic
selection or storage owner.

## Canonical Provenance

One provider operation is executable only through this chain:

```text
SsPackageUniverseAdmission
  -> package.json#exports selected under condition "swarm"
  -> SwarmSelectedExportTarget
  -> authored capability-role validator
  -> SealedContractExportRef + ProviderLaneBinding
  -> checked exact-member provider-effect coverage
  -> SourceEntrypointSelectedProviderLineage
  -> provider preparation installs one adapter for the selected implementation
  -> InstalledCapabilityOperation
  -> SelectedProviderInvocation
  -> Ready | Failed | Parked | StreamEvent | Deadline | Cancelled
  -> final runtime observation
```

Every arrow consumes an owner product or returns a typed retaining refusal. No
later phase may recreate an earlier fact from strings, ids, package records,
paths, manifests, generated constants, image sites, or runtime observations.

The installed operation is generative. One installation event binds all of the
following and does not expose them independently:

- the admitted package-universe identity;
- the selected `swarm` export;
- the authored capability import role;
- the sealed contract export;
- the provider implementation selection;
- canonical TSON contract identity and fingerprint;
- the exact admitted operation;
- the checked provider-effect coverage certificate;
- the installed adapter authority; and
- the invocation/output settlement lineage created for that operation.

A matching package name, provider id, export string, fingerprint string, or
operation string is not a substitute for this correspondence.

## Current Symbol Convergence And Divergence

The present source topology converges through these actual symbols:

```text
program_assembly/package_universe_admission.rs
  SsPackageUniverseAdmission
    |
    v
package_universe_admission/validation.rs
  SsPackageUniverseAdmission::select_swarm_export_target_for_source_entrypoint_compiler_owner_v1
    |
    v
package_universe_admission/selection.rs
  ss_select_bare_static_capability_swarm_export_target_for_route_resolution_owner_v1
    |
    v
source_entrypoint_compiler_admission_session/static_capability_route_resolution.rs
  resolve_static_capability_requirement_route_for_source_work_set_owner_v1
  validate_package_selected_static_capability_role_for_route_resolution_owner_v1
  CompilerProviderRouteRoleValidatedLaneForTypecheckOwnerV1
  CompilerCheckedCapabilityExactOperationCohortMemberForTypecheckOwnerV1
    |
    v
provider_call_admission_owner/selected_requirements_and_batch_consumption.rs
  SourceEntrypointBoundProviderRequirementBindingFactForSourceWorkSetOwnerV1
  CompilerPendingProviderCommandExecutionExactOperationCohortBatchForSourceWorkSetOwnerV1
  select_package_contract_provider_coverage_for_source_work_set_owner_v1
  CapabilityImplementationInstallAdmissionForSourceWorkSetOwnerV1
  ProviderAdapterPreparationRequestForSourceWorkSetOwnerV1
  SourceEntrypointSelectedProviderRequirementsForSourceWorkSetOwnerV1
  SourceEntrypointSelectedProviderLineageForSourceWorkSetOwnerV1
```

That path already proves an important invariant: an unused capability import is
not provider execution. The checked call must join the exact coverage target,
selected operation, contract TSON, and pending execution row before the source
work set can mint selected provider lineage.

## Current Scope And Future Compatibility Non-Goals

The current host migration scope is exactly:

- Rust SDK/static implementations;
- loaded native implementations using the current native ABI; and
- the existing libbun backend lifecycle and semantic-settlement path.

Arbitrary renamed third-party packages, built-in virtual packages, and
`@swarm/test` must be indistinguishable after checked operation selection on
each applicable current host. “Built-in” and “test” are provenance and
outer-lifecycle facts, never installed-operation families.

Python, Go, a general TypeScript SDK, and a WASM provider host are future
compatibility only. They are not current implementation goals, migration lanes,
acceptance gates, or ETA inputs. Existing libbun backend work remains current
where this law names its drive, interrupt/quiescence, and semantic-settlement
owners; that work does not schedule a general TypeScript SDK, bindings or
generator project, or cross-host authoring suite. This law requires only that
the common algebra not preclude a later opaque mechanical adapter. It does not
specify WIT, a common host enum, decorators, a Go bridge, or any other
future-host SDK design.

The first current divergences are:

| Provider family                    | First current divergence                                                                                                                                                                                                                                                                                                                                                                                 | Required convergence                                                                                                                                                                          |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arbitrary Rust SDK static provider | `RustSdkStaticProviderHostOwner::admit_exact_contract_native_provider_registration_for_product_environment_owner_v1` independently binds `CapabilityContractIdentity` to `Box<dyn RustSdkStaticProviderExecutor>`; `RustSdkStaticProviderHostAdmissionKind::ContractFamily` additionally routes by package family                                                                                        | consume selected lineage and an adapter admission into `InstalledCapabilityOperation`; Rust becomes an adapter-private implementation                                                         |
| `@swarm/test`                      | `swarmscript-capability-registry/src/static_test_provider_targets.rs` introduces `RUST_SDK_STATIC_TEST_PROVIDER_FACADE_MANIFEST`, `RUST_SDK_STATIC_TEST_PROVIDER_TARGET_MANIFEST`, facade/target/inventory types, followed by `RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner` and `JoinedResolvedStaticCapabilityProviderRootSelectionForAuthorityCarrierOwnerV1::RustSdkStaticTestFacade` | keep declaration collection, case selection, fixture scope, and reporting in the test owner; install checked test operations through the same exact operation product as every other provider |
| Built-in virtual package           | `RustSdkBuiltinProviderCatalogue`, built-in binding inventories, `RustInternalProviderTarget`, `KNOWN_RUST_INTERNAL_PROVIDER_TARGETS`, and `known_rust_sdk_static_provider_target_for_selected_operation` rematch package/export strings after package admission                                                                                                                                         | retain virtual package records but install their implementation adapters through the common operation; no runtime built-in vocabulary                                                         |
| Native extension                   | capability-role validation lawfully classifies the selected provider manifest, then `NativeProviderInstalledHostAdmission`, `LoadedNativeProviderArtifactSet`, and `ProviderHostSet::loaded_native_providers` re-form a separate identity and execution family                                                                                                                                           | native loader yields an adapter admission consumed by the common install; handles and C ABI remain native-adapter-private                                                                     |
| Existing libbun backend            | fixed `./swarm` implementation seed becomes `PackageGraphImplementationDeclaration`; `PackageGraphHostEnvironment::derive_from_provider_implementation_source_for_package_graph_provider_requirements_owner_v1` classifies suffixes and `SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1::ManifestResolvedExternal` chooses an external callback session                                   | role validation yields a libbun adapter admission; provider host invokes it through the same selected invocation and settles its cargo itself; no general TypeScript SDK is implied           |

The former `SsSourceWorkSetProviderLaneBindingForSourceWorkSetOwnerV1`
placeholder is no longer the current seam. The compiler-private
`CompilerProviderRouteRoleValidatedLaneForTypecheckOwnerV1` and sealed
exact-operation cohort products keep route correlation and occurrence custody
inside the typecheck owner. They are still pre-install products: they do not
become runtime authority until the finite common install operation consumes a
complete cohort and one adapter admission.

## Closed Install Algebra

The private semantic shape begins by consuming the two sealed layers that the
current source can only join:

```text
CompilerCheckedCapabilityExactOperationSpecializationForTypecheckOwnerV1
  -> private consuming specialization eliminator
  -> cohort-member owner operation

CompilerCheckedCapabilityExactOperationCohortMemberForTypecheckOwnerV1
  -> private consuming cohort-member eliminator
  -> complete pending-row cohort fold
```

These eliminators do not return field bundles or accept caller-supplied code.
They move the whole sealed input directly into the next finite compiler-owner
operation. The closed cohort fold then is:

```text
CompletePendingProviderRows(first, remaining)
  -> fold_complete_exact_operation_cohort(...)
  -> Formed {
       cohort: NonEmptyExactOperationCohort(first, matching occurrences),
       unmatched: CompletePendingProviderRows
     }
  | ActorReplyRefusal {
       first: CompletePendingProviderRow,
       remaining: CompletePendingProviderRows
     }
  | CohortFormationFault(retained first, retained remaining, typed cause)
```

`unmatched` is not absence and is not discarded work. It is the complete,
authored-order input to the next fold. Every refusal retains the cohort already
formed, the current candidate, all unmatched rows, and any acquired adapter
resource. The fold cannot return `None`, an empty/default cohort, or a partial
row list.

The install transition is:

```text
NonEmptyExactOperationCohort
  + CapabilityImplementationInstallAdmission
  + ProviderAdapterAdmission
  -> install_capability_implementation(...)
  -> InstalledCapabilityOperationSet
       + UnmatchedPendingProviderRows
  | CapabilityImplementationInstallFault(
       retained complete cohort,
       retained unmatched rows,
       retained adapter resources,
       typed cause
    )

InstalledCapabilityOperationSet
  -> consume_exact_operation(checked selected operation)
  -> InstalledCapabilityOperation
  | ExactOperationSelectionFault(retained inputs)
```

`CapabilityImplementationInstallAdmission` is minted only by consuming the
provider-prepared source-entrypoint receipt. It contains the sealed provenance
listed above. `ProviderAdapterAdmission` is minted by the adapter owner after
host-specific loading and validation. Neither product has public fields,
getters, clone, serde, parts conversion, or constructors usable by sibling
crates.

One cohort causes exactly one provider census, one exact Contract-TSON
admission, and one adapter installation. Later occurrences in that cohort reuse
the same installed operation generatively; they do not repeat package/provider
census, TSON parsing or normalization, adapter preparation, registration, or
installation. A cohort fault settles or retains the entire cohort at once.

Installation must be complete before the operation enters the private owner's
installed-operation storage.
There is no installed-but-missing-executor state and no later fallback to
another host. A failed installation returns a typed fault with the selected
implementation and adapter resources still in custody.

The private `installed_capability_implementation_owner` stores a homogeneous
collection of opaque installed operations. It does not have fields named
`rust_sdk`, `loaded_native_providers`,
`loaded_native_link_providers`, `external_transport_capability_provider`, or
equivalent host families. It does not expose provider listings as execution
input.

## Closed Invocation Algebra

The private installed-implementation owner alone creates and drives selected
invocations:

```text
InstalledCapabilityOperation
  + ProviderValue input admitted against the exact TSON operation
  + selected output settlement custody
  + cancellation/effect/resource custody
  -> SelectedInstalledCapabilityInvocation

drive_selected_invocation(SelectedInstalledCapabilityInvocation)
  -> InstalledCapabilityOperationStep
```

The logical step algebra is closed over semantic outcomes, not hosts:

```text
InstalledCapabilityOperationStep =
    Ready(accepted authored cargo, terminal custody receipt)
  | Failed(typed host fault, terminal or retaining custody receipt)
  | Parked(sealed continuation, deadline, cancellation, effect/resource custody)
  | StreamEvent(admitted authored event cargo, same live invocation custody)
  | Deadline(typed deadline settlement, terminal custody receipt)
  | Cancelled(typed cancellation settlement, terminal custody receipt)
```

These names describe the algebra; their concrete Rust representation may be
private structs plus consuming eliminators. A public enum is forbidden if a
sibling crate could match it to select a host, route, continuation, or owner
operation.

`Failed` is a typed host fault and never an authored
`Result<Accepted, Rejected>` value. `StreamEvent` is nonterminal and returns
the same invocation's next sealed state to the owner. `Parked` is not a public
continuation ticket. `Deadline` and `Cancelled` consume the exact live
invocation; neither is empty output or generic re-entry.

`SelectedInstalledCapabilityInvocation` is one-shot and must remain opaque
outside its owner. An installed operation may admit many distinct invocations,
but each selected invocation owns one input and one output settlement
correspondence. It cannot be cloned, borrowed to mint a second invocation,
serialized, replayed, or split into request and settlement parts.

## Adapter Boundary

An adapter performs implementation mechanics. It does not own semantic
selection or settlement.

The common owner may use an internal trait, vtable, FFI function table, isolate
handle, or guest call as mechanical dispatch only when all of these are true:

- the adapter is installed behind one exact operation;
- the common owner calls it, not the reverse;
- its input is already selected and does not expose provider ids, package
  names, export names, host kinds, route variants, output settlement authority,
  continuation authority, or runtime receipts;
- its output is implementation cargo or an adapter-mechanical fault, not a
  receipt/proof/ticket chosen by adapter code;
- the common owner validates output shape and TSON closed sums;
- the common owner decides `Ready`, `Failed`, `Parked`, `StreamEvent`,
  `Deadline`, or `Cancelled`;
- the common owner owns cancellation, retry, teardown, effect draining, and
  terminal settlement; and
- caller code cannot substitute an adapter outcome for another invocation.

The smallest host-independent adapter input is therefore an opaque
`AdapterInvocationInput` borrowed from the selected invocation for the duration
of one drive. The smallest adapter output is opaque `AdapterOutcomeCargo`
correlated internally to that drive. At an FFI, process, or libbun boundary,
bytes and DTOs may transport that cargo, but the receiving Rust owner must
re-admit them immediately. Wire values never become semantic authority.

The current Rust seam is transitional. A borrowed adapter input or cargo view
may temporarily bridge
`SelectedInstalledCapabilityInvocation` to the existing Rust
executor while custody is moved, but it must stay private to
`installed_capability_implementation_owner`, cannot expose selectors or
settlement authority, and cannot become the durable SDK ABI. The final Rust
adapter returns only implementation cargo or a mechanical fault to the common
owner.

## Adapter-Private Matrix

| Adapter           | Mechanics kept private                                                                                                               | Common owner still owns                                                                                                          |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| Rust SDK/static   | implementation object, generated Rust input/output types, internal trait/vtable dispatch, panic containment, `Send`/`Sync` mechanics | exact install correspondence, selected invocation, output/fault admission, cancellation, effects, resources, settlement          |
| Loaded native     | library handle, platform/ABI checks, symbol table, provider context, buffers, admitted return-code normalization, process-fatal violation boundary | exact operation install, activation, selected invocation, output/fault admission, release and terminal settlement     |
| Existing libbun backend | module/isolate handle, JS promise/event-loop mechanics, JS value encoding, import execution, isolate faults                   | provider selection, invocation identity, cancellation policy, output contract validation, effect/resource custody and settlement |

No row introduces a common `AdapterKind`, `HostKind`, `ProviderDomain`, or
language enum. The installed operation already carries the opaque adapter
chosen at installation.

## Current Rust-Host Audit

The audit at `ecbe8f791e122fc8d05a8ebb8639c7522b1de730` finds useful substrate,
not the approved installed-operation owner:

- `RustSdkProviderAdapterAdmissionForProviderHostOwnerV1` prepares one exact
  Rust adapter operation and quarantines it after a caught Rust unwind;
- typed request admission, selected output authority, preflighted authored
  output, effect-drain receipts, and host-resource release transfers preserve
  parts of the required linear custody; and
- loaded-native loading retains the library, admitted bytes, function table,
  and provider context together, validates the required v1 functions, and calls
  `drop_provider` once from its owning drop path.

The hard gaps are structural:

- `RustSdkStaticProviderHostAdmissionKind::ContractFamily`,
  `exact_providers`, and `contract_families` still install and select by a
  Rust-only family model;
- `RustSdkProviderAdapterOperation::duplicate_for_provider_host_owner_v1`
  duplicates implementation operations, while
  `RustSdkProviderAdapterInvocationInput::authored_input_for_rust_sdk_provider_adapter_owner_v1`
  publicly exposes raw `ProviderValue`; neither is the final opaque one-census,
  one-install adapter boundary;
- `ProviderHostSet` still stores `rust_sdk`, `loaded_native_providers`,
  `loaded_native_link_providers`, and
  `external_transport_capability_provider`, then scans contract identity and
  matches `SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1`;
- the Rust execution payload is presently `Ready`-only rather than the complete
  `Ready | Failed | Parked | StreamEvent | Deadline | Cancelled` owner algebra;
- the public external-provider callback accepts invocation authority and
  returns a selected execution result; and
- `ProviderHostExecutionSession::drop` discards a fallible external shutdown
  result. Shutdown must instead be an explicit owner transition; `Drop` may
  perform only an already-staged infallible release.

These findings fix the migration order: first make Rust an opaque mechanical
adapter to the private owner, then migrate loaded native, then migrate libbun.
Do not migrate all three in parallel by adding a family enum or neutral route
DTO; each later adapter consumes the already-frozen common operation.

### Process-fatal native rule

An in-process native adapter cannot turn memory unsafety or lost control flow
into `Failed`. A segmentation fault, abort, foreign unwind across the C ABI,
invalid pointer/length pair that makes memory access unsound, or corruption of
the loader/owner state is process-fatal. The host must not catch it, continue
with the next family, retry the invocation, manufacture a typed settlement, or
claim that shutdown completed.

Ordinary validated ABI return codes, admitted response bytes, and explicitly
reported provider failures may become `Failed` under the common owner. If a
deployment requires crash containment, the native implementation must run
behind a supervised process adapter; process death is then a mechanical
adapter fault that the live Rust owner may settle as `Failed`. This does not
create a native runtime family or change the common algebra.

## Authored Cargo And Typed Host Faults

Authored contract cargo and runtime faults are disjoint.

An authored TSON operation may declare a `Result<Accepted, Rejected>`-shaped
closed sum. In that exact case, the installed-implementation owner validates
the returned closed sum and settles its accepted or rejected authored payload.
A value that merely contains `{ kind: "err" }`, `{ error: ... }`, a string,
JSON object, or similarly shaped fields cannot mint authored Result settlement.

The following are typed host faults by default:

- installation, loading, ABI, symbol, interpreter, isolate, or transport
  failure;
- missing or mismatched exact operation;
- stale contract identity or fingerprint;
- malformed adapter cargo or output type mismatch;
- provider panic, admitted native return-code failure, or libbun rejection
  before authored settlement;
- consumed-input failure;
- cancellation, timeout, retry, resource, effect-drain, teardown, or drop
  failure;
- scheduler, admission, replay, restore, or continuation failure; and
- unexpected adapter termination or lost correspondence.

Such a fault may become authored rejected cargo only when a named language
owner operation explicitly settles that exact fault into the declared authored
contract. A generic provider host, adapter, runner, or SDK cannot perform that
conversion.

## Cancellation, Resource, Effect, Retry, And Drop Custody

Selected work cannot disappear. After selection, every path must consume the
invocation into the next sealed state or return a typed retaining fault.

The installed-implementation owner must stage all fallible work before one
infallible commit:

1. validate the installed operation and exact TSON input;
2. reserve output settlement correspondence;
3. reserve cancellation and retry policy;
4. acquire or verify adapter resources;
5. stage effect-drain and resource-release obligations;
6. invoke the adapter under host-appropriate failure containment;
7. validate adapter cargo completely;
8. settle authored output or typed host fault; and
9. commit continuation/effect/resource state once.

Refusal before commit retains the invocation and every acquired obligation, or
consumes them into a terminal typed fault. Retry consumes a sealed retry
admission bound to the same operation and invocation; no caller can rebuild it
from ids or replay the input.

Cancellation is not an empty output or re-entry hint. It consumes the selected
invocation and produces `Cancelled` or a retaining `Failed` step.

`Parked` carries the exact continuation, effect, cancellation, deadline, and
resource custody needed for the next owner operation. No host-local promise,
callback, thread handle, or JS object is runtime continuation authority.

Rust unwind, Rust panic, libbun rejection, and supervised adapter termination
are caught or normalized at the adapter boundary before they can bypass
settlement. The common owner then drains effects and releases resources under
the same invocation custody. The process-fatal native cases above are not
recoverable steps.

Drop is defense in depth, not the normal settlement path. A droppable guard may
perform only an already-staged infallible release and record bounded fault
provenance. A `Drop` implementation must not ignore a fallible shutdown result,
discard selected work, manufacture a success, or let a host handle outlive the
installed operation. If release can fail, explicit settlement must own it
before drop becomes reachable.

## Generated Binding Law

Generated bindings are implementation support only.

Allowed generated material includes:

- host-language types derived from admitted TSON;
- input/output codecs used inside an adapter;
- an adapter-private method table or closed dispatch algebra;
- compile-time implementation conformance checks; and
- a mechanical implementation object consumed by a finite install operation.

Generated material must not:

- mint `CapabilityContractIdentity` or an installed operation from constants;
- expose public package/export/provider/operation getters used for routing;
- enumerate package or contract families in the common runtime;
- construct provider requests, settlement receipts, continuation tickets, or
  output authority;
- deserialize execution authority;
- select a host or fallback path; or
- let user implementation code return proof that execution occurred.

A public SDK trait is lawful only as implementation dispatch. Its operation
method receives an opaque, already-selected adapter input and returns cargo. It
cannot receive raw selectors or output settlement authority, and it cannot
return a receipt, ticket, selected product, or continuation accepted as proof by
the owner. Generated host-specific dispatch remains private to the adapter or
implementation package.

## Built-Ins And `@swarm/test`

Built-ins are virtual packages, not runtime variants. Their package records and
`swarm` exports enter `SsPackageUniverseAdmission` and the capability-role
validator exactly like ordinary dependency packages. After installation, no
runtime code may learn that an operation came from a built-in package.

`@swarm/test` has lawful test-owner semantics that remain distinct from provider
routing:

- provisioning of the test execution environment;
- collection of `test`, `skip`, and `todo` declarations;
- test-case selection and reporting;
- fixture and body-local lifecycle;
- capture and assertion behavior expressed by exact admitted contract
  operations; and
- final diagnostic formatting.

Those semantics do not justify a static-test provider facade, target inventory,
flat operation list, provider-root enum variant, or direct-run provider target.
Checked test effects install and execute through the same exact operation as an
arbitrarily renamed third-party package. Test declaration planning may recognize
the authored test contract only inside its semantic declaration owner; that
recognition cannot feed provider selection.

Therefore `@swarm/test` is special only at test-environment provisioning and
the outer case/fixture/reporting lifecycle. Through package resolution,
contract admission, provider installation, invocation, and settlement it is an
ordinary capability package.

## Cross-Host Conformance

Every adapter implementation must pass the same semantic conformance suite.
Host-specific tests supplement this suite but cannot replace it.

Required positive proofs:

1. An arbitrary fixture package can be renamed, moved, and given a different
   export spelling while retaining the same admitted TSON contract and adapter;
   it still installs and executes without a catalogue change.
2. An ordinary dependency package and a built-in virtual package take the same
   path after `SwarmSelectedExportTarget` and produce the same installed
   operation shape.
3. Rust static, loaded native, and libbun adapters install through the same
   finite operation and settle equivalent inputs to equivalent authored cargo
   or typed host faults.
4. Exact checked member coverage is required before a provider feed or install
   admission exists.
5. Each complete exact-operation cohort performs one provider census, one
   Contract-TSON admission, and one adapter installation while retaining all
   unmatched rows for the next fold.
6. A selected operation consumes one input and one output settlement
   correspondence exactly once.
7. Authored Result rejection remains authored cargo, while host failure remains
   a typed host fault.
8. Cancellation before invocation, while parked, and during teardown
   preserves effect/resource custody and produces no fallback output.
9. Rust panic, libbun rejection, admitted native error return, and supervised
   adapter-process termination settle through typed faults without leaked
   resources or forged receipts; process-fatal in-process native violations do
   not resume the host.
10. Retry, if admitted, is bound to the same invocation and cannot duplicate its
   selected output authority.
11. `@swarm/test` declaration collection remains test-owner-local while its
    checked provider effects use the common installed operation.

Required negative construction proofs:

- sibling crates cannot construct, clone, deserialize, inspect, or split an
  installed operation or selected invocation;
- SDK implementation code cannot mint a request, route, receipt, settlement,
  continuation, or installed-operation proof;
- no public getter exposes provider/package/export/operation/host selectors for
  later execution;
- no host-family enum can be matched to choose execution;
- no package/family catalogue is required to add or rename a provider;
- no provider feed exists for an unused import or absent checked effect;
- no cohort fold can drop, duplicate, reorder, or rescan an occurrence or an
  unmatched row;
- missing/mismatched operations refuse with typed retaining faults; and
- adapter wire DTOs cannot be re-admitted without the live invocation
  correspondence held by the common owner.

## Hard Cut And Poison List

Delete, privatize, or type-poison the following execution shapes as their owner
cuts land:

- `RustSdkStaticProviderHostAdmissionKind::ContractFamily` and
  `RustSdkStaticProviderContractFamily` as execution admission;
- `RustSdkBuiltinProviderCatalogue` and built-in provider inventories as
  runtime authority;
- `RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner`;
- `RustSdkStaticTestProviderFacadeForRouteResolutionOwnerV1`;
- `RustSdkStaticTestProviderTargetForProviderRouteOwnerV1` and its
  inventory/batch families;
- `JoinedResolvedStaticCapabilityProviderRootSelectionForAuthorityCarrierOwnerV1::RustSdkStaticTestFacade`;
- `RustInternalProviderTarget::{RustSdkStaticCatalogue, RustSdkStaticTest}`;
- `KNOWN_RUST_INTERNAL_PROVIDER_TARGETS` wherever it represents authored or
  built-in provider routing rather than a true VM-internal operation;
- `known_rust_sdk_static_provider_target_for_selected_operation`,
  `matches_authored_facade_binding`, and `matches_selected_operation`;
- `swarm_event_provider_requires_product_session_boundary` and any equivalent
  provider-id branch outside the event product owner;
- the fixed `./swarm` implementation seed and any implementation selection
  derived from a plain-string `exports["./swarm"]` lookup;
- `PackageGraphImplementationDeclaration` when used as executable authority;
- suffix-derived `PackageGraphHostEnvironment` selection;
- `ProviderHostSet` storage split by `rust_sdk`, `loaded_native_providers`,
  `loaded_native_link_providers`, or `external_transport_capability_provider`;
- `SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1`;
- public `RustSdkStaticProviderExecutor::invoke` in its current
  authority-bearing request/context/result form;
- loaded-native contract/provider getters and artifact provider-id listings
  used to rematch execution;
- public external-provider callbacks that accept an invocation authority or
  return selected execution results;
- public raw adapter-input accessors and implementation-operation duplication,
  including the present `authored_input_for_rust_sdk_provider_adapter_owner_v1`
  and `duplicate_for_provider_host_owner_v1` shapes;
- fallible shutdown performed only from `Drop`, including discarded shutdown
  results;
- direct event-kernel, mesh, datastore, process, test, or other subsystem
  bypasses selected by provider/package/export ids;
- public provider contract, provider id, module path, export, host-domain,
  register, or route getters used to mint, choose, or resume execution; and
- DTO/parts/JSON/manifest/prepared-image reconstruction of installation,
  invocation, continuation, or settlement authority.

Poison must preserve true VM-internal operations whose authority is minted by
the VM owner and never represents an authored provider. Name overlap with an
old catalogue is not enough to delete a real internal semantic operation.

## Smallest Disjoint Owner Cuts

The implementation should proceed through these non-overlapping file families:

1. **Package/export and provider-lineage producer.** Under
   `ss-runtime-source-compiler-owner/src/program_assembly/package_universe_admission*`
   and `source_entrypoint_compiler_admission_session/source_work_set*`, finish
   admitted system-package identity, `swarm` export selection, role validation,
   and a sealed provider lineage that replaces the fixed `./swarm` seed.
2. **Common installed-operation owner move.** Create the private
   `ss-runtime-source-compiler-owner::installed_capability_implementation_owner`
   at the checked provider-install co-location boundary. Its first
   compile-coherent bucket adds the consuming specialization eliminator, the
   consuming cohort-member eliminator, and then consumes one complete cohort,
   compiler-private lineage, and one adapter admission through one census and
   Contract-TSON admission. It owns install, exact-operation selection,
   selected invocation, adapter drive, settlement, storage, and shutdown;
   `swarm-provider-host-set` retains no semantic storage or host-family route.
3. **Rust adapter and custody.** In
   `swarm-rust-sdk-static-provider-host/src/lib_parts/{admission_model,host_owner,host_set,native_request_and_executor,request_and_output}.rs`,
   make Rust execution an adapter to the common owner and complete cancellation,
   resource, effect, refusal, unwind, and drop custody.
4. **Built-in and direct-run poison.** In
   `swarm-rust-sdk-static-provider-{listing,inventory,package-graph}` and
   `ss-runtime-source-compiler-owner/src/direct_run/{part_001_rust_internal_provider_targets.rs,runtime_authority/live_persist_receipts.rs,runtime_authority/process_session_public_aperture.rs}`,
   remove catalogue, raw rematch, event-id bypass, and host-family routing after
   the common install product exists.
5. **Test semantic owner.** In
   `swarmscript-capability-registry/src/static_test_provider_targets.rs`,
   `ss-runtime-test-execution-owner`, and test planning files, retain
   declaration/case/fixture semantics while deleting provider-route authority.
6. **Loaded-native adapter.** In `swarm-native-provider-authority`,
   `durable-native-provider-loader`, and
   `swarm-provider-host-set/src/loaded_native.rs`, keep ABI/loading mechanics
   private and yield a common adapter admission.
7. **libbun adapter.** In
   `libswarm-package-graph-provider-requirements`,
   `libswarm-package-graph-model`, and
   `ss-runtime-external-capability-provider-owner`, remove suffix/DTO/callback
   authority and install the libbun adapter from role-validated provider
   lineage.

At `ecbe8f791e122fc8d05a8ebb8639c7522b1de730`, the package-universe,
role-validation, sealed-lineage, Rust adapter hard-cut, and exact-operation
cohort prerequisites cover substantial parts of cuts 1 and 3. Cut 2 is still
open. The reconciled Oracle/Fable owner-root verdict is `GO`, but implementation
of its compile-coherent first bucket remains pending. Cuts 4 through 7 follow
once the common operation makes their old routes compile-time stale.

## Exact Implementation Order

1. Execute phase A at the exact owner boundary as literally the first
   source/code edit. No test, fixture, model, stale-caller, or cohort-eliminator
   edit precedes the line-2510 route-lineage move.
2. Immediately after A, write external compile-fail tests proving sibling
   crates cannot construct, clone, match, unwrap, obtain parts, obtain raw
   `ImportId`, choose a family or site, or hold authority behind a precommit
   `Arc`. Add first/middle/final package-candidate fault injection proving
   prefix, current, suffix, requirements, declarations, contracts, imports,
   and package transaction are retained.
3. Make the cohort producer own root plus at least 127 continuations
   generatively. Prove no raw rematch or cross-cohort splice is possible.
4. Add the finite allocator-to-site-bound-instruction consume. Prove first,
   middle, terminal, exhaustion, exact-once terminal return, and no retry
   replay.
5. Adopt the whole installed-capability transaction and mandatory drain in the
   canonical compiler root/worker transaction. Inject failures at region
   selection, allocation, materialization, site binding, pairing, and
   Session-LIR admission; every failure has zero commits and retains all
   transaction state.
6. Reverse recursive preparation so the complete mixed ordinary/static-child
   forest exists before root runtime construction. Move callable definitions
   and dispatch into that transaction and remove precommit `Arc`, clone,
   `OnceLock`, selector, fallback, and mutable child-slot shapes.
7. Implement phase C at `executable_image/projection.rs:1359-1366`. Instrument
   the sole commit to prove zero allocation, panic, lock, traversal, selection,
   or fallibility, followed by exactly one complete-runtime `Arc`.
8. Route ordinary and static-child repeated open through the same consuming
   algebra. Prove there is no second site installation or commit.
9. Prove retry, cancellation, explicit shutdown, unwind, and ordinary
   root/worker drop drain exactly once under default parallelism. Ordinary
   `Drop` may perform only an already-proven infallible, nonallocating,
   nonpanicking drain; fallible shutdown remains explicit and retaining.
10. Migrate Rust SDK/static, loaded-native, the existing libbun backend,
    built-ins, and `@swarm/test` as mechanical adapters only, then run hostile
    construction, old-shape search, arbitrarily renamed package, and
    current-host conformance gates on one source SHA. Do not schedule a general
    TypeScript SDK from the libbun migration.

Current status at `ecbe8f791e122fc8d05a8ebb8639c7522b1de730`: complete
nonempty exact-operation cohort formation exists, but the selected-requirements
consumer still loops per row; the private common owner does not exist; and
Rust, native, and libbun still use divergent storage and invocation shapes.
This documentation change records no fresh compiler check or suite matrix and
does not claim green.

## Expected Compile Map

The hard cut is expected to break these consumers. The breakage is the migration
map, not a reason to restore a route or selector:

- source compiler static capability route resolution and checked-effect
  finalization;
- source-work-set provider coverage and selected lineage;
- package-graph provider requirements, implementation declarations, and
  prepared-runtime provider imports;
- static-test registry, provider bridge, execution state, and fixture helpers;
- built-in listing/inventory/package-graph crates;
- direct-run `RustInternalProviderTarget` selection and live-persist rematching;
- process-session public-aperture provider drive and the event product-session
  bypass;
- `ProviderHostSet`, execution session, resource release, and external transport
  session;
- mesh, datastore, process, event, and other Rust executor implementations;
- native provider manifest admission, loader, artifact storage, and libswarm
  loaded-native facade drive;
- libbun provider module import and settlement code;
- prepared-runtime image/provider-import carrier and artifact code;
- session-runtime provider resume, cancellation, resource, and effect custody;
  and
- diagnostics/tests that borrow provider, contract, module, export, host, or
  route selectors.

## Completion Evidence

Provider execution is converged only when all of the following are current on
the same tree:

- the arbitrary renamed fixture package passes package admission, typecheck,
  provider preparation, installation, execution, and terminal settlement;
- the built-in, ordinary package, `@swarm/test`, native, and libbun proofs have
  an identical post-selection owner trace;
- every current-host adapter passes the shared positive and negative
  conformance suite;
- no checked provider effect means no provider feed or installed invocation;
- typed faults cover missing/mismatched operations and all adapter failure
  modes;
- cancellation, retry, park, Rust-unwind, resource, effect, teardown, and drop
  proofs retain custody, while process-fatal native violations never resume;
- compile-fail tests prevent raw selector access, authority reconstruction,
  receipt minting, callback-selected proof, and duplicate invocation;
- searches are clean for the poison list outside historical documentation and
  inert final observations; and
- no new provider package, host language, or adapter requires a common-runtime
  enum variant, family list, package-name match, or provider-route edit.
