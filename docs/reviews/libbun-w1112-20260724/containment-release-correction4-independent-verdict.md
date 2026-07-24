PART BUNDLE REVISE

# W1-11/W1-12 Correction-4 Containment And Release Independent Verdict

Date: 2026-07-24

## Bound identity

- Correction-4 candidate: `4dd3395129a221d8c1fb2d1dbbdae509b2331f0e`
- Candidate tree: `fbd9f82cfae0554abe87623f080f0ce4eb1c6b91`
- Exact libbun product: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Review Lane owner: `libbun-c4-lifecycle-review-20260724`
- Worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c4-lifecycle-review-20260724`
- `CARGO_TARGET_DIR`: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-17`
- Containment manifest SHA-256: `c433a06d9d59a9c5f241da21033f6cbfb17aa47480942778bf0c243d9c58217e`
- Containment ordered plan SHA-256: `cd03bc8454b144cf93820b6d13973e2fe9e6ec68b5ca862096d8ebb6f7c30a96`
- Containment prompt SHA-256: `9ca91668ca5f266e4432fe6e0b7ace604cd5096b1c28e7d94c8177e6fee0e7be`
- Containment dry-run SHA-256: `6ad5b915c626be855c67596ed78a085265a5804725a83b518df4f1e7988a9e63`
- Generator SHA-256: `f4158b8bf8d9862de47105b87957bf961d7d87f8ec1ba7096de5156c5fa31309`
- Verifier SHA-256: `52c35938408a84107acb43282f27761844369494fd6fcb6b3ae1ce6a9e54b749`
- Oracle and Fable: `NOT LAUNCHED`

Correction 4 validly restores the full real-worker fixture, retained provider
pool/replacement implementation, stale plugin migration sites, all four
nonvendored locks, complete external privacy harness, vendored workspace/lock,
license inventory, active package/link/reproducibility scripts, release
contract, and CI source. Their identities and ordering replay exactly. The
generator and clean-checkout verifier both exit zero.

## Determining defect: final-shutdown callers remain outside the source set

The bundle attaches the inner retained-host shutdown chain:

```text
source_work_set_live_feed_session.rs
  -> SsRuntimeExecutionDomainOwner::shutdown
  -> ProviderSettlementLane::shutdown
  -> ExternalCapabilityProviderPool::shutdown
```

It also exposes that the inner close currently consumes the runtime-domain
owner without calling the separately defined shutdown helper. That closes the
correction-3 filename omission, but not the full final-close SCC.

A fresh repository-wide exact-SHA caller search finds three active outer
carrier/consumer files absent from `containment-release-files.txt`,
`ADJACENT_PATHS`, the adjacent snapshots, and the containment verifier's
required set:

```text
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs:295
    defines the next close_for_execution_graph_owner carrier;
artifact_session.rs:305
    calls the attached live-feed close;

crates/ss-runtime-test-execution-owner/src/test_runner/state/
  source_work_set_execution_graph_owner.rs:1424,1431
    defines and calls the next close carrier;

crates/ss-runtime-test-execution-owner/src/test_runner/state/
  source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs:1594
    is the final close caller and owns the success/fault branch.
```

The last caller is lifecycle-significant: before close it shuts down/reaps the
runtime file pool, consumes graph settlements and the live runtime-plan feed,
then calls `close_for_execution_graph_owner`. On error it restores
`runtime_file_execution_session` and marks the plan closeout failed. The live
feed and its runtime-domain owner have already been consumed. An exact
implementation must reconcile shutdown failure, already-consumed retained
runtime custody, session reinsertion, retry, and final `Drop` without returning
a backend husk or losing process/output/containment custody.

None of those success/fault transitions is present in the ordered model
inputs. A model can patch the attached inner method but cannot prove the sole
final consumer's refusal/retry/error ownership or whether the proposed typed
shutdown result is propagated without authority reconstruction.

The verifier searches only the fixed `RETAINED_HOST_SHUTDOWN_PATHS` tuple and
asserts strings within those snapshots. It does not discover the outer callers
repository-wide. Its count assertion for
`shutdown_runtime_execution_domain_owner` proves one definition and zero calls
inside the selected subset; it does not prove the complete final-close SCC.

## Clean correction

