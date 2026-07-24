#!/usr/bin/env python3
"""Fail-closed correction-3 verifier for the libbun W1-11/W1-12 review bundle."""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

SOURCE_SHA = "6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb"
SOURCE_TREE = "cb964de8ab8162449fbe95959bf34d231570aa5c"
REVIEW_BASE = "51b0118428d7881f39f32df396ef32349a5a52ab"
REVIEW_BASE_TREE = "55f063f9df9491aa6893c233eb2f15950bb428b2"
SWARM_SHA = "95323ff17cb29928e31467f651ef03bae2099c14"
SWARM_TREE = "43b47bbd49a6053d270b3e15cc141cb1b1bb86da"
VERDICT_COMMIT = "b046f85a3dd41ac86cabed2de6391876ea77c0f4"
BASE = Path("docs/reviews/libbun-w1112-20260724")
SNAPSHOT_BASE = BASE / f"adjacent-swarm-{SWARM_SHA}"
GENERATOR = Path("scripts/generate-libbun-w1112-review-evidence-20260724.py")
VERIFIER = Path("scripts/verify-libbun-w1112-review-bundle-20260724.py")
PARTS = ("owner-generative", "lifecycle", "containment-release", "synthesis")
TOKEN_CAP = 272_000

PRIOR_VERDICTS = {
    "owner-generative": (
        "47acbee0fe5a67231969efd8141ba8195bcecc8a",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction2-independent-verdict.md",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction2-independent-verdict.md",
    ),
    "lifecycle": (
        "0a3844d11bb42d67550da0bb1e069ecf17fbe69d",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction2-independent-verdict.md",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction2-independent-verdict.md",
    ),
    "containment-release": (
        "84feaf68aa99c5bc0e393cbfc1b6a92716cefdf1",
        "docs/reviews/libbun-w1112-containment-release-correction2-independent-verdict-20260724.md",
        "docs/reviews/libbun-w1112-20260724/containment-release-correction2-independent-verdict.md",
    ),
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
    "docs/PROVIDER_EXECUTION_AND_SDK_LAW.md",
    "docs/PROVIDER_VALUE_JSON_WIRE_V1.md",
    "docs/SWARMSCRIPT_ROADMAP.md",
    "docs/WAVE0_WAVE1_SEMANTIC_CLOSURE_INDEX.md",
    "crates/swarm-provider-value-model/src/lib.rs",
    "crates/swarm-provider-value-model/Cargo.toml",
    "crates/swarm-capability-linker-core/src/lib.rs",
    "crates/swarm-capability-linker-core/Cargo.toml",
    "crates/swarm-rust-sdk-static-provider-host/Cargo.toml",
    "crates/swarmvm-image/Cargo.toml",
    "tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss",
    "crates/ss-runtime-test-execution-owner/Cargo.toml",
    "crates/ss-runtime-test-execution-owner/src/lib.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
    "crates/ss-runtime-provider-host-set-owner/Cargo.toml",
    "crates/ss-runtime-provider-host-set-owner/src/lib.rs",
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
    "vendor/bun/Cargo.lock", "vendor/bun/Cargo.toml",
    "vendor/bun/src/clap/LICENSE",
    "vendor/bun/src/unicode/uucode_lib/LICENSE.md",
    "vendor/bun/vendor/lolhtml/LICENSE",
}

