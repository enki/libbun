# SwarmScript Durable Roadmap

Status: canonical roadmap index.

This document preserves the durable product and language roadmap after the
retirement of
`docs/working/protocol-runtime-substrate-authority-repair-queue.md`. It names
the canonical contract for each remaining area; it is not a claim ledger, a
failure log, or a substitute for current test evidence.

This index is complete at durable-area granularity. Implementing one area may
use temporary parallel lanes, but lane state, retries, dependencies, and source
cuts are disposable execution detail. A lane becomes a roadmap entry only when
it discovers a durable end-state obligation not already owned below.

## Document Roles

The documentation has four distinct jobs:

1. [SwarmScript Primer](SWARMSCRIPT_PRIMER.md) is the concise authored-language
   contract.
2. Top-level `*_DESIGN.md` files and active ADRs define durable end-state laws,
   ownership boundaries, implementation shapes, and conformance obligations.
3. [SwarmScript Syntax Inventory And Conformance Coverage](ADR-2091-SWARMSCRIPT-SYNTAX-INVENTORY-AND-CONFORMANCE-COVERAGE.md)
   owns stable positive and negative coverage obligations, while
   [Negative Test Expectation Integrity](NEGATIVE_TEST_EXPECTATION_INTEGRITY.md)
   owns exact phase/code matching for configured negative suites.
4. Current measured implementation state is established by version-pinned
   source audits and certification evidence. Dated handoffs, branch heads,
   review sessions, compile maps, and pass counts are evidence for their exact
   snapshots only; they never define roadmap status or dependency direction.

[Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md)
owns the Swarm substrate and native-agent specialization for the first-party
SwarmScript agent harness, typed tools, agent profiles, qualification handoff,
reactive virtual agents, and the constraints any optional future
foreign-harness compatibility work must obey. The reusable qualification and
realization-selection lifecycle is owned by
`swarmlib/docs/QUALIFICATION_AND_REALIZATION_SELECTION.md`; Orchid's
cross-repository product sequence is owned by
`orchid/docs/ORCHID_FOUNDATION_MASTERPLAN.md`. This design is a Level-5
consumer specification over the public substrate below, not a Swarm
compiler-internal contract.

[Authored Capability Composition And Attenuation](AUTHORED_CAPABILITY_COMPOSITION_AND_ATTENUATION_DESIGN.md)
owns the generic end-state law for SwarmScript implementations of capability
contracts, capability instances passed locally or over Mesh, semantic
composition/attenuation, delegation/currentness, and typed escalation. It
retains the Primer's current `with capabilities` non-minting law and treats old
capability/provider ADRs as evidence rather than automatically live syntax.

[Capability Host Deployment And Tool Placement](CAPABILITY_HOST_DEPLOYMENT_AND_TOOL_PLACEMENT_DESIGN.md)
owns renderer-neutral capability implementation placement for ordinary
SwarmScript applications and orchestrators. A customer-controlled Rust host may
provide real attenuated capabilities over Mesh without introducing a special
brain mode.
The host may be a headless daemon, embedded process, terminal product,
browser-backed product, Electroswarm/Tauri shell, or another application; none
needs the compiler, VM, session runtime, or native agent harness merely to expose
exact providers. Electroswarm is one instantiation of this generic boundary, not
the semantic application or universal UI architecture.

The Primer is the authored-language guide for agents and people **using**
SwarmScript. Every landed addition, removal, or semantic change to the public
language must update the Primer in the same completion tranche, with the
relevant syntax, types, observable behavior, diagnostics, and concise authored
examples. The Primer must not expose compiler implementation structure, Rust
symbols, private owners or sealed products, work machines, Lane workflow,
repair history, or transient test evidence. Those details belong in design
documents, ADRs, implementation sources, and version-pinned evidence. A
language change is not roadmap-complete while the Primer teaches an obsolete
public surface.

Historical queue rows and repair histories remain provenance only. A design
decision is not live merely because an old row says `available`, and a current
failure does not become a language law merely because it once appeared in the
queue.

Dated session handoffs and superseded implementation snapshots live under
[`docs/historic/`](historic/README.md). They are provenance only. A future
instance resumes from this roadmap, the linked canonical design/ADR for the
owned area, and current version-pinned evidence; it must not recover live state
from a historic handoff.

## Compiler Level And Ownership Boundary

Swarm owns SwarmScript: the authored language, source compiler, Level-4
compiler-product reuse, SwarmVM, and runtime. SwarmScript's incremental
compiler is the content-addressed, owner-admitted reuse of sealed compiler
products described in [Incremental Compilation](INCREMENTAL_COMPILATION.md).
The Level-4/Level-5 distinction follows
[Agentic Closure And Representational Completeness](paper/agentic_closure_and_representational_completeness.md).

Swarmlib owns reusable Level-5 staged/reactive compiler libraries built **on
top of** SwarmScript. Orchid owns the concrete Level-5 software-development
compiler built with Swarmlib on that substrate. Level-5 dirty/recheck/frontier
orchestration is therefore not SwarmScript implementation work, is not a Swarm
completion gate, and must not drive SwarmScript's internal representation.
Swarm's obligation at that boundary is to provide the stable public language,
runtime operations, and admitted artifact/product surfaces needed above it.
This does not require exporting private SwarmScript compiler products or
letting Level-5 code own or reconstruct their authority.

The native-agent ownership split follows
[Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md):

- Swarm owns generic operation, protocol, actor, capability, checkpoint,
  process/sandbox, provider-ABI, authored-provider admission, and Mesh
  semantics. Decomposable capability policy defaults to authored SwarmScript;
  Rust/native code owns VM/FFI mechanics and irreducible host/OS primitives,
  not application allow lists or composition policy.
- Swarmlib owns the first-party SwarmScript model/tool harness, reusable agent
  profiles, prompt-backed integration, generic evaluation/qualification,
  realization portfolios and comparison, one-shot qualified selection,
  operational requalification, and optimization. The canonical reusable
  lifecycle is `swarmlib/docs/QUALIFICATION_AND_REALIZATION_SELECTION.md`. If
  selected later, optional foreign-harness adapters also belong here without
  changing the native public profile.
- Orchid owns software-development product meaning, prompts and tool
  portfolios, domain qualification and verification policy, compiler products,
  virtual-agent behavior, and UI projections.
- Hive owns resource requirements/catalog selection, materialization
  orchestration, placement, and resource lifecycle observation. Its admitted
  catalog binding selects the provider that materializes or attaches a
  resource; the resource owner mints its lease and the Swarm owner activates
  the opaque binding. Hive does not own agent session state, replay, actions,
  cancellation, or terminal truth.

The first cross-repository Level-5 delivery handoffs are intentionally
vertical:

```text
Swarm Operation + actor + ProtocolSession + provider substrate
  -> Swarmlib normalized model exchange
  -> deterministic scripted model driver and fixtures
  -> tool-free native memoryless SwarmScript agent harness
  -> memoryless Orchid simple-agent surface

native harness + typed tools + one exact fixed admission-time capability portfolio
  -> native typed-tool-loop surface
native typed-tool-loop + one exact remote provider binding
+ SwarmScript-free Rust capability host
  -> bounded local-vs-Mesh-remote tool placement proof
  -> cloud brain/control-surface/hands topology-compatibility conformance
native typed-tool-loop + generated ProtocolSession client projection
  -> first-party embedded client + thin CLI/IPC surface
protocol projection + typed ProductUiModel reducer
+ checked SSX/ViewTree/affordance plan + mounted terminal surface
  -> first-party TUI projection
native typed-tool-loop + scoped workspace/file/process/patch/test capabilities
  -> minimal local coding profile
  -> useful native coding harness

live owner + sealed attachment admission
  -> live attach under the same activation epoch
conversation owner + portable committed-conversation journal
+ exact terminal-boundary revision + exclusive-currentness/fencing evidence
+ continuity policy
  -> one-shot owner-sealed committed-continuation admission
committed-continuation admission + compatible target execution admission
  -> same-logical-identity continuation under one fenced successor epoch
     whose next turn is fresh and restores no in-flight authority
working native harness + actor/protocol supervision + budget attenuation
  -> fresh multi-agent surface
fresh child surface + governed purpose-scoped observations
  -> bounded child/peripheral context accumulation and materialization
+ typed assignment-derived help routing
  -> nested HelpSession composition with monotone disclosure, authority-request,
     budget, deadline, and hop scope
source conversation owner + expected committed revision
  -> one-shot owner-sealed committed-boundary fork snapshot
fork snapshot + fresh target execution admission
+ explicit memory/workspace/context/durability scopes + InitialContextPolicy
  -> source-preserving fork with a new conversation identity/activation epoch
     and no copied live authority
publication/run provenance + authorized historical read
  -> OBS selection of exact run/revision + inquiry mode
chosen mode + its normal sealed recovery/fork/seed-derivation admission
  -> exact restore then settle/fork | semantic fork | lossy seed inquiry
runtime checkpoint + runtime event journal + recovery admission
+ re-admitted local bindings + local subordinate settlement/reconciliation
  -> exact local restore under one fenced successor epoch
agent/tool/child/session protocols + Mesh grants
  -> location-transparent placement with unchanged destination-settlement law
Mesh placement + local recovery law + sealed subordinate checkpoint graph
  -> exact distributed restore under one fenced successor epoch

native harness
  -> prompt-backed StochasticFunction
prompt-backed function + Swarmlib qualification
  -> QualifiedStochasticFunction
qualified function + runtime assurance/verification/compensation
  -> AgenticFunction
AgenticFunctionSpec + finite RealizationPortfolio + QualificationSuite
+ exact evaluation-world/effect admission
  -> owner-bound evaluation targets
  -> authored graph execution + receipt-bound capture
  -> admitted evidence + case and comparison records
  -> qualified realization portfolio
current qualified portfolio
+ declared role/profile/safety/effect/availability/budget policy
  -> one-shot selected qualified realization + selection receipt

typed pending action + actor/protocol-owned decision admission
  -> local DecisionCase/DecisionSession queue with explicit timeout disposition
+ evaluation target over exact decision-description/process receipts
  -> independently qualified decision-description realizations
+ reactive dependency impact and running-work dispositions
  -> dependent-only DecisionPrerequisite fence and exact recheck on settlement
+ Mesh identity/authorization and durable owner journals
  -> multi-human coordination without global transcript or Git-only state

Swarm ProtocolSession + provider/resource/datastore substrate
  -> Hive phase one as the first deterministic Level-5 reconciler

Hive phase one
+ Swarmlib Graphstore/Memory/Retrieval/GraphRAG receipt continuity
+ a real ProtocolSession product surface
  -> first durable memory-integrated Orchid slice

Orchid AgentDefinition<OuterProfile> selecting an authored
  AgentProgram<OuterProfile>
+ exact AgentExecutionAdmission<OuterProfile>
+ child execution admissions and/or exact qualified AgenticFunction bindings
+ Swarmlib reusable reactive-compiler library
+ Orchid domain products and verification policy
  -> reactive virtual agent exposing the same admitted Profile through
     an outer ProtocolSession<AgentTurn<Profile>>

admitted evidence/currentness/decision receipt
+ SemanticWorkKey + opaque RunningWorkBinding set
+ exact dependency/assumption/decision footprint
+ base generation + versioned replan strategy
  -> Candidate<{closed dispositions, future-frontier patch}>
  -> verification + graph/frontier-owner compare-and-swap reservation/fence
  -> owner-sealed ReconciliationReservation
reservation + exact opaque bindings
  -> owning orchestration operation + one-shot RuntimeControlTicket per
     disposition that actually changes live work at a lawful safe boundary
  -> RuntimeControlReceipt set + reservation finalization/requeue/conflict
  -> durable reconciliation journal
  -> bounded redacted replay-stable explanation OBS
```

The sequence above is the active native delivery spine. Receipt-bearing memory,
evaluation/qualification, governed help, human decision work, and reactive
virtual agents are later composable branches with their own exact prerequisites;
they are not a single conjunctive gate. Pi, Codex, and Grok Build are
source-level harness design references and behavioral-conformance inventories,
not runtime dependencies or delivery commitments. Optional future Codex-, ACP-,
Claude-like-, Grok-, or other foreign-harness adapters may
join behind an already-stable public profile protocol, but are not an active
wave, prerequisite, or conformance gate.

Hive supplies admitted catalog selection and materialization orchestration; the
selected provider materializes or attaches the resource and the Swarm owner
activates its opaque binding. Swarmlib supplies semantic dependency receipts.
Orchid supplies product meaning. This
handoff does not make Hive, Swarmlib, or Orchid implementation a SwarmScript
completion gate, and a memoryless Orchid TUI/web client does not require Hive.
Likewise, the native agent harness is a flagship downstream SwarmScript
vertical, not part of the current compiler convergence gate.

Cross-repository prerequisites are public products and current conformance
evidence, not foreign repository phase names or queue state. Swarm publishes
the language/runtime products above; Swarmlib consumes only those public
surfaces and publishes reusable library products; Orchid owns the product-level
joins. A coordinator may schedule all repositories without introducing a
source dependency or synchronized repository lifecycle.

## Current Convergence Gate

The current gate remains:

```text
sole shallow compiler representation and complete stack-safe semantic SCCs
+ fail-closed checked emission and executable handoff
+ complete reviewed compiler product custody and one clean root close
+ conformance and configured-suite closure
  -> integration and configured-suite green
  -> every later consumer starts only from its exact landed producer
```

The frontier below replaces the former blanket “post-green” ordering with exact
producer edges. A frozen producer cut may proceed before broad green when it
does not consume an unfinished Wave-0 product. A downstream consumer may not.

The required Level-4 compiler-product reuse closeout belongs to the integration
gate: early lookup, owner re-admission, invalidation wiring, and current-source
proof. Optional cache expansion and Level-5 Swarmlib/Orchid work do not.

### Canonical implementation frontier — 2026-07-24

This is the implementation-order law. The
[Wave-0/Wave-1 Semantic Closure Index](WAVE0_WAVE1_SEMANTIC_CLOSURE_INDEX.md)
is a version-pinned review index; where its old row status or broad wave edge
conflicts with the current-source audits summarized here, this section controls.

