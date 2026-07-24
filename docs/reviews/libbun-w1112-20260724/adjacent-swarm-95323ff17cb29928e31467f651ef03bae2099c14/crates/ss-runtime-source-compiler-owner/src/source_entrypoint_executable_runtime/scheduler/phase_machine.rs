use std::collections::BTreeMap;

use swarmvm_image::NodeId;

use crate::session::execution_kernel::executable_image;

use super::{
    ActorRequestObservationDriveAuthority, ActorSchedulerDriveAuthority,
    CompactDeclaredReadinessDriveAuthority, CompactInstructionDispatchScope,
    CompactReadinessDriveScope, EventWaitProducerPublicApertureDriveAuthority,
    FullBoundarySchedulerDriveAuthority, InstructionExecutionAdvanceAuthority,
    PendingActivityWaitInstallAuthority, PublicApertureSchedulerDriveAuthority,
};
use crate::session::{
    CompactActorContinuationDiagnosticSnapshot, CompactActorOwnedAwaiterRestoreOutcomeV0,
    CompactChildDriveOutcome, CompactContinueProof, CurrentHandlerAwaiterNotReadyPolicy,
    DirectRunProcessSessionPublicApertureProgressProductV1,
    DirectRunProcessSessionRunResultProductV1, EventWaitProducerProviderReachabilityReceipt,
    HostBoundaryMismatchEvidenceContext, InstructionIndex, OneShotBoundaryEvent, OneShotHostResult,
    PendingActivityEffectFrame, PrivilegedHostcallInputContractFamily,
    ProcessBoundaryReadinessCertificateV1, ProcessCompletedTerminalBoundaryReadinessOwnerV1,
    ProcessCompletedTerminalOutcomeAuthorityV1, ProcessSessionActiveActorTurnDriveOutcome,
    ProcessSessionActivityEffectDescriptorV0, ProcessSessionActorRequestEffectResultOutcome,
    ProcessSessionActorSchedulerQuiescenceOutcome,
    ProcessSessionCompactExternalActorRequestReadinessStepV0,
    ProcessSessionCompactExternalActorRequestRegionOutcomeV0,
    ProcessSessionCompletedTerminalOutputEffectSettlementProductV1,
    ProcessSessionCompletedTerminalPublicOutputProductV1, ProcessSessionEntryOutcomeV0,
    ProcessSessionEventWaitParkedActivityFrameV0, ProcessSessionEventWaitParkedActivityIdentityV0,
    ProcessSessionEventWaitProducerBoundaryKindV0,
    ProcessSessionEventWaitProducerProgressOutcomeV0,
    ProcessSessionEventWaitProducerPublicApertureInstructionOutcomeV0,
    ProcessSessionEventWaitProducerPublicApertureProgressOutcomeV0,
    ProcessSessionEventWaitProducerReachabilityProofV0,
    ProcessSessionEventWaitProducerSettlementModeV0,
    ProcessSessionExternalActorRequestReadinessOutcomeV0,
    ProcessSessionIntrinsicWaitContinuationAppliedReceiptV1,
    ProcessSessionIntrinsicWaitContinuationSealV1,
    ProcessSessionObservableEffectRuntimeReachabilityV1,
    ProcessSessionPublicDiagnosticProjectionAuthority,
    ProcessSessionResultAdmissionBoundaryContextV0, ProcessSessionRunError,
    ProcessSessionRunOutcomeV0, ProcessSessionSchedulerActorReadinessCounts,
    ProcessSessionSchedulerHostActivityCounts, ProcessSessionTerminalContinuationOutcome,
    ProcessSessionV0, ProcessSessionVmTerminalProductV0,
    ProcessTerminalBoundaryReadinessSourceKindV1, ProcessTerminalBoundaryReadinessSourceV1,
    ProcessWaitingOnLivenessBoundaryReadinessOwnerV1, ProviderBoundaryIngressFault, RegisterIndex,
    RuntimeValue, SchedulerInvariantEvidence, SchedulerInvariantEvidenceContext,
    SealedProcessSessionDriveOutcome, SessionContinuationStepFaultEvidence,
    SessionContinuationStepReceipt, SessionLirRegionId, TerminalResultEvidence, VmBoundaryValue,
    execution_kernel, observable_effect_pending_reachability_for_required_order,
    process_session_public_diagnostic_projection_value, process_session_run_outcome_kind,
    session_lir_observable_effect_kind_as_str, session_lir_observable_effect_owner_kind_as_str,
    session_lir_observable_effect_required_order_as_str, swarmvm_isa,
};

