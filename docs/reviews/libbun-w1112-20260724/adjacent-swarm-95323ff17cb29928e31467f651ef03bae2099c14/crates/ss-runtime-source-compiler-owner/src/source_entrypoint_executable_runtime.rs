//! Compiler-owned source-entrypoint executable runtime SCC.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

use self::execution_kernel::executable_image;
pub(crate) use self::execution_kernel::executable_image::CompleteProcessPlanRegionCapabilityPreservationSetForSessionExecutableImageOwnerV1;
use self::execution_kernel::executable_image::{
    Instruction, SessionLirCapabilitySlotId, SessionLirCheckpointRegionV0, SessionLirDurableSiteId,
    SessionLirDurableSiteV0, SessionLirEntrypointId,
    SessionLirObservableEffectCoverageLedgerForSwarmvmSessionRuntimeOwnerV1,
    SessionLirObservableEffectRequiredOrderV0, SessionLirRegionId,
};
use self::work_runtime::EffectRef;
use regex::RegexBuilder;
use serde::{Serialize, Serializer};
use swarm_substrate_invariant::{
    ProjectionCargoForbiddenAtAuthorityBoundary, RawTransportValueForbiddenAtSemanticBoundary,
};
use swarmscript_capability_registry::{
    CanonicalPrivilegedHostcallInputContractFamily as PrivilegedHostcallInputContractFamily,
    privileged_hostcall_surface_input_contract_family,
};
use swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_CURRENT_PROCESS_FIELD;
use swarmvm_host_abi::{
    AdmittedOneShotHostResult, CapabilityIdentity, EffectiveCapabilitySet, HostBridgeError,
    MachineId, OneShotHost, OneShotHostCallEnvelope, OneShotHostResult, OneShotHostResumeEnvelope,
    machine_vocabulary::MachineEffectiveCapabilityStack,
};
use swarmvm_image::{AdmittedOneShotImportBinding, NodeId, OperationId, VmImportBinding};
use swarmvm_isa_types::authority_ids::{
    ActivityAttemptId, ActorRefId, ActorRequestId, ActorTurnId, DeliveryId, HostResourceHandleId,
    ImportId, InstructionIndex, InstructionOpId, RegisterIndex,
};
use swarmvm_isa_types::{
    ActorHandlerCompletionMode, EventWaitLocalApplyPolicy, EventWaitLocalInstructionPolicy,
    EventWaitLocalPlanKind, EventWaitLocalSemanticClass, HostActivityResultMode, InstructionOpcode,
    ManagedRegionExitCompletion, RetainedDependencyContract, ReturnBoundaryContract,
};
use swarmvm_runtime_types::{
    ModuleExportMemberRefValue, ModuleExportRefValue, NodeIdValue, SemanticTypeRefValue,
    VmBoundaryObjectValue, VmBoundaryValue,
};
pub use swarmvm_session_runtime_model::{
    ProcessSessionActorRequestOwnerContextV0, ProcessSessionEventWaitProducerBoundaryKindV0,
    ProcessSessionEventWaitProducerReachabilityProofV0,
    ProcessSessionEventWaitProducerSettlementModeV0,
};
pub use swarmvm_session_runtime_model::{
    ProcessSessionClassifiedFailureCauseV0, ProcessSessionPublicDiagnosticProjectionAuthority,
    ProcessSessionPublicDiagnosticProjectionValueForbiddenRequireDiagnosticProjectionAuthority,
    ProcessSessionResultAdmissionBoundaryContextV0, ProcessSessionResultAdmissionBoundaryKindV0,
};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

mod swarmvm_isa {
    pub(super) use swarmvm_isa_types::authority_ids::{
        ActivityAttemptId, ActorRequestId, ActorTurnId, DeliveryId, InstructionOpId,
        OperationHandleId, PlanNodeId, SpliceId, StreamHandleId, TransactionHandleId,
    };
    pub(super) use swarmvm_isa_types::{
        HostActivityKind, InstructionExecutionClass, ObjectEntry, OutputProjection,
        ReturnBoundaryContract, ReturnCompletionKind, SourceReturnTerminalEdge,
    };
    pub(super) use swarmvm_runtime_types::{
        OPERATION_MEMBER_TARGET_SCHEMA, OperationHandleValue, StreamHandleValue,
        TransactionHandleValue, VmBoundaryOperationHandleValue, VmBoundaryStreamHandleValue,
    };
}

