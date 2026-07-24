use super::provider_resume_lifecycle::*;
use super::session_route_lifecycle::*;
use super::*;

struct DirectRunProcessChildDriveEffectsV1 {
    observations:
        crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
    process_output_records:
        Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
}

impl DirectRunProcessChildDriveEffectsV1 {
    fn empty_for_process_kernel_owner_v1() -> Self {
        Self {
            observations: crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle::empty_for_direct_run_event_publication_owner_v1(),
            process_output_records: None,
        }
    }

    fn absorb_engine_result_for_process_kernel_owner_v1(
        &mut self,
        result: &mut EngineProcessSessionRunResultV1,
    ) {
        let (observations, process_output_records) =
            result.take_accumulated_drive_effects_for_process_session_result_owner_v1();
        self.observations
            .extend_for_direct_run_process_session_result_owner_v1(observations);
        self.absorb_process_output_records_for_process_kernel_owner_v1(process_output_records);
    }

    fn absorb_process_output_records_for_process_kernel_owner_v1(
        &mut self,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
    ) {
        let Some(process_output_records) = process_output_records else {
            return;
        };
        match &mut self.process_output_records {
            Some(existing) => {
                existing.extend_for_direct_run_process_child_owner_v1(process_output_records);
            }
            None => self.process_output_records = Some(process_output_records),
        }
    }

    fn absorb_process_child_terminal_observations_for_process_kernel_owner_v1(
        &mut self,
        observations: Vec<Value>,
    ) {
        self.observations
            .extend_for_direct_run_process_session_result_owner_v1(
                crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle::from_process_child_terminal_observations_for_process_kernel_owner_v1(
                    observations,
                ),
            );
    }

    fn into_parts_for_process_kernel_owner_v1(
        self,
    ) -> (
        crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
        Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
    ) {
        (self.observations, self.process_output_records)
    }
}

struct DirectRunProcessChildSessionFrameV1 {
    session: EngineLiveProcessSessionV1,
    drive_context: DirectRunProcessKernelChildDriveContext,
    effects: DirectRunProcessChildDriveEffectsV1,
}

struct DirectRunActiveProcessChildSessionV1 {
    frame: DirectRunProcessChildSessionFrameV1,
    result: EngineProcessSessionRunResultV1,
}

enum DirectRunNestedProcessChildResumeV1 {
    Invoke(crate::MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1),
    Run(crate::MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1),
}

struct DirectRunSuspendedProcessChildSessionV1 {
    frame: DirectRunProcessChildSessionFrameV1,
    resume: DirectRunNestedProcessChildResumeV1,
}

enum DirectRunProcessChildProviderTransitionV1 {
    Continue(DirectRunActiveProcessChildSessionV1),
    Descend {
        parent: DirectRunSuspendedProcessChildSessionV1,
        child: DirectRunActiveProcessChildSessionV1,
    },
    Refused(DirectRunProcessChildProviderRefusalV1),
    Fault(DirectRunProcessChildProviderFaultV1),
}

enum DirectRunProcessChildProviderFaultPhaseV1 {
    SelectedBoundaryTake,
    RouteSelection,
    ProcessLoadExecute,
    ProcessLoadCommit,
    ProcessCheckpointExecute,
    ProcessCheckpointCommit,
    ProcessRestoreExecute,
    ProcessRestoreCommit,
    ProcessInvokeContext,
    ProcessInvokePrepare,
    ProcessRunContext,
    ProcessRunPrepare,
    ProcessActivateContext,
    ProcessActivatePrepare,
    EventHostExecute,
}

enum DirectRunProcessChildProviderFaultV1 {
    Frame {
        frame: DirectRunProcessChildSessionFrameV1,
        phase: DirectRunProcessChildProviderFaultPhaseV1,
        failure: String,
    },
    ProcessInvokeAdmission {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: crate::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    ProcessRunAdmission {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: crate::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    ProcessActivateAdmission {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: crate::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    ProcessLoad {
        frame: DirectRunProcessChildSessionFrameV1,
        refusal: DirectRunProcessLoadExecutionRefusalV1,
    },
    ProcessCheckpoint {
        frame: DirectRunProcessChildSessionFrameV1,
        refusal: DirectRunProcessCheckpointExecutionRefusalV1,
    },
    ProcessRestore {
        frame: DirectRunProcessChildSessionFrameV1,
        refusal: DirectRunProcessRestoreExecutionRefusalV1,
    },
    ProcessInvokeStart {
        parent: DirectRunProcessChildSessionFrameV1,
        ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
        failure: String,
    },
    ProcessInvokeIngress {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
        failure: crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1,
    },
    ProcessInvokeBoundarySelection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
        failure: String,
    },
    ProcessInvokeBoundaryJoin {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    },
    ProcessRunStart {
        parent: DirectRunProcessChildSessionFrameV1,
        ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
        failure: String,
    },
    ProcessRunIngress {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
        failure: crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1,
    },
    ProcessRunBoundarySelection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
        failure: String,
    },
    ProcessRunBoundaryJoin {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    },
    HostAdmission {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: swarm_capability_model::CapabilitySdkError,
    },
    HostExecution {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: swarm_capability_model::CapabilitySdkError,
    },
    HostResultAdmission {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: swarm_capability_model::CapabilitySdkError,
    },
    ProviderDriveCommit {
        frame: DirectRunProcessChildSessionFrameV1,
        fault: crate::ProviderDriveSessionExecutionCommitFault,
    },
}

impl DirectRunProcessChildProviderFaultPhaseV1 {
    fn diagnostic_kind_for_process_kernel_owner_v1(&self) -> &'static str {
        match self {
            Self::SelectedBoundaryTake => "process_child_selected_boundary_take_fault",
            Self::RouteSelection => "process_child_provider_route_selection_fault",
            Self::ProcessLoadExecute => "process_child_process_load_execute_fault",
            Self::ProcessLoadCommit => "process_child_process_load_commit_fault",
            Self::ProcessCheckpointExecute => "process_child_process_checkpoint_execute_fault",
            Self::ProcessCheckpointCommit => "process_child_process_checkpoint_commit_fault",
            Self::ProcessRestoreExecute => "process_child_process_restore_execute_fault",
            Self::ProcessRestoreCommit => "process_child_process_restore_commit_fault",
            Self::ProcessInvokeContext => "process_child_process_invoke_context_fault",
            Self::ProcessInvokePrepare => "process_child_process_invoke_prepare_fault",
            Self::ProcessRunContext => "process_child_process_run_context_fault",
            Self::ProcessRunPrepare => "process_child_process_run_prepare_fault",
            Self::ProcessActivateContext => "process_child_process_activate_context_fault",
            Self::ProcessActivatePrepare => "process_child_process_activate_prepare_fault",
            Self::EventHostExecute => "process_child_event_host_execute_fault",
        }
    }
}

enum DirectRunProcessChildProviderRefusalV1 {
    InvokeIngress {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
        failure: crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1,
    },
    InvokeBoundaryJoin {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    },
    InvokeBoundarySelection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
        failure: String,
    },
    RunIngress {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
        failure: crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1,
    },
    RunBoundaryJoin {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    },
    RunBoundarySelection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunActiveProcessChildSessionV1,
        registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
        failure: String,
    },
}

struct DirectRunCompletedProcessChildSessionV1 {
    session: EngineLiveProcessSessionV1,
    drive_context: DirectRunProcessKernelChildDriveContext,
}

enum DirectRunProcessChildResumeRefusalV1 {
    Invoke {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure: crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1,
    },
    Run {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure: crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1,
    },
}

enum DirectRunProcessChildResumeFaultV1 {
    Invoke {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure: crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1,
    },
    Run {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure: crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1,
    },
}

enum DirectRunProcessChildPostTerminalRefusalV1 {
    InvokeCapture {
        boundary: crate::MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
        result: ProviderValue,
        event_publication_backend_output_drain_receipts: Vec<Value>,
        process_output_records:
            Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure: String,
    },
    RunCapture {
        boundary: crate::MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
        terminal: ProviderValue,
        process_output_records:
            Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure: String,
    },
}

struct DirectRunProcessInvokeChildStartFaultV1 {
    boundary: crate::MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    failure: String,
}

struct DirectRunProcessRunChildStartFaultV1 {
    boundary: crate::MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    failure: String,
}

enum DirectRunProcessChildOuterFaultV1 {
    InvokeSelection(DirectRunProcessInvokeExecutionSelectionFaultV1),
    RunSelection(DirectRunProcessRunChildSelectionFaultV1),
    InvokeStart(DirectRunProcessInvokeChildStartFaultV1),
    RunStart(DirectRunProcessRunChildStartFaultV1),
    InvokeTerminalProjection {
        boundary: crate::MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure:
            super::super::process_session_result_authority::DirectRunProcessChildTerminalProjectionFaultV1,
    },
    RunTerminalProjection {
        boundary: crate::MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure:
            super::super::process_session_result_authority::DirectRunProcessChildTerminalProjectionFaultV1,
    },
}

