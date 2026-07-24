use std::collections::BTreeMap;

// Track I: finite dead-code owner rows for these authority/projection modules
// are documented in docs/tracks/i.md.
#[allow(dead_code)]
#[path = "checkpoint_host_resources.rs"]
mod checkpoint_host_resources;
#[path = "durability_policy.rs"]
mod durability_policy;
#[allow(dead_code)]
#[path = "machine.rs"]
mod machine;
#[path = "one_shot_boundary_events.rs"]
mod one_shot_boundary_events;
#[path = "one_shot_checkpoint_projection.rs"]
mod one_shot_checkpoint_projection;
#[path = "one_shot_failures.rs"]
mod one_shot_failures;
#[allow(dead_code)]
#[path = "one_shot_faults.rs"]
mod one_shot_faults;
#[path = "one_shot_host_interaction.rs"]
mod one_shot_host_interaction;
#[path = "one_shot_machine_primitives.rs"]
mod one_shot_machine_primitives;
#[path = "one_shot_metering.rs"]
mod one_shot_metering;
#[path = "one_shot_projection_api.rs"]
mod one_shot_projection_api;
#[path = "one_shot_run_api.rs"]
mod one_shot_run_api;
#[path = "one_shot_run_outcome.rs"]
mod one_shot_run_outcome;
#[allow(dead_code)]
#[path = "privileged_hostcalls.rs"]
mod privileged_hostcalls;
#[allow(dead_code)]
#[path = "process_liveness.rs"]
mod process_liveness;
#[path = "process_tree.rs"]
mod process_tree;
#[path = "runtime_obligation_owner.rs"]
mod runtime_obligation_owner;

#[path = "interaction_runtime_domain.rs"]
mod interaction_runtime_domain;
#[path = "volatile_coroutine_frames.rs"]
mod volatile_coroutine_frames;

pub(crate) use session::execution_kernel::executable_value::*;
pub use session::execution_kernel::provider_effect_runtime::{
    CheckerMintedForOfIterationBindingReadAnchorForCheckerOwnerV1,
    CheckerMintedForOfIterationBindingReadRecorderForCheckerOwnerV1,
    CheckerMintedForOfIterationBindingScanRolesForCheckerOwnerV1,
    ExecutableLocalSnapshotForRuntimeLoweringOwnerV1,
    ExecutableLocalSnapshotMintErrorForCheckerScanOwnerV1,
    ProviderBoundaryInputSnapshotForRuntimeLoweringOwnerV1,
    ProviderEffectRuntimeInstructionSiteBindingForRuntimeOperationOwnerV1,
    ProviderEffectRuntimeLoweringMaterializationErrorForRuntimeOperationOwnerV1,
    ProviderEffectRuntimeSelectedBindingValueForRuntimeActivityInputOwnerV1,
};