Keep the correction-4 package/link/reproducibility, lock/privacy/compliance,
real-worker fixture, inner retained-host pool, stable-label replay,
zero-product-delta, and `NOT LAUNCHED` improvements. Correct review evidence:

1. Make the first generator edit an exact-SHA repository-wide definition and
   caller discovery for `close_for_execution_graph_owner`,
   `shutdown_runtime_execution_domain_owner`, `SsRuntimeExecutionDomainOwner`,
   `ProviderSettlementLane`, and `ExternalCapabilityProviderPool` across the
   adjacent Rust source tree.
2. Add exact snapshots and complete owning-item excerpts for the outer close
   carrier in `artifact_session.rs`, the wrapper in
   `state/source_work_set_execution_graph_owner.rs`, and the final
   success/error consumer in
   `state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs`.
   Bind the relevant module boundary in `artifact_session/runtime_plan_owner.rs`.
3. Extend `ADJACENT_PATHS`, retained-host attachment sets, containment and
   synthesis ordered inputs, and `REQUIRED_ATTACHMENTS` to require these
   carrier/consumer items. Report their Git blobs, full-file SHA-256/bytes,
   exact spans, and excerpt SHA-256 values.
4. Extend the containment prompt so a concrete patch must preserve custody
   when final shutdown fails after the feed is consumed and the caller restores
   its runtime-file session. Require exact-once final shutdown, no retry from a
   consumed backend, and silent durable adoption on every unwind/Drop branch.
5. Add a negative repository-discovery verifier: omitting any discovered
   definition or caller must fail before bundle rendering. A string mutation
   of the preselected text is insufficient.
6. Regenerate snapshots, reports, plans, dry runs, manifests, hashes, totals,
   Fable inputs, and synthesis inputs. The containment estimate is `241868`,
   leaving ample room for complete owning items while preserving the 272000
   cap.
7. Replay from a differently named clean checkout, retain zero product/test/
   Cargo/vendor/workflow/release-source delta, and obtain a fresh literal
   containment verdict before model launch.

## Preserved containment and release law

`BunProviderBackend` continues to own the child, exact platform containment,
pipes, persistent bounded pumps, overflow/EOF/barrier state, joins, and
settlement ledger from construction through consuming shutdown. Linux
namespace, macOS sandbox/process control, or Windows non-breakaway job custody
must exist before executable work; no process-group fallback or caller-minted
receipt is allowed. Same-worker reuse requires complete proof-bound drain;
replacement requires proof-bound retirement. Every refusal/retry,
cancellation, deadline, unwind, pump/join failure, replacement race, process
escape, final shutdown, and `Drop` consumes or durably adopts all custody.

The worker package remains immutable-tag, deterministic, hash/inventory bound,
and executable only after freshly extracted linked-worker smoke. The four
nonvendored locks, complete privacy fixture, vendored workspace/lock, exact-tree
license inventory, stale-plugin deletion, repeat-lock gate, and worker-only
release workflow remain mandatory.

## Executed evidence

```text
LIBBUN_REPO=<assigned-worktree> SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 0: all eight generated reports OK

SWARM_REPO=/home/ubuntu/swarm \
  python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 0: independent clean-checkout replay PASS; containment-release 55 files,
241868 tokens

git -C /home/ubuntu/swarm grep -n -E \
  'ExternalCapabilityProviderPool|ProviderSettlementLane|SsRuntimeExecutionDomainOwner|close_for_execution_graph_owner|shutdown_runtime_execution_domain_owner|current_native_plugin_asset|libbun/plugin/target/release' \
  95323ff17cb29928e31467f651ef03bae2099c14 -- crates
exit 0: finds the attached inner chain plus the three unbundled outer
carrier/consumer files and the attached stale-plugin migration sites

git grep -n -E \
  'package-prepared-export-worker-release|prepare-native-bun-link|verify-vendored-bun-reproducible|verify-vendored-bun|current_native_plugin_asset|plugin/target/release|libbun-runtime-native' \
  6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- . ':!vendor'
exit 0: current package/link/reproducibility, build, and CI sites are
represented; the adjacent stale-plugin sites are covered by the preceding
exact-Swarm search and inputs
```

Final disposition: `PART BUNDLE REVISE`. Package and release evidence is
materially repaired, but model launch remains unauthorized until the complete
retained-host final-close caller/fault closure is source-bound and independently
passes.