Status is product-specific and has exactly these meanings:

- **LANDED**: the named bounded product is present on the canonical source line
  and may be consumed by a downstream owner. It does not complete a larger
  family that still has later rows.
- **ACTIVE IMPLEMENTATION**: the frozen owner cut is being executed. Its output
  is not a downstream dependency until it lands.
- **FROZEN — AWAITING EDITOR**: the owner, algebra, first source edit, deletion
  order, and proof gate are approved, but no active implementation owns it.
- **AWAITING INDEPENDENT APPROVAL**: a proposed contract or checkpoint exists,
  but it cannot authorize composition or positive source work until a
  source-aware independent review approves the complete semantic SCC.
- **BLOCKED BY UPSTREAM PRODUCT**: no lawful consumer edit exists until the
  named direct producer lands. Compile failures, stale callers, test
  reachability, and review availability never qualify for this status.

Rejected topologies are hard cuts, not a sixth status. A rejected checkpoint,
legacy path, or compatibility shape remains forbidden and cannot become active
work through a status relabel.

| Status | Product frontier | Direct dependency and implementation order | Downstream fan-out |
| --- | --- | --- | --- |
| **LANDED** | Parser-private shallow syntax substrate | Immutable shallow storage, branded consuming construction, opaque local cursors, and hostile-depth traversal/destruction exist. The next product is sole active parser settlement into that substrate; recursive `AuthoredSyntaxRoot` ingress and the fixed depth cap remain outside the landed claim. | Sole shallow ingress unlocks the remaining compiler stack-safety SCC, then recursion and SSX. |
| **FROZEN — AWAITING EDITOR** | W0-01 compiler transaction root and final cache observation | One private `SourceEntrypointCompilerTransactionV1` must begin at both real selected ingresses, retain the complete forest through `PreparedRoot`, consume branded forest/image/authored-family/epoch success into one proof, perform one infallible runtime/image commit and compiler-host finish, then publish one program-level atomic-manifest cache OBS. The unresolved first edit is the real `ss test` selected ingress; the current split root, early registry publication, repeatable cache marker, and late mint are rejected. | Its landed transaction is the sole direct producer for package/import atomic close and one required input to complete executable-artifact publication. |
| **BLOCKED BY UPSTREAM PRODUCT** | W0-02/W0-03/W0-04 package and import atomic close | Direct producer: landed W0-01 transaction. Then one authored-order batch of exactly six roles is resolved and fixed-role validated inside one root-owned non-closable subtransaction; `ResourceImport { Required \| Optional }` is authored before resolution and closes into `ResourceBinding`; the mechanically infallible close advances the same forest and returns no standalone receipt. | The closed package/import product feeds checker, contract, provider-lineage, resource, child-program, and executable-artifact owners. |
| **ACTIVE IMPLEMENTATION** | W0-05 installed capability transaction | Phase A linearly moves exact package implementation lineage before route sharing. Phase B consumes one generative root/continuation occurrence cohort through the finite allocator into site-bound instructions and a complete ordinary/static-child lifecycle precommit. Phase C performs the sole zero-work executable exposure and complete-runtime publication. Each phase consumes the previous product; none is independently claimable. | Completion unlocks W0-06 exact provider image-open close, Rust SDK/static, loaded-native, existing libbun settlement, built-ins, `@swarm/test`, renamed-package E2E, and shared host conformance. |
| **BLOCKED BY UPSTREAM PRODUCT** | W0-06 exact provider occurrence image-open close | Direct producer: W0-05 Phase-B generative occurrence/site product, followed by W0-05 Phase C. The present eight-way local preflight is retainable substrate, not an independently committable product; raw `ImportId`, `Arc`/`OnceLock`, positional queue/ordinal recovery, optional drain, and a second commit are forbidden. | W0-06 plus actor W1-01 unlock actor image/cell installation; W0-06 also feeds complete executable closure. |
| **LANDED** | Actor definition and whole-definition arm settlement | The combined callable/actor attachment family and typed retained refusal are in production. Whole-definition type-scope settlement now consumes all arm output facts into reusable exact arm contracts. This is the producer only; exact member occurrences and runtime closure remain separate products. | Feeds active W1-01 exact-member occurrence custody. |
| **ACTIVE IMPLEMENTATION** | W1-01 actor exact-member custody | Consume one settled reusable arm contract with the exact receiver/member occurrence into generative `Request \| Delivery \| Close`, carry it through the checked-expression-use graph and native body, and bind it once to an opaque image handler. Names, provider classification, regions, Session-LIR keys, and diagnostics may not remint the handler. | W1-01 plus W0-06 unlock W1-02. |
| **BLOCKED BY UPSTREAM PRODUCT** | W1-02 actor image binding and atomic cell install | Direct producers: the W1-01 exact-member product and W0-06 exact provider image close, after both land. Then whole-image preflight binds every occurrence, `InstallActorCell` performs one infallible commit, and the runtime mints intrinsic opaque `RuntimeActorRef`. | Unlocks W1-03 mixed FIFO, complete turn, lifecycle, checkpoint, shutdown, Drop, and remote-grant closure. |
| **BLOCKED BY UPSTREAM PRODUCT** | W1-03 actor turn and lifecycle closure | Direct producer: W1-02 `InstalledActorCellCommitReceipt`. One cell owns the mixed issue-sequence FIFO and selected turn; `ProcessSessionV0` owns entry, suspend/resume, settlement, atomic Close, checkpoint, shutdown, and iterative destruction. | Supplies the actor member of complete executable closure and the native harness/protocol consumer verticals. |
| **FROZEN — AWAITING EDITOR** | W1-04/W1-05 protocol Compiler C0 and W1-06 registered lifecycle | C0.1 consumes the complete placement-bound checked occurrence into table-owned `InstalledProtocolDefinition` while deleting the occurrence-free table and content/name/site fallbacks; C0.2 binds installed identity plus table ordinal into the image; C0.3 closes compiler lifecycle. Only then do runtime C.1 nine consuming family pairs, C.2 five-state registry settlement, and C.3 child-tree/checkpoint/shutdown/terminal/Drop closure proceed. These are direct internal edges, not parallel rows. | Installed protocol image authority feeds registered sessions and executable closure; the registered-session product feeds native clients, views, actors, Mesh, and harnesses. |
| **FROZEN — AWAITING EDITOR** | W1-07 primitive `Operation<Event, Output>` whole-body custody | The private `CheckedOperationBodyClosure = NotOperationBody \| Operation(SealedOperationBodyEmissionPlan)` producer and retaining refusal substrate exist. The next edit adds one consuming whole-body lowering cursor, exact `Yield` site/value settlement, terminal join, finish check, sole infallible image commit, and runtime step/cancellation/replay/restore/shutdown custody. Old W1-08 and W1-09 are subsumed, not parallel prerequisites. | Unified W1-07 feeds W1-14 complete executable closure and protocol/actor/provider operation consumers. |
| **LANDED** | W1-10 ProviderValue JSON V1 | Canonical fail-closed ProviderValue encode/decode is the sole current wire value product. | Direct producer for existing libbun mechanical drive. |
| **FROZEN — AWAITING EDITOR** | W1-11/W1-12 existing libbun mechanical drive and quiescence | W1-10 -> prepared export drive -> interrupt/quiescence mechanical outcome. This is the already-scoped libbun backend, not a general TypeScript SDK. | W1-12 plus W0-05 feeds W1-13 semantic settlement. |
| **BLOCKED BY UPSTREAM PRODUCT** | W1-13 existing libbun semantic settlement | Direct producers: W0-05 installed operation and W1-12 mechanical outcome. The common installed-capability owner alone converts that outcome into exact typed settlement. | Feeds W1-14 complete executable closure and current-host parity. |
| **BLOCKED BY UPSTREAM PRODUCT** | W1-14 executable closure publication | Direct producers: W0-01 clean compiler close, W0-05/W0-06 provider closure, W1-02 actor image authority, W1-05 installed protocol image authority, W1-07 primitive whole-body closure, and W1-13 libbun settlement. Only their joined complete closure may classify as persistable, serialize deterministically, close compiler custody, publish atomically, read back, and mint one admitted artifact receipt. | The admitted artifact receipt is the sole input to W1-15 and later cold/warm restore, cache, and distribution work. |
| **BLOCKED BY UPSTREAM PRODUCT** | W1-15 source-free execution worker | Direct producer: W1-14 admitted complete-artifact receipt. A fresh worker consumes that receipt plus launch-environment authority; source paths, manifests, compiler caches/libraries, recompilation, rederivation, and recovery compile are absent. | Unlocks artifact-only execution equivalence, source-worker-route deletion, and portable host consumption. |

The current-source audits intentionally supersede two stale index readings.
Package rows marked “A done” or “B done” were documentation milestones, not
landed Rust products; the atomic package close remains blocked by W0-01.
Conversely, protocol C0 and primitive whole-body custody each have a lawful
first owner edit on the current topology and are frozen-awaiting-editor; a
broad Wave-1 edge from W0-04 is not a semantic blocker for those producer cuts.

### Language implementation frontier

The [Primer](SWARMSCRIPT_PRIMER.md) and
[Language Improvements](LANGUAGE_IMPROVEMENTS.md) state one authored language,
including required surfaces whose verticals remain incomplete. Acceptance of a
forbidden form is an implementation defect, never a compatibility edition.

| Status | Language product | Direct dependency and next owner product |
| --- | --- | --- |
| **LANDED** | Bounded language substrates | The parser-private shallow arena, removal of `stacker`, immutable `IntegerRange`, exact primitive equality/refusal, nominal graph application/await/materialization/run, graph reconcile lineage, canonical scalar helper substrate, and actor definition/arm correspondence are real bounded products. Each may be consumed only for the law it already proves; none implies its surrounding vertical is complete. |
| **FROZEN — AWAITING EDITOR** | Sole shallow parser ingress and complete compiler stack safety | Activate the existing shallow parse settlement as the only syntax ingress, migrate every parser/checker/lowering/diagnostic/destruction consumer to opaque cursors and heap work machines, then delete recursive `AuthoredSyntaxRoot`, synchronous SCC re-entry, the fixed depth cap, and parallel recursive teardown. |
| **FROZEN — AWAITING EDITOR** | Must-use and affine authority | Introduce the origin-keyed checked obligation that preserves consuming-position facts through flow joins and terminal settlement, then carry it into runtime/provider custody. `Take<T>` returning naked `T`, binding without settlement, double take, implicit discard, clone/serde, and reconstruction from ids or parts are hard-cut. |
| **FROZEN — AWAITING EDITOR** | Canonical number, text, Bytes, and regex values | Replace authored `f64` and overlapping string/number carriers with one closed canonical scalar algebra: exact unbounded integers, finite non-integral floats, NFC text on one grapheme ruler, exact `Bytes`, pinned Unicode semantics, and grapheme-boundary regex indices. Host numeric/string fallbacks and UTF-16/byte/scalar ruler drift are rejected. |
| **FROZEN — AWAITING EDITOR** | Collection value semantics | Runtime non-aliasing value-tree transfer is first; nominal primitive Map/Set, fresh-literal exactness, array operation settlement and typed sort mode, and dynamic deterministic object-key iteration follow in that order. Shared heap-cell mutation, ProviderValue-backed Map/Set authority, string-render sort, dead `copyWithin`, and width-subtyped fresh literals are not compatibility behavior. |
| **FROZEN — AWAITING EDITOR** | Graph capture, sink binding, and reconcile execution | Existing nominal graph/program authority remains landed. Next, checked graph-body capability requirements move through placement/application/materialization into one finite sink rebind; then caller-selected materialization result generics are deleted and reconcile must start added work and cancel/drain removed work. Empty-capture fallback and structural graph/program promotion are forbidden. |
| **FROZEN — AWAITING EDITOR** | Authored absence hard cut | The approved current-source integration order is syntax/type algebra -> checked use -> lowering/runtime/provider boundary -> deletion. Authored `null`, storable `undefined`, nullable/undefined unions, raw optional sentinels, and compatibility readers disappear; `Option<T>`, omitted optional fields, `void`, and explicit boundary `JsonField<T>` are the only final forms. |
| **AWAITING INDEPENDENT APPROVAL** | One authored Result/error channel and managed cleanup | The complete SCC is frozen as prefix `try`, expression `catch`, closed error sets, recursive typed `ErrorValue.cause`, and activation-owned `defer`/`errdefer` frames across suspension/checkpoint/cancel/unwind/shutdown. Independent source-aware approval and dual-path red evidence precede its first parser edit. VM-only branching, exception syntax, `Throw -> Retry`, public Result parts, callback cleanup, and terminal-only substitutes are rejected. |
| **FROZEN — AWAITING EDITOR** | Scalar obligations and nominal capability/protocol constraints | The bounded top-level `requires`/`ensures` path is landed only for its covered scalar cases. Complete field/local/protocol/boundary obligations first; separately add a checker-owned finite nominal requirement algebra over already-owned authority. Object-shape matching, callbacks, manifests, or reusable public proofs cannot satisfy a capability/protocol requirement. |
| **FROZEN — AWAITING EDITOR** | Program budget authority | Trusted launcher grant -> admitted requirements -> lexical ceiling -> deterministic branch reservation -> child subgrant -> authenticated settlement -> policy-governed reclaim -> cleanup-safe final settlement is one owner chain. Raw counters, internal watchdogs, local unlimited grants, `None`, and compatibility metering cannot stand in for it. |
| **BLOCKED BY UPSTREAM PRODUCT** | Recursive calls, tail transfer, and TRMC | Direct producers: sole shallow parser ingress and complete checker semantic-SCC stack safety; ordinary recursive runtime admission also consumes the canonical program-budget plan. After those land, mint one checked recursive callable family, then ordinary calls, proper tail transfer, TRMC, lifecycle, checkpoint, and hostile-depth proof. |
| **BLOCKED BY UPSTREAM PRODUCT** | SSX checking and `ViewTree` | Direct producer: sole active shallow SSX parser occurrence. Then the independent closed surface/view owner supplies the versioned intrinsic algebra, surface-indexed inert tree, checked projection, reference observation, and mount-owned affordance plan before the existing checker refusal is removed. Raw element strings, object/list fallback lowering, callbacks, DOM/native refs, and renderer-selected authority are forbidden. |
| **FROZEN — AWAITING EDITOR** | Declaration-only TSON | One admitted `.contract.ts` occurrence plus one exact demanded export/member is joined by the compiler owner to typed normalized TSON candidate facts, yielding a private concrete member or generic template. `.tson.ts`, `defineContract(...)`, `toContractTson`, runtime-valued builders, normalized JSON admission, public selector/parts/binder APIs, source regeneration, and provider-route fallback are deleted; they remain only as explicit negative fixtures or historical text. |

