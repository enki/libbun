PART BUNDLE REVISE

# W1-11/W1-12 Containment And Release Correction-2 Independent Verdict

Date: 2026-07-24

## Reviewed identity

- Exact correction-2 candidate: `51b0118428d7881f39f32df396ef32349a5a52ab`
- Candidate tree: `55f063f9df9491aa6893c233eb2f15950bb428b2`
- Exact libbun product source: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Exact libbun product tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Correction review base: `ec6a7f249120a833aeaa4e0211fe0f41d17e0565`
- Exact adjacent Swarm source: `95323ff17cb29928e31467f651ef03bae2099c14`
- Exact adjacent Swarm tree: `43b47bbd49a6053d270b3e15cc141cb1b1bb86da`
- Containment manifest SHA-256:
  `789e3f8c86b8cb37964528d160e07d350e8205b919cd041bda3b35841bc37569`
- Prior containment REVISE record SHA-256:
  `ba792c4113a6e8af5097c636f45a5858b15073409d508f17d2eb384167628837`
- Lane owner: `libbun-w1112-c2-rv-containment-20260724`
- Lane worktree:
  `/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-c2-rv-containment-20260724`
- Lane cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-11`
- Review scope: evidence and source review only; no product/test edit and no
  Oracle or Fable launch.

`lane run` resolved the exact owner, worktree, cargo target, detached HEAD, and
clean starting state. The candidate has zero product, test, Cargo, workflow,
vendor, or package-source delta from the exact product source; its only allowed
delta is correction-2 review evidence and the two evidence scripts.

## Mechanical evidence

The following correction claims reproduce:

- the manifest and ordered plan contain the same 53 paths in the same order;
- all 53 manifest SHA-256 values and byte counts match their attachments;
- the Oracle and Fable plans remain `NOT LAUNCHED` and bind the same ordered
  attachments;
- the Oracle estimate is 262438 tokens, below the 272000-token cap;
- all four nonvendored lockfiles and all six files of the external privacy
  harness are direct attachments;
- `LICENSE`, vendor provenance, the Bun license, and the vendored lock are
  direct attachments; and
- four of the six deterministic reports replay byte-for-byte.

The required fail-closed gate does not pass:

```text
python3 scripts/verify-libbun-w1112-review-bundle-20260724.py
exit: 1
FAIL: deterministic evidence replay failed
DRIFT docs/reviews/libbun-w1112-20260724/exact-source-search-report.md
DRIFT docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md
```

Running the generator directly with `--check` returns the same two `DRIFT`
results. Normalizing only the generating and reviewing Lane paths makes both
generated reports byte-identical, proving the observed output and source
matches are otherwise stable.

## Determining defects

### 1. The deterministic evidence serializes its construction Lane

`command_text` in
`scripts/generate-libbun-w1112-review-evidence-20260724.py` renders
`str(ROOT)`. The two checked-in reports consequently contain nine commands
rooted at:

```text
/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724
```

The assigned independent Lane correctly regenerates those commands with its
own worktree path. Byte replay must therefore fail in every differently named
Lane even when every searched Git object and result is exact. The manifest
binds the generator and both drifting report hashes, and the verifier requires
all six reports to replay, so a literal part pass is unavailable.

The exact generator correction is:

```diff
 def command_text(repo: Path, args: list[str]) -> str:
-    return "git -C " + shlex.quote(str(repo)) + " " + " ".join(shlex.quote(arg) for arg in args)
+    if repo == ROOT:
+        displayed_repo = "."
+    elif repo == SWARM_ROOT:
+        displayed_repo = "/home/ubuntu/swarm"
+    else:
+        raise ValueError(f"unbound evidence repository: {repo}")
+    return "git -C " + shlex.quote(displayed_repo) + " " + " ".join(
+        shlex.quote(arg) for arg in args
+    )
```

The verifier must also reject a serialized construction-Lane prefix in either
report and run the generator from the repository root. Then replay must pass
both in the producing checkout and a differently named clean Lane.

### 2. The package/lock/license search does not search locks or licenses

The section titled `Package, lock, license, compliance, release, and
extracted-smoke topology` uses these pathspecs:

```text
Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml scripts
.github README.md docs vendor/README.md vendor/bun.LIBBUN_VENDOR.json
```

It does not search any `Cargo.lock`, `LICENSE`, `vendor/bun/LICENSE.md`, or
`vendor/bun/Cargo.lock` attachment. A grep pattern containing the text
`Cargo.lock|license` does not inventory filenames excluded by the pathspec.
Thus correction 2 attaches the prior missing bytes but does not satisfy the
prior verdict's reproducible lock/license/compliance search requirement.

An independent tree inventory at the exact product SHA finds 140 tracked Cargo
manifests/locks and 15 tracked license/notice-family files. The part attaches
10 and 2 respectively. Full vendored test/tool closure need not enter this
bounded part, but the linked native source closure cannot be selected from the
vendored lock alone. At minimum the current plan omits these directly relevant
inputs:

```text
vendor/bun/Cargo.toml
  blob e1232a4cf29435189b91182b901f9a724da999b5