enum DirectRunProcessChildLoopFaultV1 {
    TerminalResultConversion {
        session: EngineLiveProcessSessionV1,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure: super::super::process_session_result_authority::EngineProcessSessionTerminalResultConversionFaultV1,
    },
    TerminalMaterialization {
        session: EngineLiveProcessSessionV1,
        drive_context: DirectRunProcessKernelChildDriveContext,
        failure: super::super::process_session_result_authority::EngineProcessSessionChildTerminalMaterializationFaultV1,
    },
    InvokeTerminalProjection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure:
            super::super::process_session_result_authority::DirectRunProcessChildTerminalProjectionFaultV1,
    },
    RunTerminalProjection {
        parent: DirectRunProcessChildSessionFrameV1,
        child: DirectRunCompletedProcessChildSessionV1,
        failure:
            super::super::process_session_result_authority::DirectRunProcessChildTerminalProjectionFaultV1,
    },
    ProviderBoundarySelection {
        frame: DirectRunProcessChildSessionFrameV1,
        failure: String,
    },
    HostResourceFinalizationSelection {
        frame: DirectRunProcessChildSessionFrameV1,
        failure: String,
    },
    HostResourceFinalizationCommit {
        frame: DirectRunProcessChildSessionFrameV1,
        failure: String,
    },
    WaitingOnLiveness {
        active: DirectRunActiveProcessChildSessionV1,
    },
    UnhandledOutcome {
        active: DirectRunActiveProcessChildSessionV1,
    },
}

enum DirectRunProcessChildOwnedRefusalV1 {
    Provider(DirectRunProcessChildProviderRefusalV1),
    ProviderFault(DirectRunProcessChildProviderFaultV1),
    Resume(DirectRunProcessChildResumeRefusalV1),
    ResumeFault(DirectRunProcessChildResumeFaultV1),
    PostTerminal(DirectRunProcessChildPostTerminalRefusalV1),
    LoopFault(DirectRunProcessChildLoopFaultV1),
}

pub(crate) struct DirectRunProcessChildDriveRefusalV1 {
    suspended: Vec<DirectRunSuspendedProcessChildSessionV1>,
    refusal: DirectRunProcessChildOwnedRefusalV1,
}

struct DirectRunProcessChildTerminalDriveProductV1 {
    terminal: DirectRunProcessChildTerminalMaterializationV1,
    drive_context: DirectRunProcessKernelChildDriveContext,
}

enum DirectRunProcessChildDriveOutcomeV1 {
    Terminal(DirectRunProcessChildTerminalDriveProductV1),
    RetryableRefused(DirectRunProcessChildDriveRefusalV1),
    Fault(DirectRunProcessChildDriveRefusalV1),
}

pub(crate) struct DirectRunProcessChildParentResumeFaultV1 {
    kind: DirectRunProcessChildParentResumeFaultKindV1,
}

enum DirectRunProcessChildParentResumeFaultKindV1 {
    InvokeCommit {
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        observations:
            crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
        process_output_records:
            Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
        failure:
            super::super::live_process_session_registry::DirectRunProcessInvokeLiveSessionResumeFaultV1,
    },
    InvokeAppend {
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        result: EngineProcessSessionRunResultV1,
        failure:
            super::super::live_process_session_registry::DirectRunPendingProcessChildEffectsAppendFaultV1,
    },
    RunCommit {
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        process_output_records:
            Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
        failure: super::super::live_process_session_registry::DirectRunProcessRunLiveSessionResumeFaultV1,
    },
    RunAppend {
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        result: EngineProcessSessionRunResultV1,
        failure:
            super::super::live_process_session_registry::DirectRunPendingProcessChildEffectsAppendFaultV1,
    },
}

impl DirectRunProcessChildParentResumeFaultV1 {
    fn invoke_commit(
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        observations: crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
        failure: super::super::live_process_session_registry::DirectRunProcessInvokeLiveSessionResumeFaultV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessChildParentResumeFaultKindV1::InvokeCommit {
                parent_route,
                observations,
                process_output_records,
                failure,
            },
        }
    }

    fn invoke_append(
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        result: EngineProcessSessionRunResultV1,
        failure: super::super::live_process_session_registry::DirectRunPendingProcessChildEffectsAppendFaultV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessChildParentResumeFaultKindV1::InvokeAppend {
                parent_route,
                result,
                failure,
            },
        }
    }

    fn run_commit(
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
        failure: super::super::live_process_session_registry::DirectRunProcessRunLiveSessionResumeFaultV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessChildParentResumeFaultKindV1::RunCommit {
                parent_route,
                process_output_records,
                failure,
            },
        }
    }

    fn run_append(
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        result: EngineProcessSessionRunResultV1,
        failure: super::super::live_process_session_registry::DirectRunPendingProcessChildEffectsAppendFaultV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessChildParentResumeFaultKindV1::RunAppend {
                parent_route,
                result,
                failure,
            },
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        let kind = match self.kind {
            DirectRunProcessChildParentResumeFaultKindV1::InvokeCommit {
                parent_route,
                observations,
                process_output_records,
                failure,
            } => {
                let _retained_authority =
                    (parent_route, observations, process_output_records, failure);
                "process_invoke_child_parent_resume_commit_fault"
            }
            DirectRunProcessChildParentResumeFaultKindV1::InvokeAppend {
                parent_route,
                result,
                failure,
            } => {
                let _retained_authority = (parent_route, result, failure);
                "process_invoke_child_parent_effect_append_fault"
            }
            DirectRunProcessChildParentResumeFaultKindV1::RunCommit {
                parent_route,
                process_output_records,
                failure,
            } => {
                let _retained_authority = (parent_route, process_output_records, failure);
                "process_run_child_parent_resume_commit_fault"
            }
            DirectRunProcessChildParentResumeFaultKindV1::RunAppend {
                parent_route,
                result,
                failure,
            } => {
                let _retained_authority = (parent_route, result, failure);
                "process_run_child_parent_effect_append_fault"
            }
        };
        json!({
            "kind": kind,
            "reason": "the process-child parent resume stage retained its complete route and child cargo after a typed owner fault",
        })
        .to_string()
    }
}

pub(crate) struct DirectRunProcessChildDriveFailureV1 {
    kind: DirectRunProcessChildDriveFailureKindV1,
}

enum DirectRunProcessChildDriveFailureKindV1 {
    RetryableRefused(DirectRunProcessChildDriveRefusalV1),
    Fault(DirectRunProcessChildDriveRefusalV1),
    OuterFault(DirectRunProcessChildOuterFaultV1),
}

impl DirectRunProcessChildDriveFailureV1 {
    fn retryable_refused_for_process_kernel_owner_v1(
        refusal: DirectRunProcessChildDriveRefusalV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessChildDriveFailureKindV1::RetryableRefused(refusal),
        }
    }

    fn fault_for_process_kernel_owner_v1(fault: DirectRunProcessChildDriveRefusalV1) -> Self {
        Self {
            kind: DirectRunProcessChildDriveFailureKindV1::Fault(fault),
        }
    }

    fn outer_fault_for_process_kernel_owner_v1(fault: DirectRunProcessChildOuterFaultV1) -> Self {
        Self {
            kind: DirectRunProcessChildDriveFailureKindV1::OuterFault(fault),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        match self.kind {
            DirectRunProcessChildDriveFailureKindV1::RetryableRefused(refusal)
            | DirectRunProcessChildDriveFailureKindV1::Fault(refusal) => {
                refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1()
            }
            DirectRunProcessChildDriveFailureKindV1::OuterFault(fault) => {
                fault.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1()
            }
        }
    }
}

impl DirectRunProcessChildOuterFaultV1 {
    fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(self) -> String {
        let (kind, reason) = match self {
            Self::InvokeSelection(fault) => match fault {
                DirectRunProcessInvokeExecutionSelectionFaultV1::Unmatched { boundary } => {
                    let _retained_boundary = boundary;
                    (
                        "process_invoke_execution_registration_unmatched",
                        "the selected process.invoke await boundary did not join any exact registered execution authority".to_owned(),
                    )
                }
                DirectRunProcessInvokeExecutionSelectionFaultV1::RegistryUnavailable {
                    boundary,
                } => {
                    let _retained_boundary = boundary;
                    (
                        "process_invoke_execution_registry_unavailable",
                        "the process.invoke execution registry was unavailable while selecting the exact await boundary".to_owned(),
                    )
                }
                DirectRunProcessInvokeExecutionSelectionFaultV1::RegistryBorrowRetryExhausted {
                    boundary,
                } => {
                    let _retained_boundary = boundary;
                    (
                        "process_invoke_execution_registry_borrow_retry_exhausted",
                        "the process.invoke execution registry remained borrowed while selecting the exact await boundary".to_owned(),
                    )
                }
            },
            Self::RunSelection(fault) => match fault {
                DirectRunProcessRunChildSelectionFaultV1::Unmatched { boundary } => {
                    let _retained_boundary = boundary;
                    (
                        "process_run_child_registration_unmatched",
                        "the selected process.run terminal boundary did not join any exact registered child authority".to_owned(),
                    )
                }
                DirectRunProcessRunChildSelectionFaultV1::RegistryUnavailable { boundary } => {
                    let _retained_boundary = boundary;
                    (
                        "process_run_child_registry_unavailable",
                        "the process.run child registry was unavailable while selecting the exact terminal boundary".to_owned(),
                    )
                }
                DirectRunProcessRunChildSelectionFaultV1::RegistryBorrowRetryExhausted {
                    boundary,
                } => {
                    let _retained_boundary = boundary;
                    (
                        "process_run_child_registry_borrow_retry_exhausted",
                        "the process.run child registry remained borrowed while selecting the exact terminal boundary".to_owned(),
                    )
                }
            },
            Self::InvokeStart(DirectRunProcessInvokeChildStartFaultV1 { boundary, failure }) => {
                let _retained_boundary = boundary;
                ("process_invoke_child_start_fault", failure)
            }
            Self::RunStart(DirectRunProcessRunChildStartFaultV1 { boundary, failure }) => {
                let _retained_boundary = boundary;
                ("process_run_child_start_fault", failure)
            }
            Self::InvokeTerminalProjection {
                boundary,
                drive_context,
                failure,
            } => {
                let _retained_authority = (boundary, drive_context);
                (
                    "process_invoke_child_outer_terminal_projection_fault",
                    failure
                        .consume_into_rejected_terminal_and_message_for_process_kernel_owner_v1()
                        .1,
                )
            }
            Self::RunTerminalProjection {
                boundary,
                drive_context,
                failure,
            } => {
                let _retained_authority = (boundary, drive_context);
                (
                    "process_run_child_outer_terminal_projection_fault",
                    failure
                        .consume_into_rejected_terminal_and_message_for_process_kernel_owner_v1()
                        .1,
                )
            }
        };
        json!({ "kind": kind, "reason": reason }).to_string()
    }
}