use checkpoint_host_resources::require_no_unmatched_host_resource_rebind_evidence;
pub(crate) use durability_policy::{
    ProcessSessionDurabilityPolicyAdmissionError, ProcessSessionDurabilityPolicyV0,
};
pub use interaction_runtime_domain::{
    InteractionCommandAuthority, InteractionViewAuthority, SessionRuntimeFrontier,
};
pub(crate) use interaction_runtime_domain::{
    InteractionRuntimeFrontierPreparationFault, ProtocolRuntimeFrontierPreparationFault,
    SessionRuntimeFrontierCheckpointLivenessProduct,
};
pub(crate) use machine::{
    HostFailureProjection, HostRunOutcomeProjection, MachineContinuation, MachineExecutionIdentity,
    MachineExecutionRecord, MachineHostBoundaryState, MachineHostProtocol,
    MachineImportBindingState, MachineNodeContext, MachineNodeSnapshot, MachineNodeState,
    MachineRunOutcome, MachineState,
};
pub(crate) use one_shot_boundary_events::{
    OneShotActorRequestContinuationAwaitingNodeAttachedEvent,
    OneShotActorRequestContinuationAwaitingNodeConsumedEvent,
    OneShotActorRequestContinuationAwaitingNodeConsumedEventSchema,
    OneShotActorRequestContinuationErrorAppliedEvent, OneShotBoundaryEvent,
    OneShotReturnCompletionEvent,
};
pub(crate) use one_shot_checkpoint_projection::{
    OneShotCallBoundaryProjection, OneShotCheckpointActorActiveTurnProjection,
    OneShotCheckpointActorKeyProjection, OneShotCheckpointActorLifecycleStateProjection,
    OneShotCheckpointActorRecordProjection, OneShotCheckpointActorStateShapeProjection,
    OneShotCheckpointActorStoreProjection, OneShotCheckpointRestoreError,
    OneShotCheckpointResumeError, OneShotContinuationState, OneShotRecordTranscriptProjection,
    OneShotSuspendedBoundaryProjection, OneShotSuspendedBoundaryProjectionError,
    clone_scoped_frame_lifecycle_state_without_host_resource_binding_authority,
};
pub(crate) use one_shot_failures::{
    RetainedCollectProjectionFailure, RetainedOutputProjectionFailure,
    RetainedOutputsMissingFailure, RetainedOutputsProjection,
};
use one_shot_faults::OneShotHostResultPhase;
pub(crate) use one_shot_faults::{
    MachineError, MachineFailureProjection, OneShotFault, ProgramFault, SemanticFailureContext,
    project_machine_error_failure_projection, project_one_shot_fault_failure_projection,
    project_one_shot_machine_fault, project_one_shot_program_fault,
};
pub(crate) use one_shot_host_interaction::{
    OneShotHostSuspensionPolicy, clone_boundary_object_without_host_resource_binding_authority,
};
pub(crate) use one_shot_machine_primitives::{
    OneShotExitOutputsProduct, one_shot_failed_record, one_shot_suspended_record,
};
use one_shot_metering::{
    OneShotComputeStepMeter, OneShotHostInteractionTurnMeter, OneShotHostResultMeteringDebitMeters,
};
pub(crate) use one_shot_metering::{
    OneShotMeteringBudget, OneShotMeteringBudgetGrantProjection, OneShotMeteringLedgerProjection,
};
pub(crate) use one_shot_projection_api::{
    OneShotExecutionRecord, OneShotMachineState, OneShotRecordCheckpointProjection,
    OneShotRecordProcessProjection, project_one_shot_run_outcome_status_projection,
};
pub(crate) use one_shot_run_api::{
    AdmittedOneShotExecutableObligationIdentityV1, AdmittedOneShotRunMachineParts,
    AdmittedOneShotRunV1, AdmittedOneShotRuntimeObligationLedgerAuthorityV1,
    OneShotOwnedImageExecutionAuthorityV1, OneShotRuntimeObligationLedgerClosedV1,
};
pub(crate) use one_shot_run_outcome::{
    OneShotRunOutcome, OneShotRunOutcomeProjection, OneShotRunOutcomeProjectionBodyForbidden,
};
pub(crate) use privileged_hostcalls::VmPrivilegedHostcallHost;
pub(crate) use privileged_hostcalls::actor_store::VmActorRequestContinuationErrorStatus;
pub use privileged_hostcalls::actor_store::{
    SameHostWakeSessionActorRouterEnvelopeEmissionProduct,
    SameHostWakeSessionBodyLineageForNativeTransportHandoffProduct,
    SameHostWakeSessionMeshTargetCorrespondenceProduct,
    SameHostWakeSessionRemoteActorDeliveryCancellation,
    SameHostWakeSessionRemoteActorDeliveryFault,
    SameHostWakeSessionRemoteActorDeliveryHandoff,
    SameHostWakeSessionRemoteActorDeliveryRefusal,
    SameHostWakeSessionRemoteActorDeliveryTeardownReceipt,
};
pub use privileged_hostcalls::{ActorTurnFault, ActorTurnReceipt, SelectedActorTurn};
pub(crate) use runtime_obligation_owner::{
    BoundRuntimeFunctionContractChecksV1, ExecutableRuntimeObligationCheckV1,
    RuntimeObligationLiveValueReadV1, evaluate_selected_h7_runtime_obligation_for_kernel_owner_v1,
};
pub use runtime_obligation_owner::{
    RuntimeFunctionContractCallableIdentityV1, RuntimeFunctionContractCheckPlanSetFaultV1,
    RuntimeFunctionContractCheckPlanSetV1, RuntimeObligationCheckEvaluationFaultV1,
    RuntimeObligationCheckExecutionFaultV1, RuntimeObligationCheckPlanValue,
    RuntimeObligationCheckPlanValueFault, RuntimeObligationH7ConstraintValue,
};
pub(crate) use session::ScopedFrameLifecycleState;
pub(crate) use session::SourceEntrypointExecutableRuntimeStaticChildTemplateConversionRefusalForDirectRunPreparedRuntimeOwnerV1;
pub use session::prepared_program::{
    PreparedStaticChildRuntimeTemplateForDirectRunOwnerV1, SealedPreparedRuntime,
};
pub(crate) use session::{
    DirectRunExactStaticChildDispatchInputBatchForPreparedRuntimeOwnerV1,
    DirectRunExactStaticChildDispatchInstallationRefusalForPreparedRuntimeOwnerV1,
    DirectRunExactStaticChildDispatchInstalledPreparedRuntimeForPreparedRuntimeOwnerV1,
};
pub(crate) use swarmvm_host_abi::machine_vocabulary::MachineEffectiveCapabilityStack;
use swarmvm_host_sql_authority::live_primitives::{
    LiveCancelScopeRef, LiveChannelRef, LivePrimitiveOwner, LivePrimitiveRuntime,
    LivePrimitiveRuntimeError, LiveWakeTargetRef,
};
pub(crate) use swarmvm_runtime_types::{
    ContinuationDiagnosticTransitionV1, continuation_diagnostic_event_sink_enabled_v1,
    continuation_diagnostic_recording_enabled_v1, record_continuation_diagnostic_event_v1,
};
pub(crate) use volatile_coroutine_frames::{
    VolatileCoroutineCapturedBinding, VolatileCoroutineFramePark, VolatileCoroutineFrameRecord,
    VolatileCoroutineFrameRef, VolatileCoroutineFrameRuntime, VolatileCoroutineFrameStatus,
};

