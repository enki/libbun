
// compiler-custody: symbol=SelectedActorRequestContinuationOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedActorRequestContinuationOperation {
    site: ExecutablePlanSite,
    prepared: Arc<PreparedActorRequestContinuationOperation>,
    executable_contract: Arc<CompilerExactProviderExecutableCommandContractForExecutableImageOwnerV1>,
    input: crate::SessionRuntimeMaterializedActivityInputPayloadProduct,
    contract: CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
    binding: crate::session::execution_kernel::provider_effect_runtime::ProviderEffectExecutableActorRequestBindingWithResultCommitForSessionExecutionKernelOwnerV1,
}

/// Exact suspended provider boundary retained inside linear execution
/// custody.  The host result can only consume this value together with the
/// correspondence half minted for the same selected boundary; cursor and
/// result-binding metadata never become independently replayable.
#[must_use = "a pending executable provider boundary must be consumed by its corresponded ready output"]
pub(in crate::session) struct PendingExecutableProviderBoundary {
    site: ExecutablePlanSite,
    pending_activity: crate::session::work_runtime::PendingActivityEffectFrame,
    pending_output_authority:
        swarm_capability_model::PendingProviderBoundaryOutputCommitAuthority,
    result_mode: swarmvm_isa_types::HostActivityResultMode,
    result_commit: crate::session::execution_kernel::provider_effect_runtime::ProviderEffectExecutableResultCommitForSessionExecutionKernelOwnerV1,
    continuation: ExecutableCursorTarget,
    observable_effect_obligation:
        ProviderBoundaryObservableEffectExecutionObligationForSessionExecutionKernelOwnerV1,
}

pub(in crate::session) enum ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1
{
    Committed,
    Unmatched {
        pending: PendingExecutableProviderBoundary,
        ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    },
    Fault {
        source: crate::ProcessNominalProviderIngressCommitFaultV1,
    },
}

pub(in crate::session) enum ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1 {
    Committed,
    Unmatched {
        pending: PendingExecutableProviderBoundary,
        ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
    },
    Fault {
        source: crate::ProcessNominalProviderIngressCommitFaultV1,
    },
}

/// The sole suspended executable-kernel boundary slot. Provider readiness,
/// invoke-frontier awaiting, and child-terminal driving are mutually exclusive
/// states by construction; no pair of optional side rails can coexist.
#[must_use = "a pending executable kernel boundary must be resumed by its exact owner"]
pub(in crate::session) enum PendingExecutableKernelBoundary {
    Root(PendingTerminalSettlementCustody),
    Provider(PendingExecutableProviderBoundary),
    ProcessInvokeAwaitExecution(
        crate::PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1,
    ),
    ProcessRunDriveTerminal(
        crate::PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1,
    ),
    ProcessControl(crate::PendingProcessControlResumeForSessionExecutionKernelOwnerV1),
}

impl PendingExecutableKernelBoundary {
    pub(in crate::session) fn diagnostic_kind(&self) -> &'static str {
        match self {
            Self::Root(_) => "root_terminal_settlement",
            Self::Provider(_) => "provider",
            Self::ProcessInvokeAwaitExecution(_) => "process_invoke_await_execution",
            Self::ProcessRunDriveTerminal(_) => "process_run_drive_terminal",
            Self::ProcessControl(_) => "process_control",
        }
    }

    pub(in crate::session) fn record_live_heap_roots_for_checkpoint_owner_v1<'a>(
        &'a self,
        roots: &mut Vec<&'a crate::RuntimeValue>,
    ) {
        if let Self::Root(custody) = self {
            custody.record_live_heap_roots_for_checkpoint_owner_v1(roots);
        }
    }

    pub(in crate::session) fn record_runtime_handle_restore_requirements_for_checkpoint_owner_v1(
        &self,
        requirements: &mut Vec<
            crate::VmRuntimeHandleRestoreRequirementsForSwarmvmSessionRuntimeOwnerV1,
        >,
    ) {
        let mut roots = Vec::new();
        self.record_live_heap_roots_for_checkpoint_owner_v1(&mut roots);
        requirements.extend(roots.into_iter().map(|value| {
            value.runtime_handle_restore_requirements_for_swarmvm_session_runtime_owner_v1()
        }));
    }
}