PART_TERMS = {
    "owner-generative": (
        "W1-10 ProviderValue is the sole by-value invocation input cargo",
        "ProviderInvocation<Brand>", "BunProviderBackend", "PreparedExport::drive",
        "mechanical-only", "OfferCustody", "OfferReadyProof", "ReservedCustody",
        "ReservationReleaseProof", "libbun-only generative reconstruction as impossible",
        "acyclic concrete owner boundary", "caller-minted receipt", "CONCRETE IMPLEMENTATION",
    ),
    "lifecycle": (
        "BunProviderBackend", "DriveCustody", "InvocationReadyProof", "RetirementProof",
        "RetirementQuarantine", "DurableReaper", "QuarantineObservation",
        "QuarantineCompletionClaim", "RetiredDisposal", "request_termination",
        "notify_need_termination", "JSC__VM__deinit body is empty", "proven drained",
        "ExternalCapabilityProviderPool", "working-directory replacement",
        "shutdown consumes BunProviderBackend", "CONCRETE IMPLEMENTATION",
    ),
    "containment-release": (
        "BunProviderBackend", "Linux namespace", "macOS", "Windows job",
        "persistent bounded", "same-worker", "replacement epoch", "all four nonvendored locks",
        "complete six-file privacy harness", "vendored workspace/lock", "exact-tree Cargo/license inventory",
        "immutable-tag", "freshly extracted", "caller-minted receipt", "CONCRETE IMPLEMENTATION",
    ),
    "synthesis": (
        "producer -> exact selected call", "W1-10 ProviderValue", "BunProviderBackend",
        "generatively branded", "PreparedExport::drive", "mechanical-only settlement",
        "JSC__VM__deinit", "retirement/quarantine/durable reaper", "platform containment",
        "Fifteen-Step Hard-Cut", "CONCRETE IMPLEMENTATION",
    ),
}

ALL_ADJACENT = {str(SNAPSHOT_BASE / path) for path in ADJACENT_PATHS}
POOL_ADJACENT = {
    str(SNAPSHOT_BASE / path) for path in ADJACENT_PATHS
    if "ss-runtime-test-execution-owner" in path or "ss-runtime-provider-host-set-owner" in path
}
CORE_ADJACENT = {
    str(SNAPSHOT_BASE / "crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs"),
    str(SNAPSHOT_BASE / "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs"),
    str(SNAPSHOT_BASE / "crates/ss-runtime-external-capability-provider-owner/src/lib.rs"),
    str(SNAPSHOT_BASE / "crates/swarm-provider-host-set/src/external_transport.rs"),
}

REQUIRED_ATTACHMENTS = {
    "owner-generative": {
        "src/lib.rs", "src/prepared_export.rs", "tests/public_api_boundary.rs",
        str(BASE / "owner-generative-correction2-independent-verdict.md"),
        str(BASE / "adjacent-swarm-source-index.md"), *ALL_ADJACENT,
    },
    "lifecycle": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "vendor/bun/src/jsc/VirtualMachine.rs", "vendor/bun/src/jsc/JSGlobalObject.rs",
        "vendor/bun/src/jsc/VM.rs", "vendor/bun/src/jsc/virtual_machine_exports.rs",
        str(BASE / "lifecycle-correction2-independent-verdict.md"),
        str(BASE / "lifecycle-vendored-jsc-source-bundle.md"),
        str(BASE / "process-drop-caller-and-fixture-report.md"), *POOL_ADJACENT,
        str(SNAPSHOT_BASE / "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs"),
    },
    "containment-release": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        ".github/workflows/ci.yml",
        str(BASE / "containment-release-correction2-independent-verdict.md"),
        str(BASE / "lifecycle-vendored-jsc-source-bundle.md"),
        str(BASE / "process-drop-caller-and-fixture-report.md"),
        str(BASE / "lock-privacy-compliance-index.md"),
        *LOCK_PRIVACY_COMPLIANCE, *CORE_ADJACENT,
    },
    "synthesis": {
        *(str(BASE / f"{part}-manifest.json") for part in PARTS[:3]),
        str(BASE / "owner-generative-correction2-independent-verdict.md"),
        str(BASE / "lifecycle-correction2-independent-verdict.md"),
        str(BASE / "containment-release-correction2-independent-verdict.md"),
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "vendor/bun/src/jsc/VirtualMachine.rs", "vendor/bun/src/jsc/JSGlobalObject.rs",
        "vendor/bun/src/jsc/VM.rs", "vendor/bun/src/jsc/virtual_machine_exports.rs",
    },
}