pub fn project_actor_ready_work_selection_diagnostics_from_checkpoint_actor_state_v1(
    _actor_state: &OneShotCheckpointActorStoreProjection,
    _host_resource_rebind_evidence: &[OneShotHostResourceRebindEvidence],
    poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
) -> Result<serde_json::Value, OneShotCheckpointRestoreError> {
    match poison {}
}

pub fn project_actor_ready_work_selection_diagnostics_from_engine_checkpoint_v1(
    _checkpoint: &session::ProcessSessionCheckpointV0,
    _host_resource_rebind_evidence: &[OneShotHostResourceRebindEvidence],
    poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
) -> serde_json::Value {
    match poison {}
}

pub(crate) use crate::session::open_process_child_session_v0_from_sealed_prepared_runtime_for_direct_run_owner_v1;
pub use process_liveness::{
    ProcessLivenessCheckpointStateV1, ProcessLivenessDirectRunWaitStoreCancellationV1,
    ProcessLivenessDirectRunWaitStoreReceiptV1, ProcessLivenessDirectRunWaitStoreSettlementV1,
    ProcessLivenessDirectRunWaitStoreTeardownV1,
    ProcessLivenessHostedActorDischargeProductV1, ProcessLivenessLiveBlockerCreationContractV1,
    ProcessLivenessLiveBlockerV1, ProcessLivenessLiveBlockersV1,
    ProcessLivenessProcessControlChildPolicyV1, ProcessLivenessProcessControlFaultV1,
    ProcessLivenessProcessControlKindV1, ProcessLivenessProcessControlOperationV1,
    ProcessLivenessProcessControlReceiptV1, apply_process_liveness_direct_run_wait_store_v1,
    apply_process_liveness_process_control_operation_v1,
};
pub use process_tree::{
    PROCESS_TREE_HOST_RESOURCE_REBIND_REQUIREMENT_SCHEMA_V1, PROCESS_TREE_MEMBER_SCHEMA_V1,
    PROCESS_TREE_RESUME_PLAN_SCHEMA_V1, PROCESS_TREE_RESUME_RECEIPT_SCHEMA_V1,
};
pub(crate) use session::SelectedProviderResumeRouteForDirectRunOwnerV1;
pub(crate) use session::DirectRunProcessSessionWaitingOnLivenessStoreTransitionV1;
pub(crate) use session::open_process_session_v0_from_exact_static_child_dispatch_installed_prepared_runtime_for_direct_run_owner_v1;
pub use session::{
    AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1,
    AdmittedProcessLoadChildLaunchForDirectRunOwnerV1,
    AdmittedProcessRunChildLaunchForDirectRunOwnerV1,
    DirectRunProcessSessionCompletedTerminalMaterializationV1,
    DirectRunProcessSessionEmptySourceTerminalMaterializationFaultV1,
    DirectRunProcessSessionFailedTerminalMaterializationV1,
    DirectRunProcessSessionFailedTerminalObservationV1,
    DirectRunProcessSessionPublicApertureForbiddenBoundaryProductV1,
    DirectRunProcessSessionPublicApertureProgressProductV1,
    DirectRunProcessSessionRunResultProductV1,
    DirectRunProcessSessionRuntimeExactTerminalObservationV1,
    DirectRunProcessSessionRuntimeTerminalFaultObservationV1,
    DirectRunProcessSessionTerminalMaterializationV1,
    DirectRunProcessSessionTerminalResultConversionFaultV1,
    DirectRunProcessSessionTerminalResultProductV1,
    DirectRunProcessSessionWaitingOnLivenessProductV1,
    DirectRunRegisteredCaseTerminalObservationSetV1, DirectRunRegisteredCaseTerminalObservationV1,
    DirectRunRegisteredCaseTerminalSettlementContinuationV1, EVENT_TURN_LEDGER_V0_SCHEMA,
    EventTurnLedgerActivityEventV0, EventTurnLedgerAppendError, EventTurnLedgerReplayCursorV0,
    EventTurnLedgerReplayError, EventTurnLedgerSchema, EventTurnLedgerV0,
    ExecutableSessionImagePreparationFault, HostResourceFinalizationBoundaryFaultV1,
    HostResourceFinalizationDriveFaultV1,
    MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    PROCESS_SESSION_CHECKPOINT_V0_SCHEMA, ProcessBoundaryReadinessCertificateV1,
    ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    ProcessControlChildPolicyForDirectRunOwnerV1, ProcessControlCompletionForDirectRunOwnerV1,
    ProcessControlKindForDirectRunOwnerV1, ProcessControlResumeDriveFailureForDirectRunOwnerV1,
    ProcessControlResumeProductForDirectRunOwnerV1,
    ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1,
    ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1,
    ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1,
    ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
    ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    ProcessKernelBoundaryResumeCommitFaultV1, ProcessNominalProviderIngressCommitFaultV1,
    ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1,
    ProcessRunChildProviderIngressForDirectRunOwnerV1,
    ProcessRunChildProviderOutputForDirectRunOwnerV1,
    ProcessRunChildRegistrationForDirectRunOwnerV1,
    ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1,
    ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1,
    ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    ProcessSessionActivityResultBodyForbiddenV0, ProcessSessionCheckpointRestoreError,
    ProcessSessionCheckpointSchema, ProcessSessionCheckpointV0,
    ProcessSessionClassifiedFailureCauseV0,
    ProcessSessionCompletedTerminalOutputEffectSettlementFaultV1,
    ProcessSessionCompletedTerminalOutputEffectSettlementProductV1,
    ProcessSessionCompletedTerminalOutputEffectSettlementRefusalV1, ProcessSessionDriveFault,
    ProcessSessionEventTurnLedgerRecoveryError, ProcessSessionEventTurnLedgerRecoveryV0,
    ProcessSessionHostActivityEffectDriveReceiptV0, ProcessSessionInitialInputForDirectRunOwnerV1,
    ProcessSessionOpenError, ProcessSessionProcessIdentityForDirectRunLaunchOwnerV1,
    ProcessSessionPublicDiagnosticProjectionAuthority,
    ProcessSessionPublicDiagnosticProjectionValueForbiddenRequireDiagnosticProjectionAuthority,
    KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    ProcessSessionRegionLifecycleOperation, ProcessSessionRegionStateV0,
    ProcessSessionResultAdmissionBoundaryContextV0, ProcessSessionResultAdmissionBoundaryKindV0,
    ProcessSessionResumeError, ProcessSessionRunError, ProcessSessionV0,
    ProviderBoundaryExecutionCommitFault, ProviderBoundaryIngressFault,
    RegisteredCaseTerminalObservationFaultV1, RegisteredCaseTerminalSettlementProductV1,
    SealedProcessSessionDriveOutcome, SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessControlBoundaryForDirectRunOwnerV1,
    SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    SelectedProviderResumeHostInputForDirectRunOwnerV1, SourceEntrypointExecutableImage,
    SourceEntrypointExecutableRuntime,
    consume_source_entrypoint_executable_image_into_runtime_owner_v1,
    mint_process_invoke_execution_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_with_process_for_durable_direct_run_owner_v1,
};
use swarmscript_capability_registry::CanonicalPrivilegedHostcallInputContractFamily as PrivilegedHostcallInputContractFamily;
pub use swarmvm_host_abi::process_boundary::ProcessLifecyclePayloadCarrierForSessionRuntimeOwnerV1;
use swarmvm_host_abi::{MachineId, OneShotHost, OneShotHostResultSchema};
use swarmvm_image::{AdmittedOneShotImage, HandleKind, NodeId};
use swarmvm_isa_types::authority_ids::{ImportId, InstructionIndex, RegisterIndex};
use swarmvm_isa_types::{
    ManagedRegionExitCompletion, OutputProjection, ReturnBoundaryContract, ReturnCompletionKind,
};
use swarmvm_runtime_types::{VmBoundaryObjectValue, VmBoundaryUndefinedKind, VmBoundaryValue};
pub(crate) use swarmvm_runtime_types::{
    vm_runtime_logical_materialization_fact_enabled_v1, vm_runtime_logical_materialization_fact_v1,
    vm_runtime_memory_materialization_fact_v1, vm_runtime_profile_span_v1,
    vm_runtime_trap_breadcrumb_v1,
};
mod swarmvm_isa {
    pub(crate) use swarmvm_isa_types::authority_ids::{
        ActorRefId, ActorRequestId, ActorTurnId, DeliveryId, RequestEffectId,
    };
}