The language fan-out is deliberate: the sole shallow syntax product unlocks
both recursion and SSX without making either depend on the other. The admitted
declaration-contract member feeds boundary checking and the installed-
capability owner; only the latter fans out to Rust/static, loaded-native, and
existing libbun adapters. Option work does not stand in for Result handling,
and scalar obligations do not stand in for nominal authority constraints.

### Hard-cut and no-compatibility law

Every arrow in these frontiers is a replacement, not an alternate route:

- package resolution uses the `swarm` export condition only; `default`,
  `types`, suffix, path, content, or manifest fallback cannot select authored
  meaning or reclassify an import;
- declaration contracts are `.contract.ts` only; `defineContract(...)`,
  `.tson.ts`, runtime builders, source regeneration, and normalized JSON never
  remain as compatibility admission;
- selected work reaches the next sealed state or a typed retaining fault;
  `None`, empty/default output, string faults, placeholder success, and
  caller-selected callback receipts are not transitions;
- owner authority never crosses as public ids, names, paths, descriptors,
  manifests, registers, DTO/parts, borrowed getters, clone, or serde for a
  sibling to remint; and
- executable workers consume admitted artifacts only. There is no source-path
  worker, recovery compile, dual source/artifact mode, or runtime fallback after
  W1-15.

Future Python, Go, general TypeScript, and WebAssembly hosts must consume the
same contract and semantic products. Portability is a constraint on today's
algebra, not permission to add provisional adapters or compatibility bridges.

### Compiler-root production contract

This block restates the frozen owner contract. A later document may supersede
it only by naming a new reviewed owner contract and explicitly replacing this
block; the current-source audit controls which recorded source cut remains
unresolved.

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
    its late caller. Positive implementation begins with the required
    dual-path red evidence and retains the complete lifecycle and hostile-depth
    proof gate.

**FUTURE COMPATIBILITY REQUIREMENT — OUT OF CURRENT IMPLEMENTATION SCOPE.**
Python SDK and host work, Go SDK and host work, a general TypeScript SDK, a
WASM/component provider host, and their generated bindings are required future
compatibility destinations. Their implementations are not current milestones,
prerequisites, acceptance gates, or current deliverables.
The current provider-host goals are Rust SDK/static, loaded-native, and the
already-scoped existing libbun backend drive, interrupt/quiescence, and
semantic-settlement path. Existing libbun lifecycle work does not imply a
general TypeScript SDK or authoring project.

Current work must keep the common semantic algebra, canonical contract facts,
and wire language-neutral so those later hosts can consume them without a
second contract or authority model. That compatibility constraint is required
now even though the adapters are not. Current work must not design or implement
the future SDKs, hosts, generators, WIT surfaces, decorator conventions,
compatibility bridges, fixtures, or cross-host suites. A later separately
approved roadmap schedules their implementation; no current freeze implicitly
does so.

Independent design reviews are evidence rather than architecture. A checkpoint
becomes **LANDED** only through current-source reconciliation and the applicable
behavior, lifecycle, hostile-depth, and deletion proofs. A rejected topology or
pending review cannot be relabeled as active implementation.

### Audit-derived convergence obligations

Convergence audits update this roadmap only when they identify a durable
semantic obligation. Exact failures, candidate branches, and temporary Lane
state remain version-pinned execution evidence, not roadmap prose.

The checked-expression and statement-attachment audits establish these
required end states:

- **One total machine per recursive semantic component.** Every mutually
  recursive compiler operation whose depth can be controlled by authored
  syntax, types, values, dependencies, or accumulated runtime state is closed
  as one phase-specific semantic SCC and owned by one private heap
  work/continuation machine. The sealed boundary is the semantic owner's
  admitted request and settled product or typed fault; internal tasks and
  continuations remain private ordinary enums rather than cross-crate products.
  Nested statements, expressions, callable bodies, member receivers, type
  algebra, diagnostics, and teardown may not synchronously re-enter another
  copy of the same SCC, expose a recursive compatibility entry point, or rely
  on a generic machine framework, native-stack growth, arbitrary depth fuel,
  or lossy memoization. Legal fixpoints and illegal cycles use existing stable
  content-addressed semantic product identity plus exact operation identity;
  transient arena ordinals and a second incremental-query system are not
  identity. This is part of the current integration-green and full Wave-0
  stack-safety gate.
- **One fail-closed compiler-worker supervision boundary.** Compiler machines
  retain bounded recent-transition provenance sufficient to name phase,
  operation, semantic product identity, exact occurrence when available, and
  bounded work/result counts. The compiler-worker owner converts unexpected
  worker termination, signal, or admitted watchdog exhaustion into one typed
  compiler fault with that last bounded provenance, so a suite or application
  cannot hang indefinitely or lose the responsible phase. This is defense in
  depth, not permission for recursion: ordinary authored success and refusal
  still complete in-process on the fixed tiny-stack proofs. Supervision and
  progress observation live once at the worker boundary, not as a sealed
  receipt, heartbeat product, or persistence ceremony on every internal
  transition.
- **Lazy selected-expression evaluation.** A selected operation used by a
  recursive boolean condition belongs to the exact boolean atom that consumes
  it. The operation executes only after lazy control reaches that atom;
  `false && call()` and `true || call()` do not execute the skipped call.
  Child-before-parent order, one-shot result settlement, and typed
  missing/duplicate/foreign-body refusal remain exact. This is part of the
  current integration-green gate.
- **One checked selected-expression evaluation product.** Provider boundaries,
  authored-call arguments, computed returns, boundary decode, and lazy boolean
  atoms must converge on one exact occurrence-directed evaluation product and
  one exhaustive operation emitter. Destination-specific syntax walkers,
  `SrcLoc` containment recovery, result-register bundles, and duplicate
  operation-variant matches are migration state, not the destination. This is
  required post-green semantic-compiler consolidation.
- **Fail-closed checked lowering and emission.** Every executable semantic
  operation whose meaning is decided during checking must reach lowering as
  one exact occurrence-owned checked disposition. Missing, duplicate,
  foreign-body, mismatched, or unconsumed dispositions refuse executable
  preparation through a typed owner fault before an image can be published.
  Lowering must not recover absent checked meaning from syntax text, property
  names, source coordinates, inferred runtime shape, or a plausible generic
  instruction. A dynamic or generic operation is legal only when the checker
  explicitly selected that closed variant; it is never the fallback for a lost
  specialized product. Each operation family requires positive specialized
  emission proofs, adjacent generic controls, and negative
  missing/duplicate/foreign/mismatch refusal proofs. The runtime retains its
  own typed validation as defense in depth, but runtime rejection is not a
  substitute for compiler emission closure. This is part of the current
  integration-green gate and the post-green one-emitter consolidation.
- **Fail-closed executable handoff and runtime observation.** Compiler closure
  alone is insufficient: image derivation, publication, admission, instruction
  dispatch, provider/effect selection, and terminal settlement each validate
  their complete sealed input before making the corresponding transition
  observable. Missing, duplicate, foreign, mismatched, unknown, unsupported,
  or unconsumed authority returns the typed fault owned by that boundary. It
  never selects a plausible generic instruction, reconstructs meaning from
  names/positions/runtime shape, or emits a sentinel, empty, default, or
  fabricated success value. Every boundary that can still refuse before
  commit does so before publishing an image, launching work, emitting an
  authored output/event/effect, advancing a checkpoint, or settling a result.
  If an external system has already accepted an irreversible effect, the
  runtime preserves that explicit uncertain/failed settlement state; it does
  not pretend the effect did not happen or manufacture success. Each family
  requires positive execution proof plus negative refusal proof that no
  partial image, output, effect, checkpoint advance, or settlement escaped.
  This is pulled into the current convergence gate rather than deferred to a
  later runtime vertical.
- **Exact durable event-publication operation.** The checked
  `@swarm/event:publishEvent` disposition remains exact through image
  admission and selects `EventAppend`, never generic `ProviderResume`. One
  sealed owner-native selection and execution operation retains the typed
  publication payload and completion ticket together across every pre-commit
  refusal, drives any required durable resume, admits exactly one primary
  append receipt, and completes that exact ticket exactly once. Missing,
  mismatched, duplicate, encoding, append, or receipt-drift cases remain typed
  refusals with retry custody wherever retry is still lawful; they do not
  collapse into strings, raw-parts carriers, callback/DTO bridges, or dormant
  tuple scaffolding. This becomes part of the Wave-0 gate when configured
  fixtures exercise event publication; otherwise it remains the required
  durable runtime-operation destination.
- **Sealed contract identity admission.** An admitted Contract-TSON package
  identity remains opaque through package-graph selection and capability
  linking and is consumed to mint the exact capability-contract identity.
  Package specifier, export name, and contract fingerprint do not cross owner
  boundaries as a public tuple or independently borrowed fields from which a
  sibling can reconstruct provider-routing authority.
- **Sealed native-provider import.** Selected native-provider artifact,
  manifest binding, prepared-runtime image, and host-load admission compose as
  one purpose-specific import operation. Artifact label, root, path, and
  fingerprint remain inside its sealed input and cannot be caller-re-admitted
  or duplicated into execution authority. The prepared-runtime implementation
  of this integrated owner cut is active and owns the first complete handoff.
- **Finite selected-provider execution.** A selected provider input is consumed
  inside the session execution kernel by the exact activate, load, checkpoint,
  or restore operation that owns its meaning. Callers receive only that
  operation's sealed ingress, result, or typed refusal; raw provider values and
  invocation fingerprints do not cross into a sibling runtime owner for
  interpretation or identity construction.
- **Owner-minted process preservation.** The owner that holds actor state,
  scoped resources, image facts, and live-plan inventory derives and joins the
  exact preservation set and mints the resulting sealed certificate. A ticket
  cannot accept caller-selected fact identifiers, including an empty set, and
  raw process identity is not a public selector; foreign joins retain their
  original products or return the owning typed refusal.
- **One-shot carrier-witness and router-send admission.** Admission by a
  recorded carrier token atomically consumes the exact stored witness, but the
  finite owner boundary closes only when one consuming router-send runtime
  admission takes the native receipt by value and moves its actor and accepted
  transport-attempt evidence exactly once into one sealed runtime send witness
  or typed fault. Borrowed authority getters, receipt reissue, and duplicate
  native receipts are not lawful continuations of the take. Reuse, absence,
  and forgery return typed refusal; deterministic content addressing may
  identify the recorded witness but does not make admission replayable.
- **Sealed package-resource route selection.** The package-resource owner
  consumes a selected target once, performs provider-manifest branching and
  contract-target resolution internally, and returns the final sealed route or
  typed fault observation. Public paths, duplicated resolution seeds,
  repeatably borrowed contract modules, and reconstructible branch enums are
  not route authority.
- **Selected-startup-owned executable identity.** The selected startup product
  carries its exact package root, entry path, and closure fingerprint into
  executable-source admission and finite materialization. A sibling cannot
  inject or split out raw identity material and then mint prepared-runtime or
  checkpoint authority from it.

These are seven purpose-specific products and finite owner operations, not a
universal owner-boundary framework, generic carrier, DTO layer, or naming
doctrine. Consuming transitions that return only non-forgeable sealed products
and terminal observations with no inverse authority path remain lawful. This
is implementation convergence, not an authored-language change: the Primer
remains free of compiler/runtime owner structure and changes only when the
authored language surface changes.
- **Statement-owned semantic products.** Computed-return classifications,
  plainness, reads and writes, callback work, return publications, condition
  evaluations, and direct/prepared supersession attach through the exact
  checker-minted statement occurrence. Body-global positions, ordinals,
  counts, vector alignment, and source coordinates may remain diagnostics or
  private traversal state, but never attachment authority. A multi-statement
  attachment validates completely before any statement is mutated. Speculative
  lowering consumes a complete move-only input transaction: every refusal
  restores the entire candidate/verdict/publication set before another arm or
  retry may proceed. Per-expression partial mutation, cloned backup authority,
  and counter reset are not recovery.
- **Concrete checked-body attachment owner.** Exact source-body, statement,
  expression, call, executable-local producer/read, and selected-expression
  attachment authority belongs below source-work-set composition and above
  typed HIR/runtime emission. The owner-boundary audit settles the required
  owner as `swarmscript-checked-body-attachment-owner`: first move the complete
  module-run/callable/actor-handler source-body identity family, then occurrence
  attachment, selected-local/value-use pools, and checked provider-input
  construction; finally compile the scanner normally through that owner and
  delete cross-crate path inclusion. Runtime instruction materialization stays
  in the kernel. Generic carrier, DTO, session-kernel, source-work-set
  orchestration, and public raw-coordinate alternatives are rejected.