vendor/bun/src/clap/LICENSE
  blob cf1ab25da0349f84a3fdd40032f0ce99db813b8b
vendor/bun/src/unicode/uucode_lib/LICENSE.md
  blob 412454e31dfa9bac8c6ba8263cd3c49e87dfd1ce
vendor/bun/vendor/lolhtml/LICENSE
  blob 98b3bec0935e5c2539f70348d2151e1d9b7f00b3
```

The first file defines the vendored workspace and dependency selection. The
next two correspond to linked Bun/JSC dependencies, and the repository's own
vendor provenance explicitly names the linked lolhtml source dependency.

Replace the grep-only compliance section with two deterministic inventories:

1. all tracked Cargo manifests and locks, followed by the exact linked native
   package closure selected from `native/Cargo.toml`, the vendored workspace,
   and the locked graph; and
2. all tracked license/notice paths, followed by the exact applicable linked
   and source-package license set.

Each selected row must record path, Git blob, SHA-256, bytes, selection reason,
and the linked package that requires it. Attach `vendor/bun/Cargo.toml` and all
selected license texts directly, or attach a checked-in exact-source compliance
bundle containing their complete bytes. Amend the index's claim from “all
current” to the proved linked/package closure unless every tracked input is
actually covered.

### 3. The ordered part cannot support its requested full-SCC implementation

The correction-2 prompt requests a concrete implementation across every
repository that must move and requires JSC interrupt/quiescence, teardown,
Drop/shutdown, and adjacent Swarm consumer migration. The 53-file containment
plan attaches none of:

```text
docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md
docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md
docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md
docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/**
```

It also attaches no direct `vendor/bun/src/jsc/` source. Broad grep output can
name operations, but it cannot supply exact surrounding definitions for a
commit-grade change. The correction index says those source bundles exist;
their absence from this part's Oracle/Fable order prevents the requested
reviewer from verifying or editing the complete producer -> opaque backend ->
interrupt/finalization -> release/shutdown SCC.

Attach the complete-item JSC termination/reset/drain/deinit excerpts, the
process/Drop/shutdown report, and the exact adjacent producer, consumer,
transport, Cargo, and fixture snapshots needed by this part. If the resulting
bundle exceeds the token cap, replace broad repeated grep output with compact
complete-item excerpts and identity tables. Do not weaken the requested
deliverable or omit the ownership transition.

## Preserved implementation ruling

The bundle correction does not change the frozen implementation shape. The
highest owner remains opaque by-value `BunProviderBackend`. It privately owns
the exact contained worker, epoch, protocol, persistent bounded output pumps,
reservation/dispatch custody, typed finalization obligations, retirement
custody, and preallocated durable-queue node.

- Linux namespace containment, macOS spawn-denying sandboxing, and atomic
  Windows Job assignment must complete before selected work reaches Bun.
- `OfferReadyProof`, `ReservationReleaseProof`, `InvocationReadyProof`, and
  `RetirementProof` remain distinct sealed evidence; ambiguity transfers the
  intact private quarantine by value before any public typed fault.
- Cooperative JSC interrupt can preserve the same worker and epoch only after
  exact invocation, microtask, diagnostic, output-barrier, and join quiescence.
  Empty `JSC__VM__deinit` is not retirement evidence.
- Drop remains nonblocking silent adoption; consuming shutdown retains typed
  failure custody and cannot return a backend husk.
- The wire and native engines move behind private facade/runtime ownership;
  public frames, `DriveRequest`, `drive_prepared_export`, raw descriptors,
  callbacks, caller-minted receipts, and prepared-export RAW escape are
  deleted rather than bridged.

After the evidence correction, regenerate the two reports, compliance index,
ordered plan, manifest, Oracle dry run, Fable plan, all generator bindings, and
synthesis input hashes. Reproduce all attachment identities, prove zero
product delta, and make the fail-closed verifier pass in an independently named
Lane before requesting a fresh literal `PART BUNDLE PASS`.

No product/test source, Cargo file, workflow, vendor source, or model state was
changed by this review.
