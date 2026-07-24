use crate::ProviderDriveResult;
use crate::direct_run::{
    DirectProcessSessionResultProjection, DirectProcessSessionTerminalProjection,
    DirectRunLaunchDurabilityPolicyAuthority, DirectRunProcessSessionPublicApertureDriveOutputV1,
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1, DirectSwarmScriptRunKernelState,
    DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand, EngineVmObjectValueV1,
    direct_run_take_process_session_root_input,
};
use crate::session::{
    ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1,
    ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1,
};
use crate::{
    DirectRunExactStaticChildDispatchInstalledPreparedRuntimeForPreparedRuntimeOwnerV1 as EngineInstalledPreparedSessionRuntimeV1,
    ProcessSessionPublicDiagnosticProjectionAuthority as EnginePublicDiagnosticProjectionAuthority,
    ProcessSessionV0 as EngineLiveProcessSessionV1,
    SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProviderResumeHostInputForDirectRunOwnerV1,
    SelectedProviderResumeRouteForDirectRunOwnerV1,
    mint_process_invoke_execution_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_with_process_for_durable_direct_run_owner_v1,
    open_process_session_v0_from_exact_static_child_dispatch_installed_prepared_runtime_for_direct_run_owner_v1,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};
use swarm_capability_linker_core::ProviderValue;
use swarm_provider_host_set::ProviderHostExecutionSession;
use swarm_rust_sdk_static_provider_host::swarm_event_provider_requires_product_session_boundary;
use swarm_rust_sdk_static_provider_listing::{
    RustSdkBuiltinProviderCatalogue, SWARM_PROCESS_ACTIVATE_EXPORT,
    SWARM_PROCESS_CHECKPOINT_EXPORT, SWARM_PROCESS_INVOKE_EXPORT, SWARM_PROCESS_LOAD_EXPORT,
    SWARM_PROCESS_MODULE_ID, SWARM_PROCESS_RESTORE_EXPORT, SWARM_PROCESS_RUN_EXPORT,
};
use swarmvm_session_runtime_model::ProcessSessionDurabilityPolicyV0;

use super::kernel_state_substrate::{
    DirectRunDrivenProcessInvokeChildProductV1, DirectRunDrivenProcessRunChildProductV1,
    DirectRunHostResourceFinalizationNextStepV1, DirectRunKernelStateRefOwnerKind,
    DirectRunKernelStateRefRetentionKind, DirectRunProcessControlNextStepV1,
    DirectRunProcessInvokeAwaitExecutionNextStepV1, DirectRunProcessKernelBoundaryParentRouteV1,
    DirectRunProcessRunDriveTerminalNextStepV1, DirectRunProcessSessionContinuationDriveAuthority,
    DirectRunProcessSessionOwnerExecutionSubstrate,
    DirectRunProcessSessionProjectionDriveAuthority,
    DirectRunProcessSessionPublicApertureClosedDriveStateV1,
    DirectRunProcessSessionPublicApertureNextStepOutputProductV1,
    DirectRunProcessSessionPublicApertureRouteOutput,
    DirectRunProcessSessionReawakenContinuationToken,
    DirectRunProcessSessionReawakenDriveAuthority, DirectRunProcessSessionResultRouteAuthority,
    DirectRunProcessSessionStartContinuationToken, DirectRunProcessSessionStartDriveAuthority,
    DirectRunProviderResumeContinuationToken,
    DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
    DirectRunPublicApertureKernelStateRef,
    admit_direct_run_process_liveness_wait_from_typed_engine_boundary_v1,
    continue_after_process_session_reawaken_result_with_typed_authority,
    continue_after_process_session_result_with_typed_route_authority,
    continue_after_process_session_start_result_with_typed_authority,
    take_provider_resume_host_boundary_private_storage_from_public_aperture_kernel_state_ref_for_runtime_authority_owner,
};
use super::process_kernel_boundary::{
    DirectRunProcessActivateChildExecutionV1, DirectRunProcessKernelChildDriveContext,
    MatchedRegisteredProcessInvokeExecution, MatchedRegisteredProcessLifecycle,
    MatchedRegisteredProcessRunChild, ProcessCheckpointRegistrationAdmission,
    ProcessCheckpointRegistrySelection, ProcessInvokeExecutionRegistrationAdmission,
    ProcessInvokeExecutionRegistrySelection, ProcessLifecycleRegistrationAdmission,
    ProcessLifecycleRegistrySelection, ProcessRunChildRegistrationAdmission,
    ProcessRunChildRegistrySelection, RegisteredProcessRunChildExecutionV1,
    register_process_activate_child, register_process_activate_open_refusal,
    register_process_checkpoint, register_process_invoke_execution, register_process_lifecycle,
    register_process_run_child, select_registered_process_checkpoint,
    select_registered_process_invoke_execution, select_registered_process_lifecycle,
    select_registered_process_run_child,
};
use super::process_session_result_authority::DirectRunProcessChildTerminalMaterializationV1;
use super::runtime_operation::DirectRunRuntimeAuthorityOwner;
use super::typed_continuation_token_admission::require_process_session_projection_typed_token_holds_frame;
use super::{DirectRunProcessSessionDriveFaultV1, EngineProcessSessionRunResultV1};

pub(super) mod process_child_lifecycle;
pub(super) mod provider_resume_lifecycle;
mod public_aperture_drive;
pub(super) mod session_route_lifecycle;
