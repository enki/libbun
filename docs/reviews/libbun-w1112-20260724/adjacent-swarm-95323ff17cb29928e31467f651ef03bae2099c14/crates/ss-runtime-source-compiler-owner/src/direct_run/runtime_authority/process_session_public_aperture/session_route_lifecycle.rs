use super::*;

// Moved from part_001_process_session_start_public_aperture_driver.rs under ADR-2128.
pub(in crate::direct_run::direct_run_runtime_authority_refs) fn process_session_public_aperture_token_admission_operation(
    owner_kind: DirectRunKernelStateRefOwnerKind,
) -> Option<&'static str> {
    match owner_kind {
        DirectRunKernelStateRefOwnerKind::ProcessSessionStart => Some(
            "drive_continuation_ref_to_public_aperture.process_session_start.typed_token_admission",
        ),
        DirectRunKernelStateRefOwnerKind::ProcessSessionReawaken => Some(
            "drive_continuation_ref_to_public_aperture.process_session_reawaken.typed_token_admission",
        ),
        DirectRunKernelStateRefOwnerKind::ProcessSessionProjection => Some(
            "drive_continuation_ref_to_public_aperture.process_session_projection.typed_token_admission",
        ),
        _ => None,
    }
}

pub(super) fn process_session_public_aperture_take_authority_label(
    owner_kind: DirectRunKernelStateRefOwnerKind,
) -> Option<&'static str> {
    match owner_kind {
        DirectRunKernelStateRefOwnerKind::ProcessSessionStart => {
            Some("start_public_aperture_entry")
        }
        DirectRunKernelStateRefOwnerKind::ProcessSessionReawaken => {
            Some("reawaken_public_aperture_entry")
        }
        DirectRunKernelStateRefOwnerKind::ProcessSessionProjection => {
            Some("projection_public_aperture_entry")
        }
        _ => None,
    }
}

pub(super) fn process_session_public_aperture_substrate_take_operation(
    owner_kind: DirectRunKernelStateRefOwnerKind,
) -> Option<&'static str> {
    match owner_kind {
        DirectRunKernelStateRefOwnerKind::ProcessSessionStart => Some(
            "drive_continuation_ref_to_public_aperture.process_session_start.owned_execution_substrate_take",
        ),
        DirectRunKernelStateRefOwnerKind::ProcessSessionReawaken => Some(
            "drive_continuation_ref_to_public_aperture.process_session_reawaken.owned_execution_substrate_take",
        ),
        DirectRunKernelStateRefOwnerKind::ProcessSessionProjection => Some(
            "drive_continuation_ref_to_public_aperture.process_session_projection.owned_execution_substrate_take",
        ),
        _ => None,
    }
}

pub(super) fn admit_process_session_owner_execution_substrate_from_public_aperture_kernel_state_ref(
    kernel_state_ref: DirectRunPublicApertureKernelStateRef,
    owner_kind: DirectRunKernelStateRefOwnerKind,
) -> Result<DirectRunProcessSessionOwnerExecutionSubstrate, String> {
    let take_authority_label =
        process_session_public_aperture_take_authority_label(owner_kind).ok_or_else(|| {
            format!(
                "process_session_owner_execution_substrate_owner_kind_forbidden: expected process-session owner, received '{}'",
                owner_kind.as_str()
            )
        })?;
    let token_admission_operation =
        process_session_public_aperture_token_admission_operation(owner_kind).ok_or_else(|| {
            format!(
                "process_session_owner_execution_substrate_token_admission_owner_kind_forbidden: expected process-session owner, received '{}'",
                owner_kind.as_str()
            )
        })?;
    let operation =
        process_session_public_aperture_substrate_take_operation(owner_kind).ok_or_else(|| {
            format!(
                "process_session_owner_execution_substrate_take_operation_owner_kind_forbidden: expected process-session owner, received '{}'",
                owner_kind.as_str()
            )
        })?;
    kernel_state_ref.into_process_session_owner_execution_substrate_for_runtime_authority_owner(
        owner_kind,
        take_authority_label,
        token_admission_operation,
        Some(DirectRunKernelStateRefRetentionKind::Continuation),
        operation,
    )
}

