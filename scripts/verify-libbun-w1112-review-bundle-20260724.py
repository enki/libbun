#!/usr/bin/env python3
"""Fail-closed correction-2 verifier for the libbun W1-11/W1-12 review bundle."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

SOURCE_SHA = "6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb"
SOURCE_TREE = "cb964de8ab8162449fbe95959bf34d231570aa5c"
REVIEW_BASE = "ec6a7f249120a833aeaa4e0211fe0f41d17e0565"
REVIEW_BASE_TREE = "6da13ed79ca5df4554b7c0bf3c89cde7d9dcea0d"
SWARM_SHA = "95323ff17cb29928e31467f651ef03bae2099c14"
SWARM_TREE = "43b47bbd49a6053d270b3e15cc141cb1b1bb86da"
VERDICT_COMMIT = "b046f85a3dd41ac86cabed2de6391876ea77c0f4"
BASE = Path("docs/reviews/libbun-w1112-20260724")
SNAPSHOT_BASE = BASE / f"adjacent-swarm-{SWARM_SHA}"
VERIFIER = Path("scripts/verify-libbun-w1112-review-bundle-20260724.py")
GENERATOR = Path("scripts/generate-libbun-w1112-review-evidence-20260724.py")
PARTS = ("owner-generative", "lifecycle", "containment-release", "synthesis")
TOKEN_CAP = 272_000

PRIOR_VERDICTS = {
    "owner-generative": "bd86b8863ed21c19fa46bfdf1a006d8a83ff0330",
    "lifecycle": "7a5bfc1cf71299681a9edfb8d4f5a8a7501494e1",
    "containment-release": "54241a683e1c68366715456b4517bcf2966bbdf7",
}

ADJACENT_PATHS = (
    "crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs",
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs",
    "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
    "crates/swarm-provider-host-set/src/external_transport.rs",
    "crates/ss-runtime-external-capability-provider-owner/Cargo.toml",
    "Cargo.toml",
    "crates/swarm-provider-host-set/src/provider_host_set.rs",
    "crates/ss/src/product.rs",
    "crates/ss/tests/external_capability_provider.rs",
    "crates/ss/Cargo.toml",
    "crates/swarm-provider-host-set/Cargo.toml",
    "tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss",
    "tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss",
)

LOCK_PRIVACY_COMPLIANCE = {
    "Cargo.lock", "native/Cargo.lock", "runtime/Cargo.lock",
    "tests/fixtures/public_api_boundary/Cargo.lock",
    "tests/fixtures/public_api_boundary/Cargo.toml",
    "tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs",
    "tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs",
    "tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs",
    "tests/public_api_boundary.rs", "LICENSE", "vendor/README.md",
    "vendor/bun.LIBBUN_VENDOR.json", "vendor/bun/LICENSE.md",
    "vendor/bun/Cargo.lock",
}

PART_TERMS = {
    "owner-generative": (
        "BunProviderBackend", "SelectedProviderPackage", "ProviderInvocation",
        "ManifestResolvedExternalProviderCallAuthority",
        "DurableExternalProviderInvocationAuthority", "OfferCustody",
        "OfferReadyProof", "ReservedCustody", "ReservationReleaseProof",
        "libbun-only generative reconstruction as impossible",
        "acyclic concrete owner boundary", "CONCRETE IMPLEMENTATION",
        "exact first positive", "typed-fault",
    ),
    "lifecycle": (
        "BunProviderBackend", "DriveCustody", "InvocationReadyProof",
        "RetirementProof", "RetirementQuarantine", "DurableReaper",
        "QuarantineObservation", "QuarantineCompletionClaim", "RetiredDisposal",
        "request_termination", "notify_need_termination",
        "JSC__VM__deinit body is empty", "proven drained",
        "consumes BunProviderBackend", "CONCRETE IMPLEMENTATION",
    ),
    "containment-release": (
        "BunProviderBackend", "Linux namespace", "macOS", "Windows job",
        "persistent bounded", "same-worker", "replacement epoch",
        "all four nonvendored locks", "six-file privacy harness",
        "LICENSE", "vendored Cargo.lock", "immutable-tag",
        "freshly extracted", "CONCRETE IMPLEMENTATION",
    ),
    "synthesis": (
        "producer -> exact selected call", "BunProviderBackend",
        "generatively branded", "JSC__VM__deinit", "retirement/quarantine",
        "durable reaper", "platform containment", "locks", "compliance",
        "Fifteen-Step Hard-Cut", "CONCRETE IMPLEMENTATION",
    ),
}

CORE_ADJACENT = {
    str(SNAPSHOT_BASE / "crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs"),
    str(SNAPSHOT_BASE / "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs"),
    str(SNAPSHOT_BASE / "crates/ss-runtime-external-capability-provider-owner/src/lib.rs"),
    str(SNAPSHOT_BASE / "crates/swarm-provider-host-set/src/external_transport.rs"),
    str(SNAPSHOT_BASE / "crates/ss-runtime-external-capability-provider-owner/Cargo.toml"),
    str(SNAPSHOT_BASE / "Cargo.toml"),
}

REQUIRED_ATTACHMENTS = {
    "owner-generative": {
        "src/lib.rs", "src/prepared_export.rs", "tests/public_api_boundary.rs",
        str(BASE / "owner-generative-independent-verdict.md"),
        str(BASE / "adjacent-swarm-source-index.md"),
        str(BASE / "process-drop-caller-and-fixture-report.md"),
        *CORE_ADJACENT,
    },
    "lifecycle": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "runtime/src/main.rs", "vendor/bun/src/jsc/VM.rs",
        "vendor/bun/src/jsc/virtual_machine_exports.rs",
        str(BASE / "lifecycle-independent-verdict.commit"),
        str(BASE / "lifecycle-correction-ruling.md"),
        str(BASE / "lifecycle-vendored-jsc-source-bundle.md"),
        str(BASE / "process-drop-caller-and-fixture-report.md"),
        str(SNAPSHOT_BASE / "crates/swarm-provider-host-set/src/provider_host_set.rs"),
        str(SNAPSHOT_BASE / "crates/ss/src/product.rs"),
        str(SNAPSHOT_BASE / "crates/ss/tests/external_capability_provider.rs"),
        *CORE_ADJACENT,
    },
    "containment-release": {
        "src/prepared_export.rs", "native/src/lib.rs", "native/build.rs",
        "wire/src/lib.rs", "runtime/src/main.rs", "runtime/build.rs",
        ".github/workflows/ci.yml",
        str(BASE / "containment-release-independent-verdict.md"),
        str(BASE / "vendored-bun-boundary-report.md"),
        str(BASE / "lock-privacy-compliance-index.md"),
        str(GENERATOR), *LOCK_PRIVACY_COMPLIANCE,
    },
    "synthesis": {
        *(str(BASE / f"{part}-manifest.json") for part in PARTS[:3]),
        str(BASE / "owner-generative-independent-verdict.md"),
        str(BASE / "lifecycle-independent-verdict.commit"),
        str(BASE / "lifecycle-correction-ruling.md"),
        str(BASE / "containment-release-independent-verdict.md"),
        str(BASE / "lifecycle-vendored-jsc-source-bundle.md"),
        str(BASE / "lock-privacy-compliance-index.md"),
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
    },
}

TOP_LEVEL_REVIEW_FILES = {
    *(BASE / f"{part}-manifest.json" for part in PARTS),
    *(BASE / f"{part}-prompt.md" for part in PARTS),
    *(BASE / f"{part}-files.txt" for part in PARTS),
    *(BASE / f"{part}-oracle-dry-run.txt" for part in PARTS),
    *(BASE / f"{part}-fable-plan.md" for part in PARTS),
    BASE / ".gitattributes",
    BASE / "adjacent-swarm-source-index.md",
    BASE / "containment-release-independent-verdict.md",
    BASE / "correction2-index.md",
    BASE / "exact-source-search-report.md",
    BASE / "lifecycle-correction-ruling.md",
    BASE / "lifecycle-independent-verdict.commit",
    BASE / "lifecycle-vendored-jsc-source-bundle.md",
    BASE / "lock-privacy-compliance-index.md",
    BASE / "owner-generative-independent-verdict.md",
    BASE / "process-drop-caller-and-fixture-report.md",
    BASE / "vendored-bun-boundary-report.md",
    BASE / "verdict-snapshot.md",
}
EXPECTED_REVIEW_FILES = TOP_LEVEL_REVIEW_FILES | {
    SNAPSHOT_BASE / path for path in ADJACENT_PATHS
}


class Failure(Exception):
    pass


def fail(message: str) -> None:
    raise Failure(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def run(*args: str, cwd: Path | None = None, check: bool = True) -> str:
    result = subprocess.run(
        args,
        cwd=cwd or ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and result.returncode:
        fail(f"command failed ({result.returncode}): {' '.join(args)}\n{result.stderr}")
    return result.stdout


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_bytes(repo: Path, revision: str) -> bytes:
    result = subprocess.run(
        ["git", "show", revision],
        cwd=repo,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode:
        fail(f"cannot read {revision} from {repo}: {result.stderr.decode(errors='replace')}")
    return result.stdout


def read_lines(path: Path) -> list[str]:
    require(path.is_file(), f"missing file: {path}")
    raw = path.read_text()
    require(raw.endswith("\n"), f"file lacks final newline: {path}")
    lines = raw.splitlines()
    require(all(line and line == line.strip() for line in lines), f"invalid path line in {path}")
    return lines


def verify_identity_and_delta() -> None:
    require(run("git", "rev-parse", f"{SOURCE_SHA}^{{tree}}").strip() == SOURCE_TREE,
            "candidate SHA does not resolve to the frozen product tree")
    require(run("git", "rev-parse", f"{REVIEW_BASE}^{{tree}}").strip() == REVIEW_BASE_TREE,
            "review base does not resolve to the frozen bundle tree")
    require(run("git", "rev-parse", f"{SWARM_SHA}^{{tree}}", cwd=SWARM_ROOT).strip() == SWARM_TREE,
            "adjacent swarm SHA does not resolve to the frozen tree")
    for ancestor in (SOURCE_SHA, REVIEW_BASE):
        result = subprocess.run(["git", "merge-base", "--is-ancestor", ancestor, "HEAD"], cwd=ROOT)
        require(result.returncode == 0, f"{ancestor} is not an ancestor of HEAD")

    allowed_prefix = str(BASE) + "/"
    allowed_scripts = {str(VERIFIER), str(GENERATOR)}
    changed = set(run("git", "diff", "--name-only", SOURCE_SHA, "--").splitlines())
    changed.update(run("git", "diff", "--name-only", "--cached", "--").splitlines())
    changed.update(run("git", "diff", "--name-only", "--").splitlines())
    changed.update(run("git", "ls-files", "--others", "--exclude-standard").splitlines())
    forbidden = sorted(
        path for path in changed
        if path and not path.startswith(allowed_prefix) and path not in allowed_scripts
    )
    require(not forbidden, "product/test/Cargo/workflow delta detected: " + ", ".join(forbidden))

    actual = {path.relative_to(ROOT) for path in (ROOT / BASE).rglob("*") if path.is_file()}
    missing = sorted(str(path) for path in EXPECTED_REVIEW_FILES - actual)
    extra = sorted(str(path) for path in actual - EXPECTED_REVIEW_FILES)
    require(not missing, "missing review artifacts: " + ", ".join(missing))
    require(not extra, "unexpected review artifacts: " + ", ".join(extra))
    for script in (VERIFIER, GENERATOR):
        require((ROOT / script).is_file(), f"missing review script: {script}")


def verify_verdicts_snapshots_and_generator() -> None:
    frozen = git_bytes(
        ROOT,
        f"{VERDICT_COMMIT}:docs/LIBBUN-W1112-FINAL-COMPOSITION-REVIEW-20260724.md",
    )
    require((ROOT / BASE / "verdict-snapshot.md").read_bytes() == frozen,
            "frozen final-composition verdict snapshot drift")

    owner_revision = (
        f"{PRIOR_VERDICTS['owner-generative']}:"
        "docs/reviews/libbun-w1112-20260724/owner-generative-independent-verdict.md"
    )
    require((ROOT / BASE / "owner-generative-independent-verdict.md").read_bytes()
            == git_bytes(ROOT, owner_revision), "owner REVISE verdict snapshot drift")
    containment_revision = (
        f"{PRIOR_VERDICTS['containment-release']}:"
        "docs/reviews/libbun-w1112-20260724/containment-release-bundle-review.md"
    )
    require((ROOT / BASE / "containment-release-independent-verdict.md").read_bytes()
            == git_bytes(ROOT, containment_revision),
            "containment REVISE verdict snapshot drift")

    lifecycle_object = run("git", "cat-file", "-p", PRIOR_VERDICTS["lifecycle"]).encode()
    require((ROOT / BASE / "lifecycle-independent-verdict.commit").read_bytes()
            == lifecycle_object, "lifecycle empty-commit verdict record drift")
    lifecycle_tree = run("git", "rev-parse", f"{PRIOR_VERDICTS['lifecycle']}^{{tree}}").strip()
    lifecycle_parent_tree = run(
        "git", "rev-parse", f"{PRIOR_VERDICTS['lifecycle']}^^{{tree}}"
    ).strip()
    require(lifecycle_tree == lifecycle_parent_tree == REVIEW_BASE_TREE,
            "lifecycle verdict is no longer the exact empty-tree BUNDLE REVISE commit")

    for path in ADJACENT_PATHS:
        snapshot = ROOT / SNAPSHOT_BASE / path
        require(snapshot.read_bytes() == git_bytes(SWARM_ROOT, f"{SWARM_SHA}:{path}"),
                f"adjacent source snapshot drift: {path}")

    check = subprocess.run(
        [sys.executable, str(GENERATOR), "--check"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    require(check.returncode == 0, "deterministic evidence replay failed:\n" + check.stdout + check.stderr)
    require(check.stdout.count("OK ") == 6, "generator did not verify all six reports")


def verify_reports() -> None:
    search = (ROOT / BASE / "exact-source-search-report.md").read_text()
    require(SOURCE_SHA in search and SOURCE_TREE in search
            and SWARM_SHA in search and SWARM_TREE in search,
            "exact-source report lacks both exact repository identities")
    required_sections = (
        "Required owner and lifecycle definitions (expected negative)",
        "Native/wire public and RAW bridge shapes",
        "Process containment, raw handle, and join topology",
        "Output drain, overflow, barrier, EOF, and diagnostic topology",
        "Lifecycle, refusal, retry, cancellation, unwind, Drop, and shutdown topology",
        "Package, lock, license, compliance, release, and extracted-smoke topology",
        "Current test and external privacy fixture definitions",
        "Adjacent exact-call and invocation producers",
        "Adjacent sole consumer, transport, process, and shutdown graph",
    )
    for section in required_sections:
        require(section in search, f"exact-source report lacks section: {section}")
    require(search.count("Command: git -C ") == len(required_sections),
            "exact-source report lacks a literal command for a search family")
    require(search.count("Pattern: ") == len(required_sections),
            "exact-source report lacks an exact pattern for a search family")
    require(search.count("Pathspecs: ") == len(required_sections),
            "exact-source report lacks exact pathspecs")
    require(search.count("Expected result: ") == len(required_sections),
            "exact-source report lacks expected-exit meaning")
    require("Exit: 1" in search and "Exit 1 means every named required definition is absent" in search,
            "expected-negative search is unlabeled")
    require(search.count("Exit: 1") == 1, "an unexpected source-search family returned exit 1")

    vendor = (ROOT / BASE / "vendored-bun-boundary-report.md").read_text()
    require("awk:" not in vendor, "vendored report retains awk diagnostics")
    for term in (
        "No shell pipeline or awk transformation participates",
        "Git blob", "SHA-256", "Bytes", "Excerpt line span", "Excerpt SHA-256",
        "vendor/bun/src/jsc/bindings/bindings.cpp",
    ):
        require(term in vendor, f"vendored report lacks {term}")

    jsc = (ROOT / BASE / "lifecycle-vendored-jsc-source-bundle.md").read_text()
    for term in (
        "VirtualMachine.rs", "VM.rs", "JSGlobalObject.rs",
        "bindings/bindings.cpp", "ZigGlobalObject.cpp",
        "JSC__VM__deinit", "empty body", "requestTermination",
        "clearTerminationException", "notifyNeedTermination",
        "Excerpt SHA-256",
    ):
        require(term in jsc, f"JSC lifecycle bundle lacks {term}")

    callers = (ROOT / BASE / "process-drop-caller-and-fixture-report.md").read_text()
    for term in (
        "Libbun process, thread, Drop, cancellation, and shutdown callers",
        "Vendored VM termination, reset, drain, and deinit callers",
        "Adjacent consumer, transport, process, Drop, and shutdown callers",
        "Adjacent external fixture graph",
    ):
        require(term in callers, f"caller/fixture report lacks {term}")

    compliance = (ROOT / BASE / "lock-privacy-compliance-index.md").read_text()
    for path in LOCK_PRIVACY_COMPLIANCE:
        require(f"| {path} |" in compliance, f"lock/privacy/compliance index lacks {path}")


def parse_fable_rows(text: str) -> list[tuple[str, str, int]]:
    rows: list[tuple[str, str, int]] = []
    pattern = re.compile(r"^\| (\d+) \| ([^|]+) \| ([0-9a-f]{64}) \| (\d+) \|$")
    for line in text.splitlines():
        match = pattern.match(line)
        if match:
            index = int(match.group(1))
            require(index == len(rows) + 1, "Fable attachment indices are not contiguous")
            rows.append((match.group(2).strip(), match.group(3), int(match.group(4))))
    return rows


def verify_part(part: str) -> dict:
    manifest_path = ROOT / BASE / f"{part}-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    require(manifest.get("schema") == "libbun.w1112.external-review-manifest.v2",
            f"{part}: wrong manifest schema")
    require(manifest.get("correction") == 2, f"{part}: wrong correction number")
    require(manifest.get("part") == part, f"{part}: manifest identity mismatch")
    require(manifest.get("exact_source_sha") == SOURCE_SHA, f"{part}: wrong product SHA")
    require(manifest.get("exact_source_tree") == SOURCE_TREE, f"{part}: wrong product tree")
    require(manifest.get("review_base_commit") == REVIEW_BASE, f"{part}: wrong review base")
    adjacent = manifest.get("adjacent_source", {})
    require((adjacent.get("repository"), adjacent.get("sha"), adjacent.get("tree"))
            == ("/home/ubuntu/swarm", SWARM_SHA, SWARM_TREE),
            f"{part}: wrong adjacent source identity")
    require(manifest.get("verdict_contract_commit") == VERDICT_COMMIT,
            f"{part}: wrong verdict contract")
    require(manifest.get("deliverable") == "CONCRETE IMPLEMENTATION",
            f"{part}: wrong deliverable")

    generator = manifest.get("evidence_generator", {})
    require(generator.get("path") == str(GENERATOR)
            and generator.get("sha256") == digest(ROOT / GENERATOR)
            and generator.get("check_command")
            == f"python3 {GENERATOR} --check",
            f"{part}: evidence generator binding drift")

    prompt_path = Path(manifest["prompt"]["path"])
    require(prompt_path == BASE / f"{part}-prompt.md", f"{part}: wrong prompt path")
    require(digest(ROOT / prompt_path) == manifest["prompt"]["sha256"],
            f"{part}: prompt hash drift")
    prompt = (ROOT / prompt_path).read_text()
    require(SOURCE_SHA in prompt and SWARM_SHA in prompt,
            f"{part}: prompt lacks exact dual-repository identity")
    for term in PART_TERMS[part]:
        require(term in prompt, f"{part}: prompt lacks required correction term: {term}")

    plan = manifest["ordered_file_plan"]
    plan_path = Path(plan["path"])
    require(plan_path == BASE / f"{part}-files.txt", f"{part}: wrong plan path")
    require(digest(ROOT / plan_path) == plan["sha256"], f"{part}: path-block hash drift")
    paths = read_lines(ROOT / plan_path)
    require(len(paths) == len(set(paths)), f"{part}: duplicate attachment")
    require(len(paths) == plan["count"], f"{part}: plan count mismatch")
    require(REQUIRED_ATTACHMENTS[part].issubset(set(paths)),
            f"{part}: required correction source/evidence attachment missing")

    attachments = manifest["ordered_attachments"]
    require([item["path"] for item in attachments] == paths,
            f"{part}: missing, duplicate, extra, or reordered manifest attachment")
    total_bytes = 0
    for item in attachments:
        path = ROOT / item["path"]
        require(path.is_file(), f"{part}: missing attachment {item['path']}")
        require(digest(path) == item["sha256"], f"{part}: hash drift for {item['path']}")
        require(path.stat().st_size == item["bytes"],
                f"{part}: byte-count drift for {item['path']}")
        total_bytes += item["bytes"]
    require(total_bytes == manifest["total_attachment_bytes"],
            f"{part}: total attachment bytes drift")

    oracle = manifest["oracle"]
    require((oracle.get("provider"), oracle.get("engine"), oracle.get("model"),
             oracle.get("reasoning_mode"))
            == ("openai", "api", "gpt-5.6-sol", "pro"),
            f"{part}: wrong Oracle route/model/reasoning")
    require(oracle.get("required_live_banner") == [
        "first-party OpenAI", "gpt-5.6-sol", "Responses API Pro", "xhigh reasoning"
    ], f"{part}: incomplete live banner gate")
    require(oracle.get("state") == "NOT LAUNCHED", f"{part}: Oracle launch state changed")
    require(all(oracle.get(key) is None for key in ("session_id", "request_id", "response_id")),
            f"{part}: Oracle identifiers show a launch")
    require(oracle.get("output_paths") == [], f"{part}: Oracle outputs show a launch")
    require(oracle["estimated_total_tokens"] < TOKEN_CAP, f"{part}: token cap exceeded")

    dry_path = Path(oracle["dry_run_report"]["path"])
    require(dry_path == BASE / f"{part}-oracle-dry-run.txt", f"{part}: wrong dry-run path")
    require(digest(ROOT / dry_path) == oracle["dry_run_report"]["sha256"],
            f"{part}: dry-run hash drift")
    dry = (ROOT / dry_path).read_text()
    for term in (
        "[oracle-policy] provider=openai engine=api model=gpt-5.6-sol reasoning-mode=pro",
        "[dry-run]", "would call gpt-5.6-sol", f"and {len(paths)} files.",
        f"Total: {oracle['estimated_total_tokens']:,} tokens",
        f"({oracle['estimated_total_tokens'] / TOKEN_CAP * 100:.2f}% of 272,000)",
    ):
        require(term in dry, f"{part}: dry-run lacks {term!r}")

    fable = manifest["fable"]
    require((fable.get("model"), fable.get("effort"), fable.get("state"))
            == ("claude-fable-5", "max", "NOT LAUNCHED"),
            f"{part}: wrong Fable model/effort/state")
    require(all(fable.get(key) is None for key in ("session_id", "request_id", "response_id")),
            f"{part}: Fable identifiers show a launch")
    require(fable.get("output_paths") == [], f"{part}: Fable outputs show a launch")
    fable_path = Path(fable["file_plan"]["path"])
    require(fable_path == BASE / f"{part}-fable-plan.md", f"{part}: wrong Fable plan path")
    require(digest(ROOT / fable_path) == fable["file_plan"]["sha256"],
            f"{part}: Fable plan hash drift")
    fable_text = (ROOT / fable_path).read_text()
    require("- State: NOT LAUNCHED" in fable_text
            and "- Model: claude-fable-5" in fable_text
            and "- Effort: max" in fable_text,
            f"{part}: incomplete Fable configuration")
    require(parse_fable_rows(fable_text)
            == [(item["path"], item["sha256"], item["bytes"]) for item in attachments],
            f"{part}: Fable/Oracle attachment mismatch")

    require(manifest.get("launch_state") == "NOT LAUNCHED", f"{part}: launch state changed")
    require("PENDING" in manifest.get("correction_evidence_state", ""),
            f"{part}: correction evidence state does not preserve fresh-review gate")
    bundle_review = manifest.get("independent_bundle_review", {})
    require(bundle_review.get("reviewer", "").startswith("PENDING correction-2")
            and "PENDING" in bundle_review.get("verdict", ""),
            f"{part}: fresh independent-review gate changed")
    require(manifest.get("omissions"), f"{part}: omissions must remain explicit")

    if part != "synthesis":
        prior = manifest.get("prior_independent_verdict", {})
        require(prior.get("commit") == PRIOR_VERDICTS[part]
                and prior.get("verdict") == "BUNDLE REVISE",
                f"{part}: prior REVISE verdict binding drift")
        for record in prior.get("records", []):
            path = ROOT / record["path"]
            require(path.is_file() and digest(path) == record["sha256"],
                    f"{part}: prior verdict record hash drift")
    return manifest


def verify_synthesis(manifests: dict[str, dict]) -> None:
    synth = manifests["synthesis"]
    inputs = synth.get("synthesis_inputs", [])
    require([item.get("part") for item in inputs] == list(PARTS[:3]),
            "synthesis part order mismatch")
    for item in inputs:
        part = item["part"]
        path = BASE / f"{part}-manifest.json"
        require(item.get("manifest_path") == str(path)
                and item.get("manifest_sha256") == digest(ROOT / path),
                f"synthesis input drift for {part}")
        require("FRESH BUNDLE PASS PENDING" in item.get("state", ""),
                f"synthesis prematurely authorizes {part}")
    prior = synth.get("prior_independent_verdicts", [])
    require([(item.get("part"), item.get("commit"), item.get("verdict")) for item in prior]
            == [(part, PRIOR_VERDICTS[part], "BUNDLE REVISE") for part in PARTS[:3]],
            "synthesis prior-verdict order/binding drift")


def main() -> int:
    verify_identity_and_delta()
    verify_verdicts_snapshots_and_generator()
    verify_reports()
    manifests = {part: verify_part(part) for part in PARTS}
    verify_synthesis(manifests)
    print(
        "PASS: correction-2 bundles bind both exact repositories, all three "
        "REVISE verdicts, replayable evidence, zero product delta, NOT LAUNCHED "
        "plans, and sub-272k token budgets."
    )
    for part in PARTS:
        manifest = manifests[part]
        print(
            f"{part}: {manifest['ordered_file_plan']['count']} files; "
            f"{manifest['oracle']['estimated_total_tokens']} tokens"
        )
    return 0


ROOT = Path(
    subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    ).stdout.strip()
)
SWARM_ROOT = Path("/home/ubuntu/swarm")

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