impl DirectRunProcessChildDriveRefusalV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        let Self { suspended, refusal } = self;
        let nested_depth = suspended.len();
        let (kind, reason) = match refusal {
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::InvokeIngress {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the exact process.invoke ingress transition was refused before the direct-run boundary committed".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::InvokeBoundaryJoin {
                    parent,
                    child,
                    boundary,
                    registration,
                },
            ) => {
                let _retained_authority = (parent, child, boundary, registration);
                (
                    "nested_process_invoke_exact_resume_correspondence_mismatch",
                    "the process.invoke resume boundary did not join the registration minted with its committed nominal ingress".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::RunIngress {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the exact process.run ingress transition was refused before the direct-run boundary committed".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::InvokeBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    "nested_process_invoke_resume_boundary_selection_failed",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::RunBoundaryJoin {
                    parent,
                    child,
                    boundary,
                    registration,
                },
            ) => {
                let _retained_authority = (parent, child, boundary, registration);
                (
                    "nested_process_run_exact_resume_correspondence_mismatch",
                    "the process.run resume boundary did not join the registration minted with its committed nominal ingress".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Provider(
                DirectRunProcessChildProviderRefusalV1::RunBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    "nested_process_run_resume_boundary_selection_failed",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::Frame {
                    frame,
                    phase,
                    failure,
                },
            ) => {
                let _retained_frame = frame;
                (
                    phase.diagnostic_kind_for_process_kernel_owner_v1(),
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeAdmission { frame, fault },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_invoke_admission_fault",
                    fault.to_string(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRunAdmission { frame, fault },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_run_admission_fault",
                    fault.to_string(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessActivateAdmission { frame, fault },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_activate_admission_fault",
                    fault.to_string(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessLoad { frame, refusal },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_load_execution_refused",
                    refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessCheckpoint { frame, refusal },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_checkpoint_execution_refused",
                    refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRestore { frame, refusal },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_process_restore_execution_refused",
                    refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeStart {
                    parent,
                    ingress,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, ingress, registration);
                ("process_child_process_invoke_start_fault", failure.clone())
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeIngress {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the committed process.invoke ingress transition faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    "process_child_process_invoke_boundary_selection_fault",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeBoundaryJoin {
                    parent,
                    child,
                    boundary,
                    registration,
                },
            ) => {
                let _retained_authority = (parent, child, boundary, registration);
                (
                    "process_child_process_invoke_boundary_join_fault",
                    "the committed process.invoke boundary did not join its retained registration"
                        .to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRunStart {
                    parent,
                    ingress,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, ingress, registration);
                ("process_child_process_run_start_fault", failure.clone())
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRunIngress {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the committed process.run ingress transition faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRunBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, child, registration);
                (
                    "process_child_process_run_boundary_selection_fault",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProcessRunBoundaryJoin {
                    parent,
                    child,
                    boundary,
                    registration,
                },
            ) => {
                let _retained_authority = (parent, child, boundary, registration);
                (
                    "process_child_process_run_boundary_join_fault",
                    "the committed process.run boundary did not join its retained registration"
                        .to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::HostAdmission { frame, fault },
            ) => {
                let _retained_frame = frame;
                ("process_child_host_admission_fault", fault.to_string())
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::HostExecution { frame, fault },
            ) => {
                let _retained_frame = frame;
                ("process_child_host_execution_fault", fault.to_string())
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::HostResultAdmission { frame, fault },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_host_result_admission_fault",
                    fault.to_string(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ProviderFault(
                DirectRunProcessChildProviderFaultV1::ProviderDriveCommit { frame, fault },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_provider_drive_commit_fault",
                    fault.to_string(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Resume(
                DirectRunProcessChildResumeRefusalV1::Invoke {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the exact process.invoke parent resume transition was refused or its committed continuation drive faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::Resume(
                DirectRunProcessChildResumeRefusalV1::Run {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the exact process.run parent resume transition was refused or its committed continuation drive faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ResumeFault(
                DirectRunProcessChildResumeFaultV1::Invoke {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the committed process.invoke parent resume transition faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::ResumeFault(
                DirectRunProcessChildResumeFaultV1::Run {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    failure.diagnostic_kind_for_direct_run_owner_v1(),
                    "the committed process.run parent resume transition faulted".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::TerminalResultConversion {
                    session,
                    drive_context,
                    failure,
                },
            ) => {
                let _retained_authority = (session, drive_context);
                (
                    "process_child_terminal_result_conversion_fault",
                    failure.consume_into_message_for_direct_run_boundary_owner_v1(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::TerminalMaterialization {
                    session,
                    drive_context,
                    failure,
                },
            ) => {
                let _retained_authority = (session, drive_context);
                (
                    "process_child_terminal_materialization_fault",
                    failure.consume_into_message_for_process_kernel_owner_v1(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::InvokeTerminalProjection {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    "process_child_invoke_terminal_projection_fault",
                    failure
                        .consume_into_rejected_terminal_and_message_for_process_kernel_owner_v1()
                        .1,
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::RunTerminalProjection {
                    parent,
                    child,
                    failure,
                },
            ) => {
                let _retained_authority = (parent, &child.session, &child.drive_context);
                (
                    "process_child_run_terminal_projection_fault",
                    failure
                        .consume_into_rejected_terminal_and_message_for_process_kernel_owner_v1()
                        .1,
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::ProviderBoundarySelection { frame, failure },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_provider_boundary_selection_fault",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::HostResourceFinalizationSelection {
                    frame,
                    failure,
                },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_host_finalization_selection_fault",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::HostResourceFinalizationCommit { frame, failure },
            ) => {
                let _retained_frame = frame;
                (
                    "process_child_host_finalization_commit_fault",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::WaitingOnLiveness { active },
            ) => {
                let _retained_active = active;
                (
                    "process_child_waiting_on_liveness_without_closed_driver_route",
                    "a closed process child reached a liveness wait that cannot be delegated outside its owning iterative child stack".to_owned(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::LoopFault(
                DirectRunProcessChildLoopFaultV1::UnhandledOutcome { active },
            ) => {
                let outcome_kind = active.result.outcome_kind();
                let _retained_active = active;
                (
                    "process_child_outcome_kind_without_iterative_driver_route",
                    format!(
                        "the iterative process-child driver reached outcome {outcome_kind} with no finite route"
                    ),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::PostTerminal(
                DirectRunProcessChildPostTerminalRefusalV1::InvokeCapture {
                    boundary,
                    result,
                    event_publication_backend_output_drain_receipts,
                    process_output_records,
                    drive_context,
                    failure,
                },
            ) => {
                let _retained_authority = (
                    boundary,
                    result,
                    event_publication_backend_output_drain_receipts,
                    process_output_records,
                    drive_context,
                );
                (
                    "process_invoke_child_terminal_capture_refused",
                    failure.clone(),
                )
            }
            DirectRunProcessChildOwnedRefusalV1::PostTerminal(
                DirectRunProcessChildPostTerminalRefusalV1::RunCapture {
                    boundary,
                    terminal,
                    process_output_records,
                    drive_context,
                    failure,
                },
            ) => {
                let _retained_authority =
                    (boundary, terminal, process_output_records, drive_context);
                (
                    "process_run_child_terminal_capture_refused",
                    failure.clone(),
                )
            }
        };
        let rendered = json!({
            "kind": kind,
            "reason": reason,
            "nested_depth": nested_depth,
        })
        .to_string();
        let _settled_suspended_sessions = suspended;
        rendered
    }
}

fn admit_process_child_engine_result_for_process_kernel_owner_v1(
    session: &EngineLiveProcessSessionV1,
    outcome: crate::DirectRunProcessSessionRunResultProductV1,
    boundary_context: &'static str,
) -> EngineProcessSessionRunResultV1 {
    EngineProcessSessionRunResultV1::admitted(
        outcome,
        None,
        crate::direct_run::process_creation_export_readiness_for_live_process_session_owner_v1(
            session,
            boundary_context,
        ),
        boundary_context,
    )
}

fn start_process_invoke_child_session_for_process_kernel_owner_v1(
    matched: MatchedRegisteredProcessInvokeExecution,
) -> Result<
    (
        DirectRunActiveProcessChildSessionV1,
        crate::MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    ),
    DirectRunProcessInvokeChildStartFaultV1,
> {
    let MatchedRegisteredProcessInvokeExecution {
        boundary,
        execution,
        drive_context,
    } = matched;
    let active = match start_prepared_process_invoke_child_session_for_process_kernel_owner_v1(
        execution,
        drive_context,
    ) {
        Ok(active) => active,
        Err(failure) => {
            return Err(DirectRunProcessInvokeChildStartFaultV1 { boundary, failure });
        }
    };
    Ok((active, boundary))
}

fn start_prepared_process_invoke_child_session_for_process_kernel_owner_v1(
    execution: crate::direct_run::DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority,
    drive_context: DirectRunProcessKernelChildDriveContext,
) -> Result<DirectRunActiveProcessChildSessionV1, String> {
    let mut session = execution
        .open_child_session_for_process_invoke_owner_v1(drive_context.current_process())?;
    let outcome =
        session.drive_process_session_until_external_boundary_for_session_runtime_owner_v1()?;
    let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
        &session,
        outcome,
        "process_invoke_child_initial_drive",
    );
    Ok(DirectRunActiveProcessChildSessionV1 {
        frame: DirectRunProcessChildSessionFrameV1 {
            session,
            drive_context,
            effects: DirectRunProcessChildDriveEffectsV1::empty_for_process_kernel_owner_v1(),
        },
        result,
    })
}

fn start_process_run_child_session_for_process_kernel_owner_v1(
    matched: MatchedRegisteredProcessRunChild,
) -> Result<
    (
        DirectRunActiveProcessChildSessionV1,
        crate::MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    ),
    DirectRunProcessRunChildStartFaultV1,
> {
    let MatchedRegisteredProcessRunChild {
        boundary,
        execution,
        drive_context,
    } = matched;
    let active_result = match execution {
        RegisteredProcessRunChildExecutionV1::ProcessRun(execution) => {
            start_prepared_process_run_child_session_for_process_kernel_owner_v1(
                execution,
                drive_context,
            )
        }
        RegisteredProcessRunChildExecutionV1::ProcessActivate {
            execution,
            lifecycle_recovery,
        } => start_prepared_process_activate_child_session_for_process_kernel_owner_v1(
            execution,
            lifecycle_recovery,
            drive_context,
        ),
    };
    let active = match active_result {
        Ok(active) => active,
        Err(failure) => {
            return Err(DirectRunProcessRunChildStartFaultV1 { boundary, failure });
        }
    };
    Ok((active, boundary))
}

fn start_prepared_process_run_child_session_for_process_kernel_owner_v1(
    execution: crate::direct_run::DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority,
    drive_context: DirectRunProcessKernelChildDriveContext,
) -> Result<DirectRunActiveProcessChildSessionV1, String> {
    let mut session =
        execution.open_child_session_for_process_run_owner_v1(drive_context.current_process())?;
    let outcome =
        session.drive_process_session_until_external_boundary_for_session_runtime_owner_v1()?;
    let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
        &session,
        outcome,
        "process_run_child_initial_drive",
    );
    Ok(DirectRunActiveProcessChildSessionV1 {
        frame: DirectRunProcessChildSessionFrameV1 {
            session,
            drive_context,
            effects: DirectRunProcessChildDriveEffectsV1::empty_for_process_kernel_owner_v1(),
        },
        result,
    })
}

fn start_prepared_process_activate_child_session_for_process_kernel_owner_v1(
    execution: DirectRunProcessActivateChildExecutionV1,
    lifecycle_recovery: super::super::process_kernel_boundary::DirectRunProcessActivateLifecycleRecoveryV1,
    drive_context: DirectRunProcessKernelChildDriveContext,
) -> Result<DirectRunActiveProcessChildSessionV1, String> {
    let opened = match execution {
        DirectRunProcessActivateChildExecutionV1::Prepared(execution) => {
            execution.open_child_session_for_process_activate_owner_v1()
        }
        DirectRunProcessActivateChildExecutionV1::RetryOpen(refusal) => {
            refusal.retry_for_process_activate_owner_v1()
        }
    };
    let mut session = match opened {
        Ok(session) => session,
        Err(refusal) => {
            retain_process_activate_open_refusal_for_process_lifecycle_owner_v1(
                super::super::process_kernel_boundary::MatchedRegisteredProcessActivateOpenRefusalV1 {
                    authority: lifecycle_recovery.authority,
                    activation_process_carrier: lifecycle_recovery.activation_process_carrier,
                    refusal,
                    checkpoint_state: lifecycle_recovery.checkpoint_state,
                },
            )?;
            return Err(
                "process.activate exact child open remains in retryable lifecycle custody"
                    .to_owned(),
            );
        }
    };
    let outcome =
        session.drive_process_session_until_external_boundary_for_session_runtime_owner_v1()?;
    let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
        &session,
        outcome,
        "process_activate_child_initial_drive",
    );
    Ok(DirectRunActiveProcessChildSessionV1 {
        frame: DirectRunProcessChildSessionFrameV1 {
            session,
            drive_context,
            effects: DirectRunProcessChildDriveEffectsV1::empty_for_process_kernel_owner_v1(),
        },
        result,
    })
}

fn commit_process_child_provider_drive_result_for_process_kernel_owner_v1(
    session: &mut EngineLiveProcessSessionV1,
    provider_drive_result: ProviderDriveResult,
) -> Result<EngineProcessSessionRunResultV1, crate::ProviderDriveSessionExecutionCommitFault> {
    let (outcome, output_effect_drain_receipts) = provider_drive_result
        .commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1(
            session,
            "direct_process_child_provider_resume_ready_output",
        )?;
    let (observations, process_output_records) =
        crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle::from_provider_drive_output_effect_drain_receipts_and_process_output_records_for_direct_run_event_publication_owner_v1(
            output_effect_drain_receipts,
        );
    Ok(
        admit_process_child_engine_result_for_process_kernel_owner_v1(
            session,
            outcome,
            "direct_process_child_provider_resume_ready_output",
        )
        .with_event_publication_backend_output_drain_observations_for_direct_run_owner_v1(
            observations,
        )
        .with_provider_process_output_records_for_direct_run_owner_v1(process_output_records),
    )
}

fn continue_process_child_provider_transition_for_process_kernel_owner_v1(
    frame: DirectRunProcessChildSessionFrameV1,
    result: EngineProcessSessionRunResultV1,
) -> DirectRunProcessChildProviderTransitionV1 {
    DirectRunProcessChildProviderTransitionV1::Continue(DirectRunActiveProcessChildSessionV1 {
        frame,
        result,
    })
}

fn complete_prepared_process_invoke_descent_for_process_kernel_owner_v1(
    mut parent: DirectRunProcessChildSessionFrameV1,
    ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    child: DirectRunActiveProcessChildSessionV1,
) -> DirectRunProcessChildProviderTransitionV1 {
    let outcome = match parent
        .session
        .commit_process_invoke_execution_provider_ingress_and_drive_for_direct_run_owner_v1(ingress)
    {
        Ok(outcome) => outcome,
        Err(failure) => {
            let retryable = matches!(
                &failure,
                crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::NoPendingBoundary { .. }
                    | crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary { .. }
                    | crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch { .. }
            );
            return if retryable {
                DirectRunProcessChildProviderTransitionV1::Refused(
                    DirectRunProcessChildProviderRefusalV1::InvokeIngress {
                        parent,
                        child,
                        registration,
                        failure,
                    },
                )
            } else {
                DirectRunProcessChildProviderTransitionV1::Fault(
                    DirectRunProcessChildProviderFaultV1::ProcessInvokeIngress {
                        parent,
                        child,
                        registration,
                        failure,
                    },
                )
            };
        }
    };
    let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
        &parent.session,
        outcome,
        "direct_nested_process_invoke_nominal_provider_ingress",
    );
    let selected_boundary = match result.into_selected_process_invoke_await_execution_boundary() {
        Ok(boundary) => boundary,
        Err(failure) => {
            return DirectRunProcessChildProviderTransitionV1::Fault(
                DirectRunProcessChildProviderFaultV1::ProcessInvokeBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            );
        }
    };
    match selected_boundary.try_join_registration_for_durable_direct_run_owner_v1(registration) {
        crate::ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1::Joined(boundary) => {
            DirectRunProcessChildProviderTransitionV1::Descend {
                parent: DirectRunSuspendedProcessChildSessionV1 {
                    frame: parent,
                    resume: DirectRunNestedProcessChildResumeV1::Invoke(boundary),
                },
                child,
            }
        }
        crate::ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1::Unmatched {
            boundary,
            registration,
        } => DirectRunProcessChildProviderTransitionV1::Fault(
            DirectRunProcessChildProviderFaultV1::ProcessInvokeBoundaryJoin {
                parent,
                child,
                boundary,
                registration,
            },
        ),
    }
}

fn complete_prepared_process_run_descent_for_process_kernel_owner_v1(
    mut parent: DirectRunProcessChildSessionFrameV1,
    ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
    registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    child: DirectRunActiveProcessChildSessionV1,
    boundary_context: &'static str,
) -> DirectRunProcessChildProviderTransitionV1 {
    let outcome = match parent
        .session
        .commit_process_run_child_provider_ingress_and_drive_for_direct_run_owner_v1(ingress)
    {
        Ok(outcome) => outcome,
        Err(failure) => {
            let retryable = matches!(
                &failure,
                crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::NoPendingBoundary { .. }
                    | crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary { .. }
                    | crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch { .. }
            );
            return if retryable {
                DirectRunProcessChildProviderTransitionV1::Refused(
                    DirectRunProcessChildProviderRefusalV1::RunIngress {
                        parent,
                        child,
                        registration,
                        failure,
                    },
                )
            } else {
                DirectRunProcessChildProviderTransitionV1::Fault(
                    DirectRunProcessChildProviderFaultV1::ProcessRunIngress {
                        parent,
                        child,
                        registration,
                        failure,
                    },
                )
            };
        }
    };
    let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
        &parent.session,
        outcome,
        boundary_context,
    );
    let selected_boundary = match result.into_selected_process_run_drive_terminal_boundary() {
        Ok(boundary) => boundary,
        Err(failure) => {
            return DirectRunProcessChildProviderTransitionV1::Fault(
                DirectRunProcessChildProviderFaultV1::ProcessRunBoundarySelection {
                    parent,
                    child,
                    registration,
                    failure,
                },
            );
        }
    };
    match selected_boundary.try_join_registration_for_durable_direct_run_owner_v1(registration) {
        crate::ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1::Joined(boundary) => {
            DirectRunProcessChildProviderTransitionV1::Descend {
                parent: DirectRunSuspendedProcessChildSessionV1 {
                    frame: parent,
                    resume: DirectRunNestedProcessChildResumeV1::Run(boundary),
                },
                child,
            }
        }
        crate::ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1::Unmatched {
            boundary,
            registration,
        } => DirectRunProcessChildProviderTransitionV1::Fault(
            DirectRunProcessChildProviderFaultV1::ProcessRunBoundaryJoin {
                parent,
                child,
                boundary,
                registration,
            },
        ),
    }
}

fn drive_process_child_selected_provider_boundary_for_process_kernel_owner_v1(
    mut frame: DirectRunProcessChildSessionFrameV1,
    selected_boundary: crate::SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> DirectRunProcessChildProviderTransitionV1 {
    macro_rules! string_fault {
        ($result:expr, $phase:expr) => {
            match $result {
                Ok(value) => value,
                Err(failure) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::Frame {
                            frame,
                            phase: $phase,
                            failure,
                        },
                    );
                }
            }
        };
    }

    let selected_provider_input = string_fault!(
        frame
            .session
            .take_selected_provider_resume_host_input_for_direct_run_owner_v1(selected_boundary),
        DirectRunProcessChildProviderFaultPhaseV1::SelectedBoundaryTake
    );
    let selected_route = string_fault!(
        select_provider_resume_route_for_direct_run_owner_v1(selected_provider_input),
        DirectRunProcessChildProviderFaultPhaseV1::RouteSelection
    );
    match selected_route {
        DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected_input) => {
            let admitted_load = string_fault!(
                frame
                    .session
                    .admit_selected_process_load_child_launch_for_direct_run_owner_v1(
                        selected_input,
                    )
                    .map_err(|fault| fault.to_string()),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessLoadExecute
            );
            let provider_drive_result =
                match execute_kernel_internal_process_load_with_static_child_context_for_provider_resume_owner_v1(
                    admitted_load
                        .commit_into_process_lifecycle_registration_for_direct_run_owner_v1(),
                    frame.drive_context.root_scope_id(),
                    frame.drive_context.live_process_session_id(),
                    frame.drive_context.current_process(),
                ) {
                    Ok(provider_drive_result) => provider_drive_result,
                    Err(refusal) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProcessLoad { frame, refusal },
                        );
                    }
                };
            let result =
                match commit_process_child_provider_drive_result_for_process_kernel_owner_v1(
                    &mut frame.session,
                    provider_drive_result,
                ) {
                    Ok(result) => result,
                    Err(fault) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProviderDriveCommit {
                                frame,
                                fault,
                            },
                        );
                    }
                };
            continue_process_child_provider_transition_for_process_kernel_owner_v1(frame, result)
        }
        DirectRunSelectedProviderResumeRouteV1::ProcessCheckpoint(selected_input) => {
            let provider_drive_result =
                match execute_kernel_internal_process_checkpoint_for_provider_resume_owner_v1(
                    selected_input,
                ) {
                    Ok(provider_drive_result) => provider_drive_result,
                    Err(refusal) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProcessCheckpoint {
                                frame,
                                refusal,
                            },
                        );
                    }
                };
            let result =
                match commit_process_child_provider_drive_result_for_process_kernel_owner_v1(
                    &mut frame.session,
                    provider_drive_result,
                ) {
                    Ok(result) => result,
                    Err(fault) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProviderDriveCommit {
                                frame,
                                fault,
                            },
                        );
                    }
                };
            continue_process_child_provider_transition_for_process_kernel_owner_v1(frame, result)
        }
        DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected_input) => {
            let provider_drive_result =
                match execute_kernel_internal_process_restore_with_static_child_context_for_provider_resume_owner_v1(
                    selected_input,
                ) {
                    Ok(provider_drive_result) => provider_drive_result,
                    Err(refusal) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProcessRestore {
                                frame,
                                refusal,
                            },
                        );
                    }
                };
            let result =
                match commit_process_child_provider_drive_result_for_process_kernel_owner_v1(
                    &mut frame.session,
                    provider_drive_result,
                ) {
                    Ok(result) => result,
                    Err(fault) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProviderDriveCommit {
                                frame,
                                fault,
                            },
                        );
                    }
                };
            continue_process_child_provider_transition_for_process_kernel_owner_v1(frame, result)
        }
        DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected_input) => {
            let nested_drive_context = string_fault!(
                frame
                    .drive_context
                    .duplicate_for_nested_process_child_owner_v1(),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessInvokeContext
            );
            let admitted_launch = match frame
                .session
                .admit_selected_process_invoke_child_launch_for_direct_run_owner_v1(selected_input)
            {
                Ok(admitted_launch) => admitted_launch,
                Err(fault) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::ProcessInvokeAdmission {
                            frame,
                            fault,
                        },
                    );
                }
            };
            let PreparedProcessInvokeProviderIngressV1 {
                ingress,
                registration,
                execution,
            } = string_fault!(
                prepare_process_invoke_provider_ingress_for_process_kernel_owner_v1(
                    admitted_launch,
                ),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessInvokePrepare
            );
            let child =
                match start_prepared_process_invoke_child_session_for_process_kernel_owner_v1(
                    execution,
                    nested_drive_context,
                ) {
                    Ok(child) => child,
                    Err(failure) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProcessInvokeStart {
                                parent: frame,
                                ingress,
                                registration,
                                failure,
                            },
                        );
                    }
                };
            complete_prepared_process_invoke_descent_for_process_kernel_owner_v1(
                frame,
                ingress,
                registration,
                child,
            )
        }
        DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected_input) => {
            let nested_drive_context = string_fault!(
                frame
                    .drive_context
                    .duplicate_for_nested_process_child_owner_v1(),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessRunContext
            );
            let admitted_launch = match frame
                .session
                .admit_selected_process_run_child_launch_for_direct_run_owner_v1(selected_input)
            {
                Ok(admitted_launch) => admitted_launch,
                Err(fault) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::ProcessRunAdmission { frame, fault },
                    );
                }
            };
            let PreparedProcessRunProviderIngressV1 {
                ingress,
                registration,
                execution,
            } = string_fault!(
                prepare_process_run_provider_ingress_for_process_kernel_owner_v1(admitted_launch,),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessRunPrepare
            );
            let child = match start_prepared_process_run_child_session_for_process_kernel_owner_v1(
                execution,
                nested_drive_context,
            ) {
                Ok(child) => child,
                Err(failure) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::ProcessRunStart {
                            parent: frame,
                            ingress,
                            registration,
                            failure,
                        },
                    );
                }
            };
            complete_prepared_process_run_descent_for_process_kernel_owner_v1(
                frame,
                ingress,
                registration,
                child,
                "direct_nested_process_run_nominal_provider_ingress",
            )
        }
        DirectRunSelectedProviderResumeRouteV1::ProcessActivate(selected_input) => {
            let nested_drive_context = string_fault!(
                frame
                    .drive_context
                    .duplicate_for_nested_process_child_owner_v1(),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessActivateContext
            );
            let PreparedProcessActivateProviderIngressV1 {
                ingress,
                registration,
                execution,
                lifecycle_recovery,
            } = string_fault!(
                prepare_process_activate_provider_ingress_for_process_kernel_owner_v1(
                    selected_input,
                ),
                DirectRunProcessChildProviderFaultPhaseV1::ProcessActivatePrepare
            );
            let child =
                match start_prepared_process_activate_child_session_for_process_kernel_owner_v1(
                    execution,
                    lifecycle_recovery,
                    nested_drive_context,
                ) {
                    Ok(child) => child,
                    Err(failure) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProcessRunStart {
                                parent: frame,
                                ingress,
                                registration,
                                failure,
                            },
                        );
                    }
                };
            complete_prepared_process_run_descent_for_process_kernel_owner_v1(
                frame,
                ingress,
                registration,
                child,
                "direct_nested_process_activate_nominal_provider_ingress",
            )
        }
        DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected_host_input) => {
            let admitted_request = match selected_host_input
                .admit_host_typed_request_for_direct_run_provider_resume_owner_v1(
                    provider_execution_session,
                ) {
                Ok(admitted_request) => admitted_request,
                Err(fault) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::HostAdmission { frame, fault },
                    );
                }
            };
            let execution_result = if swarm_event_provider_requires_product_session_boundary(
                admitted_request.provider_id(),
            ) {
                string_fault!(
                    crate::direct_run::event::DirectRunEventProductOwner::execute_selected_product_session_provider_effect_for_direct_run_provider_resume_owner_v1(
                    frame.drive_context.root_scope_id(),
                    frame.drive_context.live_process_session_id(),
                    frame.drive_context.node_id(),
                    admitted_request,
                    &durable_execution_core::EventAppendOccurredAtClock::from_run_started_at(
                        frame.drive_context.started_at(),
                    ),
                    ),
                    DirectRunProcessChildProviderFaultPhaseV1::EventHostExecute
                )
            } else {
                match provider_execution_session
                    .invoke_selected_provider_boundary_request_for_direct_run_owner_v1(
                        admitted_request,
                    ) {
                    Ok(execution_result) => execution_result,
                    Err(fault) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::HostExecution { frame, fault },
                        );
                    }
                }
            };
            let provider_drive_result = match ProviderDriveResult::ready_from_rust_sdk_static_provider_execution_result_for_provider_drive_result_owner_v1(execution_result) {
                Ok(provider_drive_result) => provider_drive_result,
                Err(fault) => {
                    return DirectRunProcessChildProviderTransitionV1::Fault(
                        DirectRunProcessChildProviderFaultV1::HostResultAdmission { frame, fault },
                    );
                }
            };
            let result =
                match commit_process_child_provider_drive_result_for_process_kernel_owner_v1(
                    &mut frame.session,
                    provider_drive_result,
                ) {
                    Ok(result) => result,
                    Err(fault) => {
                        return DirectRunProcessChildProviderTransitionV1::Fault(
                            DirectRunProcessChildProviderFaultV1::ProviderDriveCommit {
                                frame,
                                fault,
                            },
                        );
                    }
                };
            continue_process_child_provider_transition_for_process_kernel_owner_v1(frame, result)
        }
    }
}

