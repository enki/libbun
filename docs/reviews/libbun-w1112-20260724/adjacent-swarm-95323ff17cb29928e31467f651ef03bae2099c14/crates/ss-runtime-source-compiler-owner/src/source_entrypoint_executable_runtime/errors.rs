#[derive(Debug, PartialEq, Error)]
pub enum EventTurnLedgerReplayError {
    #[error("event turn ledger replay has no pending activity to resume")]
    NoPendingActivity,
    #[error(
        "event turn ledger replay cannot materialize a pending activity request body from an ADR-2024 pending effect frame"
    )]
    PendingActivityRequestBodyMaterializationForbidden,
    #[error("event turn ledger replay missing activity event at index {next_activity_event}")]
    MissingActivityEvent { next_activity_event: usize },
    #[error(
        "event turn ledger replay activity event index mismatch: expected {expected}, observed {observed}"
    )]
    ActivityEventIndexMismatch { expected: u64, observed: u64 },
    #[error(
        "event turn ledger replay activity effect frame mismatch: expected effect_ref {expected_effect_ref}, observed {observed_effect_ref}"
    )]
    ActivityEffectFrameMismatchForbiddenRequireEffectFrame {
        expected_effect_ref: String,
        observed_effect_ref: String,
    },
    #[error(
        "event turn ledger activity frame read authority requires the selected event to belong to the replay ledger"
    )]
    ActivityEventFrameReadAuthorityForbiddenRequireLedgerMembership,
    #[error(
        "event turn ledger replay cannot apply an activity result ref without ADR-2024 effect-ledger payload resolution for effect_ref {effect_ref}"
    )]
    ActivityResultRefReplayRequiresEffectLedgerPayloadResolution { effect_ref: String },
    #[error(
        "event turn ledger replay stopped at activity event {next_activity_event} with {activity_event_count} recorded activity events"
    )]
    UnconsumedActivityEvents {
        next_activity_event: usize,
        activity_event_count: usize,
    },
    #[error("{source}")]
    Resume {
        #[from]
        source: ProcessSessionResumeError,
    },
    #[error("{source}")]
    Run {
        #[from]
        source: ProcessSessionRunError,
    },
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum EventTurnLedgerAppendError {
    #[error("event turn ledger append has no pending session activity to record")]
    NoPendingActivity,
    #[error(
        "event turn ledger append cannot materialize a pending activity request body from an ADR-2024 pending effect frame"
    )]
    PendingActivityRequestBodyMaterializationForbidden,
    #[error(
        "event turn ledger append cannot store a full activity result body; record a provider-result PayloadHandle descriptor"
    )]
    ActivityResultBodyStorageForbidden,
    #[error(
        "event turn ledger append cursor mismatch: expected next event index {expected_next_event_index}, observed event cursor {observed_event_cursor}"
    )]
    EventCursorMismatch {
        expected_next_event_index: u64,
        observed_event_cursor: u64,
    },
}