/// Exact caller continuation parked in the actor store while one checked
/// actor request is in flight. The result destination was resolved at image
/// open and the selected command contract is retained as the non-forgeable
/// witness for this invocation; neither can be reconstructed from a request
/// id, slot, register snapshot, or Session-LIR coordinate.
#[must_use = "a parked actor-request continuation must be consumed by its ready reply"]
pub(crate) struct ParkedActorRequestContinuation {
    site: ExecutablePlanSite,
    result_commit: crate::session::execution_state::ParkedActorRequestResultCommit,
    _provider_contract: CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
}

#[must_use = "a prepared parent actor reply application must commit or cancel exactly once"]
// compiler-custody: symbol=PreparedParentActorReplyReadyApplication disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: compiler_custody_coverage.py must admit runtime-only authority prepared by ParkedActorRequestContinuation::prepare_ready_result after compiler-root close"
pub(crate) struct PreparedParentActorReplyReadyApplication {
    site: ExecutablePlanSite,
    result_commit: crate::session::execution_state::PreparedActorRequestReadyResultCommit,
    heap_admission: crate::session::execution_kernel::executable_value::PreparedActorRequestReadyHeapAdmissionForSessionExecutionKernelOwnerV1,
    provider_contract: CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
}

#[must_use = "a refused parent actor reply application retains the parked continuation"]
pub(crate) struct RefusedParentActorReplyReadyApplication {
    parked: ParkedActorRequestContinuation,
    fault: KernelExecutionFault,
}

impl RefusedParentActorReplyReadyApplication {
    pub(crate) fn into_parked_and_fault(
        self,
    ) -> (ParkedActorRequestContinuation, KernelExecutionFault) {
        (self.parked, self.fault)
    }
}

impl fmt::Debug for ParkedActorRequestContinuation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParkedActorRequestContinuation")
            .field("site", &self.site)
            .field("result_commit", &"sealed")
            .field("provider_contract", &"sealed")
            .finish()
    }
}

impl PendingExecutableProviderBoundary {
    pub(in crate::session) fn pending_activity_for_observation(
        &self,
    ) -> &crate::session::work_runtime::PendingActivityEffectFrame {
        &self.pending_activity
    }

