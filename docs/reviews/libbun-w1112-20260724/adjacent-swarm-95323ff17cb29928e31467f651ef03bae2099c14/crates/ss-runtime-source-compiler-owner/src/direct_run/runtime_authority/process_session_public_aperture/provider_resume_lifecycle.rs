use super::*;

pub(super) fn exact_process_builtin_contract_identity_for_kernel_internal_route(
    identity_cell: &'static OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    >,
    export_name: &'static str,
) -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    let identity = identity_cell.get_or_init(|| {
        let binding = RustSdkBuiltinProviderCatalogue::builtin_for_static_provider_host_owner_v1()
            .map_err(|error| error.to_string())?
            .into_provider_bindings()
            .into_iter()
            .find(|binding| {
                binding.package_specifier() == SWARM_PROCESS_MODULE_ID
                    && binding.export_name() == export_name
            })
            .ok_or_else(|| {
                format!(
                    "builtin provider catalogue has no exact @swarm/process:{export_name} binding"
                )
            })?;
        binding
            .sealed_identity_for_static_provider_host_owner_v1()
            .ok_or_else(|| {
                format!(
                    "builtin @swarm/process:{export_name} binding did not retain its sealed contract identity"
                )
            })
    });
    match identity {
        Ok(identity) => Ok((identity, export_name)),
        Err(error) => Err(error.clone()),
    }
}

pub(super) fn exact_process_load_builtin_contract_identity_for_kernel_internal_route() -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_LOAD_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_LOAD_CONTRACT,
        SWARM_PROCESS_LOAD_EXPORT,
    )
}

pub(super) fn exact_process_activate_builtin_contract_identity_for_kernel_internal_route() -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_ACTIVATE_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_ACTIVATE_CONTRACT,
        SWARM_PROCESS_ACTIVATE_EXPORT,
    )
}

pub(super) fn exact_process_checkpoint_builtin_contract_identity_for_kernel_internal_route()
-> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_CHECKPOINT_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_CHECKPOINT_CONTRACT,
        SWARM_PROCESS_CHECKPOINT_EXPORT,
    )
}

pub(super) fn exact_process_restore_builtin_contract_identity_for_kernel_internal_route() -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_RESTORE_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_RESTORE_CONTRACT,
        SWARM_PROCESS_RESTORE_EXPORT,
    )
}

pub(super) fn exact_process_invoke_builtin_contract_identity_for_kernel_internal_route() -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_INVOKE_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_INVOKE_CONTRACT,
        SWARM_PROCESS_INVOKE_EXPORT,
    )
}

pub(super) fn exact_process_run_builtin_contract_identity_for_kernel_internal_route() -> Result<
    (
        &'static swarm_capability_model::CapabilityContractIdentity,
        &'static str,
    ),
    String,
> {
    static PROCESS_RUN_CONTRACT: OnceLock<
        Result<swarm_capability_model::CapabilityContractIdentity, String>,
    > = OnceLock::new();
    exact_process_builtin_contract_identity_for_kernel_internal_route(
        &PROCESS_RUN_CONTRACT,
        SWARM_PROCESS_RUN_EXPORT,
    )
}