#[derive(Debug, PartialEq, Error)]
pub enum ProcessSessionEventTurnLedgerRecoveryError {
    #[error(
        "process session checkpoint event cursor {checkpoint_event_cursor} does not match event turn ledger base event index {ledger_base_event_index}"
    )]
    CheckpointEventCursorMismatch {
        checkpoint_event_cursor: u64,
        ledger_base_event_index: u64,
    },
    #[error(
        "process session event-turn recovery exceeded {step_limit} scheduler steps without reaching declared readiness or consuming replay progress; replay cursor {next_activity_event}/{activity_event_count}, pending_activity={pending_activity_summary}"
    )]
    ProgressFault {
        step_limit: usize,
        next_activity_event: usize,
        activity_event_count: usize,
        pending_activity_summary: String,
    },
    #[error("{source}")]
    CheckpointRestore {
        #[from]
        source: ProcessSessionCheckpointRestoreError,
    },
    #[error("{source}")]
    Run {
        #[from]
        source: ProcessSessionRunError,
    },
    #[error("{source}")]
    Replay {
        #[from]
        source: EventTurnLedgerReplayError,
    },
    #[error("process session event-turn recovery checkpoint projection failed: {message}")]
    CheckpointProjection { message: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessSessionDriveFault {
    CallLocalRegion {
        source: String,
    },
    ActivityRequestBoundaryInputRequiresSealedProduct {
        input_register: RegisterIndex,
    },
    RegionLifecycle {
        operation: ProcessSessionRegionLifecycleOperation,
        source: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSessionRegionLifecycleOperation {
    ActorStartReceiverWrite,
    ApplySelectedCallHostResult,
    SetRuntimeValueRegister,
    EnterSelectedActorHandlerRegion,
    RestoreSelectedActorHandlerRegion,
    RestoreRootOrCallerReadyAwaiter,
}

impl ProcessSessionDriveFault {
    pub(in crate::session) fn call_local_region(source: String) -> Self {
        Self::CallLocalRegion { source }
    }

    pub(in crate::session) fn region_lifecycle(
        operation: ProcessSessionRegionLifecycleOperation,
        source: String,
    ) -> Self {
        Self::RegionLifecycle { operation, source }
    }

    fn actor_scheduler_compatibility_message(&self) -> Option<String> {
        match self {
            Self::CallLocalRegion { source } => Some(
                serde_json::json!({
                    "kind": "call_local_region_entry_failed",
                    "detail": source,
                })
                .to_string(),
            ),
            Self::ActivityRequestBoundaryInputRequiresSealedProduct { .. } => None,
            Self::RegionLifecycle {
                operation: _,
                source,
            } => Some(source.clone()),
        }
    }

    fn diagnostic_details(&self) -> serde_json::Value {
        match self {
            Self::ActivityRequestBoundaryInputRequiresSealedProduct { input_register } => {
                serde_json::json!({
                    "schema": "swarm.vm.process_session.run_error.actor_scheduler_selected_activity_request_boundary_input_requires_sealed_product.details.v1",
                    "kind": "actor_scheduler_selected_activity_request_boundary_input_requires_sealed_product",
                    "input_register": input_register.to_string(),
                    "required_product": "sealed_boundary_object_input_product",
                })
            }
            Self::RegionLifecycle { .. } => {
                let message = self
                    .actor_scheduler_compatibility_message()
                    .expect("region lifecycle faults have compatibility messages");
                serde_json::json!({
                    "schema": "swarm.vm.process_session.run_error.actor_scheduler.details.v1",
                    "kind": "actor_scheduler",
                    "message": message,
                    "parsed_message": ProcessSessionRunError::parse_diagnostic_json(&message),
                })
            }
            _ => {
                let message = self
                    .actor_scheduler_compatibility_message()
                    .expect("actor-scheduler drive faults have compatibility messages");
                serde_json::json!({
                    "schema": "swarm.vm.process_session.run_error.actor_scheduler.details.v1",
                    "kind": "actor_scheduler",
                    "message": message,
                    "parsed_message": ProcessSessionRunError::parse_diagnostic_json(&message),
                })
            }
        }
    }
}

impl std::fmt::Display for ProcessSessionDriveFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ActivityRequestBoundaryInputRequiresSealedProduct { input_register } => write!(
                formatter,
                "process session actor scheduler selected activity request boundary input register {input_register} requires sealed boundary-object input product"
            ),
            _ => write!(
                formatter,
                "process session actor scheduler failed: {}",
                self.actor_scheduler_compatibility_message()
                    .expect("actor-scheduler drive faults have compatibility messages")
            ),
        }
    }
}

impl std::error::Error for ProcessSessionDriveFault {}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ProviderBoundaryIngressFault {
    #[error("provider-ready boundary commit has no selected pending provider application")]
    NoPendingApplication,
    #[error(
        "provider-ready boundary commit found a different pending kernel boundary: {pending_kind}"
    )]
    DifferentPendingBoundary { pending_kind: &'static str },
    #[error("provider-ready boundary correspondence failed: {source}")]
    OutputCorrespondence {
        #[from]
        source: swarm_capability_model::ProviderBoundaryOutputCorrespondenceFault,
    },
    #[error("provider-ready boundary runtime-value admission failed: {source}")]
    RuntimeValueAdmission {
        #[from]
        source: crate::VmRuntimeHeapAllocationError,
    },
    #[error("provider-ready boundary executable-state commit failed: {source}")]
    ExecutionCommit {
        #[from]
        source: ProviderBoundaryExecutionCommitFault,
    },
}