    pub(in crate::session) fn consume_corresponded_ready_output(
        self,
        ready_output: swarm_capability_model::ProviderReadyBoundaryOutput,
        host_resource_releases: swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseTransferSetV1,
        runtime_heap: &mut crate::SessionRuntimeHeapOwner,
        execution_state: &mut crate::session::execution_state::KernelExecutionState,
        cursor: &mut ExecutableCursor,
    ) -> Result<(), crate::ProviderBoundaryIngressFault> {
        let Self {
            site: _,
            pending_activity: _,
            pending_output_authority,
            result_mode,
            result_commit,
            continuation,
            observable_effect_obligation: _observable_effect_obligation,
        } = self;
        let provider_output = pending_output_authority
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready_output)
            .map_err(crate::ProviderBoundaryIngressFault::from)?;
        let runtime_output = provider_output
            .try_map_payload_for_session_runtime_owner_v1(|provider_output| {
                runtime_heap
                    .admit_corresponded_provider_output_transactionally_for_session_execution_kernel_owner_v1(
                        provider_output,
                        host_resource_releases,
                    )
            })
            .map_err(crate::ProviderBoundaryIngressFault::from)?;
        let settlement = crate::session::result_carrier_owner::SessionRuntimeProviderBoundarySettlement::from_correlated_provider_boundary_output(
            runtime_output,
        );
        execution_state
            .commit_corresponded_provider_effect_settlement(
                runtime_heap,
                result_mode,
                result_commit,
                settlement,
            )
            .map_err(crate::ProviderBoundaryExecutionCommitFault::from_provider_execution_frame)?;
        cursor.commit_target(&continuation);
        Ok(())
    }

    pub(in crate::session) fn try_commit_process_invoke_execution_provider_ingress(
        self,
        ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
        _runtime_heap: &crate::SessionRuntimeHeapOwner,
        execution_state: &mut crate::session::execution_state::KernelExecutionState,
        cursor: &mut ExecutableCursor,
    ) -> ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1 {
        let Self {
            site,
            pending_activity,
            pending_output_authority,
            result_mode,
            result_commit,
            continuation,
            observable_effect_obligation,
        } = self;
        let matched = match ingress
            .try_join_pending_output_authority_for_session_execution_kernel_owner_v1(
                pending_output_authority,
            ) {
            crate::ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1::Joined(
                matched,
            ) => matched,
            crate::ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending_output_authority,
                ingress,
            } => {
                return ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Unmatched {
                    pending: Self {
                        site,
                        pending_activity,
                        pending_output_authority,
                        result_mode,
                        result_commit,
                        continuation,
                        observable_effect_obligation,
                    },
                    ingress,
                };
            }
        };
        let output = match matched.into_nominal_output_for_session_execution_kernel_owner_v1() {
            Ok(output) => output,
            Err(source) => {
                return ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault { source };
            }
        };
        let prepared = match execution_state
            .prepare_nominal_process_provider_effect_result_for_session_execution_kernel_owner_v1(
                result_mode,
                result_commit,
                output,
            ) {
            Ok(prepared) => prepared,
            Err(source) => {
                return ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault {
                    source: crate::ProcessNominalProviderIngressCommitFaultV1::execution_commit_for_session_execution_kernel_owner_v1(
                        crate::ProviderBoundaryExecutionCommitFault::from_provider_execution_frame(source),
                    ),
                };
            }
        };
        prepared.commit_for_session_execution_kernel_owner_v1();
        cursor.commit_target(&continuation);
        ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Committed
    }

    pub(in crate::session) fn try_commit_process_run_child_provider_ingress(
        self,
        ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
        _runtime_heap: &crate::SessionRuntimeHeapOwner,
        execution_state: &mut crate::session::execution_state::KernelExecutionState,
        cursor: &mut ExecutableCursor,
    ) -> ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1 {
        let Self {
            site,
            pending_activity,
            pending_output_authority,
            result_mode,
            result_commit,
            continuation,
            observable_effect_obligation,
        } = self;
        let matched = match ingress
            .try_join_pending_output_authority_for_session_execution_kernel_owner_v1(
                pending_output_authority,
            ) {
            crate::ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1::Joined(
                matched,
            ) => matched,
            crate::ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending_output_authority,
                ingress,
            } => {
                return ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Unmatched {
                    pending: Self {
                        site,
                        pending_activity,
                        pending_output_authority,
                        result_mode,
                        result_commit,
                        continuation,
                        observable_effect_obligation,
                    },
                    ingress,
                };
            }
        };
        let output = match matched.into_nominal_output_for_session_execution_kernel_owner_v1() {
            Ok(output) => output,
            Err(source) => {
                return ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault { source };
            }
        };
        let prepared = match execution_state
            .prepare_nominal_process_provider_effect_result_for_session_execution_kernel_owner_v1(
                result_mode,
                result_commit,
                output,
            ) {
            Ok(prepared) => prepared,
            Err(source) => {
                return ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault {
                    source: crate::ProcessNominalProviderIngressCommitFaultV1::execution_commit_for_session_execution_kernel_owner_v1(
                        crate::ProviderBoundaryExecutionCommitFault::from_provider_execution_frame(source),
                    ),
                };
            }
        };
        prepared.commit_for_session_execution_kernel_owner_v1();
        cursor.commit_target(&continuation);
        ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Committed
    }
}