pub(super) enum DirectRunSelectedProviderResumeRouteV1 {
    ProcessLoad(SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProcessActivate(SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1),
    ProcessCheckpoint(SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1),
    ProcessRestore(SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1),
    ProcessInvoke(SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProcessRun(SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProviderHost(SelectedProviderResumeHostInputForDirectRunOwnerV1),
}

pub(super) fn select_provider_resume_route_for_direct_run_owner_v1(
    selected_input: SelectedProviderResumeHostInputForDirectRunOwnerV1,
) -> Result<DirectRunSelectedProviderResumeRouteV1, String> {
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_LOAD_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_load_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(_selected) => {
                Err("exact process.load provider work reached ordinary kernel routing".to_owned())
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_ACTIVATE_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_activate_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessActivate(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_CHECKPOINT_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_checkpoint_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessCheckpoint(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_RESTORE_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_restore_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(_selected) => Err(
                "exact process.restore provider work reached ordinary kernel routing".to_owned(),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_INVOKE_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_invoke_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(_selected) => {
                Err("exact process.invoke provider work reached ordinary kernel routing".to_owned())
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    if selected_input.contract_package_export_matches_for_direct_run_routing_observation_v1(
        SWARM_PROCESS_MODULE_ID,
        SWARM_PROCESS_RUN_EXPORT,
    ) {
        let (identity, operation) =
            exact_process_run_builtin_contract_identity_for_kernel_internal_route()?;
        return match selected_input
            .select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
                identity, operation,
            )? {
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(selected) => {
                Ok(DirectRunSelectedProviderResumeRouteV1::ProcessRun(selected))
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessLoad(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessInvoke(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProcessRestore(selected),
            ),
            SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(_selected) => {
                Err("exact process.run provider work reached ordinary kernel routing".to_owned())
            }
            SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(selected) => Ok(
                DirectRunSelectedProviderResumeRouteV1::ProviderHost(selected),
            ),
        };
    }
    Ok(DirectRunSelectedProviderResumeRouteV1::ProviderHost(
        selected_input,
    ))
}

pub(super) struct PreparedProcessInvokeProviderIngressV1 {
    pub(super) ingress: crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    pub(super) registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    pub(super) execution:
        crate::direct_run::DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority,
}

pub(super) struct PreparedProcessRunProviderIngressV1 {
    pub(super) ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
    pub(super) registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    pub(super) execution:
        crate::direct_run::DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority,
}

pub(super) struct PreparedProcessActivateProviderIngressV1 {
    pub(super) ingress: crate::ProcessRunChildProviderIngressForDirectRunOwnerV1,
    pub(super) registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    pub(super) execution: DirectRunProcessActivateChildExecutionV1,
    pub(super) lifecycle_recovery:
        super::super::process_kernel_boundary::DirectRunProcessActivateLifecycleRecoveryV1,
}

pub(super) fn prepare_process_invoke_provider_ingress_for_process_kernel_owner_v1(
    admitted_launch: crate::AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1,
) -> Result<PreparedProcessInvokeProviderIngressV1, String> {
    let (open_plan, callable_input, input, options, output_settlement_authority) =
        admitted_launch.consume_for_direct_run_process_kernel_owner_v1();
    let execution = crate::direct_run::DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority::from_admitted_process_invoke_launch_for_process_kernel_owner_v1(
        open_plan,
        callable_input,
        input,
        options,
    );
    let (output, registration) =
        mint_process_invoke_execution_carrier_for_durable_direct_run_owner_v1();
    let ingress = output_settlement_authority
        .admit_process_invoke_execution_output_for_direct_run_owner_v1(output);
    Ok(PreparedProcessInvokeProviderIngressV1 {
        ingress,
        registration,
        execution,
    })
}

pub(super) fn prepare_process_run_provider_ingress_for_process_kernel_owner_v1(
    admitted_launch: crate::AdmittedProcessRunChildLaunchForDirectRunOwnerV1,
) -> Result<PreparedProcessRunProviderIngressV1, String> {
    let (open_plan, program_input, options, output_settlement_authority) =
        admitted_launch.consume_for_direct_run_process_kernel_owner_v1();
    let execution = crate::direct_run::DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority::from_admitted_process_run_launch_for_process_kernel_owner_v1(
        open_plan,
        program_input,
        options,
    );
    let (output, registration) = mint_process_run_child_carrier_for_durable_direct_run_owner_v1();
    let ingress =
        output_settlement_authority.admit_process_run_child_output_for_direct_run_owner_v1(output);
    Ok(PreparedProcessRunProviderIngressV1 {
        ingress,
        registration,
        execution,
    })
}

pub(super) fn prepare_process_activate_provider_ingress_for_process_kernel_owner_v1(
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
) -> Result<PreparedProcessActivateProviderIngressV1, String> {
    let (provider_input, output_settlement_authority, _invocation_fingerprint) = selected_input
        .into_provider_input_output_settlement_and_invocation_fingerprint_for_direct_run_owner_v1();
    let ProviderValue::Array(mut positional) = provider_input else {
        return Err("kernel-internal process:activate requires positional arguments".to_owned());
    };
    if positional.len() != 1 {
        return Err(format!(
            "kernel-internal process:activate requires exactly one argument; received {}",
            positional.len()
        ));
    }
    let ProviderValue::Object(mut args) = positional
        .pop()
        .expect("activate arity checked before consuming input")
    else {
        return Err("kernel-internal process:activate argument must be an object".to_owned());
    };
    if args.len() != 1 || !args.contains_key("process") {
        return Err(
            "kernel-internal process:activate argument must contain exactly 'process'".to_owned(),
        );
    }
    let ProviderValue::CurrentProcess(process) = args
        .remove("process")
        .expect("activate process field presence checked")
    else {
        return Err(
            "kernel-internal process:activate requires an exact owner-issued Process carrier"
                .to_owned(),
        );
    };
    let (authority, activation_process_carrier, checkpoint_state, execution) =
        match select_registered_process_lifecycle(process) {
            ProcessLifecycleRegistrySelection::Joined(matched) => {
                let MatchedRegisteredProcessLifecycle {
                    authority,
                    subject: _exact_process_correspondence,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                } = matched;
                let execution = match crate::direct_run::authority_kernel::prepared_runtime::DirectRunPreparedProcessActivateExactStaticChildExecutionAuthorityV1::from_process_activate_loaded_lifecycle_for_process_kernel_owner_v1(
                open_plan,
                loaded_process,
            ) {
                Ok(execution) => execution,
                Err(refusal) => {
                    let cancelled = refusal.cancel_for_process_lifecycle_owner_v1();
                    let (open_plan, loaded_process) = cancelled
                        .into_ready_lifecycle_custody_for_process_lifecycle_owner_v1();
                    register_process_lifecycle_after_control_for_process_kernel_owner_v1(
                        authority,
                        activation_process_carrier,
                        loaded_process,
                        open_plan,
                        checkpoint_state,
                    )?;
                    return Err("process.activate exact lifecycle preparation was refused and restored"
                        .to_owned());
                }
            };
                (
                    authority,
                    activation_process_carrier,
                    checkpoint_state,
                    DirectRunProcessActivateChildExecutionV1::Prepared(execution),
                )
            }
            ProcessLifecycleRegistrySelection::OpenRefused(matched) => (
                matched.authority,
                matched.activation_process_carrier,
                matched.checkpoint_state,
                DirectRunProcessActivateChildExecutionV1::RetryOpen(matched.refusal),
            ),
            ProcessLifecycleRegistrySelection::Unmatched { .. } => {
                return Err(
                    "kernel-internal process:activate Process did not join its lifecycle owner"
                        .to_owned(),
                );
            }
            ProcessLifecycleRegistrySelection::RegistryBorrowed { .. } => {
                return Err("kernel-internal process lifecycle registry is borrowed".to_owned());
            }
            ProcessLifecycleRegistrySelection::RegistryUnavailable { .. } => {
                return Err("kernel-internal process lifecycle registry is unavailable".to_owned());
            }
        };
    let running_process = activation_process_carrier.duplicate_for_session_runtime_owner_v1();
    let (output, registration) =
        mint_process_run_child_carrier_with_process_for_durable_direct_run_owner_v1(
            running_process,
        );
    let ingress =
        output_settlement_authority.admit_process_run_child_output_for_direct_run_owner_v1(output);
    Ok(PreparedProcessActivateProviderIngressV1 {
        ingress,
        registration,
        execution,
        lifecycle_recovery:
            super::super::process_kernel_boundary::DirectRunProcessActivateLifecycleRecoveryV1 {
                authority,
                activation_process_carrier,
                checkpoint_state,
            },
    })
}

pub(crate) struct DirectRunProcessLoadExecutionRefusalV1 {
    cause: DirectRunProcessLoadExecutionRefusalCauseV1,
}

pub(super) enum DirectRunProcessLoadExecutionRefusalCauseV1 {
    Preparation(String),
    OutputPreflight {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        refusal: crate::KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    },
    LifecycleRegistryBorrowed {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        settlement: crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    },
    LifecycleRegistryUnavailable {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        settlement: crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    },
}

pub(super) fn cancel_kernel_internal_plain_output_preflight_refusal_for_direct_run_boundary_owner_v1(
    refusal: crate::KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
) -> String {
    match refusal.retry_for_direct_run_owner_v1() {
        Ok(settlement) => {
            settlement.cancel_for_direct_run_owner_v1();
            "plain-output settlement preflight succeeded on boundary retry and was cancelled"
                .to_owned()
        }
        Err(refusal) => refusal
            .cancel_into_fault_for_direct_run_owner_v1()
            .to_string(),
    }
}

impl DirectRunProcessLoadExecutionRefusalV1 {
    fn preparation(failure: String) -> Self {
        Self {
            cause: DirectRunProcessLoadExecutionRefusalCauseV1::Preparation(failure),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        match self.cause {
            DirectRunProcessLoadExecutionRefusalCauseV1::Preparation(failure) => failure,
            DirectRunProcessLoadExecutionRefusalCauseV1::OutputPreflight {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                refusal,
            } => {
                let fault =
                    cancel_kernel_internal_plain_output_preflight_refusal_for_direct_run_boundary_owner_v1(
                        refusal,
                    );
                let _retained_lifecycle = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                );
                format!("process.load output settlement preflight was refused: {fault}")
            }
            DirectRunProcessLoadExecutionRefusalCauseV1::LifecycleRegistryBorrowed {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                settlement,
            } => {
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_lifecycle = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                );
                "process lifecycle registry remained borrowed during process.load".to_owned()
            }
            DirectRunProcessLoadExecutionRefusalCauseV1::LifecycleRegistryUnavailable {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                settlement,
            } => {
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_lifecycle = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                );
                "process lifecycle registry was unavailable during process.load".to_owned()
            }
        }
    }
}

pub(super) fn execute_kernel_internal_process_load_with_static_child_context_for_provider_resume_owner_v1(
    admitted_registration: crate::direct_run::DirectRunAdmittedProcessLoadLifecycleRegistrationV1,
    identity_scope: &str,
    identity_suffix: &str,
    current_process: &crate::direct_run::DirectSwarmScriptRunCurrentProcess,
) -> Result<ProviderDriveResult, DirectRunProcessLoadExecutionRefusalV1> {
    let crate::direct_run::DirectRunAdmittedProcessLoadLifecycleRegistrationV1 {
        open_plan,
        output_settlement_authority,
    } = admitted_registration;
    let program_id = open_plan
        .reusable_site_plan
        .staged_child_runtime_handle
        .live_entry
        .program_admission
        .program_id()
        .to_owned();
    let identity_preimage = json!({
        "schema": "swarm.direct_run.process_load.identity_preimage.v1",
        "identity_scope": identity_scope,
        "identity_suffix": identity_suffix,
        "program_id": program_id,
    });
    let mut hasher = Sha256::new();
    hasher.update(identity_preimage.to_string().as_bytes());
    let identity_fragment = hex::encode(hasher.finalize());
    let process_id = format!("process.loaded.{identity_fragment}");
    let root_scope_id = format!("scope.loaded.{identity_fragment}.root");
    let host_id = current_process
        .host_id_for_kernel_internal_process_load_owner_v1()
        .to_owned();
    let process_projection = ProviderValue::Object(
        BTreeMap::from([
            (
                "process_id".to_owned(),
                ProviderValue::String(process_id.clone()),
            ),
            (
                "root_scope_id".to_owned(),
                ProviderValue::String(root_scope_id.clone()),
            ),
            ("host_id".to_owned(), ProviderValue::String(host_id.clone())),
            (
                "lifecycle_state".to_owned(),
                ProviderValue::String("loaded".to_owned()),
            ),
        ])
        .into(),
    );
    let lifecycle_authority = swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1::mint_for_session_process_authority_owner_v1();
    let process_carrier = lifecycle_authority
        .mint_current_process_carrier_for_session_process_authority_owner_v1(
            process_id.clone(),
            root_scope_id.clone(),
            host_id.clone(),
            process_projection,
        )
        .map_err(|_| {
            DirectRunProcessLoadExecutionRefusalV1::preparation(
                "kernel-internal process:load projection unexpectedly contained authority cargo"
                    .to_owned(),
            )
        })?;
    let activation_process_carrier = process_carrier.duplicate_for_session_runtime_owner_v1();
    let loaded_process =
        crate::direct_run::DirectSwarmScriptRunCurrentProcess::admitted_for_loaded_process_lifecycle_owner_v1(
            process_id.clone(),
            root_scope_id.clone(),
            host_id,
            program_id,
        )
        .map_err(DirectRunProcessLoadExecutionRefusalV1::preparation)?;
    let checkpoint_state =
        crate::ProcessLivenessCheckpointStateV1::running_for_process_lifecycle_owner_v1(
            root_scope_id,
            process_id,
        )
        .map_err(DirectRunProcessLoadExecutionRefusalV1::preparation)?;
    let settlement = match output_settlement_authority
        .preflight_kernel_internal_plain_output_settlement_for_direct_run_owner_v1(
            ProviderValue::CurrentProcess(process_carrier),
        ) {
        Ok(settlement) => settlement,
        Err(refusal) => {
            return Err(DirectRunProcessLoadExecutionRefusalV1 {
                cause: DirectRunProcessLoadExecutionRefusalCauseV1::OutputPreflight {
                    authority: lifecycle_authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                    refusal,
                },
            });
        }
    };
    match super::super::process_kernel_boundary::register_process_lifecycle(
        lifecycle_authority,
        activation_process_carrier,
        loaded_process,
        open_plan,
        checkpoint_state,
    ) {
        super::super::process_kernel_boundary::ProcessLifecycleRegistrationAdmission::Registered => Ok(
            ProviderDriveResult::ready_from_preflighted_kernel_internal_plain_output_for_provider_drive_result_owner_v1(
                settlement,
            ),
        ),
        super::super::process_kernel_boundary::ProcessLifecycleRegistrationAdmission::RegistryBorrowed {
            authority,
            activation_process_carrier,
            loaded_process,
            open_plan,
            checkpoint_state,
        } => Err(DirectRunProcessLoadExecutionRefusalV1 {
            cause: DirectRunProcessLoadExecutionRefusalCauseV1::LifecycleRegistryBorrowed {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                settlement,
            },
        }),
        super::super::process_kernel_boundary::ProcessLifecycleRegistrationAdmission::RegistryUnavailable {
            authority,
            activation_process_carrier,
            loaded_process,
            open_plan,
            checkpoint_state,
        } => Err(DirectRunProcessLoadExecutionRefusalV1 {
            cause: DirectRunProcessLoadExecutionRefusalCauseV1::LifecycleRegistryUnavailable {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                settlement,
            },
        }),
    }
}

pub(super) fn execute_kernel_internal_process_load_for_provider_resume_owner_v1(
    admitted_registration: crate::direct_run::DirectRunAdmittedProcessLoadLifecycleRegistrationV1,
    provider_resume_token: &DirectRunProviderResumeContinuationToken,
    provider_resume_private_storage: &DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
) -> Result<ProviderDriveResult, DirectRunProcessLoadExecutionRefusalV1> {
    execute_kernel_internal_process_load_with_static_child_context_for_provider_resume_owner_v1(
        admitted_registration,
        provider_resume_token.run_namespace(),
        provider_resume_token.run_suffix(),
        provider_resume_private_storage.current_process(),
    )
}

pub(crate) struct DirectRunProcessCheckpointExecutionRefusalV1 {
    cause: DirectRunProcessCheckpointExecutionRefusalCauseV1,
}

pub(super) enum DirectRunProcessCheckpointExecutionRefusalCauseV1 {
    Preparation(String),
    OutputPreflight {
        authority: swarm_provider_value_model::ProcessCheckpointAuthorityForDirectRunOwnerV1,
        lifecycle: MatchedRegisteredProcessLifecycle,
        refusal: crate::KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    },
    CheckpointRegistryBorrowed {
        authority: swarm_provider_value_model::ProcessCheckpointAuthorityForDirectRunOwnerV1,
        lifecycle: MatchedRegisteredProcessLifecycle,
        settlement: crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    },
    CheckpointRegistryUnavailable {
        authority: swarm_provider_value_model::ProcessCheckpointAuthorityForDirectRunOwnerV1,
        lifecycle: MatchedRegisteredProcessLifecycle,
        settlement: crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    },
}

impl DirectRunProcessCheckpointExecutionRefusalV1 {
    fn preparation(failure: String) -> Self {
        Self {
            cause: DirectRunProcessCheckpointExecutionRefusalCauseV1::Preparation(failure),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        match self.cause {
            DirectRunProcessCheckpointExecutionRefusalCauseV1::Preparation(failure) => failure,
            DirectRunProcessCheckpointExecutionRefusalCauseV1::OutputPreflight {
                authority,
                lifecycle,
                refusal,
            } => {
                let fault =
                    cancel_kernel_internal_plain_output_preflight_refusal_for_direct_run_boundary_owner_v1(
                        refusal,
                    );
                let _retained_checkpoint_custody = (authority, lifecycle);
                format!("process.checkpoint output settlement preflight was refused: {fault}")
            }
            DirectRunProcessCheckpointExecutionRefusalCauseV1::CheckpointRegistryBorrowed {
                authority,
                lifecycle,
                settlement,
            } => {
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_checkpoint_custody = (authority, lifecycle);
                "kernel-internal process:checkpoint registry is already borrowed".to_owned()
            }
            DirectRunProcessCheckpointExecutionRefusalCauseV1::CheckpointRegistryUnavailable {
                authority,
                lifecycle,
                settlement,
            } => {
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_checkpoint_custody = (authority, lifecycle);
                "kernel-internal process:checkpoint registry is unavailable".to_owned()
            }
        }
    }
}

pub(super) fn execute_kernel_internal_process_checkpoint_for_provider_resume_owner_v1(
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
) -> Result<ProviderDriveResult, DirectRunProcessCheckpointExecutionRefusalV1> {
    let (provider_input, output_settlement_authority, invocation_fingerprint) = selected_input
        .into_provider_input_output_settlement_and_invocation_fingerprint_for_direct_run_owner_v1();
    let ProviderValue::Array(mut positional) = provider_input else {
        return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            "kernel-internal process:checkpoint requires positional arguments".to_owned(),
        ));
    };
    if positional.len() != 1 {
        return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            format!(
                "kernel-internal process:checkpoint requires exactly one argument; received {}",
                positional.len()
            ),
        ));
    }
    let ProviderValue::Object(mut args) = positional
        .pop()
        .expect("checkpoint arity checked before consuming input")
    else {
        return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            "kernel-internal process:checkpoint argument must be an object".to_owned(),
        ));
    };
    if args.len() != 1 || !args.contains_key("process") {
        return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            "kernel-internal process:checkpoint argument must contain exactly 'process'".to_owned(),
        ));
    }
    let ProviderValue::CurrentProcess(process) = args
        .remove("process")
        .expect("checkpoint process field presence checked")
    else {
        return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            "kernel-internal process:checkpoint requires an exact owner-issued Process carrier"
                .to_owned(),
        ));
    };
    let matched = match select_registered_process_lifecycle(process) {
        ProcessLifecycleRegistrySelection::Joined(matched) => matched,
        ProcessLifecycleRegistrySelection::OpenRefused(matched) => {
            retain_process_activate_open_refusal_for_process_lifecycle_owner_v1(matched)
                .map_err(DirectRunProcessCheckpointExecutionRefusalV1::preparation)?;
            return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
                "kernel-internal process:checkpoint cannot consume retryable activation-open custody"
                    .to_owned(),
            ));
        }
        ProcessLifecycleRegistrySelection::Unmatched { .. } => {
            return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
                "kernel-internal process:checkpoint Process carrier did not join its lifecycle owner"
                    .to_owned(),
            ));
        }
        ProcessLifecycleRegistrySelection::RegistryBorrowed { .. } => {
            return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
                "kernel-internal process lifecycle registry is borrowed".to_owned(),
            ));
        }
        ProcessLifecycleRegistrySelection::RegistryUnavailable { .. } => {
            return Err(DirectRunProcessCheckpointExecutionRefusalV1::preparation(
                "kernel-internal process lifecycle registry is unavailable".to_owned(),
            ));
        }
    };
    let projection = matched
        .loaded_process
        .checkpoint_projection_for_process_lifecycle_owner_v1(invocation_fingerprint);
    let roles = swarm_provider_value_model::ProcessCheckpointRolesForDirectRunOwnerV1::mint_for_direct_run_owner_v1(
        projection,
    )
    .map_err(|_| {
        DirectRunProcessCheckpointExecutionRefusalV1::preparation(
            "kernel-internal process:checkpoint projection was not data-only".to_owned(),
        )
    })?;
    let (checkpoint_authority, checkpoint_carrier) =
        roles.into_authority_and_carrier_for_direct_run_owner_v1();
    let settlement = match output_settlement_authority
        .preflight_kernel_internal_plain_output_settlement_for_direct_run_owner_v1(
            ProviderValue::ProcessCheckpoint(checkpoint_carrier),
        ) {
        Ok(settlement) => settlement,
        Err(refusal) => {
            return Err(DirectRunProcessCheckpointExecutionRefusalV1 {
                cause: DirectRunProcessCheckpointExecutionRefusalCauseV1::OutputPreflight {
                    authority: checkpoint_authority,
                    lifecycle: matched,
                    refusal,
                },
            });
        }
    };
    match register_process_checkpoint(checkpoint_authority, matched) {
        ProcessCheckpointRegistrationAdmission::Registered => Ok(
            ProviderDriveResult::ready_from_preflighted_kernel_internal_plain_output_for_provider_drive_result_owner_v1(
                settlement,
            ),
        ),
        ProcessCheckpointRegistrationAdmission::RegistryBorrowed {
            authority,
            lifecycle,
        } => Err(DirectRunProcessCheckpointExecutionRefusalV1 {
            cause: DirectRunProcessCheckpointExecutionRefusalCauseV1::CheckpointRegistryBorrowed {
                authority,
                lifecycle,
                settlement,
            },
        }),
        ProcessCheckpointRegistrationAdmission::RegistryUnavailable {
            authority,
            lifecycle,
        } => Err(DirectRunProcessCheckpointExecutionRefusalV1 {
            cause:
                DirectRunProcessCheckpointExecutionRefusalCauseV1::CheckpointRegistryUnavailable {
                    authority,
                    lifecycle,
                    settlement,
                },
        }),
    }
}