use crate::VmRuntimeHeapGraphCheckpointV0;
use crate::privileged_hostcalls::actor_store::{
    ActorRequestScopedActiveTurnAdmission, ActorRequestScopedAwaitedChildAdmission,
    ActorRequestScopedReadyAwaiterRestoreAdmission,
};
use crate::privileged_hostcalls::{
    ActorTurnReceipt, ProcessAuthority, VmActorParentReplyDisposition,
    VmActorTurnCompletionPrepared, VmReadyActorRequestAwaitingNode,
};
use crate::{
    AdmittedOneShotExecutableObligationIdentityV1,
    AdmittedOneShotRuntimeObligationLedgerAuthorityV1, HostResourceHandleValue,
    HostResourceResumePolicy, LiveChannelRef, LivePrimitiveOwner, LivePrimitiveRuntime,
    OneShotActorRequestContinuationAwaitingNodeConsumedEvent,
    OneShotActorRequestContinuationAwaitingNodeConsumedEventSchema, OneShotBoundaryEvent,
    OneShotCheckpointActorStoreProjection, OneShotHostResourceFinalizationObligation,
    OneShotHostResourceFinalizationReason, OneShotHostResourceRebindEvidence,
    OneShotHostResourceRebindRequirement, OneShotSuspendedBoundaryProjectionError,
    ProcessSessionDurabilityPolicyAdmissionError, ProcessSessionDurabilityPolicyV0, RuntimeValue,
    ScopedHostResourceFrameLifecycle, SessionRuntimeHeapOwner,
    VmActorRequestContinuationErrorStatus, VmPrivilegedHostcallHost, VmRegisterSnapshot,
    VolatileCoroutineCapturedBinding, VolatileCoroutineFramePark, VolatileCoroutineFrameRef,
    VolatileCoroutineFrameRuntime,
};
#[path = "source_entrypoint_executable_runtime/compiler_owned_callable_store.rs"]
mod compiler_owned_callable_store;
#[cfg(test)]
pub(crate) use compiler_owned_callable_store::empty_compiler_owned_callable_store_and_scope_for_test_owner_v1;
pub(crate) use compiler_owned_callable_store::{
    CompilerOwnedOrdinaryCallableStoreForSessionRuntimeOwnerV1,
    ModuleDeclarationRuntimeBindingInstallerForSessionRuntimeOwnerV1,
};
#[path = "source_entrypoint_executable_runtime/exact_capability_scope.rs"]
mod exact_capability_scope;
#[path = "source_entrypoint_executable_runtime/execution_state.rs"]
mod execution_state;
pub(crate) use exact_capability_scope::{
    AdmittedExactStaticChildCapabilityScopeForChildSessionOpenOwnerV1,
    AdmittedProcessInvokeExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    AdmittedProcessLoadExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    AdmittedProcessRunExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    CapturedExactCapabilityScopeForActorStartOwnerV1,
    ExactStaticChildCapabilityScopeAdmissionFaultForSessionWorkRuntimeOwnerV1,
    ProcessInvokeExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    ProcessLoadExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    ProcessRunExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    SelectedCurrentExactCapabilityScopeForExactStaticChildOwnerV1,
};
pub(in crate::session) use exact_capability_scope::{
    CapturedExactCapabilityScopeForInvocationOwnerV1, CurrentExactCapabilityScopeAuthority,
    CurrentExactCapabilityScopePopError,
    EffectiveCapabilityDispatchAdmissionFaultForSwarmvmSessionRuntimeOwnerV1,
    EffectiveCapabilityIdentityExcludedFromCurrentFrameForSwarmvmSessionRuntimeOwnerV1,
    EnteredActorExactCapabilityScopeExchangeForActorRuntimeOwnerV1,
    EnteredInvocationExactCapabilityScopeExchangeForCallableRuntimeOwnerV1,
    ProcessSessionExactCapabilityScopeOpenSeedForSessionRuntimeOwnerV1,
    SealedExactCapabilityPresenceExecutionForSessionScopeOwnerV1,
    SelectedImageBoundExactCapabilityLexicalHeaderEnterForSessionScopeOwnerV1,
    SelectedImageBoundExactCapabilityLexicalHeaderRestoreForSessionScopeOwnerV1,
    SelectedImageBoundExactCapabilityPresenceBranchForSessionScopeOwnerV1,
    SettledActorExactCapabilityScopeExchangeForActorRuntimeOwnerV1,
    SuspendedExactCapabilityScopeForContinuationOwnerV1,
};

