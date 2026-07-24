#!/usr/bin/env python3
"""Deterministically generate exact-source evidence for the W1-11/W1-12 review bundle."""

from __future__ import annotations

import argparse
import hashlib
import re
import shlex
import subprocess
import sys
from pathlib import Path

LIBBUN_SHA = "6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb"
LIBBUN_TREE = "cb964de8ab8162449fbe95959bf34d231570aa5c"
SWARM_SHA = "95323ff17cb29928e31467f651ef03bae2099c14"
SWARM_TREE = "43b47bbd49a6053d270b3e15cc141cb1b1bb86da"
BASE = Path("docs/reviews/libbun-w1112-20260724")
SNAPSHOT_BASE = BASE / f"adjacent-swarm-{SWARM_SHA}"

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

LOCK_PRIVACY_PATHS = (
    "Cargo.lock",
    "native/Cargo.lock",
    "runtime/Cargo.lock",
    "tests/fixtures/public_api_boundary/Cargo.lock",
    "tests/fixtures/public_api_boundary/Cargo.toml",
    "tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs",
    "tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs",
    "tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs",
    "tests/public_api_boundary.rs",
)

COMPLIANCE_PATHS = (
    "LICENSE",
    "vendor/README.md",
    "vendor/bun.LIBBUN_VENDOR.json",
    "vendor/bun/LICENSE.md",
    "vendor/bun/Cargo.lock",
)