include!("phase_machine_payload_refs_and_boundary_activity.inc.rs");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::session) enum ProcessSessionSchedulerPhase {
    ActorOwnedReadyAwaiter,
    ActiveActorTurn,
    ActorOwnedHostActivity,
    ScheduledActorDelivery,
    DependencyFrame,
    CallerOwnedHostActivity,
    RootReadyAwaiter,
    ActorRequestObservation,
    ProcessLiveness,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProcessSessionSchedulerPhaseDiagnosticsMode {
    FullBoundary,
    CompactPublicAperture,
}

impl ProcessSessionSchedulerPhaseDiagnosticsMode {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::FullBoundary => "full_boundary",
            Self::CompactPublicAperture => "compact_public_aperture",
        }
    }
}

const PROCESS_SESSION_SCHEDULER_PHASE_ORDER: [ProcessSessionSchedulerPhase; 9] = [
    ProcessSessionSchedulerPhase::ActorOwnedReadyAwaiter,
    ProcessSessionSchedulerPhase::ActiveActorTurn,
    ProcessSessionSchedulerPhase::ActorOwnedHostActivity,
    ProcessSessionSchedulerPhase::RootReadyAwaiter,
    ProcessSessionSchedulerPhase::ScheduledActorDelivery,
    ProcessSessionSchedulerPhase::DependencyFrame,
    ProcessSessionSchedulerPhase::CallerOwnedHostActivity,
    ProcessSessionSchedulerPhase::ActorRequestObservation,
    ProcessSessionSchedulerPhase::ProcessLiveness,
];

impl ProcessSessionSchedulerPhase {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ActorOwnedReadyAwaiter => "actor_owned_ready_awaiter",
            Self::ActiveActorTurn => "active_actor_turn",
            Self::ActorOwnedHostActivity => "actor_owned_host_activity",
            Self::ScheduledActorDelivery => "scheduled_actor_delivery",
            Self::DependencyFrame => "dependency_frame",
            Self::CallerOwnedHostActivity => "caller_owned_host_activity",
            Self::RootReadyAwaiter => "root_ready_awaiter",
            Self::ActorRequestObservation => "actor_request_observation",
            Self::ProcessLiveness => "process_liveness",
        }
    }
}

fn event_wait_producer_reachability_proof_kind(
    proof: &ProcessSessionEventWaitProducerReachabilityProofV0,
) -> &'static str {
    match proof {
        ProcessSessionEventWaitProducerReachabilityProofV0::ParkedWaiter(_) => "parked_waiter",
        ProcessSessionEventWaitProducerReachabilityProofV0::SameOwnerInterveningActivity(_) => {
            "same_owner_intervening_activity"
        }
        ProcessSessionEventWaitProducerReachabilityProofV0::PendingActivityAwaitsSelectedProducer(_) => {
            "pending_activity_awaits_selected_producer"
        }
        ProcessSessionEventWaitProducerReachabilityProofV0::ObservedActorRequestProducerActivity(_) => {
            "observed_actor_request_producer_activity"
        }
        ProcessSessionEventWaitProducerReachabilityProofV0::SchedulerSelectedProducerActivity(_) => {
            "scheduler_selected_producer_activity"
        }
        ProcessSessionEventWaitProducerReachabilityProofV0::Missing(_) => "missing",
    }
}

fn event_wait_producer_boundary_kind_label(
    kind: &ProcessSessionEventWaitProducerBoundaryKindV0,
) -> &'static str {
    match kind {
        ProcessSessionEventWaitProducerBoundaryKindV0::ConcreteProviderActivity(_) => {
            "concrete_provider_activity"
        }
        ProcessSessionEventWaitProducerBoundaryKindV0::IntrinsicWaitExpansion(_) => {
            "intrinsic_wait_expansion"
        }
        ProcessSessionEventWaitProducerBoundaryKindV0::ParkedWaiter(_) => "parked_waiter",
        ProcessSessionEventWaitProducerBoundaryKindV0::ForbiddenRootOrWaiter(_) => {
            "forbidden_root_or_waiter"
        }
        ProcessSessionEventWaitProducerBoundaryKindV0::MissingPrerequisite(_) => {
            "missing_prerequisite"
        }
    }
}