#[cfg(test)]
pub(crate) fn captured_empty_exact_capability_scope_for_actor_store_test_owner_v1()
-> CapturedExactCapabilityScopeForActorStartOwnerV1 {
    CurrentExactCapabilityScopeAuthority::open_root_for_session_scope_owner_v1(Vec::new())
        .capture_for_actor_start_scope_owner_v1()
}
pub use execution_kernel::executable_value::{
    MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
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
    SelectedProcessControlBoundaryForDirectRunOwnerV1,
    SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    mint_process_invoke_execution_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_for_durable_direct_run_owner_v1,
    mint_process_run_child_carrier_with_process_for_durable_direct_run_owner_v1,
};
pub use execution_state::{
    DirectRunRegisteredCaseTerminalObservationSetV1, DirectRunRegisteredCaseTerminalObservationV1,
    RegisteredCaseTerminalObservationFaultV1, RegisteredCaseTerminalSettlementProductV1,
};
#[path = "source_entrypoint_executable_runtime/runtime_owned_activity.rs"]
mod runtime_owned_activity;
#[path = "source_entrypoint_executable_runtime/scoped_frame_lifecycle.rs"]
mod scoped_frame_lifecycle;
#[path = "source_entrypoint_executable_runtime/work_runtime/mod.rs"]
mod work_runtime;
use runtime_owned_activity::{
    ProcessSessionActivityEffectDescriptorV0, ProcessSessionEventWaitParkedActivityFrameV0,
    ProcessSessionEventWaitParkedActivityIdentityV0,
    ProcessSessionPublicApertureCompactActivityDescriptorV0,
};
pub(crate) use scoped_frame_lifecycle::{
    CompleteScopedResourcePreservationSetForSessionRuntimeOwnerV1, ScopedFrameLifecycleState,
};
pub(crate) use work_runtime::CheckpointManifest;
pub use work_runtime::PayloadHandle;
pub(crate) use work_runtime::PendingActivityEffectFrame;
pub(crate) use work_runtime::SelectedProviderResumeRouteForDirectRunOwnerV1;
pub(crate) use work_runtime::WorkRuntimeStores;
pub(crate) use work_runtime::{ActorCheckpointBodyPayloadProduct, WorkHandle};
pub use work_runtime::{
    AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1,
    AdmittedProcessLoadChildLaunchForDirectRunOwnerV1,
    AdmittedProcessRunChildLaunchForDirectRunOwnerV1,
    KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    SelectedProviderResumeHostInputForDirectRunOwnerV1,
};
pub(crate) use work_runtime::{
    ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1,
    ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1,
    ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1,
    SelectedProcessRestoreExecutionInputForDirectRunOwnerV1,
};

pub const PROCESS_SESSION_CHECKPOINT_V0_SCHEMA: &str = "swarm.vm.process_session_checkpoint.v0";
pub const EVENT_TURN_LEDGER_V0_SCHEMA: &str = "swarm.vm.event_turn_ledger.v0";

struct ProcessSessionActivityFallbackHost;

struct ProcessSessionActorHandlerTerminal {
    prepared_completion: VmActorTurnCompletionPrepared,
    projection: VmBoundaryValue,
}

enum ProcessSessionActorHandlerCompletion {
    Completed {
        prepared_completion: VmActorTurnCompletionPrepared,
    },
    Rejected {
        reason: &'static str,
        error: VmBoundaryValue,
        rejected_result: VmBoundaryValue,
    },
}

#[derive(Debug, PartialEq, Eq)]
#[must_use = "actor request checkpoint cancellation authority must be consumed at the checkpoint cancellation boundary"]
pub(crate) struct ProcessSessionActorRequestCheckpointCancellationAuthority {
    _private: (),
}

impl ProcessSessionActorRequestCheckpointCancellationAuthority {
    pub(crate) fn actor_request_cancel_at_checkpoint_input_v1() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[must_use = "pending activity checkpoint alignment authority must be consumed at the checkpoint alignment boundary"]
pub(crate) struct ProcessSessionPendingActivityCheckpointAlignmentAuthority {
    _private: (),
}

impl ProcessSessionPendingActivityCheckpointAlignmentAuthority {
    pub(crate) fn align_pending_activity_for_owned_live_checkpoint_projection_transition() -> Self {
        Self { _private: () }
    }
}

enum ProcessSessionActiveActorTurnDriveOutcome {
    Progress,
    RunnableRegionReady,
}

enum ProcessSessionActorTurnCompletionOutcome {
    CompletedTurn,
    FailedTurn,
}

struct ProcessSessionActorTurnCompletionReceipt {
    outcome: ProcessSessionActorTurnCompletionOutcome,
    parent_request_id: Option<ActorRequestId>,
}

enum ProcessSessionActorSchedulerQuiescenceOutcome {
    Quiescent { progressed: bool },
    RunnableRegionReady,
    Pending(Box<ProcessSessionRunOutcomeV0>),
}

enum ProcessSessionTerminalContinuationOutcome {
    Outcome(Box<ProcessSessionRunOutcomeV0>),
    Continue,
}

enum ProcessSessionTerminalInstructionBoundaryOutcomeV0 {
    Continue,
    Outcome(ProcessSessionRunOutcomeV0),
    Terminal(ProcessSessionVmTerminalProductV0),
}

pub(in crate::session) fn process_session_run_outcome_kind(
    outcome: &ProcessSessionRunOutcomeV0,
) -> &'static str {
    match outcome {
        ProcessSessionRunOutcomeV0::NeedsHostActivityEffect { .. } => {
            "blocked_needs_host_activity_effect"
        }
        ProcessSessionRunOutcomeV0::NeedsHostResourceFinalization { .. } => {
            "blocked_needs_host_resource_finalization"
        }
        ProcessSessionRunOutcomeV0::WaitingOnLiveness { .. } => "blocked_waiting_on_liveness",
        ProcessSessionRunOutcomeV0::Completed { .. } => "terminal_completed",
        ProcessSessionRunOutcomeV0::Failed { .. } => "terminal_failed",
    }
}

impl OneShotHost for ProcessSessionActivityFallbackHost {
    fn call(&mut self, _: OneShotHostCallEnvelope) -> Result<OneShotHostResult, HostBridgeError> {
        Err(HostBridgeError::Failure {
            code: None,
            message: "process session activity host fallback should not be called for privileged hostcall"
                .to_string(),
            details: None,
        })
    }