fn drive_process_child_session_iteratively_for_process_kernel_owner_v1(
    mut active: DirectRunActiveProcessChildSessionV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> DirectRunProcessChildDriveOutcomeV1 {
    let mut suspended = Vec::<DirectRunSuspendedProcessChildSessionV1>::new();
    loop {
        let DirectRunActiveProcessChildSessionV1 {
            mut frame,
            mut result,
        } = active;
        frame
            .effects
            .absorb_engine_result_for_process_kernel_owner_v1(&mut result);
        match result.outcome_kind() {
            "completed" | "failed" | "terminal_completed" | "terminal_failed" => {
                let DirectRunProcessChildSessionFrameV1 {
                    session,
                    drive_context,
                    effects,
                } = frame;
                let (observations, process_output_records) =
                    effects.into_parts_for_process_kernel_owner_v1();
                let terminal_product = match result
                    .with_accumulated_drive_effects_for_process_session_result_owner_v1(
                        observations,
                        process_output_records,
                    )
                    .into_terminal_result_product_for_direct_run_process_session_result_route_owner_v1(
                        "iterative_process_child_terminal",
                    ) {
                    Ok(terminal_product) => terminal_product,
                    Err(failure) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                    DirectRunProcessChildLoopFaultV1::TerminalResultConversion {
                                        session,
                                        drive_context,
                                        failure,
                                    },
                                ),
                            },
                        );
                    }
                };
                let terminal = match terminal_product
                    .into_process_child_terminal_materialization_for_process_kernel_owner_v1(
                        "iterative_process_child_terminal",
                    ) {
                    Ok(terminal) => terminal,
                    Err(failure) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                    DirectRunProcessChildLoopFaultV1::TerminalMaterialization {
                                        session,
                                        drive_context,
                                        failure,
                                    },
                                ),
                            },
                        );
                    }
                };
                let Some(parent) = suspended.pop() else {
                    return DirectRunProcessChildDriveOutcomeV1::Terminal(
                        DirectRunProcessChildTerminalDriveProductV1 {
                            terminal,
                            drive_context,
                        },
                    );
                };
                let completed_child = DirectRunCompletedProcessChildSessionV1 {
                    session,
                    drive_context,
                };
                let DirectRunSuspendedProcessChildSessionV1 {
                    frame: mut parent_frame,
                    resume,
                } = parent;
                let outcome = match resume {
                    DirectRunNestedProcessChildResumeV1::Invoke(boundary) => {
                        let (
                            value,
                            child_event_publication_backend_output_drain_receipts,
                            child_process_output_records,
                        ) = match terminal.into_process_invoke_result_for_process_kernel_owner_v1()
                        {
                            Ok(parts) => parts,
                            Err(failure) => {
                                return DirectRunProcessChildDriveOutcomeV1::Fault(
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                            DirectRunProcessChildLoopFaultV1::InvokeTerminalProjection {
                                                parent: parent_frame,
                                                child: completed_child,
                                                failure,
                                            },
                                        ),
                                    },
                                );
                            }
                        };
                        parent_frame
                            .effects
                            .absorb_process_child_terminal_observations_for_process_kernel_owner_v1(
                                child_event_publication_backend_output_drain_receipts,
                            );
                        parent_frame
                            .effects
                            .absorb_process_output_records_for_process_kernel_owner_v1(
                                child_process_output_records,
                            );
                        let resume = boundary.admit_result_for_durable_direct_run_owner_v1(value);
                        match parent_frame.session
                            .commit_process_invoke_await_execution_resume_and_drive_for_direct_run_owner_v1(
                                resume,
                            )
                        {
                            Ok(outcome) => outcome,
                            Err(failure) => {
                                let retryable = matches!(
                                    &failure,
                                    crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::NoPendingBoundary { .. }
                                        | crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary { .. }
                                        | crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch { .. }
                                );
                                let refusal = if retryable {
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::Resume(
                                                DirectRunProcessChildResumeRefusalV1::Invoke {
                                                    parent: parent_frame,
                                                    child: completed_child,
                                                    failure,
                                                },
                                            ),
                                    }
                                } else {
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::ResumeFault(
                                                DirectRunProcessChildResumeFaultV1::Invoke {
                                                    parent: parent_frame,
                                                    child: completed_child,
                                                    failure,
                                                },
                                            ),
                                    }
                                };
                                return if retryable {
                                    DirectRunProcessChildDriveOutcomeV1::RetryableRefused(refusal)
                                } else {
                                    DirectRunProcessChildDriveOutcomeV1::Fault(refusal)
                                };
                            }
                        }
                    }
                    DirectRunNestedProcessChildResumeV1::Run(boundary) => {
                        let (terminal, child_process_output_records) = match terminal
                            .into_process_run_terminal_for_process_kernel_owner_v1(
                                completed_child.drive_context.current_process(),
                            ) {
                            Ok(parts) => parts,
                            Err(failure) => {
                                return DirectRunProcessChildDriveOutcomeV1::Fault(
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                            DirectRunProcessChildLoopFaultV1::RunTerminalProjection {
                                                parent: parent_frame,
                                                child: completed_child,
                                                failure,
                                            },
                                        ),
                                    },
                                );
                            }
                        };
                        parent_frame
                            .effects
                            .absorb_process_output_records_for_process_kernel_owner_v1(
                                child_process_output_records,
                            );
                        let resume =
                            boundary.admit_terminal_for_durable_direct_run_owner_v1(terminal);
                        match parent_frame.session
                            .commit_process_run_drive_terminal_resume_and_drive_for_direct_run_owner_v1(
                                resume,
                            )
                        {
                            Ok(outcome) => outcome,
                            Err(failure) => {
                                let retryable = matches!(
                                    &failure,
                                    crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::NoPendingBoundary { .. }
                                        | crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary { .. }
                                        | crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch { .. }
                                );
                                let refusal = if retryable {
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::Resume(
                                                DirectRunProcessChildResumeRefusalV1::Run {
                                                    parent: parent_frame,
                                                    child: completed_child,
                                                    failure,
                                                },
                                            ),
                                    }
                                } else {
                                    DirectRunProcessChildDriveRefusalV1 {
                                        suspended,
                                        refusal: DirectRunProcessChildOwnedRefusalV1::ResumeFault(
                                                DirectRunProcessChildResumeFaultV1::Run {
                                                    parent: parent_frame,
                                                    child: completed_child,
                                                    failure,
                                                },
                                            ),
                                    }
                                };
                                return if retryable {
                                    DirectRunProcessChildDriveOutcomeV1::RetryableRefused(refusal)
                                } else {
                                    DirectRunProcessChildDriveOutcomeV1::Fault(refusal)
                                };
                            }
                        }
                    }
                };
                let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
                    &parent_frame.session,
                    outcome,
                    "iterative_nested_process_child_resume",
                );
                active = DirectRunActiveProcessChildSessionV1 {
                    frame: parent_frame,
                    result,
                };
            }
            "needs_host_activity_effect" | "blocked_needs_host_activity_effect" => {
                let selected_boundary = match result
                    .into_selected_provider_resume_boundary_for_direct_run_process_session_result_route_owner_v1(
                        "iterative_process_child_provider_boundary",
                    ) {
                    Ok(selected_boundary) => selected_boundary,
                    Err(failure) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                    DirectRunProcessChildLoopFaultV1::ProviderBoundarySelection {
                                        frame,
                                        failure,
                                    },
                                ),
                            },
                        );
                    }
                };
                match drive_process_child_selected_provider_boundary_for_process_kernel_owner_v1(
                    frame,
                    selected_boundary,
                    provider_execution_session,
                ) {
                    DirectRunProcessChildProviderTransitionV1::Continue(next) => active = next,
                    DirectRunProcessChildProviderTransitionV1::Descend { parent, child } => {
                        suspended.push(parent);
                        active = child;
                    }
                    DirectRunProcessChildProviderTransitionV1::Refused(refusal) => {
                        return DirectRunProcessChildDriveOutcomeV1::RetryableRefused(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::Provider(refusal),
                            },
                        );
                    }
                    DirectRunProcessChildProviderTransitionV1::Fault(fault) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::ProviderFault(fault),
                            },
                        );
                    }
                }
            }
            "needs_host_resource_finalization" | "blocked_needs_host_resource_finalization" => {
                let selected_boundary = match result
                    .into_selected_host_resource_finalization_boundary()
                {
                    Ok(selected_boundary) => selected_boundary,
                    Err(failure) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                    DirectRunProcessChildLoopFaultV1::HostResourceFinalizationSelection {
                                        frame,
                                        failure,
                                    },
                                ),
                            },
                        );
                    }
                };
                let outcome = match frame
                    .session
                    .commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1(
                        provider_execution_session,
                        selected_boundary,
                    ) {
                    Ok(outcome) => outcome,
                    Err(fault) => {
                        return DirectRunProcessChildDriveOutcomeV1::Fault(
                            DirectRunProcessChildDriveRefusalV1 {
                                suspended,
                                refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                                    DirectRunProcessChildLoopFaultV1::HostResourceFinalizationCommit {
                                        frame,
                                        failure: fault.to_string(),
                                    },
                                ),
                            },
                        );
                    }
                };
                let result = admit_process_child_engine_result_for_process_kernel_owner_v1(
                    &frame.session,
                    outcome,
                    "iterative_process_child_host_resource_finalization",
                );
                active = DirectRunActiveProcessChildSessionV1 { frame, result };
            }
            "waiting_on_liveness" | "blocked_waiting_on_liveness" => {
                return DirectRunProcessChildDriveOutcomeV1::Fault(
                    DirectRunProcessChildDriveRefusalV1 {
                        suspended,
                        refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                            DirectRunProcessChildLoopFaultV1::WaitingOnLiveness {
                                active: DirectRunActiveProcessChildSessionV1 { frame, result },
                            },
                        ),
                    },
                );
            }
            _other => {
                return DirectRunProcessChildDriveOutcomeV1::Fault(
                    DirectRunProcessChildDriveRefusalV1 {
                        suspended,
                        refusal: DirectRunProcessChildOwnedRefusalV1::LoopFault(
                            DirectRunProcessChildLoopFaultV1::UnhandledOutcome {
                                active: DirectRunActiveProcessChildSessionV1 { frame, result },
                            },
                        ),
                    },
                );
            }
        }
    }
}

