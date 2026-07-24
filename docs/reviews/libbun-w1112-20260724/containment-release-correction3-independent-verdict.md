PART BUNDLE REVISE

# W1-11/W1-12 Containment And Release Correction-3 Independent Verdict

Date: 2026-07-24

## Reviewed identity

- Exact correction-3 candidate: `c2ea016e4c9810fa86ddfd21bd4b30823746a9b9`
- Candidate tree: `67bdbd8830930ed39d19e7f37be092c108de01f7`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Correction review base: `51b0118428d7881f39f32df396ef32349a5a52ab`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Containment manifest SHA-256:
  `c5646166de5f2763613a7cd37efc5ed987cd7c9148241e5e403468aade55671c`
- Ordered file-plan SHA-256:
  `9d02c5946e9a46b231179cc6baf5950d2853f96c08b5b068be4741bdd512330e`
- Prompt SHA-256:
  `5bd10e1239095553de044151d47a59b8fc15e78429e62b7eaf8764510b75af7c`
- Oracle dry-run SHA-256:
  `f5728c3b80bd01f91dae041d9a2deb5aa7e770b9338d03a5870a7e7197845c49`
- Fable plan SHA-256:
  `c85ca578e74cd95fd74ef19265e9bcab7e84506db88798ad237782a9af5a53d3`
- Generator SHA-256:
  `029181b9fea6b356bf3144860447d312c5299bf732016b1ccd731fe50afecdaa`
- Verifier SHA-256:
  `e428c140a4b4485732dec06c21049e1f5247684c35847a8cde9f082f55ed53bd`
- Review Lane owner: `libbun-c3-containment-review-20260724`
- Review worktree:
  `/home/ubuntu/bridge-ops/dev-worktrees/libbun-c3-containment-review-20260724`
- Review cargo target:
  `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-14`
- Model state: Oracle `NOT LAUNCHED`; Fable `NOT LAUNCHED`.

`lane run` resolved the exact assigned owner, worktree, cargo target, detached
candidate HEAD, and clean starting tree. The candidate has no product, test,
Cargo, workflow, vendor, or release-source delta from the exact product source.

## Mechanical replay

The correction-3 deterministic generator replayed all six reports byte for
byte from this independently named Lane:

```text
OK exact-source-search-report.md
OK vendored-bun-boundary-report.md
OK adjacent-swarm-source-index.md
OK lifecycle-vendored-jsc-source-bundle.md
OK process-drop-caller-and-fixture-report.md
OK lock-privacy-compliance-index.md
```

The fail-closed verifier also exited zero and independently replayed the
generator in a differently named clean checkout:

```text
PASS: correction-3 bundles are Lane-independent, exact-source complete,
clean-checkout replayable, zero-product-delta, NOT LAUNCHED, and sub-272k.
containment-release: 46 files; 271075 tokens
```

The Lane-independence correction is valid. The literal part pass remains
unavailable because the verifier's containment source-completeness set is
smaller than the concrete implementation requested by the containment prompt.

## Determining defects

### 1. The retained-host shutdown SCC is represented by grep output, not source

The containment ordered plan does not attach the exact retained-host pool,
checkout, replacement, runtime-domain shutdown, or final close sources. In
particular it omits:

```text
crates/ss-runtime-test-execution-owner/src/lib.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs
```

The last two paths are not even present in `ADJACENT_PATHS`, the adjacent
snapshot index, or either generated search report. Repository-wide search at
the exact adjacent SHA proves they are part of the direct shutdown SCC:

```text
source_work_set_live_feed_session.rs:5:
    runtime_execution_domain_owner: SsRuntimeExecutionDomainOwner,
source_work_set_live_feed_session.rs:120:
    close_for_execution_graph_owner(self, ...)
source_work_set_worker_execution.rs:731:
    fn shutdown_runtime_execution_domain_owner(...)
source_work_set_worker_execution.rs:735:
    runtime_execution_domain_owner.shutdown(session)
runtime_execution_domain.rs:167:
    let shutdown_result = self.provider_settlement_pool.shutdown();
provider_settlement_lane.rs:40:
    self.provider_pool.shutdown()
external_capability_provider_pool.rs:55:
    provider.shutdown()?;
```

The current `close_for_execution_graph_owner` consumes the live-feed session
without calling the separately defined shutdown helper. That stale final edge
is exactly the source a concrete implementation must inspect and repair. The
79,220-byte process report only prints selected grep matches, omits both files,
and cannot supply the enclosing ownership or consumption path.

The source identities independently bound at the adjacent SHA include:

| Path | Git blob | SHA-256 | Bytes |
| --- | --- | --- | ---: |
| `.../runtime_plan_owner/source_work_set_live_feed_session.rs` | `7379a6c2a2a8fcf9db5d882f3d314f7a7e930bf9` | `3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012` | 6025 |
| `.../runtime_plan_owner/source_work_set_worker_execution.rs` | `14d0aff964240b651d64b6cbc02622554dba61df` | `54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760` | 29621 |
| `.../runtime_plan_owner/external_capability_provider_pool.rs` | `92ceabd5eda889f6d1763082c90d5932b78a5086` | `3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de` | 2269 |
| `.../runtime_plan_owner/provider_settlement_lane.rs` | `1caaed34441eb4de28053e6ea0acd0212981cbf8` | `48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323` | 3030 |