TOP_LEVEL_NAMES = {
    ".gitattributes", "adjacent-swarm-source-index.md",
    "containment-release-correction2-independent-verdict.md",
    "containment-release-fable-plan.md", "containment-release-files.txt",
    "containment-release-independent-verdict.md", "containment-release-manifest.json",
    "containment-release-oracle-dry-run.txt", "containment-release-prompt.md",
    "correction2-index.md", "correction3-index.md", "exact-source-search-report.md",
    "lifecycle-correction-ruling.md", "lifecycle-correction2-independent-verdict.md",
    "lifecycle-fable-plan.md", "lifecycle-files.txt", "lifecycle-independent-verdict.commit",
    "lifecycle-manifest.json", "lifecycle-oracle-dry-run.txt", "lifecycle-prompt.md",
    "lifecycle-vendored-jsc-source-bundle.md", "lock-privacy-compliance-index.md",
    "owner-generative-correction2-independent-verdict.md", "owner-generative-fable-plan.md",
    "owner-generative-files.txt", "owner-generative-independent-verdict.md",
    "owner-generative-manifest.json", "owner-generative-oracle-dry-run.txt",
    "owner-generative-prompt.md", "process-drop-caller-and-fixture-report.md",
    "synthesis-fable-plan.md", "synthesis-files.txt", "synthesis-manifest.json",
    "synthesis-oracle-dry-run.txt", "synthesis-prompt.md", "vendored-bun-boundary-report.md",
    "verdict-snapshot.md",
}
EXPECTED_REVIEW_FILES = {BASE / name for name in TOP_LEVEL_NAMES} | {
    SNAPSHOT_BASE / path for path in ADJACENT_PATHS
}


class Failure(Exception):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Failure(message)


