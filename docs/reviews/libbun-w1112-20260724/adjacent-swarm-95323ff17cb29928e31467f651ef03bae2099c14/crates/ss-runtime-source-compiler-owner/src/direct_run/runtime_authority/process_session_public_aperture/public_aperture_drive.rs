use super::provider_resume_lifecycle::*;
use super::session_route_lifecycle::*;
use super::*;

fn direct_run_public_aperture_next_step_without_closed_drive_consumer_fault(
    owner_kind: DirectRunKernelStateRefOwnerKind,
) -> String {
    json!({
        "kind": "direct_run_public_aperture_next_step_owner_kind_without_closed_drive_consumer",
        "reason": "selected body launch closed drive requires a finite owner consumer for each public-aperture next step",
        "owner_kind": owner_kind.as_str(),
    })
    .to_string()
}

impl DirectRunRuntimeAuthorityOwner {
    pub(in crate::direct_run) fn drive_prepared_runtime_process_start_command_public_aperture_until_terminal_or_next_step_with_runtime_terminal_observation_for_ss_test_owner_v1(
        command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let (kernel_state, prepared_runtime_executable_image) = command
            .into_start_product_for_direct_run_public_aperture_owner_v1()?
            .into_kernel_state_and_session_open_inputs_for_direct_run_runtime_authority_owner_v1(
            )?;
        Self::drive_process_start_kernel_state_public_aperture_until_terminal_or_next_step(
            kernel_state,
            prepared_runtime_executable_image,
            "prepared_runtime_process_start_public_aperture",
        )
    }