- **Zero cross-crate Rust source compilation.** The current eleven
  cross-crate `#[path]` inclusions are temporary migration overlap, not durable
  crate boundaries. The exact hard-cut inventory is:

  - `crates/ss-runtime-source-work-set-admission/src/lib.rs` ->
    `crates/swarmscript-check-syntax-type-scope-owner/src/type_scope.rs`
  - `crates/ss-runtime-source-work-set-admission/src/lib.rs` ->
    `crates/swarmscript-check-syntax-type-scope-owner/src/boundary_contract_index.rs`
  - `crates/ss-runtime-source-work-set-admission/src/artifacts.rs` ->
    `crates/swarmscript-check-syntax-type-scope-owner/src/syntax_module/provider_call_evidence.rs`
  - `crates/swarmscript-parser/src/authored_provenance.rs` ->
    `crates/swarmscript-syntax/src/implementation.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/binding_declarations.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/expression.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/module_items.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/statement.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/type_expression.rs`
  - `crates/swarmscript-parser/src/authored_syntax_types.rs` ->
    `crates/swarmscript-authored-syntax/src/value_expression.rs`
  - `crates/swarmscript-parser/src/lib.rs` ->
    `crates/swarmscript-checked-body-attachment-owner/src/implementation.rs`

  The destination is ownership, not consolidation.
  `type_scope.rs` and `boundary_contract_index.rs` compile exactly once in
  `swarmscript-check-syntax-type-scope-owner`, which owns a consuming private
  type-scope/boundary-contract operation used through the existing normal
  dependency. The provider-call scanner compiles exactly once behind the
  checked-body attachment operation named above; syntax/type-scope selection
  phases may remain private to their owner, but source-work-set composition
  does not compile or interpret them. The six authored-syntax sources and the
  lowered-syntax implementation belong to the generative parser/computed-member
  owner while it owns exact artifact, occurrence, and computed-member
  correlation; after the installed-syntax fixture and callable transition is
  frozen, those sources move under that owner and the authored-syntax and
  syntax outward crates use ordinary dependency re-exports of sealed products.
  Checked-body implementation moves in the other direction: it compiles in
  `swarmscript-checked-body-attachment-owner`, and the parser/computed-member
  owner depends normally on its sealed attachment operation.

  Dependency order is: finish the active production compiler-session fixture
  and parser installed-syntax/callable migrations; make checked-body a real
  compiling owner; close the parser-owned authored/lowered source relocation;
  expose the type-scope operation before the boundary-contract operation; then
  close provider-call scanning through those owner operations and delete the
  local source-work-set copies. Acceptance is an exact repository scan with
  zero cross-crate `#[path]` attributes, every implementation compiled in one
  semantic owner, and every consumer using a normal Cargo dependency plus a
  private or sealed owner operation. A public raw getter, parts API, callback,
  or DTO facade is not an accepted way to break any cycle.
- **One production compiler-session transition for fixtures and applications.**
  Full-module and package conformance fixtures consume the same production
  source-entrypoint compile owner used by applications, with fixture package
  universes admitted as purpose-specific owner inputs. Test code may select
  inputs and observe final receipts, but it does not independently sequence
  parse, checker selection, capability facts, attachment, lowering, or the
  typecheck hard cut. This closes proof-topology drift as part of Wave 0 after
  the private whole-module lowering-session interface freezes; it does not
  require exposing compiler internals or replacing focused unit tests. The
  parser-authored fixture boundary mints the caller's module identity once and
  owns parsing, lowering, canonical traversal, issuance, and installation as
  one transition returning an opaque move-only installed fixture observation.
  Fixture helpers do not lend raw AST/run-statement parts, retain pre-install
  occurrences or reconstructible selectors, remint module identity, replay a
  second exact-call traversal, or independently create attachment authority.
  Purpose-specific negative scenarios are selected inputs handled inside that
  transition before closure; assertions consume checker-owned receipts from
  the installed product.
- **Interaction-facade checked lowering.** Authored `@swarm/interaction` open
  and policy-bearing observe operations retain their exact checked protocol,
  input/policy, and local-result producer through purpose-specific checked
  plans, exhaustive emission, and runtime settlement. The current typed
  unlowered refusals are not the destination language behavior. This is the
  first post-green interaction vertical unless a configured Wave-0 fixture
  exercises the surface, in which case that fixture makes the missing vertical
  part of the current green gate.

Completed foundations must not remain phrased as future prerequisites. The
parser-private shallow snapshot foundation is landed: it has immutable shallow
node/child storage, opaque snapshot-local cursors, a generatively branded
consuming builder, cross-snapshot refusal, and hostile-depth tiny-stack
traversal/destruction proof. The broader stack-safety area remains open until
the parser returns that snapshot as its sole syntax product, downstream
consumers migrate, recursive destruction is gone, and the temporary
authored-depth cap is deleted. The `stacker` call and Cargo-dependency boundary
has already closed. The entire remaining
tranche is Wave 0 work: parser construction/recovery/lowering; authored and
checked syntax construction, traversal, traits, and destruction; type,
statement, expression, attachment, HIR, and emission consumers; bounded
diagnostic/fault rendering; and safe teardown of valid, refused, partial, and
pending work. No member of that tranche may be deferred as post-green
foundation work. Independent owner/file families should proceed concurrently
as soon as their shallow-input predecessor has landed; final recursive-consumer
and depth-cap removal follows the remaining hostile-depth proofs.

Two designs are settled during convergence rather than deferred behind their
first product consumer:

- SSX participates in the shallow-syntax representation cut because its parser
  and recursive authored nodes already exist; semantic checker/lowering work is
  the first post-convergence language vertical.
- runtime provider/backend activation is one host law, not a minimal/standard
  edition split. Native plugins remain inactive until an admitted requirement
  selects them.

Expression-context closure is likewise a pre-roadmap trust gate, not optional
post-green semantic-compiler consolidation. The concrete
`swarmscript-checked-body-attachment-owner` owns exact body, statement,
expression, and call attachment. Its first ownership move is the complete
module-run/callable/actor-handler
`ProviderEffectSourceBodyIdentityForCheckerScanOwnerV1` family; occurrence
attachment, selected-local/value-use pools, and checked provider input then
move behind that boundary. The kernel retains runtime instruction
materialization and does not rediscover checked expression meaning.

The closed semantic unit is:

```text
exact checked body + exact parent expression use
  -> one opaque CheckedExpressionUse
  -> shallow checked evaluation graph
       exact occurrences and checked types/value uses
       lazy control edges and child-before-parent order
       exact selected-operation dispositions
  -> one exhaustive emitter
  -> final parent continuation
```

This graph is a shallow arena/table work machine, never another recursive AST
or a generic source walker. Context determines only how the settled result is
consumed; condition, return, yield, argument, initializer, assignment,
iteration, member/index/optional-chain, callable/default/callback,
await/try/decode, actor/provider/process/graph/protocol/operation,
module/export/actor-state, resource/retry, and nested
object/list/template/spread/operator uses may not select different expression
semantics implementations.

Closure is proved over that finite context set across exact syntax occurrence,
checker/type/flow selection, selected-operation/value-use closure, exact
checked-body attachment, exhaustive lowering/emission, runtime typed
validation, and refusal/diagnostic/drop. Every matrix cell requires positive
root and nested behavior, lazy and child-before-parent ordering where
applicable, refusal of missing, duplicate, foreign-body/same-span,
mismatched-context, and unconsumed dispositions, plus an adjacent ordinary
non-specialized control. Adding another destination variant, semantic optional
sidecar, or position-scoped expression walker does not satisfy this gate.

## Durable Execution Order

Waves group products by dependency depth; they are not release trains,
staffing plans, or permission to guess an upstream API.

| Wave | Products admitted at this depth | Required incoming products |
| --- | --- | --- |
| 0. Compiler convergence | Sole shallow syntax ingress and full semantic-SCC stack safety; W0-01 compiler transaction root; fail-closed checked expression/lowering/emission; integration and configured-suite closure; required Level-4 product reuse | Existing parser-private shallow substrate and current sealed compiler products |
| 1. Semantic foundations | Package/import atomic close; declaration-only TSON; installed capability transaction; primitive operation whole-body closure; actor exact-member/image/cell/turn closure; protocol C0 and registered lifecycle; complete executable closure and source-free launch | Only the direct producer edges in the canonical frontier table; no broad wave edge substitutes for them |
| 2. Authored language and first consumers | Must-use/affine, Result/error/cleanup, absence, canonical values/collections, budgets, recursion, SSX/ViewTree, provider adapter parity, runtime activation, normalized model exchange, deterministic model fixtures, memoryless native turn, typed tools, generated client, minimal local coding harness | Each language vertical's exact checker/runtime product; harness work additionally requires landed TSON/provider, `Operation`, actor, protocol, and scoped tool-capability products |
| 3. Runtime and host verticals | Replan/output fencing, reload/upgrades, Mesh grants, Datastore/GraphStore, explicit ingress, renderer and SwarmWeb products, committed conversation continuity, fresh children, fork/provenance, local recovery, then Mesh placement and distributed recovery; memory, qualification, decision, and reactive branches join when their own inputs land | Public Swarm products from Waves 1-2, never private compiler products or foreign-repository phase state |
| 4. Product closeout | End-to-end host paths, selected backend parity, public deployment surfaces, final syntax inventory, Primer alignment, normative specification, and full certification | Landed semantic verticals plus current evidence for every selected product path |

Independent products may overlap only when each consumes already-landed inputs
and produces a sealed result that later joins unchanged. An active or proposed
producer is not an available dependency. Store, publication, adapter, and host
work may start early only when its semantic owner already has every required
input; otherwise only its proof oracle or independent mechanical substrate may
advance.

The agent entries in Waves 2-3 are downstream delivery opportunities, not
SwarmScript completion gates. The memoryless native harness does not wait for
Hive, semantic memory, the coding profile, optional foreign compatibility, or
Mesh. Each later capability expansion begins only after the public owner
products it consumes have landed.

The embedded client and thin CLI/IPC protocol smoke projection land in Wave 2
as soon as the typed-tool-loop generated protocol surface exists. A TUI consumes
that same projection only after the typed reducer/ProductUiModel, checked SSX/
ViewTree affordance plan, mount owner, and terminal renderer exist. The clients
become useful coding surfaces when the minimal local coding profile lands; none
waits for fresh children, durability, optional foreign compatibility, Hive,
semantic memory, or Mesh beyond its own exact view prerequisites.

Once the typed-tool selection and settlement law is stable, a parallel
remote-hands proof binds one exact tool contract through the generic Rust
capability host and compares it with the local provider placement. This proof is
owned by [Capability Host Deployment And Tool Placement](CAPABILITY_HOST_DEPLOYMENT_AND_TOOL_PLACEMENT_DESIGN.md).
It does not gate the local coding harness and does not claim general remote
model, child, conversation, session, or recovery placement.

The active native product order is deterministic model exchange, one memoryless
turn, typed tools, the generated client, the minimal local coding harness,
committed journal/resume, fresh children, fork/provenance, local recovery, Mesh
placement, and only then a distributed checkpoint graph. Receipt-bearing
memory, evaluation/qualification, and reactive virtual agents are later
composable branches rather than a conjunctive tail. Foreign-harness adapters
and cross-realization suites are optional future compatibility work outside the
active waves.

### Streaming Compile-To-Image DAG Destination

The executable-artifact destination is a streaming product DAG, not a
compile-all/execute-all wave:

This is the mandatory dependency map after compiler-product custody freezes.
It is not a Wave-0 prerequisite, and the current tree makes no complete
cutover claim.

```text
selected source/module closure
  -> content-addressed checked compiler products
  -> immutable executable-closure image
  -> deterministic serialized closure, compiler-owner private
  -> closure-local compiler custody close
  -> external child publication and atomic root commit
  -> root read-back and owner re-admission
  -> sealed admitted publication receipt
  -> immediate runtime admission and execution
```

The semantic compiler owner consumes each exact admitted predecessor bundle and
produces each exact compiler-product identity once. It may use bounded internal
parallel mechanics, but no mechanical task becomes an independent compiler
worker, admits or re-derives a product, chooses dependents, or mints receipts.
The owner resolves each exact identity as
`ExactHit | JoinExisting | FreshDerivation`, admits the candidate or exact cache
hit, and immediately hands the exact opaque predecessor product to every
newly-ready actual typed dependent. An exact admitted hit performs zero
derivation, and concurrent requesters join one owner-held attempt. Every ready
node starts as soon as bounded capacity exists: no phase wave, authored-order
wave, suite/workspace barrier, deterministic commit order, slow lower-index
node, or unrelated closure may withhold ready work.

Cache persistence, indexing, replication, retention, and deterministic
observation ordering may stage privately only after in-memory admission and
handoff. Cache failure cannot delay, revoke, or change that product. No staged
envelope becomes externally visible until one deterministic serialized complete
closure exists, its owner preflights publication, joins its closure-local
scopes, and closes custody. Only then may external child writes and the atomic
root commit begin. Read-back and owner re-admission precede the sealed receipt.
That receipt may launch while unrelated closure roots continue compiling.

The suite-wide Merkle root is observation and cache identity, never an execution
barrier. Owner-owned in-flight identity deduplication prevents duplicate
compilation of the same node, and artifact publication exposes no partial root.

The current `execute_fallible_bounded_product_dag` is a numeric readiness gate:
jobs already contain their input, predecessor outputs remain pending, and all
outputs escape after graph completion. Completion-streaming and feed-streaming
callbacks construct downstream jobs. These APIs may supply private mechanical
readiness/spawn policy, but they are not the semantic compiler-product owner;
callback-selected jobs and deterministic commit ordering are forbidden on the
typed predecessor handoff path. This supersedes prior claims that the landed
scheduler already completed the semantic product DAG.

Wave 0 and Wave 1 are ordered owner laws, not alternate production tracks:

```text
Wave 0:
one private SourceEntrypointCompilerTransactionV1
  -> exact compiler products + one clean compiler-custody close

Wave 1 W1-14 (unlanded):
sealed phase products
  -> canonical product envelopes and typed Merkle links
  -> complete executable-closure classification
  -> deterministic serialized closure
  -> compiler custody close
  -> external children + atomic root commit
  -> root read-back and owner re-admission
  -> sealed admitted publication receipt

Wave 1 W1-15 (unlanded):
sealed admitted publication receipt + launch-environment authority
  -> fresh execution-worker admission and runtime installation
```

Wave 0 forbids a late, child-local, retry, fixture, or compatibility compiler
root. W1-14 keeps all compilation inside the compiler owner. W1-15 gives a
worker only the completed sealed executable product and exact launch resources;
the worker never compiles, re-derives, re-admits cache bytes, or remints
authority. Existing selected-source/path child compilation is stale
implementation to delete, not a current-law green track or compatibility mode.