#[allow(dead_code)]
enum OneShotStepOutcome {
    Continue,
    Return {
        value: RuntimeValue,
        completion: Option<ReturnCompletionKind>,
        boundary: Option<ReturnBoundaryContract>,
    },
    Suspend(OneShotContinuationState),
    ActorSchedulerSuspended,
    Fail(OneShotFault),
}

#[allow(dead_code)]
impl OneShotStepOutcome {
    fn continue_step() -> Self {
        Self::Continue
    }

    fn return_value(value: RuntimeValue) -> Self {
        Self::Return {
            value,
            completion: None,
            boundary: None,
        }
    }

    fn return_value_with_completion(value: RuntimeValue, completion: ReturnCompletionKind) -> Self {
        Self::Return {
            value,
            completion: Some(completion),
            boundary: None,
        }
    }

    fn return_value_with_boundary(
        value: RuntimeValue,
        completion: ReturnCompletionKind,
        boundary: ReturnBoundaryContract,
    ) -> Self {
        Self::Return {
            value,
            completion: Some(completion),
            boundary: Some(boundary),
        }
    }

    fn suspend(continuation: OneShotContinuationState) -> Self {
        Self::Suspend(continuation)
    }

    fn actor_scheduler_suspended() -> Self {
        Self::ActorSchedulerSuspended
    }