/// Opaque typed refusal from the private provider executable-frame owner.
/// Public callers can report it but cannot inspect slots, registers, or
/// reconstruct a retry application from its internals.
pub struct ProviderBoundaryExecutionCommitFault {
    source: crate::session::execution_state::ProviderExecutionFrameFault,
}

impl ProviderBoundaryExecutionCommitFault {
    pub(in crate::session) fn from_provider_execution_frame(
        source: crate::session::execution_state::ProviderExecutionFrameFault,
    ) -> Self {
        Self { source }
    }
}

impl std::fmt::Debug for ProviderBoundaryExecutionCommitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderBoundaryExecutionCommitFault")
            .field("source", &self.source.to_string())
            .finish()
    }
}

impl std::fmt::Display for ProviderBoundaryExecutionCommitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(formatter)
    }
}

impl std::error::Error for ProviderBoundaryExecutionCommitFault {}

impl PartialEq for ProviderBoundaryExecutionCommitFault {
    fn eq(&self, other: &Self) -> bool {
        self.source.to_string() == other.source.to_string()
    }
}

impl Eq for ProviderBoundaryExecutionCommitFault {}

/// Closed typed refusals for the linear host-resource finalization boundary.
/// No variant carries the private correspondence identity or obligation.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum HostResourceFinalizationBoundaryFaultV1 {
    #[error(
        "host-resource finalization selection refused because another selected boundary is already pending"
    )]
    SelectionAlreadyPending,
    #[error("host-resource finalization selection identity space is exhausted")]
    SelectionIdentitySpaceExhausted,
    #[error("host-resource finalization commit has no pending selected boundary")]
    CommitWithoutPendingSelection,
    #[error(
        "host-resource finalization commit does not correspond to this process session's pending selection"
    )]
    CommitIdentityMismatch,
    #[error(
        "host-resource finalization selection state invariant failed: expected {expected}, observed {observed}"
    )]
    SelectionStateInvariant {
        expected: &'static str,
        observed: &'static str,
    },
    #[error(
        "process-session drive refused until the pending selected host-resource finalization boundary is committed"
    )]
    DriveBeforeSelectedBoundaryCommit,
}