pub(super) struct DirectRunProcessRestoreSelectedExecutionRemainderV1 {
    program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    output_settlement_authority:
        crate::SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    invocation_fingerprint: String,
}

pub(super) struct DirectRunProcessRestoreReadyCommitCustodyV1 {
    program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    settlement: crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    subject: swarm_provider_value_model::CurrentProcessSubjectForProcessReplanAuthorityOwnerV1,
    invocation_fingerprint: String,
}

pub(crate) struct DirectRunProcessRestoreExecutionRefusalV1 {
    cause: DirectRunProcessRestoreExecutionRefusalCauseV1,
}

pub(super) enum DirectRunProcessRestoreExecutionRefusalCauseV1 {
    InputAdmission {
        refusal: ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1,
    },
    CheckpointUnmatched {
        checkpoint: swarm_provider_value_model::ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
        remainder: DirectRunProcessRestoreSelectedExecutionRemainderV1,
    },
    CheckpointRegistryBorrowed {
        checkpoint: swarm_provider_value_model::ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
        remainder: DirectRunProcessRestoreSelectedExecutionRemainderV1,
    },
    CheckpointRegistryUnavailable {
        checkpoint: swarm_provider_value_model::ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
        remainder: DirectRunProcessRestoreSelectedExecutionRemainderV1,
    },
    ProgramOpenPlanUnmatched {
        lifecycle: MatchedRegisteredProcessLifecycle,
        remainder: DirectRunProcessRestoreSelectedExecutionRemainderV1,
    },
    OutputPreflight {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
        subject: swarm_provider_value_model::CurrentProcessSubjectForProcessReplanAuthorityOwnerV1,
        invocation_fingerprint: String,
        refusal: crate::KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    },
    LifecycleRegistryBorrowed {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        custody: DirectRunProcessRestoreReadyCommitCustodyV1,
    },
    LifecycleRegistryUnavailable {
        authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
        activation_process_carrier:
            swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
        custody: DirectRunProcessRestoreReadyCommitCustodyV1,
    },
}