impl ParkedActorRequestContinuation {
    pub(in crate::session) fn prepare_ready_result_for_return_ensures_owner_v1(
        self,
        session: &mut crate::session::ProcessSessionV0,
        result: crate::VmBoundaryValue,
    ) -> Result<PreparedParentActorReplyReadyApplication, RefusedParentActorReplyReadyApplication>
    {
        let epoch = session.execution_custody.ordinal_for_final_observation();
        let heap_admission = match session
            .runtime_heap
            .prepare_actor_request_ready_heap_admission_for_session_execution_kernel_owner_v1(
                result,
            ) {
            Ok(prepared) => prepared,
            Err(source) => {
                let site = self.site.clone();
                return Err(RefusedParentActorReplyReadyApplication {
                    parked: self,
                    fault: KernelExecutionFault::transition(
                        epoch,
                        site.clone(),
                        ExecutableOperationClass::ActorRequestContinuation,
                        KernelTransitionOwner::CommitActorRequestReadyResult,
                        KernelTransitionFaultSource::ActorRequestCommit(
                            ActorRequestCommitFault::ReadyIngress(source),
                        ),
                    ),
                });
            }
        };
        let Self {
            site,
            result_commit,
            _provider_contract: provider_contract,
        } = self;
        let result_commit = match session
            .execution_custody
            .execution_state
            .prepare_actor_request_ready_result_for_return_ensures_owner_v1(
                &heap_admission,
                result_commit,
            ) {
            Ok(prepared) => prepared,
            Err(refusal) => {
                let (result_commit, source) =
                    refusal.into_commit_and_fault_for_session_runtime_owner_v1();
                return Err(RefusedParentActorReplyReadyApplication {
                    parked: ParkedActorRequestContinuation {
                        site: site.clone(),
                        result_commit,
                        _provider_contract: provider_contract,
                    },
                    fault: KernelExecutionFault::transition(
                        epoch,
                        site.clone(),
                        ExecutableOperationClass::ActorRequestContinuation,
                        KernelTransitionOwner::CommitActorRequestReadyResult,
                        KernelTransitionFaultSource::ActorRequestCommit(
                            ActorRequestCommitFault::Frame(source),
                        ),
                    ),
                });
            }
        };
        Ok(PreparedParentActorReplyReadyApplication {
            site,
            result_commit,
            heap_admission,
            provider_contract,
        })
    }

    /// The sole ready-reply consume. The exact prepared result destination and
    /// command witness are spent together; resume never rebuilds a program
    /// counter or register file from checkpoint projections.
    pub(in crate::session) fn consume_ready_result(
        self,
        session: &mut crate::session::ProcessSessionV0,
        result: crate::VmBoundaryValue,
    ) -> Result<(), KernelExecutionFault> {
        let Self {
            site,
            result_commit,
            _provider_contract,
        } = self;
        let epoch = session.execution_custody.ordinal_for_final_observation();
        let commit = (|| -> Result<(), ActorRequestCommitFault> {
            let runtime_value = session
                .runtime_heap
                .admit_actor_request_ready_boundary_ingress_for_swarmvm_session_runtime_owner_v1(
                    result,
                )
                .map_err(ActorRequestCommitFault::ReadyIngress)?;
            session
                .execution_custody
                .execution_state
                .commit_actor_request_result(&session.runtime_heap, result_commit, runtime_value)
                .map_err(ActorRequestCommitFault::Frame)
        })();
        commit.map_err(|source| {
            KernelExecutionFault::transition(
                epoch,
                site,
                ExecutableOperationClass::ActorRequestContinuation,
                KernelTransitionOwner::CommitActorRequestReadyResult,
                KernelTransitionFaultSource::ActorRequestCommit(source),
            )
        })
    }
}

impl PreparedParentActorReplyReadyApplication {
    pub(in crate::session) fn cancel_for_return_ensures_owner_v1(
        self,
    ) -> ParkedActorRequestContinuation {
        let result_commit = crate::session::execution_state::KernelExecutionState::cancel_prepared_actor_request_ready_result_for_return_ensures_owner_v1(
            self.result_commit,
        );
        drop(self.heap_admission);
        ParkedActorRequestContinuation {
            site: self.site,
            result_commit,
            _provider_contract: self.provider_contract,
        }
    }

    pub(in crate::session) fn commit_for_return_ensures_owner_v1(
        self,
        session: &mut crate::session::ProcessSessionV0,
    ) {
        let value = self
            .heap_admission
            .commit_actor_request_ready_heap_admission_for_session_execution_kernel_owner_v1(
                &mut session.runtime_heap,
            );
        session
            .execution_custody
            .execution_state
            .commit_prepared_actor_request_ready_result_for_return_ensures_owner_v1(
                self.result_commit,
                value,
            );
        drop(self.site);
        drop(self.provider_contract);
    }
}

