BUNDLE REVISE

Reviewed libbun candidate: `ec6a7f249120a833aeaa4e0211fe0f41d17e0565`
(tree `6da13ed79ca5df4554b7c0bf3c89cde7d9dcea0d`), carrying frozen product
source `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb` (tree
`cb964de8ab8162449fbe95959bf34d231570aa5c`). The fail-closed verifier,
all fourteen attachment hashes and byte counts, Oracle dry-run identity, Fable
order, zero product delta, and `NOT LAUNCHED` state pass.

## Determining omission

The owner/generative bundle does not attach or report the real selected-route
producer, branded-invocation producer, sole libbun consumer, or their Cargo
dependency boundary. The source report is confined to the libbun product SHA.
An adjacent source-wide search at swarm
`95323ff17cb29928e31467f651ef03bae2099c14` (tree
`43b47bbd49a6053d270b3e15cc141cb1b1bb86da`) finds the missing SCC:

- `/home/ubuntu/swarm/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs`:
  `ManifestResolvedExternalProviderCallAuthority`,
  `ManifestResolvedExternalProviderCallAdmission`, their exact-route mint, and
  `into_contract_and_module_for_durable_external_provider_owner_v1`.
- `/home/ubuntu/swarm/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs`:
  `DurableExternalProviderInvocationAuthority`, its mint from the exact call,
  input, and output-settlement authority, and
  `into_call_input_and_output_settlement_for_durable_external_provider_owner_v1`.
- `/home/ubuntu/swarm/crates/ss-runtime-external-capability-provider-owner/src/lib.rs`:
  `SsExternalCapabilityProviderHost`, the sole libbun backend consumer,
  `invoke_manifest_resolved_call_for_provider_host_set_owner_v1`, the adapter
  source constructor, raw `ProviderRequest`, and current shutdown.
- `/home/ubuntu/swarm/crates/swarm-provider-host-set/src/external_transport.rs`:
  the invocation and shutdown trait boundary.
- `/home/ubuntu/swarm/crates/ss-runtime-external-capability-provider-owner/Cargo.toml`
  and `/home/ubuntu/swarm/Cargo.toml`: the actual dependency direction and
  pinned libbun API.

These files show that the current producer already owns one sealed exact call,
input, and output-settlement authority, but the sole consumer publicly splits
that product, extracts path/export/contract material, constructs JavaScript
adapter source, and rebuilds a libbun request. None of those constructors or
callers appears in the fourteen-file plan. No attached source defines a lawful
mint for `SelectedProviderPackage<Brand>` or `ProviderInvocation<Brand>`.

Rust privacy makes the omission decisive. A libbun-private constructor cannot
be called by the downstream producer; a public constructor would be the raw
mint the contract forbids; and a trait, callback, parts product, selector, or
caller-chosen receipt would reintroduce the forbidden bridge. The requested
exact export/backend correspondence therefore cannot be reviewed or patched
until the dependency/ownership move is selected from the complete SCC.

## Highest owner and closed replacement

`BunProviderBackend` remains the highest runtime owner. The generative
selection operation must move into the same concrete owner boundary that can
consume the exact manifest-resolved call, provider input, output-settlement
authority, and retained backend without public RAW. The corrected bundle must
choose and patch that owner move explicitly; it cannot leave the producer in
swarm while asking a libbun-only patch to invent an inaccessible constructor.

The closed algebra remains:

```text
BunProviderBackend = Ready(ReadyCustody) | Restartable(RestartableCustody)

prepare(backend, SelectedProviderPackage<B>, ProviderInvocation<B>)
  -> Reserved(PreparedExport<B>)
   | Refused(OfferRefusal<B>)
   | AdmissionFault(AdmissionFault<B>)

OfferRefusal<B>
  -> retry with the same moved selection and private OfferReadyProof<B>
   | consuming shutdown

PreparedExport<B>
  -> drive, consuming the sole dispatch permit into DriveCustody<B>
   | cancel_before_dispatch, requiring ReservationReleaseProof<B>
   | consuming shutdown

Every terminal
  -> exact Ready continuation proven by OfferReadyProof,
     ReservationReleaseProof, or InvocationReadyProof
   | exact Restartable continuation proven by RetirementProof
   | queue-owned RetirementQuarantine<Purpose> adopted before a bounded fault
   | no continuation after consuming shutdown
```

The two branded inputs are move-only, non-cloneable, non-serializable, and
cannot be independently constructed. Differently branded values cannot enter
one `prepare` call. No terminal exposes backend, selected inputs, proof,
reservation, path, export, adapter source, request bytes, or settlement parts.

## Exact first edit and migration order

The poison cut is already complete. The first positive libbun edit is
`src/lib.rs`: introduce the opaque by-value `BunProviderBackend` owner through
a private retained-backend module, with only private `Ready | Restartable`
state, and establish the process-wide preallocated durable queue substrate
before exposing any live constructor. Do not restore or alias
`install_prepared_export`.

Then:

1. Attach the six missing source/Cargo files above at one exact adjacent SHA
   and regenerate the source report from both repositories.
2. Record the same-owner/dependency ruling that moves the exact selected-call
   producer and retained-backend admission into one compilable owner. If no
   acyclic placement can own both, mark the decomposition invalid and move the
   owner boundary; do not add a bridge.
3. Add the generative branded mint at that owner, then delete the raw
   `into_contract_and_module_for_durable_external_provider_owner_v1`, raw
   adapter-source/request reconstruction, and every stale dynamic-loading
   caller in the same migration.
4. Add private `OfferCustody`, `OfferReadyProof`, closed refusal/retry/fault and
   consuming shutdown.
5. Add private `ReservedCustody`, its single dispatch permit, affine
   `PreparedExport`, `ReservationReleaseProof`, and pre-dispatch release.
6. Hand reserved/drive/retirement custody privately into the lifecycle tranche;
   ambiguous release retires or transfers to the durable queue and never
   returns Ready.
7. Regenerate this part's file plan, manifest hashes, source-search report,
   Oracle dry run, and Fable plan. Keep both models `NOT LAUNCHED` until a new
   independent pass.

## Required hostile evidence

- External compile failures for raw installer import/call, branded-product
  construction, field/parts/raw selector access, clone/serde, proof minting,
  and differently branded package/invocation composition.
- Focused default-parallel tests proving exact selected export/backend
  correspondence and that one selected invocation cannot be replayed.
- Proof-bound refusal followed by unchanged retry on the same worker/epoch.
- Pre-dispatch release proving no request, output, cancellation, or invocation
  task existed; ambiguous release must retire or enter queue ownership.
- Cancellation and injected unwind at offer, reservation, and dispatch
  boundaries with no lost selected input, permit, backend, or OS custody.
- Consuming shutdown from Ready, Restartable, refusal, prepared export, and
  quarantine-facing terminals; no shutdown path returns a backend husk.
- Repository-wide searches proving removal of the raw route/module splitter,
  adapter-source reconstruction, raw libbun request constructors, dynamic
  loader/plugin compatibility, callback proof, and caller-minted receipts.

Until those producer and consumer sources are included and the owner move is
made exact, the part cannot authorize a model launch or a positive
owner/generative implementation.
