#![allow(dead_code)]
#[cfg(test)]
use crate::source_entrypoint_compiler_admission_session::checker_resource::package_graph_resource_manifest_contract_checker_projection_from_tson_source_for_package_graph_checker_context_owner_v1 as package_graph_resource_manifest_contract_checker_projection_from_tson_source;
use durable_direct_run_provider_result_mode_model::DirectSwarmScriptRunProviderResultMode;
use durable_direct_run_source_import_model::DirectSwarmScriptRunPendingSourceImport;
use durable_execution_core::process_store_owner_authority::{
    ProcessStoreAuthority, ProductFrontdoorDurablePostgresLivePlanCommitApi,
    ProductFrontdoorInMemoryLivePlanRouteRuntime,
};
use durable_execution_core::{
    ActorStateProductApi, ActorStateStoreOwnerRef, CommandAdmissionRegistry,
    DurableExecutionLivePlanActorStateRouteProductFrontdoorHandoff,
    DurableExecutionLivePlanPatchResultCommitReceipt, DurableExecutionSupportedLiveOperationFacts,
    DurableExecutionSupportedLiveStreamFacts, DurableExecutionSupportedWakeTargetPersistCommand,
    DurablePostgresActorStateProductApi, DurablePostgresActorStateProductApiProviderRequired,
    DurablePostgresEventJournalProductApiProviderRequired,
    DurablePostgresLivePlanConnectionProviderKind,
    DurablePostgresLivePlanConnectionProviderPolicyEvidence,
    DurablePostgresLivePlanStatementExecutor, DurablePostgresLivePlanTokioClientProvider,
    InMemoryEventJournalProductApi, RootScopeId, SubstrateInvariantLedgerV1,
    SubstrateTransitionKind, SubstrateTransitionRecordV1,
};
#[cfg(test)]
use libswarm_package_graph_checker_model::package_graph_builtin_checker_context_module_ids;
use libswarm_package_graph_checker_model::package_graph_is_builtin_checker_context_module;
use libswarm_package_graph_contract_source_admission::{
    PackageGraphPreparedRuntimeContractTsonDerivationInput,
    PackageGraphRuntimeContractTsonDerivationInput,
    PackageGraphRuntimeContractTsonDerivationInputArtifactSidecarForPackageGraphContractSourceOwner,
};
use libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority;
use libswarm_package_graph_model::{CapabilityContractResolutionRecord, ProviderImportIdentity};
use libswarm_package_graph_provider_requirements::{
    PackageGraphImplementationDeclaration,
    PackageGraphStaticProviderContractForProviderRequirementsOwner,
};
use libswarm_package_graph_root_resolution::{
    normalize_package_graph_path as normalize_direct_run_path,
    normalize_package_graph_path_string as normalize_direct_run_path_string,
};
use libswarm_package_graph_source_model::package_source_path as shared_package_source_path;
use libswarm_package_graph_source_session::{
    PackageGraphSourceSessionManifestReceiptProduct, PackageGraphSourceSessionReceiptProduct,
};
use prepared_runtime_image_manifest_model::{
    ImplementationSelectionOwner, PreparedRuntimeImageProviderImportIdentity,
    PreparedRuntimeProviderImportAuthoritySet,
    PreparedRuntimeProviderImportExecutionStartAdmissionSet,
    PreparedRuntimeProviderImportManifestObservation,
};
use serde::{Serialize, Serializer};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

fn duplicate_prepared_runtime_contract_tson_derivation_inputs_for_direct_run_owner(
    inputs: &[PackageGraphPreparedRuntimeContractTsonDerivationInput],
) -> Vec<PackageGraphPreparedRuntimeContractTsonDerivationInput> {
    inputs
        .iter()
        .map(|input| input.duplicate_for_direct_run_prepared_runtime_owner_v1())
        .collect()
}
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
type DirectRunKernelRawAuthorityPoison =
    swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary;

#[derive(Debug)]
enum OneShotActorDeliveryWakeRequirement {}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::direct_run) struct DirectRunLiveProcessSessionRef {
    id: String,
}

impl DirectRunLiveProcessSessionRef {
    pub(in crate::direct_run) fn generated_for_direct_run_live_process_session_registry_owner_v1(
        id: String,
    ) -> Self {
        Self { id }
    }

    pub(in crate::direct_run) fn as_str(&self) -> &str {
        &self.id
    }

    pub(in crate::direct_run) fn duplicate_for_direct_run_live_process_session_registry_owner_v1(
        &self,
    ) -> Self {
        Self {
            id: self.id.clone(),
        }
    }
}