/// Local-call selection closes the callee frame before mutable session custody
/// is entered. Neither the callable region identity nor its register layout can
/// be recovered by the execution-state owner.
// compiler-custody: symbol=SelectedLocalRegionCallOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedLocalRegionCallOperation {
    site: ExecutablePlanSite,
    return_dst: RegisterIndex,
    input: RegisterIndex,
    callee: ExecutableCursorTarget,
    continuation: ExecutionContinuation,
    callee_seed: crate::session::execution_state::ExecutionRegionFrameSeed,
    capture_transfer_plan: Option<
        Arc<
            crate::session::execution_state::ExecutionLexicalCaptureTransferPlanForSessionExecutionKernelOwnerV1,
        >,
    >,
}

// compiler-custody: symbol=SelectedRuntimeCallableCallOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedRuntimeCallableCallOperation {
    site: ExecutablePlanSite,
    return_dst: RegisterIndex,
    input: RegisterIndex,
    callee: ExecutableCursorTarget,
    continuation: ExecutionContinuation,
    callee_seed: crate::session::execution_state::ExecutionRegionFrameSeed,
    invocation:
        crate::session::compiler_owned_callable_store::JoinedCompilerOwnedOrdinaryCallableInvocationForSessionRuntimeOwnerV1,
    capture_transfer_plan: Option<
        Arc<
            crate::session::execution_state::ExecutionLexicalCaptureTransferPlanForSessionExecutionKernelOwnerV1,
        >,
    >,
    argument_bindings:
        PreparedRuntimeCallableArgumentBindingPlanForSessionExecutionKernelOwnerV1,
}

// compiler-custody: symbol=SelectedOperationLaunchOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedOperationLaunchOperation {
    site: ExecutablePlanSite,
    return_dst: RegisterIndex,
    args: RegisterIndex,
    operation_body: ExecutableCursorTarget,
    continuation: ExecutionContinuation,
    callee_seed: crate::session::execution_state::ExecutionRegionFrameSeed,
}

// compiler-custody: symbol=SelectedArrayCallbackMemberOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedArrayCallbackMemberOperation {
    site: ExecutablePlanSite,
    return_destination: RegisterIndex,
    operation: swarmvm_isa_types::ArrayCallbackMemberOperation,
    receiver_disposition: swarmvm_isa_types::ArrayCallbackReceiverDisposition,
    object: RegisterIndex,
    initial: Option<RegisterIndex>,
    callback_parameter_names: Vec<String>,
    callback_target: ExecutableCursorTarget,
    callback_frame_plan: Arc<crate::session::execution_state::ExecutionRegionFramePlan>,
    capture_transfer_plan: Option<
        Arc<
            crate::session::execution_state::ExecutionLexicalCaptureTransferPlanForSessionExecutionKernelOwnerV1,
        >,
    >,
    final_continuation: ExecutionContinuation,
}

/// One-shot authority to interrupt the current executable region and enter the
/// exact actor-handler region selected by the actor scheduler.
///
/// The handler target, suspended cursor continuation, and prepared register
/// frame are minted together while the opened image is borrowed.  Scheduler
/// code can consume this value only through the finite custody commit below;
/// it cannot inspect or independently install any of those authority facts.
// compiler-custody: symbol=SelectedActorHandlerEntryOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(in crate::session) struct SelectedActorHandlerEntryOperation {
    target: ExecutableCursorTarget,
    continuation: ExecutionContinuation,
    handler_seed: crate::session::execution_state::ExecutionRegionFrameSeed,
    placement_join: JoinedSessionLirPlacedActorHandlerRegionForExecutablePlanOwnerV1,
}

/// One queue-consumed registered-case launch. Selection has already minted
/// the exact callable frame seed and optional heap-admitted context input; the
/// commit can only install the corresponding TestCase frame and cursor.
// compiler-custody: symbol=SelectedTestCaseLaunchOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedTestCaseLaunchOperation {
    site: ExecutablePlanSite,
    target: ExecutableCursorTarget,
    continuation: ExecutionContinuation,
    case_seed: crate::session::execution_state::ExecutionRegionFrameSeed,
    case_input: Option<crate::RuntimeValue>,
    settlement_ordinal: crate::source_entrypoint_compiler_admission_session::test_body_entrypoint_planning::SsTestRegisteredCaseSettlementOrdinalForRuntimeActivityInputOwnerV1,
    test_id: String,
    case_name: String,
}