    fn fail(fault: OneShotFault) -> Self {
        Self::Fail(fault)
    }

    fn from_fault(fault: OneShotFault) -> Self {
        Self::fail(fault)
    }

    fn from_return_result(result: Result<RuntimeValue, OneShotFault>) -> Self {
        match result {
            Ok(value) => Self::return_value(value),
            Err(fault) => Self::from_fault(fault),
        }
    }

    fn from_return_completion_result(
        result: Result<RuntimeValue, OneShotFault>,
        completion: ReturnCompletionKind,
    ) -> Self {
        match result {
            Ok(value) => Self::return_value_with_completion(value, completion),
            Err(fault) => Self::from_fault(fault),
        }
    }

    fn from_return_boundary_result(
        result: Result<RuntimeValue, OneShotFault>,
        completion: ReturnCompletionKind,
        boundary: ReturnBoundaryContract,
    ) -> Self {
        match result {
            Ok(value) => Self::return_value_with_boundary(value, completion, boundary),
            Err(fault) => Self::from_fault(fault),
        }
    }

    fn from_machine_error(source: MachineError) -> Self {
        Self::fail(project_one_shot_machine_fault(&source))
    }

    fn from_program_fault(source: ProgramFault) -> Self {
        Self::fail(project_one_shot_program_fault(&source))
    }