impl DirectRunProcessRestoreExecutionRefusalV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        match self.cause {
            DirectRunProcessRestoreExecutionRefusalCauseV1::InputAdmission { refusal } => {
                let (selected_input, fault) =
                    refusal.into_selected_input_and_fault_for_direct_run_owner_v1();
                let _retained_selected_input = selected_input;
                format!("process.restore input admission refused: {fault:?}")
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointUnmatched {
                checkpoint,
                remainder,
            } => {
                let _retained = (checkpoint, remainder);
                "process.restore Checkpoint did not join its owner".to_owned()
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointRegistryBorrowed {
                checkpoint,
                remainder,
            } => {
                let _retained = (checkpoint, remainder);
                "process.restore checkpoint registry is borrowed".to_owned()
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointRegistryUnavailable {
                checkpoint,
                remainder,
            } => {
                let _retained = (checkpoint, remainder);
                "process.restore checkpoint registry is unavailable".to_owned()
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::ProgramOpenPlanUnmatched {
                lifecycle,
                remainder,
            } => {
                let _retained = (lifecycle, remainder);
                "process.restore Program authority did not join the checkpointed open plan"
                    .to_owned()
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::OutputPreflight {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                program_authority,
                subject,
                invocation_fingerprint,
                refusal,
            } => {
                let fault =
                    cancel_kernel_internal_plain_output_preflight_refusal_for_direct_run_boundary_owner_v1(
                        refusal,
                    );
                let _retained_restore_authority = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                    program_authority,
                    subject,
                    invocation_fingerprint,
                );
                format!("process.restore output settlement preflight was refused: {fault}")
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::LifecycleRegistryBorrowed {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                custody,
            } => {
                let DirectRunProcessRestoreReadyCommitCustodyV1 {
                    program_authority,
                    settlement,
                    subject,
                    invocation_fingerprint,
                } = custody;
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_restore_authority = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                    program_authority,
                    subject,
                    invocation_fingerprint,
                );
                "process.restore lifecycle registry is borrowed".to_owned()
            }
            DirectRunProcessRestoreExecutionRefusalCauseV1::LifecycleRegistryUnavailable {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                custody,
            } => {
                let DirectRunProcessRestoreReadyCommitCustodyV1 {
                    program_authority,
                    settlement,
                    subject,
                    invocation_fingerprint,
                } = custody;
                settlement.cancel_for_direct_run_owner_v1();
                let _retained_restore_authority = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                    program_authority,
                    subject,
                    invocation_fingerprint,
                );
                "process.restore lifecycle registry is unavailable".to_owned()
            }
        }
    }
}

pub(super) fn execute_kernel_internal_process_restore_with_static_child_context_for_provider_resume_owner_v1(
    selected_input: SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
) -> Result<ProviderDriveResult, DirectRunProcessRestoreExecutionRefusalV1> {
    let execution_input = selected_input
        .consume_into_process_restore_execution_input_for_direct_run_owner_v1()
        .map_err(|refusal| DirectRunProcessRestoreExecutionRefusalV1 {
            cause: DirectRunProcessRestoreExecutionRefusalCauseV1::InputAdmission { refusal },
        })?;
    let (checkpoint, program_authority, output_settlement_authority, invocation_fingerprint) =
        execution_input.consume_for_process_restore_owner_v1();
    let remainder = DirectRunProcessRestoreSelectedExecutionRemainderV1 {
        program_authority,
        output_settlement_authority,
        invocation_fingerprint,
    };
    let matched = match select_registered_process_checkpoint(checkpoint) {
        ProcessCheckpointRegistrySelection::Joined(matched) => matched,
        ProcessCheckpointRegistrySelection::Unmatched { checkpoint } => {
            return Err(DirectRunProcessRestoreExecutionRefusalV1 {
                cause: DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointUnmatched {
                    checkpoint,
                    remainder,
                },
            });
        }
        ProcessCheckpointRegistrySelection::RegistryBorrowed { checkpoint } => {
            return Err(DirectRunProcessRestoreExecutionRefusalV1 {
                cause: DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointRegistryBorrowed {
                    checkpoint,
                    remainder,
                },
            });
        }
        ProcessCheckpointRegistrySelection::RegistryUnavailable { checkpoint } => {
            return Err(DirectRunProcessRestoreExecutionRefusalV1 {
                cause:
                    DirectRunProcessRestoreExecutionRefusalCauseV1::CheckpointRegistryUnavailable {
                        checkpoint,
                        remainder,
                    },
            });
        }
    };
    let DirectRunProcessRestoreSelectedExecutionRemainderV1 {
        program_authority,
        output_settlement_authority,
        invocation_fingerprint,
    } = remainder;
    let MatchedRegisteredProcessLifecycle {
        authority,
        subject,
        activation_process_carrier,
        loaded_process,
        open_plan,
        checkpoint_state,
    } = matched.lifecycle;
    let (program_authority, open_plan) = match program_authority
        .try_join_loaded_process_open_plan_for_direct_run_process_restore_owner_v1(open_plan)
    {
        ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1::Joined {
            program_authority,
            open_plan,
        } => (program_authority, open_plan),
        ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1::Unmatched {
            program_authority,
            open_plan,
        } => {
            return Err(DirectRunProcessRestoreExecutionRefusalV1 {
                cause: DirectRunProcessRestoreExecutionRefusalCauseV1::ProgramOpenPlanUnmatched {
                    lifecycle: MatchedRegisteredProcessLifecycle {
                        authority,
                        subject,
                        activation_process_carrier,
                        loaded_process,
                        open_plan,
                        checkpoint_state,
                    },
                    remainder: DirectRunProcessRestoreSelectedExecutionRemainderV1 {
                        program_authority,
                        output_settlement_authority,
                        invocation_fingerprint,
                    },
                },
            });
        }
    };
    let carrier = activation_process_carrier.duplicate_for_session_runtime_owner_v1();
    let settlement = match output_settlement_authority
        .preflight_kernel_internal_plain_output_settlement_for_direct_run_owner_v1(
            ProviderValue::CurrentProcess(carrier),
        ) {
        Ok(settlement) => settlement,
        Err(refusal) => {
            return Err(DirectRunProcessRestoreExecutionRefusalV1 {
                cause: DirectRunProcessRestoreExecutionRefusalCauseV1::OutputPreflight {
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                    program_authority,
                    subject,
                    invocation_fingerprint,
                    refusal,
                },
            });
        }
    };
    let custody = DirectRunProcessRestoreReadyCommitCustodyV1 {
        program_authority,
        settlement,
        subject,
        invocation_fingerprint,
    };
    match register_process_lifecycle(
        authority,
        activation_process_carrier,
        loaded_process,
        open_plan,
        checkpoint_state,
    ) {
        ProcessLifecycleRegistrationAdmission::Registered => {
            let DirectRunProcessRestoreReadyCommitCustodyV1 {
                program_authority: _program_authority,
                settlement,
                subject: _subject,
                invocation_fingerprint: _invocation_fingerprint,
            } = custody;
            Ok(ProviderDriveResult::ready_from_preflighted_kernel_internal_plain_output_for_provider_drive_result_owner_v1(
                settlement,
            ))
        }
        ProcessLifecycleRegistrationAdmission::RegistryBorrowed {
            authority,
            activation_process_carrier,
            loaded_process,
            open_plan,
            checkpoint_state,
        } => Err(DirectRunProcessRestoreExecutionRefusalV1 {
            cause: DirectRunProcessRestoreExecutionRefusalCauseV1::LifecycleRegistryBorrowed {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                custody,
            },
        }),
        ProcessLifecycleRegistrationAdmission::RegistryUnavailable {
            authority,
            activation_process_carrier,
            loaded_process,
            open_plan,
            checkpoint_state,
        } => Err(DirectRunProcessRestoreExecutionRefusalV1 {
            cause: DirectRunProcessRestoreExecutionRefusalCauseV1::LifecycleRegistryUnavailable {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
                custody,
            },
        }),
    }
}

pub(super) const PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT: usize = 8;

pub(super) fn retain_process_activate_open_refusal_for_process_lifecycle_owner_v1(
    matched: super::super::process_kernel_boundary::MatchedRegisteredProcessActivateOpenRefusalV1,
) -> Result<(), String> {
    if let Err(retained) = register_process_activate_open_refusal(
        matched.authority,
        matched.activation_process_carrier,
        matched.refusal,
        matched.checkpoint_state,
    ) {
        let recovery =
            super::super::process_kernel_boundary::DirectRunProcessActivateLifecycleRecoveryV1 {
                authority: retained.authority,
                activation_process_carrier: retained.activation_process_carrier,
                checkpoint_state: retained.checkpoint_state,
            };
        let cancellation = retained.refusal.cancel_for_process_lifecycle_owner_v1();
        return Err(recovery
            .settle_with_transitioned_open_cancellation_for_process_lifecycle_owner_v1(cancellation)
            .to_string());
    }
    Ok(())
}

pub(super) fn register_process_invoke_execution_after_ingress_commit_for_process_kernel_owner_v1(
    registration: crate::ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    execution: crate::direct_run::DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority,
    drive_context: DirectRunProcessKernelChildDriveContext,
) -> Result<(), String> {
    let mut pending = Some((registration, execution, drive_context));
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        let (registration, execution, drive_context) = pending
            .take()
            .expect("process.invoke registration retry retains complete authority cargo");
        match register_process_invoke_execution(registration, execution, drive_context) {
            ProcessInvokeExecutionRegistrationAdmission::Registered => return Ok(()),
            ProcessInvokeExecutionRegistrationAdmission::RegistryBorrowed {
                registration,
                execution,
                drive_context,
            } => {
                pending = Some((registration, execution, drive_context));
            }
            ProcessInvokeExecutionRegistrationAdmission::RegistryUnavailable {
                registration,
                execution,
                drive_context,
            } => {
                let _settled_authority_cargo = (registration, execution, drive_context);
                return Err(json!({
                    "kind": "process_invoke_execution_registry_unavailable",
                    "reason": "the process.invoke execution registry became unavailable after nominal provider ingress committed; the retained registration authority was settled without reconstructing it",
                })
                .to_string());
            }
        }
    }
    let _settled_authority_cargo = pending
        .take()
        .expect("process.invoke exhausted borrow retries retain complete authority cargo");
    Err(json!({
        "kind": "process_invoke_execution_registry_borrow_retry_exhausted",
        "reason": "the process.invoke execution registry remained borrowed after bounded retries; the retained registration authority was settled without reconstructing it",
        "retry_limit": PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT,
    })
    .to_string())
}

