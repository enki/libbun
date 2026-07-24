#!/usr/bin/env python3
"""Deterministically generate correction-4 exact-source evidence for W1-11/W1-12."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
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

DISCOVERED_ROOT = Path(
    subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    ).stdout.strip()
)
ROOT = Path(os.environ.get("LIBBUN_REPO", DISCOVERED_ROOT)).resolve()
SWARM_ROOT = Path(os.environ.get("SWARM_REPO", "/home/ubuntu/swarm")).resolve()

REPO_LABELS = {
    ROOT: '"$LIBBUN_REPO"',
    SWARM_ROOT: '"$SWARM_REPO"',
}

GENERATIVE_SEARCH_PATTERN = (
    "DurableExternalProviderInvocationAuthority|"
    "SelectedProviderResumeHostInputForDirectRunOwnerV1|"
    "SelectedProviderBoundaryHostRequest|"
    "SelectedProviderBoundaryExecutionResultForProviderHostOwner|"
    "mint_provider_boundary_output_correspondence_v1|"
    "PendingProviderBoundaryOutputCommitAuthority|"
    "invoke_selected_provider_boundary_request_for_direct_run_owner_v1|"
    "admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1"
)

# Exact repository-wide git-grep result at SWARM_SHA.  The generator re-runs
# the search before rendering the compact complete-source bundle, so a new
# definition/module/caller/test cannot silently fall outside the evidence set.
GENERATIVE_PATHS = (
    "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/event/mod.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/provider_resume_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/public_aperture_drive.rs",
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/process_carriers.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/root.inc.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/mod.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_runtime_stores_impl.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store/types.rs",
    "crates/swarm-capability-model/src/lib.rs",
    "crates/swarm-capability-model/src/provider_boundary_correspondence.rs",
    "crates/swarm-provider-host-set/src/external_transport.rs",
    "crates/swarm-provider-host-set/src/provider_host_set.rs",
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs",
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs",
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs",
)

BASE_ADJACENT_PATHS = (
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
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
    "crates/ss-runtime-provider-host-set-owner/Cargo.toml",
    "crates/ss-runtime-provider-host-set-owner/src/lib.rs",
)

ADJACENT_PATHS = tuple(dict.fromkeys(BASE_ADJACENT_PATHS + GENERATIVE_PATHS + (
    "crates/swarm-capability-model/Cargo.toml",
    "crates/ss-runtime-source-compiler-owner/Cargo.toml",
    "crates/swarm-rust-sdk-static-provider-host/src/lib.rs",
)))

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
    "vendor/bun/Cargo.toml",
    "vendor/bun/src/clap/LICENSE",
    "vendor/bun/src/unicode/uucode_lib/LICENSE.md",
    "vendor/bun/vendor/lolhtml/LICENSE",
)

CORRECTION2_VERDICTS = {
    "owner-generative-correction2-independent-verdict.md": (
        "47acbee0fe5a67231969efd8141ba8195bcecc8a",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction2-independent-verdict.md",
    ),
    "lifecycle-correction2-independent-verdict.md": (
        "0a3844d11bb42d67550da0bb1e069ecf17fbe69d",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction2-independent-verdict.md",
    ),
    "containment-release-correction2-independent-verdict.md": (
        "84feaf68aa99c5bc0e393cbfc1b6a92716cefdf1",
        "docs/reviews/libbun-w1112-containment-release-correction2-independent-verdict-20260724.md",
    ),
}

CORRECTION3_VERDICTS = {
    "owner-generative-correction3-independent-verdict.md": (
        "29136ad08f0103cd4338db51552a2a566625d81d",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction3-independent-verdict.md",
    ),
    "lifecycle-correction3-independent-verdict.md": (
        "a5ab10f422fb955b899e6ce1089b8c74a4600860",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction3-independent-verdict.md",
    ),
    "containment-release-correction3-independent-verdict.md": (
        "16ae0060d9c8648048b89c8451cc51cfe1ec72db",
        "docs/reviews/libbun-w1112-containment-release-correction3-independent-verdict-20260724.md",
    ),
}

PARTS = ("owner-generative", "lifecycle", "containment-release", "synthesis")
PART_TITLES = {
    "owner-generative": "Owner/W1-10/generative correspondence/admission/reservation/release",
    "lifecycle": "Lifecycle/JSC interruption/retained-host/quarantine/reaper/shutdown",
    "containment-release": "Containment/persistent output/locks/packaging/release",
    "synthesis": "W1-11/W1-12 full-SCC synthesis",
}
PRIOR_VERDICTS = {
    "owner-generative": (
        "29136ad08f0103cd4338db51552a2a566625d81d",
        "owner-generative-correction3-independent-verdict.md",
    ),
    "lifecycle": (
        "a5ab10f422fb955b899e6ce1089b8c74a4600860",
        "lifecycle-correction3-independent-verdict.md",
    ),
    "containment-release": (
        "16ae0060d9c8648048b89c8451cc51cfe1ec72db",
        "containment-release-correction3-independent-verdict.md",
    ),
}
REVIEW_BASE = "c2ea016e4c9810fa86ddfd21bd4b30823746a9b9"
VERDICT_CONTRACT = "b046f85a3dd41ac86cabed2de6391876ea77c0f4"
VERIFIER = Path("scripts/verify-libbun-w1112-review-bundle-20260724.py")

GENERATIVE_EXCERPT_SPANS = {
    "crates/ss-runtime-source-compiler-owner/src/direct_run/event/mod.rs": ((1, 30), (225, 679)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs": ((1, 30), (822, 831)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs": ((1, 35), (1586, 1940)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/provider_resume_lifecycle.rs": ((1, 35), (140, 336)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/public_aperture_drive.rs": ((1, 20), (172, 466)),
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs": ((1, 30), (130, 180), (400, 450)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime.rs": ((190, 220),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs": ((1, 40),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/process_carriers.rs": ((1, 20), (130, 174), (582, 678)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/root.inc.rs": ((260, 305),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs": ((710, 760),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_runtime_stores_impl.rs": ((1, 30), (560, 618), (1332, 1437)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store.rs": ((1, 75),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store/types.rs": ((1, 30), (250, 423), (860, 904), (1674, 1695)),
    "crates/swarm-capability-model/src/lib.rs": ((1, 30),),
    "crates/swarm-provider-host-set/src/provider_host_set.rs": ((1, 30), (650, 730), (870, 940), (980, 1030)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs": ((1, 30), (500, 565)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs": ((1, 35), (110, 190), (680, 730), (904, 975), (1177, 1374), (2550, 2611)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs": ((400, 530),),
}

LIFECYCLE_SOURCE_PATHS = (
    "vendor/bun/src/runtime/node/node_process.rs",
    "vendor/bun/src/runtime/jsc_hooks.rs",
    "vendor/bun/src/jsc/web_worker.rs",
    "vendor/bun/src/jsc/bindings/BunProcess.cpp",
    "vendor/bun/src/jsc/bindings/webcore/Worker.cpp",
    "vendor/bun/src/jsc/bindings/vm/SigintWatcher.cpp",
    "vendor/bun/src/jsc/bindings/NodeVM.cpp",
    "vendor/bun/src/jsc/bindings/NodeVMScript.cpp",
)

LIFECYCLE_EXCERPT_SPANS = {
    "vendor/bun/src/runtime/node/node_process.rs": ((1, 64),),
    "vendor/bun/src/runtime/jsc_hooks.rs": ((1188, 1199), (1397, 1428), (1518, 1527)),
    "vendor/bun/src/jsc/web_worker.rs": ((90, 203), (233, 396), (540, 710), (990, 1346)),
    "vendor/bun/src/jsc/bindings/BunProcess.cpp": ((280, 304), (3245, 3263)),
    "vendor/bun/src/jsc/bindings/webcore/Worker.cpp": ((350, 430),),
    "vendor/bun/src/jsc/bindings/vm/SigintWatcher.cpp": ((105, 208),),
    "vendor/bun/src/jsc/bindings/NodeVM.cpp": ((847, 870),),
    "vendor/bun/src/jsc/bindings/NodeVMScript.cpp": ((278, 300),),
}

RETAINED_HOST_SHUTDOWN_PATHS = (
    "crates/ss-runtime-test-execution-owner/src/lib.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
    "crates/ss/tests/external_capability_provider.rs",
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
    try:
        repo_label = REPO_LABELS[repo.resolve()]
    except KeyError as error:
        raise RuntimeError(f"unbound report repository: {repo}") from error
    return "git -C " + repo_label + " " + " ".join(shlex.quote(arg) for arg in args)


def repository_prologue() -> str:
    return clean(
        "Repository labels used by every durable command record:\n\n"
        f"- `LIBBUN_REPO`: any checkout containing exact libbun SHA `{LIBBUN_SHA}`.\n"
        f"- `SWARM_REPO`: any checkout containing exact Swarm SHA `{SWARM_SHA}`.\n\n"
        "The resolved filesystem values are execution inputs only and never enter generated bytes.\n"
    )


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
            "Selected package and lock topology",
            ROOT,
            LIBBUN_SHA,
            "^name = |^version = |^members = |^resolver = |^\\[package\\]|"
            "^\\[\\[package\\]\\]|libbun|lolhtml|JavaScriptCore|WebKit|bun",
            (
                "Cargo.toml", "Cargo.lock", "native/Cargo.toml", "native/Cargo.lock",
                "runtime/Cargo.toml", "runtime/Cargo.lock", "wire/Cargo.toml",
                "tests/fixtures/public_api_boundary/Cargo.toml",
                "tests/fixtures/public_api_boundary/Cargo.lock",
                "vendor/bun/Cargo.toml", "vendor/bun/Cargo.lock",
            ),
            "Searches every directly attached package manifest and lock used by the bounded linked native/package closure.",
            "Exit 0; selected workspace, package, and locked dependency records are visible at the exact product SHA.",
        ),
        search_section(
            "Privacy harness topology",
            ROOT,
            LIBBUN_SHA,
            "install_prepared_export|compile-fail|public.API|adjacent|raw.installer|libbun",
            LOCK_PRIVACY_PATHS[4:],
            "Searches every directly attached privacy manifest, fixture, and harness source.",
            "Exit 0; all six external privacy inputs and their obsolete-aperture tripwires are visible.",
        ),
        search_section(
            "License, provenance, and compliance topology",
            ROOT,
            LIBBUN_SHA,
            "copyright|Copyright|license|License|LICENSE|permission|Permission|"
            "provenance|source|commit|lolhtml|WebKit|Bun|libbun",
            COMPLIANCE_PATHS,
            "Searches every directly attached license, provenance, vendoring, and compliance input.",
            "Exit 0; every selected compliance input participates in the exact-source search.",
        ),
        search_section(
            "Release and extracted-smoke topology",
            ROOT,
            LIBBUN_SHA,
            "package|archive|release|linked|unlinked|fallback|fresh-process|workflow|"
            "tag|symbol|extract|smoke",
            (
                "scripts", ".github", "README.md", "docs", "vendor/README.md",
                "vendor/bun.LIBBUN_VENDOR.json",
            ),
            "Finds current packaging/release modes and the immutable archive/extracted-smoke boundary.",
            "Exit 0; current fresh-process/fallback and release workflow facts are visible.",
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
            "Adjacent repository-wide generative mint/carrier/consumer closure",
            SWARM_ROOT,
            SWARM_SHA,
            GENERATIVE_SEARCH_PATTERN,
            ("crates",),
            "Discovers the complete 24-file selected invocation, output-correspondence mint/join, carrier, drive-consumer, final-output, fault, and static-host source closure before any fixed attachment list is trusted.",
            "Exit 0 with exactly the 24 paths bound by the compact complete-source bundle.",
        ),
        search_section(
            "Adjacent W1-10 ProviderValue input and governing law",
            SWARM_ROOT,
            SWARM_SHA,
            "ProviderValue|W1-10|canonical|duplicate|NFC|JSON|StructuralValue|"
            "DurableExternalProviderInvocationAuthority",
            (
                "docs/PROVIDER_EXECUTION_AND_SDK_LAW.md",
                "docs/PROVIDER_VALUE_JSON_WIRE_V1.md",
                "docs/SWARMSCRIPT_ROADMAP.md",
                "docs/WAVE0_WAVE1_SEMANTIC_CLOSURE_INDEX.md",
                "crates/swarm-provider-value-model/src/lib.rs",
                "crates/swarm-capability-linker-core/src/lib.rs",
                "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs",
                "tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss",
            ),
            "Binds W1-10 ProviderValue as the exact by-value W1-11 invocation cargo and its hostile canonical-wire refusal law.",
            "Exit 0; the defining type, reexport/conversion boundary, laws, closure index, invocation field, and negative fixture are visible.",
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
            "Adjacent sole consumer, transport, retained-host pool, and shutdown graph",
            SWARM_ROOT,
            SWARM_SHA,
            "SsExternalCapabilityProviderHost|invoke_manifest_resolved_call|ProviderRequest|"
            "adapter_source|begin_invocation|settle_provider|shutdown|impl Drop|Command::new|"
            "wait_with_output|libbun|ExternalCapabilityProviderPool|checkout|replace|"
            "provider_settlement_lane|runtime_execution_domain|body_authority_registry",
            (
                "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
                "crates/swarm-provider-host-set/src/external_transport.rs",
                "crates/swarm-provider-host-set/src/provider_host_set.rs",
                "crates/ss/src/product.rs",
                "crates/ss/tests/external_capability_provider.rs",
                "crates/ss-runtime-external-capability-provider-owner/Cargo.toml",
                "crates/ss-runtime-test-execution-owner/src/lib.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
                "crates/ss-runtime-provider-host-set-owner/src/lib.rs",
                "crates/ss/Cargo.toml",
                "Cargo.toml",
            ),
            "Binds the sole libbun consumer, callback trait boundary, raw reconstruction, retained-host checkout/replacement chain, final shutdown, and Cargo direction.",
            "Exit 0; all current cross-repository ownership, pool custody, and compatibility shapes are visible.",
        ),
        search_section(
            "Adjacent retained-runtime final shutdown edge",
            SWARM_ROOT,
            SWARM_SHA,
            "close_for_execution_graph_owner|shutdown_runtime_execution_domain_owner|provider_settlement_pool\\.shutdown|provider_pool\\.shutdown|current_native_plugin_asset|plugin/target/release",
            RETAINED_HOST_SHUTDOWN_PATHS,
            "Binds the retained-host checkout/replacement pools, the live-feed close carrier, the separately defined final shutdown helper, and both stale real-worker plugin migration sites.",
            "Exit 0; definitions and callers expose whether final close actually consumes the retained runtime and where stale plugin packaging must be removed.",
        ),
        search_section(
            "Adjacent package and dependency direction",
            SWARM_ROOT,
            SWARM_SHA,
            "^name = |^version = |^\\[dependencies|libbun|swarm.provider.value|"
            "capability.linker|provider.host|test.execution|swarmvm.image",
            tuple(path for path in ADJACENT_PATHS if path.endswith("Cargo.toml")),
            "Searches every attached adjacent package manifest needed to choose the acyclic producer/consumer owner move.",
            "Exit 0; all attached adjacent package identities and dependency edges are visible.",
        ),
    ]
    return clean(
        "# libbun W1-11/W1-12 exact-source search report (correction 4)\n\n"
        f"Libbun product SHA: {LIBBUN_SHA}\n\n"
        f"Libbun product tree: {LIBBUN_TREE}\n\n"
        f"Adjacent swarm SHA: {SWARM_SHA}\n\n"
        f"Adjacent swarm tree: {SWARM_TREE}\n\n"
        + repository_prologue()
        + "\nEvery section records its literal Git command, pattern, pathspecs, semantic meaning, expected exit, observed exit, and output. "
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


def full_source(repo: Path, sha: str, path: str) -> str:
    data = git_blob(repo, sha, path)
    lines = data.decode(errors="replace").splitlines()
    full_sha, blob, size = identity_row(repo, sha, path)
    numbered = "\n".join(f"{number:6d}  {line}" for number, line in enumerate(lines, 1))
    return clean(
        f"## {path}: complete file\n\n"
        f"- Full-file Git blob: {blob}\n"
        f"- Full-file SHA-256: {full_sha}\n"
        f"- Full-file bytes: {size}\n"
        f"- Complete file line span: 1-{len(lines)}\n\n"
        f"{numbered}\n"
    )


def discovered_generative_paths() -> tuple[str, ...]:
    args = ["grep", "-l", "-E", GENERATIVE_SEARCH_PATTERN, SWARM_SHA, "--", "crates"]
    exit_code, output = run(["git", *args], SWARM_ROOT)
    if exit_code != 0:
        raise RuntimeError("repository-wide adjacent generative discovery failed")
    prefix = f"{SWARM_SHA}:"
    paths = tuple(
        line[len(prefix):] if line.startswith(prefix) else line
        for line in output.splitlines()
        if line
    )
    if paths != GENERATIVE_PATHS:
        raise RuntimeError(
            "adjacent generative closure drift; expected="
            + repr(GENERATIVE_PATHS)
            + " observed="
            + repr(paths)
        )
    return paths


def source_bundle(
    title: str,
    repo: Path,
    sha: str,
    tree: str,
    paths: tuple[str, ...],
    spans: dict[str, tuple[tuple[int, int], ...]],
    required_terms: tuple[str, ...],
    introduction: str,
) -> str:
    rows = []
    sections = []
    emitted = bytearray()
    for path in paths:
        data = git_blob(repo, sha, path)
        file_sha, blob, size = identity_row(repo, sha, path)
        mode = "complete owning items" if path in spans else "complete file"
        rows.append(f"| {path} | {blob} | {file_sha} | {size} | {mode} |")
        if path in spans:
            for start, end in spans[path]:
                sections.append(excerpt(repo, sha, path, start, end))
                emitted.extend(("\n".join(data.decode(errors="replace").splitlines()[start - 1:end]) + "\n").encode())
        else:
            sections.append(full_source(repo, sha, path))
            emitted.extend(data)
    emitted_text = emitted.decode(errors="replace")
    for term in required_terms:
        if term not in emitted_text:
            raise RuntimeError(f"{title}: selected complete source lacks required term {term}")
    return clean(
        f"# {title}\n\n"
        f"Exact source SHA: {sha}\n\n"
        f"Exact source tree: {tree}\n\n"
        f"{introduction}\n\n"
        "Every compact excerpt names the complete owning item span selected from the exact file, "
        "plus the full-file blob/SHA-256/byte identity and an excerpt SHA-256. Small bounded files "
        "are included completely. The repository-wide discovery gate runs before this fixed closure is rendered.\n\n"
        "## Bound source inventory\n\n"
        "| Path | Git blob | Full-file SHA-256 | Bytes | Included source |\n"
        "| --- | --- | --- | ---: | --- |\n"
        + "\n".join(rows)
        + "\n\n"
        + "\n".join(sections)
    )


def adjacent_generative_source_bundle() -> str:
    paths = discovered_generative_paths()
    return source_bundle(
        "Adjacent Swarm generative mint, carrier, consumer, fault, and test source bundle (correction 4)",
        SWARM_ROOT,
        SWARM_SHA,
        SWARM_TREE,
        paths,
        GENERATIVE_EXCERPT_SPANS,
        (
            "mint_provider_boundary_output_correspondence_v1",
            "ReadyOutputDoesNotMatchPendingBoundary",
            "PendingProviderBoundaryOutputCommitAuthority",
            "SelectedProviderResumeHostInputForDirectRunOwnerV1",
            "admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1",
            "invoke_selected_provider_boundary_request_for_direct_run_owner_v1",
            "SelectedProviderBoundaryExecutionResultForProviderHostOwner",
            "into_provider_ready_boundary_output_and_effect_drain_receipts_for_provider_drive_result_owner_v1",
            "mint_provider_boundary_output_correspondence_v1();",
        ),
        "This bundle is driven by the exact repository-wide 24-file search. It exposes the existing one-occurrence output seal, its mismatch fault, every selected request carrier, both direct-run drive consumers, the sibling event consumer, the final ready-output commit, and static-host correspondence tests. A new package/invocation brand must extend or atomically replace this seal; a parallel seal is forbidden.",
    )


def lifecycle_process_worker_source_bundle() -> str:
    return source_bundle(
        "Vendored Bun process-exit, WebWorker termination/wait, and ordered shutdown source bundle (correction 4)",
        ROOT,
        LIBBUN_SHA,
        LIBBUN_TREE,
        LIFECYCLE_SOURCE_PATHS,
        LIFECYCLE_EXCERPT_SPANS,
        (
            "Bun__Process__exit",
            "global_exit",
            "terminate_all_workers_and_wait",
            "terminate_all_and_wait",
            "live_workers::register",
            "live_workers::unregister",
            "WebWorker__notifyNeedTermination",
            "fn spin(&self)",
            "fn shutdown(&self)",
            "pub fn exit(&self)",
            "clearHasTerminationRequest",
        ),
        "The ordered source path is public process exit -> Rust process owner -> main-VM global exit or worker exit -> concrete RuntimeHooks binding -> process-global worker registry termination sweep/wait -> per-worker termination checkpoint -> ordered VM unpublish, exit handlers, JSC teardown, unregister, exit dispatch, and worker-resource destruction. Timeout or surviving nested-worker state is ambiguous custody and cannot authorize reuse.",
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
        "# libbun W1-11/W1-12 vendored Bun boundary report (correction 4)\n\n"
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
        "These snapshots bind W1-10 ProviderValue and law -> exact producer -> invocation/output settlement -> sole libbun consumer -> retained-host pool -> transport/process/shutdown/test SCC. "
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
        ("vendor/bun/src/jsc/bindings/bindings.cpp", 4880, 4995),
        ("vendor/bun/src/jsc/bindings/bindings.cpp", 6124, 6145),
        ("vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp", 2988, 3055),
        ("vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp", 3120, 3155),
        ("vendor/bun/src/jsc/VirtualMachine.zig", 2095, 2135),
    )
    return clean(
        "# Vendored JSC lifecycle supplemental source bundle (correction 4)\n\n"
        f"Exact product SHA: {LIBBUN_SHA}\n\n"
        f"Exact product tree: {LIBBUN_TREE}\n\n"
        "The ordered lifecycle plan directly attaches complete `VirtualMachine.rs`, `JSGlobalObject.rs`, `VM.rs`, and `virtual_machine_exports.rs` bytes. "
        "This supplemental bundle binds their exact full-file identities plus the complete relevant C++/Zig termination, reset, teardown, and VM-call items. "
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
            "Repository-wide vendored process-exit, termination, worker-wait, and shutdown inventory",
            ROOT,
            LIBBUN_SHA,
            "Bun__Process__exit|process_exit|global_exit|terminate_all_workers_and_wait|"
            "terminate_all_and_wait|notifyNeedTermination|request[Tt]ermination|"
            "clear[Tt]ermination|has[Tt]ermination[Rr]equest|WebWorker__notifyNeedTermination|"
            "live_workers::(register|unregister)|fn (spin|shutdown|destroy|exit)",
            ("vendor/bun/src",),
            "Discovers the concrete process-exit to WebWorker termination/wait and ordered shutdown SCC plus every adjacent termination reset/caller before the compact source bundle is selected.",
            "Exit 0; the compact lifecycle source bundle must bind every source that changes process-exit selection, worker termination clearing, live-worker wait, or ordered shutdown semantics.",
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
            "Adjacent consumer, transport, retained-host pool, process, Drop, and shutdown callers",
            SWARM_ROOT,
            SWARM_SHA,
            "invoke_manifest_resolved_call|begin_execution_session|shutdown|impl Drop|"
            "Command::new|\\.spawn\\(|wait_with_output|libbun|ProviderRequest|"
            "into_call_input_and_output_settlement|into_contract_and_module|"
            "ExternalCapabilityProviderPool|checkout|replace|working_directory|"
            "provider_settlement_lane|runtime_execution_domain",
            (
                "crates/ss-runtime-external-capability-provider-owner/src/lib.rs",
                "crates/swarm-provider-host-set/src/external_transport.rs",
                "crates/swarm-provider-host-set/src/provider_host_set.rs",
                "crates/ss/src/product.rs",
                "crates/ss/tests/external_capability_provider.rs",
                "crates/ss-runtime-test-execution-owner/src/lib.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
                "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
                "crates/ss-runtime-provider-host-set-owner/src/lib.rs",
            ),
            "Traces every attached external consumer, retained-host checkout/replacement, process, Drop, and final shutdown edge.",
            "Exit 0; all attached direct callers, pool custody, and compatibility edges are visible.",
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
                "tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss",
            ),
            "Binds the current real-binary retained-runtime and external-result fixture graph.",
            "Exit 0; every attached external fixture/test definition is visible.",
        ),
    ]
    return clean(
        "# Process, Drop, shutdown caller, and external fixture report\n\n"
        f"Libbun source: {LIBBUN_SHA} ({LIBBUN_TREE})\n\n"
        f"Adjacent swarm source: {SWARM_SHA} ({SWARM_TREE})\n\n"
        + repository_prologue()
        + "\n"
        + "\n".join(sections)
    )


def lock_privacy_compliance_index() -> str:
    paths = LOCK_PRIVACY_PATHS + COMPLIANCE_PATHS
    selection_reasons = {
        "Cargo.lock": "libbun facade workspace lock",
        "native/Cargo.lock": "native linked engine lock",
        "runtime/Cargo.lock": "worker runtime lock",
        "tests/fixtures/public_api_boundary/Cargo.lock": "external privacy fixture lock",
        "tests/fixtures/public_api_boundary/Cargo.toml": "external privacy package",
        "tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs": "adjacent public-control fixture",
        "tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs": "raw installer call refusal fixture",
        "tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs": "raw installer import refusal fixture",
        "tests/public_api_boundary.rs": "privacy harness owner",
        "LICENSE": "libbun source-package license",
        "vendor/README.md": "vendored-source policy",
        "vendor/bun.LIBBUN_VENDOR.json": "Bun provenance and linked dependency declaration",
        "vendor/bun/LICENSE.md": "Bun source-package license",
        "vendor/bun/Cargo.lock": "vendored Bun locked dependency graph",
        "vendor/bun/Cargo.toml": "vendored Bun workspace and dependency selection",
        "vendor/bun/src/clap/LICENSE": "linked Bun clap source license",
        "vendor/bun/src/unicode/uucode_lib/LICENSE.md": "linked Bun Unicode source license",
        "vendor/bun/vendor/lolhtml/LICENSE": "provenance-declared linked lolhtml license",
    }
    selected_rows = []
    for path in paths:
        data = git_blob(ROOT, LIBBUN_SHA, path)
        blob = git_blob_oid(ROOT, LIBBUN_SHA, path)
        package_names = sorted(set(re.findall(rb'^name = "([^"]+)"$', data, re.MULTILINE)))
        package_summary = str(len(package_names)) if path.endswith("Cargo.lock") else "-"
        selected_rows.append(
            f"| {path} | {blob} | {sha256(data)} | {len(data)} | {package_summary} | "
            f"{selection_reasons[path]} |"
        )
    tracked = run(["git", "ls-tree", "-r", "--name-only", LIBBUN_SHA], ROOT)[1].splitlines()
    cargo_paths = sorted(path for path in tracked if path.endswith(("Cargo.toml", "Cargo.lock")))
    license_paths = sorted(
        path for path in tracked
        if re.search(r"(^|/)(LICENSE|LICENCE|NOTICE|COPYING)([._-].*)?$", path, re.IGNORECASE)
    )
    inventory_rows = []
    for family, inventory in (("Cargo manifest/lock", cargo_paths), ("license/notice", license_paths)):
        for path in inventory:
            data = git_blob(ROOT, LIBBUN_SHA, path)
            inventory_rows.append(
                f"| {family} | {path} | {git_blob_oid(ROOT, LIBBUN_SHA, path)} | "
                f"{sha256(data)} | {len(data)} | {'selected' if path in paths else 'inventory only'} |"
            )
    return clean(
        "# Lock, privacy fixture, license, provenance, and compliance index\n\n"
        f"Exact product SHA: {LIBBUN_SHA}\n\n"
        f"Exact product tree: {LIBBUN_TREE}\n\n"
        "The table below selects the bounded linked native/package closure: all four nonvendored locks, the complete six-file external privacy harness, "
        "the vendored workspace/lock, source-package licenses, vendor provenance, and licenses for the linked Bun/JSC dependencies named by that provenance. "
        "The second table inventories every tracked Cargo manifest/lock and license/notice path so the selected closure is reproducible without claiming that every vendored tool/test license is attached.\n\n"
        "## Selected direct attachments\n\n"
        "| Path | Git blob | SHA-256 | Bytes | Unique lock packages | Selection reason |\n"
        "| --- | --- | --- | ---: | ---: | --- |\n"
        + "\n".join(selected_rows)
        + "\n\n## Exact-tree inventory\n\n"
        f"Command: {command_text(ROOT, ['ls-tree', '-r', '--name-only', LIBBUN_SHA])}\n\n"
        "| Family | Path | Git blob | SHA-256 | Bytes | Disposition |\n"
        "| --- | --- | --- | --- | ---: | --- |\n"
        + "\n".join(inventory_rows)
        + "\n"
    )


REPORTS = {
    "exact-source-search-report.md": exact_source_search_report,
    "vendored-bun-boundary-report.md": vendored_boundary_report,
    "adjacent-swarm-source-index.md": adjacent_source_index,
    "lifecycle-vendored-jsc-source-bundle.md": lifecycle_jsc_bundle,
    "process-drop-caller-and-fixture-report.md": process_drop_report,
    "lock-privacy-compliance-index.md": lock_privacy_compliance_index,
    "adjacent-generative-source-bundle.md": adjacent_generative_source_bundle,
    "lifecycle-process-worker-source-bundle.md": lifecycle_process_worker_source_bundle,
}


def write_exact(path: Path, data: bytes) -> None:
    destination = ROOT / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(data)
    print(f"WROTE {path}")


def refresh() -> None:
    for path in ADJACENT_PATHS:
        write_exact(SNAPSHOT_BASE / path, git_blob(SWARM_ROOT, SWARM_SHA, path))
    for destination, (commit, source_path) in CORRECTION2_VERDICTS.items():
        write_exact(BASE / destination, git_blob(ROOT, commit, source_path))
    for destination, (commit, source_path) in CORRECTION3_VERDICTS.items():
        write_exact(BASE / destination, git_blob(ROOT, commit, source_path))
    for name, generate in REPORTS.items():
        write_exact(BASE / name, generate().encode())


def refresh_dry_runs(parts: tuple[str, ...] = PARTS) -> None:
    for part in parts:
        prompt = (ROOT / BASE / f"{part}-prompt.md").read_text()
        files = (ROOT / BASE / f"{part}-files.txt").read_text().splitlines()
        result = subprocess.run(
            [
                "oracle", "--provider", "openai", "--engine", "api",
                "--model", "gpt-5.6-sol", "--reasoning-mode", "pro",
                "--dry-run", "summary", "--files-report", "--prompt", prompt,
                *[argument for path in files for argument in ("--file", path)],
            ],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
        )
        if result.returncode:
            raise RuntimeError(
                f"Oracle dry-run failed for {part} ({result.returncode}):\n"
                + result.stdout.decode(errors="replace")
            )
        write_exact(BASE / f"{part}-oracle-dry-run.txt", result.stdout)


def file_digest(path: Path) -> str:
    return sha256((ROOT / path).read_bytes())


def ordered_attachments(part: str) -> list[dict[str, object]]:
    paths = (ROOT / BASE / f"{part}-files.txt").read_text().splitlines()
    attachments = []
    for raw in paths:
        path = Path(raw)
        data = (ROOT / path).read_bytes()
        attachments.append({"path": raw, "sha256": sha256(data), "bytes": len(data)})
    return attachments


def dry_run_counts(part: str) -> tuple[int, int]:
    text = (ROOT / BASE / f"{part}-oracle-dry-run.txt").read_text()
    call = re.search(r"would call gpt-5\.6-sol with ~([0-9,]+) tokens", text)
    total = re.search(r"^Total: ([0-9,]+) tokens", text, re.MULTILINE)
    if not call or not total:
        raise RuntimeError(f"cannot parse Oracle dry-run counts for {part}")
    return int(total.group(1).replace(",", "")), int(call.group(1).replace(",", ""))


def fable_plan(part: str, attachments: list[dict[str, object]]) -> str:
    prompt_path = BASE / f"{part}-prompt.md"
    plan_path = BASE / f"{part}-files.txt"
    if part == "synthesis":
        prior_line = "- Prior independent verdicts: all three correction-3 parts are PART BUNDLE REVISE"
    else:
        commit, _ = PRIOR_VERDICTS[part]
        prior_line = f"- Prior independent verdict: PART BUNDLE REVISE at {commit}"
    rows = "\n".join(
        f"| {index} | {item['path']} | {item['sha256']} | {item['bytes']} |"
        for index, item in enumerate(attachments, 1)
    )
    return clean(
        f"# Correction-4 Fable file plan: {PART_TITLES[part]}\n\n"
        f"{prior_line}\n"
        "- State: NOT LAUNCHED\n"
        "- Engine: local Fable wrapper\n"
        "- Model: claude-fable-5\n"
        "- Effort: max\n"
        "- Deliverable: CONCRETE IMPLEMENTATION\n"
        f"- Prompt: {prompt_path}\n"
        f"- Prompt SHA-256: {file_digest(prompt_path)}\n"
        f"- Ordered file plan: {plan_path}\n"
        f"- Ordered file count: {len(attachments)}\n"
        "- Identical to Oracle ordered attachments: yes\n\n"
        "## Ordered attachments\n\n"
        "| # | Path | SHA-256 | Bytes |\n"
        "| ---: | --- | --- | ---: |\n"
        f"{rows}\n\n"
        "No Fable session, request, response, or output exists. Fresh literal independent "
        "PART BUNDLE PASS verdicts for correction 4 remain required before launch.\n"
    )


def manifest(part: str, attachments: list[dict[str, object]]) -> dict[str, object]:
    prompt_path = BASE / f"{part}-prompt.md"
    plan_path = BASE / f"{part}-files.txt"
    dry_path = BASE / f"{part}-oracle-dry-run.txt"
    fable_path = BASE / f"{part}-fable-plan.md"
    total_tokens, call_tokens = dry_run_counts(part)
    result: dict[str, object] = {
        "schema": "libbun.w1112.external-review-manifest.v4",
        "correction": 4,
        "part": part,
        "title": PART_TITLES[part],
        "exact_source_sha": LIBBUN_SHA,
        "exact_source_tree": LIBBUN_TREE,
        "review_base_commit": REVIEW_BASE,
        "adjacent_source": {
            "repository": "SWARM_REPO",
            "sha": SWARM_SHA,
            "tree": SWARM_TREE,
        },
        "verdict_contract_commit": VERDICT_CONTRACT,
        "deliverable": "CONCRETE IMPLEMENTATION",
        "evidence_generator": {
            "path": str(Path("scripts/generate-libbun-w1112-review-evidence-20260724.py")),
            "sha256": file_digest(Path("scripts/generate-libbun-w1112-review-evidence-20260724.py")),
            "check_command": "LIBBUN_REPO=<libbun-checkout> SWARM_REPO=<swarm-checkout> python3 scripts/generate-libbun-w1112-review-evidence-20260724.py --check",
        },
        "bundle_verifier": {
            "path": str(VERIFIER),
            "sha256": file_digest(VERIFIER),
            "command": f"SWARM_REPO=<swarm-checkout> python3 {VERIFIER}",
            "independent_checkout_replay": True,
        },
        "prompt": {"path": str(prompt_path), "sha256": file_digest(prompt_path)},
        "ordered_file_plan": {
            "path": str(plan_path),
            "sha256": file_digest(plan_path),
            "count": len(attachments),
        },
        "ordered_attachments": attachments,
        "total_attachment_bytes": sum(int(item["bytes"]) for item in attachments),
        "oracle": {
            "provider": "openai",
            "engine": "api",
            "model": "gpt-5.6-sol",
            "reasoning_mode": "pro",
            "required_live_banner": [
                "first-party OpenAI", "gpt-5.6-sol", "Responses API Pro", "xhigh reasoning"
            ],
            "dry_run_command": (
                f"mapfile -t files < {plan_path}; oracle --provider openai --engine api "
                f"--model gpt-5.6-sol --reasoning-mode pro --dry-run summary --files-report "
                f"--prompt \"$(cat {prompt_path})\" --file \"${{files[@]}}\""
            ),
            "dry_run_report": {"path": str(dry_path), "sha256": file_digest(dry_path)},
            "estimated_total_tokens": total_tokens,
            "estimated_call_tokens": call_tokens,
            "state": "NOT LAUNCHED",
            "session_id": None,
            "request_id": None,
            "response_id": None,
            "output_paths": [],
        },
        "fable": {
            "model": "claude-fable-5",
            "effort": "max",
            "file_plan": {"path": str(fable_path), "sha256": file_digest(fable_path)},
            "state": "NOT LAUNCHED",
            "session_id": None,
            "request_id": None,
            "response_id": None,
            "output_paths": [],
        },
        "correction_evidence_state": "CORRECTION 4 COMPLETE; FRESH LITERAL INDEPENDENT REVIEW PENDING",
        "independent_bundle_review": {
            "reviewer": "PENDING correction-4 source-aware independent reviewer",
            "verdict": "PENDING; literal PART BUNDLE PASS required before launch",
        },
        "omissions": [
            "Oracle and Fable response artifacts are intentionally absent because model launch is out of scope.",
            "Fresh literal independent PART BUNDLE PASS at the correction-4 commit is pending and this manifest does not authorize launch.",
        ],
        "launch_state": "NOT LAUNCHED",
    }
    if part == "synthesis":
        result["prior_independent_verdicts"] = [
            {"part": name, "commit": PRIOR_VERDICTS[name][0], "verdict": "PART BUNDLE REVISE"}
            for name in PARTS[:3]
        ]
        result["synthesis_inputs"] = [
            {
                "part": name,
                "manifest_path": str(BASE / f"{name}-manifest.json"),
                "manifest_sha256": file_digest(BASE / f"{name}-manifest.json"),
                "state": "FRESH LITERAL PART BUNDLE PASS PENDING",
            }
            for name in PARTS[:3]
        ]
    else:
        commit, record = PRIOR_VERDICTS[part]
        record_path = BASE / record
        result["prior_independent_verdict"] = {
            "commit": commit,
            "verdict": "PART BUNDLE REVISE",
            "records": [{"path": str(record_path), "sha256": file_digest(record_path)}],
        }
    return result


def refresh_metadata() -> None:
    refresh_dry_runs(PARTS[:3])
    for part in PARTS[:3]:
        attachments = ordered_attachments(part)
        write_exact(BASE / f"{part}-fable-plan.md", fable_plan(part, attachments).encode())
        payload = json.dumps(manifest(part, attachments), indent=2, sort_keys=False) + "\n"
        write_exact(BASE / f"{part}-manifest.json", payload.encode())
    refresh_dry_runs(("synthesis",))
    attachments = ordered_attachments("synthesis")
    write_exact(BASE / "synthesis-fable-plan.md", fable_plan("synthesis", attachments).encode())
    payload = json.dumps(manifest("synthesis", attachments), indent=2, sort_keys=False) + "\n"
    write_exact(BASE / "synthesis-manifest.json", payload.encode())


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit", choices=REPORTS)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--refresh", action="store_true")
    parser.add_argument("--refresh-dry-runs", action="store_true")
    parser.add_argument("--refresh-metadata", action="store_true")
    args = parser.parse_args()
    if sum((bool(args.emit), args.check, args.refresh, args.refresh_dry_runs, args.refresh_metadata)) != 1:
        parser.error(
            "choose exactly one of --emit REPORT, --check, --refresh, --refresh-dry-runs, or --refresh-metadata"
        )
    if args.emit:
        sys.stdout.write(REPORTS[args.emit]())
        return 0
    if args.refresh:
        refresh()
        return 0
    if args.refresh_dry_runs:
        refresh_dry_runs()
        return 0
    if args.refresh_metadata:
        refresh_metadata()
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