def run(args: list[str], cwd: Path, allow_search_miss: bool = False) -> tuple[int, str]:
    result = subprocess.run(
        args,
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode and not (allow_search_miss and result.returncode == 1):
        raise RuntimeError(
            f"command failed ({result.returncode}) in {cwd}: "
            + " ".join(shlex.quote(arg) for arg in args)
            + "\n"
            + result.stdout
        )
    return result.returncode, result.stdout


def git_blob(repo: Path, sha: str, path: str) -> bytes:
    result = subprocess.run(
        ["git", "show", f"{sha}:{path}"],
        cwd=repo,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode:
        raise RuntimeError(result.stderr.decode(errors="replace"))
    return result.stdout


def git_blob_oid(repo: Path, sha: str, path: str) -> str:
    return run(["git", "rev-parse", f"{sha}:{path}"], repo)[1].strip()


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def clean(text: str) -> str:
    return "\n".join(line.rstrip() for line in text.splitlines()) + "\n"


def command_text(repo: Path, args: list[str]) -> str:
    return "git -C " + shlex.quote(str(repo)) + " " + " ".join(shlex.quote(arg) for arg in args)


def search_section(
    title: str,
    repo: Path,
    sha: str,
    pattern: str,
    paths: tuple[str, ...],
    meaning: str,
    expected: str,
) -> str:
    args = ["grep", "-n", "-E", pattern, sha, "--", *paths]
    exit_code, output = run(["git", *args], repo, allow_search_miss=True)
    return clean(
        f"## {title}\n\n"
        f"Meaning: {meaning}\n\n"
        f"Expected result: {expected}\n\n"
        f"Command: {command_text(repo, args)}\n\n"
        f"Pattern: {pattern}\n\n"
        f"Pathspecs: {' '.join(paths)}\n\n"
        f"Exit: {exit_code}\n\n"
        "Output:\n\n"
        + (output if output else "<no matches>\n")
    )


def identity_row(repo: Path, sha: str, path: str) -> tuple[str, str, int]:
    data = git_blob(repo, sha, path)
    return sha256(data), git_blob_oid(repo, sha, path), len(data)


def exact_source_search_report() -> str:
    impl_paths = (
        "src", "native/src", "native/build.rs", "wire/src", "runtime/src",
        "runtime/build.rs", "scripts", ".github", "tests", "Cargo.toml",
        "native/Cargo.toml", "runtime/Cargo.toml", "wire/Cargo.toml",
    )
    sections = [
        search_section(
            "Required owner and lifecycle definitions (expected negative)",
            ROOT,
            LIBBUN_SHA,
            "BunProviderBackend|SelectedProviderPackage|ProviderInvocation|OfferCustody|"
            "OfferReadyProof|ReservedCustody|ReservationReleaseProof|DriveCustody|"
            "InvocationReadyProof|RetirementProof|DurableReaper|RetirementQuarantine|"
            "QuarantineObservation|QuarantineCompletionClaim|RetiredDisposal",
            impl_paths,
            "Proves the poisoned candidate has no positive retained-backend/proof implementation.",
            "Exit 1 means every named required definition is absent from the complete implementation pathset.",
        ),
        search_section(
            "Native/wire public and RAW bridge shapes",
            ROOT,
            LIBBUN_SHA,
            "pub (struct|enum|fn)|DriveRequest|drive_prepared_export|internal-adapter|"
            "install_prepared_export|from_parts|into_parts|selector|descriptor|Clone|Serialize|"
            "Deserialize|callback|receipt",
            impl_paths,
            "Finds current public protocol/native entry points and forbidden raw/parts/callback proof shapes.",
            "Exit 0 with every current public bridge available for migration/deletion review.",
        ),
        search_section(
            "Process containment, raw handle, and join topology",
            ROOT,
            LIBBUN_SHA,
            "Command::new|Child|try_wait|\\.wait\\(|\\.kill\\(|setpgid|SIGKILL|"
            "CreateJobObject|AssignProcessToJobObject|namespace|sandbox|RawFd|RawHandle|"
            "JoinHandle|Receiver|sync_channel|\\.join\\(",
            impl_paths,
            "Finds all worker process, containment, raw descriptor/handle, channel, and join custody.",
            "Exit 0; every match is current candidate topology, including rejected process-group fallback.",
        ),
        search_section(
            "Output drain, overflow, barrier, EOF, and diagnostic topology",
            ROOT,
            LIBBUN_SHA,
            "stdout|stderr|diagnostic|log|OutputCapture|drain|flush|overflow|barrier|EOF|"
            "read_single_candidate|write_request|pipe",
            impl_paths,
            "Finds every current output path and the absence/presence of persistent bounded pumps and barriers.",
            "Exit 0; matches enumerate candidate output custody and tests.",
        ),
        search_section(
            "Lifecycle, refusal, retry, cancellation, unwind, Drop, and shutdown topology",
            ROOT,
            LIBBUN_SHA,
            "release|reservation|ready|retire|reaper|quarantine|cancel|deadline|"
            "catch_unwind|panic|impl Drop|process::abort|shutdown|restart|fault",
            impl_paths,
            "Finds current lifecycle transitions, fault paths, destructors, aborts, and missing proof algebra.",
            "Exit 0; all current lifecycle and destructor sites are named.",
        ),
        search_section(
            "Package, lock, license, compliance, release, and extracted-smoke topology",
            ROOT,
            LIBBUN_SHA,
            "package|archive|release|linked|unlinked|fallback|fresh-process|Cargo.lock|"
            "license|notice|compliance|workflow|tag|symbol|extract|smoke",
            (
                "Cargo.toml", "native/Cargo.toml", "runtime/Cargo.toml", "wire/Cargo.toml",
                "scripts", ".github", "README.md", "docs", "vendor/README.md",
                "vendor/bun.LIBBUN_VENDOR.json",
            ),
            "Finds all current packaging/release modes and compliance inputs without scanning lock payload noise.",
            "Exit 0; matches expose current fresh-process/fallback and missing immutable release workflow.",
        ),
        search_section(
            "Current test and external privacy fixture definitions",
            ROOT,
            LIBBUN_SHA,
            "#\\[test\\]|#\\[cfg\\(test\\)\\]|fn [A-Za-z0-9_]+\\(|"
            "install_prepared_export|compile-fail|public.API|fixture",
            ("src", "native/src", "wire/src", "runtime/src", "tests"),
            "Enumerates every current unit/integration/privacy fixture definition to retain, delete, or migrate.",
            "Exit 0; all candidate test families and raw-installer tripwires are visible.",
        ),
        search_section(
            "Adjacent exact-call and invocation producers",
            SWARM_ROOT,
            SWARM_SHA,
            "ManifestResolvedExternalProviderCallAuthority|ManifestResolvedExternalProviderCallAdmission|"
            "DurableExternalProviderInvocationAuthority|select_exact_call|"
            "into_contract_and_module|into_call_input_and_output_settlement",
            (
                "crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs",
                "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs",
            ),
            "Binds the real exact-route producer and sealed invocation/output-settlement producer.",
            "Exit 0; constructors, raw splitters, and settlement operations are all present.",
        ),
        search_section(
            "Adjacent sole consumer, transport, process, and shutdown graph",
            SWARM_ROOT,
            SWARM_SHA,
            "SsExternalCapabilityProviderHost|invoke_manifest_resolved_call|ProviderRequest|"
            "adapter_source|begin_invocation|settle_provider|shutdown|impl Drop|Command::new|"
            "wait_with_output|libbun",
            (
                "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
                "crates/swarm-provider-host-set/src/external_transport.rs",
                "crates/swarm-provider-host-set/src/provider_host_set.rs",
                "crates/ss/src/product.rs",
                "crates/ss/tests/external_capability_provider.rs",
                "crates/ss-runtime-external-capability-provider-owner/Cargo.toml",
                "crates/ss/Cargo.toml",
                "Cargo.toml",
            ),
            "Binds the sole libbun consumer, callback trait boundary, raw reconstruction, process callers, shutdown, and Cargo direction.",
            "Exit 0; all current cross-repository ownership and compatibility shapes are visible.",
        ),
    ]
    return clean(
        "# libbun W1-11/W1-12 exact-source search report (correction 2)\n\n"
        f"Libbun product SHA: {LIBBUN_SHA}\n\n"
        f"Libbun product tree: {LIBBUN_TREE}\n\n"
        f"Adjacent swarm SHA: {SWARM_SHA}\n\n"
        f"Adjacent swarm tree: {SWARM_TREE}\n\n"
        "Every section records its literal Git command, pattern, pathspecs, semantic meaning, expected exit, observed exit, and output. "
        "Exit 1 is accepted only for the explicitly labeled required-definition absence search.\n\n"
        + "\n".join(sections)
    )


def excerpt(repo: Path, sha: str, path: str, start: int, end: int) -> str:
    data = git_blob(repo, sha, path)
    lines = data.decode(errors="replace").splitlines()
    selected = lines[start - 1:end]
    excerpt_bytes = ("\n".join(selected) + "\n").encode()
    full_sha, blob, size = identity_row(repo, sha, path)
    numbered = "\n".join(f"{number:6d}  {line}" for number, line in enumerate(selected, start))
    return clean(
        f"## {path}:{start}-{end}\n\n"
        f"- Full-file Git blob: {blob}\n"
        f"- Full-file SHA-256: {full_sha}\n"
        f"- Full-file bytes: {size}\n"
        f"- Excerpt line span: {start}-{end}\n"
        f"- Excerpt SHA-256: {sha256(excerpt_bytes)}\n\n"
        f"{numbered}\n"
    )


def vendored_boundary_report() -> str:
    paths = (
        "patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch",
        "patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch",
        "native/build.rs",
        "runtime/build.rs",
        "scripts/prepare-native-bun-link.sh",
        "BUN_SOURCE_COMMIT",
        "vendor/bun/src/jsc/bindings/bindings.cpp",
    )
    rows = []
    for path in paths:
        file_sha, blob, size = identity_row(ROOT, LIBBUN_SHA, path)
        rows.append(f"| {path} | {blob} | {file_sha} | {size} |")
    patches = []
    for path in paths[:2]:
        patches.append(
            f"## Full patch: {path}\n\n"
            + git_blob(ROOT, LIBBUN_SHA, path).decode(errors="replace")
        )
    return clean(
        "# libbun W1-11/W1-12 vendored Bun boundary report (correction 2)\n\n"
        f"Generated from exact product SHA {LIBBUN_SHA}, tree {LIBBUN_TREE}.\n\n"
        "Generator command: python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --emit vendored-bun-boundary-report.md\n\n"
        "No shell pipeline or awk transformation participates in identity generation.\n\n"
        "## Full-file identities\n\n"
        "| Path | Git blob | SHA-256 | Bytes |\n"
        "| --- | --- | --- | ---: |\n"
        + "\n".join(rows)
        + "\n\n"
        + "\n".join(patches)
        + "\n"
        + excerpt(ROOT, LIBBUN_SHA, "vendor/bun/src/jsc/bindings/bindings.cpp", 4340, 4395)
        + "\n"
        + excerpt(ROOT, LIBBUN_SHA, "native/src/lib.rs", 235, 315)
        + "\n"
        + excerpt(ROOT, LIBBUN_SHA, "native/build.rs", 1, 145)
        + "\n"
        + excerpt(ROOT, LIBBUN_SHA, "runtime/build.rs", 1, 130)
        + "\n"
        + excerpt(ROOT, LIBBUN_SHA, "runtime/src/main.rs", 1, 38)
    )


def adjacent_source_index() -> str:
    rows = []
    for path in ADJACENT_PATHS:
        source = git_blob(SWARM_ROOT, SWARM_SHA, path)
        blob = git_blob_oid(SWARM_ROOT, SWARM_SHA, path)
        snapshot_path = SNAPSHOT_BASE / path
        snapshot = (ROOT / snapshot_path).read_bytes()
        rows.append(
            f"| {path} | {blob} | {sha256(source)} | {len(source)} | "
            f"{snapshot_path} | {sha256(snapshot)} | {'yes' if source == snapshot else 'NO'} |"
        )
    return clean(
        "# Adjacent swarm owner/producer/consumer source index\n\n"
        f"Adjacent source SHA: {SWARM_SHA}\n\n"
        f"Adjacent source tree: {SWARM_TREE}\n\n"
        "These snapshots bind the real producer -> invocation/output settlement -> sole libbun consumer -> transport/process/shutdown/test SCC. "
        "They are review evidence only and do not move product ownership.\n\n"
        "| Original path | Git blob | Source SHA-256 | Bytes | Snapshot path | Snapshot SHA-256 | Exact |\n"
        "| --- | --- | --- | ---: | --- | --- | --- |\n"
        + "\n".join(rows)
        + "\n"
    )


def lifecycle_jsc_bundle() -> str:
    full_paths = (
        "vendor/bun/src/jsc/VirtualMachine.rs",
        "vendor/bun/src/jsc/VM.rs",
        "vendor/bun/src/jsc/JSGlobalObject.rs",
        "vendor/bun/src/jsc/virtual_machine_exports.rs",
        "vendor/bun/src/jsc/bindings/bindings.cpp",
        "vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp",
        "vendor/bun/src/jsc/bindings/ZigGlobalObject.h",
        "vendor/bun/src/jsc/VirtualMachine.zig",
    )
    rows = []
    for path in full_paths:
        file_sha, blob, size = identity_row(ROOT, LIBBUN_SHA, path)
        rows.append(f"| {path} | {blob} | {file_sha} | {size} |")
    spans = (
        ("vendor/bun/src/jsc/VirtualMachine.rs", 605, 810),
        ("vendor/bun/src/jsc/VirtualMachine.rs", 1170, 1230),
        ("vendor/bun/src/jsc/VirtualMachine.rs", 1360, 1570),
        ("vendor/bun/src/jsc/VirtualMachine.rs", 1941, 2180),
        ("vendor/bun/src/jsc/VirtualMachine.rs", 2208, 2410),
        ("vendor/bun/src/jsc/VirtualMachine.rs", 4302, 4360),
        ("vendor/bun/src/jsc/VM.rs", 1, 220),
        ("vendor/bun/src/jsc/JSGlobalObject.rs", 190, 235),
        ("vendor/bun/src/jsc/JSGlobalObject.rs", 960, 1005),
        ("vendor/bun/src/jsc/bindings/bindings.cpp", 4880, 4995),
        ("vendor/bun/src/jsc/bindings/bindings.cpp", 6124, 6145),
        ("vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp", 2988, 3055),
        ("vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp", 3120, 3155),
        ("vendor/bun/src/jsc/VirtualMachine.zig", 2095, 2135),
    )
    return clean(
        "# Vendored JSC lifecycle source bundle\n\n"
        f"Exact product SHA: {LIBBUN_SHA}\n\n"
        f"Exact product tree: {LIBBUN_TREE}\n\n"
        "This bundle binds VM construction, global-object access, event-loop drain, termination request/reset, worker teardown, and C++ VM calls. "
        "The C++ excerpt proves JSC__VM__deinit has an empty body; it cannot prove process death, containment drain, output joins, or retirement. "
        "Cooperative termination reset is therefore reusable only after the owner proves complete invocation and output drain independently.\n\n"
        "## Full-file identities\n\n"
        "| Path | Git blob | SHA-256 | Bytes |\n"
        "| --- | --- | --- | ---: |\n"
        + "\n".join(rows)
        + "\n\n"
        + "\n".join(excerpt(ROOT, LIBBUN_SHA, path, start, end) for path, start, end in spans)
    )


def process_drop_report() -> str:
    sections = [
        search_section(
            "Libbun process, thread, Drop, cancellation, and shutdown callers",
            ROOT,
            LIBBUN_SHA,
            "Command::new|\\.spawn\\(|Child|try_wait|wait_with_output|\\.wait\\(|"
            "\\.kill\\(|JoinHandle|\\.join\\(|impl Drop|process::abort|catch_unwind|"
            "cancel|deadline|retire|shutdown",
            (
                "src/prepared_export.rs", "native/src/lib.rs", "runtime/src/main.rs",
                "wire/src/lib.rs", "tests", "scripts",
            ),
            "Enumerates every candidate process/thread/destructor/cancellation/shutdown caller.",
            "Exit 0; no omitted libbun process or Drop family.",
        ),
        search_section(
            "Vendored VM termination, reset, drain, and deinit callers",
            ROOT,
            LIBBUN_SHA,
            "request_termination|clear_termination|notify_need_termination|"
            "has_termination_request|drain_microtasks|JSC__VM__deinit|pub fn destroy|"
            "pub fn deinit|terminate_all_workers|shutdown",
            (
                "vendor/bun/src/jsc/VirtualMachine.rs",
                "vendor/bun/src/jsc/VM.rs",
                "vendor/bun/src/jsc/JSGlobalObject.rs",
                "vendor/bun/src/jsc/virtual_machine_exports.rs",
                "vendor/bun/src/jsc/bindings/bindings.cpp",
                "vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp",
                "vendor/bun/src/jsc/VirtualMachine.zig",
            ),
            "Binds the JSC cooperative interrupt/reset, drain, teardown, and empty C++ deinit facts.",
            "Exit 0; all named VM/global/C++ lifecycle operations are visible.",
        ),
        search_section(
            "Adjacent consumer, transport, process, Drop, and shutdown callers",
            SWARM_ROOT,
            SWARM_SHA,
            "invoke_manifest_resolved_call|begin_execution_session|shutdown|impl Drop|"
            "Command::new|\\.spawn\\(|wait_with_output|libbun|ProviderRequest|"
            "into_call_input_and_output_settlement|into_contract_and_module",
            (
                "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
                "crates/swarm-provider-host-set/src/external_transport.rs",
                "crates/swarm-provider-host-set/src/provider_host_set.rs",
                "crates/ss/src/product.rs",
                "crates/ss/tests/external_capability_provider.rs",
            ),
            "Traces every attached external consumer/process/shutdown edge and current raw reconstruction.",
            "Exit 0; all attached direct callers and compatibility edges are visible.",
        ),
        search_section(
            "Adjacent external fixture graph",
            SWARM_ROOT,
            SWARM_SHA,
            "#\\[test\\]|libbun|external.provider|counter|multiple.capability|"
            "pool.child|cancellation|deadline|shutdown|provider result",
            (
                "crates/ss/tests/external_capability_provider.rs",
                "tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss",
                "tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss",
            ),
            "Binds the current real-binary retained-runtime and external-result fixture graph.",
            "Exit 0; every attached external fixture/test definition is visible.",
        ),
    ]
    return clean(
        "# Process, Drop, shutdown caller, and external fixture report\n\n"
        f"Libbun source: {LIBBUN_SHA} ({LIBBUN_TREE})\n\n"
        f"Adjacent swarm source: {SWARM_SHA} ({SWARM_TREE})\n\n"
        + "\n".join(sections)
    )


def lock_privacy_compliance_index() -> str:
    paths = LOCK_PRIVACY_PATHS + COMPLIANCE_PATHS
    rows = []
    for path in paths:
        data = git_blob(ROOT, LIBBUN_SHA, path)
        blob = git_blob_oid(ROOT, LIBBUN_SHA, path)
        package_names = sorted(set(re.findall(rb'^name = "([^"]+)"$', data, re.MULTILINE)))
        package_summary = str(len(package_names)) if path.endswith("Cargo.lock") else "-"
        rows.append(
            f"| {path} | {blob} | {sha256(data)} | {len(data)} | {package_summary} |"
        )
    return clean(
        "# Lock, privacy fixture, license, provenance, and compliance index\n\n"
        f"Exact product SHA: {LIBBUN_SHA}\n\n"
        f"Exact product tree: {LIBBUN_TREE}\n\n"
        "All four nonvendored locks, the complete six-file external privacy harness, and all current license/vendor provenance inputs are direct attachments. "
        "The vendored lock is also direct, so a concrete implementation can derive repeat-lock and compliance inventory without reconstructing omitted bytes.\n\n"
        "| Path | Git blob | SHA-256 | Bytes | Unique lock packages |\n"
        "| --- | --- | --- | ---: | ---: |\n"
        + "\n".join(rows)
        + "\n"
    )


REPORTS = {
    "exact-source-search-report.md": exact_source_search_report,
    "vendored-bun-boundary-report.md": vendored_boundary_report,
    "adjacent-swarm-source-index.md": adjacent_source_index,
    "lifecycle-vendored-jsc-source-bundle.md": lifecycle_jsc_bundle,
    "process-drop-caller-and-fixture-report.md": process_drop_report,
    "lock-privacy-compliance-index.md": lock_privacy_compliance_index,
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit", choices=REPORTS)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if bool(args.emit) == bool(args.check):
        parser.error("choose exactly one of --emit REPORT or --check")
    if args.emit:
        sys.stdout.write(REPORTS[args.emit]())
        return 0

    failed = False
    for name, generate in REPORTS.items():
        path = ROOT / BASE / name
        expected = generate().encode()
        if not path.is_file():
            print(f"MISSING {path.relative_to(ROOT)}", file=sys.stderr)
            failed = True
        elif path.read_bytes() != expected:
            print(f"DRIFT {path.relative_to(ROOT)}", file=sys.stderr)
            failed = True
        else:
            print(f"OK {path.relative_to(ROOT)}")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