Without these bytes, the requested concrete patch would have to invent the
final shutdown carrier and consumption order.

### 2. The real retained-runtime hostile tests are omitted

The containment plan attaches three `.ss` ProviderValue fixtures but omits the
adjacent Rust real-binary test owner:

```text
crates/ss/tests/external_capability_provider.rs
blob: e248a842cca385d15f8270b6abef25e2faf9b94d
sha256: 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191
bytes: 14251
```

That file owns both required current migration sites:

- `ss_reuses_one_libbun_runtime_for_multiple_capability_imports`; and
- `ss_test_pool_child_conserves_package_roots_for_test_and_libbun_providers`.

It also contains the stale release path that the hard cut must delete:
`libbun::release::current_native_plugin_asset()` and
`../libbun/plugin/target/release`. The process report prints only matching
lines. It omits the fixture constructors, package topology, assertions, binary
launch, and complete stale helper body from the ordered model inputs. A
reviewer cannot return the required exact-source-applicable retained-worker,
pool-child, replacement-epoch, or freshly packaged binary test migration from
those matches.

### 3. The active package and release implementation is omitted

The prompt requests commit-grade changes for the package manifest, immutable
archive, linked inputs, extracted smoke execution, repeat-lock behavior,
compliance, and worker-only release workflow. The plan directly attaches the
Cargo/lock/license closure and current CI workflow, but it does not attach the
active package/link/reproducibility implementations or the governing release
contract:

| Path | Git blob | SHA-256 | Bytes |
| --- | --- | --- | ---: |
| `scripts/package-prepared-export-worker-release.sh` | `240ae76143dad2acdde8b360ad7747b1ee49fd19` | `ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44` | 2111 |
| `scripts/prepare-native-bun-link.sh` | `2b44b7e3ec653f663ee8857bc7fd188b5cff113b` | `498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6` | 3233 |
| `scripts/verify-vendored-bun-reproducible.sh` | `fe4e2abdb280d61882c8b8176a2254a112f7433c` | `96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304` | 1913 |
| `scripts/verify-vendored-bun.sh` | `b1d2b4fbc6a4840a7688c0be4e72abb00b70f945` | `d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7` | 1515 |
| `docs/LIBBUN-WORKER-RELEASE-CONTRACT.md` | `2dc8ccc43fbaa6427204d35d85200596ef52ed8b` | `030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c` | 8765 |

The exact-source report exposes selected package-script lines, including
`fresh-process-only` and `fallback`, but not the complete script. It cannot
support an exact unified diff for its target normalization, hashing,
manifest-construction, archive ordering, cleanup, or smoke protocol. The
vendored report records only the full-file identity of the link-preparation
script, not its source bytes. File identity proves provenance; it is not source
available to the requested concrete implementation reviewer.

## Clean replacement

Keep the Lane-independent generator labels, complete lock/privacy/license
inventory, exact JSC complete-item evidence, zero-product-delta gate, and
`NOT LAUNCHED` state. Correct only the prelaunch evidence bundle:

1. Expand `ADJACENT_PATHS` and the adjacent snapshot index to include
   `source_work_set_live_feed_session.rs` and
   `source_work_set_worker_execution.rs`. Extend the adjacent process search
   pathspec to both files and prove the actual call count of
   `shutdown_runtime_execution_domain_owner`, not merely its definition.
2. Add the complete adjacent pool, provider-settlement lane, test-execution
   owner, real-binary Rust hostile test, live-feed close, and shutdown-helper
   sources to the containment ordered inputs. A compact generated bundle may
   use exact complete-item excerpts for the large runtime-domain and worker-
   execution files, but every excerpt must include the full owning item,
   callers, line span, blob, full-file SHA-256, excerpt SHA-256, and bytes.
3. Add complete bytes for the package, native-link preparation, vendored
   reproducibility scripts, and worker release contract to the containment
   order. Keep `runtime/src/main.rs` and `native/build.rs` complete either as
   direct files or as their already complete bound excerpts.
4. Preserve the three `.ss` ProviderValue fixtures and add the full adjacent
   Rust real-worker test so hostile W1-10 ingress and retained execution are
   both patchable.
5. Amend `REQUIRED_ATTACHMENTS["containment-release"]`; its current
   `CORE_ADJACENT` subset must require the retained-host/final-shutdown and
   release/package sources above. Add a verifier rule that rejects a report-
   only substitution for a requested concrete implementation source.
6. Recover token budget by removing the broad 25,494-token process grep report
   from the containment model order after its facts are replaced by compact
   complete source items. Keep that deterministic report checked in for
   cross-part audit if useful. Regenerate the ordered plan, manifest, Oracle
   dry run, Fable plan, generator/verifier hashes, and synthesis manifest.
7. Replay generator and verifier from a differently named clean Lane, prove
   zero product/test/Cargo/workflow/vendor/release-source delta, and obtain a
   fresh independent literal part verdict at that exact correction commit.

This replacement remains below the same product boundary and requires no
Oracle or Fable launch. It preserves the opaque by-value `BunProviderBackend`,
private wire/native phases, persistent pumps, exact platform containment,
proof-bound cooperative reset, nonblocking durable adoption, consuming
shutdown, selected lock/license closure, and hard-cut deletion order already
frozen by the contracts.

No product, test, Cargo, workflow, vendor, package, prompt, manifest, plan,
report, or model-state artifact was modified by this independent review.