fn event_wait_producer_settlement_mode_label(
    mode: &ProcessSessionEventWaitProducerSettlementModeV0,
) -> &'static str {
    match mode {
        ProcessSessionEventWaitProducerSettlementModeV0::LiveFrameResume(_) => "live_frame_resume",
        ProcessSessionEventWaitProducerSettlementModeV0::AlreadyConsumedByActorOwnedAwaiter(_) => {
            "already_consumed_by_actor_owned_awaiter"
        }
        ProcessSessionEventWaitProducerSettlementModeV0::StaleWithoutSettlementAuthority(_) => {
            "stale_without_settlement_authority"
        }
    }
}

fn event_wait_producer_public_aperture_run_outcome_kind(
    outcome: &ProcessSessionRunOutcomeV0,
) -> &'static str {
    match outcome {
        ProcessSessionRunOutcomeV0::NeedsHostActivityEffect { .. } => "needs_host_activity_effect",
        ProcessSessionRunOutcomeV0::NeedsHostResourceFinalization { .. } => {
            "needs_host_resource_finalization"
        }
        ProcessSessionRunOutcomeV0::WaitingOnLiveness { .. } => "waiting_on_liveness",
        ProcessSessionRunOutcomeV0::Completed { .. } => "completed",
        ProcessSessionRunOutcomeV0::Failed { .. } => "failed",
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ProcessSessionActivitySchedulerOwner {
    Caller,
    ActorRequest(swarmvm_isa::ActorRequestId),
    ActorTurn(swarmvm_isa::ActorTurnId),
}

pub(in crate::session) enum ProcessSessionSchedulerPhaseDriveOutcome {
    Consumed(ProcessSessionSchedulerPhaseOutcome),
    RootContinuationReady(ProcessSessionSchedulerPhaseOutcome),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingProviderResumeOwnerForSessionRuntimeOwnerV1 {
    Caller,
    ActorHandler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::session) enum ProcessSessionReadyActorRequestAwaiterActivation {
    NoReadyAwaiter,
    RestoredContinuation,
    RetiredStaleAwaiter,
}

impl ProcessSessionReadyActorRequestAwaiterActivation {
    pub(super) fn progressed(self) -> bool {
        !matches!(self, Self::NoReadyAwaiter)
    }
}

fn compact_readiness_json_summary_value(value: Option<&serde_json::Value>) -> String {
    match value {
        Some(serde_json::Value::String(value)) => value.clone(),
        Some(serde_json::Value::Number(value)) => value.to_string(),
        Some(serde_json::Value::Bool(value)) => value.to_string(),
        Some(serde_json::Value::Null) | None => "null".to_owned(),
        Some(value) => {
            serde_json::to_string(value).unwrap_or_else(|_| "<unserializable>".to_owned())
        }
    }
}

fn compact_readiness_decision_summary(
    facts: &BTreeMap<String, serde_json::Value>,
    request_id: &swarmvm_isa::ActorRequestId,
    phase: &str,
    decision: &str,
    reason: &str,
) -> String {
    let active_region_id = compact_readiness_json_summary_value(facts.get("active_region_id"));
    let active_instruction_index =
        compact_readiness_json_summary_value(facts.get("active_instruction_index"));
    let active_opcode = compact_readiness_json_summary_value(facts.get("active_opcode"));
    let top_actor_handler_frame =
        compact_readiness_json_summary_value(facts.get("top_actor_handler_suspension_frame"));
    let pending_activity_attempt_id =
        compact_readiness_json_summary_value(facts.get("pending_activity_attempt_id"));
    let memory_pages = compact_readiness_json_summary_value(facts.get("memory_pages"));
    let scheduler_step = compact_readiness_json_summary_value(facts.get("scheduler_step"));
    let blocked_turn_id = compact_readiness_json_summary_value(facts.get("blocked_turn_id"));
    let blocked_handler_region_id =
        compact_readiness_json_summary_value(facts.get("blocked_handler_region_id"));
    let waiting_request_id = compact_readiness_json_summary_value(facts.get("waiting_request_id"));
    let selected_actor = facts
        .get("ready_work_selection")
        .and_then(|selection| selection.get("selected_actor"))
        .map(|value| compact_readiness_json_summary_value(Some(value)))
        .unwrap_or_else(|| "unknown".to_owned());
    let _ = request_id;
    let request_id = "sealed_request";

    format!(
        "request_id={request_id} phase={phase} decision={decision} reason={reason} scheduler_step={scheduler_step} active_region_id={active_region_id} active_instruction_index={active_instruction_index} active_opcode={active_opcode} top_actor_handler_frame={top_actor_handler_frame} pending_activity_attempt_id={pending_activity_attempt_id} blocked_turn_id={blocked_turn_id} blocked_handler_region_id={blocked_handler_region_id} waiting_request_id={waiting_request_id} selected_actor={selected_actor} memory_pages={memory_pages}"
    )
}

fn compact_readiness_decision_is_high_signal(decision: &str) -> bool {
    matches!(
        decision,
        "drive_child_request"
            | "entered_turn"
            | "needs_host_activity_effect"
            | "parent_continuation_restored"
            | "progress_fault"
            | "ready"
            | "ready_result_visible"
            | "terminal_fault"
            | "waiting_on_liveness"
    )
}

impl ProcessSessionV0 {
    pub(super) fn record_compact_readiness_decision(
        &self,
        request_id: &swarmvm_isa::ActorRequestId,
        scheduler_step: Option<usize>,
        phase: &'static str,
        decision: &'static str,
        reason: &'static str,
        mut facts: BTreeMap<String, serde_json::Value>,
    ) {
        if !compact_readiness_decision_is_high_signal(decision) {
            return;
        }
        let decision_start_pages = compact_external_actor_request_wasm_memory_page_count();
        let _ = request_id;
        let request_id_text = "sealed_request";
        let mut event_facts =
            self.compact_actor_scheduler_scalar_context_facts(Some(request_id_text), None, None);
        event_facts.insert(
            "schema".to_owned(),
            serde_json::Value::String(
                "swarm.vm.compact_external_actor_request_readiness.decision.v1".to_owned(),
            ),
        );
        event_facts.insert(
            "phase".to_owned(),
            serde_json::Value::String(phase.to_owned()),
        );
        event_facts.insert(
            "decision".to_owned(),
            serde_json::Value::String(decision.to_owned()),
        );
        event_facts.insert(
            "reason".to_owned(),
            serde_json::Value::String(reason.to_owned()),
        );
        event_facts.insert(
            "memory_pages".to_owned(),
            serde_json::Value::Number(serde_json::Number::from(
                compact_external_actor_request_wasm_memory_page_count(),
            )),
        );
        if let Some(scheduler_step) = scheduler_step {
            event_facts.insert(
                "scheduler_step".to_owned(),
                serde_json::Value::Number(serde_json::Number::from(scheduler_step as u64)),
            );
        }
        event_facts.append(&mut facts);
        let summary =
            compact_readiness_decision_summary(&event_facts, request_id, phase, decision, reason);
        event_facts.insert(
            "summary".to_owned(),
            serde_json::Value::String(summary.clone()),
        );
        let detail = serde_json::to_string(&event_facts).unwrap_or_else(|_| {
            serde_json::json!({
                "schema": "swarm.vm.compact_external_actor_request_readiness.decision.v1",
                "request_id": request_id_text,
                "phase": phase,
                "decision": decision,
                "reason": reason,
                "summary": summary,
                "diagnostic_error": "serialization_failed",
            })
            .to_string()
        });
        let detail_len = detail.len();
        if crate::vm_runtime_logical_materialization_fact_enabled_v1() {
            let logical_bytes = detail_len as u64;
            crate::vm_runtime_logical_materialization_fact_v1(
                "vm.actor_scheduler.compact_readiness.decision.serialized_fact",
                &format!(
                    "request_id={request_id_text} phase={phase} decision={decision} reason={reason} scheduler_step={} serialized_bytes={logical_bytes}",
                    scheduler_step
                        .map(|step| step.to_string())
                        .unwrap_or_else(|| "none".to_owned())
                ),
                logical_bytes,
                logical_bytes,
            );
        }
        let detail_for_breadcrumb = detail.clone();
        crate::vm_runtime_trap_breadcrumb_v1(
            "vm.compact_external_actor_request_readiness.decision",
            move || detail_for_breadcrumb.clone(),
        );
        let before_diagnostic_record_pages =
            compact_external_actor_request_wasm_memory_page_count();
        crate::record_continuation_diagnostic_event_v1(
            "vm.actor_scheduler",
            "vm.actor_scheduler.compact_readiness.decision",
            crate::ContinuationDiagnosticTransitionV1::Completed,
            event_facts,
        );
        let after_diagnostic_record_pages = compact_external_actor_request_wasm_memory_page_count();
        if after_diagnostic_record_pages > before_diagnostic_record_pages {
            crate::vm_runtime_memory_materialization_fact_v1(
                "vm.actor_scheduler.compact_readiness.decision.diagnostic_event_record",
                &format!(
                    "request_id={request_id} phase={phase} decision={decision} reason={reason} scheduler_step={}",
                    scheduler_step
                        .map(|step| step.to_string())
                        .unwrap_or_else(|| "none".to_owned())
                ),
                before_diagnostic_record_pages,
                after_diagnostic_record_pages,
            );
        }
        let decision_after_pages = compact_external_actor_request_wasm_memory_page_count();
        if decision_after_pages > decision_start_pages {
            crate::vm_runtime_memory_materialization_fact_v1(
                "vm.actor_scheduler.compact_readiness.decision.emit",
                &format!(
                    "request_id={request_id} phase={phase} decision={decision} reason={reason} scheduler_step={} serialized_bytes={}",
                    scheduler_step
                        .map(|step| step.to_string())
                        .unwrap_or_else(|| "none".to_owned()),
                    detail_len
                ),
                decision_start_pages,
                decision_after_pages,
            );
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::session) enum ProcessSessionSchedulerPhaseOutcomeKind {
    Consumed,
    RootContinuationReady,
    Quiescent,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::session) struct ProcessSessionSchedulerPhaseOutcome {
    phase: ProcessSessionSchedulerPhase,
    kind: ProcessSessionSchedulerPhaseOutcomeKind,
    reason: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::session) struct ProcessSessionSchedulerQuiescenceProof {
    phase_outcome: ProcessSessionSchedulerPhaseOutcome,
}

impl ProcessSessionSchedulerPhaseOutcome {
    fn consumed(phase: ProcessSessionSchedulerPhase, reason: &'static str) -> Self {
        Self {
            phase,
            kind: ProcessSessionSchedulerPhaseOutcomeKind::Consumed,
            reason,
        }
    }

    fn root_continuation_ready(phase: ProcessSessionSchedulerPhase, reason: &'static str) -> Self {
        Self {
            phase,
            kind: ProcessSessionSchedulerPhaseOutcomeKind::RootContinuationReady,
            reason,
        }
    }

    fn quiescent(phase: ProcessSessionSchedulerPhase, reason: &'static str) -> Self {
        Self {
            phase,
            kind: ProcessSessionSchedulerPhaseOutcomeKind::Quiescent,
            reason,
        }
    }

    fn fault(phase: ProcessSessionSchedulerPhase, reason: &'static str) -> Self {
        Self {
            phase,
            kind: ProcessSessionSchedulerPhaseOutcomeKind::Fault,
            reason,
        }
    }

    pub(super) fn phase(self) -> ProcessSessionSchedulerPhase {
        self.phase
    }

    pub(in crate::session) fn kind(self) -> ProcessSessionSchedulerPhaseOutcomeKind {
        self.kind
    }

    pub(super) fn reason(self) -> &'static str {
        self.reason
    }

    pub(in crate::session) fn diagnostic_labels_for_session_runtime_owner_v1(
        self,
    ) -> (&'static str, &'static str) {
        (self.phase.as_str(), self.reason)
    }
}

impl ProcessSessionSchedulerQuiescenceProof {
    fn new(phase_outcome: ProcessSessionSchedulerPhaseOutcome) -> Self {
        debug_assert_eq!(
            phase_outcome.kind(),
            ProcessSessionSchedulerPhaseOutcomeKind::Quiescent
        );
        Self { phase_outcome }
    }

    pub(super) fn phase_outcome(self) -> ProcessSessionSchedulerPhaseOutcome {
        self.phase_outcome
    }
}

#[cfg(target_arch = "wasm32")]
fn compact_external_actor_request_wasm_memory_page_count() -> u64 {
    core::arch::wasm32::memory_size(0) as u64
}

#[cfg(not(target_arch = "wasm32"))]
fn compact_external_actor_request_wasm_memory_page_count() -> u64 {
    0
}

impl ProcessSessionActivitySchedulerOwner {
    pub(super) fn actor_turn_id(&self) -> Option<&str> {
        match self {
            Self::Caller => None,
            Self::ActorRequest(_) => None,
            Self::ActorTurn(_turn_id) => Some("sealed"),
        }
    }
}

include!("phase_machine_drive_entrypoints.inc.rs");
include!("phase_machine_drive_loop.inc.rs");
include!("phase_machine_event_wait_public_aperture_boundary.inc.rs");
