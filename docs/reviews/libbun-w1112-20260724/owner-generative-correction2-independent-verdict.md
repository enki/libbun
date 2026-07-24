PART BUNDLE REVISE

Reviewed correction-2 commit
`51b0118428d7881f39f32df396ef32349a5a52ab` (tree
`55f063f9df9491aa6893c233eb2f15950bb428b2`) from the independent Lane
`libbun-w1112-c2-rv-owner-20260724`. The frozen libbun product remains
`6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb` (tree
`cb964de8ab8162449fbe95959bf34d231570aa5c`), and the adjacent Swarm source
remains `95323ff17cb29928e31467f651ef03bae2099c14` (tree
`43b47bbd49a6053d270b3e15cc141cb1b1bb86da`). No product, test, fixture,
Cargo, vendor, or model-launch change was made by this review.

Correction 2 resolves the prior review's six named producer/consumer
attachments and correctly keeps `BunProviderBackend` as the opaque by-value
highest runtime owner. It still cannot receive `PART BUNDLE PASS`: the
fail-closed verifier is checkout-path dependent, and the owner part remains
source-incomplete for the W1-10 input and the complete retained-host lifecycle
SCC.

## Determining defects

### 1. The fail-closed verifier fails in an independent Lane

Both advertised replay commands fail at the exact correction-2 HEAD:

```text
python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check
exit 1

DRIFT docs/reviews/libbun-w1112-20260724/exact-source-search-report.md
DRIFT docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md
OK docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md
OK docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md
OK docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md
OK docs/reviews/libbun-w1112-20260724/lock-privacy-compliance-index.md

python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit 1

FAIL: deterministic evidence replay failed
```

The generator embeds `ROOT` in each displayed `git -C` command. The committed
reports name:

```text
/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724
```

while independent replay emits:

```text
/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-c2-rv-owner-20260724
```

After replacing those two absolute worktree prefixes with one logical
`<LIBBUN_REPO>` token in memory, both report comparisons return exit 0. The
source outputs are stable, but the checked bytes are not. A fail-closed bundle
must replay from a fresh Lane without relying on the construction Lane's
absolute pathname.

### 2. W1-10 `ProviderValue` is present in source but absent from the reviewed contract

The attached
`crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs`
proves the current sealed invocation owns:

```rust
pub struct DurableExternalProviderInvocationAuthority {
    call_authority: ManifestResolvedExternalProviderCallAuthority,
    provider_input: ProviderValue,
    output_settlement: DurableExternalProviderOutputSettlementAuthority,
}
```

The owner prompt and manifest never name `ProviderValue` or W1-10. They say
only "provider input." The report pattern also omits `ProviderValue`, and the
bundle does not attach its defining source or governing W1-10 law. Repository-
wide search at the adjacent SHA locates the missing authority/cargo boundary:

```text
docs/SWARMSCRIPT_ROADMAP.md:332
  W1-10 ProviderValue JSON V1 is the direct producer for the existing libbun
  mechanical drive.

docs/WAVE0_WAVE1_SEMANTIC_CLOSURE_INDEX.md:32-33
  W1-11 consumes W1-10.

crates/swarm-provider-value-model/src/lib.rs:114
  pub enum ProviderValue

crates/swarm-capability-linker-core/src/lib.rs:29-40
  reexports ProviderValue and its fail-closed canonical conversion.
```

This omission permits a proposed implementation to replace W1-10 cargo with
`serde_json::Value`, JSON text, raw bytes, `StructuralValue`, or another
parallel input product while still satisfying every literal prompt term.
`ProviderValue` must remain by-value cargo sealed inside the one branded
invocation; canonical wire conversion must stay private to the finite drive
owner and must not become a public selector, parts bridge, or second admission.

The existing negative fixture
`tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss`
is also absent even though repository-wide fixture search identifies it as the
hostile fail-closed W1-10/external-provider boundary case.

### 3. The owner attachment plan contains only 6 of 13 indexed adjacent snapshots

`adjacent-swarm-source-index.md` binds thirteen exact adjacent files, but
`owner-generative-files.txt` attaches only these six:

```text
crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs
crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs
crates/ss-runtime-external-capability-provider-owner/src/lib.rs
crates/swarm-provider-host-set/src/external_transport.rs
crates/ss-runtime-external-capability-provider-owner/Cargo.toml
Cargo.toml
```

The seven indexed-but-unattached files include the full provider-host-set
owner, its Cargo edge, both direct `ss` install/test callers, and the real `.ss`
fixtures. Search-report excerpts are not source-complete attachments and cannot
support an exact-source-applicable cross-repository patch.

The current plan also omits the Cargo manifests for the exact-route producer
and the invocation producer. A reviewer cannot select and compile the required
acyclic owner/dependency move without:

```text
crates/swarmvm-image/Cargo.toml
crates/swarm-rust-sdk-static-provider-host/Cargo.toml
crates/swarm-capability-linker-core/Cargo.toml
crates/swarm-provider-value-model/Cargo.toml
```

### 4. The retained-host lifecycle search stops before the real ss-test pool

Repository-wide source search finds an omitted direct owner/caller chain:

```text
crates/ss-runtime-test-execution-owner/src/lib.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/external_capability_provider_pool.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/provider_settlement_lane.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/runtime_execution_domain.rs
```

`ExternalCapabilityProviderPool` owns
`Option<(PathBuf, SsExternalCapabilityProviderHost)>`, shuts down and replaces
the retained host on working-directory changes, exposes a mutable checkout,
and performs final shutdown. `runtime_execution_domain.rs` calls the pool
shutdown. None of these files is attached or searched by
`process-drop-caller-and-fixture-report.md`, which therefore proves only the
attached subset rather than complete lifecycle custody.

The omission is material to consuming shutdown, Drop, retry, replacement, and
same-worker epoch semantics. A concrete implementation could leave this pool
holding a stale backend husk or keep the current borrowed mutable lifecycle
while the proposed libbun owner becomes by-value.

## Source-grounded facts that remain valid

The repeated searches confirm these correction-2 facts and they should be
preserved:

- `install_prepared_export` remains poisoned; only the two external
  compile-fail fixtures mention it.
- No positive `BunProviderBackend`, `OfferCustody`, `ReservedCustody`, branded
  producer, release proof, invocation-ready proof, or quarantine/reaper
  implementation exists in the frozen product implementation paths.
- Current `PreparedExport::drive` is the sole libbun mechanical drive owner and
  is affine, non-cloneable, and non-serializable.
- The adjacent sealed invocation already joins exact call authority,
  `ProviderValue`, and output-settlement authority, but the consumer splits it
  into RAW route fields, JSON/adapter source, and `ProviderRequest`.
- The C++ `JSC__VM__deinit` body is exactly empty. Cooperative termination,
  reset, microtask drain, worker teardown, process retirement, EOF, pump joins,
  and containment drain must therefore remain separate proof obligations.
- The current mechanical implementation has typed terminal/fault classes and
  explicit process/thread/Drop sites; the retained implementation must migrate
  them into complete by-value custody rather than add a parallel driver.

## Exact source-complete correction

### A. Make report commands Lane-independent

Apply this semantic edit to
`scripts/generate-libbun-w1112-review-evidence-20260724.py`:

```diff
 def command_text(repo: Path, args: list[str]) -> str:
-    return "git -C " + shlex.quote(str(repo)) + " " + " ".join(shlex.quote(arg) for arg in args)
+    if repo == ROOT:
+        repo_arg = '"${LIBBUN_REPO}"'
+    elif repo == SWARM_ROOT:
+        repo_arg = '"${SWARM_REPO}"'
+    else:
+        raise RuntimeError(f"unbound report repository: {repo}")
+    return "git -C " + repo_arg + " " + " ".join(shlex.quote(arg) for arg in args)
```

Add a prologue to both generated search reports defining:

```text
LIBBUN_REPO = any checkout containing exact libbun SHA 6066a5b85...
SWARM_REPO = any checkout containing exact Swarm SHA 95323ff17...
```

Keep execution against the verified `ROOT` and `SWARM_ROOT`; only the durable
command spelling becomes checkout-independent and replayable.

### B. Attach the complete W1-10 and owner/dependency boundary

Extend `ADJACENT_PATHS`, snapshot generation, exact searches, and the owner
ordered file plan with these exact-SHA files:

```text
docs/PROVIDER_EXECUTION_AND_SDK_LAW.md
docs/PROVIDER_VALUE_JSON_WIRE_V1.md
docs/SWARMSCRIPT_ROADMAP.md
docs/WAVE0_WAVE1_SEMANTIC_CLOSURE_INDEX.md
crates/swarm-provider-value-model/src/lib.rs
crates/swarm-provider-value-model/Cargo.toml
crates/swarm-capability-linker-core/src/lib.rs
crates/swarm-capability-linker-core/Cargo.toml
crates/swarm-rust-sdk-static-provider-host/Cargo.toml
crates/swarmvm-image/Cargo.toml
tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss
```

Also attach all seven already-indexed adjacent snapshots currently omitted from
`owner-generative-files.txt`:

```text
crates/swarm-provider-host-set/src/provider_host_set.rs
crates/ss/src/product.rs
crates/ss/tests/external_capability_provider.rs
crates/ss/Cargo.toml
crates/swarm-provider-host-set/Cargo.toml
tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss
tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss
```

### C. Attach and search the complete retained-host caller/lifecycle chain

Add exact snapshots, Cargo/privacy edges, and process/Drop/shutdown searches for:

```text
crates/ss-runtime-test-execution-owner/Cargo.toml
crates/ss-runtime-test-execution-owner/src/lib.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/body_authority_registry.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/external_capability_provider_pool.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/provider_settlement_lane.rs
crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/
  runtime_plan_owner/runtime_execution_domain.rs
crates/ss-runtime-provider-host-set-owner/Cargo.toml
crates/ss-runtime-provider-host-set-owner/src/lib.rs
```

The regenerated report must trace constructor/open, checkout, replacement,
borrowed access, invocation, retry/refusal, cancellation/unwind, shutdown,
Drop, and final pool destruction through the sole backend.

### D. Freeze the missing prompt invariants

Add literal required terms to `owner-generative-prompt.md` and
`PART_TERMS["owner-generative"]`:

```text
W1-10 ProviderValue is the sole by-value invocation input cargo.
ProviderInvocation<Brand> seals that ProviderValue with the exact selected call
and output-settlement authority. No JSON/Value/string/bytes/StructuralValue
parallel input, public conversion, re-admission, parts getter, or callback may
replace it.

PreparedExport::drive is the one mechanical drive owner. The retained backend
moves the existing drive under its private ReservedCustody/DriveCustody; no
second drive, compatibility drive, callback driver, or caller-minted receipt is
introduced.

Every typed refusal/fault/cancellation/unwind/shutdown terminal owns the exact
backend, branded invocation cargo, reservation/dispatch permit, process,
containment, pipe, pump, channel, join, output ledger, and settlement custody
required by its phase. No terminal returns RAW parts or a backend husk.
```

### E. Regenerate the complete dependency closure

After the source-plan correction:

1. Regenerate all six deterministic reports and verify them from a second
   independent Lane.
2. Update the generator hash in all four manifests.
3. Update attachment hashes, byte counts, totals, file counts, and Fable rows
   for owner, lifecycle, containment/release, and synthesis.
4. Rerun all four Oracle dry-runs because the attachment sets and token counts
   changed; keep Oracle and Fable `NOT LAUNCHED` and preserve the 272,000-token
   cap.
5. Update the three part-manifest hashes embedded in the synthesis manifest,
   then regenerate the synthesis dry-run and Fable plan.
6. Extend verifier `ADJACENT_PATHS`, `REQUIRED_ATTACHMENTS`, report sections,
   expected review files, and owner `PART_TERMS` so deleting any W1-10,
   Cargo, pool, shutdown, fixture, or prompt input fails closed.
7. Run both verifier commands from the fresh review Lane and require literal
   exit 0 before requesting another part review.

## Launch ruling

Oracle and Fable remain `NOT LAUNCHED`. The corrected adjacent core is useful,
but the failed independent replay and omitted W1-10/lifecycle source prevent a
source-aware `PART BUNDLE PASS`. Apply the exact correction above, regenerate
the full manifest/dry-run closure, and obtain a fresh independent part verdict
before any model launch or W1-11/W1-12 positive source edit.