Current bounded readiness, cache/publication substrate, and live-image code are
implementation evidence only. No current operation is yet portable
executable-artifact admission. Passing the current suite does not certify
W1-14 or W1-15. Artifact encoding, publication, admission, and source-free
activation may proceed only after their exact predecessor owner products land;
their work streams by product readiness and never imposes a suite-wide barrier.

The frozen first completeness cut is
`DirectRunPreparedRuntimeAuthorityOwner::install_compiler_prepared_source_program_image_v1`.
That operation is in
`crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime/image_install_transaction.rs`
and is reached from
`install_compiler_prepared_source_program_image_for_source_entrypoint_owner_v1`.
The first post-freeze edit remains an
owner-private `CompleteExecutableClosure | NonRestorable(typed fault)`
classification there, before the current live image install commit. The lower
scheduler, cache, manifest, worker wire, source path, hash, or Session-LIR
projection is not the owner.

The ordered post-freeze dependencies are:

```text
custody freeze and clean closure-local close
  -> semantic compiler-product owner and exact typed predecessor handoff
  -> exact identity single-flight and early admitted hit
  -> complete executable-closure classification
  -> deterministic serialized closure product
  -> close-before-external-publication
  -> child verification and atomic root commit
  -> root read-back and owner re-admission
  -> fresh-process artifact-only execution-worker launch
  -> source-absent behavior equivalence
  -> selected-source/path worker route hard deletion
  -> terminal cleanup and negative API proof
```

Optionality applies only to additional persisted product families and
operational replication/retention policy. Immediate typed handoff,
single-flight, zero derivation after an exact admitted hit, off-critical-path
cache persistence, atomic publication, re-admission, and artifact-only launch
are mandatory for the final cut.

The current tree may still contain redundant compilation as incomplete
implementation evidence, but no law preserves it. Wave 0 does not wait on the
unlanded portable schema; W1-14 and W1-15 do not inherit a worker compile route
or compatibility obligation from that sequencing.

Acceptance requires evidence that two independent closures compile
concurrently and an early closure executes while a later closure is still
compiling; long chains/wide fans and slow low-index products never impose
deterministic-commit or unrelated waits; same-identity work derives once and an
exact hit derives zero times; slow/failing cache persistence never delays
handoff; spawn failure, panic, watchdog, disconnect, retry, cancellation,
ancestor failure, unwind, `Drop`, and shutdown settle all custody; crash before
root commit, corrupt/missing children, and admission version mismatch mint no
receipt; shuffled schedules produce identical roots and behavior; hostile
authored depth succeeds through production, serialization, admission,
diagnostics, and teardown on a 128 KiB stack; a fresh execution worker runs
callable/entry/actor/protocol/operation/provider/cleanup/continuation/budget and
checkpoint behavior with source, manifests, compiler cache, and compiler
libraries absent; and the obsolete source-path worker compile route and every
RAW reconstruction aperture have zero callers.

After the hard cut, execution workers receive only the sealed admitted complete
artifact receipt plus launch-environment authority. There is no compatibility
sum, recovery compile, or runtime source fallback. Python, Go, and WASM host
implementations remain required future host work and are not prerequisites for
this Rust-owned dependency map; the artifact and semantic contracts must remain
portable to them.

## Language And Compiler Roadmap

| Area | Canonical contract | Durable outcome |
| --- | --- | --- |
| Package, module, and import resolution | [Source Entrypoint Package Resolution And Import Binding Law](SOURCE_ENTRYPOINT_PACKAGE_RESOLUTION_LAW.md) | One compiler-root-owned, private, non-closable same-epoch package/import subtransaction; exactly six move-only parser roles; `ResourceImport { Required \| Optional }` authored before resolution; typed relative/package target validation into `ResourceBinding`; exact child-program universes; checker-owned callable/data classification; and no public receipt ladder, role reclassification, or downstream route reconstruction from names or manifests |
| Language hardening | [Language Improvements](LANGUAGE_IMPROVEMENTS.md) | Must-use and affine authority, typed Result handling, closed errors, contract obligations, managed cleanup, totality, exact value laws, durable error provenance, and boundary diagnostics |
| First-class graph-value composition — **broken until the ADR-2216 gates pass** | [Nominal Node/Graph Hard Cut](ADR-2216-NOMINAL-NODE-GRAPH-HARD-CUT.md), [Authored Orchestration](done/ADR-SECV2-027-SEQUENTIAL-BY-DEFAULT-AUTHORED-ORCHESTRATION-AWAIT-PARALLEL-AND-RACE.md), and [Graph Function Instantiation](done/ADR-1662-GRAPH-FUNCTIONS-INERT-GRAPH-INSTANTIATION-START-AWAIT-SINKS-LAUNCHED-RESULTS-AND-EXECUTIONREF-ADMINISTRATION.md) | One canonical opaque `Node<T>` plus nominal `Graph<T> <: Node<T>`; graph-function invocation produces Graph while ordinary staged calls produce Node; only Graph enters materialization, reconciliation, deployment, and graph-only inspection; both compose through ordinary functions, `await`, `with`, `parallel`, and `race`; structural Graph/`Ctx`, PromiseLike/thenables, public `Slot`/`fill`, duplicate graph aliases, and graph-authority projection re-entry are deleted without compatibility paths |
| Absence | [Absence And Option Design](ABSENCE_AND_OPTION_DESIGN.md) | Authored `Option`, no authored `null` or storable `undefined`, `void` as no-output type, and explicit boundary `JsonField` only when three states matter |
| Recursion | [Recursion Design](RECURSION_DESIGN.md) | Ordinary recursion, proper tail calls, TRMC, scheduler visibility, metering, checkpointing, and GC safety |
| Execution budgets | [Budget Design](BUDGET_DESIGN.md) | Program-declared requirements and monotone attenuation over host-minted conserved grants |
| Compiler stack safety | [Compiler Stack-Safety Design](COMPILER_STACK_SAFETY_DESIGN.md) | Wave-0 closure over shallow admitted syntax; one phase-specific total heap machine per authored-depth-sensitive semantic SCC; sealed request/result boundaries with private internal transitions; existing content-addressed identity for legal fixpoints and typed illegal-cycle refusal; iterative construction, recovery, traversal, lowering, checking, evaluation, fault rendering, and destruction of valid/refused/partial work; one bounded fail-closed compiler-worker supervision boundary; complete `stacker` removal; and no fixed authored nesting cap |
| Compiler product custody verification | [Compiler Product Custody Verification Design](PRODUCT_CUSTODY_VERIFICATION_DESIGN.md) | Wave-0 reviewed coverage of move-only compiler products through one root epoch, explicit parallel worker-scope leases, exact owner-typed family keys, direct per-family node state, and owner-local source coverage declarations; no duplicate mint, double consumption, silent drop, omission, or incomplete split/join lineage; a private clean root close after exact checked-image/direct-run preparation and before `SourceEntrypointDirectRunPreparedRuntime` escapes; future portable Merkle-envelope publication adopts the same close-before-escape gate; no Wave-0 trace or correctness-observation adapter; explicit partial rollout without a complete-coverage claim; and byte-identical products, Merkle roots, and executable identities with verification enabled or disabled |
| Semantic compiler convergence | [Typed Continuation Demand Spine](ADR-2209-TYPED-CONTINUATION-DEMAND-SPINE.md), [Unified Flow-State Checker](ADR-2210-UNIFIED-FLOW-STATE-CHECKER-SUBSTRATE.md), and [One Expression Evaluator](ADR-2211-EVALUATOR-CONVERGENCE-ONE-EXPRESSION-EVALUATOR.md) | One checker flow-state substrate, typed continuation/effect handoff, checker-owned binding identity, exact statement-owned semantic products, one context-complete checked-expression-use algebra, one occurrence-directed selected-expression evaluation product with lazy leaf ownership, fail-closed exact checked dispositions, one exhaustive compute-operation emitter, and deletion of syntax-derived semantic fallback, the recursive provider-frame interpreter, positional attachment authority, duplicated syntax walkers, and parallel expression algebra |
| Contracts and polyglot FFI | [TSON Contract And FFI Law](TSON.md), [Capability Contract TSON Authority](ADR-2157-CAPABILITY-CONTRACT-TSON-AUTHORITY-VS-CHECKER-SURFACE.md), and [Language Improvements](LANGUAGE_IMPROVEMENTS.md) | Declaration-style `.contract.ts` is the sole authored contract role. The compiler owner joins one exact demanded declaration member to typed normalized TSON facts before boundary/provider admission. `.tson.ts`, `defineContract(...)`, `toContractTson`, runtime-valued builders, normalized-JSON admission, source regeneration, public selector/parts/binder ladders, and route fallback are hard-cut with no compatibility reader. The contract remains language-neutral without making a general TypeScript SDK or future host binding a current deliverable. |
| Typed view expressions | [SSX](SSX.md), [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md), and [Terminal Surface Profiles](TERMINAL_SURFACE_PROFILES.md) | Native SSX in ordinary `.ss` files, checked view purity, typed intrinsics/components, explicit surface refinement, multiple pure view projections over one semantic model, renderer-neutral `ViewTree` lowering, and a shared terminal core with honest sibling inline/fullscreen surfaces |
| Syntax and lowering coverage | [ADR-2091](ADR-2091-SWARMSCRIPT-SYNTAX-INVENTORY-AND-CONFORMANCE-COVERAGE.md) | Every admitted form has positive behavior evidence, every forbidden form has a stable negative, and destructuring, defaults/rest, arrows, calls, parallel/race, events, results, module initialization, and other executable forms reach one checked lowering path |
| Negative diagnostic integrity | [Negative Test Expectation Integrity](NEGATIVE_TEST_EXPECTATION_INTEGRITY.md) and [ADR-2099](ADR-2099-SS-TEST-BUN-STYLE-CONFIGURED-SUITES-AND-EXPECTATIONS.md) | Every configured negative fixture is joined to its intended terminal phase and stable public fault code; phase/code movement, missing expectations, and unexpected passes fail the suite instead of allowing an unrelated rejection to count as green |
| Compiler-product reuse | [Incremental Compilation](INCREMENTAL_COMPILATION.md) | Existing content-addressed sealed products, with early lookup, owner re-admission, per-export interface identity, front/back scheduling, typed Merkle storage, precise invalidation, cache validation/retention/GC, and redundant-work removal completed across `swarm` and `swarm test` |
| Normative language specification | [SwarmScript Language Design Specification Spine](future/ADR-2020-SWARMSCRIPT-LANGUAGE-DESIGN-SPECIFICATION-SPINE.md) | Assemble the grammar, resolution, type, control-flow, boundary, contract, effect, actor, carrier, diagnostic, and conformance laws into one normative specification without replacing the concise primer or detailed design owners |

The stack-safety representation is a prerequisite for safe recursion checking
and other depth-sensitive SwarmScript compiler algorithms. Its migration must
preserve the existing Level-4 product identities, admission boundaries, and
reuse laws; it is not a prerequisite for, or implementation of, the Level-5
compiler. Budget and recursion semantics share safepoints and continuation
state, but neither is implemented as compiler stack protection.

The ADR-2216 graph hard cut is two complete owner migrations, not a contract
alias cleanup. The type owner must introduce the nominal Graph refinement across
type resolution, assignability, normalization, substitution, hashing, stable
snapshots, diagnostics, checked-expression use, and iterative teardown before
the public contract stops representing Graph structurally. The selected-work
and runtime owners must then require Graph at every graph-only sink and delete
the complete `GraphFill*` checker, lowering, image, VM, runtime, and fixture
family. The negative TDD fixtures intentionally make the current unlawful
acceptance visible. No optional brand, structural union, sink-local provenance
guess, temporary alias, or compatibility `fill` wrapper closes this row.

## Runtime And Artifact Roadmap