impl HostResourceFinalizationBoundaryFaultV1 {
    pub const fn diagnostic_kind(&self) -> &'static str {
        match self {
            Self::SelectionAlreadyPending => "host_resource_finalization_selection_already_pending",
            Self::SelectionIdentitySpaceExhausted => {
                "host_resource_finalization_selection_identity_space_exhausted"
            }
            Self::CommitWithoutPendingSelection => {
                "host_resource_finalization_commit_without_pending_selection"
            }
            Self::CommitIdentityMismatch => "host_resource_finalization_commit_identity_mismatch",
            Self::SelectionStateInvariant { .. } => {
                "host_resource_finalization_selection_state_invariant"
            }
            Self::DriveBeforeSelectedBoundaryCommit => {
                "host_resource_finalization_drive_before_selected_boundary_commit"
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum HostResourceFinalizationDriveFaultV1 {
    #[error("host-resource finalization provider release failed: {source}")]
    ProviderRelease {
        #[from]
        source: swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseFaultV1,
    },
}

#[derive(Debug, PartialEq, Error)]
pub enum ProcessSessionRunError {
    #[error(
        "process session region '{region_id}' instruction[{instruction_index}] is out of bounds"
    )]
    InstructionOutOfBounds {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
    },
    #[error(
        "process session register {register} is unbound at region '{region_id}' instruction[{instruction_index}] opcode '{opcode}'; actor checkpoint {actor_checkpoint_summary}; scheduler_context={scheduler_context}; register_context={register_context}"
    )]
    UnboundRegister {
        register: RegisterIndex,
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        opcode: String,
        actor_checkpoint_summary: String,
        scheduler_context: String,
        register_context: String,
    },
    #[error(
        "process session machine instruction failed at region '{region_id}' instruction[{instruction_index}] opcode '{opcode}': {source}; actor checkpoint {actor_checkpoint_summary}; scheduler_context={scheduler_context}; instruction_context={instruction_context}"
    )]
    MachineInstruction {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        opcode: String,
        actor_checkpoint_summary: String,
        scheduler_context: String,
        instruction_context: String,
        source: crate::MachineError,
    },
    #[error(
        "process session call-host input register {input_register} is unbound at region '{region_id}' instruction[{instruction_index}] contract family {contract_family:?}; actor checkpoint {actor_checkpoint_summary}; runtime_context={runtime_context}"
    )]
    CallHostInputRegisterUnbound {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        import_id: ImportId,
        input_register: RegisterIndex,
        contract_family: Option<PrivilegedHostcallInputContractFamily>,
        actor_checkpoint_summary: String,
        runtime_context: String,
    },
    #[error(
        "ADR-2036 consumed host-boundary frontier re-entry forbidden at region '{region_id}' instruction[{instruction_index}] input register {input_register}; consumed boundary {consumed_boundary_summary}; actor checkpoint {actor_checkpoint_summary}"
    )]
    ConsumedHostBoundaryFrontierReentryForbidden {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        import_id: ImportId,
        input_register: RegisterIndex,
        contract_family: Option<PrivilegedHostcallInputContractFamily>,
        actor_checkpoint_summary: String,
        consumed_boundary_summary: serde_json::Value,
    },
    #[error(
        "process session missing activity site for region '{region_id}' instruction[{instruction_index}]"
    )]
    ActivitySiteMissing {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        import_id: ImportId,
    },
    #[error(
        "process session does not yet support region '{region_id}' instruction[{instruction_index}] opcode '{opcode:?}'"
    )]
    UnsupportedInstruction {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        opcode: InstructionOpcode,
    },
    #[error("process session privileged host bridge failed: {message}")]
    PrivilegedHostBridge { message: String },
    #[error("process session privileged host result failed admission: {message}")]
    PrivilegedHostContract { message: String },
    #[error("process session volatile coroutine frame transition failed: {message}")]
    VolatileCoroutineFrame { message: String },
    #[error("process session actor-handler entry preparation refused: {source}")]
    ActorHandlerEntryPreparation {
        source: crate::session::ProcessSessionActorHandlerEntryPreparationFault,
    },
    #[error(
        "process session kernel intrinsic '{operation_id}' failed at region '{region_id}' instruction[{instruction_index}] input register {input_register}: {source}; input {input_context}"
    )]
    KernelIntrinsic {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        operation_id: InstructionOpId,
        input_register: RegisterIndex,
        input_context: String,
        source: crate::MachineError,
    },
    #[error(
        "process session unsupported kernel intrinsic '{operation_id}' at region '{region_id}' instruction[{instruction_index}] input register {input_register}; input {input_context}"
    )]
    KernelIntrinsicUnsupported {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        operation_id: InstructionOpId,
        input_register: RegisterIndex,
        input_context: String,
    },
    #[error(
        "process session result branch failed at region '{region_id}' instruction[{instruction_index}] carrier register {carrier_register}: {source}; input {input_context}"
    )]
    ResultBranch {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        carrier_register: RegisterIndex,
        input_context: String,
        source: crate::MachineError,
    },
    #[error("process session actor scheduler failed: {message}")]
    SchedulerInvariant { message: String },
    #[error("process session runtime-obligation ledger close failed: {fault:?}")]
    RuntimeObligationClose {
        fault: crate::one_shot_run_api::RuntimeObligationLedgerCloseFaultV1,
    },
    #[error("process session actor scheduler failed: {observation}")]
    RuntimeTerminal {
        observation: DirectRunProcessSessionRuntimeTerminalFaultObservationV1,
        context: &'static str,
    },
    #[error("{source}")]
    KernelExecution {
        source: Box<crate::session::execution_kernel::executable_image::KernelExecutionFault>,
    },
    #[error("{source}")]
    ProviderBoundaryIngress {
        #[from]
        source: ProviderBoundaryIngressFault,
    },
    #[error("{source}")]
    HostResourceFinalizationBoundary {
        #[from]
        source: HostResourceFinalizationBoundaryFaultV1,
    },
    #[error("{source}")]
    HostResourceFinalizationDrive {
        #[from]
        source: HostResourceFinalizationDriveFaultV1,
    },
    #[error("{source}")]
    Drive { source: ProcessSessionDriveFault },
    #[error("{source}")]
    Resume {
        #[from]
        source: ProcessSessionResumeError,
    },
    #[error("{source}")]
    Machine { source: crate::MachineError },
    #[error("{source}")]
    Program {
        #[from]
        source: crate::ProgramFault,
    },
}