pub(super) fn register_process_run_child_after_ingress_commit_for_process_kernel_owner_v1(
    registration: crate::ProcessRunChildRegistrationForDirectRunOwnerV1,
    execution: crate::direct_run::DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority,
    drive_context: DirectRunProcessKernelChildDriveContext,
) -> Result<(), String> {
    let mut pending = Some((registration, execution, drive_context));
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        let (registration, execution, drive_context) = pending
            .take()
            .expect("process.run registration retry retains complete authority cargo");
        match register_process_run_child(registration, execution, drive_context) {
            ProcessRunChildRegistrationAdmission::Registered => return Ok(()),
            ProcessRunChildRegistrationAdmission::RegistryBorrowed {
                registration,
                execution,
                drive_context,
            } => {
                pending = Some((registration, execution, drive_context));
            }
            ProcessRunChildRegistrationAdmission::RegistryUnavailable {
                registration,
                execution,
                drive_context,
            } => {
                let _settled_authority_cargo = (registration, execution, drive_context);
                return Err(json!({
                    "kind": "process_run_child_registry_unavailable",
                    "reason": "the process.run child registry became unavailable after nominal provider ingress committed; the retained registration authority was settled without reconstructing it",
                })
                .to_string());
            }
        }
    }
    let _settled_authority_cargo = pending
        .take()
        .expect("process.run exhausted borrow retries retain complete authority cargo");
    Err(json!({
        "kind": "process_run_child_registry_borrow_retry_exhausted",
        "reason": "the process.run child registry remained borrowed after bounded retries; the retained registration authority was settled without reconstructing it",
        "retry_limit": PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT,
    })
    .to_string())
}