| Area | Canonical contract | Durable outcome |
| --- | --- | --- |
| Primitive operations and runtime flow | [Operation Execution Design](OPERATION_EXECUTION_DESIGN.md) and [Unified Flow-State Runtime Substrate](ADR-2214-UNIFIED-FLOW-STATE-RUNTIME-SUBSTRATE.md) | `Operation<Event, Output>`, one running/parked/completed/cancelled lifecycle, durable nonblocking yields, replay-stable step identity, exact park/resume ownership, cooperative cancellation, observation methods, exact result or typed-fault settlement, and fail-closed dispatch that cannot turn missing/mismatched authority into fallback output or effects |
| Executable artifacts | [Executable Artifact Lifecycle Design](EXECUTABLE_ARTIFACT_LIFECYCLE_DESIGN.md), [Incremental Compilation](INCREMENTAL_COMPILATION.md), and [Compiler Product Custody Verification](PRODUCT_CUSTODY_VERIFICATION_DESIGN.md) | After Wave-0 custody freeze, streaming content-addressed compiler-product DAGs prepare deterministic serialized closures before close, then publish immutable executable-closure images atomically after close; owner-admitted dependency products immediately release downstream compiler work, while only a read-back and re-admitted executable-closure receipt releases runtime admission; callable/actor compute is preserved; fresh workers admit and activate images without source; cold/warm restoration and checkpoint-image correspondence remain fail-closed; no partial artifact, duplicate same-identity compilation, dual source/artifact execution path, or execution-time authority reconstruction remains |
| Process sessions and lifecycle | [Process Run/Invoke API](done/ADR-1853-PROCESS-RUN-INVOKE-API-AND-START-AWAIT-EXPORTS-DELETION.md), [Process Liveness And Explicit Shutdown](done/ADR-1854-PROCESS-LIVENESS-OWNED-HANDLES-AND-EXPLICIT-SHUTDOWN.md), [Process Ontology](distilled/ADR-1771-PROGRAM-ARTIFACT-HOST-LOCAL-PROCESS-AND-CHECKPOINT-ONTOLOGY.md), and [Public Process Wait Retirement](done/ADR-1614-EXECUTION-REF-CAPABILITY-FAMILY-PROCESS-HANDLE-INTERNALIZATION-AND-PUBLIC-PROCESS-WAIT-RETIREMENT.md) | Opaque `Program`, host-local `Process`, and durable `Checkpoint` authority; `run` for lifecycle launch, `invoke` for typed selected-entry work, exact terminal/liveness observation, restore/checkpoint and typed control, entry completion distinct from process termination, and no public `process.wait`, detach folklore, or raw-handle reconstruction |
| Execution-context observations | [No Ambient `sys`](ADR-1498-NO-AMBIENT-SYS-EXPLICIT-CAPABILITY-ONLY-EXECUTABLE-SURFACES-AND-TINY-LANGUAGE-PRIMITIVE-SET.md), [Current-Node Intrinsic](distilled/ADR-1699-CURRENT-NODE-INTRINSIC-TSOP-VOCABULARY-RETIREMENT-AND-STDLIB-WRAPPER-HARD-CUT.md), and [Rust SDK Host Capability Linker](ADR-1805-INTERNAL-RUST-SDK-HOST-CAPABILITY-LINKER-PROOF.md) | `current_node()` and sibling immutable execution observations derive from the admitted active VM context without provider dispatch or ambient host state; node-local lifecycle commands remain separately scoped authority, and observations grant no discovery, retargeting, checkpoint, or control authority |
| Actor isolation | [Swarm Actor Isolation](SWARM_ACTOR_ISOLATION.md) | Owner-isolated actor cells, exact message/turn settlement, suspension-safe mutation and input binding, typed scheduler faults, lifecycle observation, hibernation/wake, fault/cleanup settlement, and no raw actor authority reconstruction |
| Runtime heap and deterministic collection | [Heap-Backed VM Values And Tracing GC](done/ADR-1991-HEAP-BACKED-VM-VALUES-JS-OBJECT-IDENTITY-AND-TRACING-GC.md) and [Swarm Actor Isolation](SWARM_ACTOR_ISOLATION.md) | Iterative cycle-safe tracing from exact instruction/continuation/mailbox roots; deterministic safe-point collection; owner-driven terminal side-store retirement; actor-local heap/root domains with copy-or-admit cross-actor transfer; and fail-closed checkpoint restore until every embedded authority and stored continuation handle has durable rebind authority |
| Execution backend portability | [Swarm Actor Isolation](SWARM_ACTOR_ISOLATION.md) | One semantic instruction/runtime contract across Rust-direct and WASM execution, with admitted backend eligibility, safepoints, roots, stack maps, code products, and differential conformance; Cranelift JIT and native AOT breadth remain optional performance work |
| Protocol sessions and liveness | [Protocols](PROTOCOLS.md) | Authored state/event/action/terminal interaction over compiler-first installed-definition correspondence and nine sealed registered-session families; `DirectRunRuntimeAuthorityOwner` owns registry/route/effect settlement, private consuming `ProcessSessionV0` operations own turns, and iterative shutdown covers refusal, retry, recovery, child trees, checkpoint, cancellation, unwind, diagnostics, and destruction |
| Scoped resources and managed cleanup | [Scoped Resource Control Syntax](ADR-1611-SCOPED-RESOURCE-CONTROL-SYNTAX-DOCTRINE-WITH-AND-TRANSACTION-AS-REGION-OWNERS.md), [Completion-Aware Managed Cleanup](done/ADR-1818-COMPLETION-AWARE-MANAGED-REGION-CLEANUP.md), [With-Owned Resource Policy](ADR-1970-WITH-OWNED-SCOPED-TRANSACTION-RESOURCES-AND-SUSPENSION-POLICY.md), and [Structured Resource Lifetime](done/ADR-1993-STRUCTURED-RESOURCE-LIFETIME-FOR-WITH-REGIONS-AND-DURABLE-RELEASE-OBLIGATIONS.md) | `with` as the single lexical lifetime owner, typed resource acquisition, source-order acquire and reverse-order release, admitted suspension policy, completion-aware cleanup across every exit, checkpoint/rebind continuity, and exactly one durable release-obligation path without duck-typed disposal or GC finalization; managed finalization also receives one opaque observation of the exact completed dynamic region, including nested staged work and fault/cancellation settlement, without gaining execution authority |
| Opaque authority | [Opaque Authority Carriers](OPAQUE_AUTHORITY_CARRIERS.md) and [Authored Capability Composition And Attenuation](AUTHORED_CAPABILITY_COMPOSITION_AND_ATTENUATION_DESIGN.md) | Non-forgeable, non-reconstructible, one-shot carrier and owner-operation boundaries; lexical narrowing remains non-minting, while owner-minted bindings/grants may carry exact authored or native capability authority |
| Actor graphs and replanning | [Actor Graph Interaction](ACTOR_GRAPH_INTERACTION.md) | Checked graph composition, staged graph invocation, exact keyed reconciliation, owner-issued snapshot/plan identity, running/suspended-work control, output fencing, append-only history, and owner-governed replanning |
| Hot reload and live upgrades | [Hot Reload As Replan](ADR-2212-HOT-RELOAD-AS-REPLAN.md) and [Live Program Upgrades](ADR-2213-LIVE-PROGRAM-UPGRADES-VERSIONED-UNITS-AUTHORED-MIGRATIONS-AND-MESH-SKEW.md) | Reload by replanning with a newly admitted image under the one plan-epoch fence; content-addressed version lineage, total typed atomic migrations, bounded image residency, session/actor continuity, replay, and mesh-skew disposition for upgrades |
| Runtime bindings and activation | [Runtime Provider And Backend Activation](RUNTIME_PROVIDER_AND_BACKEND_ACTIVATION_DESIGN.md) | Program-selectable opaque bindings, environment catalog admission, demand-driven libbun/PGlite/Postgres/renderer activation, exact terminal-profile leases and restoration, and typed platform faults without language editions or silent fallback |
| Provider execution and SDKs | [Provider Execution And SDK Law](PROVIDER_EXECUTION_AND_SDK_LAW.md), [TypeScript SDK Refactor](TYPESCRIPT_SDK_REFACTOR.md), [Rust Capability SDK](ADR-2034-RUST-CAPABILITY-SDK-EXACT-PROVIDER-ABI-AND-CROSS-HOST-CONFORMANCE.md), [Rust Provider Host Spine](ADR-2037-RUST-SDK-PROVIDER-HOST-SPINE-AND-NATIVE-BINARY-MANIFEST.md), and [Loadable Native Providers](ADR-2042-LOADABLE-NATIVE-PROVIDER-PACKAGES-AND-CONTRACT-DERIVED-MANIFESTS.md) | `Phase A → Phase B → Phase C`: linearly move exact package implementation lineage before route sharing; consume one generative occurrence cohort through the finite ISA allocator into a complete child-forest/lifecycle precommit owned by the canonical compiler transaction; then perform one zero-work final executable exposure, one complete-runtime `Arc`, and one ordinary/static-child consuming dispatch algebra. Mechanical Rust SDK/static, loaded-native, and existing libbun-backend migration follows only after that SCC closes and consumes the compiler-owned declaration-contract member; it does not schedule a general TypeScript SDK. TLS, string faults, empty/default settlement, DTO/parts/raw `ImportId`, precommit `Arc`/clone/`OnceLock`, family/site selectors, legacy `defineContract`/normalized-JSON authority, and second commits are forbidden. Python, Go, general TypeScript SDK, and WASM-host implementations remain required future compatibility destinations, while their implementation is outside current acceptance gates. |
| Authored capability implementations and attenuation | [Authored Capability Composition And Attenuation](AUTHORED_CAPABILITY_COMPOSITION_AND_ATTENUATION_DESIGN.md) | After exact contracts/TSON, opaque authority, and provider admission/execution exist, SwarmScript functions, operations, actors, protocols, and composites implement the same capability contracts as current installed implementations; authored code owns decomposable validation and attenuation, exact owners mint bindings/grants, receipts remain honest, and dynamic delegation/currentness/recovery compose later without gating a fixed-portfolio harness |
| Mesh authorization, native transport, and remote grants | [Mesh Native Transport Repair](MESH_NATIVE_TRANSPORT_REPAIR.md), [Mesh And Protocol Authorization](ADR-2215-MESH-PROTOCOL-AUTHORIZATION-OBJECT-CAPABILITY-MODEL.md), [Mesh Identity Evidence](ADR-2092-MESH-IDENTITY-EVIDENCE-CORE.md), [SwarmScript Mesh API](done/ADR-2113-SWARMSCRIPT-MESH-API-AND-CONNECTION-CONTROL-CAPABILITIES.md), [Persistent Mesh Associations](done/ADR-2123-PERSISTENT-MESH-ASSOCIATION-LIFECYCLE.md), and [Mesh Capability Service](done/ADR-2124-MESH-CAPABILITY-SERVICE-SUBSTRATE-AND-FINITE-OWNER-OPERATIONS.md) | Identity remains distinct from authorization; real independently owned listener/dialer channels feed receiver-owned admission and ordinary destination actor/provider/protocol dispatch; destination-issued settlement is distinct from transport acknowledgement; owner-controlled attenuation/export, destination admission, release, persistence, revocation, reconnect, and exact remote use carry opaque grants over actual post-handshake association authority without public grant encoding syntax or DTO reconstruction |
| View mounting and rendering | [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md) and [Terminal Surface Profiles](TERMINAL_SURFACE_PROFILES.md) | Pure renderer-neutral trees joined to live protocol affordances only by a finite mount owner; each selected client/view projection has its own checked plan and mount epoch, while terminal inline/fullscreen profiles use exact bindings, profile-local mechanics, and new-epoch transitions |

For process authority, the later ADR-1853/ADR-1854 `run`/`invoke`, terminal,
and liveness law is authoritative wherever older load/start/wait inventories
conflict. ADR-1614 remains the explicit hard cut against public
`process.wait(...)`, public detach folklore, and reconstructed live handles; it
does not revive its older package-family migration direction over the later
process contract.

## Application And Host Verticals

Application surfaces consume the language/runtime contracts above. They are
not prerequisites for calling the core SwarmScript language complete.

The durable dependency shape is:

```text
shallow syntax (including SSX) -> SSX checker/ViewTree lowering
protocol sessions              -> mounted-view/affordance owner
runtime binding activation     -> platform provider/renderer catalogs
TSON + provider activation     -> semantic datastore and provider packages
protocol/actor authority + an explicit host binding
                              -> future local HTTP ingress and API frontdoors

TSON + provider activation/execution + Operation
                              -> normalized model exchange
normalized model exchange     -> deterministic scripted model fixtures
contracts + opaque authority
+ operations/protocols         -> typed tools
contracts/TSON + opaque authority + provider admission/execution
                              -> authored SwarmScript capability implementations
+ operations/actors/protocols -> composite capability services and static
                                 semantic attenuation
agent definition + exact native AgentProgram realization
+ admitted native model binding
+ one exact fixed admission-time opaque tool/resource/budget portfolio
+ exact required native profile-feature evidence
+ freshness/continuity/durability/evidence policies
                              -> sealed agent execution admission
model exchange + execution admission + actors
+ ProtocolSession              -> tool-free native general-agent harness
native harness + typed tools
+ fixed admitted portfolio     -> typed-tool-loop expansion
typed-tool loop + exact remote provider binding
+ SwarmScript-free capability host
                              -> local-vs-Mesh-remote tool placement proof
typed-tool loop + generated
protocol client projection     -> embedded client + thin CLI/IPC
protocol client projection + typed ProductUiModel reducer
+ checked SSX/ViewTree/affordance plan + mount owner/terminal renderer
                              -> TUI
typed-tool loop + admitted workspace/file/process/patch/test capabilities
                              -> minimal local coding-agent profile
                              -> useful native coding harness
live conversation owner
+ sealed attachment admission -> attach under the same activation epoch
conversation owner + portable committed-conversation journal
+ exact terminal revision + exclusive-currentness/fencing evidence
+ continuity policy           -> one-shot committed-continuation admission
continuation admission
+ compatible target execution admission
                              -> same logical identity under a fenced successor
                                 epoch; the next turn is fresh
working native harness + actor/protocol supervision
+ scoped child admissions + budget attenuation
                              -> fresh child-agent composition
fresh child composition + governed purpose-scoped observations
                              -> independent TUI/supervisor/peripheral cursors
                                 + bounded context accumulator/materialization
+ assignment-derived help routing
                              -> nested typed HelpSessions with monotone
                                 disclosure/request/budget/deadline/hop scope
source conversation owner + expected committed revision
                              -> one-shot committed-boundary fork snapshot
fork snapshot + fresh target execution admission
+ explicit memory/workspace/context/durability scopes + InitialContextPolicy
                              -> source-preserving fork with new identity/epoch
                                 and no copied live authority
publication/run provenance
+ authorized historical read -> OBS selection of exact run/revision + mode
chosen mode + normal sealed recovery/fork/seed-derivation admission
                              -> exact restore then settle/fork, semantic fork,
                                 or explicitly lossy seed inquiry
Swarm runtime checkpoint + runtime event journal
+ Swarm-owned admitted durability binding
+ selected backing resource (optionally Hive-materialized)
+ local recovery admission/re-admitted bindings
+ exact local subordinate settlement/reconciliation
                              -> exact local restore under a fenced successor
                                 epoch
agent/tool/child/session protocols
+ Mesh grants                  -> location-transparent model, tool,
                                 child-agent, and session placement
Mesh placement + local recovery law
+ sealed subordinate checkpoint graph
+ re-admitted remote bindings -> exact distributed restore under a fenced
                                 successor epoch
durable conversations
+ semantic receipt continuity
+ bounded peripheral selection/omission/redaction/gap receipts
                              -> memory-integrated/compiler-grade agents
AgenticFunctionSpec
+ finite RealizationPortfolio
+ QualificationSuite + exact evaluation-world/effect admission
                              -> owner-bound evaluation targets
                              -> authored graph execution
                                 + receipt-bound capture
                              -> admitted evidence
                                 + case/comparison records
                              -> qualified realization portfolio
current qualified portfolio
+ role/profile/safety/effect/availability/budget policy
                              -> one-shot selected qualified realization
                                 + selection receipt
typed decision candidate + actor/protocol decision owner
                              -> durable DecisionCase/DecisionSession
                                 + exact decider actions and timeout disposition
+ reactive dependency/assumption footprint
                              -> dependent-only DecisionPrerequisite fence
                                 + exact settlement-triggered recheck/replan
+ Mesh identity/authorization + partitioned owner journals
                              -> multi-human distributed coordination
                                 without global transcript/total order
Orchid AgentDefinition<OuterProfile> selecting an authored
  AgentProgram<OuterProfile>
+ exact AgentExecutionAdmission<OuterProfile>
+ child execution admissions and/or exact qualified AgenticFunction bindings
+ reusable reactive-compiler library
+ Orchid concrete compiler/domain products and verification policy
                              -> virtual agent exposing the same admitted
                                 Profile through an outer
                                 ProtocolSession<AgentTurn<Profile>>
admitted evidence/currentness/decision receipt + SemanticWorkKey
+ exact dependency/assumption/decision footprint
+ opaque RunningWorkBinding set + base generation + strategy
                              -> Candidate<{closed dispositions, future patch}>
candidate + verification
+ graph/frontier current state
                              -> compare-and-swap reservation/fence
                              -> owner-sealed ReconciliationReservation
reservation + exact bindings  -> owner RuntimeControlTicket only for each
                                 disposition that changes live work
                              -> RuntimeControlReceipt set
                              -> reservation finalization/requeue/conflict
                                 + durable journal
                                 + bounded redacted replay-stable explanation OBS
capability decision + exact assignment/dependency impact
+ graph-owner reservation     -> successor assignment/capability epoch;
                                 old attempts never widen retroactively

executable artifact + Hive
deployment protocol            -> integrated `ss deploy` product front door
                              -> local, self-hosted, or managed Hive binding

SSX + mounted views            -> SwarmWeb trusted renderer
SwarmWeb + browser catalog     -> browser SwarmScript/WASM placement
surface-specific projections
+ shared terminal-core/sibling-profile law
+ same ViewTree/mount contract -> inline/fullscreen TUI and markup/email
                              -> optional later Tauri, Swift/native, other apps
```