/// Plan-owned classification of the work, if any, that may occur between
/// entry initialization and root settlement. The no-case proof is minted only
/// from the sealed launch inventory prepared with the executable plan; queue
/// exhaustion after a real campaign cannot manufacture it.
// compiler-custody: symbol=SelectedEntryTerminalCampaignRole disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
enum SelectedEntryTerminalCampaignRole {
    RegisteredCases {
        campaign_boundary_target: ExecutableCursorTarget,
    },
    NoRegisteredCases(SelectedNoRegisteredCaseCampaignProof),
}

// compiler-custody: symbol=SelectedNoRegisteredCaseCampaignProof disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(in crate::session) struct SelectedNoRegisteredCaseCampaignProof {
    _private: (),
}

#[cfg(test)]
impl SelectedNoRegisteredCaseCampaignProof {
    pub(in crate::session) fn for_execution_state_campaign_test() -> Self {
        Self { _private: () }
    }
}

/// Exact entry terminal selected after ordered module initialization. The
/// live cursor is duplicated only into this one-shot product: a registered
/// campaign parks it until drain, while a sealed no-case plan settles it in
/// the same kernel transition.
// compiler-custody: symbol=SelectedEntryTerminalOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedEntryTerminalOperation {
    terminal: SelectedPreparedExecutableOperation<TerminalOperation>,
    entry_terminal_target: ExecutableCursorTarget,
    campaign_role: SelectedEntryTerminalCampaignRole,
}

impl SelectedActorHandlerEntryOperation {
    pub(in crate::session) fn cancel_for_session_runtime_owner_v1(self) {
        let Self {
            target,
            continuation,
            handler_seed,
            placement_join,
        } = self;
        let _settled_selected_operation = (target, continuation, handler_seed, placement_join);
    }

    pub(in crate::session) fn install_for_prepared_actor_handler_entry_transition_owner_v1(
        self,
        custody: &mut crate::session::ExecutionCustody,
        handler_input: Option<crate::RuntimeValue>,
    ) {
        let Self {
            target,
            continuation,
            handler_seed,
            placement_join,
        } = self;
        let _consumed_placement_join = placement_join;
        custody
            .execution_state
            .enter_actor_handler(handler_seed, handler_input, continuation);
        custody.cursor.commit_target(&target);
    }
}

// compiler-custody: symbol=SelectedExecutableOperation disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
enum SelectedExecutableOperation {
    LocalRegionCall(SelectedLocalRegionCallOperation),
    RuntimeCallableCall(SelectedRuntimeCallableCallOperation),
    GraphApply(SelectedPreparedExecutableOperation<GraphApplyOperation>),
    GraphAwait(SelectedPreparedExecutableOperation<GraphAwaitOperation>),
    GraphMaterializeProgram(SelectedGraphMaterializeProgramOperation),
    ProcessRunGraphProgram(SelectedProcessRunGraphProgramOperation),
    ExecutionMetadata(SelectedExecutionMetadataOperation),
    PrivilegedProcess(SelectedPrivilegedProcessOperation),
    InteractionOpen(SelectedInteractionOpenOperation),
    InteractionRuntime(SelectedPreparedExecutableOperation<InteractionRuntimeOperation>),
    EventStreamStep(SelectedPreparedExecutableOperation<EventStreamStepOperation>),
    OperationLaunch(SelectedOperationLaunchOperation),
    OperationObserve(SelectedPreparedExecutableOperation<OperationObserveOperation>),
    ProcessInvokeAwaitExecution(
        SelectedPreparedExecutableOperation<ProcessInvokeAwaitExecutionOperation>,
    ),
    ProcessRunDriveTerminal(SelectedPreparedExecutableOperation<ProcessRunDriveTerminalOperation>),
    ArrayCallbackMember(SelectedArrayCallbackMemberOperation),
    HostResourceFrame(SelectedPreparedExecutableOperation<HostResourceFrameOperation>),
    MakeHostResourceBinding(SelectedPreparedExecutableOperation<MakeHostResourceBindingOperation>),
    ControlFlowJump(SelectedPreparedExecutableOperation<ControlFlowJumpOperation>),
    CapabilityControl(SelectedPreparedExecutableOperation<CapabilityControlOperation>),
    MaterializeActorState(SelectedPreparedExecutableOperation<MaterializeActorStateOperation>),
    WriteActorState(SelectedPreparedExecutableOperation<WriteActorStateOperation>),
    RuntimeObligation(SelectedPreparedExecutableOperation<RuntimeObligationOperation>),
    OrdinaryLocalValue(SelectedPreparedExecutableOperation<OrdinaryLocalValueOperation>),
    ExecutableLocalRead(SelectedExecutableLocalReadOperation),
    ExecutableLocalSettlementUnit(
        SelectedPreparedExecutableOperation<ExecutableLocalSettlementUnitOperation>,
    ),
    RuntimeBindingRead(SelectedPreparedExecutableOperation<RuntimeBindingReadOperation>),
    RuntimeBindingWrite(SelectedPreparedExecutableOperation<RuntimeBindingWriteOperation>),
    BranchOnResult(SelectedPreparedExecutableOperation<BranchOnResultOperation>),
    ActorRequestContinuation(SelectedActorRequestContinuationOperation),
    ActorRequestEffectTerminal(
        SelectedPreparedExecutableOperation<ActorRequestEffectTerminalOperation>,
    ),
    ActorStart(SelectedActorStartOperation),
    Terminal(SelectedPreparedExecutableOperation<TerminalOperation>),
    ProviderBoundary(SelectedProviderBoundaryOperation),
}

