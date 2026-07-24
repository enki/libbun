#!/usr/bin/env python3
"""Fail-closed correction-5 verifier for the libbun W1-11/W1-12 review bundle."""

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
REVIEW_BASE = "4dd3395129a221d8c1fb2d1dbbdae509b2331f0e"
REVIEW_BASE_TREE = "fbd9f82cfae0554abe87623f080f0ce4eb1c6b91"
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
        "d7292c2c3beaabb807efc5b551f4beaae1d70a3c",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction4-independent-verdict.md",
        "docs/reviews/libbun-w1112-20260724/owner-generative-correction4-independent-verdict.md",
    ),
    "lifecycle": (
        "d6f9ae079eea0d635115fabae13526b29266b491",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction4-independent-verdict.md",
        "docs/reviews/libbun-w1112-20260724/lifecycle-correction4-independent-verdict.md",
    ),
    "containment-release": (
        "d6f9ae079eea0d635115fabae13526b29266b491",
        "docs/reviews/libbun-w1112-20260724/containment-release-correction4-independent-verdict.md",
        "docs/reviews/libbun-w1112-20260724/containment-release-correction4-independent-verdict.md",
    ),
}

GENERATIVE_SEARCH_PATTERN = (
    "DurableExternalProviderInvocationAuthority|SelectedProviderResumeHostInputForDirectRunOwnerV1|"
    "SelectedProviderBoundaryHostRequest|SelectedProviderBoundaryExecutionResultForProviderHostOwner|"
    "mint_provider_boundary_output_correspondence_v1|PendingProviderBoundaryOutputCommitAuthority|"
    "invoke_selected_provider_boundary_request_for_direct_run_owner_v1|"
    "admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1"
)

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

OWNER_SUPPLEMENTAL_PATHS = (
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs",
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs",
    "crates/swarm-provider-host-set/src/lib.rs",
    "crates/durable-native-provider-loader/src/lib.rs",
    "crates/durable-native-provider-loader/Cargo.toml",
)

OWNER_SOURCE_PATHS = tuple(dict.fromkeys(GENERATIVE_PATHS + OWNER_SUPPLEMENTAL_PATHS))

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