The graph above describes the active native path. A future compatibility project
may add a closed normalized foreign driver and private adapter realization behind
the already-stable public profile protocol, with explicit fidelity reporting.
That possibility does not add an active prerequisite, package, wave, or shared
conformance gate.

| Vertical | Design owner | Roadmap placement |
| --- | --- | --- |
| Future HTTP ingress binding | Gateway and host boundary law | SwarmScript has no special `route` declaration or compiler-owned route catalog. A future explicit capability binding must consume checker-selected exact `ActorRef` member/arm authority and produce a sealed host-owned binding; the current `ref.arm(...)` surface is an execution sink, not an existing first-class arm value. Typed boundary decoding and the framework-neutral `swarm-web-wire` transport remain reusable and independent of public Gateway deployment. |
| Semantic Datastore and GraphStore | [Semantic Datastore Authority](done/ADR-2129-SWARMSCRIPT-SEMANTIC-DATASTORE-AUTHORITY-DUMB-PIPES-AND-BACKEND-BINDING.md), [Datastore Migrations And Transactions](done/ADR-2130-SWARMSCRIPT-OWNED-DATASTORE-MIGRATIONS-BACKEND-ADMISSION-TRANSACTIONS-AND-SAGAS.md), [GraphStore Substrate](ADR-2143-GRAPHSTORE-SUBSTRATE-SCOPE-AND-DATASTORE-BACKEND-SPLIT.md), and [GraphStore Feature Closure](ADR-2159-GRAPHSTORE-DATASTORE-BACKEND-FEATURE-CLOSURE.md) | Package-owned `.ss` datastore/graph semantics, migrations, operations, projections, transactions, branches, and sagas over Rust-owned backend admission, resource, journal, replay, and receipt authority; reads may bind to an exact opaque branch head and preserve that head plus their read footprint so multi-read decisions are reproducible; PGlite/Postgres remain selected implementations. Concrete planning/product schemas such as Orchid Workspec remain application-owned projections and governed commands over admitted product state, not Swarm or universal Swarmlib domain authority. |
| Terminal capability renderer | [Terminal Surface Profiles](TERMINAL_SURFACE_PROFILES.md), [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md), and [Runtime Provider And Backend Activation](RUNTIME_PROVIDER_AND_BACKEND_ACTIVATION_DESIGN.md) | First capability-renderer vertical: Swarmlib may implement exact Ratatui inline and fullscreen bindings over the common mount law alongside SSX substrate work; it does not wait for Axum/SwarmWeb, and it keeps Taffy/layout, native scrollback emission, managed scroll, hit testing, and terminal-mode mechanics renderer-private |
| SwarmWeb server-owned application path | [SwarmWeb](future/SWARMWEB.md) | First web renderer/product vertical after protocol views and mounted-view law; does not wait for browser SwarmScript or block the terminal renderer vertical |
| Trusted SSX renderer | [SSX](SSX.md) and [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md) | Follows core SSX lowering and consumes the common ViewTree/surface/mount contract; products may select distinct web and non-web projections |
| Browser SwarmScript/WASM host | [SwarmWeb](future/SWARMWEB.md) | Later execution-placement vertical for browser-local protocols, actors, and signals; not an SSX grammar prerequisite and not a WASM/component provider-host milestone |
| Native agent programming substrate | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | First-party SwarmScript `AgentDefinition<Profile>` stored programs, sealed native `AgentExecutionAdmission<Profile>`, live `ProtocolSession<AgentTurn<Profile>>` sessions, normalized model exchange, deterministic scripted-model fixtures, and typed tool admission/settlement over existing actor/protocol/provider laws; first prove one memoryless tool-free turn, then a typed-tool loop over one exact fixed admission-time capability portfolio |
| Capability-only host and remote tool placement | [Capability Host Deployment And Tool Placement](CAPABILITY_HOST_DEPLOYMENT_AND_TOOL_PLACEMENT_DESIGN.md) | Parallel proof after typed-tool admission stabilizes: the same exact tool contract executes locally and through a SwarmScript-free Rust Mesh capability host with exact contract/grant/currentness admission, destination-issued settlement, typed uncertainty, reverse-authority negatives, and a dependency-closure proof excluding the compiler, VM, session runtime, and native harness. This is topology compatibility, not a day-one production deployment system; Hive later materializes and composes brain, hands, and control-surface resources. |
| Generated native client | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | An embedded client plus thin CLI/IPC protocol smoke surface immediately after the typed-tool loop; every projection consumes the same generated protocol semantics and cannot hydrate action/session authority. A TUI additionally consumes the typed ProductUiModel reducer, checked SSX/ViewTree affordance plan, mount owner, and terminal renderer; no client waits for unrelated children, durability, Hive, memory, or Mesh work |
| Minimal local coding profile and harness | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | One exact fixed admission-time portfolio of capability-scoped workspace, file read/search, patch/diff, bounded command/process, diagnostic, test, artifact, approval, sandbox-refusal, and cleanup mechanics composed over the working typed-tool loop; dynamic authority requests and authored composite libraries are later expansions, and this baseline does not require child agents, isolated child workspaces, checkpoint recovery, Mesh, or semantic memory |
| Committed conversation journal and resume | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | The portable committed-conversation journal remains owner-distinct from runtime checkpoints, semantic memory, and evidence; live attach stays on the same owner/epoch, while owner-admitted committed-boundary continuation consumes exact terminal revision plus exclusive-currentness/fencing evidence and starts a fresh next turn under one successor epoch without restoring in-flight authority |
| Fresh child composition | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | After the useful single-agent coding harness, exact one-shot child selection/spawn, scoped child capabilities and isolated workspaces, hierarchical budgets, typed fan-in, structured cancellation, and one typed child-session terminal extend the same public profile; no fork, checkpoint, or Mesh dependency is implied |
| Governed peripheral context and hierarchical help | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Later composable branch over actors/protocols, fresh children, and governed observation: independent TUI/supervisor/context grants and cursors; bounded source-coverage/omission/redaction/gap rollups; receipt-bound context materialization; and typed nested `HelpSession`s with assignment-derived routing and monotonically attenuated disclosure, authority-request, budget, deadline, and hop scope |
| Committed-boundary fork and provenance | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Owner-sealed fork consumes the exact committed revision, fresh target admission, scopes, and context policy; leaves the source live; copies no pending action, capability, mutable workspace, budget, or other live authority; and mints a new identity/epoch. Contribution provenance and historical selection remain OBS, and every semantic fork, exact restore, or lossy seed inquiry consumes its own sealed admission |
| Local agent recovery | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Swarm runtime checkpoint/event-journal recovery first proves local predecessor fencing, re-admitted bindings, exact local subordinate settlement/reconciliation, successor-epoch one-shot authority, and no duplicated effect, process, tool, or child work; it remains distinct from committed conversation continuity and does not wait for Mesh or a distributed checkpoint graph |
| Mesh agent placement | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | After local semantics and recovery work, Mesh-transparent model, tool, child, and session placement preserves the same facade, authority, cancellation, and destination-settlement laws; this slice does not claim distributed checkpoint recovery |
| Distributed agent checkpoint graph and recovery | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | After Mesh placement, a sealed distributed checkpoint graph may publish only when every subordinate is terminal, durably parked, or transferred exactly once, with deduplication, cursor/grant currentness, cleanup, and old-epoch fencing restored before dispatch |
| Receipt-bearing agent memory and context | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Later composable branch over durable conversations plus the existing Graphstore/Memory/Retrieval/GraphRAG receipt chain and bounded peripheral accumulators; source coverage, selected and omitted material, redaction, replay gaps, currentness, and exact context consumption remain receipt-bound, and transcript text or model summaries never become semantic truth by proximity |
| Agentic-function evaluation, qualification, and realization selection | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Later composable branch over first-class graph combinators, exact settled run receipts, managed-region observation, and head-bound evidence reads, independent of memory or Mesh unless a suite selects them: ordinary authored graph functions express repeated execution, comparison, judging, and selection; Swarmlib records receipt-bound evidence and provides qualification mechanics without a second workflow or query language; domain owners retain suites, meanings, gates, and output admission |
| Human decision work and distributed coordination | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Later compatible branch from one local typed pending action to actor/protocol-owned `DecisionCase`/`DecisionSession` queues, exact decider action authority, explicit per-assignment timeout and root fallback/terminal settlement, and qualified decision descriptions; reactive integration fences only exact dependents, while later Mesh/multi-human coordination uses partitioned owner journals and causal receipts without a global transcript, total order, or Git-only state |
| Reactive virtual-agent control | [Native Agent Programming And Harness Design](NATIVE_AGENT_PROGRAMMING_AND_HARNESS_DESIGN.md) | Later composable branch over the native outer profile plus child admissions and/or exact qualified `AgenticFunction` bindings and the reusable reactive-compiler library: stable owner-minted semantic work keys, typed assignments and sealed running bindings, evidence/currentness/decision-driven impact over exact dependency/assumption footprints, capability successor epochs, decision prerequisites, graph/frontier compare-and-swap reservation before runtime control, one-shot safe-boundary tickets, affected-region fences, durable owner-partitionable reconciliation journals, and redacted explanation behind the same outer agent profile |
| Durable Level-5 reconciler bootstrap | [Hive](HIVE.md) | After the required provider/resource/datastore and protocol substrate, phase-one Hive is the first deterministic Level-5 reconciler and gates durable Orchid semantic-state integration; it does not gate memoryless clients or local SSX/renderer development |
| Integrated deployment front door | [Hive](HIVE.md) and [Executable Artifact Lifecycle](EXECUTABLE_ARTIFACT_LIFECYCLE_DESIGN.md) | `swarm` is the canonical product front door: build/publish remains compiler/artifact ownership while plan/deploy/status/logs consume public Hive protocols; managed Swarm/Hive Cloud may be the polished first-party default binding, but receives no private language semantics or authority unavailable to conforming local, self-hosted, or enterprise Hive implementations |
| Public SwarmWeb deployment | [Gateway](GATEWAY.md) and [Hive](HIVE.md) | Gateway gates public ingress/certificates and Hive gates placement and durable environment binding; neither gates local SSX or renderer development |
| Electroswarm/Tauri | [Capability Host Deployment And Tool Placement](CAPABILITY_HOST_DEPLOYMENT_AND_TOOL_PLACEMENT_DESIGN.md), [Electroswarm](future/ELECTROSWARM-TAURI-DESKTOP-CAPABILITY-SHELL.md), and [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md) | Enabled downstream future, not immediate work; one interactive capability-shell and renderer instantiation over the generic tool-placement boundary. It consumes the same semantic product model and common SSX/ViewTree/surface/mount law as TUI, browser, and later native projections; it may run capability-only with no local SwarmScript or deliberately bundle a sidecar without defining another application mode. |
| Swift/native application renderer option | [View Tree And Renderer Design](VIEW_TREE_AND_RENDERER_DESIGN.md) | Optional future consumer, like Electroswarm; not immediate roadmap work or a core acceptance gate |

## Extracted Language Audit

The retired queue contained a late language-audit wave whose decisions are now
canonical. Their durable homes are:

- classic C-style `for` retirement, Python-compatible `range(...)`, and
  first-class integer ranges over the canonical iterable substrate:
  [Classic `for` Retirement](CLASSIC_FOR_RETIREMENT.md);
- typed expression-level `catch`, closed errors, managed cleanup, must-use and
  affine carriers: [Language Improvements](LANGUAGE_IMPROVEMENTS.md);
- collection mutation, floating effect carriers, exact fresh object literals,
  proven-number operators, and lower range execution: the Core Value, Effect, And
  Collection Laws section of that document;
- authored and boundary absence: [Absence And Option Design](ABSENCE_AND_OPTION_DESIGN.md);
- operation yield, replay, cancellation, and observation:
  [Operation Execution Design](OPERATION_EXECUTION_DESIGN.md); and
- cold/warm executable restoration:
  [Executable Artifact Lifecycle Design](EXECUTABLE_ARTIFACT_LIFECYCLE_DESIGN.md).

Settled surface decisions include:

- `switch` and statement `enum` are removed in favor of `match`, closed sums,
  and ordinary `if`;
