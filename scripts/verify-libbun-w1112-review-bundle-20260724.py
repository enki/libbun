#!/usr/bin/env python3
"""Fail-closed integrity verifier for the libbun W1-11/W1-12 review bundle."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

SOURCE_SHA = "6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb"
SOURCE_TREE = "cb964de8ab8162449fbe95959bf34d231570aa5c"
VERDICT_COMMIT = "b046f85a3dd41ac86cabed2de6391876ea77c0f4"
BASE = Path("docs/reviews/libbun-w1112-20260724")
VERIFIER = Path("scripts/verify-libbun-w1112-review-bundle-20260724.py")
PARTS = ("owner-generative", "lifecycle", "containment-release", "synthesis")
TOKEN_CAP = 272_000

PART_TERMS = {
    "owner-generative": (
        "BunProviderBackend", "SelectedProviderPackage", "ProviderInvocation",
        "OfferCustody", "OfferReadyProof", "ReservedCustody", "PreparedExport",
        "ReservationReleaseProof", "CONCRETE IMPLEMENTATION",
        "exact first positive", "external privacy", "typed fault",
    ),
    "lifecycle": (
        "BunProviderBackend", "DriveCustody", "InvocationReadyProof",
        "RetirementProof", "RetirementQuarantine", "DurableReaper",
        "QuarantineObservation", "QuarantineCompletionClaim", "RetiredDisposal",
        "consumes the backend", "CONCRETE IMPLEMENTATION",
        "exact first positive", "Drop", "typed fault",
    ),
    "containment-release": (
        "BunProviderBackend", "wire", "native", "Linux namespace",
        "macOS", "Windows job", "persistent bounded", "same-worker",
        "replacement epoch", "immutable-tag", "freshly extracted",
        "CONCRETE IMPLEMENTATION", "exact first positive", "hostile",
    ),
    "synthesis": (
        "producer -> branded admission", "BunProviderBackend",
        "generative brands", "quarantine", "reaper", "platform containment",
        "output pumps", "packaging", "Fifteen-Step Hard-Cut",
        "CONCRETE IMPLEMENTATION", "exact first positive", "hostile",
    ),
}

REQUIRED_SOURCE_ATTACHMENTS = {
    "owner-generative": {
        "src/lib.rs", "src/prepared_export.rs", "Cargo.toml",
        "tests/public_api_boundary.rs",
        "docs/LIBBUN-LIFECYCLE-CONTRACT.md",
    },
    "lifecycle": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "runtime/src/main.rs", "docs/LIBBUN-LIFECYCLE-CONTRACT.md",
        "docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md",
    },
    "containment-release": {
        "src/prepared_export.rs", "native/src/lib.rs", "native/build.rs",
        "wire/src/lib.rs", "runtime/src/main.rs", "runtime/build.rs",
        ".github/workflows/ci.yml",
        "scripts/package-prepared-export-worker-release.sh",
        "scripts/prepare-native-bun-link.sh",
        "docs/LIBBUN-WORKER-RELEASE-CONTRACT.md",
    },
    "synthesis": {
        str(BASE / "owner-generative-manifest.json"),
        str(BASE / "lifecycle-manifest.json"),
        str(BASE / "containment-release-manifest.json"),
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "runtime/src/main.rs",
    },
}

EXPECTED_REVIEW_FILES = {
    *(BASE / f"{part}-manifest.json" for part in PARTS),
    *(BASE / f"{part}-prompt.md" for part in PARTS),
    *(BASE / f"{part}-files.txt" for part in PARTS),
    *(BASE / f"{part}-oracle-dry-run.txt" for part in PARTS),
    *(BASE / f"{part}-fable-plan.md" for part in PARTS),
    BASE / "exact-source-search-report.md",
    BASE / "vendored-bun-boundary-report.md",
    BASE / "verdict-snapshot.md",
}


class Failure(Exception):
    pass


def fail(message: str) -> None:
    raise Failure(message)


def run(*args: str, check: bool = True) -> str:
    result = subprocess.run(
        args,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and result.returncode:
        fail(f"command failed ({result.returncode}): {' '.join(args)}\n{result.stderr}")
    return result.stdout


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read_lines(path: Path) -> list[str]:
    require(path.is_file(), f"missing file: {path}")
    raw = path.read_text()
    require(raw.endswith("\n"), f"file lacks final newline: {path}")
    lines = raw.splitlines()
    require(all(line and line == line.strip() for line in lines), f"invalid path line in {path}")
    return lines


def verify_identity_and_delta() -> None:
    require(run("git", "rev-parse", f"{SOURCE_SHA}^{{tree}}").strip() == SOURCE_TREE,
            "candidate SHA does not resolve to the frozen tree")
    ancestor = subprocess.run(
        ["git", "merge-base", "--is-ancestor", SOURCE_SHA, "HEAD"], cwd=ROOT
    )
    require(ancestor.returncode == 0, "candidate SHA is not an ancestor of HEAD")

    allowed_prefix = str(BASE) + "/"
    allowed_exact = str(VERIFIER)
    changed = set(run("git", "diff", "--name-only", SOURCE_SHA, "--").splitlines())
    changed.update(run("git", "diff", "--name-only", "--cached", "--").splitlines())
    changed.update(run("git", "diff", "--name-only", "--").splitlines())
    changed.update(run("git", "ls-files", "--others", "--exclude-standard").splitlines())
    forbidden = sorted(
        path for path in changed
        if path and not path.startswith(allowed_prefix) and path != allowed_exact
    )
    require(not forbidden, "product/test/Cargo/workflow delta detected: " + ", ".join(forbidden))

    actual_review = {p.relative_to(ROOT) for p in (ROOT / BASE).glob("*") if p.is_file()}
    missing = sorted(str(p) for p in EXPECTED_REVIEW_FILES - actual_review)
    extra = sorted(str(p) for p in actual_review - EXPECTED_REVIEW_FILES)
    require(not missing, "missing review artifacts: " + ", ".join(missing))
    require(not extra, "unexpected review artifacts: " + ", ".join(extra))
    require((ROOT / VERIFIER).is_file(), f"missing verifier: {VERIFIER}")


def verify_verdict_and_reports() -> None:
    verdict_path = ROOT / BASE / "verdict-snapshot.md"
    expected = run(
        "git", "show",
        f"{VERDICT_COMMIT}:docs/LIBBUN-W1112-FINAL-COMPOSITION-REVIEW-20260724.md",
    ).encode()
    require(verdict_path.read_bytes() == expected,
            "verdict snapshot differs from the frozen verdict-contract blob")

    for report in ("exact-source-search-report.md", "vendored-bun-boundary-report.md"):
        text = (ROOT / BASE / report).read_text()
        require(SOURCE_SHA in text and SOURCE_TREE in text,
                f"{report} lacks exact candidate identity")
    search = (ROOT / BASE / "exact-source-search-report.md").read_text()
    for term in (
        "required implementation symbols; expected absent",
        "current forbidden/rejected shapes", "test definitions",
        "crate and workflow topology",
    ):
        require(term in search, f"source search report lacks section: {term}")
    vendor = (ROOT / BASE / "vendored-bun-boundary-report.md").read_text()
    for term in (
        "tracked boundary file hashes", "libbun patch bodies",
        "patched call-frame definitions and references",
        "native integration imports and call sites", "build/link integration",
    ):
        require(term in vendor, f"vendored boundary report lacks section: {term}")


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
    require(manifest.get("schema") == "libbun.w1112.external-review-manifest.v1",
            f"{part}: wrong manifest schema")
    require(manifest.get("part") == part, f"{part}: manifest identity mismatch")
    require(manifest.get("exact_source_sha") == SOURCE_SHA, f"{part}: wrong source SHA")
    require(manifest.get("exact_source_tree") == SOURCE_TREE, f"{part}: wrong source tree")
    require(manifest.get("verdict_contract_commit") == VERDICT_COMMIT,
            f"{part}: wrong verdict commit")
    require(manifest.get("deliverable") == "CONCRETE IMPLEMENTATION",
            f"{part}: wrong deliverable")

    prompt_path = Path(manifest["prompt"]["path"])
    require(prompt_path == BASE / f"{part}-prompt.md", f"{part}: wrong prompt path")
    require(digest(ROOT / prompt_path) == manifest["prompt"]["sha256"],
            f"{part}: prompt hash drift")
    prompt = (ROOT / prompt_path).read_text()
    require(SOURCE_SHA in prompt, f"{part}: prompt lacks exact source SHA")
    for term in PART_TERMS[part]:
        require(term in prompt, f"{part}: prompt lacks required semantic/test term: {term}")

    plan = manifest["ordered_file_plan"]
    plan_path = Path(plan["path"])
    require(plan_path == BASE / f"{part}-files.txt", f"{part}: wrong file-plan path")
    require(digest(ROOT / plan_path) == plan["sha256"], f"{part}: path-block hash drift")
    paths = read_lines(ROOT / plan_path)
    require(len(paths) == len(set(paths)), f"{part}: duplicate attachment")
    require(len(paths) == plan["count"], f"{part}: file count mismatch")
    require(REQUIRED_SOURCE_ATTACHMENTS[part].issubset(set(paths)),
            f"{part}: required source/contract attachment missing")

    attachments = manifest["ordered_attachments"]
    require([item["path"] for item in attachments] == paths,
            f"{part}: missing, extra, or reordered manifest attachment")
    total_bytes = 0
    for item in attachments:
        path = ROOT / item["path"]
        require(path.is_file(), f"{part}: missing attachment {item['path']}")
        require(digest(path) == item["sha256"], f"{part}: hash drift for {item['path']}")
        require(path.stat().st_size == item["bytes"], f"{part}: byte-count drift for {item['path']}")
        total_bytes += item["bytes"]
    require(total_bytes == manifest["total_attachment_bytes"],
            f"{part}: total attachment byte mismatch")

    oracle = manifest["oracle"]
    require(
        (oracle.get("provider"), oracle.get("engine"), oracle.get("model"),
         oracle.get("reasoning_mode"))
        == ("openai", "api", "gpt-5.6-sol", "pro"),
        f"{part}: wrong Oracle route/model/reasoning",
    )
    require(oracle.get("required_live_banner") == [
        "first-party OpenAI", "gpt-5.6-sol", "Responses API Pro", "xhigh reasoning"
    ], f"{part}: live banner gate is incomplete")
    require(oracle.get("state") == "NOT LAUNCHED", f"{part}: Oracle launch state changed")
    require(all(oracle.get(key) is None for key in ("session_id", "request_id", "response_id")),
            f"{part}: Oracle identifiers show a launch")
    require(oracle.get("output_paths") == [], f"{part}: Oracle outputs show a launch")
    require(oracle["estimated_total_tokens"] < TOKEN_CAP, f"{part}: token cap exceeded")

    dry_path = Path(oracle["dry_run_report"]["path"])
    require(dry_path == BASE / f"{part}-oracle-dry-run.txt", f"{part}: wrong dry-run path")
    require(digest(ROOT / dry_path) == oracle["dry_run_report"]["sha256"],
            f"{part}: dry-run report hash drift")
    dry = (ROOT / dry_path).read_text()
    required_dry = (
        "[oracle-policy] provider=openai engine=api model=gpt-5.6-sol reasoning-mode=pro",
        "[dry-run]", "would call gpt-5.6-sol", f"and {len(paths)} files.",
        f"Total: {oracle['estimated_total_tokens']:,} tokens",
        f"({oracle['estimated_total_tokens'] / TOKEN_CAP * 100:.2f}% of 272,000)",
    )
    for term in required_dry:
        require(term in dry, f"{part}: dry-run report lacks {term!r}")

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
    fable_rows = parse_fable_rows(fable_text)
    expected_rows = [(item["path"], item["sha256"], item["bytes"]) for item in attachments]
    require(fable_rows == expected_rows, f"{part}: Fable/Oracle file-list mismatch")

    require(manifest.get("launch_state") == "NOT LAUNCHED", f"{part}: launch state changed")
    bundle_review = manifest.get("independent_bundle_review", {})
    require(bundle_review.get("reviewer") == "PENDING"
            and bundle_review.get("verdict", "").startswith("PENDING; literal BUNDLE PASS"),
            f"{part}: prelaunch independent-review gate changed")
    require(manifest.get("omissions"), f"{part}: omissions must be explicit")
    return manifest


def verify_synthesis_inputs(manifests: dict[str, dict]) -> None:
    synth = manifests["synthesis"]
    inputs = synth.get("synthesis_inputs", [])
    require([item.get("part") for item in inputs]
            == ["owner-generative", "lifecycle", "containment-release"],
            "synthesis part order mismatch")
    for item in inputs:
        part = item["part"]
        path = BASE / f"{part}-manifest.json"
        require(item.get("manifest_path") == str(path),
                f"synthesis path mismatch for {part}")
        require(item.get("manifest_sha256") == digest(ROOT / path),
                f"synthesis hash drift for {part}")


def main() -> int:
    verify_identity_and_delta()
    verify_verdict_and_reports()
    manifests = {part: verify_part(part) for part in PARTS}
    verify_synthesis_inputs(manifests)
    print(
        "PASS: exact-SHA W1-11/W1-12 review bundles are intact, "
        "product delta is zero, all plans are NOT LAUNCHED, and token caps hold."
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

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