impl ProcessSessionRunError {
    pub fn diagnostic_details(&self) -> Option<serde_json::Value> {
        match self {
            Self::UnboundRegister {
                register,
                region_id,
                instruction_index,
                opcode,
                actor_checkpoint_summary,
                scheduler_context,
                register_context,
            } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.unbound_register.details.v1",
                "kind": "unbound_register",
                "opcode": opcode,
                "actor_checkpoint_summary": actor_checkpoint_summary,
                "scheduler_context": Self::parse_diagnostic_json(scheduler_context),
                "register_context": Self::parse_diagnostic_json(register_context),
            })),
            Self::MachineInstruction {
                region_id,
                instruction_index,
                opcode,
                actor_checkpoint_summary,
                scheduler_context,
                instruction_context,
                source,
            } => {
                let source_projection = crate::project_machine_error_failure_projection(source);
                Some(serde_json::json!({
                    "schema": "swarm.vm.process_session.run_error.machine_instruction.details.v1",
                    "kind": "machine_instruction",
                    "opcode": opcode,
                    "actor_checkpoint_summary": actor_checkpoint_summary,
                    "scheduler_context": Self::parse_diagnostic_json(scheduler_context),
                    "instruction_context": Self::parse_diagnostic_json(instruction_context),
                    "source": source.to_string(),
                    "source_projection": {
                        "kind": source_projection.kind,
                        "code": source_projection.code,
                        "message": source_projection.message,
                        "details_present": source_projection.details_present(),
                    },
                }))
            }
            Self::CallHostInputRegisterUnbound {
                region_id,
                instruction_index,
                import_id,
                input_register,
                contract_family,
                actor_checkpoint_summary,
                runtime_context,
            } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.call_host_input_register_unbound.details.v1",
                "kind": "call_host_input_register_unbound",
                "contract_family": contract_family
                    .as_ref()
                    .map(|family| format!("{family:?}")),
                "actor_checkpoint_summary": actor_checkpoint_summary,
                "runtime_context": Self::parse_diagnostic_json(runtime_context),
            })),
            Self::ConsumedHostBoundaryFrontierReentryForbidden {
                region_id,
                instruction_index,
                import_id,
                input_register,
                contract_family,
                actor_checkpoint_summary,
                consumed_boundary_summary,
            } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.consumed_host_boundary_frontier_reentry_forbidden.details.v1",
                "kind": "consumed_host_boundary_frontier_reentry_forbidden",
                "adr": "ADR-2036",
                "contract_family": contract_family
                    .as_ref()
                    .map(|family| format!("{family:?}")),
                "actor_checkpoint_summary": actor_checkpoint_summary,
                "consumed_boundary_summary": consumed_boundary_summary,
            })),
            Self::KernelIntrinsic {
                region_id,
                instruction_index,
                operation_id,
                input_register,
                input_context,
                source,
            } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.kernel_intrinsic.details.v1",
                "kind": "kernel_intrinsic",
                "input_context": input_context,
                "source": source.to_string(),
            })),
            Self::ResultBranch {
                region_id,
                instruction_index,
                carrier_register,
                input_context,
                source,
            } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.result_branch.details.v1",
                "kind": "result_branch",
                "input_context": input_context,
                "source": source.to_string(),
            })),
            Self::SchedulerInvariant { message } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.actor_scheduler.details.v1",
                "kind": "actor_scheduler",
                "message": message,
                "parsed_message": Self::parse_diagnostic_json(message),
            })),
            Self::RuntimeTerminal { .. } => None,
            Self::KernelExecution { source } => {
                let (code, message) = source.fault_observation();
                Some(serde_json::json!({
                    "schema": "swarm.vm.process_session.run_error.kernel_execution.details.v1",
                    "kind": code,
                    "message": message,
                }))
            }
            Self::ProviderBoundaryIngress { source } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.provider_boundary_ingress.details.v1",
                "kind": "provider_boundary_ingress",
                "source": source.to_string(),
            })),
            Self::HostResourceFinalizationBoundary { source } => Some(serde_json::json!({
                "schema": "swarm.vm.process_session.run_error.host_resource_finalization_boundary.details.v1",
                "kind": source.diagnostic_kind(),
                "source": source.to_string(),
            })),
            Self::Drive { source } => Some(source.diagnostic_details()),
            _ => None,
        }
    }

    fn parse_diagnostic_json(input: &str) -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>(input)
            .unwrap_or_else(|_| serde_json::Value::String(input.to_owned()))
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ProcessSessionResumeError {
    #[error("process session has no pending activity to resume")]
    NoPendingActivity,
    #[error(
        "process session pending activity mismatch expected attempt '{expected_activity_attempt_id}' site '{expected_site_id}' observed attempt '{observed_activity_attempt_id}' site '{observed_site_id}'"
    )]
    PendingActivityMismatch {
        expected_activity_attempt_id: swarmvm_isa::ActivityAttemptId,
        expected_site_id: SessionLirDurableSiteId,
        observed_activity_attempt_id: swarmvm_isa::ActivityAttemptId,
        observed_site_id: SessionLirDurableSiteId,
    },
    #[error("process session resume region mismatch expected '{expected}' observed '{observed}'")]
    RegionMismatch {
        expected: SessionLirRegionId,
        observed: SessionLirRegionId,
    },
    #[error(
        "process session resume instruction mismatch expected instruction[{expected}] observed instruction[{observed}]"
    )]
    InstructionIndexMismatch {
        expected: InstructionIndex,
        observed: InstructionIndex,
    },
    #[error("process session privileged boundary surface is invalid: {message}")]
    PrivilegedBoundarySurface { message: String },
    #[error("process session privileged boundary input is invalid: {message}")]
    PrivilegedBoundaryInput { message: String },
    #[error("process session privileged boundary replay failed: {message}")]
    PrivilegedBoundaryReplay { message: String },
    #[error(
        "process session activity result body resume is forbidden by ADR-2024; provider results must resume through a ProviderResume work handle and PayloadHandle-backed result record: {message}"
    )]
    ActivityResultBodyResumeForbidden { message: String },
    #[error("process session resume register restore failed: {message}")]
    RegisterRestore { message: String },
    #[error("process session volatile coroutine frame resume failed: {message}")]
    VolatileCoroutineFrame { message: String },
    #[error("process session pending activity resume site is not admitted")]
    PendingActivitySiteMissing {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        import_id: ImportId,
    },
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ProcessSessionOpenError {
    #[error("process session entrypoint '{entrypoint_id}' is not admitted by prepared program")]
    UnknownEntrypoint {
        entrypoint_id: SessionLirEntrypointId,
    },
    #[error("process session prepared-program open admission was rejected: {admission_error:?}")]
    PreparedProgramOpenAdmissionRejected {
        admission_error: executable_image::SessionLirPreparedProgramAdmissionErrorV1,
    },
    #[error("process session entrypoint region '{region_id}' is not admitted: {message}")]
    PreparedRegionPageAdmission {
        region_id: SessionLirRegionId,
        message: String,
    },
    #[error("process session open requires owned prepared program authority: {message}")]
    OwnedPreparedProgramRequired { message: String },
    #[error("process session entry-frame input admission failed: {message}")]
    EntryFrameInputAdmission { message: String },
    #[error("{source}")]
    DurabilityPolicy {
        #[from]
        source: ProcessSessionDurabilityPolicyAdmissionError,
    },
}