    fn from_failed_outcome(outcome: OneShotRunOutcome, context: &str) -> Self {
        Self::fail(outcome.failed_fault_or_integrity_fault(context))
    }

    fn apply_to_program_state(
        self,
        program_state: &mut MachineProgramState,
        instruction_index_before: InstructionIndex,
    ) -> Option<OneShotNodeExecutionOutcome> {
        match self {
            Self::Continue => {
                if program_state.instruction_index_for_swarmvm_session_runtime_owner_v1()
                    == instruction_index_before
                {
                    program_state.advance_for_swarmvm_session_runtime_owner_v1();
                }
                None
            }
            Self::Return {
                value,
                completion,
                boundary,
            } => Some(OneShotNodeExecutionOutcome::returned(
                value, completion, boundary,
            )),
            Self::Suspend(continuation) => {
                Some(OneShotNodeExecutionOutcome::suspended(continuation))
            }
            Self::ActorSchedulerSuspended => {
                Some(OneShotNodeExecutionOutcome::actor_scheduler_suspended())
            }
            Self::Fail(fault) => Some(OneShotNodeExecutionOutcome::failed(fault)),
        }
    }

    fn apply_to_resumed_program_state(
        self,
        _program_state: &mut MachineProgramState,
    ) -> Option<OneShotNodeExecutionOutcome> {
        match self {
            Self::Continue => None,
            Self::Return {
                value,
                completion,
                boundary,
            } => Some(OneShotNodeExecutionOutcome::returned(
                value, completion, boundary,
            )),
            Self::Suspend(continuation) => {
                Some(OneShotNodeExecutionOutcome::suspended(continuation))
            }
            Self::ActorSchedulerSuspended => {
                Some(OneShotNodeExecutionOutcome::actor_scheduler_suspended())
            }
            Self::Fail(fault) => Some(OneShotNodeExecutionOutcome::failed(fault)),
        }
    }
}

#[allow(dead_code)]
enum OneShotNodeExecutionOutcome {
    Returned {
        value: RuntimeValue,
        completion: Option<ReturnCompletionKind>,
        boundary: Option<ReturnBoundaryContract>,
    },
    Suspended(OneShotContinuationState),
    ActorSchedulerSuspended,
    Failed(OneShotFault),
}

#[allow(dead_code)]
impl OneShotNodeExecutionOutcome {
    fn returned(
        value: RuntimeValue,
        completion: Option<ReturnCompletionKind>,
        boundary: Option<ReturnBoundaryContract>,
    ) -> Self {
        Self::Returned {
            value,
            completion,
            boundary,
        }
    }

    fn suspended(continuation: OneShotContinuationState) -> Self {
        Self::Suspended(continuation)
    }

    fn actor_scheduler_suspended() -> Self {
        Self::ActorSchedulerSuspended
    }

    fn failed(fault: OneShotFault) -> Self {
        Self::Failed(fault)
    }