def command(*args: str, cwd: Path | None = None, check: bool = True,
            env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        args, cwd=cwd or ROOT, env=env, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    if check and result.returncode:
        raise Failure(f"command failed ({result.returncode}): {' '.join(args)}\n{result.stderr}")
    return result


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_bytes(repo: Path, revision: str) -> bytes:
    result = subprocess.run(
        ["git", "show", revision], cwd=repo,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    require(result.returncode == 0, f"cannot read {revision}: {result.stderr.decode(errors='replace')}")
    return result.stdout


def read_plan(path: Path) -> list[str]:
    require(path.is_file(), f"missing file plan: {path}")
    text = path.read_text()
    require(text.endswith("\n"), f"file plan lacks final newline: {path}")
    lines = text.splitlines()
    require(all(line and line == line.strip() for line in lines), f"invalid file-plan row: {path}")
    require(len(lines) == len(set(lines)), f"duplicate file-plan row: {path}")
    return lines


def verify_identity_and_delta() -> None:
    require(command("git", "rev-parse", f"{SOURCE_SHA}^{{tree}}").stdout.strip() == SOURCE_TREE,
            "frozen product tree mismatch")
    require(command("git", "rev-parse", f"{REVIEW_BASE}^{{tree}}").stdout.strip() == REVIEW_BASE_TREE,
            "correction-2 review base tree mismatch")
    require(command("git", "rev-parse", f"{SWARM_SHA}^{{tree}}", cwd=SWARM_ROOT).stdout.strip() == SWARM_TREE,
            "adjacent Swarm tree mismatch")
    for ancestor in (SOURCE_SHA, REVIEW_BASE):
        require(command("git", "merge-base", "--is-ancestor", ancestor, "HEAD", check=False).returncode == 0,
                f"required ancestor missing: {ancestor}")

    allowed_prefix = str(BASE) + "/"
    allowed_scripts = {str(GENERATOR), str(VERIFIER)}
    changed = set(command("git", "diff", "--name-only", SOURCE_SHA, "--").stdout.splitlines())
    changed.update(command("git", "diff", "--name-only", "--cached", "--").stdout.splitlines())
    changed.update(command("git", "diff", "--name-only", "--").stdout.splitlines())
    changed.update(command("git", "ls-files", "--others", "--exclude-standard").stdout.splitlines())
    forbidden = sorted(
        path for path in changed
        if path and not path.startswith(allowed_prefix) and path not in allowed_scripts
    )
    require(not forbidden, "product/test/Cargo/vendor/workflow delta detected: " + ", ".join(forbidden))

    actual = {path.relative_to(ROOT) for path in (ROOT / BASE).rglob("*") if path.is_file()}
    require(EXPECTED_REVIEW_FILES == actual,
            "review artifact set mismatch; missing="
            + ",".join(map(str, sorted(EXPECTED_REVIEW_FILES - actual)))
            + " extra=" + ",".join(map(str, sorted(actual - EXPECTED_REVIEW_FILES))))


def verify_verdicts_and_snapshots() -> None:
    frozen = git_bytes(
        ROOT, f"{VERDICT_COMMIT}:docs/LIBBUN-W1112-FINAL-COMPOSITION-REVIEW-20260724.md"
    )
    require((ROOT / BASE / "verdict-snapshot.md").read_bytes() == frozen,
            "final-composition verdict snapshot drift")
    for part, (commit, source, destination) in PRIOR_VERDICTS.items():
        require((ROOT / destination).read_bytes() == git_bytes(ROOT, f"{commit}:{source}"),
                f"{part}: correction-2 verdict snapshot drift")
    for path in ADJACENT_PATHS:
        require((ROOT / SNAPSHOT_BASE / path).read_bytes() == git_bytes(SWARM_ROOT, f"{SWARM_SHA}:{path}"),
                f"adjacent exact snapshot drift: {path}")


def run_generator_check(repo: Path) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["LIBBUN_REPO"] = str(repo)
    env["SWARM_REPO"] = str(SWARM_ROOT)
    return command(sys.executable, str(GENERATOR), "--check", cwd=repo, check=False, env=env)


def verify_generator_replay() -> None:
    local = run_generator_check(ROOT)
    require(local.returncode == 0, "local deterministic replay failed:\n" + local.stdout + local.stderr)
    require(local.stdout.count("OK ") == 6, "local generator did not verify all six reports")

    require(not command("git", "status", "--porcelain").stdout,
            "independent-checkout replay requires the correction commit to be clean")
    head = command("git", "rev-parse", "HEAD").stdout.strip()
    with tempfile.TemporaryDirectory(prefix="libbun-w1112-c3-independent-") as temporary:
        clone = Path(temporary) / "different-checkout-name"
        command("git", "clone", "--shared", "--quiet", str(ROOT), str(clone), cwd=Path(temporary))
        command("git", "checkout", "--detach", "--quiet", head, cwd=clone)
        require(not command("git", "status", "--porcelain", cwd=clone).stdout,
                "independent replay checkout is not clean before generation")
        replay = run_generator_check(clone)
        require(replay.returncode == 0,
                "clean independent-checkout deterministic replay failed:\n" + replay.stdout + replay.stderr)
        require(replay.stdout.count("OK ") == 6,
                "independent generator did not verify all six reports")
        require(not command("git", "status", "--porcelain", cwd=clone).stdout,
                "independent replay dirtied its checkout")


def verify_reports() -> None:
    report_paths = (
        BASE / "exact-source-search-report.md", BASE / "process-drop-caller-and-fixture-report.md"
    )
    for path in report_paths:
        text = (ROOT / path).read_text()
        require("/home/ubuntu/bridge-ops/dev-worktrees" not in text,
                f"construction Lane path serialized in {path}")
        require("Command: git -C /" not in text and "Command: git -C '$" not in text,
                f"absolute repository command serialized in {path}")
        require('git -C "$LIBBUN_REPO"' in text or 'git -C "$SWARM_REPO"' in text,
                f"stable repository labels absent from {path}")

    search = (ROOT / report_paths[0]).read_text()
    sections = (
        "Required owner and lifecycle definitions (expected negative)",
        "Native/wire public and RAW bridge shapes",
        "Process containment, raw handle, and join topology",
        "Output drain, overflow, barrier, EOF, and diagnostic topology",
        "Lifecycle, refusal, retry, cancellation, unwind, Drop, and shutdown topology",
        "Selected package and lock topology", "Privacy harness topology",
        "License, provenance, and compliance topology", "Release and extracted-smoke topology",
        "Current test and external privacy fixture definitions",
        "Adjacent W1-10 ProviderValue input and governing law",
        "Adjacent exact-call and invocation producers",
        "Adjacent sole consumer, transport, retained-host pool, and shutdown graph",
        "Adjacent package and dependency direction",
    )
    for section in sections:
        require(f"## {section}" in search, f"exact-source report lacks {section}")
    for field in ("Command: git -C ", "Pattern: ", "Pathspecs: ", "Expected result: ", "Exit: "):
        require(search.count(field) == len(sections), f"search report field count mismatch: {field}")
    require(search.count("Exit: 1") == 1 and "Exit 1 means every named required definition is absent" in search,
            "expected-negative search is not the sole exit-1 section")
    for path in LOCK_PRIVACY_COMPLIANCE:
        require(path in search, f"attached package/privacy/compliance input not searched: {path}")
    for path in ADJACENT_PATHS:
        if path.endswith("Cargo.toml"):
            require(path in search, f"attached adjacent package not searched: {path}")

    jsc = (ROOT / BASE / "lifecycle-vendored-jsc-source-bundle.md").read_text()
    for term in (
        "directly attaches complete `VirtualMachine.rs`, `JSGlobalObject.rs`, `VM.rs`",
        "bindings/bindings.cpp", "ZigGlobalObject.cpp", "JSC__VM__deinit", "empty body",
        "requestTermination", "clearTerminationException", "notifyNeedTermination", "Excerpt SHA-256",
    ):
        require(term in jsc, f"JSC supplemental bundle lacks {term}")

    callers = (ROOT / report_paths[1]).read_text()
    for term in (
        "Libbun process, thread, Drop, cancellation, and shutdown callers",
        "Vendored VM termination, reset, drain, and deinit callers",
        "retained-host pool", "ExternalCapabilityProviderPool", "working_directory",
        "Adjacent external fixture graph",
    ):
        require(term in callers, f"caller/lifecycle report lacks {term}")

    compliance = (ROOT / BASE / "lock-privacy-compliance-index.md").read_text()
    for path in LOCK_PRIVACY_COMPLIANCE:
        require(f"| {path} |" in compliance, f"compliance selection lacks {path}")
    tracked = command("git", "ls-tree", "-r", "--name-only", SOURCE_SHA).stdout.splitlines()
    inventory = [
        path for path in tracked
        if path.endswith(("Cargo.toml", "Cargo.lock"))
        or re.search(r"(^|/)(LICENSE|LICENCE|NOTICE|COPYING)([._-].*)?$", path, re.IGNORECASE)
    ]
    for path in inventory:
        require(f"| {path} |" in compliance, f"exact-tree compliance inventory lacks {path}")


def parse_fable_rows(text: str) -> list[tuple[str, str, int]]:
    rows: list[tuple[str, str, int]] = []
    pattern = re.compile(r"^\| (\d+) \| ([^|]+) \| ([0-9a-f]{64}) \| (\d+) \|$")
    for line in text.splitlines():
        match = pattern.match(line)
        if match:
            require(int(match.group(1)) == len(rows) + 1, "Fable indices are not contiguous")
            rows.append((match.group(2).strip(), match.group(3), int(match.group(4))))
    return rows


def verify_part(part: str) -> dict[str, object]:
    manifest_path = ROOT / BASE / f"{part}-manifest.json"
    data = json.loads(manifest_path.read_text())
    require(data.get("schema") == "libbun.w1112.external-review-manifest.v3", f"{part}: schema drift")
    require(data.get("correction") == 3 and data.get("part") == part, f"{part}: identity drift")
    require(data.get("exact_source_sha") == SOURCE_SHA and data.get("exact_source_tree") == SOURCE_TREE,
            f"{part}: product identity drift")
    require(data.get("review_base_commit") == REVIEW_BASE, f"{part}: review-base drift")
    require(data.get("adjacent_source") == {
        "repository": "SWARM_REPO", "sha": SWARM_SHA, "tree": SWARM_TREE
    }, f"{part}: adjacent identity or stable label drift")
    require(data.get("verdict_contract_commit") == VERDICT_COMMIT, f"{part}: verdict contract drift")
    require(data.get("deliverable") == "CONCRETE IMPLEMENTATION", f"{part}: deliverable drift")

    generator = data.get("evidence_generator", {})
    require(generator.get("path") == str(GENERATOR)
            and generator.get("sha256") == digest(ROOT / GENERATOR)
            and "LIBBUN_REPO=<libbun-checkout>" in generator.get("check_command", "")
            and "SWARM_REPO=<swarm-checkout>" in generator.get("check_command", ""),
            f"{part}: generator binding drift")
    verifier = data.get("bundle_verifier", {})
    require(verifier.get("path") == str(VERIFIER)
            and verifier.get("sha256") == digest(ROOT / VERIFIER)
            and verifier.get("independent_checkout_replay") is True,
            f"{part}: verifier binding drift")

    prompt_path = BASE / f"{part}-prompt.md"
    require(data.get("prompt") == {"path": str(prompt_path), "sha256": digest(ROOT / prompt_path)},
            f"{part}: prompt binding drift")
    prompt = (ROOT / prompt_path).read_text()
    require(SOURCE_SHA in prompt and SWARM_SHA in prompt, f"{part}: dual-source identity absent")
    for term in PART_TERMS[part]:
        require(term in prompt, f"{part}: prompt lacks required invariant: {term}")

    plan_path = BASE / f"{part}-files.txt"
    paths = read_plan(ROOT / plan_path)
    plan = data.get("ordered_file_plan", {})
    require(plan == {"path": str(plan_path), "sha256": digest(ROOT / plan_path), "count": len(paths)},
            f"{part}: ordered plan binding drift")
    require(REQUIRED_ATTACHMENTS[part].issubset(paths), f"{part}: required exact source missing")

    attachments = data.get("ordered_attachments", [])
    require([item.get("path") for item in attachments] == paths, f"{part}: manifest order drift")
    expected_rows = []
    total_bytes = 0
    for item in attachments:
        path = ROOT / item["path"]
        require(path.is_file(), f"{part}: missing attachment {item['path']}")
        require(item.get("sha256") == digest(path) and item.get("bytes") == path.stat().st_size,
                f"{part}: attachment identity drift {item['path']}")
        total_bytes += path.stat().st_size
        expected_rows.append((item["path"], item["sha256"], item["bytes"]))
    require(data.get("total_attachment_bytes") == total_bytes, f"{part}: byte total drift")

    oracle = data.get("oracle", {})
    require((oracle.get("provider"), oracle.get("engine"), oracle.get("model"), oracle.get("reasoning_mode"))
            == ("openai", "api", "gpt-5.6-sol", "pro"), f"{part}: Oracle policy drift")
    require(oracle.get("required_live_banner") == [
        "first-party OpenAI", "gpt-5.6-sol", "Responses API Pro", "xhigh reasoning"
    ], f"{part}: live banner gate drift")
    require(oracle.get("state") == "NOT LAUNCHED"
            and all(oracle.get(key) is None for key in ("session_id", "request_id", "response_id"))
            and oracle.get("output_paths") == [], f"{part}: Oracle launch state changed")
    require(isinstance(oracle.get("estimated_total_tokens"), int)
            and oracle["estimated_total_tokens"] < TOKEN_CAP, f"{part}: token cap exceeded")
    dry_path = BASE / f"{part}-oracle-dry-run.txt"
    require(oracle.get("dry_run_report") == {"path": str(dry_path), "sha256": digest(ROOT / dry_path)},
            f"{part}: dry-run binding drift")
    dry = (ROOT / dry_path).read_text()
    for term in (
        "[oracle-policy] provider=openai engine=api model=gpt-5.6-sol reasoning-mode=pro",
        "[dry-run]", "would call gpt-5.6-sol", f"and {len(paths)} files.",
        f"Total: {oracle['estimated_total_tokens']:,} tokens",
        f"({oracle['estimated_total_tokens'] / TOKEN_CAP * 100:.2f}% of 272,000)",
    ):
        require(term in dry, f"{part}: dry-run lacks {term!r}")

    fable = data.get("fable", {})
    require((fable.get("model"), fable.get("effort"), fable.get("state"))
            == ("claude-fable-5", "max", "NOT LAUNCHED"), f"{part}: Fable policy drift")
    require(all(fable.get(key) is None for key in ("session_id", "request_id", "response_id"))
            and fable.get("output_paths") == [], f"{part}: Fable launch state changed")
    fable_path = BASE / f"{part}-fable-plan.md"
    require(fable.get("file_plan") == {"path": str(fable_path), "sha256": digest(ROOT / fable_path)},
            f"{part}: Fable plan binding drift")
    fable_text = (ROOT / fable_path).read_text()
    require("- State: NOT LAUNCHED" in fable_text and "- Model: claude-fable-5" in fable_text
            and "- Effort: max" in fable_text, f"{part}: Fable configuration incomplete")
    require(parse_fable_rows(fable_text) == expected_rows, f"{part}: Fable/Oracle input mismatch")

    require(data.get("launch_state") == "NOT LAUNCHED", f"{part}: launch state changed")
    require("PENDING" in data.get("correction_evidence_state", ""), f"{part}: review gate lost")
    review = data.get("independent_bundle_review", {})
    require(review.get("reviewer", "").startswith("PENDING correction-3")
            and "PART BUNDLE PASS" in review.get("verdict", ""), f"{part}: literal pass gate drift")
    if part != "synthesis":
        commit, _, destination = PRIOR_VERDICTS[part]
        prior = data.get("prior_independent_verdict", {})
        require(prior.get("commit") == commit and prior.get("verdict") == "PART BUNDLE REVISE",
                f"{part}: correction-2 verdict binding drift")
        require(prior.get("records") == [{
            "path": destination, "sha256": digest(ROOT / destination)
        }], f"{part}: correction-2 verdict record drift")
    return data


def verify_synthesis(manifests: dict[str, dict[str, object]]) -> None:
    synthesis = manifests["synthesis"]
    inputs = synthesis.get("synthesis_inputs", [])
    require([item.get("part") for item in inputs] == list(PARTS[:3]), "synthesis part order drift")
    for item in inputs:
        part = item["part"]
        path = BASE / f"{part}-manifest.json"
        require(item.get("manifest_path") == str(path)
                and item.get("manifest_sha256") == digest(ROOT / path)
                and item.get("state") == "FRESH LITERAL PART BUNDLE PASS PENDING",
                f"synthesis input drift: {part}")
    expected = [
        {"part": part, "commit": PRIOR_VERDICTS[part][0], "verdict": "PART BUNDLE REVISE"}
        for part in PARTS[:3]
    ]
    require(synthesis.get("prior_independent_verdicts") == expected,
            "synthesis correction-2 verdict binding drift")


def main() -> int:
    verify_identity_and_delta()
    verify_verdicts_and_snapshots()
    verify_reports()
    manifests = {part: verify_part(part) for part in PARTS}
    verify_synthesis(manifests)
    verify_generator_replay()
    print(
        "PASS: correction-3 bundles are Lane-independent, exact-source complete, "
        "clean-checkout replayable, zero-product-delta, NOT LAUNCHED, and sub-272k."
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
        ["git", "rev-parse", "--show-toplevel"], text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True,
    ).stdout.strip()
).resolve()
SWARM_ROOT = Path(os.environ.get("SWARM_REPO", "/home/ubuntu/swarm")).resolve()

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