use crate::{
    ProcessSessionCheckpointV0 as EngineProcessSessionCheckpointV1,
    ProcessSessionV0 as EngineLiveProcessSessionV1,
    SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    SelectedProviderResumeBoundaryForDirectRunOwnerV1,
};
use swarmscript_engine_types::{
    EngineProcessSessionFaultV1, serialize_engine_process_session_fault_v1,
};
use swarmscript_source::SourcePath;
use swarmvm_host_abi::process_creation::ProcessExportKeyCarrier;
use swarmvm_host_abi::{
    ACTOR_CLOSE_RECEIVER_EXPORT_NAME, ACTOR_DELIVERY_EFFECT_EXPORT_NAME,
    ACTOR_DELIVERY_EFFECT_RESULT_EXPORT_NAME, ACTOR_DEMONITOR_EXPORT_NAME, ACTOR_LINK_EXPORT_NAME,
    ACTOR_MONITOR_EXPORT_NAME, ACTOR_OPEN_RECEIVER_EXPORT_NAME, ACTOR_REPLY_EXPORT_NAME,
    ACTOR_REQUEST_EFFECT_CANCEL_EXPORT_NAME, ACTOR_REQUEST_EFFECT_EXPORT_NAME,
    ACTOR_REQUEST_EFFECT_NOTIFY_EXPORT_NAME, ACTOR_REQUEST_EFFECT_RESULT_EXPORT_NAME,
    ACTOR_REQUEST_EFFECT_STATUS_EXPORT_NAME, ACTOR_REQUEST_EFFECT_TERMINAL_EXPORT_NAME,
    ACTOR_REQUEST_EXPORT_NAME, ACTOR_SEND_EXPORT_NAME, ACTOR_START_EXPORT_NAME,
    ACTOR_UNLINK_EXPORT_NAME, CapabilityIdentity, SWARM_ACTOR_EFFECTS_MODULE_SPECIFIER,
};
use swarmvm_image::{HandleKind, HostAbiVersion, HostInteractionFamily};
use swarmvm_isa::{
    HostActivityResultMode, VmBoundaryObjectValue as EngineVmObjectValueV1,
    VmBoundaryValue as EngineVmValueV1,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
struct DirectSwarmScriptRunModuleImportId(String);

impl DirectSwarmScriptRunModuleImportId {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for DirectSwarmScriptRunModuleImportId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for DirectSwarmScriptRunModuleImportId {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl AsRef<str> for DirectSwarmScriptRunModuleImportId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DirectSwarmScriptRunModuleImportId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSwarmScriptRunContractModule {
    id: String,
    source_text: String,
    source_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    canonical_source_path: Option<String>,
    source_fingerprint: String,
    #[serde(default)]
    imports: Vec<DirectSwarmScriptRunModuleImportId>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSwarmScriptRunSourceModule {
    id: String,
    source_text: String,
    source_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    canonical_source_path: Option<String>,
    source_fingerprint: String,
    #[serde(default)]
    imports: Vec<DirectSwarmScriptRunModuleImportId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum DirectSwarmScriptRunPassiveSourceAssetKind {
    LiquidTemplate,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSwarmScriptRunPassiveSourceAsset {
    id: String,
    kind: DirectSwarmScriptRunPassiveSourceAssetKind,
    source_text: String,
    source_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    canonical_source_path: Option<String>,
    source_fingerprint: String,
    #[serde(default)]
    imports: Vec<DirectSwarmScriptRunModuleImportId>,
}

#[derive(Clone, Debug, PartialEq)]
enum DirectSwarmScriptRunPackageResolutionPurpose {
    SourceImport,
    ProviderModule,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::direct_run) struct DirectSwarmScriptRunSemanticTypeRef {
    type_bundle_hash: String,
    symbol_path: String,
}

impl DirectSwarmScriptRunSemanticTypeRef {
    fn duplicate_for_direct_run_process_model_owner_v1(&self) -> Self {
        Self {
            type_bundle_hash: self.type_bundle_hash.clone(),
            symbol_path: self.symbol_path.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DirectSwarmScriptRunProcessRunInput {
    launch_options: DirectSwarmScriptRunProcessLaunchOptions,
}

impl DirectSwarmScriptRunProcessRunInput {
    fn new(launch_options: DirectSwarmScriptRunProcessLaunchOptions) -> Self {
        Self { launch_options }
    }

    fn duplicate_for_direct_run_process_model_owner_v1(&self) -> Self {
        Self {
            launch_options: self
                .launch_options
                .duplicate_for_direct_run_process_model_owner_v1(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct DirectSwarmScriptRunProcessLaunchOptions {
    continuity_key: Option<String>,
    on_existing: Option<String>,
    future_adoptability: Option<String>,
}

impl DirectSwarmScriptRunProcessLaunchOptions {
    fn new(
        continuity_key: Option<String>,
        on_existing: Option<String>,
        future_adoptability: Option<String>,
    ) -> Self {
        Self {
            continuity_key,
            on_existing,
            future_adoptability,
        }
    }

    fn duplicate_for_direct_run_process_model_owner_v1(&self) -> Self {
        Self {
            continuity_key: self.continuity_key.clone(),
            on_existing: self.on_existing.clone(),
            future_adoptability: self.future_adoptability.clone(),
        }
    }
}

fn direct_run_boundary_value_to_json(value: &EngineVmValueV1) -> Value {
    json!({
        "shape": swarmvm_runtime_types::one_shot_executable_value_diagnostic_text_for_swarmvm_session_runtime_owner_v1(value),
        "logicalByteEstimate": swarmvm_runtime_types::runtime_memory_boundary_value_logical_byte_estimate_for_swarmvm_session_runtime_observation_owner_v1(value, 4096),
    })
}

#[derive(Clone, Debug, PartialEq)]
enum DirectSwarmScriptRunProcessExecutionTarget {
    Run,
    Invoke(DirectSwarmScriptRunProcessInvokeTarget),
}

impl DirectSwarmScriptRunProcessExecutionTarget {
    fn duplicate_for_direct_run_process_model_owner_v1(&self) -> Self {
        match self {
            Self::Run => Self::Run,
            Self::Invoke(invoke) => {
                Self::Invoke(invoke.duplicate_for_direct_run_process_model_owner_v1())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DirectSwarmScriptRunProcessInvokeTarget {
    export_name: String,
    target_member_path: Vec<String>,
    result_type_ref: DirectSwarmScriptRunSemanticTypeRef,
    output_callable_ref_paths: Vec<Vec<String>>,
}

impl DirectSwarmScriptRunProcessInvokeTarget {
    fn new(
        export_name: String,
        target_member_path: Vec<String>,
        result_type_ref: DirectSwarmScriptRunSemanticTypeRef,
        output_callable_ref_paths: Vec<Vec<String>>,
    ) -> Self {
        Self {
            export_name,
            target_member_path,
            result_type_ref,
            output_callable_ref_paths,
        }
    }

    fn duplicate_for_direct_run_process_model_owner_v1(&self) -> Self {
        Self {
            export_name: self.export_name.clone(),
            target_member_path: self.target_member_path.clone(),
            result_type_ref: self
                .result_type_ref
                .duplicate_for_direct_run_process_model_owner_v1(),
            output_callable_ref_paths: self.output_callable_ref_paths.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::direct_run) struct DirectSwarmScriptRunProcessProgramRequest {
    program_id: String,
    payload: DirectSwarmScriptRunProcessProgramRequestPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::direct_run) enum DirectSwarmScriptRunProcessProgramRequestPayload {
    SourceText {
        source_path: String,
        source_text: String,
        source_fingerprint: String,
        contract_modules: Vec<DirectSwarmScriptRunContractModule>,
    },
    GraphFunctionMaterialization {
        module_id: String,
        binding_name: String,
        graph_function_key: String,
    },
    StaticChildSource {
        source_module_id: String,
    },
}

include!("direct_run/part_000_diagnostics_core.rs");
include!("direct_run/part_000_diagnostic_repro_conformance.rs");
include!("direct_run/part_000_diagnostic_exports.rs");
include!("direct_run/part_000_observable_effect_expectations.rs");
include!("direct_run/part_000_rust_profile_runtime.rs");
include!("direct_run/part_001_memory_materialization_facts.rs");
#[path = "direct_run/capability_admission.rs"]
mod capability_admission;
#[path = "direct_run/capability_binding_surface.rs"]
mod capability_binding_surface;
pub(in crate::direct_run) use capability_binding_surface::DirectRunCapabilityIdentityLookupKey;

#[path = "direct_run/runtime_authority/mod.rs"]
pub(in crate::direct_run) mod direct_run_runtime_authority_refs;
pub(crate) use self::direct_run_runtime_authority_refs::DirectRunProcessSessionDriveFaultV1;
#[cfg(test)]
use self::direct_run_runtime_authority_refs::*;
pub use self::direct_run_runtime_authority_refs::{
    DirectRunProcessSessionPublicApertureDriveOutputV1,
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    DirectRunSsTestBodyWorkMaterializationAuthority,
    DirectRunSsTestBodyWorkMaterializationRootAuthority,
    DirectRunSsTestCaseTerminalObservationAuthority, DirectRunSsTestExecutedFileResultAuthority,
    DirectRunSsTestExecutedFileTerminalObservationCursor,
    DirectRunSsTestExecutedFileTerminalObservationProductAuthority,
    DirectRunSsTestTerminalObservationFaultV1, enter_direct_run_public_abi_boundary,
};
use self::direct_run_runtime_authority_refs::{
    DirectRunProcessSessionTerminalFinalizationProductV1, DirectRunRuntimeAuthorityOwner,
    EngineProcessSessionTerminalResultProductV1,
    known_rust_internal_provider_target_for_authored_facade_binding,
    known_rust_internal_provider_target_for_selected_operation,
};

include!("direct_run/part_001.rs");
include!("direct_run/part_002.rs");
include!("direct_run/part_003.rs");
include!("direct_run/run/mod.rs");
