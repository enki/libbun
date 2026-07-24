#!/usr/bin/env python3
"""Deterministically generate correction-6 exact-source evidence for W1-11/W1-12."""

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

SEMANTIC_OWNER_SEARCH_PATTERN = (
    "ProviderBoundaryOutputCorrespondenceFault|"
    "consume_corresponded_ready_output_for_provider_boundary_owner_v1|"
    "ProviderBoundaryIngressFault|DirectRunProcessChildProviderFaultV1|"
    "DirectRunProcessSessionDriveFaultV1|ProviderDriveSessionExecutionCommitFault|"
    "ProviderHostExecutionSession|begin_provider_execution_session_v1|"
    "cross_boundary_swap_is_a_typed_fault|nominal_join_preserves_both_halves_on_mismatch"
)

# Exact ordered repository-wide semantic owner/correspondence result at
# SWARM_SHA.  Unlike the older 24-path lexical regression set, this gate binds
# every active execution-session producer/carrier/consumer and final release
# path named by the correction-5 independent verdict.
SEMANTIC_OWNER_PATHS = (
    "crates/durable-native-provider-loader/src/lib.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/public_aperture_entrypoint/trusted_step.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/public_aperture_drive.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/session_route_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_result_route.rs",
    "crates/ss-runtime-source-compiler-owner/src/lib.rs",
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_direct_run_prepared_runtime.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/final_observation/host_resource_finalization.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/root.inc.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store/types.rs",
    "crates/swarm-capability-model/src/lib.rs",
    "crates/swarm-capability-model/src/provider_boundary_correspondence.rs",
    "crates/swarm-provider-host-set/src/lib.rs",
    "crates/swarm-provider-host-set/src/provider_host_set.rs",
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs",
)

PROVIDER_EXECUTION_SESSION_SEARCH_PATTERN = (
    "ProviderHostExecutionSession|begin_provider_execution_session_v1"
)

PROVIDER_EXECUTION_SESSION_PATHS = (
    "crates/durable-native-provider-loader/src/lib.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/public_aperture_entrypoint/trusted_step.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/public_aperture_drive.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/session_route_lifecycle.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_result_route.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_direct_run_prepared_runtime.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/final_observation/host_resource_finalization.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs",
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store/types.rs",
    "crates/swarm-provider-host-set/src/lib.rs",
    "crates/swarm-provider-host-set/src/provider_host_set.rs",
)

# The lexical 24-path hit set is a regression gate, not the complete semantic
# SCC.  These exact-source supplements bind the final correspondence join,
# typed drive/fault carriers, selected finite owner, its public boundary and
# destructor, and the constructor-side dependency/caller.
OWNER_SUPPLEMENTAL_PATHS = (
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs",
    "crates/swarm-provider-host-set/src/lib.rs",
    "crates/durable-native-provider-loader/src/lib.rs",
    "crates/durable-native-provider-loader/Cargo.toml",
)

OWNER_SOURCE_PATHS = tuple(
    dict.fromkeys(GENERATIVE_PATHS + OWNER_SUPPLEMENTAL_PATHS + SEMANTIC_OWNER_PATHS)
)

FINAL_CLOSE_SEARCH_PATTERN = (
    "close_for_execution_graph_owner|shutdown_runtime_execution_domain_owner|"
    "SsRuntimeExecutionDomainOwner|ProviderSettlementLane|ExternalCapabilityProviderPool"
)