pub(in crate::direct_run) fn commit_process_invoke_resume_and_route_parent_for_process_kernel_owner_v1(
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    resume: crate::ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    event_publication_backend_output_drain_observations: crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
    process_output_records: Option<
        crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    >,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, DirectRunProcessSessionDriveFaultV1> {
    match parent_route {
        DirectRunProcessKernelBoundaryParentRouteV1::Start {
            process_session_start_token,
            execution_substrate,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_invoke_await_execution_resume_for_live_process_session(
                    process_session_start_token.live_process_session_id(),
                    process_session_start_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::invoke_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::Start { process_session_start_token, execution_substrate },
                            event_publication_backend_output_drain_observations,
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_child_effects_for_live_process_session(
                process_session_start_token.live_process_session_id(),
                process_session_start_token.root_scope_id(),
                event_publication_backend_output_drain_observations,
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::invoke_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::Start { process_session_start_token, execution_substrate },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_start_owner(
                process_session_start_token,
                execution_substrate,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
        DirectRunProcessKernelBoundaryParentRouteV1::Reawaken {
            process_session_reawaken_token,
            execution_substrate,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_invoke_await_execution_resume_for_live_process_session(
                    process_session_reawaken_token.live_process_session_id(),
                    process_session_reawaken_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::invoke_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::Reawaken { process_session_reawaken_token, execution_substrate },
                            event_publication_backend_output_drain_observations,
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_child_effects_for_live_process_session(
                process_session_reawaken_token.live_process_session_id(),
                process_session_reawaken_token.root_scope_id(),
                event_publication_backend_output_drain_observations,
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::invoke_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::Reawaken { process_session_reawaken_token, execution_substrate },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_reawaken_owner(
                process_session_reawaken_token,
                execution_substrate,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
        DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume {
            provider_resume_token,
            provider_resume_private_storage,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_invoke_await_execution_resume_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::invoke_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume { provider_resume_token, provider_resume_private_storage },
                            event_publication_backend_output_drain_observations,
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_child_effects_for_live_process_session(
                provider_resume_token.live_process_session_id(),
                provider_resume_token.root_scope_id(),
                event_publication_backend_output_drain_observations,
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::invoke_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume { provider_resume_token, provider_resume_private_storage },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_provider_resume_owner(
                provider_resume_token,
                provider_resume_private_storage,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
    }
}

pub(in crate::direct_run) fn drive_process_control_and_route_parent_for_process_kernel_owner_v1(
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessControlBoundaryForDirectRunOwnerV1,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let (process, completion) = selected_boundary.consume_for_durable_direct_run_owner_v1();
    let MatchedRegisteredProcessLifecycle {
        authority,
        subject: _subject,
        activation_process_carrier,
        loaded_process,
        open_plan,
        checkpoint_state,
    } = select_process_lifecycle_for_process_kernel_owner_v1(process)?;

    let control = match completion.control_for_direct_run_owner_v1() {
        crate::ProcessControlKindForDirectRunOwnerV1::TerminateGracefully => {
            crate::ProcessLivenessProcessControlKindV1::TerminateGracefully
        }
        crate::ProcessControlKindForDirectRunOwnerV1::TerminateForcefully => {
            crate::ProcessLivenessProcessControlKindV1::TerminateForcefully
        }
        crate::ProcessControlKindForDirectRunOwnerV1::Interrupt => {
            crate::ProcessLivenessProcessControlKindV1::Interrupt
        }
    };
    let children = match completion.children_for_direct_run_owner_v1() {
        crate::ProcessControlChildPolicyForDirectRunOwnerV1::FailIfChildren => {
            crate::ProcessLivenessProcessControlChildPolicyV1::FailIfChildren
        }
        crate::ProcessControlChildPolicyForDirectRunOwnerV1::Cascade => {
            crate::ProcessLivenessProcessControlChildPolicyV1::Cascade
        }
        crate::ProcessControlChildPolicyForDirectRunOwnerV1::TransferToInit => {
            crate::ProcessLivenessProcessControlChildPolicyV1::TransferToInit
        }
    };
    let operation =
        crate::ProcessLivenessProcessControlOperationV1::from_exact_checkpoint_for_process_liveness_owner_v1(
            checkpoint_state,
            control,
            children,
        );
    let receipt = match crate::apply_process_liveness_process_control_operation_v1(operation) {
        Ok(receipt) => receipt,
        Err(fault) => {
            let rejected_checkpoint_state =
                fault.into_rejected_checkpoint_state_for_process_liveness_owner_v1();
            register_process_lifecycle_after_control_for_process_kernel_owner_v1(
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                rejected_checkpoint_state,
            )?;
            return Err(json!({
                "kind": "process_control_liveness_transition_refused",
                "reason": "the process-liveness owner lawfully refused the exact process-control transition",
            })
            .to_string());
        }
    };
    let process_id = receipt
        .process_id_for_process_control_observation_v1()
        .to_owned();
    let root_scope_id = receipt
        .process_root_scope_id_for_process_control_observation_v1()
        .to_owned();
    let next_checkpoint_state = receipt.into_next_checkpoint_state_for_process_liveness_owner_v1();
    register_process_lifecycle_after_control_for_process_kernel_owner_v1(
        authority,
        activation_process_carrier,
        loaded_process,
        open_plan,
        next_checkpoint_state,
    )?;
    let resume =
        completion.admit_owner_control_receipt_for_direct_run_owner_v1(process_id, root_scope_id);
    commit_process_control_resume_and_route_parent_for_process_kernel_owner_v1(parent_route, resume)
}

fn commit_process_control_resume_and_route_parent_for_process_kernel_owner_v1(
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    resume: crate::ProcessControlResumeProductForDirectRunOwnerV1,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    match parent_route {
        DirectRunProcessKernelBoundaryParentRouteV1::Start {
            process_session_start_token,
            execution_substrate,
        } => {
            let result = DirectRunRuntimeAuthorityOwner::commit_process_control_resume_for_live_process_session(
                process_session_start_token.live_process_session_id(),
                process_session_start_token.root_scope_id(),
                resume,
            )?;
            route_engine_process_session_result_for_public_aperture_start_owner(
                process_session_start_token,
                execution_substrate,
                result,
            )
        }
        DirectRunProcessKernelBoundaryParentRouteV1::Reawaken {
            process_session_reawaken_token,
            execution_substrate,
        } => {
            let result = DirectRunRuntimeAuthorityOwner::commit_process_control_resume_for_live_process_session(
                process_session_reawaken_token.live_process_session_id(),
                process_session_reawaken_token.root_scope_id(),
                resume,
            )?;
            route_engine_process_session_result_for_public_aperture_reawaken_owner(
                process_session_reawaken_token,
                execution_substrate,
                result,
            )
        }
        DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume {
            provider_resume_token,
            provider_resume_private_storage,
        } => {
            let result = DirectRunRuntimeAuthorityOwner::commit_process_control_resume_for_live_process_session(
                provider_resume_token.live_process_session_id(),
                provider_resume_token.root_scope_id(),
                resume,
            )?;
            route_engine_process_session_result_for_public_aperture_provider_resume_owner(
                provider_resume_token,
                provider_resume_private_storage,
                result,
            )
        }
    }
}

pub(in crate::direct_run) fn commit_process_run_resume_and_route_parent_for_process_kernel_owner_v1(
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    resume: crate::ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    process_output_records: Option<
        crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    >,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, DirectRunProcessSessionDriveFaultV1> {
    match parent_route {
        DirectRunProcessKernelBoundaryParentRouteV1::Start {
            process_session_start_token,
            execution_substrate,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_run_drive_terminal_resume_for_live_process_session(
                    process_session_start_token.live_process_session_id(),
                    process_session_start_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::run_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::Start { process_session_start_token, execution_substrate },
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_output_records_for_live_process_session(
                process_session_start_token.live_process_session_id(),
                process_session_start_token.root_scope_id(),
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::run_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::Start { process_session_start_token, execution_substrate },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_start_owner(
                process_session_start_token,
                execution_substrate,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
        DirectRunProcessKernelBoundaryParentRouteV1::Reawaken {
            process_session_reawaken_token,
            execution_substrate,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_run_drive_terminal_resume_for_live_process_session(
                    process_session_reawaken_token.live_process_session_id(),
                    process_session_reawaken_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::run_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::Reawaken { process_session_reawaken_token, execution_substrate },
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_output_records_for_live_process_session(
                process_session_reawaken_token.live_process_session_id(),
                process_session_reawaken_token.root_scope_id(),
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::run_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::Reawaken { process_session_reawaken_token, execution_substrate },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_reawaken_owner(
                process_session_reawaken_token,
                execution_substrate,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
        DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume {
            provider_resume_token,
            provider_resume_private_storage,
        } => {
            let result = match
                DirectRunRuntimeAuthorityOwner::commit_process_run_drive_terminal_resume_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    resume,
                ) {
                    Ok(result) => result,
                    Err(failure) => return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                        DirectRunProcessChildParentResumeFaultV1::run_commit(
                            DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume { provider_resume_token, provider_resume_private_storage },
                            process_output_records,
                            failure,
                        ),
                    )),
                };
            if let Err(failure) = DirectRunRuntimeAuthorityOwner::append_pending_process_output_records_for_live_process_session(
                provider_resume_token.live_process_session_id(),
                provider_resume_token.root_scope_id(),
                process_output_records,
            ) {
                return Err(DirectRunProcessSessionDriveFaultV1::ProcessChildParentResume(
                    DirectRunProcessChildParentResumeFaultV1::run_append(
                        DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume { provider_resume_token, provider_resume_private_storage },
                        result,
                        failure,
                    ),
                ));
            }
            route_engine_process_session_result_for_public_aperture_provider_resume_owner(
                provider_resume_token,
                provider_resume_private_storage,
                result,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
        }
    }
}

pub(in crate::direct_run) fn drive_selected_process_invoke_child_to_matching_resume_for_process_kernel_owner_v1(
    selected_boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> Result<DirectRunDrivenProcessInvokeChildProductV1, DirectRunProcessChildDriveFailureV1> {
    let matched = select_process_invoke_execution_for_process_kernel_owner_v1(selected_boundary)
        .map_err(|fault| {
            DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
                DirectRunProcessChildOuterFaultV1::InvokeSelection(fault),
            )
        })?;
    let (active, boundary) = start_process_invoke_child_session_for_process_kernel_owner_v1(
        matched,
    )
    .map_err(|fault| {
        DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
            DirectRunProcessChildOuterFaultV1::InvokeStart(fault),
        )
    })?;
    let drive_outcome = drive_process_child_session_iteratively_for_process_kernel_owner_v1(
        active,
        provider_execution_session,
    );
    let terminal_product = match drive_outcome {
        DirectRunProcessChildDriveOutcomeV1::Terminal(terminal) => terminal,
        DirectRunProcessChildDriveOutcomeV1::RetryableRefused(refusal) => {
            return Err(
                DirectRunProcessChildDriveFailureV1::retryable_refused_for_process_kernel_owner_v1(
                    refusal,
                ),
            );
        }
        DirectRunProcessChildDriveOutcomeV1::Fault(fault) => {
            return Err(
                DirectRunProcessChildDriveFailureV1::fault_for_process_kernel_owner_v1(fault),
            );
        }
    };
    let DirectRunProcessChildTerminalDriveProductV1 {
        terminal,
        drive_context,
    } = terminal_product;
    let (result, event_publication_backend_output_drain_receipts, process_output_records) =
        match terminal.into_process_invoke_result_for_process_kernel_owner_v1() {
            Ok(parts) => parts,
            Err(failure) => {
                return Err(
                    DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
                        DirectRunProcessChildOuterFaultV1::InvokeTerminalProjection {
                            boundary,
                            drive_context,
                            failure,
                        },
                    ),
                );
            }
        };
    if let Err(failure) =
        admit_process_child_output_to_body_local_test_capture_for_process_kernel_owner_v1(
            provider_execution_session,
            process_output_records.as_ref(),
        )
    {
        return Err(
            DirectRunProcessChildDriveFailureV1::fault_for_process_kernel_owner_v1(
                DirectRunProcessChildDriveRefusalV1 {
                    suspended: Vec::new(),
                    refusal: DirectRunProcessChildOwnedRefusalV1::PostTerminal(
                        DirectRunProcessChildPostTerminalRefusalV1::InvokeCapture {
                            boundary,
                            result,
                            event_publication_backend_output_drain_receipts,
                            process_output_records,
                            drive_context,
                            failure,
                        },
                    ),
                },
            ),
        );
    }
    let resume = boundary.admit_result_for_durable_direct_run_owner_v1(result);
    Ok(
        DirectRunDrivenProcessInvokeChildProductV1::from_matching_child_drive_for_process_kernel_owner_v1(
            resume,
            crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle::from_process_child_terminal_observations_for_process_kernel_owner_v1(
                event_publication_backend_output_drain_receipts,
            ),
            process_output_records,
        ),
    )
}

pub(in crate::direct_run) fn drive_selected_process_run_child_to_matching_resume_for_process_kernel_owner_v1(
    selected_boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> Result<DirectRunDrivenProcessRunChildProductV1, DirectRunProcessChildDriveFailureV1> {
    let matched = select_process_run_child_for_process_kernel_owner_v1(selected_boundary).map_err(
        |fault| {
            DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
                DirectRunProcessChildOuterFaultV1::RunSelection(fault),
            )
        },
    )?;
    let (active, boundary) = start_process_run_child_session_for_process_kernel_owner_v1(matched)
        .map_err(|fault| {
        DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
            DirectRunProcessChildOuterFaultV1::RunStart(fault),
        )
    })?;
    let drive_outcome = drive_process_child_session_iteratively_for_process_kernel_owner_v1(
        active,
        provider_execution_session,
    );
    let terminal_product = match drive_outcome {
        DirectRunProcessChildDriveOutcomeV1::Terminal(terminal) => terminal,
        DirectRunProcessChildDriveOutcomeV1::RetryableRefused(refusal) => {
            return Err(
                DirectRunProcessChildDriveFailureV1::retryable_refused_for_process_kernel_owner_v1(
                    refusal,
                ),
            );
        }
        DirectRunProcessChildDriveOutcomeV1::Fault(fault) => {
            return Err(
                DirectRunProcessChildDriveFailureV1::fault_for_process_kernel_owner_v1(fault),
            );
        }
    };
    let DirectRunProcessChildTerminalDriveProductV1 {
        terminal,
        drive_context,
    } = terminal_product;
    let (terminal, process_output_records) = match terminal
        .into_process_run_terminal_for_process_kernel_owner_v1(drive_context.current_process())
    {
        Ok(parts) => parts,
        Err(failure) => {
            return Err(
                DirectRunProcessChildDriveFailureV1::outer_fault_for_process_kernel_owner_v1(
                    DirectRunProcessChildOuterFaultV1::RunTerminalProjection {
                        boundary,
                        drive_context,
                        failure,
                    },
                ),
            );
        }
    };
    if let Err(failure) =
        admit_process_child_output_to_body_local_test_capture_for_process_kernel_owner_v1(
            provider_execution_session,
            process_output_records.as_ref(),
        )
    {
        return Err(
            DirectRunProcessChildDriveFailureV1::fault_for_process_kernel_owner_v1(
                DirectRunProcessChildDriveRefusalV1 {
                    suspended: Vec::new(),
                    refusal: DirectRunProcessChildOwnedRefusalV1::PostTerminal(
                        DirectRunProcessChildPostTerminalRefusalV1::RunCapture {
                            boundary,
                            terminal,
                            process_output_records,
                            drive_context,
                            failure,
                        },
                    ),
                },
            ),
        );
    }
    let resume = boundary.admit_terminal_for_durable_direct_run_owner_v1(terminal);
    Ok(
        DirectRunDrivenProcessRunChildProductV1::from_matching_child_drive_for_process_kernel_owner_v1(
            resume,
            process_output_records,
        ),
    )
}

fn admit_process_child_output_to_body_local_test_capture_for_process_kernel_owner_v1(
    provider_execution_session: &mut ProviderHostExecutionSession,
    process_output_records: Option<
        &crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    >,
) -> Result<(), String> {
    let Some(process_output_records) = process_output_records else {
        return Ok(());
    };
    let observations = process_output_records
        .body_local_process_output_observations_for_direct_run_process_child_owner_v1();
    match provider_execution_session
        .admit_body_local_process_output_observations_for_direct_run_process_child_owner_v1(
            observations,
        )
        .map_err(|fault| fault.to_string())?
    {
        swarm_rust_sdk_static_provider_host::RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1::ObservedByBodyLocalStaticTestExecutor
        | swarm_rust_sdk_static_provider_host::RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1::BodyLocalStaticTestExecutorAbsent => Ok(()),
    }
}
