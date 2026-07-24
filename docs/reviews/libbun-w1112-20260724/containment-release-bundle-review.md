BUNDLE REVISE

# W1-11/W1-12 Containment And Release Bundle Review

Date: 2026-07-24

## Bound review identity

- Review bundle commit: `ec6a7f249120a833aeaa4e0211fe0f41d17e0565`
- Review bundle tree: `6da13ed79ca5df4554b7c0bf3c89cde7d9dcea0d`
- Exact product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Part manifest SHA-256: `a6d19bc45336c727a1c6662e425760178291c84481bc91df0b8d05483c03d831`
- Ordered file-plan SHA-256: `cf723736eb055d33a6543893f9be3c7025bb0b6fbaaa3369a8ccbf6ad8f73ff6`
- Prompt SHA-256: `cb898e74365e4932208772f60450f59b14b68680e4fd919f50ec40a721788f53`
- Oracle dry-run SHA-256: `541d07aff856c19ac3ad099c0fa004edb174483f7e000383abb7010a536d8322`
- Fable plan SHA-256: `784922d9f2303e3285ab4806a686f72e897e281a746033bfb006b7817ee6c723`
- Exact-source report SHA-256: `8460cf4417ad0ef6742cbf2e82bd573dff2a9a8d2b0afff939e5e8657769ef93`
- Vendored-boundary report SHA-256: `4b3b06f539b945a52e44012e1d065cc7202b98f0bc861f4f0119763edef863f7`
- Model launch: `none`; Oracle and Fable remain `NOT LAUNCHED`.

The 35-file plan is internally hash-consistent, stays below the token cap, and
contains the current facade, worker, native, wire, build, packaging, CI, and
vendored-patch implementation files. It is not yet source-complete enough to
request an exact-SHA-applicable concrete implementation.

## Determining omissions

### 1. The vendored boundary identity failed to generate

In `vendored-bun-boundary-report.md`, the section named
`[tracked boundary file hashes]` contains seven repeated `awk` failures and
prints paths without any hash. The report therefore does not bind its sole
vendored `bindings.cpp` excerpt, build inputs, or patch bodies to independently
checkable source bytes.

The clean replacement must remove every `awk:` diagnostic and record at least
`SHA-256`, Git blob OID, byte length, and path for the intended boundary set.
The exact values independently obtained from the frozen product source are:

```text
c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee 29b85eb51d9b35735369831c50c1b6a3df3f2a17 5960 patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch
4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 d263927c2a5209a8cebabd6f1cc5dc982536b142 804 patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch
4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d b2d755d810a45a4c972953cdf5521174bcd10925 5195 native/build.rs
c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 b889b6059332d8adb8c40dd70177084f93e52b77 4902 runtime/build.rs
498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 2b44b7e3ec653f663ee8857bc7fd188b5cff113b 3233 scripts/prepare-native-bun-link.sh
e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca a895f3402c48a07fefc650ee455e79ebb34de770 41 BUN_SOURCE_COMMIT
e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff b08737cc9ba97c08fccd8d291f73cc03275031d0 255398 vendor/bun/src/jsc/bindings/bindings.cpp
```

Every bounded vendored excerpt must additionally record its exact path, blob,
line or byte span, and excerpt SHA-256 so a reviewer can distinguish current
source from copied terminal output.

### 2. The exact-source searches are not reproducible

`exact-source-search-report.md` labels a required-symbol absence section and
records only `exit=1`; it does not record the symbol expression, pathspec, or
command. Neither report records the command that generated its content. The
part therefore cannot prove which required constructors were absent or that
source-wide release, output, `Drop`, raw-handle, caller-proof, and shutdown
searches were actually complete.

The replacement report must print each exact command before its output and
must include independent positive and negative searches over the frozen SHA
for:

- native/wire public APIs, raw constructors, parts, selectors, clone, serde,
  callbacks, caller-selected receipts, and public drive entry points;