    pub(in crate::direct_run) fn drive_prepared_runtime_process_start_command_public_aperture_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
        command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let mut output =
            Self::drive_prepared_runtime_process_start_command_public_aperture_until_terminal_or_next_step_with_runtime_terminal_observation_for_ss_test_owner_v1(
                command,
            )?;
        loop {
            match output.into_closed_drive_state_for_direct_run_runtime_execution_owner_v1() {
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::Terminal(output) => {
                    return Ok(output);
                }
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::NextStep(next_step) => {
                    output = Self::drive_public_aperture_next_step_until_terminal_or_next_step(
                        next_step,
                        provider_execution_session,
                    )?;
                }
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::HostResourceFinalization(
                    finalization,
                ) => {
                    output = Self::drive_host_resource_finalization_until_terminal_or_next_step(
                        finalization,
                        provider_execution_session,
                    )
                    .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?;
                }
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessInvokeAwaitExecution(
                    boundary,
                ) => {
                    output = Self::drive_process_invoke_await_execution_until_terminal_or_next_step(
                        boundary,
                        provider_execution_session,
                    )?;
                }
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessRunDriveTerminal(
                    boundary,
                ) => {
                    output = Self::drive_process_run_drive_terminal_until_terminal_or_next_step(
                        boundary,
                        provider_execution_session,
                    )?;
                }
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessControl(
                    boundary,
                ) => {
                    output = finish_direct_run_public_aperture_drive_output(
                        boundary
                            .drive_for_process_kernel_owner_v1()
                            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?,
                    )
                    .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?;
                }
            }
        }
    }

    fn drive_public_aperture_next_step_until_terminal_or_next_step(
        next_step: DirectRunProcessSessionPublicApertureNextStepOutputProductV1,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let owner_kind = next_step
            .owner_kind_for_direct_run_process_session_public_aperture_owner_v1(
                "direct_run_public_aperture_closed_drive.next_step.owner_kind",
            )?;
        match owner_kind {
            DirectRunKernelStateRefOwnerKind::ProviderResume => {
                Self::drive_provider_resume_public_aperture_next_step_until_terminal_or_next_step(
                    next_step,
                    provider_execution_session,
                )
            }
            DirectRunKernelStateRefOwnerKind::ProcessSessionStart
            | DirectRunKernelStateRefOwnerKind::ProcessSessionReawaken
            | DirectRunKernelStateRefOwnerKind::ProcessSessionProjection => {
                let kernel_state_ref = next_step
                    .into_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
                    );
                drive_process_session_continuation_ref_to_public_aperture_with_typed_live_authority(
                    kernel_state_ref,
                )
            }
            other => Err(DirectRunProcessSessionDriveFaultV1::Generic(
                direct_run_public_aperture_next_step_without_closed_drive_consumer_fault(other),
            )),
        }
    }

    fn drive_host_resource_finalization_until_terminal_or_next_step(
        finalization: DirectRunHostResourceFinalizationNextStepV1,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<DirectRunProcessSessionPublicApertureDriveOutputV1, String> {
        let route_output =
            finalization.drive_for_direct_run_owner_v1(provider_execution_session)?;
        Ok(finish_direct_run_public_aperture_drive_output(
            route_output,
        )?)
    }

    fn drive_process_invoke_await_execution_until_terminal_or_next_step(
        next_step: DirectRunProcessInvokeAwaitExecutionNextStepV1,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let route_output = next_step
            .into_child_drive_stage_for_process_kernel_owner_v1()
            .drive_matching_child_for_process_kernel_owner_v1(provider_execution_session)?
            .commit_and_route_parent_for_process_kernel_owner_v1()?;
        Ok(finish_direct_run_public_aperture_drive_output(
            route_output,
        )?)
    }

    fn drive_process_run_drive_terminal_until_terminal_or_next_step(
        next_step: DirectRunProcessRunDriveTerminalNextStepV1,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let route_output = next_step
            .into_child_drive_stage_for_process_kernel_owner_v1()
            .drive_matching_child_for_process_kernel_owner_v1(provider_execution_session)?
            .commit_and_route_parent_for_process_kernel_owner_v1()?;
        Ok(finish_direct_run_public_aperture_drive_output(
            route_output,
        )?)
    }

    fn drive_provider_resume_public_aperture_next_step_until_terminal_or_next_step(
        next_step: DirectRunProcessSessionPublicApertureNextStepOutputProductV1,
        provider_execution_session: &mut ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let kernel_state_ref = next_step
            .into_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1();
        let token_admission_operation =
            "direct_run_public_aperture_closed_drive.provider_resume.typed_token_admission";
        let mut provider_resume_token = kernel_state_ref
            .provider_resume_continuation_token_for_runtime_authority_owner(
                token_admission_operation,
            )?;
        let provider_resume_private_storage =
            take_provider_resume_host_boundary_private_storage_from_public_aperture_kernel_state_ref_for_runtime_authority_owner(
                kernel_state_ref,
                &provider_resume_token,
                token_admission_operation,
                "direct_run_public_aperture_closed_drive.provider_resume.private_storage_take",
            )?;
        let selected_boundary = provider_resume_token
            .take_provider_resume_boundary_for_direct_run_provider_resume_owner_v1()?;
        let selected_provider_input =
            Self::take_selected_provider_resume_host_input_for_live_process_session(
                provider_resume_token.live_process_session_id(),
                provider_resume_token.root_scope_id(),
                selected_boundary,
            )?;
        let selected_route =
            select_provider_resume_route_for_direct_run_owner_v1(selected_provider_input)?;
        let engine_result = match selected_route {
            DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected_input) => {
                let admitted_load =
                    Self::admit_selected_process_load_child_launch_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        selected_input,
                    )?;
                let provider_drive_result =
                    execute_kernel_internal_process_load_for_provider_resume_owner_v1(
                        admitted_load
                            .commit_into_process_lifecycle_registration_for_direct_run_owner_v1(),
                        &provider_resume_token,
                        &provider_resume_private_storage,
                    )
                    .map_err(DirectRunProcessSessionDriveFaultV1::ProcessLoad)?;
                Self::apply_provider_drive_ready_result_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    provider_drive_result,
                )?
            }
            DirectRunSelectedProviderResumeRouteV1::ProcessCheckpoint(selected_input) => {
                let provider_drive_result =
                    execute_kernel_internal_process_checkpoint_for_provider_resume_owner_v1(
                        selected_input,
                    )
                    .map_err(DirectRunProcessSessionDriveFaultV1::ProcessCheckpoint)?;
                Self::apply_provider_drive_ready_result_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    provider_drive_result,
                )?
            }
            DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected_input) => {
                let provider_drive_result =
                    execute_kernel_internal_process_restore_with_static_child_context_for_provider_resume_owner_v1(
                        selected_input,
                    )
                    .map_err(DirectRunProcessSessionDriveFaultV1::ProcessRestore)?;
                Self::apply_provider_drive_ready_result_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    provider_drive_result,
                )?
            }
            DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected_input) => {
                let drive_context =
                    DirectRunProcessKernelChildDriveContext::from_provider_resume_owner_v1(
                        provider_resume_private_storage.current_process(),
                        provider_resume_private_storage.admitted_static_child_source_programs(),
                        provider_resume_private_storage.prepared_static_child_runtime_handles(),
                        provider_resume_token.root_scope_id(),
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.node_id(),
                        provider_resume_token.started_at(),
                    )?;
                let admitted_launch =
                    Self::admit_selected_process_invoke_child_launch_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        selected_input,
                    )?;
                let PreparedProcessInvokeProviderIngressV1 {
                    ingress,
                    registration,
                    execution,
                } = prepare_process_invoke_provider_ingress_for_process_kernel_owner_v1(
                    admitted_launch,
                )?;
                let engine_result =
                    Self::commit_process_invoke_execution_provider_ingress_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        ingress,
                    )?;
                if engine_result.outcome_kind() != "process_invoke_await_execution" {
                    let _settled_unselected_registration = (registration, execution, drive_context);
                    return Err(json!({
                        "kind": "process_invoke_nominal_ingress_outcome_mismatch",
                        "reason": "committed process.invoke nominal ingress must yield its exact await-execution boundary before execution registration",
                        "outcome_kind": engine_result.outcome_kind(),
                    })
                    .to_string()
                    .into());
                }
                register_process_invoke_execution_after_ingress_commit_for_process_kernel_owner_v1(
                    registration,
                    execution,
                    drive_context,
                )?;
                engine_result
            }
            DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected_input) => {
                let drive_context =
                    DirectRunProcessKernelChildDriveContext::from_provider_resume_owner_v1(
                        provider_resume_private_storage.current_process(),
                        provider_resume_private_storage.admitted_static_child_source_programs(),
                        provider_resume_private_storage.prepared_static_child_runtime_handles(),
                        provider_resume_token.root_scope_id(),
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.node_id(),
                        provider_resume_token.started_at(),
                    )?;
                let admitted_launch =
                    Self::admit_selected_process_run_child_launch_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        selected_input,
                    )?;
                let PreparedProcessRunProviderIngressV1 {
                    ingress,
                    registration,
                    execution,
                } = prepare_process_run_provider_ingress_for_process_kernel_owner_v1(
                    admitted_launch,
                )?;
                let engine_result =
                    Self::commit_process_run_child_provider_ingress_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        ingress,
                    )?;
                if engine_result.outcome_kind() != "process_run_drive_terminal" {
                    let _settled_unselected_registration = (registration, execution, drive_context);
                    return Err(json!({
                        "kind": "process_run_nominal_ingress_outcome_mismatch",
                        "reason": "committed process.run nominal ingress must yield its exact drive-terminal boundary before child registration",
                        "outcome_kind": engine_result.outcome_kind(),
                    })
                    .to_string()
                    .into());
                }
                register_process_run_child_after_ingress_commit_for_process_kernel_owner_v1(
                    registration,
                    execution,
                    drive_context,
                )?;
                engine_result
            }
            DirectRunSelectedProviderResumeRouteV1::ProcessActivate(selected_input) => {
                let drive_context =
                    DirectRunProcessKernelChildDriveContext::from_provider_resume_owner_v1(
                        provider_resume_private_storage.current_process(),
                        provider_resume_private_storage.admitted_static_child_source_programs(),
                        provider_resume_private_storage.prepared_static_child_runtime_handles(),
                        provider_resume_token.root_scope_id(),
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.node_id(),
                        provider_resume_token.started_at(),
                    )?;
                let PreparedProcessActivateProviderIngressV1 {
                    ingress,
                    registration,
                    execution,
                    lifecycle_recovery,
                } = prepare_process_activate_provider_ingress_for_process_kernel_owner_v1(
                    selected_input,
                )?;
                let engine_result =
                    Self::commit_process_run_child_provider_ingress_for_live_process_session(
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.root_scope_id(),
                        ingress,
                    )?;
                register_process_activate_child(
                    registration,
                    execution,
                    lifecycle_recovery,
                    drive_context,
                )
                .map_err(|refusal| {
                    let recovery = refusal.lifecycle_recovery;
                    match refusal.execution {
                        DirectRunProcessActivateChildExecutionV1::Prepared(execution) => {
                            let cancelled = execution
                                .cancel_before_process_activate_registry_commit_for_process_lifecycle_owner_v1();
                            let (open_plan, loaded_process) = cancelled
                                .into_ready_lifecycle_custody_for_process_lifecycle_owner_v1();
                            let restoration = register_process_lifecycle_after_control_for_process_kernel_owner_v1(
                                recovery.authority,
                                recovery.activation_process_carrier,
                                loaded_process,
                                open_plan,
                                recovery.checkpoint_state,
                            );
                            match restoration {
                                Ok(()) => "process.activate execution registry refused complete custody; lifecycle restored".to_owned(),
                                Err(failure) => failure,
                            }
                        }
                        DirectRunProcessActivateChildExecutionV1::RetryOpen(open_refusal) => {
                            match register_process_activate_open_refusal(
                                recovery.authority,
                                recovery.activation_process_carrier,
                                open_refusal,
                                recovery.checkpoint_state,
                            ) {
                                Ok(()) => "process.activate exact open remains in retryable lifecycle custody".to_owned(),
                                Err(retained) => {
                                    let recovery = super::super::process_kernel_boundary::DirectRunProcessActivateLifecycleRecoveryV1 {
                                        authority: retained.authority,
                                        activation_process_carrier: retained.activation_process_carrier,
                                        checkpoint_state: retained.checkpoint_state,
                                    };
                                    let cancellation = retained
                                        .refusal
                                        .cancel_for_process_lifecycle_owner_v1();
                                    recovery
                                        .settle_with_transitioned_open_cancellation_for_process_lifecycle_owner_v1(cancellation)
                                        .to_string()
                                }
                            }
                        }
                    }
                })?;
                engine_result
            }
            DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected_host_input) => {
                let admitted_request = selected_host_input
                    .admit_host_typed_request_for_direct_run_provider_resume_owner_v1(
                        provider_execution_session,
                    )
                    .map_err(|error| error.to_string())?;
                let execution_result = if swarm_event_provider_requires_product_session_boundary(
                    admitted_request.provider_id(),
                ) {
                    crate::direct_run::event::DirectRunEventProductOwner::execute_selected_product_session_provider_effect_for_direct_run_provider_resume_owner_v1(
                        provider_resume_token.root_scope_id(),
                        provider_resume_token.live_process_session_id(),
                        provider_resume_token.node_id(),
                        admitted_request,
                        &durable_execution_core::EventAppendOccurredAtClock::from_run_started_at(
                            provider_resume_token.started_at(),
                        ),
                    )?
                } else {
                    provider_execution_session
                        .invoke_selected_provider_boundary_request_for_direct_run_owner_v1(
                            admitted_request,
                        )
                        .map_err(|error| error.to_string())?
                };
                let provider_drive_result = ProviderDriveResult::ready_from_rust_sdk_static_provider_execution_result_for_provider_drive_result_owner_v1(
                    execution_result,
                )
                .map_err(|error| error.to_string())?;
                Self::apply_provider_drive_ready_result_for_live_process_session(
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                    provider_drive_result,
                )?
            }
        };
        let route_output =
            route_engine_process_session_result_for_public_aperture_provider_resume_owner(
                provider_resume_token,
                provider_resume_private_storage,
                engine_result,
            )?;
        finish_direct_run_public_aperture_drive_output(route_output)
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
    }

    fn drive_process_start_kernel_state_public_aperture_until_terminal_or_next_step(
        mut kernel_state: DirectSwarmScriptRunKernelState,
        prepared_runtime_executable_image: EngineInstalledPreparedSessionRuntimeV1,
        aperture_label: &'static str,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let root_input = direct_run_take_process_session_root_input(&mut kernel_state)?;
        Self::drive_process_start_kernel_state_public_aperture_with_root_input_until_terminal_or_next_step(
            kernel_state,
            prepared_runtime_executable_image,
            root_input,
            aperture_label,
        )
    }

    fn drive_process_start_kernel_state_public_aperture_with_root_input_until_terminal_or_next_step(
        kernel_state: DirectSwarmScriptRunKernelState,
        prepared_runtime_executable_image: EngineInstalledPreparedSessionRuntimeV1,
        root_input: EngineVmObjectValueV1,
        aperture_label: &'static str,
    ) -> Result<
        DirectRunProcessSessionPublicApertureDriveOutputV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let durability_policy =
            direct_run_process_session_durability_policy_for_public_aperture_owner(
                kernel_state.launch_durability_policy,
            )?;
        let process_identity = kernel_state
            .current_process
            .as_ref()
            .ok_or_else(|| {
                "prepared-runtime public-aperture process session open requires the launch owner's admitted current-process identity; the fresh direct-run launch path must install it before open"
                    .to_owned()
            })?
            .process_session_identity_for_session_runtime_open_owner_v1()?;
        let live_session =
            open_process_session_v0_from_exact_static_child_dispatch_installed_prepared_runtime_for_direct_run_owner_v1(
                prepared_runtime_executable_image,
                crate::ProcessSessionInitialInputForDirectRunOwnerV1::from_root_input_for_direct_run_owner_v1(root_input),
                durability_policy,
                process_identity,
            )
            .map_err(|error| {
                format!("prepared-runtime public-aperture process session open failed: {error}")
            })?;
        let process_session_start_authority =
            Self::admit_process_session_start_with_live_session_for_direct_run_owner_v1(
                kernel_state,
                live_session,
                aperture_label,
            )
            .map_err(DirectRunProcessSessionDriveFaultV1::ProcessSessionStartAdmission)?;
        let output =
            drive_process_session_start_with_typed_drive_authority_and_owner_execution_substrate(
                process_session_start_authority,
            )?;
        finish_direct_run_public_aperture_drive_output(output)
            .map_err(DirectRunProcessSessionDriveFaultV1::Generic)
    }
}