    fn missing_return() -> Self {
        Self::failed(project_one_shot_machine_fault(&MachineError::MissingReturn))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum OneShotNodeMachineApplyOutcome {
    Completed {
        completion: Option<ReturnCompletionKind>,
        boundary: Option<ReturnBoundaryContract>,
    },
    Suspended,
}

#[allow(dead_code)]
fn host_failure_projection_descriptor(projection: HostFailureProjection) -> VmBoundaryValue {
    let details_present = projection.details_present();
    swarmvm_runtime_types::host_failure_projection_observation_boundary_value_for_swarmvm_session_runtime_owner_v1(
        projection.kind().to_owned(),
        projection.code().to_owned(),
        projection.message().to_owned(),
        details_present,
    )
}

fn one_shot_checkpoint_resume_outcome_error(
    outcome: OneShotRunOutcome,
) -> OneShotCheckpointResumeError {
    OneShotCheckpointResumeError::Restore {
        source: OneShotCheckpointRestoreError::PrivilegedHostStateRestoreInvalid {
            code: "one_shot_checkpoint_resume_host_boundary_rejected".to_owned(),
            message: format!("{outcome:?}"),
        },
    }
}

#[allow(dead_code)]
struct OneShotProgramMachine<'a, H: OneShotHost> {
    image: &'a AdmittedOneShotImage,
    owned_image_execution_authority: OneShotOwnedImageExecutionAuthorityV1,
    host: &'a mut H,
    privileged_host: VmPrivilegedHostcallHost,
    execution_identity: MachineExecutionIdentity,
    params: VmBoundaryObjectValue,
    compute_step_meter: OneShotComputeStepMeter,
    host_interaction_turn_meter: OneShotHostInteractionTurnMeter,
    host_result_metering_debit_meters: OneShotHostResultMeteringDebitMeters,
    host_suspension_policy: OneShotHostSuspensionPolicy,
    boundary_events: Vec<OneShotBoundaryEvent>,
    machine_state: OneShotMachineState,
    node_param_values: BTreeMap<NodeId, VmBoundaryObjectValue>,
    operation_param_values: BTreeMap<String, VmBoundaryObjectValue>,
    runtime_heap: SessionRuntimeHeapOwner,
    scoped_frame_lifecycle: ScopedFrameLifecycleState,
}

#[allow(dead_code)]
impl<'a, H: OneShotHost> OneShotProgramMachine<'a, H> {
    fn from_admitted_run(
        _run: AdmittedOneShotRunV1<'a>,
        _host: &'a mut H,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        match poison {}
    }