pub(super) enum DirectRunProcessInvokeExecutionSelectionFaultV1 {
    Unmatched {
        boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    },
    RegistryUnavailable {
        boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    },
    RegistryBorrowRetryExhausted {
        boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
    },
}

pub(super) enum DirectRunProcessRunChildSelectionFaultV1 {
    Unmatched {
        boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    },
    RegistryUnavailable {
        boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    },
    RegistryBorrowRetryExhausted {
        boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
    },
}

pub(super) fn select_process_invoke_execution_for_process_kernel_owner_v1(
    boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
) -> Result<MatchedRegisteredProcessInvokeExecution, DirectRunProcessInvokeExecutionSelectionFaultV1>
{
    let mut boundary = Some(boundary);
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        match select_registered_process_invoke_execution(
            boundary
                .take()
                .expect("process.invoke registry selection retry retains its boundary"),
        ) {
            ProcessInvokeExecutionRegistrySelection::Joined(matched) => return Ok(matched),
            ProcessInvokeExecutionRegistrySelection::RegistryBorrowed { boundary: retained } => {
                boundary = Some(retained)
            }
            ProcessInvokeExecutionRegistrySelection::Unmatched { boundary: retained } => {
                return Err(DirectRunProcessInvokeExecutionSelectionFaultV1::Unmatched {
                    boundary: retained,
                });
            }
            ProcessInvokeExecutionRegistrySelection::RegistryUnavailable { boundary: retained } => {
                return Err(
                    DirectRunProcessInvokeExecutionSelectionFaultV1::RegistryUnavailable {
                        boundary: retained,
                    },
                );
            }
        }
    }
    Err(
        DirectRunProcessInvokeExecutionSelectionFaultV1::RegistryBorrowRetryExhausted {
            boundary: boundary.take().expect(
                "process.invoke selection exhausted borrow retries with its boundary intact",
            ),
        },
    )
}