#[derive(Debug, PartialEq, Error)]
pub enum ProcessSessionCheckpointRestoreError {
    #[error("{source}")]
    DurabilityPolicy {
        #[from]
        source: ProcessSessionDurabilityPolicyAdmissionError,
    },
    #[error("process session checkpoint program fingerprint does not match prepared program")]
    ProgramFingerprintMismatch,
    #[error(
        "process session checkpoint entrypoint '{entrypoint_id}' is not admitted by prepared program"
    )]
    UnknownEntrypoint {
        entrypoint_id: SessionLirEntrypointId,
    },
    #[error(
        "process session checkpoint active region '{region_id}' is not admitted by prepared program"
    )]
    UnknownActiveRegion { region_id: SessionLirRegionId },
    #[error("process session checkpoint duplicates region '{region_id}'")]
    DuplicateRegion { region_id: SessionLirRegionId },
    #[error("process session checkpoint references unknown region '{region_id}'")]
    UnknownRegion { region_id: SessionLirRegionId },
    #[error("process session checkpoint region '{region_id}' is not admitted: {message}")]
    PreparedRegionPageAdmission {
        region_id: SessionLirRegionId,
        message: String,
    },
    #[error("process session checkpoint missing region '{region_id}'")]
    MissingRegion { region_id: SessionLirRegionId },
    #[error(
        "process session checkpoint region '{region_id}' register snapshot has {observed} slots, expected {expected}"
    )]
    RegisterCountMismatch {
        region_id: SessionLirRegionId,
        expected: usize,
        observed: usize,
    },
    #[error(
        "process session checkpoint region '{region_id}' instruction[{instruction_index}] is out of bounds"
    )]
    InstructionOutOfBounds {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
    },
    #[error(
        "process session checkpoint region '{region_id}' live-register mask does not match admitted checkpoint layout"
    )]
    CheckpointRegionLiveRegistersMismatch { region_id: SessionLirRegionId },
    #[error(
        "process session checkpoint host resource handle at '{path}' is not checkpoint-resumable"
    )]
    HostResourceHandleNotCheckpointResumable {
        path: String,
        handle_id: HostResourceHandleId,
        resume_policy: HostResourceResumePolicy,
    },
    #[error(
        "process session checkpoint host resource handle at '{path}' requires explicit rebind evidence"
    )]
    HostResourceRebindEvidenceMissing {
        path: String,
        handle_id: HostResourceHandleId,
    },
    #[error("process session checkpoint host resource rebind requirements drifted")]
    HostResourceRebindRequirementsMismatch,
    #[error("process session checkpoint runtime heap graph is invalid: {message}")]
    RuntimeHeapGraphInvalid { message: String },
    #[error("process session checkpoint actor state is invalid: {message}")]
    ActorCheckpointStateInvalid { message: String },
    #[error("process session checkpoint process liveness state is invalid: {message}")]
    ProcessLivenessCheckpointStateInvalid { message: String },
    #[error("process session checkpoint recovery record is invalid: {message}")]
    CheckpointRecoveryRecordInvalid { message: String },
    #[error(
        "process session checkpoint restore requires owned prepared program authority: {message}"
    )]
    OwnedPreparedProgramRequired { message: String },
    #[error("process session checkpoint pending activity region must match active region")]
    PendingActivityRegionMismatch {
        active_region_id: SessionLirRegionId,
        pending_region_id: SessionLirRegionId,
    },
    #[error(
        "process session checkpoint pending activity instruction must match active region program counter"
    )]
    PendingActivityInstructionMismatch {
        expected: InstructionIndex,
        observed: InstructionIndex,
    },
    #[error("process session checkpoint pending activity site is not admitted")]
    PendingActivitySiteMissing {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
        import_id: ImportId,
    },
    #[error("process session checkpoint pending activity site metadata drifted")]
    PendingActivitySiteMismatch { site_id: SessionLirDurableSiteId },
    #[error(
        "process session checkpoint activity site '{site_id}' carries volatile coroutine frame '{frame_id}' that is not resumable after restore"
    )]
    VolatileCoroutineFrameNotCheckpointResumable {
        site_id: SessionLirDurableSiteId,
        frame_id: String,
    },
    #[error(
        "process session checkpoint actor request continuation '{request_id}' carries a volatile awaiting-node frame that is not resumable after restore"
    )]
    VolatileActorRequestContinuationFrameNotCheckpointResumable {
        request_id: swarmvm_isa::ActorRequestId,
    },
    #[error("process session checkpoint pending activity no longer points at call_host")]
    PendingActivityInstructionNotCallHost {
        region_id: SessionLirRegionId,
        instruction_index: InstructionIndex,
    },
    #[error("process session checkpoint pending activity result register drifted")]
    PendingActivityResultDestinationMismatch {
        expected: RegisterIndex,
        observed: RegisterIndex,
    },
    #[error("process session checkpoint pending activity input register is unbound")]
    PendingActivityInputRegisterUnbound { register: RegisterIndex },
    #[error("process session checkpoint pending activity input drifted")]
    PendingActivityInputMismatch {
        expected: VmBoundaryValue,
        observed: VmBoundaryValue,
    },
    #[error(
        "process session checkpoint pending activity input could not materialize as boundary value: {message}"
    )]
    PendingActivityInputMaterializationFailed {
        register: RegisterIndex,
        message: String,
    },
}