    fn from_record_checkpoint(
        _image: &'a AdmittedOneShotImage,
        _host: &'a mut H,
        _checkpoint: &'a OneShotRecordCheckpointProjection,
        _metering_budget: OneShotMeteringBudget,
        _host_suspension_policy: OneShotHostSuspensionPolicy,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, OneShotCheckpointRestoreError> {
        match poison {}
    }

    fn from_record_checkpoint_with_host_resource_rebinds(
        _image: &'a AdmittedOneShotImage,
        _host: &'a mut H,
        _checkpoint: &'a OneShotRecordCheckpointProjection,
        _host_resource_rebind_evidence: &[OneShotHostResourceRebindEvidence],
        _metering_budget: OneShotMeteringBudget,
        _host_suspension_policy: OneShotHostSuspensionPolicy,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, OneShotCheckpointRestoreError> {
        match poison {}
    }

    fn fail(&self, source: MachineError) -> OneShotFault {
        project_one_shot_machine_fault(&source)
    }

    fn metering_ledger_projection(&self) -> OneShotMeteringLedgerProjection {
        OneShotMeteringLedgerProjection {
            compute_steps: self.compute_step_meter.ledger_projection(),
            host_interaction_turns: self.host_interaction_turn_meter.ledger_projection(),
            output_bytes: self
                .host_result_metering_debit_meters
                .output_byte_ledger_projection(),
            child_spawns: self
                .host_result_metering_debit_meters
                .child_spawn_ledger_projection(),
            durable_writes: self
                .host_result_metering_debit_meters
                .durable_write_ledger_projection(),
        }
    }

    fn metering_budget_grant_projection(&self) -> OneShotMeteringBudgetGrantProjection {
        OneShotMeteringBudgetGrantProjection {
            compute_steps: self.compute_step_meter.budget.grant_projection(),
            host_interaction_turns: self.host_interaction_turn_meter.budget.grant_projection(),
            output_bytes: self
                .host_result_metering_debit_meters
                .output_byte_budget_grant_projection(),
            child_spawns: self
                .host_result_metering_debit_meters
                .child_spawn_budget_grant_projection(),
            durable_writes: self
                .host_result_metering_debit_meters
                .durable_write_budget_grant_projection(),
        }
    }

    fn push_metering_projection(&mut self) {
        let metering_budget_grant = self.metering_budget_grant_projection();
        let metering_ledger = self.metering_ledger_projection();
        self.boundary_events
            .push(OneShotBoundaryEvent::MeteringBudgetGrant(
                metering_budget_grant,
            ));
        self.boundary_events
            .push(OneShotBoundaryEvent::MeteringLedger(metering_ledger));
    }

    fn drain_host_resource_stack(
        &mut self,
        reason: OneShotHostResourceFinalizationReason,
        completion: ManagedRegionExitCompletion,
    ) {
        self.boundary_events.extend(
            self.scoped_frame_lifecycle
                .drain_all_host_resource_finalization_obligations_for_root_owner_v1(
                    reason, completion,
                )
                .into_iter()
                .map(OneShotBoundaryEvent::HostResourceFinalized),
        );
    }

    fn push_transaction_frame_event(&mut self, event: OneShotTransactionFrameEvent) {
        self.boundary_events
            .push(OneShotBoundaryEvent::TransactionFrame(event));
    }

    fn drain_transaction_stack_as_rollback(&mut self) {
        for event in self
            .scoped_frame_lifecycle
            .drain_transaction_frame_rollback_events()
        {
            self.push_transaction_frame_event(event);
        }
    }

    fn actor_checkpoint_projection_fault(
        error: OneShotSuspendedBoundaryProjectionError,
        original_fault: Option<&OneShotFault>,
    ) -> OneShotFault {
        let original_fault = original_fault.map(|original_fault| {
            host_failure_projection_descriptor(project_one_shot_fault_failure_projection(
                original_fault,
            ))
        });
        let details = swarmvm_runtime_types::actor_checkpoint_projection_failed_details_boundary_value_for_swarmvm_session_runtime_owner_v1(
            error.to_string(),
            original_fault,
        );
        Self::actor_scheduler_fault(
            "actor_checkpoint_projection_failed",
            format!("actor checkpoint projection failed while producing execution record: {error}"),
            Some(details),
        )
    }

    fn into_actor_checkpoint_projection_failed_record(
        mut self,
        error: OneShotSuspendedBoundaryProjectionError,
        original_fault: Option<&OneShotFault>,
    ) -> OneShotExecutionRecord {
        self.drain_transaction_stack_as_rollback();
        self.drain_host_resource_stack(
            OneShotHostResourceFinalizationReason::ExecutionFailed,
            ManagedRegionExitCompletion::BODY_FAULT,
        );
        self.push_metering_projection();
        let runtime_heap = self.runtime_heap;
        one_shot_failed_record(
            self.boundary_events,
            self.machine_state,
            Self::actor_checkpoint_projection_fault(error, original_fault),
        )
        .with_runtime_heap(runtime_heap)
    }

    fn into_failed_record(mut self, fault: OneShotFault) -> OneShotExecutionRecord {
        self.drain_transaction_stack_as_rollback();
        self.drain_host_resource_stack(
            OneShotHostResourceFinalizationReason::ExecutionFailed,
            ManagedRegionExitCompletion::BODY_FAULT,
        );
        self.push_metering_projection();
        let runtime_heap = self.runtime_heap;
        one_shot_failed_record(self.boundary_events, self.machine_state, fault)
            .with_runtime_heap(runtime_heap)
    }

    fn actor_request_unanswered_error_value(request_id: &str) -> VmBoundaryValue {
        swarmvm_runtime_types::actor_request_unanswered_error_boundary_value_for_swarmvm_session_runtime_owner_v1(
            request_id.to_owned(),
        )
    }
    fn actor_scheduler_fault(
        code: &'static str,
        message: String,
        details: Option<VmBoundaryValue>,
    ) -> OneShotFault {
        let _ = details;
        project_one_shot_machine_fault(&MachineError::ActorSchedulerRequiresSealedOwnerProduct {
            operation: code,
            message,
        })
    }

    fn resume_record_checkpoint_with_host_result(
        self,
        _checkpoint: &OneShotRecordCheckpointProjection,
        _host_result: swarmvm_host_abi::OneShotHostResult,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<OneShotExecutionRecord, OneShotCheckpointResumeError> {
        let _ = self;
        match poison {}
    }

    fn resume_record_checkpoint_with_host_result_and_host_resource_rebinds(
        self,
        _checkpoint: &OneShotRecordCheckpointProjection,
        _host_result: swarmvm_host_abi::OneShotHostResult,
        _host_resource_rebind_evidence: &[OneShotHostResourceRebindEvidence],
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<OneShotExecutionRecord, OneShotCheckpointResumeError> {
        let _ = self;
        match poison {}
    }
}
