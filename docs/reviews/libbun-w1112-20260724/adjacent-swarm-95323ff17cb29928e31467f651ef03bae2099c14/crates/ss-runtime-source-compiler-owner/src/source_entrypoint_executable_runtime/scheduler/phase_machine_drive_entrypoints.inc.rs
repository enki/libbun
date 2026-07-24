impl ProcessSessionV0 {
    fn admit_process_child_capability_exclusion_failed_terminal_for_session_runtime_owner_v1(
        &mut self,
        _exclusion:
            crate::session::EffectiveCapabilityIdentityExcludedFromCurrentFrameForSwarmvmSessionRuntimeOwnerV1,
        boundary_context: &'static str,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, ProcessSessionRunError> {
        const CODE: &str = "process_child_capability_identity_excluded";
        const MESSAGE: &str = "process child capability dispatch was excluded from the exact delegated effective frame";
        let classified_cause =
            crate::session::ProcessSessionClassifiedFailureCauseV0::capability_admission_failure(
                CODE, MESSAGE,
            );
        let readiness_source =
            crate::session::ProcessTerminalBoundaryReadinessSourceV1::from_session_runtime_terminal_owner_v1(
                crate::session::ProcessTerminalBoundaryReadinessSourceKindV1::ProcessChildCapabilityAdmissionFailure,
                boundary_context,
            );
        let readiness_certificate =
            crate::session::ProcessBoundaryReadinessCertificateV1::new_for_failed_terminal_readiness_certificate(
                crate::session::ProcessFailedTerminalBoundaryReadinessOwnerV1::from_classified_failure_cause_receipts(
                    readiness_source,
                    CODE,
                    &classified_cause,
                ),
            );
        self.admit_direct_run_result_product_for_session_runtime_owner_v1(
            crate::session::ProcessSessionRunOutcomeV0::Failed {
                terminal_authority:
                    crate::session::ProcessFailedTerminalOutcomeAuthorityV1::from_session_runtime_terminal_owner_v1(
                        "process_child_capability_admission_failure",
                    ),
                code: CODE.to_owned(),
                message: MESSAGE.to_owned(),
                classified_cause,
                readiness_certificate,
                diagnostics: None,
            },
            crate::session::ProcessSessionResultAdmissionBoundaryContextV0::engine_public_projection(
                boundary_context,
            ),
        )
    }

    pub fn commit_provider_ready_boundary_output_for_session_execution_kernel_owner_v1(
        &mut self,
        ready_output: swarm_capability_model::ProviderReadyBoundaryOutput,
        host_resource_releases: swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseTransferSetV1,
    ) -> Result<(), ProcessSessionRunError> {
        let pending_boundary = self
            .execution_custody
            .pending_kernel_boundary
            .take()
            .ok_or(ProviderBoundaryIngressFault::NoPendingApplication)?;
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::Provider(pending) => pending,
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(ProviderBoundaryIngressFault::DifferentPendingBoundary {
                    pending_kind,
                }
                .into());
            }
        };
        let crate::session::ExecutionCustody {
            opened_plan: _,
            cursor,
            execution_state,
            pending_kernel_boundary: _,
            deferred_finalization: _,
            resource_policy: _,
            progress: _,
        } = &mut self.execution_custody;
        pending.consume_corresponded_ready_output(
            ready_output,
            host_resource_releases,
            &mut self.runtime_heap,
            execution_state,
            cursor,
        )?;
        Ok(())
    }

    pub fn commit_provider_ready_boundary_output_and_drive_to_direct_run_result_product_for_direct_run_owner_v1(
        &mut self,
        ready_output: swarm_capability_model::ProviderReadyBoundaryOutput,
        host_resource_releases: swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseTransferSetV1,
        boundary_context: &'static str,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, ProcessSessionRunError> {
        self.commit_provider_ready_boundary_output_for_session_execution_kernel_owner_v1(
            ready_output,
            host_resource_releases,
        )?;
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            boundary_context,
        )
    }

    pub fn commit_process_invoke_execution_provider_ingress_and_drive_for_direct_run_owner_v1(
        &mut self,
        ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    ) -> Result<
        DirectRunProcessSessionRunResultProductV1,
        crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1,
    > {
        let pending_boundary = match self.execution_custody.pending_kernel_boundary.take() {
            Some(pending) => pending,
            None => {
                return Err(
                    crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::NoPendingBoundary {
                        ingress,
                    },
                );
            }
        };
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::Provider(pending) => pending,
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(
                    crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary {
                        pending_kind,
                        ingress,
                    },
                );
            }
        };
        let crate::session::ExecutionCustody {
            opened_plan: _,
            cursor,
            execution_state,
            pending_kernel_boundary: _,
            deferred_finalization: _,
            resource_policy: _,
            progress: _,
        } = &mut self.execution_custody;
        match pending.try_commit_process_invoke_execution_provider_ingress(
            ingress,
            &self.runtime_heap,
            execution_state,
            cursor,
        ) {
            executable_image::ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Committed => {}
            executable_image::ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Unmatched {
                pending,
                ingress,
            } => {
                self.execution_custody.pending_kernel_boundary = Some(
                    executable_image::PendingExecutableKernelBoundary::Provider(pending),
                );
                return Err(
                    crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch {
                        ingress,
                    },
                );
            }
            executable_image::ProcessInvokeExecutionProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault {
                source,
            } => {
                return Err(
                    crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::Commit {
                        source,
                    },
                );
            }
        }
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "direct_process_invoke_nominal_provider_ingress",
        )
        .map_err(|source| {
            crate::ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1::Drive {
                source,
            }
        })
    }

    pub fn commit_process_run_child_provider_ingress_and_drive_for_direct_run_owner_v1(
        &mut self,
        ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
    ) -> Result<
        DirectRunProcessSessionRunResultProductV1,
        crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1,
    > {
        let pending_boundary = match self.execution_custody.pending_kernel_boundary.take() {
            Some(pending) => pending,
            None => {
                return Err(
                    crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::NoPendingBoundary {
                        ingress,
                    },
                );
            }
        };
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::Provider(pending) => pending,
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(
                    crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary {
                        pending_kind,
                        ingress,
                    },
                );
            }
        };
        let crate::session::ExecutionCustody {
            opened_plan: _,
            cursor,
            execution_state,
            pending_kernel_boundary: _,
            deferred_finalization: _,
            resource_policy: _,
            progress: _,
        } = &mut self.execution_custody;
        match pending.try_commit_process_run_child_provider_ingress(
            ingress,
            &self.runtime_heap,
            execution_state,
            cursor,
        ) {
            executable_image::ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Committed => {}
            executable_image::ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Unmatched {
                pending,
                ingress,
            } => {
                self.execution_custody.pending_kernel_boundary = Some(
                    executable_image::PendingExecutableKernelBoundary::Provider(pending),
                );
                return Err(
                    crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch {
                        ingress,
                    },
                );
            }
            executable_image::ProcessRunChildProviderIngressCommitForSessionExecutionKernelOwnerV1::Fault {
                source,
            } => {
                return Err(
                    crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::Commit {
                        source,
                    },
                );
            }
        }
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "direct_process_run_nominal_provider_ingress",
        )
        .map_err(|source| {
            crate::ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1::Drive { source }
        })
    }

    pub fn commit_process_invoke_await_execution_resume_and_drive_for_direct_run_owner_v1(
        &mut self,
        resume: crate::ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    ) -> Result<
        DirectRunProcessSessionRunResultProductV1,
        crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1,
    > {
        let pending_boundary = match self.execution_custody.pending_kernel_boundary.take() {
            Some(pending) => pending,
            None => {
                return Err(
                    crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::NoPendingBoundary {
                        resume,
                    },
                );
            }
        };
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::ProcessInvokeAwaitExecution(
                pending,
            ) => pending,
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(
                    crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary {
                        pending_kind,
                        resume,
                    },
                );
            }
        };
        let matched = match pending
            .try_join_resume_for_session_execution_kernel_owner_v1(resume)
        {
            crate::ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1::Joined(
                matched,
            ) => matched,
            crate::ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending,
                resume,
            } => {
                self.execution_custody.pending_kernel_boundary = Some(
                    executable_image::PendingExecutableKernelBoundary::ProcessInvokeAwaitExecution(
                        pending,
                    ),
                );
                return Err(
                    crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch {
                        resume,
                    },
                );
            }
        };
        let crate::MatchedProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
            producer,
            continuation,
            result,
        } = matched;
        let heap_transition = self
            .runtime_heap
            .prepare_process_kernel_boundary_result_transition_for_session_execution_kernel_owner_v1(
                result,
            )
            .map_err(|source| {
                crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::result_admission_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        let destination = self
            .execution_custody
            .execution_state
            .prepare_process_kernel_boundary_result_destination_for_session_execution_kernel_owner_v1(
                producer.into_destination_for_session_execution_kernel_owner_v1(),
            )
            .map_err(|source| {
                crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::execution_commit_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        let result = heap_transition.commit_for_session_execution_kernel_owner_v1();
        destination.commit_for_session_execution_kernel_owner_v1(result);
        self.execution_custody.cursor.commit_target(&continuation);
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "direct_process_invoke_await_execution_resume",
        )
        .map_err(|source| {
            crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1::Drive {
                source,
            }
        })
    }

    pub fn commit_process_run_drive_terminal_resume_and_drive_for_direct_run_owner_v1(
        &mut self,
        resume: crate::ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    ) -> Result<
        DirectRunProcessSessionRunResultProductV1,
        crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1,
    > {
        let pending_boundary = match self.execution_custody.pending_kernel_boundary.take() {
            Some(pending) => pending,
            None => {
                return Err(
                    crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::NoPendingBoundary {
                        resume,
                    },
                );
            }
        };
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::ProcessRunDriveTerminal(pending) => {
                pending
            }
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(
                    crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary {
                        pending_kind,
                        resume,
                    },
                );
            }
        };
        let matched = match pending
            .try_join_resume_for_session_execution_kernel_owner_v1(resume)
        {
            crate::ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1::Joined(
                matched,
            ) => matched,
            crate::ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending,
                resume,
            } => {
                self.execution_custody.pending_kernel_boundary = Some(
                    executable_image::PendingExecutableKernelBoundary::ProcessRunDriveTerminal(
                        pending,
                    ),
                );
                return Err(
                    crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch {
                        resume,
                    },
                );
            }
        };
        let crate::MatchedProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
            producer,
            continuation,
            terminal,
        } = matched;
        let heap_transition = self
            .runtime_heap
            .prepare_process_kernel_boundary_result_transition_for_session_execution_kernel_owner_v1(
                terminal,
            )
            .map_err(|source| {
                crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::result_admission_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        let destination = self
            .execution_custody
            .execution_state
            .prepare_process_kernel_boundary_result_destination_for_session_execution_kernel_owner_v1(
                producer.into_destination_for_session_execution_kernel_owner_v1(),
            )
            .map_err(|source| {
                crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::execution_commit_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        let terminal = heap_transition.commit_for_session_execution_kernel_owner_v1();
        destination.commit_for_session_execution_kernel_owner_v1(terminal);
        self.execution_custody.cursor.commit_target(&continuation);
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "direct_process_run_drive_terminal_resume",
        )
        .map_err(|source| {
            crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1::Drive { source }
        })
    }

    pub fn commit_process_control_resume_and_drive_for_direct_run_owner_v1(
        &mut self,
        resume: crate::ProcessControlResumeProductForDirectRunOwnerV1,
    ) -> Result<
        DirectRunProcessSessionRunResultProductV1,
        crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1,
    > {
        let pending_boundary = match self.execution_custody.pending_kernel_boundary.take() {
            Some(pending) => pending,
            None => {
                return Err(
                    crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::NoPendingBoundary {
                        resume,
                    },
                );
            }
        };
        let pending = match pending_boundary {
            executable_image::PendingExecutableKernelBoundary::ProcessControl(pending) => pending,
            other => {
                let pending_kind = other.diagnostic_kind();
                self.execution_custody.pending_kernel_boundary = Some(other);
                return Err(
                    crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::DifferentPendingBoundary {
                        pending_kind,
                        resume,
                    },
                );
            }
        };
        let matched = match pending.try_join_resume_for_session_execution_kernel_owner_v1(resume) {
            crate::ProcessControlResumeJoinForSessionExecutionKernelOwnerV1::Joined(matched) => {
                matched
            }
            crate::ProcessControlResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending,
                resume,
            } => {
                self.execution_custody.pending_kernel_boundary = Some(
                    executable_image::PendingExecutableKernelBoundary::ProcessControl(pending),
                );
                return Err(
                    crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::CorrespondenceMismatch {
                        resume,
                    },
                );
            }
        };
        let crate::MatchedProcessControlResumeForSessionExecutionKernelOwnerV1 {
            result_mode,
            result_commit,
            continuation,
            receipt,
        } = matched;
        let receipt = self
            .runtime_heap
            .admit_process_kernel_boundary_result_transactionally_for_session_execution_kernel_owner_v1(
                receipt,
            )
            .map_err(|source| {
                crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::result_admission_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        self.execution_custody
            .execution_state
            .commit_provider_effect_result(
                &self.runtime_heap,
                result_mode,
                result_commit,
                receipt,
            )
            .map_err(|source| {
                crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::Commit {
                    source: crate::ProcessKernelBoundaryResumeCommitFaultV1::provider_result_commit_for_session_execution_kernel_owner_v1(source),
                }
            })?;
        self.execution_custody.cursor.commit_target(&continuation);
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "direct_process_control_resume",
        )
        .map_err(|source| {
            crate::ProcessControlResumeDriveFailureForDirectRunOwnerV1::Drive { source }
        })
    }

    pub fn commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1(
        &mut self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
        selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, ProcessSessionRunError> {
        // Pending custody stays installed while the provider runs. The matched
        // proof borrows it, so an unwind can cancel and later reissue the exact
        // obligation instead of dropping moved provider authority.
        let pending = self
            .pending_selected_host_resource_finalization
            .as_mut()
            .ok_or(crate::HostResourceFinalizationBoundaryFaultV1::CommitWithoutPendingSelection)?;
        let matched =
            match pending.consume_exact_selection_for_session_runtime_owner_v1(selected_boundary) {
                Ok(matched) => matched,
                Err(crate::session::SelectedHostResourceFinalizationConsumptionRefusalV1::Correspondence {
                    selected,
                    fault,
                }) => {
                    drop(selected);
                    return Err(fault.into());
                }
                Err(crate::session::SelectedHostResourceFinalizationConsumptionRefusalV1::Invariant {
                    fault,
                }) => {
                    return Err(fault.into());
                }
            };
        let _receipt = match matched
            .commit_exact_provider_release_for_session_execution_kernel_owner_v1(
                provider_execution_session,
            ) {
            Ok(receipt) => receipt,
            Err(fault) => {
                return Err(
                    crate::HostResourceFinalizationDriveFaultV1::ProviderRelease { source: fault }
                        .into(),
                );
            }
        };
        let consumed_pending = self
            .pending_selected_host_resource_finalization
            .take()
            .ok_or(crate::HostResourceFinalizationBoundaryFaultV1::CommitWithoutPendingSelection)?;

        let continuation_receipt = self.drive_one_owned_runtime_turn_for_session_runtime_owner_v1();
        let publication = self
            .admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
                continuation_receipt,
                "direct_host_resource_finalization_commit",
            );
        // Keep the consumed obligation alive through continuation admission.
        // This makes provider Consumed visible before publication while the
        // exact session-owned custody cannot drop prematurely.
        drop(consumed_pending);
        publication
    }

    pub(crate) fn drive_to_declared_readiness_for_full_process_session_boundary(
        &mut self,
        projection_poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<ProcessSessionRunOutcomeV0, ProcessSessionRunError> {
        let _ = self;
        match projection_poison {}
    }

    pub(crate) fn drive_event_wait_producer_progress_for_full_process_session_boundary(
        &mut self,
        parked_waiter_identity: &ProcessSessionEventWaitParkedActivityIdentityV0,
        projection_poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<ProcessSessionEventWaitProducerProgressOutcomeV0, ProcessSessionRunError> {
        let _ = (self, parked_waiter_identity);
        match projection_poison {}
    }

    fn admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
        &mut self,
        receipt: SessionContinuationStepReceipt,
        boundary_context: &'static str,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, ProcessSessionRunError> {
        let sealed_outcome = match receipt {
            SessionContinuationStepReceipt::NextHostBoundary(boundary) => {
                SealedProcessSessionDriveOutcome::from_external_boundary_evidence_for_session_runtime_owner_v1(
                    boundary,
                )
            }
            SessionContinuationStepReceipt::ProcessInvokeAwaitExecution(selected_boundary) => {
                return Ok(
                    DirectRunProcessSessionRunResultProductV1::from_process_invoke_await_execution_for_session_runtime_owner_v1(
                        selected_boundary,
                    ),
                );
            }
            SessionContinuationStepReceipt::ProcessRunDriveTerminal(selected_boundary) => {
                return Ok(
                    DirectRunProcessSessionRunResultProductV1::from_process_run_drive_terminal_for_session_runtime_owner_v1(
                        selected_boundary,
                    ),
                );
            }
            SessionContinuationStepReceipt::ProcessControl(selected_boundary) => {
                return Ok(
                    DirectRunProcessSessionRunResultProductV1::from_process_control_for_session_runtime_owner_v1(
                        selected_boundary,
                    ),
                );
            }
            SessionContinuationStepReceipt::TerminalSettled(terminal) => {
                SealedProcessSessionDriveOutcome::from_terminal_result_evidence_for_session_runtime_owner_v1(
                    terminal,
                )
            }
            SessionContinuationStepReceipt::Fault(
                SessionContinuationStepFaultEvidence::TerminalObservableEffectCoverageIncomplete(
                    evidence,
                ),
            ) => SealedProcessSessionDriveOutcome::from_terminal_observable_effect_coverage_incomplete_for_session_runtime_owner_v1(
                evidence,
            ),
            SessionContinuationStepReceipt::Fault(
                SessionContinuationStepFaultEvidence::ResourceExhausted(evidence),
            ) => SealedProcessSessionDriveOutcome::from_resource_exhausted_for_session_runtime_owner_v1(
                evidence,
            ),
            SessionContinuationStepReceipt::Fault(
                SessionContinuationStepFaultEvidence::InvalidHostBoundary(evidence),
            ) => SealedProcessSessionDriveOutcome::from_invalid_host_boundary_for_session_runtime_owner_v1(
                evidence,
            ),
            SessionContinuationStepReceipt::Fault(
                SessionContinuationStepFaultEvidence::SchedulerInvariantBroken(evidence),
            ) => SealedProcessSessionDriveOutcome::from_scheduler_invariant_broken_for_session_runtime_owner_v1(
                evidence,
            ),
            SessionContinuationStepReceipt::Fault(
                SessionContinuationStepFaultEvidence::KernelExecution(evidence),
            ) => {
                if self.effective_capability_exclusion_disposition
                    == crate::session::EffectiveCapabilityExclusionDispositionV1::FailedProcessChildTerminal
                {
                    match evidence
                        .into_effective_capability_identity_excluded_for_process_child_owner_v1()
                    {
                        Ok(exclusion) => {
                            return self.admit_process_child_capability_exclusion_failed_terminal_for_session_runtime_owner_v1(
                                exclusion,
                                boundary_context,
                            );
                        }
                        Err(evidence) => SealedProcessSessionDriveOutcome::from_kernel_execution_fault_for_session_runtime_owner_v1(
                            evidence,
                        ),
                    }
                } else {
                    SealedProcessSessionDriveOutcome::from_kernel_execution_fault_for_session_runtime_owner_v1(
                        evidence,
                    )
                }
            }
        };
        let outcome =
            sealed_outcome.into_run_outcome_for_session_liveness_owner_v1(boundary_context)?;
        self.admit_direct_run_result_product_for_session_runtime_owner_v1(
            outcome,
            ProcessSessionResultAdmissionBoundaryContextV0::engine_public_projection(
                boundary_context,
            ),
        )
    }

    pub fn drive_process_session_until_external_boundary_for_session_runtime_owner_v1(
        &mut self,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, String> {
        self.drive_process_session_until_external_boundary_with_runtime_terminal_observation_for_direct_run_owner_v1()
            .map_err(|fault| fault.to_string())
    }

    pub fn drive_process_session_until_external_boundary_with_runtime_terminal_observation_for_direct_run_owner_v1(
        &mut self,
    ) -> Result<DirectRunProcessSessionRunResultProductV1, ProcessSessionRunError> {
        let receipt = self.drive_one_owned_runtime_turn_for_session_runtime_owner_v1();
        self.admit_continuation_receipt_into_direct_run_result_product_for_session_runtime_owner_v1(
            receipt,
            "process_session_start_external_drive",
        )
    }

    pub fn take_selected_provider_resume_host_input_for_direct_run_owner_v1(
        &mut self,
        selected_boundary:
            crate::session::work_runtime::SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    ) -> Result<
        crate::session::work_runtime::SelectedProviderResumeHostInputForDirectRunOwnerV1,
        String,
    > {
        let effect_ref = selected_boundary.into_effect_ref_for_session_execution_kernel_owner_v1();
        self.work_runtime
            .take_selected_provider_resume_host_input_for_direct_run_owner_v1(&effect_ref)
    }

    pub fn admit_selected_process_run_child_launch_for_direct_run_owner_v1(
        &mut self,
        selected: crate::session::work_runtime::SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<
        crate::session::work_runtime::AdmittedProcessRunChildLaunchForDirectRunOwnerV1,
        crate::session::work_runtime::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    > {
        let preflighted = selected
            .preflight_for_session_execution_kernel_owner_v1(&self.current_exact_capability_scope)
            .map_err(|refusal| refusal.cancel_for_session_runtime_owner_v1())?;
        Ok(preflighted.commit_after_complete_preflight_for_session_runtime_owner_v1())
    }

    pub fn admit_selected_process_load_child_launch_for_direct_run_owner_v1(
        &mut self,
        selected: crate::session::work_runtime::SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<
        crate::session::work_runtime::AdmittedProcessLoadChildLaunchForDirectRunOwnerV1,
        crate::session::work_runtime::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    > {
        let preflighted = selected
            .preflight_for_session_execution_kernel_owner_v1(&self.current_exact_capability_scope)
            .map_err(|refusal| refusal.cancel_for_session_runtime_owner_v1())?;
        Ok(preflighted.commit_after_complete_preflight_for_session_runtime_owner_v1())
    }

    pub fn admit_selected_process_invoke_child_launch_for_direct_run_owner_v1(
        &mut self,
        selected: crate::session::work_runtime::SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<
        crate::session::work_runtime::AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1,
        crate::session::work_runtime::ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    > {
        let preflighted = selected
            .preflight_for_session_execution_kernel_owner_v1(&self.current_exact_capability_scope)
            .map_err(|refusal| refusal.cancel_for_session_runtime_owner_v1())?;
        Ok(preflighted.commit_after_complete_preflight_for_session_runtime_owner_v1())
    }

    pub(in crate::session) fn drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1(
        &mut self,
    ) -> SessionContinuationStepReceipt {
        self.drive_session_continuation_step_for_session_runtime_owner_v1()
    }

    pub fn drive_reawaken_to_public_aperture_boundary(
        &mut self,
    ) -> Result<DirectRunProcessSessionPublicApertureProgressProductV1, ProcessSessionRunError>
    {
        let receipt =
            self.drive_one_owned_runtime_turn_after_provider_resume_for_session_runtime_owner_v1();
        Ok(DirectRunProcessSessionPublicApertureProgressProductV1::from_session_continuation_step_receipt_for_session_runtime_owner_v1(self, receipt))
    }

    pub(in crate::session) fn drive_one_owned_runtime_turn_for_session_runtime_owner_v1(
        &mut self,
    ) -> SessionContinuationStepReceipt {
        self.drive_session_continuation_step_for_session_runtime_owner_v1()
    }

    fn drive_session_continuation_step_for_session_runtime_owner_v1(
        &mut self,
    ) -> SessionContinuationStepReceipt {
        let authority = PublicApertureSchedulerDriveAuthority::public_aperture_entrypoint();
        execution_kernel::SessionExecutionKernel::drive(self, &authority).into_outer_receipt()
    }
}