- `Child`, `try_wait`, `wait`, `kill`, process groups, namespaces, sandboxing,
  job handles, raw descriptors/handles, channels, pumps, receivers, and join
  handles;
- output drains, bounded overflow, barriers, EOF, joins, and all stdout,
  stderr, Bun-output, diagnostic, and log paths;
- release, reservation, ready, retirement, reaper, quarantine, cancellation,
  unwind, consuming shutdown, every `Drop` implementation, and abort/panic;
- package/archive, linked/unlinked modes, fallback, locks, license, notice,
  compliance, CI, release workflow, symbol, and extracted-smoke paths; and
- every current test and external fixture definition that will be retained,
  deleted, or migrated.

An expected-negative search must name the complete searched symbol set and
the meaning of exit `1`; a bare exit code is not evidence.

### 3. Lock and privacy migration sources are missing

The prompt requires one coherent lock strategy including wire deletion,
repeat-lock stability, external privacy gates, and an exact-SHA-applicable
patch. The plan omits all four nonvendored current lockfiles and the complete
external fixture suite:

```text
Cargo.lock
native/Cargo.lock
runtime/Cargo.lock
tests/fixtures/public_api_boundary/Cargo.lock
tests/fixtures/public_api_boundary/Cargo.toml
tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs
tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs
tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs
tests/public_api_boundary.rs
```

The search report names these paths but supplies only scattered matching
lines. That is insufficient to delete the public native/wire crates, converge
the lock topology, or extend the sibling-crate compile-fail harness without
inventing surrounding source.

### 4. Release/compliance inputs are missing

The prompt requires package compliance inventory, notices, license texts, and
immutable release assets. The bundle does not attach the current authoritative
inputs:

```text
LICENSE
vendor/README.md
vendor/bun.LIBBUN_VENDOR.json
vendor/bun/LICENSE.md
vendor/bun/Cargo.lock
```

Without these bytes, a concrete review cannot derive or verify the package's
license/notices/dependency inventory against the linked Bun inputs. They must
be attached directly, or replaced by a checked-in exact-SHA generated
compliance report that identifies every source path, blob, hash, dependency,
license, and packaged output it covers. A prose promise to add compliance is
not a substitute.

## Preserved owner and implementation shape

The highest owner remains opaque by-value `BunProviderBackend`. Its private
custody must retain the contained worker, exact epoch, protocol, persistent
output pumps, reservation/dispatch authority, finalization obligations,
retirement custody, and preallocated durable-queue node. Generatively
co-branded `SelectedProviderPackage<Brand>` and `ProviderInvocation<Brand>` are
the only selected inputs.

- Refusal retry requires private `OfferReadyProof` and returns unchanged
  selected custody on the same Ready worker and epoch.
- Pre-dispatch release requires sealed `ReservationReleaseProof`; ambiguity
  retires or transfers intact custody to the durable queue.
- Fulfilled/rejected/cooperative-cancel settlement requires
  `InvocationReadyProof` after output barriers and all per-invocation joins.
- Forced cancel, deadline, unwind, shutdown, or ambiguous finalization requires
  exact `RetirementProof` or by-value adoption of private
  `RetirementQuarantine<Purpose>`.
- Persistent stdout/stderr/Bun-output/log pumps are installed before selected
  work can reach Bun, keep draining after bounded overflow, use per-invocation
  barriers, and reserve EOF/join for retirement.
- Linux namespace containment, macOS spawn-denying sandboxing, and atomic
  Windows job assignment complete before dispatch. Unsupported platforms
  refuse; process groups are never a fallback.
- No raw worker/native/protocol handle, public completion receipt, observable
  quarantine identity, borrowed authority mint, callback proof, or
  caller-chosen proof enters the public surface.
- Drop is nonblocking and silently publishes unresolved custody to the
  preallocated durable queue; it never waits, joins, aborts, fabricates a
  terminal, or exposes recovery authority.

## Clean replacement order

Bundle correction must occur without product-source changes:

1. Add a checked-in deterministic report generator or exact command record.
2. Regenerate the vendored boundary report from product SHA `6066a5b8...`,
   with the corrected full-file identities above and bound excerpt spans.
3. Regenerate the exact-source report with literal commands, patterns,
   pathspecs, exits, and complete release/output/Drop/raw-handle/test searches.
4. Add the nine lock/privacy paths and the five compliance paths above to the
   containment ordered plan and manifest with exact SHA-256 and byte counts.
5. Regenerate the Fable ordered table and Oracle dry run from that identical
   order. Keep both states `NOT LAUNCHED`.
6. Extend the verifier to require the new sources; reject `awk:` diagnostics,
   unlabeled negative exits, absent search families, any plan/manifest/Fable
   order mismatch, and any product delta.
7. Update the synthesis input hash only after the corrected part manifest is
   committed, then obtain a fresh independent source-aware part verdict at
   that exact review commit.

After the owner/generative and lifecycle tranche establishes the opaque
backend and durable queue, the first containment-owned product edit is hard-cut
step 9: move the wire codec into private facade/runtime modules and redirect
the two exact owners before deleting the public `wire` crate. Then move the
native engine into binary-only runtime ownership and delete the native library,
`internal-adapter`, and public Rust drive entry point; establish exact
pre-dispatch containment; install persistent pumps; complete retained
release/dispatch/finalization/reaper integration; migrate package, locks, CI,
release workflow and compliance; finally delete every fresh-process/fallback
field and stale symbol. The public wire/native surfaces must not survive as an
intermediate compatibility layer.

## Required correction and implementation gates

The corrected bundle must pass:

```text
python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
rg -n '^awk:' docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md
  expected exit: 1
exact-source command/pattern/pathspec replay at 6066a5b8...
  expected: byte-identical report and explicit exits
ordered plan == manifest paths == Fable rows
all attachment SHA-256 and byte counts exact
Oracle dry run: provider=openai, engine=api, model=gpt-5.6-sol,
  reasoning-mode=pro, NOT LAUNCHED, under 272000 tokens
git diff --quiet 6066a5b8... -- product/test/Cargo/workflow paths
  expected: zero product delta for the bundle correction
```

The eventual implementation must use default-parallel hostile tests covering
external privacy, refusal/retry, proof-bound pre-dispatch release,
same-worker/same-epoch second invocation, rejection and undefined/null
distinction, cancellation/deadline/unwind/fault dominance, silent Drop,
reaper adoption/retry/panic, all three containment platforms on their release
runners, output saturation/overflow/barrier/EOF/join, linked nonzero execution,
lock stability, package/archive creation, compliance inventory, immutable-tag
release, and complete freshly extracted smoke execution. Source and binary
stale-symbol searches and independent full-SCC review follow those gates.

## Commands and observed results

- Lane identity matched owner `libbun-w1112-rv-containment-20260724`, exact
  worktree, target directory, and review HEAD.
- `python3 scripts/verify-libbun-w1112-review-bundle-20260724.py` exited `0`:
  the frozen artifacts are internally intact, product delta is zero, all plans
  are not launched, and the containment plan reports 35 files/93583 tokens.
- The exact Oracle dry-run command exited `0` and confirmed first-party OpenAI
  routing metadata, `gpt-5.6-sol`, API/pro mode, 35 files, and 93583 tokens.
  It launched no model.
- Independent SHA-256 checks matched the manifest, file plan, prompt, dry-run,
  Fable plan, exact-source report, and vendored-boundary report values recorded
  above.
- Independent source-wide searches at the product SHA found the current
  output/release/Drop/native/wire implementation in the attached core files,
  but found the omitted privacy, lock, and compliance paths listed above.
- Before this verdict was recorded, `git status --short` was empty and the
  Lane-path process audit reported
  `lane_path_processes_excluding_audit_shell=0`.

No model was launched and no product, test, Cargo, workflow, part-manifest,
Fable-plan, Oracle-report, synthesis, or source-report artifact was modified by
this independent review.