    fn resume(
        &mut self,
        _: OneShotHostResumeEnvelope,
    ) -> Result<OneShotHostResult, HostBridgeError> {
        Err(HostBridgeError::ResumeNotSupported)
    }
}

enum ProcessSessionPrivilegedHostcallOutcome {
    Ready {
        value: VmBoundaryValue,
        post_ready_cancelled_turn_ids: Vec<swarmvm_isa::ActorTurnId>,
    },
    ActorReplyCompletedWithoutRegisterWrite,
    Failed {
        code: String,
        message: String,
        details: Option<VmBoundaryValue>,
    },
}

enum ProcessSessionActorRequestEffectResultOutcome {
    Continued,
    RunnableRegionReady,
    Pending(Box<ProcessSessionRunOutcomeV0>),
    ObservationRequired {
        request_id: ActorRequestId,
        dst: RegisterIndex,
    },
    ReadyToObserve {
        request_id: ActorRequestId,
        dst: RegisterIndex,
        quiescence_proof: scheduler::ProcessSessionSchedulerQuiescenceProof,
    },
}

#[derive(Debug)]
pub(crate) struct ProcessSessionCompactLivenessWaitV0 {
    blocker_count: usize,
}

#[derive(Debug, Serialize)]
pub(crate) struct CompactProgressFaultEvidenceV0 {
    recent_key_count: u8,
    last_state_key: Option<String>,
    last_frontier_key: Option<String>,
    tail_0: Option<String>,
    tail_1: Option<String>,
    tail_2: Option<String>,
    tail_3: Option<String>,
    tail_4: Option<String>,
    tail_5: Option<String>,
    tail_6: Option<String>,
    tail_7: Option<String>,
}

fn compact_progress_fault_scalar_key(value: &str) -> String {
    const COMPACT_PROGRESS_FAULT_KEY_CHAR_LIMIT: usize = 160;

    value
        .chars()
        .take(COMPACT_PROGRESS_FAULT_KEY_CHAR_LIMIT)
        .collect()
}

pub(crate) fn compact_progress_fault_scalar_evidence(
    recent_keys: &[String],
    last_state_key: Option<&str>,
    last_frontier_key: Option<&str>,
) -> CompactProgressFaultEvidenceV0 {
    const COMPACT_PROGRESS_FAULT_TAIL_LIMIT: usize = 8;

    let mut tail = recent_keys
        .iter()
        .rev()
        .take(COMPACT_PROGRESS_FAULT_TAIL_LIMIT)
        .map(|key| compact_progress_fault_scalar_key(key))
        .collect::<Vec<_>>();
    tail.reverse();

    let mut tail = tail.into_iter();
    CompactProgressFaultEvidenceV0 {
        recent_key_count: recent_keys.len().min(u8::MAX as usize) as u8,
        last_state_key: last_state_key.map(compact_progress_fault_scalar_key),
        last_frontier_key: last_frontier_key.map(compact_progress_fault_scalar_key),
        tail_0: tail.next(),
        tail_1: tail.next(),
        tail_2: tail.next(),
        tail_3: tail.next(),
        tail_4: tail.next(),
        tail_5: tail.next(),
        tail_6: tail.next(),
        tail_7: tail.next(),
    }
}

#[derive(Debug)]
pub(in crate::session) enum ProcessSessionExternalActorRequestReadinessOutcomeV0 {
    NeedsHostActivityEffect {
        activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
    },
    WaitingOnLiveness {
        compact_wait: ProcessSessionCompactLivenessWaitV0,
    },
    EventWaitProducerProgressApplied {
        reason: &'static str,
    },
    TerminalFault {
        reason: &'static str,
        diagnostics: BTreeMap<String, serde_json::Value>,
    },
    ProgressFault {
        reason: &'static str,
        evidence: CompactProgressFaultEvidenceV0,
    },
}

#[derive(Debug)]
pub(in crate::session) enum ProcessSessionExternalActorRequestReadinessApplicationV0 {
    Pending {
        outcome_kind: &'static str,
    },
    TerminalFault {
        diagnostics: BTreeMap<String, serde_json::Value>,
    },
    ProgressFault {
        diagnostics: serde_json::Value,
    },
}

impl ProcessSessionExternalActorRequestReadinessOutcomeV0 {
    pub(in crate::session) fn outcome_kind(&self) -> &'static str {
        match self {
            Self::NeedsHostActivityEffect { .. } => "needs_host_activity_effect",
            Self::WaitingOnLiveness { .. } => "waiting_on_liveness",
            Self::EventWaitProducerProgressApplied { .. } => "event_wait_producer_progress_applied",
            Self::TerminalFault { .. } => "terminal_fault",
            Self::ProgressFault { .. } => "progress_fault",
        }
    }

    pub(in crate::session) fn into_application_for_external_ingress_owner_v1(
        self,
    ) -> ProcessSessionExternalActorRequestReadinessApplicationV0 {
        match self {
            Self::NeedsHostActivityEffect { .. } => {
                ProcessSessionExternalActorRequestReadinessApplicationV0::Pending {
                    outcome_kind: "needs_host_activity_effect",
                }
            }
            Self::WaitingOnLiveness { .. } => {
                ProcessSessionExternalActorRequestReadinessApplicationV0::Pending {
                    outcome_kind: "waiting_on_liveness",
                }
            }
            Self::EventWaitProducerProgressApplied { .. } => {
                ProcessSessionExternalActorRequestReadinessApplicationV0::Pending {
                    outcome_kind: "event_wait_producer_progress_applied",
                }
            }
            Self::TerminalFault { diagnostics, .. } => {
                ProcessSessionExternalActorRequestReadinessApplicationV0::TerminalFault {
                    diagnostics,
                }
            }
            Self::ProgressFault { reason, evidence } => {
                ProcessSessionExternalActorRequestReadinessApplicationV0::ProgressFault {
                    diagnostics: serde_json::json!({
                        "reason": reason,
                        "evidence": evidence,
                    }),
                }
            }
        }
    }
}