#[cfg(test)]
mod drive_fault_tests {
    use super::*;

    fn variant_name(fault: &ProcessSessionDriveFault) -> &'static str {
        match fault {
            ProcessSessionDriveFault::CallLocalRegion { .. } => "call_local_region",
            ProcessSessionDriveFault::ActivityRequestBoundaryInputRequiresSealedProduct {
                ..
            } => "activity_request_boundary_input",
            ProcessSessionDriveFault::RegionLifecycle { operation, .. } => match operation {
                ProcessSessionRegionLifecycleOperation::ActorStartReceiverWrite => {
                    "region_actor_start_receiver_write"
                }
                ProcessSessionRegionLifecycleOperation::ApplySelectedCallHostResult => {
                    "region_apply_selected_call_host_result"
                }
                ProcessSessionRegionLifecycleOperation::SetRuntimeValueRegister => {
                    "region_set_runtime_value_register"
                }
                ProcessSessionRegionLifecycleOperation::EnterSelectedActorHandlerRegion => {
                    "region_enter_selected_actor_handler"
                }
                ProcessSessionRegionLifecycleOperation::RestoreSelectedActorHandlerRegion => {
                    "region_restore_selected_actor_handler"
                }
                ProcessSessionRegionLifecycleOperation::RestoreRootOrCallerReadyAwaiter => {
                    "region_restore_ready_awaiter"
                }
            },
        }
    }

    #[test]
    fn drive_fault_algebra_is_exhaustive_and_variants_remain_distinguishable() {
        let faults = [
            ProcessSessionDriveFault::CallLocalRegion {
                source: "local-call".to_owned(),
            },
            ProcessSessionDriveFault::ActivityRequestBoundaryInputRequiresSealedProduct {
                input_register:
                    RegisterIndex::provider_effect_input_for_swarmvm_runtime_types_owner_v1(),
            },
            ProcessSessionDriveFault::RegionLifecycle {
                operation:
                    ProcessSessionRegionLifecycleOperation::RestoreSelectedActorHandlerRegion,
                source: "restore".to_owned(),
            },
        ];
        assert_eq!(
            faults.iter().map(variant_name).collect::<Vec<_>>(),
            vec![
                "call_local_region",
                "activity_request_boundary_input",
                "region_restore_selected_actor_handler",
            ]
        );
    }

    #[test]
    fn drive_fault_projection_is_terminal_and_preserves_typed_details() {
        let local = ProcessSessionRunError::Drive {
            source: ProcessSessionDriveFault::CallLocalRegion {
                source: "selected-local-call-entry-failed".to_owned(),
            },
        };
        let local_details = local.diagnostic_details().expect("drive details");
        assert_eq!(local_details["kind"], "actor_scheduler");
        assert_eq!(
            local_details["parsed_message"]["kind"],
            "call_local_region_entry_failed"
        );
        assert_eq!(
            local_details["parsed_message"]["detail"],
            "selected-local-call-entry-failed"
        );

        let boundary = ProcessSessionRunError::Drive {
            source: ProcessSessionDriveFault::ActivityRequestBoundaryInputRequiresSealedProduct {
                input_register:
                    RegisterIndex::provider_effect_input_for_swarmvm_runtime_types_owner_v1(),
            },
        };
        let boundary_details = boundary.diagnostic_details().expect("boundary details");
        assert_eq!(
            boundary_details["kind"],
            "actor_scheduler_selected_activity_request_boundary_input_requires_sealed_product"
        );
        assert_ne!(local_details["kind"], boundary_details["kind"]);
    }

    #[test]
    fn drive_fault_constructors_preserve_typed_sources_without_projection() {
        assert_eq!(
            ProcessSessionDriveFault::region_lifecycle(
                ProcessSessionRegionLifecycleOperation::SetRuntimeValueRegister,
                "typed-upstream-detail".to_owned(),
            ),
            ProcessSessionDriveFault::RegionLifecycle {
                operation: ProcessSessionRegionLifecycleOperation::SetRuntimeValueRegister,
                source: "typed-upstream-detail".to_owned(),
            }
        );
    }
}
// Compiler-owned source-entrypoint executable runtime SCC.