FINAL_CLOSE_DISCOVERED_PATHS = (
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs",
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

ADJACENT_PATHS = tuple(dict.fromkeys(BASE_ADJACENT_PATHS + OWNER_SOURCE_PATHS + FINAL_CLOSE_DISCOVERED_PATHS + (
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

CORRECTION4_VERDICTS = {
    "owner-generative-correction4-independent-verdict.md": (
        "d7292c2c3beaabb807efc5b551f4beaae1d70a3c",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction4-independent-verdict.md",
    ),
    "lifecycle-correction4-independent-verdict.md": (
        "d6f9ae079eea0d635115fabae13526b29266b491",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction4-independent-verdict.md",
    ),
    "containment-release-correction4-independent-verdict.md": (
        "d6f9ae079eea0d635115fabae13526b29266b491",
        "docs/reviews/libbun-w1112-20260724/containment-release-correction4-independent-verdict.md",
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
        "5e74c14a0125c1670be7e37cc31675ebedcd538d",
        "correction5-independent-full-family-verdict.md",
        "PART BUNDLE REVISE",
    ),
    "lifecycle": (
        "5e74c14a0125c1670be7e37cc31675ebedcd538d",
        "correction5-independent-full-family-verdict.md",
        "PART BUNDLE PASS",
    ),
    "containment-release": (
        "5e74c14a0125c1670be7e37cc31675ebedcd538d",
        "correction5-independent-full-family-verdict.md",
        "PART BUNDLE PASS",
    ),
}
REVIEW_BASE = "5e74c14a0125c1670be7e37cc31675ebedcd538d"
VERDICT_CONTRACT = "5e74c14a0125c1670be7e37cc31675ebedcd538d"
VERIFIER = Path("scripts/verify-libbun-w1112-review-bundle-20260724.py")

GENERATIVE_EXCERPT_SPANS = {
    "crates/ss-runtime-source-compiler-owner/src/direct_run.rs": ((362, 399),),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime.rs": ((961, 982), (1134, 1234), (1655, 1676)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/session_route_lifecycle.rs": ((535, 593),),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_result_route.rs": ((397, 608),),
    "crates/ss-runtime-source-compiler-owner/src/lib.rs": ((1, 66),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_direct_run_prepared_runtime.rs": ((1168, 1310), (1426, 1540)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs": ((769, 840), (882, 990)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine.rs": ((14, 54), (169, 181), (402, 420), (474, 486)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/event/mod.rs": ((1, 30), (225, 679)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs": ((1, 30), (822, 902), (1122, 1147)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs": ((1, 123),),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs": ((1, 35), (85, 445), (1387, 1410), (1586, 1940)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/provider_resume_lifecycle.rs": ((1, 35), (140, 336)),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/public_aperture_drive.rs": ((1, 530),),
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs": ((1, 30), (45, 119), (130, 180), (400, 450), (491, 519)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime.rs": ((190, 220),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs": ((215, 238),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs": ((1, 40), (148, 197)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/process_carriers.rs": ((1, 20), (130, 174), (582, 678)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/root.inc.rs": ((260, 305),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs": ((544, 600), (710, 760)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_runtime_stores_impl.rs": ((1, 30), (560, 618), (1332, 1437)),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store.rs": ((1, 75),),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/work_runtime/work_store/types.rs": ((1, 30), (250, 423), (860, 904), (1674, 1695)),
    "crates/swarm-capability-model/src/lib.rs": ((1, 30), (71, 227)),
    "crates/swarm-provider-host-set/src/lib.rs": ((1, 22),),
    "crates/swarm-provider-host-set/src/provider_host_set.rs": ((1, 30), (42, 54), (267, 276), (650, 730), (839, 1038)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs": ((1, 30), (500, 565)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs": ((1, 35), (110, 190), (680, 730), (904, 975), (1177, 1374), (2550, 2611)),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs": ((400, 530),),
    "crates/durable-native-provider-loader/src/lib.rs": ((332, 374),),
}

GENERATIVE_REQUIRED_ITEMS = {
    "crates/ss-runtime-source-compiler-owner/src/direct_run.rs": (
        "pub(crate) use self::direct_run_runtime_authority_refs::DirectRunProcessSessionDriveFaultV1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/authority_kernel/prepared_runtime.rs": (
        "DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1",
        "AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1",
        "DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1",
        "provider_execution_session: swarm_provider_host_set::ProviderHostExecutionSession",
        "drive_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1",
        "durable_refusal_keeps_command_beside_the_typed_host_refusal",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/public_aperture_entrypoint/trusted_step.rs": (
        "direct_run_public_aperture_prepared_runtime_process_start_admission_input_v1",
        "admit_direct_run_public_aperture_prepared_runtime_process_start_v1",
        "drive_direct_run_public_aperture_prepared_runtime_process_start_command_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/session_route_lifecycle.rs": (
        "drive_start_route_host_resource_finalization_for_owner_v1",
        "drive_reawaken_route_host_resource_finalization_for_owner_v1",
        "drive_provider_resume_route_host_resource_finalization_for_owner_v1",
        "provider_execution_session: &mut ProviderHostExecutionSession",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_result_route.rs": (
        "drive_matching_child_for_process_kernel_owner_v1",
        "DirectRunHostResourceFinalizationNextStepV1",
        "provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession",
        "drive_for_direct_run_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/lib.rs": (
        "mod source_entrypoint_direct_run_prepared_runtime",
        "ProviderDriveSessionExecutionCommitFault",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_direct_run_prepared_runtime.rs": (
        "direct_run_ss_test_body_work_materialization_from_process_dispatch_product_for_compiler_owner_v1",
        "admit_source_entrypoint_direct_run_prepared_runtime_process_start_for_compiler_owner_v1",
        "provider_host_set.begin_provider_execution_session_v1()",
        "cancel_into_generic_message_for_direct_run_boundary_owner_v1",
        "drive_source_entrypoint_direct_run_prepared_runtime_process_start_until_terminal_for_compiler_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_value/host_resources.rs": (
        "OneShotHostResourceFinalizationObligation",
        "SelectedProviderHostResourceReleaseV1",
        "commit_exact_provider_release_for_session_execution_kernel_owner_v1",
        "commit_selected_host_resource_release_borrowed_for_session_execution_kernel_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/final_observation/host_resource_finalization.rs": (
        "SelectedHostResourceFinalizationSelectionDropGuardV1",
        "PresentedHostResourceFinalizationSelectionV1",
        "try_reissue_cancelled_selection_for_session_runtime_owner_v1",
        "commit_exact_provider_release_for_session_execution_kernel_owner_v1",
        "selected_drop_reissues_twenty_thousand_times_with_128_kib_custody",
        "presented_guard_cancels_during_unwind_and_reissues",
        "provider_refusal_returns_exact_obligation_cancelled_for_rebind_retry",
        "process_session_provider_commit_unwind_retains_exact_custody_through_retry_publication",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine.rs": (
        "ProviderBoundaryIngressFault",
        "NeedsHostResourceFinalization",
        "ProcessSessionSchedulerQuiescenceProof",
        "ProcessSessionSchedulerPhaseOutcomeKind::Quiescent",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs": (
        "consume_corresponded_ready_output_for_provider_boundary_owner_v1",
        "ProviderBoundaryIngressFault::from",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs": (
        "pub enum ProviderBoundaryIngressFault",
        "OutputCorrespondence",
        "ProviderBoundaryOutputCorrespondenceFault",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs": (
        "DirectRunProcessChildProviderFaultV1",
        "HostAdmission",
        "HostExecution",
        "commit_process_child_provider_drive_result_for_process_kernel_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs": (
        "DirectRunProcessSessionDriveFaultV1",
        "cancel_into_generic_message_for_direct_run_boundary_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs": (
        "ProviderDriveSessionExecutionCommitFault",
        "commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs": (
        "apply_provider_drive_ready_result_for_live_process_session",
        "commit_selected_host_resource_finalization_for_live_process_session",
        "provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/scheduler/phase_machine_drive_entrypoints.inc.rs": (
        "commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1",
        "provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession",
        "consume_exact_selection_for_session_runtime_owner_v1",
        "commit_exact_provider_release_for_session_execution_kernel_owner_v1",
        "drop(consumed_pending)",
    ),
    "crates/swarm-capability-model/src/lib.rs": (
        "pub enum CapabilitySdkError",
    ),
    "crates/swarm-capability-model/src/provider_boundary_correspondence.rs": (
        "mint_provider_boundary_output_correspondence_v1",
        "ReadyOutputDoesNotMatchPendingBoundary",
        "cross_boundary_swap_is_a_typed_fault",
        "cross_boundary_swap_preserves_typed_settlement_refusal",
        "nominal_join_preserves_both_halves_on_mismatch",
    ),
    "crates/swarm-provider-host-set/src/provider_host_set.rs": (
        "pub struct ProviderHostExecutionSession",
        "begin_provider_execution_session_v1",
        "invoke_selected_provider_boundary_request_for_direct_run_owner_v1",
        "impl Drop for ProviderHostExecutionSession",
    ),
    "crates/swarm-provider-host-set/src/lib.rs": (
        "ProviderHostExecutionSession",
    ),
    "crates/durable-native-provider-loader/src/lib.rs": (
        "pub fn begin_provider_execution_session_v1",
    ),
    "crates/durable-native-provider-loader/Cargo.toml": (
        "swarm-provider-host-set",
    ),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/host_set.rs": (
        "SelectedProviderBoundaryHostRequest",
    ),
    "crates/swarm-rust-sdk-static-provider-host/src/lib_parts/tests.rs": (
        "exact_contract_tson_result_ok_settles_only_the_accepted_payload",
        "exact_contract_tson_result_err_settles_only_the_rejected_payload",
        "non_result_closed_sum_with_result_shaped_object_remains_authored_cargo",
    ),
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
    "vendor/bun/src/jsc/bindings/NodeVMModule.cpp",
)

LIFECYCLE_EXCERPT_SPANS = {
    "vendor/bun/src/runtime/node/node_process.rs": ((1, 64),),
    "vendor/bun/src/runtime/jsc_hooks.rs": ((1188, 1199), (1397, 1428), (1518, 1527)),
    "vendor/bun/src/jsc/web_worker.rs": ((90, 203), (233, 396), (540, 710), (990, 1346)),
    "vendor/bun/src/jsc/bindings/BunProcess.cpp": ((280, 304), (1205, 1247), (3245, 3263)),
    "vendor/bun/src/jsc/bindings/webcore/Worker.cpp": ((350, 430),),
    "vendor/bun/src/jsc/bindings/vm/SigintWatcher.cpp": ((105, 208),),
    "vendor/bun/src/jsc/bindings/NodeVM.cpp": ((847, 870),),
    "vendor/bun/src/jsc/bindings/NodeVMScript.cpp": ((278, 300),),
    "vendor/bun/src/jsc/bindings/NodeVMModule.cpp": ((52, 151),),
}

LIFECYCLE_REQUIRED_ITEMS = {
    "vendor/bun/src/runtime/node/node_process.rs": ("Bun__Process__exit", "global_exit"),
    "vendor/bun/src/runtime/jsc_hooks.rs": ("terminate_all_workers_and_wait",),
    "vendor/bun/src/jsc/web_worker.rs": (
        "terminate_all_and_wait",
        "live_workers::register",
        "live_workers::unregister",
        "fn spin(&self)",
        "fn shutdown(&self)",
        "pub fn exit(&self)",
    ),
    "vendor/bun/src/jsc/bindings/BunProcess.cpp": (
        "Bun__handleUncaughtException",
        "Bun__Process__exit(lexicalGlobalObject, 1)",
        "Bun__Process__exit(zigGlobal, exitCode)",
    ),
    "vendor/bun/src/jsc/bindings/webcore/Worker.cpp": ("WebWorker__notifyNeedTermination",),
    "vendor/bun/src/jsc/bindings/NodeVMScript.cpp": ("clearHasTerminationRequest",),
    "vendor/bun/src/jsc/bindings/NodeVMModule.cpp": (
        "NodeVMModule::evaluate",
        "drainMicrotasksForGlobalObject",
        "clearHasTerminationRequest",
        "ERR_SCRIPT_EXECUTION_INTERRUPTED",
        "ERR_SCRIPT_EXECUTION_TIMEOUT",
    ),
}

FINAL_CLOSE_EXCERPT_SPANS = {
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs": ((295, 311),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs": ((1, 85),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs": ((47, 176),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs": ((1, 140),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs": ((700, 736),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs": ((1424, 1435),),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs": ((1560, 1612),),
}

FINAL_CLOSE_REQUIRED_ITEMS = {
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs": (
        "pub(in crate::test_runner) fn close_for_execution_graph_owner",
        "live_feed_session.close_for_execution_graph_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs": (
        "mod runtime_execution_domain",
        "mod source_work_set_worker_execution",
        "SsRuntimeExecutionDomainOwner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs": (
        "struct ExternalCapabilityProviderPool",
        "pub(super) fn shutdown",
        "provider.shutdown()?",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs": (
        "provider_pool.shutdown()",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs": (
        "pub(super) struct SsRuntimeExecutionDomainOwner",
        "provider_settlement_pool.shutdown()",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs": (
        "fn close_for_execution_graph_owner",
        "runtime_execution_domain_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs": (
        "fn shutdown_runtime_execution_domain_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs": (
        "fn close_for_execution_graph_owner",
        "close_check_for_feed_close",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs": (
        "produce_graph_close_receipt_for_execution_graph_owner",
        "shutdown_and_reap_for_execution_graph_owner_v1",
        "settle_runtime_plan_closeout_succeeded",
        "settle_runtime_plan_closeout_failed",
        "runtime_file_execution_session =",
    ),
}

RETAINED_HOST_SHUTDOWN_PATHS = (
    "crates/ss-runtime-test-execution-owner/src/lib.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
    "crates/ss/tests/external_capability_provider.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs",
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs",
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
            "Preserves the exact 24-path lexical selected-invocation/output-correspondence hit set before semantic supplements are added.",
            "Exit 0 with exactly the 24 lexical paths bound as a regression gate, not a complete SCC claim.",
        ),
        search_section(
            "Adjacent final correspondence, typed-fault, finite-owner, and caller closure",
            SWARM_ROOT,
            SWARM_SHA,
            SEMANTIC_OWNER_SEARCH_PATTERN,
            ("crates",),
            "Discovers the complete final correspondence join, typed ingress/drive faults, selected execution-session owner lifecycle, constructor-side callers, and capability-model hostile evidence.",
            "Exit 0 with exactly the 27 ordered active owning paths bound in the compact owner bundle and exact adjacent snapshots.",
        ),
        search_section(
            "Adjacent ProviderHostExecutionSession producer, carrier, and consumer closure",
            SWARM_ROOT,
            SWARM_SHA,
            PROVIDER_EXECUTION_SESSION_SEARCH_PATTERN,
            ("crates",),
            "Discovers both source-entrypoint execution-session mints, every by-value and borrowed carrier, the finite host-set owner, exact release consumer, and public reexport.",
            "Exit 0 with exactly the 16 ordered active paths bound as complete owning source items and exact adjacent snapshots.",
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
        "# libbun W1-11/W1-12 exact-source search report (correction 6)\n\n"
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


def discovered_semantic_owner_paths() -> tuple[str, ...]:
    args = ["grep", "-l", "-E", SEMANTIC_OWNER_SEARCH_PATTERN, SWARM_SHA, "--", "crates"]
    exit_code, output = run(["git", *args], SWARM_ROOT)
    if exit_code != 0:
        raise RuntimeError("repository-wide adjacent semantic owner discovery failed")
    prefix = f"{SWARM_SHA}:"
    paths = tuple(
        line[len(prefix):] if line.startswith(prefix) else line
        for line in output.splitlines()
        if line
    )
    if paths != SEMANTIC_OWNER_PATHS:
        raise RuntimeError(
            "adjacent semantic owner closure drift; expected="
            + repr(SEMANTIC_OWNER_PATHS)
            + " observed="
            + repr(paths)
        )
    return paths


def discovered_provider_execution_session_paths() -> tuple[str, ...]:
    args = [
        "grep", "-l", "-E", PROVIDER_EXECUTION_SESSION_SEARCH_PATTERN,
        SWARM_SHA, "--", "crates",
    ]
    exit_code, output = run(["git", *args], SWARM_ROOT)
    if exit_code != 0:
        raise RuntimeError("repository-wide ProviderHostExecutionSession discovery failed")
    prefix = f"{SWARM_SHA}:"
    paths = tuple(
        line[len(prefix):] if line.startswith(prefix) else line
        for line in output.splitlines()
        if line
    )
    if paths != PROVIDER_EXECUTION_SESSION_PATHS:
        raise RuntimeError(
            "ProviderHostExecutionSession closure drift; expected="
            + repr(PROVIDER_EXECUTION_SESSION_PATHS)
            + " observed="
            + repr(paths)
        )
    return paths


def owner_semantic_search_report() -> str:
    semantic_paths = discovered_semantic_owner_paths()
    session_paths = discovered_provider_execution_session_paths()
    semantic_args = [
        "grep", "-l", "-E", SEMANTIC_OWNER_SEARCH_PATTERN, SWARM_SHA, "--", "crates",
    ]
    session_args = [
        "grep", "-l", "-E", PROVIDER_EXECUTION_SESSION_SEARCH_PATTERN,
        SWARM_SHA, "--", "crates",
    ]
    semantic_rows = "\n".join(f"{index:02d}. `{path}`" for index, path in enumerate(semantic_paths, 1))
    session_rows = "\n".join(f"{index:02d}. `{path}`" for index, path in enumerate(session_paths, 1))
    return clean(
        "# W1-11/W1-12 correction-6 compact semantic owner discovery\n\n"
        f"Adjacent Swarm SHA: `{SWARM_SHA}`  \n"
        f"Adjacent Swarm tree: `{SWARM_TREE}`\n\n"
        + repository_prologue()
        + "\n## Exact semantic owner/correspondence discovery\n\n"
        f"Command: {command_text(SWARM_ROOT, semantic_args)}\n\n"
        f"Pattern: `{SEMANTIC_OWNER_SEARCH_PATTERN}`\n\n"
        f"Observed ordered count: {len(semantic_paths)}\n\n"
        f"{semantic_rows}\n\n"
        "## Exact ProviderHostExecutionSession producer/carrier/consumer discovery\n\n"
        f"Command: {command_text(SWARM_ROOT, session_args)}\n\n"
        f"Pattern: `{PROVIDER_EXECUTION_SESSION_SEARCH_PATTERN}`\n\n"
        f"Observed ordered count: {len(session_paths)}\n\n"
        f"{session_rows}\n\n"
        "Both ordered results are executable fail-closed generator inputs. Every listed path has an exact-SHA snapshot and a complete owning-source binding in `adjacent-generative-source-bundle.md`; these path-only results are discovery evidence and never substitute for owning source.\n"
    )


def discovered_final_close_paths() -> tuple[str, ...]:
    args = [
        "grep", "-l", "-E", FINAL_CLOSE_SEARCH_PATTERN, SWARM_SHA, "--",
        "crates/ss-runtime-test-execution-owner/src",
    ]
    exit_code, output = run(["git", *args], SWARM_ROOT)
    if exit_code != 0:
        raise RuntimeError("repository-wide retained-runtime final-close discovery failed")
    prefix = f"{SWARM_SHA}:"
    paths = tuple(
        line[len(prefix):] if line.startswith(prefix) else line
        for line in output.splitlines()
        if line
    )
    if paths != FINAL_CLOSE_DISCOVERED_PATHS:
        raise RuntimeError(
            "retained-runtime final-close closure drift; expected="
            + repr(FINAL_CLOSE_DISCOVERED_PATHS)
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
    required_items: dict[str, tuple[str, ...]],
    introduction: str,
) -> str:
    rows = []
    sections = []
    emitted_by_path: dict[str, bytes] = {}
    for path in paths:
        data = git_blob(repo, sha, path)
        file_sha, blob, size = identity_row(repo, sha, path)
        mode = "complete owning items" if path in spans else "complete file"
        rows.append(f"| {path} | {blob} | {file_sha} | {size} | {mode} |")
        if path in spans:
            emitted = bytearray()
            for start, end in spans[path]:
                sections.append(excerpt(repo, sha, path, start, end))
                emitted.extend(("\n".join(data.decode(errors="replace").splitlines()[start - 1:end]) + "\n").encode())
            emitted_by_path[path] = bytes(emitted)
        else:
            sections.append(full_source(repo, sha, path))
            emitted_by_path[path] = data
    for path, terms in required_items.items():
        if path not in emitted_by_path:
            raise RuntimeError(f"{title}: required source path was not emitted: {path}")
        emitted_text = emitted_by_path[path].decode(errors="replace")
        for term in terms:
            if term not in emitted_text:
                raise RuntimeError(
                    f"{title}: {path} selected owning item lacks required term {term}"
                )
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
    discovered_generative_paths()
    discovered_semantic_owner_paths()
    discovered_provider_execution_session_paths()
    return source_bundle(
        "Adjacent Swarm owner, correspondence, drive, final-output, fault, and test source bundle (correction 6)",
        SWARM_ROOT,
        SWARM_SHA,
        SWARM_TREE,
        OWNER_SOURCE_PATHS,
        GENERATIVE_EXCERPT_SPANS,
        GENERATIVE_REQUIRED_ITEMS,
        "This bundle preserves the exact repository-wide 24-path lexical hit set and independently binds the exact ordered 27-path semantic owner closure plus the 16-path ProviderHostExecutionSession producer/carrier/consumer closure. It exposes both real session mints, by-value admission and terminal-drive custody, every borrowed route carrier, the exact host-resource release selection/commit and final publication path, complete Drop/reissue/refusal/retry/unwind tests, the existing one-occurrence output seal, final correspondence join and ingress wrapper, typed mechanical drive/fault carriers, capability-model mismatch/custody-preservation tests, and static-host nominal accepted/rejected/authored correspondence tests. No static-host mismatch or replay test exists at the bound SHA; new static-host mismatch and retained-libbun replay tests remain mandatory implementation output. A new package/invocation brand must extend or atomically replace the existing seal; a parallel seal is forbidden.",
    )


def lifecycle_process_worker_source_bundle() -> str:
    return source_bundle(
        "Vendored Bun process-exit, VM interruption/reset, WebWorker quiescence, and ordered shutdown source bundle (correction 6)",
        ROOT,
        LIBBUN_SHA,
        LIBBUN_TREE,
        LIFECYCLE_SOURCE_PATHS,
        LIFECYCLE_EXCERPT_SPANS,
        LIFECYCLE_REQUIRED_ITEMS,
        "The ordered source path is public or uncaught-exception-handler process exit -> Rust process owner -> main-VM global exit or worker exit -> concrete RuntimeHooks binding -> process-global worker registry termination sweep/wait -> per-worker termination checkpoint -> ordered VM unpublish, exit handlers, JSC teardown, unregister, exit dispatch, and worker-resource destruction. NodeVMModule::evaluate and NodeVMScript bind timeout/SIGINT exception clearing and VM termination-request reset. A reset is not quiescence: timeout, exception-path ambiguity, or surviving child/nested-worker state cannot authorize reuse.",
    )


def atomic_deletion_tests_source_bundle() -> str:
    paths = discovered_final_close_paths()
    return source_bundle(
        "Adjacent Swarm retained-runtime final close, atomic deletion, and hostile-test owner source bundle (correction 6)",
        SWARM_ROOT,
        SWARM_SHA,
        SWARM_TREE,
        paths,
        FINAL_CLOSE_EXCERPT_SPANS,
        FINAL_CLOSE_REQUIRED_ITEMS,
        "The exact repository-wide final-close discovery binds the retained provider pool and settlement lane, the separately defined shutdown helper, the consuming live-feed close, both outer close carriers, their module/privacy boundary, and the sole final success/fault consumer. The final consumer first shuts down the runtime-file pool, consumes graph settlements and the live feed, then either consumes the runtime-file session into successful settlement or restores it while marking closeout failed. A correction must reconcile the already-consumed retained runtime on failure, perform exact-once final shutdown, forbid retry from a consumed backend, delete stale plugin/callback/raw compatibility atomically, and require hostile refusal/retry/cancel/unwind/Drop/shutdown tests.",
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
        "# libbun W1-11/W1-12 vendored Bun boundary report (correction 6)\n\n"
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
        "# Vendored JSC lifecycle supplemental source bundle (correction 6)\n\n"
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
            "clear[Tt]ermination|clearHasTerminationRequest|has[Tt]ermination[Rr]equest|WebWorker__notifyNeedTermination|"
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
    "owner-semantic-search-report.md": owner_semantic_search_report,
    "vendored-bun-boundary-report.md": vendored_boundary_report,
    "adjacent-swarm-source-index.md": adjacent_source_index,
    "lifecycle-vendored-jsc-source-bundle.md": lifecycle_jsc_bundle,
    "process-drop-caller-and-fixture-report.md": process_drop_report,
    "lock-privacy-compliance-index.md": lock_privacy_compliance_index,
    "adjacent-generative-source-bundle.md": adjacent_generative_source_bundle,
    "lifecycle-process-worker-source-bundle.md": lifecycle_process_worker_source_bundle,
    "atomic-deletion-tests-source-bundle.md": atomic_deletion_tests_source_bundle,
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
    for destination, (commit, source_path) in CORRECTION4_VERDICTS.items():
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
        prior_line = "- Prior independent verdicts at 5e74c14a0125c1670be7e37cc31675ebedcd538d: owner/correspondence PART BUNDLE REVISE; lifecycle and containment/release PART BUNDLE PASS; correction-6 synthesis is blocked pending only the corrected owner pass"
    else:
        commit, _, verdict = PRIOR_VERDICTS[part]
        prior_line = f"- Prior independent verdict: {verdict} at {commit}"
    rows = "\n".join(
        f"| {index} | {item['path']} | {item['sha256']} | {item['bytes']} |"
        for index, item in enumerate(attachments, 1)
    )
    return clean(
        f"# Correction-6 Fable file plan: {PART_TITLES[part]}\n\n"
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
        "The correction-5 lifecycle and containment/release PART BUNDLE PASS verdicts remain controlling. A fresh literal owner/correspondence PART BUNDLE PASS for correction 6 remains required before launch, and synthesis stays blocked until that owner verdict passes.\n"
    )


def manifest(part: str, attachments: list[dict[str, object]]) -> dict[str, object]:
    prompt_path = BASE / f"{part}-prompt.md"
    plan_path = BASE / f"{part}-files.txt"
    dry_path = BASE / f"{part}-oracle-dry-run.txt"
    fable_path = BASE / f"{part}-fable-plan.md"
    total_tokens, call_tokens = dry_run_counts(part)
    result: dict[str, object] = {
        "schema": "libbun.w1112.external-review-manifest.v6",
        "correction": 6,
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
        "correction_evidence_state": "CORRECTION 6 OWNER CLOSURE COMPLETE; FRESH LITERAL OWNER REVIEW PENDING",
        "independent_bundle_review": {
            "reviewer": "PENDING correction-6 source-aware owner/correspondence reviewer",
            "verdict": "PENDING; literal PART BUNDLE PASS required before launch",
        },
        "omissions": [
            "Oracle and Fable response artifacts are intentionally absent because model launch is out of scope.",
            "Fresh literal independent owner/correspondence PART BUNDLE PASS at the correction-6 commit is pending and this manifest does not authorize launch.",
        ],
        "launch_state": "NOT LAUNCHED",
    }
    if part == "synthesis":
        result["correction_evidence_state"] = (
            "CORRECTION 6 SYNTHESIS BLOCKED; FRESH LITERAL OWNER/CORRESPONDENCE PART BUNDLE PASS PENDING"
        )
        result["prior_independent_verdicts"] = [
            {"part": name, "commit": PRIOR_VERDICTS[name][0], "verdict": PRIOR_VERDICTS[name][2]}
            for name in PARTS[:3]
        ]
        result["synthesis_inputs"] = [
            {
                "part": name,
                "manifest_path": str(BASE / f"{name}-manifest.json"),
                "manifest_sha256": file_digest(BASE / f"{name}-manifest.json"),
                "state": (
                    "FRESH LITERAL OWNER PART BUNDLE PASS PENDING"
                    if name == "owner-generative"
                    else "CORRECTION-5 LITERAL PART BUNDLE PASS PRESERVED"
                ),
            }
            for name in PARTS[:3]
        ]
    else:
        commit, record, verdict = PRIOR_VERDICTS[part]
        record_path = BASE / record
        result["prior_independent_verdict"] = {
            "commit": commit,
            "verdict": verdict,
            "records": [{"path": str(record_path), "sha256": file_digest(record_path)}],
        }
        if verdict == "PART BUNDLE PASS":
            result["correction_evidence_state"] = (
                "CORRECTION 6 UNION CLOSURE ONLY; CORRECTION-5 LITERAL PART BUNDLE PASS PRESERVED"
            )
            result["independent_bundle_review"] = {
                "reviewer": "correction-5 source-aware independent full-family reviewer",
                "verdict": "PART BUNDLE PASS preserved; family not reopened",
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