- boolean operators require booleans;
- object equality is explicit through `Value.equals`, while ordinary object
  `==`/`===` refuses;
- classic C-style `for` and the authored `..` operator are retired;
  Python-compatible `range(stop)` / `range(start, stop, step)` constructs a
  first-class immutable `IntegerRange` consumed through the canonical
  `Iterable<number>` substrate, while `while` owns genuinely stateful and
  unbounded repetition;
- `Number.parseInt` and `Number.parseFloat` return `Option<number>` and consume
  the entire string;
- array bracket reads fault when out of bounds, while `Array.at` is the
  opt-in `Option` accessor; and
- expression-level `catch` is typed `Result` handling, not exception handling.

## Implementation Evidence Boundary

A roadmap item is complete only when all of the following agree:

1. its canonical document describes the landed semantics rather than an
   interim bridge;
2. the implementation carries checked facts through the complete owner chain;
3. the positive, negative, replay, restore, and resource-boundary tests required
   by that capability slice pass;
4. when the slice changes the public SwarmScript language, ADR-2091 and the
   syntax inventory classify that surface accurately;
5. when the slice changes the public SwarmScript language, the primer teaches
   the implemented authored surface, is updated in the same tranche, and
   contains no compiler-owner, Rust, work-machine, Lane, queue, or repair
   vocabulary; Level-5 library APIs instead update their owning package and
   design documentation; and
6. the canonical frontier classifies the exact product as **LANDED**, and no
   current-source audit refutes that classification.

Native-agent evidence is cumulative by capability slice; a later expansion does
not gate completion of an earlier slice:

The main delivery bullets follow the working native spine through distributed
recovery. Authored capability parity, governed help, human decision work,
memory/context, evaluation/qualification, and reactive closure are later
composable evidence branches: each starts when its own owner products exist and
does not wait for Mesh or another branch unless it explicitly claims that
integration.

- the minimal memoryless slice proves one fresh, tool-free turn against a
  deterministic scripted model under one exact profile identity/version, with
  ordered `AgentFrame<Profile>` delivery, only lawful delta coalescing, exactly
  one settled outcome (`ModelTerminal` or typed fault/cancellation) per
  `ModelAttempt`, and one typed outer agent-turn session terminal; its compiling
  `.ss` vertical uses a conversation actor, concrete profile protocol, model
  `Operation`, and typed capabilities; the actor creation turn commits the
  immutable snapshot and conditional-commit right before returning the
  initialized live session without awaiting it, while a separate later mailbox
  turn may accept the sealed terminal delta; it rejects stale-view action
  reconstruction and actor/protocol wait cycles;
- the typed-tool-loop slice proves multiple attempts, invalid and unknown tool
  refusal before capability invocation, approval parking with
  duplicate/stale/foreign-turn refusal, and exactly one typed terminal for every
  tool invocation; a length-truncated response dispatches no calls, parallel
  execution preserves deterministic source-order model settlement, and steering
  versus queued follow-up remains explicit; the initial advertised tools are a
  projection of one exact fixed admission-time capability portfolio, and model
  output cannot discover or widen it;
- the authored-capability branch proves one contract behind native, authored
  SwarmScript, and deterministic fixture implementations; authored static and
  composite wrappers retain broader capabilities privately, enforce their own
  decomposable policies, return honest receipt joins, and expose no component
  or bypass authority, while dynamic delegation remains a later owner-minted
  expansion;
- the generated-client slice proves that embedded and CLI/IPC projections
  consume the same generated protocol semantics and cannot hydrate actions,
  sessions, or replay authority from JSON, ids, or views; a TUI additionally
  proves its typed ProductUiModel reducer, checked SSX/ViewTree affordance plan,
  mount owner, and terminal renderer without creating a second control path;
- the minimal-coding slice proves workspace scoping, file read/search, patch
  conflict and publication, bounded foreground command/process lifecycle,
  typed diagnostics/tests/artifacts, approval and sandbox refusal, cleanup, and
  a useful local coding CLI over the same protocol, plus a TUI when its view/mount
  prerequisites have landed; it does not require child workspaces, checkpoint
  recovery, or Mesh;
- the committed-journal/resume slice proves that the portable committed-
  conversation journal/snapshot remains distinct from exact runtime checkpoint/
  event-journal authority, semantic memory/currentness, and immutable evidence/
  provenance; live attach and committed continuation return distinct receipts,
  exact terminal revision and exclusive-currentness/fencing admit one successor
  epoch, and the resumed next turn is fresh without claiming in-flight recovery;
- the fresh-child slice proves scoped child capabilities and budgets, exact
  one-shot child selection/spawn, typed fan-in, structured cancellation, and
  exactly one typed child-session terminal; when composed with the coding
  profile, isolated child workspaces and conflicting child edits are additional
  child-slice evidence rather than baseline coding prerequisites;
- the governed-observation/help branch proves independent TUI, supervisor, and
  peripheral-context grants/cursors over the same source facts; bounded rollups
  retain coverage, selections, omissions, redaction, truncation, and replay
  gaps; context materialization is attempt-bound; nested help returns exactly
  one typed terminal per hop under monotonically attenuated disclosure,
  authority-request, budget, deadline, and hop scope; and observations, helper
  names, and role labels cannot answer, route, grant, or dirty work;
- the fork/provenance expansion proves replay, archive import, recomputation,
  and committed-boundary fork have distinct receipts and failure scopes without
  requiring in-flight checkpoint support; compaction preserves committed
  history and effect/evidence receipts; fork keeps the source live and copies no
  live authority; contribution provenance locates the exact run/conversation
  revision as OBS, while an independently authorized inquiry chooses only an OBS
  mode and then consumes normal semantic-fork or lossy-seed admission without
  provenance granting authority; an exact-restore inquiry first restores and
  settles the original work, then forks at a committed boundary, and remains
  unavailable until the local durability slice supplies recovery admission;
- the local durability/recovery slice proves exact runtime-checkpoint and
  runtime-event-journal authority, owner-distinct from the portable committed-
  conversation journal/snapshot, semantic memory/currentness, and immutable
  evidence/provenance; exact restore has its own receipt and post-admission
  terminal scope; surviving volatile and reopened persistent bindings run the
  same restore/fencing law while unavailable durability refuses honestly; only
  owner-internal pending-action/work state and receipts survive, every
  predecessor handle becomes stale, and the restored owner mints successor-
  epoch one-shot authority; cancellation, crash, restore, or replay cannot
  duplicate a committed effect, process action, tool call, or child spawn, and
  provider-continuation fidelity remains independent; no Mesh grant, remote
  cursor, or distributed checkpoint graph is required for this proof;
- later coding-breadth evidence proves bounded PTY/background-process lifecycle,
  exact terminal-or-recovery-supervisor transfer before turn settlement,
  checkpoint/reconnect without duplicated process or patch effects, and the
  already-separate isolated child-workspace laws; its absence does not reopen
  the useful minimal coding slice;
- the Mesh-placement expansion proves that local and remote model, tool, child,
  and session execution preserve destination settlement and the facade law
  before distributed recovery is claimed;
- the distributed-recovery expansion then proves every subordinate is terminal,
  durably parked, or transferred before root checkpoint publication, and proves
  actor conditional commit/conflict, successor-epoch fencing, plus Mesh
  deduplication, cursor, grant-currentness, cleanup, and recovery law;
- the receipt-bearing memory/context branch proves the complete Graphstore read
  footprint -> Memory result -> Retrieval selection/absence/snapshot -> GraphRAG
  context plus bounded peripheral source coverage/omission/redaction/gap state
  -> context-materialization -> agent-run receipt chain without treating
  retrieved text, a transcript, or a runtime checkpoint as semantic truth;
- the evaluation/qualification/assurance branch proves owner-bound evaluation
  target correspondence without trace-id authority, ordinary graph-function
  execution and receipt-bound capture, head-bound evidence reads, explicit
  per-turn/trajectory/whole-run/outcome scopes, exact judge-run
  lineage, observable sampling/persistence gaps, immutable realization-scoped
  qualification records, stable specification versus exact realization,
  requested/effective model and effort, declared complete suite/matrix coverage,
  missing/stale/insufficient-cell refusal, blind paired equivalence/
  non-inferiority/superiority judging, hard safety/effect/tail gates before
  cheapest-adequate one-shot owner selection, deterministic tool-world
  reproduction, output-bound runtime candidates, one-shot domain admission,
  receipt-bound compensation, exact delayed/censored operational outcomes, and
  propensity-recorded paired production comparisons or shadow/canary isolation
  that cannot reroute live work or duplicate effects;
  decision-description/process targets bind the exact case, presentation
  realization, exposure propensity, selected/omitted context, and interaction
  receipts; fidelity/usefulness, interaction friction, coordination process, and
  decision/downstream correctness remain separate, while complaints,
  clarifications, explanation requests, corrections, abandonment,
  reassignment, timeout, and reversal are evidence rather than truth;
  stale/foreign/mismatched/duplicate selected realizations refuse, and raw
  scores, model names, or portfolio ids cannot dispatch; and
- the human-decision branch proves one actor/protocol-owned root case/session
  with current one-shot decider actions and exactly one terminal; per-decider
  timeout and root settlement remain distinct; only declared reassignment,
  qualified-agent, deterministic, remain-blocked, or terminal fallback occurs;
  TUI/queue observations cannot claim or answer; restore fences predecessor
  actions; and only the exact dependent footprint pauses or rechecks while
  unrelated work continues, including across independently supervised Mesh
  regions with no global transcript, total order, or Git-derived authority; and
- the closure expansion proves one reactive virtual agent exposing the same
  admitted `Profile` as a simple native agent through an outer
  `ProtocolSession<AgentTurn<Profile>>`, while its internal candidates still
  require verification/admission and a newer invalidation cannot be cleared by
  a stale no-op recheck or sibling patch; it binds every occurrence to one
  owner-minted semantic work key, typed assignment, receipt-constrained
  dependency/assumption footprint, generation, and sealed running binding;
  assignment/profile/admission mismatch refuses and projected/absence/
  assumption invalidation affects only covered work;
  observations may wake owners but only admitted evidence/currentness/decision
  receipts dirty their verified dependency/assumption/decision footprints;
  capability changes require one graph-owner reservation and successor
  assignment/capability epoch, never retroactively widen an old attempt, and
  denial or failed finalization leaves no half-installed grant;
  a `DecisionPrerequisite` fences only covered dependents, and answer or explicit
  timeout/fallback settlement rechecks those exact dependents under the current
  generation;
  dirty state and invalidation receipts alone grant no runtime control; one
  candidate disposition/future-patch pair must first pass graph/frontier-owner
  compare-and-swap reservation and semantic fencing; every supported
  continue/recheck, fence, drain, pause, compatible redirect, cancel/join,
  supersede, reroute, or transfer that actually changes live work consumes one
  owner-issued one-shot `RuntimeControlTicket` over the exact reservation and
  opaque `RunningWorkBinding`, then returns one `RuntimeControlReceipt` at an
  admitted safe boundary; semantic `Conflict` settles without minting runtime-
  control authority; pause/insert/resume revalidates generation; a compiling
  `.ss` vertical lowers a reserved semantic patch through
  `current_process -> snapshot_plan -> graph.reconcile -> reconcile_plan ->
  replan`, then consumes the receipt set to finalize/requeue/conflict the
  reservation; crash restore catches up the invalidation/reconciliation journal;
  one scripted schedule compares eager-cancel, finish-fenced, and
  pause/insert strategies with zero stale commit or duplicate effect;
  stale/duplicate/wrong-binding/wrong-generation/predecessor-epoch control
  tickets refuse, and recovery reconciles unsettled tickets against receipts
  before retry with zero duplicate control;
  stale output cannot publish; explanations are replay-stable/redacted OBS and
  cannot control work; the product/domain owner seals a base-bound
  `AdmittedProductDecision` over the candidate product/patch, verified
  dependency footprint, exact receipts, changed-at marker, and covered dirty-
  generation transition while the corresponding `FrontierPatch` remains a
  candidate; the graph/frontier owner consumes both in compare-and-swap, and
  only success publishes current product state and admits/applies the frontier
  transition; and it proves a
  one-child virtual wrapper is observationally equivalent to direct execution
  of the same exact delegated leaf definition/realization under the same model,
  profile, fidelity, grants, budget, scopes, policies, input, scripted-model
  trace, and coupled deterministic world schedule, including actions,
  cancellation, terminal meaning, and receipt law; wrapper/outer definition,
  opaque fresh identity, and topology-only receipt lineage may differ.

If optional foreign-harness compatibility is selected later, that project adds
its own evidence: each private realization and closed driver stack negotiates
the facade profile separately from its private protocol, reports or refuses
every `supported`, `emulated`, `lossy`, or `unavailable` feature exactly, and
runs the applicable shared behavioral suite behind the same public profile.
That evidence gates only the selected adapter; it is never required to complete
the native model, harness, client, coding, continuity, recovery, Mesh, memory,
evaluation, or reactive slices above.

Expected-red configuration is only a temporary implementation ledger, never a
canonical destination. Each such fixture must be classified against the current
contracts: canonical positive behavior moves into ordinary conformance and its
substrate is repaired until it passes; superseded behavior is deleted or
replaced by a stable negative. A remaining expected-red cannot prove a roadmap
area complete.

Compile progress, an internal carrier, a cache hit, or one green fixture is not
completion by itself.

## Deliberately Operational Work

Exact failing fixtures, compiler E-codes, stale callers, source-file cuts,
temporary compatibility scaffolding, test commands, and measured pass/fail
counts belong in version-pinned suite artifacts or the active work ledger. They
do not belong in this roadmap unless they expose a new durable semantic
decision.

Temporary parallel lanes may be derived directly from any area above and
discarded after integration. Their status does not need to be copied back into
this document; only a newly discovered durable outcome or changed canonical
contract does.

Likewise, source-tool build failures, diagnostic rendering polish, dead-file
deletion, tracked-binary cleanup, and obsolete constructor removal are repository
hygiene. They remain important implementation work but are not separate language
or platform pillars.