pub(super) fn select_process_run_child_for_process_kernel_owner_v1(
    boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
) -> Result<MatchedRegisteredProcessRunChild, DirectRunProcessRunChildSelectionFaultV1> {
    let mut boundary = Some(boundary);
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        match select_registered_process_run_child(
            boundary
                .take()
                .expect("process.run registry selection retry retains its boundary"),
        ) {
            ProcessRunChildRegistrySelection::Joined(matched) => return Ok(matched),
            ProcessRunChildRegistrySelection::RegistryBorrowed { boundary: retained } => {
                boundary = Some(retained)
            }
            ProcessRunChildRegistrySelection::Unmatched { boundary: retained } => {
                return Err(DirectRunProcessRunChildSelectionFaultV1::Unmatched {
                    boundary: retained,
                });
            }
            ProcessRunChildRegistrySelection::RegistryUnavailable { boundary: retained } => {
                return Err(
                    DirectRunProcessRunChildSelectionFaultV1::RegistryUnavailable {
                        boundary: retained,
                    },
                );
            }
        }
    }
    Err(
        DirectRunProcessRunChildSelectionFaultV1::RegistryBorrowRetryExhausted {
            boundary: boundary
                .take()
                .expect("process.run selection exhausted borrow retries with its boundary intact"),
        },
    )
}