GENERATIVE_REQUIRED_ITEMS = {
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/execution_kernel/executable_image/plan/operation_algebra/boundary_and_work_selection.rs": (
        "consume_corresponded_ready_output_for_provider_boundary_owner_v1", "ProviderBoundaryIngressFault::from",
    ),
    "crates/ss-runtime-source-compiler-owner/src/source_entrypoint_executable_runtime/errors.rs": (
        "pub enum ProviderBoundaryIngressFault", "OutputCorrespondence", "ProviderBoundaryOutputCorrespondenceFault",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/process_session_public_aperture/process_child_lifecycle.rs": (
        "DirectRunProcessChildProviderFaultV1", "HostAdmission", "HostExecution",
        "commit_process_child_provider_drive_result_for_process_kernel_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/mod.rs": (
        "DirectRunProcessSessionDriveFaultV1", "cancel_into_generic_message_for_direct_run_boundary_owner_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/provider_drive_result.rs": (
        "ProviderDriveSessionExecutionCommitFault",
        "commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1",
    ),
    "crates/ss-runtime-source-compiler-owner/src/direct_run/runtime_authority/live_process_session_registry.rs": (
        "apply_provider_drive_ready_result_for_live_process_session",
    ),
    "crates/swarm-capability-model/src/lib.rs": ("pub enum CapabilitySdkError",),
    "crates/swarm-capability-model/src/provider_boundary_correspondence.rs": (
        "mint_provider_boundary_output_correspondence_v1", "ReadyOutputDoesNotMatchPendingBoundary",
        "cross_boundary_swap_is_a_typed_fault", "cross_boundary_swap_preserves_typed_settlement_refusal",
        "nominal_join_preserves_both_halves_on_mismatch",
    ),
    "crates/swarm-provider-host-set/src/provider_host_set.rs": (
        "pub struct ProviderHostExecutionSession", "begin_provider_execution_session_v1",
        "invoke_selected_provider_boundary_request_for_direct_run_owner_v1",
        "impl Drop for ProviderHostExecutionSession",
    ),
    "crates/swarm-provider-host-set/src/lib.rs": ("ProviderHostExecutionSession",),
    "crates/durable-native-provider-loader/src/lib.rs": ("pub fn begin_provider_execution_session_v1",),
    "crates/durable-native-provider-loader/Cargo.toml": ("swarm-provider-host-set",),
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

LIFECYCLE_REQUIRED_ITEMS = {
    "vendor/bun/src/runtime/node/node_process.rs": ("Bun__Process__exit", "global_exit"),
    "vendor/bun/src/runtime/jsc_hooks.rs": ("terminate_all_workers_and_wait",),
    "vendor/bun/src/jsc/web_worker.rs": (
        "terminate_all_and_wait", "live_workers::register", "live_workers::unregister",
        "fn spin(&self)", "fn shutdown(&self)", "pub fn exit(&self)",
    ),
    "vendor/bun/src/jsc/bindings/BunProcess.cpp": (
        "Bun__handleUncaughtException", "Bun__Process__exit(lexicalGlobalObject, 1)",
        "Bun__Process__exit(zigGlobal, exitCode)",
    ),
    "vendor/bun/src/jsc/bindings/webcore/Worker.cpp": ("WebWorker__notifyNeedTermination",),
    "vendor/bun/src/jsc/bindings/NodeVMScript.cpp": ("clearHasTerminationRequest",),
    "vendor/bun/src/jsc/bindings/NodeVMModule.cpp": (
        "NodeVMModule::evaluate", "drainMicrotasksForGlobalObject", "clearHasTerminationRequest",
        "ERR_SCRIPT_EXECUTION_INTERRUPTED", "ERR_SCRIPT_EXECUTION_TIMEOUT",
    ),
}

FINAL_CLOSE_REQUIRED_ITEMS = {
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs": (
        "pub(in crate::test_runner) fn close_for_execution_graph_owner", "live_feed_session.close_for_execution_graph_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs": (
        "mod runtime_execution_domain", "mod source_work_set_worker_execution", "SsRuntimeExecutionDomainOwner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs": (
        "struct ExternalCapabilityProviderPool", "pub(super) fn shutdown", "provider.shutdown()?",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs": (
        "provider_pool.shutdown()",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs": (
        "pub(super) struct SsRuntimeExecutionDomainOwner", "provider_settlement_pool.shutdown()",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs": (
        "fn close_for_execution_graph_owner", "runtime_execution_domain_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs": (
        "fn shutdown_runtime_execution_domain_owner",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs": (
        "fn close_for_execution_graph_owner", "close_check_for_feed_close",
    ),
    "crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs": (
        "produce_graph_close_receipt_for_execution_graph_owner", "shutdown_and_reap_for_execution_graph_owner_v1",
        "settle_runtime_plan_closeout_succeeded", "settle_runtime_plan_closeout_failed",
        "runtime_file_execution_session =",
    ),
}

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
        "mint_provider_boundary_output_correspondence_v1", "parallel package/invocation brand",
        "ProviderHostExecutionSession", "ProviderBoundaryIngressFault::OutputCorrespondence",
        "static-host mismatch and retained-libbun replay tests", "outer Swarm mechanical success/fault settlement",
    ),
    "lifecycle": (
        "BunProviderBackend", "DriveCustody", "InvocationReadyProof", "RetirementProof",
        "RetirementQuarantine", "DurableReaper", "QuarantineObservation",
        "QuarantineCompletionClaim", "RetiredDisposal", "terminate_all_workers_and_wait",
        "child-worker and nested-worker", "JSC__VM__deinit body is empty", "proven drained",
        "ExternalCapabilityProviderPool", "working-directory replacement",
        "Shutdown consumes", "BunProviderBackend", "CONCRETE IMPLEMENTATION",
        "future Drop", "final terminal Drop", "shutdown-origin custody never returns",
        "NodeVMModule::evaluate", "Bun__handleUncaughtException", "termination reset alone is never quiescence proof",
    ),
    "containment-release": (
        "BunProviderBackend", "Linux namespace", "macOS", "Windows job",
        "persistent bounded", "Same-worker", "replacement epoch", "all four nonvendored locks",
        "complete six-file privacy harness", "vendored workspace/lock", "exact-tree Cargo/license inventory",
        "immutable-tag", "freshly extracted", "caller-minted receipt", "CONCRETE IMPLEMENTATION",
        "close_for_execution_graph_owner", "shutdown_runtime_execution_domain_owner",
        "package-prepared-export-worker-release.sh", "current_native_plugin_asset()",
        "produce_graph_close_receipt_for_execution_graph_owner", "sole final success/fault consumer",
        "cannot retry from a consumed backend",
    ),
    "synthesis": (
        "producer -> exact selected call", "W1-10 ProviderValue", "BunProviderBackend",
        "generatively branded", "PreparedExport::drive", "mechanical-only settlement",
        "JSC__VM__deinit", "retirement/quarantine/durable reaper", "Containment is exact per platform",
        "Fifteen-Step Hard-Cut", "CONCRETE IMPLEMENTATION",
        "ProviderHostExecutionSession", "child/nested workers", "freshly extracted smoke",
        "Synthesis is BLOCKED", "ProviderBoundaryIngressFault::OutputCorrespondence",
        "NodeVMModule::evaluate", "final success/fault consumer",
    ),
}

POOL_ADJACENT = {
    str(SNAPSHOT_BASE / path) for path in ADJACENT_PATHS
    if "ss-runtime-test-execution-owner" in path or "ss-runtime-provider-host-set-owner" in path
}
RETAINED_HOST_ADJACENT = {
    str(SNAPSHOT_BASE / path) for path in (
        "crates/ss/tests/external_capability_provider.rs",
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
}

REQUIRED_ATTACHMENTS = {
    "owner-generative": {
        "src/lib.rs", "src/prepared_export.rs", "tests/public_api_boundary.rs",
        str(BASE / "correction5-index.md"),
        str(BASE / "owner-generative-correction4-independent-verdict.md"),
        str(BASE / "adjacent-swarm-source-index.md"),
        str(BASE / "adjacent-generative-source-bundle.md"),
        str(SNAPSHOT_BASE / "crates/swarm-capability-model/Cargo.toml"),
        str(SNAPSHOT_BASE / "crates/ss-runtime-source-compiler-owner/Cargo.toml"),
        str(SNAPSHOT_BASE / "crates/swarm-rust-sdk-static-provider-host/src/lib.rs"),
        str(SNAPSHOT_BASE / "crates/swarm-rust-sdk-static-provider-host/Cargo.toml"),
    },
    "lifecycle": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "vendor/bun/src/jsc/VirtualMachine.rs", "vendor/bun/src/jsc/JSGlobalObject.rs",
        "vendor/bun/src/jsc/VM.rs", "vendor/bun/src/jsc/virtual_machine_exports.rs",
        str(BASE / "correction5-index.md"),
        str(BASE / "lifecycle-correction4-independent-verdict.md"),
        str(BASE / "lifecycle-vendored-jsc-source-bundle.md"),
        str(BASE / "lifecycle-process-worker-source-bundle.md"),
        *RETAINED_HOST_ADJACENT,
        str(SNAPSHOT_BASE / "crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs"),
        str(SNAPSHOT_BASE / "crates/ss/src/product.rs"),
    },
    "containment-release": {
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        ".github/workflows/ci.yml",
        str(BASE / "correction5-index.md"),
        str(BASE / "containment-release-correction4-independent-verdict.md"),
        str(BASE / "atomic-deletion-tests-source-bundle.md"),
        str(BASE / "lock-privacy-compliance-index.md"),
        "docs/LIBBUN-WORKER-RELEASE-CONTRACT.md",
        "scripts/package-prepared-export-worker-release.sh",
        "scripts/prepare-native-bun-link.sh",
        "scripts/verify-vendored-bun-reproducible.sh",
        "scripts/verify-vendored-bun.sh",
        *LOCK_PRIVACY_COMPLIANCE, *RETAINED_HOST_ADJACENT,
    },
    "synthesis": {
        *(str(BASE / f"{part}-manifest.json") for part in PARTS[:3]),
        str(BASE / "correction5-index.md"),
        str(BASE / "owner-generative-correction4-independent-verdict.md"),
        str(BASE / "lifecycle-correction4-independent-verdict.md"),
        str(BASE / "containment-release-correction4-independent-verdict.md"),
        str(BASE / "adjacent-generative-source-bundle.md"),
        str(BASE / "lifecycle-process-worker-source-bundle.md"),
        str(BASE / "atomic-deletion-tests-source-bundle.md"),
        "src/prepared_export.rs", "native/src/lib.rs", "wire/src/lib.rs",
        "scripts/package-prepared-export-worker-release.sh",
        str(SNAPSHOT_BASE / "crates/ss/tests/external_capability_provider.rs"),
    },
}

TOP_LEVEL_NAMES = {
    ".gitattributes", "adjacent-swarm-source-index.md",
    "containment-release-correction2-independent-verdict.md",
    "containment-release-correction3-independent-verdict.md",
    "containment-release-correction4-independent-verdict.md",
    "containment-release-fable-plan.md", "containment-release-files.txt",
    "containment-release-independent-verdict.md", "containment-release-manifest.json",
    "containment-release-oracle-dry-run.txt", "containment-release-prompt.md",
    "correction2-index.md", "correction3-index.md", "correction4-index.md", "correction5-index.md",
    "exact-source-search-report.md", "adjacent-generative-source-bundle.md",
    "atomic-deletion-tests-source-bundle.md",
    "lifecycle-correction-ruling.md", "lifecycle-correction2-independent-verdict.md",
    "lifecycle-correction3-independent-verdict.md",
    "lifecycle-correction4-independent-verdict.md",
    "lifecycle-fable-plan.md", "lifecycle-files.txt", "lifecycle-independent-verdict.commit",
    "lifecycle-manifest.json", "lifecycle-oracle-dry-run.txt", "lifecycle-prompt.md",
    "lifecycle-vendored-jsc-source-bundle.md", "lifecycle-process-worker-source-bundle.md",
    "lock-privacy-compliance-index.md",
    "owner-generative-correction2-independent-verdict.md", "owner-generative-fable-plan.md",
    "owner-generative-correction3-independent-verdict.md",
    "owner-generative-correction4-independent-verdict.md",
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
            "correction-4 review base tree mismatch")
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
    require(local.stdout.count("OK ") == 9, "local generator did not verify all nine reports")

    require(not command("git", "status", "--porcelain").stdout,
            "independent-checkout replay requires the correction commit to be clean")
    head = command("git", "rev-parse", "HEAD").stdout.strip()
    with tempfile.TemporaryDirectory(prefix="libbun-w1112-c5-independent-") as temporary:
        clone = Path(temporary) / "different-checkout-name"
        command("git", "clone", "--shared", "--quiet", str(ROOT), str(clone), cwd=Path(temporary))
        command("git", "checkout", "--detach", "--quiet", head, cwd=clone)
        require(not command("git", "status", "--porcelain", cwd=clone).stdout,
                "independent replay checkout is not clean before generation")
        replay = run_generator_check(clone)
        require(replay.returncode == 0,
                "clean independent-checkout deterministic replay failed:\n" + replay.stdout + replay.stderr)
        require(replay.stdout.count("OK ") == 9,
                "independent generator did not verify all nine reports")
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
        "Adjacent repository-wide generative mint/carrier/consumer closure",
        "Adjacent final correspondence, typed-fault, finite-owner, and caller closure",
        "Adjacent W1-10 ProviderValue input and governing law",
        "Adjacent exact-call and invocation producers",
        "Adjacent sole consumer, transport, retained-host pool, and shutdown graph",
        "Adjacent retained-runtime final shutdown edge",
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
        "Repository-wide vendored process-exit, termination, worker-wait, and shutdown inventory",
        "Vendored VM termination, reset, drain, and deinit callers",
        "retained-host pool", "ExternalCapabilityProviderPool", "working_directory",
        "Adjacent external fixture graph",
        "NodeVMModule.cpp", "BunProcess.cpp:1238",
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

    atomic = (ROOT / BASE / "atomic-deletion-tests-source-bundle.md").read_text()
    for term in (
        "produce_graph_close_receipt_for_execution_graph_owner",
        "shutdown_and_reap_for_execution_graph_owner_v1",
        "settle_runtime_plan_closeout_succeeded",
        "settle_runtime_plan_closeout_failed",
        "runtime_file_execution_session =",
    ):
        require(term in atomic, f"atomic deletion/tests bundle lacks {term}")


def bundle_source_for_path(text: str, path: str) -> str:
    pattern = re.compile(rf"^## {re.escape(path)}(?::.*)?$", re.MULTILINE)
    starts = [match.start() for match in pattern.finditer(text)]
    sections: list[str] = []
    for start in starts:
        following = re.search(r"^## ", text[start + 1:], re.MULTILINE)
        end = len(text) if following is None else start + 1 + following.start()
        sections.append(text[start:end])
    return "\n".join(sections)


def remove_bundle_source_for_path(text: str, path: str) -> str:
    return re.sub(
        rf"^## {re.escape(path)}(?::.*)?$.*?(?=^## |\Z)",
        "",
        text,
        flags=re.MULTILINE | re.DOTALL,
    )


def verify_per_path_items(
    text: str,
    inventory_paths: tuple[str, ...],
    required_items: dict[str, tuple[str, ...]],
    family: str,
) -> None:
    for path in inventory_paths:
        require(f"| {path} |" in text, f"{family}: inventory lacks discovered path {path}")
        section = bundle_source_for_path(text, path)
        require(section, f"{family}: no source section for discovered path {path}")
        # Discovery-bound negative gate: removing the complete source section
        # must leave the discovered path without an owning-source binding even
        # though its inventory row and prompt prose remain present.
        require(not bundle_source_for_path(remove_bundle_source_for_path(text, path), path),
                f"{family}: source-section deletion did not break path binding {path}")
    for path, terms in required_items.items():
        section = bundle_source_for_path(text, path)
        for term in terms:
            require(term in section, f"{family}: {path} source item lacks {term}")
            mutated = section.replace(term, "")
            require(term not in mutated,
                    f"{family}: required-item deletion self-check failed for {path}: {term}")


def verify_source_closure() -> None:
    discovered = command(
        "git", "grep", "-l", "-E", GENERATIVE_SEARCH_PATTERN, SWARM_SHA, "--", "crates",
        cwd=SWARM_ROOT,
    ).stdout.splitlines()
    prefix = f"{SWARM_SHA}:"
    discovered_paths = tuple(
        line[len(prefix):] if line.startswith(prefix) else line for line in discovered
    )
    require(discovered_paths == GENERATIVE_PATHS,
            "repository-wide adjacent lexical hit set is not the exact 24-path regression set")

    generative = (ROOT / BASE / "adjacent-generative-source-bundle.md").read_text()
    verify_per_path_items(
        generative, OWNER_SOURCE_PATHS, GENERATIVE_REQUIRED_ITEMS, "owner/correspondence"
    )

    lifecycle = (ROOT / BASE / "lifecycle-process-worker-source-bundle.md").read_text()
    lifecycle_discovered = command(
        "git", "grep", "-l", "-E",
        "Bun__Process__exit|global_exit|terminate_all_workers_and_wait|terminate_all_and_wait|"
        "WebWorker__notifyNeedTermination|clearHasTerminationRequest|SigintWatcher|requestTermination",
        SOURCE_SHA, "--", *LIFECYCLE_SOURCE_PATHS,
    ).stdout.splitlines()
    lifecycle_prefix = f"{SOURCE_SHA}:"
    require(
        {line[len(lifecycle_prefix):] if line.startswith(lifecycle_prefix) else line for line in lifecycle_discovered}
        == set(LIFECYCLE_SOURCE_PATHS),
        "active process-exit/termination-reset lifecycle discovery drift",
    )
    verify_per_path_items(
        lifecycle, LIFECYCLE_SOURCE_PATHS, LIFECYCLE_REQUIRED_ITEMS, "lifecycle custody"
    )

    final_discovered = command(
        "git", "grep", "-l", "-E", FINAL_CLOSE_SEARCH_PATTERN, SWARM_SHA, "--",
        "crates/ss-runtime-test-execution-owner/src", cwd=SWARM_ROOT,
    ).stdout.splitlines()
    final_prefix = f"{SWARM_SHA}:"
    final_paths = tuple(
        line[len(final_prefix):] if line.startswith(final_prefix) else line
        for line in final_discovered
    )
    require(final_paths == FINAL_CLOSE_DISCOVERED_PATHS,
            "repository-wide retained-runtime final-close discovery drift")
    atomic = (ROOT / BASE / "atomic-deletion-tests-source-bundle.md").read_text()
    verify_per_path_items(
        atomic, FINAL_CLOSE_DISCOVERED_PATHS, FINAL_CLOSE_REQUIRED_ITEMS,
        "atomic deletion/tests final close",
    )

    retained = {
        path: (ROOT / SNAPSHOT_BASE / path).read_text()
        for path in (
            "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs",
            "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs",
            "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs",
            "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs",
            "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs",
            "crates/ss/tests/external_capability_provider.rs",
        )
    }
    retained_text = "\n".join(retained.values())
    for term in (
        "provider_pool.shutdown()", "provider_settlement_pool.shutdown()",
        "close_for_execution_graph_owner", "shutdown_runtime_execution_domain_owner",
        "ss_reuses_one_libbun_runtime_for_multiple_capability_imports",
        "ss_test_pool_child_conserves_package_roots_for_test_and_libbun_providers",
        "current_native_plugin_asset()", "libbun/plugin/target/release",
    ):
        require(term in retained_text, f"retained-host/final-shutdown source lacks {term}")
    require(retained_text.count("shutdown_runtime_execution_domain_owner") == 1,
            "retained-runtime shutdown helper call count drift; the exact source has one definition and zero calls")
    require("shutdown_runtime_execution_domain_owner" not in retained[
        "crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs"
    ], "exact stale final-close edge unexpectedly changed")

    release_paths = (
        "scripts/package-prepared-export-worker-release.sh",
        "scripts/prepare-native-bun-link.sh",
        "scripts/verify-vendored-bun-reproducible.sh",
        "scripts/verify-vendored-bun.sh",
        "docs/LIBBUN-WORKER-RELEASE-CONTRACT.md",
    )
    for path in release_paths:
        require((ROOT / path).read_bytes() == git_bytes(ROOT, f"{SOURCE_SHA}:{path}"),
                f"active release/package source drift: {path}")


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
    require(data.get("schema") == "libbun.w1112.external-review-manifest.v5", f"{part}: schema drift")
    require(data.get("correction") == 5 and data.get("part") == part, f"{part}: identity drift")
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
    require(review.get("reviewer", "").startswith("PENDING correction-5")
            and "PART BUNDLE PASS" in review.get("verdict", ""), f"{part}: literal pass gate drift")
    if part != "synthesis":
        commit, _, destination = PRIOR_VERDICTS[part]
        prior = data.get("prior_independent_verdict", {})
        require(prior.get("commit") == commit and prior.get("verdict") == "PART BUNDLE REVISE",
                f"{part}: correction-4 verdict binding drift")
        require(prior.get("records") == [{
            "path": destination, "sha256": digest(ROOT / destination)
        }], f"{part}: correction-4 verdict record drift")
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
            "synthesis correction-4 verdict binding drift")
    require("SYNTHESIS BLOCKED" in synthesis.get("correction_evidence_state", ""),
            "synthesis is not explicitly blocked pending part passes")


def main() -> int:
    verify_identity_and_delta()
    verify_verdicts_and_snapshots()
    verify_reports()
    verify_source_closure()
    manifests = {part: verify_part(part) for part in PARTS}
    verify_synthesis(manifests)
    verify_generator_replay()
    print(
        "PASS: correction-5 bundles are Lane-independent, exact-source complete, "
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