// compiler-custody: symbol=SelectedExecutableWork disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=ExecutableSessionPlan::select_work; consumer=ExecutableSessionPlan::commit_current; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
enum SelectedExecutableWork {
    EntryTerminal {
        operation: SelectedEntryTerminalOperation,
        effect_manifest_row: SelectedExecutableInstructionEffectManifestRowForSessionRuntimeOwnerV1,
    },
    TestCaseLaunch(SelectedTestCaseLaunchOperation),
    TestCaseLaunchesDrained {
        site: ExecutablePlanSite,
    },
    PendingActorHandler(SelectedActorHandlerEntryOperation),
    Operation {
        selected: SelectedExecutableOperation,
        effect_manifest_row: SelectedExecutableInstructionEffectManifestRowForSessionRuntimeOwnerV1,
    },
    RegionEnd {
        region: ExecutableRegionIdentity,
    },
}

#[derive(Debug)]
enum ExecutableSelectionFault {
    RegionMissing,
    OperationOutsidePreparedRegion,
    PreparedOperationInputPlanLockPoisoned,
    PreparedObservableEffectObligationLockPoisoned,
    PreparedObservableEffectObligationMissing,
    PreparedObservableEffectObligationDuplicate,
    InstructionEffectManifest(
        ExecutableInstructionEffectManifestRuntimeSelectionFaultForSessionRuntimeOwnerV1,
    ),
    LexicalCaptureInvocationForeignParent,
    PendingActorHandlerRegionMissing {
        _candidate: SessionLirPlacedActorHandlerRegionReferenceForSessionRuntimeOwnerV1,
    },
    PendingActorHandlerRegionUnmatched {
        _candidate: SessionLirPlacedActorHandlerRegionReferenceForSessionRuntimeOwnerV1,
    },
    PendingActorHandlerAtTestCaseLaunchBoundary {
        _candidate: SessionLirPlacedActorHandlerRegionReferenceForSessionRuntimeOwnerV1,
    },
    RuntimeCallableRegisterInvalid(crate::session::execution_state::ExecutionStateFault),
    RuntimeCallableValueKindMismatch,
    RuntimeCallableClosureUnmatched {
        _reference: crate::CompilerOwnedOrdinaryCallableClosureReferenceForRuntimeValueOwnerV1,
    },
    RuntimeCallableTargetUnmatched,
    RuntimeCallableParameterArityMismatch,
}

enum ExecutableWorkSelectionFault {
    Selection(ExecutableSelectionFault),
    Transition {
        selected_transition: ExecutableTransitionObservation,
        site: ExecutablePlanSite,
        operation_class: ExecutableOperationClass,
        owner: KernelTransitionOwner,
        source: KernelTransitionFaultSource,
    },
}