pub(super) fn select_process_lifecycle_for_process_kernel_owner_v1(
    process: swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
) -> Result<MatchedRegisteredProcessLifecycle, String> {
    let mut process = Some(process);
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        match select_registered_process_lifecycle(
            process
                .take()
                .expect("process lifecycle selection retry retains its exact carrier"),
        ) {
            ProcessLifecycleRegistrySelection::Joined(matched) => return Ok(matched),
            ProcessLifecycleRegistrySelection::OpenRefused(matched) => {
                retain_process_activate_open_refusal_for_process_lifecycle_owner_v1(matched)?;
                return Err(json!({
                    "kind": "process_control_activation_open_refused",
                    "reason": "process control cannot consume lifecycle custody while exact activation open remains retryable",
                })
                .to_string());
            }
            ProcessLifecycleRegistrySelection::RegistryBorrowed { process: retained } => {
                process = Some(retained);
            }
            ProcessLifecycleRegistrySelection::Unmatched { process: retained } => {
                let _retained_exact_process = retained;
                return Err(json!({
                    "kind": "process_control_lifecycle_unmatched",
                    "reason": "the selected process-control carrier did not join any exact registered process lifecycle",
                })
                .to_string());
            }
            ProcessLifecycleRegistrySelection::RegistryUnavailable { process: retained } => {
                let _retained_exact_process = retained;
                return Err(json!({
                    "kind": "process_control_lifecycle_registry_unavailable",
                    "reason": "the exact process lifecycle registry was unavailable during process control",
                })
                .to_string());
            }
        }
    }
    let _retained_exact_process = process
        .take()
        .expect("process lifecycle selection exhausted retries with its carrier intact");
    Err(json!({
        "kind": "process_control_lifecycle_registry_borrow_retry_exhausted",
        "reason": "the exact process lifecycle registry remained borrowed during process control",
        "retry_limit": PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT,
    })
    .to_string())
}

pub(super) fn register_process_lifecycle_after_control_for_process_kernel_owner_v1(
    authority: swarm_provider_value_model::CurrentProcessAuthorityForSessionRuntimeOwnerV1,
    activation_process_carrier:
        swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
    loaded_process: crate::direct_run::DirectSwarmScriptRunCurrentProcess,
    open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    checkpoint_state: crate::ProcessLivenessCheckpointStateV1,
) -> Result<(), String> {
    let mut retained = Some((
        authority,
        activation_process_carrier,
        loaded_process,
        open_plan,
        checkpoint_state,
    ));
    for _ in 0..PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT {
        let (authority, activation_process_carrier, loaded_process, open_plan, checkpoint_state) =
            retained
                .take()
                .expect("process lifecycle re-registration retains complete authority cargo");
        match register_process_lifecycle(
            authority,
            activation_process_carrier,
            loaded_process,
            open_plan,
            checkpoint_state,
        ) {
            ProcessLifecycleRegistrationAdmission::Registered => return Ok(()),
            ProcessLifecycleRegistrationAdmission::RegistryBorrowed {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
            } => {
                retained = Some((
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                ))
            }
            ProcessLifecycleRegistrationAdmission::RegistryUnavailable {
                authority,
                activation_process_carrier,
                loaded_process,
                open_plan,
                checkpoint_state,
            } => {
                let _retained_exact_lifecycle = (
                    authority,
                    activation_process_carrier,
                    loaded_process,
                    open_plan,
                    checkpoint_state,
                );
                return Err(json!({
                    "kind": "process_control_lifecycle_reregistration_unavailable",
                    "reason": "the exact transitioned process lifecycle could not be returned to its owner registry",
                })
                .to_string());
            }
        }
    }
    let _retained_exact_lifecycle = retained
        .take()
        .expect("process lifecycle re-registration exhausted retries with authority intact");
    Err(json!({
        "kind": "process_control_lifecycle_reregistration_borrow_retry_exhausted",
        "reason": "the exact transitioned process lifecycle registry remained borrowed",
        "retry_limit": PROCESS_KERNEL_REGISTRY_BORROW_RETRY_LIMIT,
    })
    .to_string())
}