pub(super) fn drive_process_session_start_with_typed_drive_authority_and_owner_execution_substrate(
    process_session_start_authority: DirectRunProcessSessionStartDriveAuthority,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, DirectRunProcessSessionDriveFaultV1> {
    DirectRunRuntimeAuthorityOwner::drive_registered_process_session_start_to_first_owner_output_for_direct_run_owner_v1(
        process_session_start_authority,
    )
    .map_err(DirectRunProcessSessionDriveFaultV1::ProcessSessionStartDrive)
}

pub(super) fn direct_run_process_session_durability_policy_for_public_aperture_owner(
    launch_durability_policy: DirectRunLaunchDurabilityPolicyAuthority,
) -> Result<ProcessSessionDurabilityPolicyV0, String> {
    match launch_durability_policy {
        DirectRunLaunchDurabilityPolicyAuthority::Volatile => {
            Ok(ProcessSessionDurabilityPolicyV0::default())
        }
        DirectRunLaunchDurabilityPolicyAuthority::DurableObservableProviderRequired => {
            Err("prepared-runtime public-aperture process start requires durable-observable session durability owner binding before non-volatile launch".to_owned())
        }
    }
}

pub(super) fn attach_pending_live_process_session_effects_for_terminal_route_owner_v1(
    engine_result: EngineProcessSessionRunResultV1,
    live_process_session_id: &str,
    root_scope_id: &str,
) -> Result<EngineProcessSessionRunResultV1, String> {
    let (observations, process_output_records) =
        DirectRunRuntimeAuthorityOwner::take_pending_process_session_effects_for_live_process_session(
            live_process_session_id,
            root_scope_id,
        )?;
    Ok(
        engine_result.with_accumulated_drive_effects_for_process_session_result_owner_v1(
            observations,
            process_output_records,
        ),
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn route_engine_process_session_result_for_public_aperture_start_owner(
    process_session_start_token: DirectRunProcessSessionStartContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    engine_result: EngineProcessSessionRunResultV1,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    match engine_result.outcome_kind() {
        "completed" | "failed" | "terminal_completed" | "terminal_failed" => {
            let engine_result =
                attach_pending_live_process_session_effects_for_terminal_route_owner_v1(
                    engine_result,
                    process_session_start_token.live_process_session_id(),
                    process_session_start_token.root_scope_id(),
                )?;
            let terminal_product = engine_result
                .into_terminal_result_product_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_start_public_aperture.terminal_result",
                )
                .map_err(|fault| {
                    fault.consume_into_message_for_direct_run_boundary_owner_v1()
                })?;
            let terminal_projection =
                DirectProcessSessionTerminalProjection::from_terminal_engine_result_for_direct_run_process_session_result_route_owner_v1(
                    terminal_product,
                )?;
            continue_after_process_session_start_result_with_typed_authority(
                process_session_start_token,
                execution_substrate,
                Box::new(DirectProcessSessionResultProjection::Terminal(
                    terminal_projection,
                )),
            )
        }
        "needs_host_resource_finalization" | "blocked_needs_host_resource_finalization" => {
            let selected_boundary =
                engine_result.into_selected_host_resource_finalization_boundary()?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::HostResourceFinalization(
                DirectRunHostResourceFinalizationNextStepV1::from_start_owner_v1(
                    process_session_start_token,
                    execution_substrate,
                    selected_boundary,
                ),
            ))
        }
        "waiting_on_liveness" | "blocked_waiting_on_liveness" => {
            let process_creation_export_readiness = engine_result
                .duplicate_process_creation_export_readiness_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_start_public_aperture.waiting_on_liveness",
                )?;
            let finished_at = process_session_start_token.started_at().to_owned();
            let liveness_projection =
                admit_direct_run_process_liveness_wait_from_typed_engine_boundary_v1(
                    engine_result,
                    "process_session_start_public_aperture.waiting_on_liveness",
                    process_creation_export_readiness,
                    finished_at,
                )?;
            continue_after_process_session_start_result_with_typed_authority(
                process_session_start_token,
                execution_substrate,
                Box::new(DirectProcessSessionResultProjection::WaitingOnLiveness(
                    liveness_projection,
                )),
            )
        }
        "needs_host_activity_effect" | "blocked_needs_host_activity_effect" => {
            let provider_resume_boundary = engine_result
                .into_selected_provider_resume_boundary_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_start_public_aperture.host_boundary",
                )?;
            let route_authority =
                DirectRunProcessSessionResultRouteAuthority::Start(process_session_start_token);
            let registry_seal_input = execution_substrate
                .into_host_activity_domain_registry_seal_input_for_converted_route(
                    &route_authority,
                    "process_session_start_public_aperture.host_boundary",
                )?;
            let seal_record = registry_seal_input
                .admit_external_provider_call_provider_resume_seal_record(
                    provider_resume_boundary,
                )?;
            let kernel_state_ref =
                DirectRunRuntimeAuthorityOwner::seal_provider_resume_kernel_state_ref_for_converted_process_session_effect_record_typed(
                    seal_record,
                )?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::NextStep(
                DirectRunProcessSessionPublicApertureNextStepOutputProductV1::from_provider_resume_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
                    kernel_state_ref,
                ),
            ))
        }
        "process_invoke_await_execution" => {
            let selected_boundary =
                engine_result.into_selected_process_invoke_await_execution_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessInvokeAwaitExecution(
                    super::super::kernel_state_substrate::DirectRunProcessInvokeAwaitExecutionNextStepV1::from_start_owner_v1(
                        process_session_start_token,
                        execution_substrate,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_run_drive_terminal" => {
            let selected_boundary =
                engine_result.into_selected_process_run_drive_terminal_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessRunDriveTerminal(
                    super::super::kernel_state_substrate::DirectRunProcessRunDriveTerminalNextStepV1::from_start_owner_v1(
                        process_session_start_token,
                        execution_substrate,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_control" => {
            let selected_boundary = engine_result.into_selected_process_control_boundary()?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::ProcessControl(
                DirectRunProcessControlNextStepV1::from_start_owner_v1(
                    process_session_start_token,
                    execution_substrate,
                    selected_boundary,
                ),
            ))
        }
        other => Err(json!({
            "kind": "process_session_start_public_aperture_outcome_kind_forbidden",
            "reason": "ProcessSessionStart public aperture reached an outcome with no finite owner route",
            "outcome_kind": other,
        })
        .to_string()),
    }
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn route_engine_process_session_result_for_public_aperture_reawaken_owner(
    process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    engine_result: EngineProcessSessionRunResultV1,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    match engine_result.outcome_kind() {
        "completed" | "failed" | "terminal_completed" | "terminal_failed" => {
            let engine_result =
                attach_pending_live_process_session_effects_for_terminal_route_owner_v1(
                    engine_result,
                    process_session_reawaken_token.live_process_session_id(),
                    process_session_reawaken_token.root_scope_id(),
                )?;
            let terminal_product = engine_result
                .into_terminal_result_product_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_reawaken_public_aperture.terminal_result",
                )
                .map_err(|fault| {
                    fault.consume_into_message_for_direct_run_boundary_owner_v1()
                })?;
            let terminal_projection =
                DirectProcessSessionTerminalProjection::from_terminal_engine_result_for_direct_run_process_session_result_route_owner_v1(
                    terminal_product,
                )?;
            continue_after_process_session_reawaken_result_with_typed_authority(
                process_session_reawaken_token,
                execution_substrate,
                Box::new(DirectProcessSessionResultProjection::Terminal(
                    terminal_projection,
                )),
            )
        }
        "needs_host_resource_finalization" | "blocked_needs_host_resource_finalization" => {
            let selected_boundary =
                engine_result.into_selected_host_resource_finalization_boundary()?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::HostResourceFinalization(
                DirectRunHostResourceFinalizationNextStepV1::from_reawaken_owner_v1(
                    process_session_reawaken_token,
                    execution_substrate,
                    selected_boundary,
                ),
            ))
        }
        "waiting_on_liveness" | "blocked_waiting_on_liveness" => {
            let process_creation_export_readiness = engine_result
                .duplicate_process_creation_export_readiness_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_reawaken_public_aperture.waiting_on_liveness",
                )?;
            let finished_at = process_session_reawaken_token.started_at().to_owned();
            let liveness_projection =
                admit_direct_run_process_liveness_wait_from_typed_engine_boundary_v1(
                    engine_result,
                    "process_session_reawaken_public_aperture.waiting_on_liveness",
                    process_creation_export_readiness,
                    finished_at,
                )?;
            continue_after_process_session_reawaken_result_with_typed_authority(
                process_session_reawaken_token,
                execution_substrate,
                Box::new(DirectProcessSessionResultProjection::WaitingOnLiveness(
                    liveness_projection,
                )),
            )
        }
        "needs_host_activity_effect" | "blocked_needs_host_activity_effect" => {
            let provider_resume_boundary = engine_result
                .into_selected_provider_resume_boundary_for_direct_run_process_session_result_route_owner_v1(
                    "process_session_reawaken_public_aperture.host_boundary",
                )?;
            let route_authority = DirectRunProcessSessionResultRouteAuthority::Reawaken(
                process_session_reawaken_token,
            );
            let registry_seal_input = execution_substrate
                .into_host_activity_domain_registry_seal_input_for_converted_route(
                    &route_authority,
                    "process_session_reawaken_public_aperture.host_boundary",
                )?;
            let seal_record = registry_seal_input
                .admit_external_provider_call_provider_resume_seal_record(
                    provider_resume_boundary,
                )?;
            let kernel_state_ref =
                DirectRunRuntimeAuthorityOwner::seal_provider_resume_kernel_state_ref_for_converted_process_session_effect_record_typed(
                    seal_record,
                )?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::NextStep(
                DirectRunProcessSessionPublicApertureNextStepOutputProductV1::from_provider_resume_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
                    kernel_state_ref,
                ),
            ))
        }
        "process_invoke_await_execution" => {
            let selected_boundary =
                engine_result.into_selected_process_invoke_await_execution_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessInvokeAwaitExecution(
                    super::super::kernel_state_substrate::DirectRunProcessInvokeAwaitExecutionNextStepV1::from_reawaken_owner_v1(
                        process_session_reawaken_token,
                        execution_substrate,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_run_drive_terminal" => {
            let selected_boundary =
                engine_result.into_selected_process_run_drive_terminal_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessRunDriveTerminal(
                    super::super::kernel_state_substrate::DirectRunProcessRunDriveTerminalNextStepV1::from_reawaken_owner_v1(
                        process_session_reawaken_token,
                        execution_substrate,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_control" => {
            let selected_boundary = engine_result.into_selected_process_control_boundary()?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::ProcessControl(
                DirectRunProcessControlNextStepV1::from_reawaken_owner_v1(
                    process_session_reawaken_token,
                    execution_substrate,
                    selected_boundary,
                ),
            ))
        }
        other => Err(json!({
            "kind": "process_session_reawaken_public_aperture_outcome_kind_forbidden",
            "reason": "ProcessSessionReawaken public aperture reached an outcome with no finite owner route",
            "outcome_kind": other,
        })
        .to_string()),
    }
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn route_engine_process_session_result_for_public_aperture_provider_resume_owner(
    provider_resume_token: DirectRunProviderResumeContinuationToken,
    provider_resume_private_storage: DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
    engine_result: EngineProcessSessionRunResultV1,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    match engine_result.outcome_kind() {
        "completed" | "failed" | "terminal_completed" | "terminal_failed" => {
            let engine_result =
                attach_pending_live_process_session_effects_for_terminal_route_owner_v1(
                    engine_result,
                    provider_resume_token.live_process_session_id(),
                    provider_resume_token.root_scope_id(),
                )?;
            let terminal_product = engine_result
                .into_terminal_result_product_for_direct_run_process_session_result_route_owner_v1(
                    "provider_resume_public_aperture.terminal_result",
                )
                .map_err(|fault| {
                    fault.consume_into_message_for_direct_run_boundary_owner_v1()
                })?;
            let terminal_projection =
                DirectProcessSessionTerminalProjection::from_terminal_engine_result_for_direct_run_process_session_result_route_owner_v1(
                    terminal_product,
                )?;
            let route_authority =
                DirectRunProcessSessionResultRouteAuthority::ProviderResume(provider_resume_token);
            let terminal_finalization = terminal_projection
                .into_terminal_finalization_product_for_direct_run_process_session_terminal_finalization_owner_v1(
                    "provider_resume_public_aperture.terminal_finalization",
                )?;
            let cleared_continuation_count =
                route_authority.clear_kernel_state_refs_for_terminal_receipt(
                    terminal_finalization
                        .terminal_finalization_receipt_for_direct_run_process_session_terminal_finalization_owner_v1(),
                )?;
            let terminal_public_output = terminal_finalization
                .into_public_output_product_for_direct_run_process_session_public_output_owner_v1(
                    cleared_continuation_count,
                );
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::TerminalPublicOutput(
                    terminal_public_output,
                ),
            )
        }
        "needs_host_activity_effect" | "blocked_needs_host_activity_effect" => {
            let provider_resume_boundary = engine_result
                .into_selected_provider_resume_boundary_for_direct_run_process_session_result_route_owner_v1(
                    "provider_resume_public_aperture.host_boundary",
                )?;
            let registry_seal_input = provider_resume_private_storage
                .into_host_activity_domain_registry_seal_input_for_provider_resume_route(
                    &provider_resume_token,
                    "provider_resume_public_aperture.host_boundary",
                )?;
            let seal_record = registry_seal_input
                .admit_external_provider_call_provider_resume_seal_record(
                    provider_resume_boundary,
                )?;
            let kernel_state_ref =
                DirectRunRuntimeAuthorityOwner::seal_provider_resume_kernel_state_ref_for_converted_process_session_effect_record_typed(
                    seal_record,
                )?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::NextStep(
                DirectRunProcessSessionPublicApertureNextStepOutputProductV1::from_provider_resume_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
                    kernel_state_ref,
                ),
            ))
        }
        "needs_host_resource_finalization" | "blocked_needs_host_resource_finalization" => {
            let selected_boundary =
                engine_result.into_selected_host_resource_finalization_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::HostResourceFinalization(
                    DirectRunHostResourceFinalizationNextStepV1::from_provider_resume_owner_v1(
                        provider_resume_token,
                        provider_resume_private_storage,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_invoke_await_execution" => {
            let selected_boundary =
                engine_result.into_selected_process_invoke_await_execution_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessInvokeAwaitExecution(
                    super::super::kernel_state_substrate::DirectRunProcessInvokeAwaitExecutionNextStepV1::from_provider_resume_owner_v1(
                        provider_resume_token,
                        provider_resume_private_storage,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_run_drive_terminal" => {
            let selected_boundary =
                engine_result.into_selected_process_run_drive_terminal_boundary()?;
            Ok(
                DirectRunProcessSessionPublicApertureRouteOutput::ProcessRunDriveTerminal(
                    super::super::kernel_state_substrate::DirectRunProcessRunDriveTerminalNextStepV1::from_provider_resume_owner_v1(
                        provider_resume_token,
                        provider_resume_private_storage,
                        selected_boundary,
                    ),
                ),
            )
        }
        "process_control" => {
            let selected_boundary = engine_result.into_selected_process_control_boundary()?;
            Ok(DirectRunProcessSessionPublicApertureRouteOutput::ProcessControl(
                DirectRunProcessControlNextStepV1::from_provider_resume_owner_v1(
                    provider_resume_token,
                    provider_resume_private_storage,
                    selected_boundary,
                ),
            ))
        }
        "waiting_on_liveness" | "blocked_waiting_on_liveness" => Err(json!({
            "kind": "provider_resume_public_aperture_result_owner_gap",
            "reason": "ProviderResume step closure reached a non-provider host-boundary result that requires its own sealed owner route; do not repair by replaying ProviderResume private storage or process-session route projections",
            "outcome_kind": engine_result.outcome_kind(),
            "provider_resume_token": provider_resume_token.diagnostic_value(),
        })
        .to_string()),
        other => Err(json!({
            "kind": "provider_resume_public_aperture_outcome_kind_forbidden",
            "reason": "ProviderResume public aperture reached an outcome with no finite owner route",
            "outcome_kind": other,
            "provider_resume_token": provider_resume_token.diagnostic_value(),
        })
        .to_string()),
    }
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_start_route_host_resource_finalization_for_owner_v1(
    process_session_start_token: DirectRunProcessSessionStartContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let engine_result =
        DirectRunRuntimeAuthorityOwner::commit_selected_host_resource_finalization_for_live_process_session(
            process_session_start_token.live_process_session_id(),
            process_session_start_token.root_scope_id(),
            provider_execution_session,
            selected_boundary,
        )?;
    route_engine_process_session_result_for_public_aperture_start_owner(
        process_session_start_token,
        execution_substrate,
        engine_result,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_reawaken_route_host_resource_finalization_for_owner_v1(
    process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let engine_result =
        DirectRunRuntimeAuthorityOwner::commit_selected_host_resource_finalization_for_live_process_session(
            process_session_reawaken_token.live_process_session_id(),
            process_session_reawaken_token.root_scope_id(),
            provider_execution_session,
            selected_boundary,
        )?;
    route_engine_process_session_result_for_public_aperture_reawaken_owner(
        process_session_reawaken_token,
        execution_substrate,
        engine_result,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_provider_resume_route_host_resource_finalization_for_owner_v1(
    provider_resume_token: DirectRunProviderResumeContinuationToken,
    provider_resume_private_storage: DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
    selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    provider_execution_session: &mut ProviderHostExecutionSession,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let engine_result =
        DirectRunRuntimeAuthorityOwner::commit_selected_host_resource_finalization_for_live_process_session(
            provider_resume_token.live_process_session_id(),
            provider_resume_token.root_scope_id(),
            provider_execution_session,
            selected_boundary,
        )?;
    route_engine_process_session_result_for_public_aperture_provider_resume_owner(
        provider_resume_token,
        provider_resume_private_storage,
        engine_result,
    )
}

pub(super) fn drive_process_session_reawaken_with_typed_drive_authority_and_owner_execution_substrate(
    process_session_reawaken_authority: DirectRunProcessSessionReawakenDriveAuthority,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let process_session_reawaken_token = process_session_reawaken_authority.token();
    let live_process_session_id = process_session_reawaken_token
        .live_process_session_id()
        .to_owned();
    let root_scope_id = process_session_reawaken_token.root_scope_id().to_owned();
    let effect_id = process_session_reawaken_token
        .process_session_effect_id()
        .to_owned();
    let engine_result =
        DirectRunRuntimeAuthorityOwner::drive_session_reawaken_to_public_aperture_boundary_for_live_process_session(
            &live_process_session_id,
            &root_scope_id,
        )?;
    match engine_result.progress_kind_for_direct_run_owner_v1() {
        "boundary" => {
            let outcome = engine_result.into_boundary_outcome_for_direct_run_owner_v1()?;
            let admitted_engine_result = DirectRunRuntimeAuthorityOwner::admit_public_aperture_boundary_outcome_for_live_process_session(
                    &live_process_session_id,
                    &root_scope_id,
                    outcome,
                    "process_session_reawaken_public_aperture_boundary",
                )?;
            return route_engine_process_session_result_for_public_aperture_reawaken_owner(
                process_session_reawaken_authority.into_token(),
                execution_substrate,
                admitted_engine_result,
            );
        }
        "forbidden_boundary" => {
            let forbidden = engine_result.into_forbidden_boundary_for_direct_run_owner_v1()?;
            let (code, message, diagnostics) =
                forbidden.into_code_message_and_diagnostics_for_direct_run_owner_v1();
            let diagnostics = diagnostics
                .into_value_forbidden_require_public_diagnostic_projection_authority(
                    EnginePublicDiagnosticProjectionAuthority::public_process_session_outcome(
                        "session_reawaken_public_aperture_forbidden_boundary",
                    ),
                );
            return Err(json!({
                "kind": code,
                "reason": message,
                "root_scope_id": root_scope_id,
                "live_process_session_ref": live_process_session_id,
                "effect_id": effect_id,
                "diagnostics": diagnostics,
            })
            .to_string());
        }
        other => {
            return Err(json!({
                    "kind": "process_session_reawaken_public_aperture_progress_kind_forbidden",
                    "reason": "session-runtime returned a public-aperture progress product with no direct-run route",
                    "progress_kind": other,
                    "root_scope_id": root_scope_id,
                    "live_process_session_ref": live_process_session_id,
                    "effect_id": effect_id,
                })
                .to_string());
        }
    }
}

// Moved from part_003_process_session_projection.rs under ADR-2128.
pub(super) fn drive_process_session_projection_with_typed_drive_authority_and_owner_execution_substrate(
    process_session_projection_authority: DirectRunProcessSessionProjectionDriveAuthority,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let root_scope_id = process_session_projection_authority
        .token()
        .root_scope_id()
        .to_owned();
    require_process_session_projection_typed_token_holds_frame(
        process_session_projection_authority.token(),
        "drive_continuation_ref_to_public_aperture.process_session_projection",
    )?;
    if execution_substrate
        .as_kernel_state_for_execution_diagnostics()
        .is_some()
    {
        return Err(json!({
            "kind": "process_session_projection_owner_drive_kernel_state_bag_forbidden",
            "reason": "ADR-2039 ProcessSessionProjection owner drive must consume the projection frame from the typed token and must not retain a boxed kernel-state bag",
            "root_scope_id": root_scope_id,
            "process_session_projection_token": process_session_projection_authority.token().diagnostic_value(),
        })
        .to_string());
    }
    let projection_frame_product = process_session_projection_authority
        .take_projection_frame_for_owner_drive(
            "drive_continuation_ref_to_public_aperture.process_session_projection",
        )?;
    continue_after_process_session_projection_result_with_typed_authority(
        projection_frame_product,
        execution_substrate,
    )
}

pub(super) fn continue_after_process_session_projection_result_with_typed_authority(
    projection_frame_product: super::super::kernel_state_substrate::DirectRunProcessSessionProjectionOwnerDriveFrameProduct,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let (process_session_projection_token, host_interactions, projection, provider_effect_state_id) =
        projection_frame_product.into_result_route_inputs_for_public_aperture_owner_v1();
    let _ = provider_effect_state_id;
    continue_after_process_session_result_with_typed_route_authority(
        DirectRunProcessSessionResultRouteAuthority::Projection(process_session_projection_token),
        execution_substrate,
        host_interactions,
        projection,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_process_session_continuation_chain_with_typed_drive_authority_and_owner_execution_substrate(
    initial_drive_authority: DirectRunProcessSessionContinuationDriveAuthority,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
) -> Result<DirectRunProcessSessionPublicApertureDriveOutputV1, DirectRunProcessSessionDriveFaultV1>
{
    let mut drive_authority = initial_drive_authority;
    let mut execution_substrate = execution_substrate;
    loop {
        let output = match drive_authority {
            DirectRunProcessSessionContinuationDriveAuthority::Start(token) => {
                let authority = DirectRunProcessSessionStartDriveAuthority::from_consumed_start_record_and_owner_execution_substrate(
                    token,
                    execution_substrate,
                );
                drive_process_session_start_with_typed_drive_authority_and_owner_execution_substrate(
                    authority,
                )
                ?
            }
            DirectRunProcessSessionContinuationDriveAuthority::Reawaken(token) => {
                let authority = DirectRunProcessSessionReawakenDriveAuthority::new(token);
                drive_process_session_reawaken_with_typed_drive_authority_and_owner_execution_substrate(
                    authority,
                    execution_substrate,
                )?
            }
            DirectRunProcessSessionContinuationDriveAuthority::Projection(token) => {
                let authority = DirectRunProcessSessionProjectionDriveAuthority::new(token);
                drive_process_session_projection_with_typed_drive_authority_and_owner_execution_substrate(
                    authority,
                    execution_substrate,
                )?
            }
        };
        let output = finish_direct_run_public_aperture_drive_output(output)?;
        match output.into_closed_drive_state_for_direct_run_runtime_execution_owner_v1() {
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::Terminal(output) => {
                return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_terminal_output_for_direct_run_process_session_public_aperture_owner_v1(output));
            }
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::HostResourceFinalization(output) => {
                return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_host_resource_finalization_for_direct_run_process_session_public_aperture_owner_v1(output));
            }
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessInvokeAwaitExecution(output) => {
                return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_invoke_await_execution_for_direct_run_process_session_public_aperture_owner_v1(output));
            }
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessRunDriveTerminal(output) => {
                return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_run_drive_terminal_for_direct_run_process_session_public_aperture_owner_v1(output));
            }
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessControl(output) => {
                return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_control_for_direct_run_process_session_public_aperture_owner_v1(output));
            }
            super::super::kernel_state_substrate::DirectRunProcessSessionPublicApertureClosedDriveStateV1::NextStep(output) => {
                let next_owner_kind = output
                    .owner_kind_for_direct_run_process_session_public_aperture_owner_v1(
                        "process_session_public_aperture.continuation_chain.next_step_owner",
                    )?;
                let Some(token_admission_operation) =
                    process_session_public_aperture_token_admission_operation(next_owner_kind)
                else {
                    // Non-process-session next steps (ProviderResume,
                    // ProcessLivenessDrain, ...) are sealed products owned by the caller's
                    // finite step consumers; the process-session continuation chain must hand
                    // them back, not re-admit them as process-session continuations.
                    return Ok(DirectRunProcessSessionPublicApertureDriveOutputV1::from_next_step_for_direct_run_process_session_public_aperture_owner_v1(output));
                };
                let kernel_state_ref = output
                    .into_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1();
                drive_authority = kernel_state_ref
                    .process_session_continuation_drive_authority_for_runtime_authority_owner(
                        next_owner_kind,
                        token_admission_operation,
                    )?;
                execution_substrate =
                    admit_process_session_owner_execution_substrate_from_public_aperture_kernel_state_ref(
                        kernel_state_ref,
                        next_owner_kind,
                    )?;
            }
        }
    }
}

pub(super) fn drive_process_session_continuation_ref_to_public_aperture_with_typed_live_authority(
    kernel_state_ref: DirectRunPublicApertureKernelStateRef,
) -> Result<DirectRunProcessSessionPublicApertureDriveOutputV1, DirectRunProcessSessionDriveFaultV1>
{
    let owner_kind = kernel_state_ref
        .owner_kind("process_session_public_aperture.continuation_ref.owner_kind")?;
    if !matches!(
        owner_kind,
        DirectRunKernelStateRefOwnerKind::ProcessSessionStart
            | DirectRunKernelStateRefOwnerKind::ProcessSessionReawaken
            | DirectRunKernelStateRefOwnerKind::ProcessSessionProjection
    ) {
        return Err(format!(
            "direct_run_public_aperture_next_step_requires_typed_step_consumer_for_owner_kind: {}",
            owner_kind.as_str()
        )
        .into());
    }
    let drive_authority = kernel_state_ref
        .process_session_continuation_drive_authority_for_runtime_authority_owner(
            owner_kind,
            process_session_public_aperture_token_admission_operation(owner_kind).ok_or_else(
                || {
                    format!(
                        "process_session_public_aperture_continuation_ref_owner_kind_forbidden: {}",
                        owner_kind.as_str()
                    )
                },
            )?,
        )?;
    let execution_substrate =
        admit_process_session_owner_execution_substrate_from_public_aperture_kernel_state_ref(
            kernel_state_ref,
            owner_kind,
        )?;
    drive_process_session_continuation_chain_with_typed_drive_authority_and_owner_execution_substrate(
        drive_authority,
        execution_substrate,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn finish_direct_run_public_aperture_drive_output(
    output: DirectRunProcessSessionPublicApertureRouteOutput,
) -> Result<DirectRunProcessSessionPublicApertureDriveOutputV1, String> {
    match output {
        DirectRunProcessSessionPublicApertureRouteOutput::NextStep(output) => {
            Ok(
                DirectRunProcessSessionPublicApertureDriveOutputV1::from_next_step_for_direct_run_process_session_public_aperture_owner_v1(
                    output,
                ),
            )
        }
        DirectRunProcessSessionPublicApertureRouteOutput::HostResourceFinalization(output) => Ok(
            DirectRunProcessSessionPublicApertureDriveOutputV1::from_host_resource_finalization_for_direct_run_process_session_public_aperture_owner_v1(output),
        ),
        DirectRunProcessSessionPublicApertureRouteOutput::ProcessInvokeAwaitExecution(output) => Ok(
            DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_invoke_await_execution_for_direct_run_process_session_public_aperture_owner_v1(output),
        ),
        DirectRunProcessSessionPublicApertureRouteOutput::ProcessRunDriveTerminal(output) => Ok(
            DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_run_drive_terminal_for_direct_run_process_session_public_aperture_owner_v1(output),
        ),
        DirectRunProcessSessionPublicApertureRouteOutput::ProcessControl(output) => Ok(
            DirectRunProcessSessionPublicApertureDriveOutputV1::from_process_control_for_direct_run_process_session_public_aperture_owner_v1(output),
        ),
        DirectRunProcessSessionPublicApertureRouteOutput::TerminalPublicOutput(output) => {
            output
                .into_public_aperture_output_emission_product_for_direct_run_process_session_public_output_owner_v1()
                .map(
                    DirectRunProcessSessionPublicApertureDriveOutputV1::from_terminal_output_for_direct_run_process_session_public_aperture_owner_v1,
                )
        }
    }
}