enum ProcessSessionCompactExternalActorRequestRegionOutcomeV0 {
    VerifiedCompactInternalTransition {
        proof: CompactContinueProof,
    },
    NeedsHostActivityEffect {
        activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
    },
    WaitingOnLiveness {
        compact_wait: ProcessSessionCompactLivenessWaitV0,
    },
    ProgressFault {
        reason: &'static str,
        evidence: CompactProgressFaultEvidenceV0,
    },
    Completed {
        value: Box<RuntimeValue>,
    },
    TerminalFault {
        reason: &'static str,
        diagnostics: BTreeMap<String, serde_json::Value>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventWaitProducerProviderReachabilityReceiptSourceKind {
    DirectPublicApertureProviderBoundary,
    ActorRequestObservationPublicAperture,
    PublicApertureSchedulerSelectedProducerActivity,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ProcessSessionIntrinsicWaitContinuationAppliedReceiptV1 {
    effect_ref_summary: String,
    ready_payload_shape: String,
}

impl ProcessSessionIntrinsicWaitContinuationAppliedReceiptV1 {
    pub(crate) fn from_session_runtime_intrinsic_wait_owner_v1(
        effect_ref: &EffectRef,
        wait_frame: &PendingActivityEffectFrame,
        ready_payload_shape: String,
    ) -> Self {
        let _ = (effect_ref, wait_frame);
        Self {
            effect_ref_summary: "sealed_effect_ref".to_owned(),
            ready_payload_shape,
        }
    }

    pub(crate) fn effect_id(&self) -> &str {
        self.effect_ref_summary.as_str()
    }

    pub(crate) fn diagnostic_value(&self) -> serde_json::Value {
        serde_json::json!({
            "kind": "intrinsic_wait_effect_continuation_await_result_applied",
            "reason": "ADR-2204 intrinsic wait consumed the owned AwaitExecution continuation and applied the ready result before liveness admission",
            "effect_ref": self.effect_ref_summary,
            "wait_frame_coordinates": "redacted",
            "ready_payload_shape": self.ready_payload_shape,
            "await_result_application": {
                "kind": "owned_await_continuation_result_application",
                "wait_frame_coordinates": "redacted",
                "effect_ref": self.effect_ref_summary,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ProcessSessionIntrinsicWaitContinuationSealV1 {
    region_id: String,
    sealed_instruction_index: u64,
    sealed_next_instruction_index: u64,
}

impl ProcessSessionIntrinsicWaitContinuationSealV1 {
    pub(crate) fn from_session_runtime_intrinsic_wait_owner_v1(
        region_id: &SessionLirRegionId,
        sealed_instruction_index: u64,
        sealed_next_instruction_index: u64,
    ) -> Self {
        Self {
            region_id: region_id.to_string(),
            sealed_instruction_index,
            sealed_next_instruction_index,
        }
    }

    fn diagnostic_value(&self) -> serde_json::Value {
        serde_json::json!({
            "schema": "swarm.vm.intrinsic_wait_continuation_seal.v1",
            "kind": "intrinsic_wait_effect_continuation_seal",
            "contract": "suspend_current_continuation_until_liveness_boundary",
            "region_id": self.region_id.as_str(),
            "sealed_instruction_index": self.sealed_instruction_index,
            "sealed_next_instruction_index": self.sealed_next_instruction_index,
        })
    }

    fn region_id(&self) -> &str {
        self.region_id.as_str()
    }

    fn sealed_instruction_index(&self) -> u64 {
        self.sealed_instruction_index
    }

    fn sealed_next_instruction_index(&self) -> u64 {
        self.sealed_next_instruction_index
    }
}

impl EventWaitProducerProviderReachabilityReceiptSourceKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::DirectPublicApertureProviderBoundary => {
                "direct_public_aperture_provider_boundary"
            }
            Self::ActorRequestObservationPublicAperture => {
                "event_wait_actor_request_observation_public_aperture"
            }
            Self::PublicApertureSchedulerSelectedProducerActivity => {
                "event_wait_public_aperture_scheduler_selected_producer_activity"
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EventWaitProducerActorRequestObservationPlan {
    parked_waiter: ProcessSessionEventWaitParkedActivityIdentityV0,
    observed_actor_request_id: ActorRequestId,
    dst: RegisterIndex,
}

#[derive(Debug, PartialEq, Eq)]
struct EventWaitProducerProviderReachabilityReceipt {
    proof_kind: ProcessSessionEventWaitProducerReachabilityProofV0,
    source_kind: EventWaitProducerProviderReachabilityReceiptSourceKind,
    parked_waiter: ProcessSessionEventWaitParkedActivityIdentityV0,
    observed_actor_request_id: Option<ActorRequestId>,
    selected_activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
}

impl EventWaitProducerProviderReachabilityReceipt {
    fn diagnostic_value(&self) -> serde_json::Value {
        serde_json::json!({
            "proof_kind": format!("{:?}", self.proof_kind),
            "source_kind": self.source_kind.as_str(),
            "parked_waiter": self.parked_waiter.diagnostic_value(),
            "observed_actor_request_id": self
                .observed_actor_request_id
                .as_ref()
                .map(|request_id| request_id.as_str()),
            "selected_activity_effect_descriptor": "sealed",
        })
    }
}

#[allow(dead_code)]
enum ProcessSessionEventWaitProducerPublicApertureInstructionOutcomeV0 {
    InternalProgress {
        proof: EventWaitPublicApertureProgressProof,
    },
    ProviderActivityEffectDescriptor {
        activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
        reachability: EventWaitProducerProviderReachabilityReceipt,
    },
    ObservationStaged {
        reason: &'static str,
        proof: EventWaitPublicApertureProgressProof,
        diagnostics: serde_json::Value,
    },
    TypedFault {
        reason: &'static str,
        diagnostics: serde_json::Value,
    },
}

impl ProcessSessionEventWaitProducerPublicApertureInstructionOutcomeV0 {
    fn internal_progress(receipt: EventWaitPublicApertureProgressReceipt) -> Self {
        match EventWaitPublicApertureProgressProof::admit(receipt) {
            Ok(proof) => Self::InternalProgress { proof },
            Err(error) => Self::TypedFault {
                reason: error.reason,
                diagnostics: error.diagnostics,
            },
        }
    }

    fn observation_staged(
        reason: &'static str,
        receipt: EventWaitPublicApertureProgressReceipt,
        diagnostics: serde_json::Value,
    ) -> Self {
        match EventWaitPublicApertureProgressProof::admit(receipt) {
            Ok(proof) => Self::ObservationStaged {
                reason,
                proof,
                diagnostics,
            },
            Err(error) => Self::TypedFault {
                reason: error.reason,
                diagnostics: error.diagnostics,
            },
        }
    }
}

enum EventWaitObservationStepOutcome {
    NoMatch,
    Progress(EventWaitPublicApertureProgressReceipt),
    ProviderBoundary {
        activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
        reachability: EventWaitProducerProviderReachabilityReceipt,
    },
    Fault {
        reason: &'static str,
        diagnostics: serde_json::Value,
    },
}

#[derive(Clone, Debug, Default)]
pub(in crate::session) struct CompactActorContinuationDiagnosticSnapshot {
    projection_facts: BTreeMap<String, serde_json::Value>,
}

impl CompactActorContinuationDiagnosticSnapshot {
    pub(in crate::session) fn projection_only(
        projection_facts: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        Self { projection_facts }
    }

    pub(in crate::session) fn clone_projection_facts(&self) -> BTreeMap<String, serde_json::Value> {
        self.projection_facts.clone()
    }

    pub(in crate::session) fn into_projection_facts(self) -> BTreeMap<String, serde_json::Value> {
        self.projection_facts
    }

    pub(in crate::session) fn projection_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.projection_facts).unwrap_or(serde_json::Value::Null)
    }
}

enum ProcessSessionCompactExternalActorRequestReadinessStepV0 {
    Outcome {
        outcome: ProcessSessionExternalActorRequestReadinessOutcomeV0,
    },
    ParentHandlerBlockedOnChildActorRequest {
        waiting_request_id: ActorRequestId,
        child_admission: ActorRequestScopedAwaitedChildAdmission,
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    ParentContextRestoredAfterTurnCompletion {
        parent_request_id: Option<ActorRequestId>,
    },
    VerifiedSchedulerTransition {
        proof: CompactContinueProof,
    },
}

pub(in crate::session) enum CompactChildDriveOutcome {
    ParentContinuationRestored {
        restored_request_id: Option<ActorRequestId>,
    },
    ChildNeedsHostActivityEffect {
        activity_effect_descriptor: ProcessSessionActivityEffectDescriptorV0,
    },
    ChildWaitingOnLiveness {
        compact_wait: ProcessSessionCompactLivenessWaitV0,
    },
    ChildTerminalFault {
        reason: &'static str,
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    ChildProgressFault {
        reason: &'static str,
        evidence: CompactProgressFaultEvidenceV0,
    },
}

pub(in crate::session) struct CompactContinueProof {
    reason: &'static str,
}

impl CompactContinueProof {
    pub(in crate::session) fn pc_or_region_advanced(reason: &'static str) -> Self {
        Self { reason }
    }

    pub(in crate::session) fn frame_stack_changed(reason: &'static str) -> Self {
        Self { reason }
    }

    pub(in crate::session) fn reason(&self) -> &'static str {
        self.reason
    }
}

struct ActorOwnedAwaiterRestoredContinuationV0 {
    request_id: ActorRequestId,
    parent_request_id: Option<ActorRequestId>,
    suspended_turn_id: ActorTurnId,
    awaiting_region_id: SessionLirRegionId,
    next_instruction_index: InstructionIndex,
    result_dst: RegisterIndex,
    restore_admission: ActorRequestScopedReadyAwaiterRestoreAdmission,
}

impl ActorOwnedAwaiterRestoredContinuationV0 {
    fn scalar_progress_key(&self) -> String {
        format!(
            "request_id={} parent_request_id={} suspended_turn_id={} awaiting_region_id={} next_instruction_index=sealed result_dst=sealed restore_stack_depth={} awaiting_node_id={}",
            self.request_id,
            self.parent_request_id
                .as_ref()
                .map(|request_id| request_id.as_str())
                .unwrap_or("none"),
            self.suspended_turn_id,
            self.awaiting_region_id,
            self.restore_admission.stack_depth(),
            self.restore_admission.awaiting_node_id()
        )
    }
}

enum CompactActorOwnedAwaiterRestoreOutcomeV0 {
    Restored {
        continuation: ActorOwnedAwaiterRestoredContinuationV0,
    },
    NoReadyAwaiterVisible,
    NoPrivilegedHostCache {
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    CurrentHandlerAwaiterExistsButNotReady {
        waiting_request_id: ActorRequestId,
        child_admission: ActorRequestScopedAwaitedChildAdmission,
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    ObservedRequestNotReadyForCurrentHandlerAwaiter {
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    RequestScopedAuthorityFault {
        reason: &'static str,
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    CurrentHandlerReadyAwaiterLookupMiss {
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
    SnapshotContractMismatch {
        reason: &'static str,
        diagnostics: CompactActorContinuationDiagnosticSnapshot,
    },
}

pub(in crate::session) enum CurrentHandlerAwaiterNotReadyPolicy {
    BlockParentDispatch,
    YieldToSchedulerSelectedWork {
        admission: ActorRequestScopedActiveTurnAdmission,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ProcessSessionObservableEffectBranchKindV0 {
    BranchOnBoolean,
    BranchOnResult(RawTransportValueForbiddenAtSemanticBoundary),
    BranchOnCapabilityPresence(RawTransportValueForbiddenAtSemanticBoundary),
}

impl Serialize for ProcessSessionObservableEffectBranchKindV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BranchOnBoolean => serializer.serialize_str("branch_on_boolean"),
            Self::BranchOnResult(poison) | Self::BranchOnCapabilityPresence(poison) => {
                match *poison {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ProcessSessionObservableEffectBranchEdgeV0 {
    BooleanTrueTarget,
    BooleanFalseTarget,
    TryOkTarget(RawTransportValueForbiddenAtSemanticBoundary),
    TryErrTarget(RawTransportValueForbiddenAtSemanticBoundary),
    CapabilityPresentTarget(RawTransportValueForbiddenAtSemanticBoundary),
    CapabilityAbsentTarget(RawTransportValueForbiddenAtSemanticBoundary),
}

impl Serialize for ProcessSessionObservableEffectBranchEdgeV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BooleanTrueTarget => serializer.serialize_str("boolean_true_target"),
            Self::BooleanFalseTarget => serializer.serialize_str("boolean_false_target"),
            Self::TryOkTarget(poison)
            | Self::TryErrTarget(poison)
            | Self::CapabilityPresentTarget(poison)
            | Self::CapabilityAbsentTarget(poison) => match *poison {},
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ProcessSessionObservableEffectClassifierKindV0 {
    TryResultClassifier(RawTransportValueForbiddenAtSemanticBoundary),
}

impl Serialize for ProcessSessionObservableEffectClassifierKindV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let _ = serializer;
        match self {
            Self::TryResultClassifier(poison) => match *poison {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProcessSessionResultCarrierKindV0 {
    Ok,
    Err,
    Malformed,
    Missing,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProcessSessionResultCarrierBranchDecisionV0 {
    carrier_register: Option<usize>,
    carrier_kind: ProcessSessionResultCarrierKindV0,
    error_code_or_shape: Option<String>,
}

impl ProcessSessionResultCarrierKindV0 {
    fn duplicate_for_swarmvm_session_runtime_owner_v1(&self) -> Self {
        match self {
            Self::Ok => Self::Ok,
            Self::Err => Self::Err,
            Self::Malformed => Self::Malformed,
            Self::Missing => Self::Missing,
        }
    }
}

impl ProcessSessionObservableEffectBranchKindV0 {
    fn duplicate_for_swarmvm_session_runtime_owner_v1(&self) -> Self {
        match self {
            Self::BranchOnBoolean => Self::BranchOnBoolean,
            Self::BranchOnResult(poison) | Self::BranchOnCapabilityPresence(poison) => {
                match *poison {}
            }
        }
    }
}

impl ProcessSessionObservableEffectBranchEdgeV0 {
    fn duplicate_for_swarmvm_session_runtime_owner_v1(&self) -> Self {
        match self {
            Self::BooleanTrueTarget => Self::BooleanTrueTarget,
            Self::BooleanFalseTarget => Self::BooleanFalseTarget,
            Self::TryOkTarget(poison)
            | Self::TryErrTarget(poison)
            | Self::CapabilityPresentTarget(poison)
            | Self::CapabilityAbsentTarget(poison) => match *poison {},
        }
    }
}

impl ProcessSessionObservableEffectClassifierKindV0 {
    fn duplicate_for_swarmvm_session_runtime_owner_v1(&self) -> Self {
        match self {
            Self::TryResultClassifier(poison) => match *poison {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProcessSessionObservableEffectBranchDecisionV0 {
    branch_instruction_index: usize,
    branch_kind: ProcessSessionObservableEffectBranchKindV0,
    taken_edge: ProcessSessionObservableEffectBranchEdgeV0,
    skipped_edge: ProcessSessionObservableEffectBranchEdgeV0,
    condition_register: Option<usize>,
    condition_import_id: Option<u32>,
    condition_value_shape: String,
    classifier_kind: Option<ProcessSessionObservableEffectClassifierKindV0>,
    classifier_input_shape: Option<String>,
    result_carrier_branch_decision: Option<ProcessSessionResultCarrierBranchDecisionV0>,
}

#[derive(Clone, Debug, Default)]
struct ProcessSessionSchedulerActorReadinessCounts {
    active_actor_turn_count: usize,
    actor_owned_awaiting_node_count: usize,
    root_or_caller_awaiting_node_count: usize,
    scheduled_delivery_count: usize,
}

#[derive(Clone, Debug, Default)]
struct ProcessSessionSchedulerHostActivityCounts {
    actor_owned_pending_count: usize,
    actor_owned_parked_count: usize,
    caller_owned_pending_count: usize,
    caller_owned_parked_count: usize,
}

#[path = "source_entrypoint_executable_runtime/prepared_program.rs"]
pub(crate) mod prepared_program;
pub(crate) use prepared_program::{
    DirectRunExactStaticChildDispatchInputBatchForPreparedRuntimeOwnerV1,
    DirectRunExactStaticChildDispatchInstallationRefusalForPreparedRuntimeOwnerV1,
    DirectRunExactStaticChildDispatchInstalledPreparedRuntimeForPreparedRuntimeOwnerV1,
    PreparedProgramV0, SealedPreparedRuntime,
};

#[path = "source_entrypoint_executable_runtime/process_replan_preservation.rs"]
mod process_replan_preservation;

#[path = "source_entrypoint_executable_runtime/result_carrier_owner.rs"]
mod result_carrier_owner;

include!("source_entrypoint_executable_runtime/state.rs");
include!("source_entrypoint_executable_runtime/accessors_diagnostics_checkpoint.rs");
include!("source_entrypoint_executable_runtime/local_value_owner.rs");
include!("source_entrypoint_executable_runtime/runtime_family_owner.rs");
include!("source_entrypoint_executable_runtime/runtime_memory_diagnostics.rs");
include!("source_entrypoint_executable_runtime/event_wait_public_aperture_progress.rs");
include!("source_entrypoint_executable_runtime/observable_effect_coverage.rs");
#[path = "source_entrypoint_executable_runtime/execution_kernel.rs"]
pub(crate) mod execution_kernel;
pub use execution_kernel::executable_image::{
    DirectRunProcessSessionRuntimeExactTerminalObservationV1,
    DirectRunProcessSessionRuntimeTerminalFaultObservationV1,
    ExecutableSessionImagePreparationFault, SourceEntrypointExecutableImage,
    SourceEntrypointExecutableRuntime,
    consume_source_entrypoint_executable_image_into_runtime_owner_v1,
};
pub(crate) use execution_kernel::executable_image::{
    SourceEntrypointExecutableRuntimeEntryForDirectRunPreparedRuntimeOwnerV1,
    SourceEntrypointExecutableRuntimeStaticChildTemplateConversionRefusalForDirectRunPreparedRuntimeOwnerV1,
    mint_source_entrypoint_executable_image_for_compiler_owner_v1,
};
#[path = "source_entrypoint_executable_runtime/scheduler/mod.rs"]
mod scheduler;
include!("source_entrypoint_executable_runtime/run_loop_and_resume.rs");
include!("source_entrypoint_executable_runtime/checkpoint_runtime_value_handle_records.rs");
include!("source_entrypoint_executable_runtime/checkpoint_authority_wall.rs");
include!("source_entrypoint_executable_runtime/final_observation.rs");
include!("source_entrypoint_executable_runtime/event_ledger_owner.rs");
include!("source_entrypoint_executable_runtime/event_turn_recovery.rs");
include!("source_entrypoint_executable_runtime/errors.rs");
include!("source_entrypoint_executable_runtime/open.rs");

include!("source_entrypoint_executable_runtime/checkpoint_recovery_typed_restore_apertures.rs");
