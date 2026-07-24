use std::fmt;

use super::super::descriptors::{
    ActorTurnDescriptor, InstructionCursor, ProjectionCursor, ProjectionKind,
};
use super::super::effect_ledger::{
    EffectHandle, EffectRef, PendingActivityEffectFrame,
    SelectedProviderResumeInvocationAuthorityForSessionWorkRuntimeOwnerV1,
};
use super::super::ids::RuntimeHandleGeneration;
use super::super::ids::WorkId;
use super::super::payload_store::{
    ActorRequestReadyErrPayloadProduct, ActorRequestReadyOkPayloadProduct, PayloadHandle,
};
use super::super::prepared_store::{PreparedRuntimeHandle, SegmentedLiveSessionHandle};
use crate::session::execution_kernel::executable_image::CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1;
use crate::session::execution_kernel::executable_value::{
    SessionRuntimeMaterializedSwarmEventPublishPayloadProduct,
    SessionRuntimeSwarmEventPublishPayloadForDurableExecutionOwnerV1,
};
use serde::{Serialize, Serializer};
use swarm_capability_linker_core::ProviderValue;
use swarm_capability_model::SelectedProviderBoundaryOutputAuthority;
use swarm_rust_sdk_static_provider_host::SelectedProviderBoundaryHostRequest;
use swarmvm_isa_types::authority_ids as swarmvm_isa;
use swarmvm_isa_types::authority_ids::ActorRequestId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkKind {
    SessionStart,
    SchedulerReawaken,
    SchedulerRunnable,
    InstructionContinuation,
    ActorTurn,
    ActorCheckpointBodyWork,
    ActorRequestReadyOkResult,
    ActorRequestReadyErrResult,
    EventWaitProducer,
    ProviderResume,
    EventAppend,
    Projection,
    ExternalIngress,
    TimerWake,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkHandle {
    pub(crate) id: WorkId,
    pub(crate) kind: WorkKind,
    pub(crate) generation: RuntimeHandleGeneration,
}

impl fmt::Debug for WorkHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkHandle")
            .field("authority", &"<session-work-runtime-owned>")
            .finish()
    }
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActorRequestReadyOkWorkHandle {
    handle: WorkHandle,
}

impl fmt::Debug for ActorRequestReadyOkWorkHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ActorRequestReadyOkWorkHandle")
            .field("authority", &"<session-work-runtime-owned>")
            .finish()
    }
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActorRequestReadyErrWorkHandle {
    handle: WorkHandle,
}

impl fmt::Debug for ActorRequestReadyErrWorkHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ActorRequestReadyErrWorkHandle")
            .field("authority", &"<session-work-runtime-owned>")
            .finish()
    }
}

pub(crate) enum StoreOwnedWorkSettlementFault {
    StoreValidationRefusal(String),
    ActorRequestReadyOkStoreOwnedSettlementRequired {
        payload_handle: PayloadHandle,
        payload_body: ActorRequestReadyOkPayloadProduct,
    },
    ActorRequestReadyErrStoreOwnedSettlementRequired {
        payload_handle: PayloadHandle,
        payload_body: ActorRequestReadyErrPayloadProduct,
    },
    ProviderResumeApplicationStoreOwnedSettlementRequired {
        pending_frame: PendingActivityEffectFrame,
        resume_work_handle: WorkHandle,
    },
}

impl From<String> for StoreOwnedWorkSettlementFault {
    fn from(error: String) -> Self {
        Self::StoreValidationRefusal(error)
    }
}

impl fmt::Debug for StoreOwnedWorkSettlementFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StoreValidationRefusal(_) => formatter
                .debug_struct("StoreValidationRefusal")
                .field("authority", &"session_work_runtime_owned")
                .finish(),
            Self::ActorRequestReadyOkStoreOwnedSettlementRequired { .. } => formatter
                .debug_struct("ActorRequestReadyOkStoreOwnedSettlementRequired")
                .field("payload_handle", &"session_work_runtime_owned")
                .field("payload_body", &"actor_request_ready_ok")
                .finish(),
            Self::ActorRequestReadyErrStoreOwnedSettlementRequired { .. } => formatter
                .debug_struct("ActorRequestReadyErrStoreOwnedSettlementRequired")
                .field("payload_handle", &"session_work_runtime_owned")
                .field("payload_body", &"actor_request_ready_err")
                .finish(),
            Self::ProviderResumeApplicationStoreOwnedSettlementRequired { .. } => formatter
                .debug_struct("ProviderResumeApplicationStoreOwnedSettlementRequired")
                .field("pending_frame", &"session_work_runtime_owned")
                .field("resume_work_handle", &"session_work_runtime_owned")
                .finish(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkRef {
    pub(crate) id: WorkId,
    pub(crate) kind: WorkKind,
    pub(crate) generation: RuntimeHandleGeneration,
}

impl WorkHandle {
    pub(crate) fn scheduler_runnable(sequence: u64) -> Self {
        Self {
            id: WorkId::scheduler_runnable(sequence),
            kind: WorkKind::SchedulerRunnable,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn actor_turn(turn_id: &swarmvm_isa::ActorTurnId) -> Self {
        Self {
            id: WorkId::actor_turn(turn_id),
            kind: WorkKind::ActorTurn,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn actor_checkpoint_body_work(sequence: u64) -> Self {
        Self {
            id: WorkId::actor_checkpoint_body_work(sequence),
            kind: WorkKind::ActorCheckpointBodyWork,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn provider_resume(attempt_id: &swarmvm_isa::ActivityAttemptId) -> Self {
        Self {
            id: WorkId::provider_resume(attempt_id),
            kind: WorkKind::ProviderResume,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn event_append(attempt_id: &swarmvm_isa::ActivityAttemptId) -> Self {
        Self {
            id: WorkId::event_append(attempt_id),
            kind: WorkKind::EventAppend,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn actor_request_ready_ok_result(request_id: &ActorRequestId) -> Self {
        Self {
            id: WorkId::actor_request_ready_ok_result(request_id),
            kind: WorkKind::ActorRequestReadyOkResult,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn actor_request_ready_err_result(request_id: &ActorRequestId) -> Self {
        Self {
            id: WorkId::actor_request_ready_err_result(request_id),
            kind: WorkKind::ActorRequestReadyErrResult,
            generation: RuntimeHandleGeneration::initial_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn id_str(&self) -> &str {
        self.id.as_str()
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation.value()
    }

    pub(crate) fn duplicate_for_session_work_runtime_owner_v1(&self) -> Self {
        Self {
            id: self.id.clone(),
            kind: self.kind.clone(),
            generation: self
                .generation
                .duplicate_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn matches_session_work_runtime_owner_v1(&self, other: &Self) -> bool {
        self.id == other.id && self.kind == other.kind && self.generation == other.generation
    }
}

impl ActorRequestReadyOkWorkHandle {
    pub(crate) fn from_work_handle(handle: WorkHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn into_store_owned_work_handle(self) -> WorkHandle {
        self.handle
    }
}

impl ActorRequestReadyErrWorkHandle {
    pub(crate) fn from_work_handle(handle: WorkHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn into_store_owned_work_handle(self) -> WorkHandle {
        self.handle
    }
}

impl SelectedProviderResumeHostInputForDirectRunOwnerV1 {
    pub(crate) fn from_session_work_runtime_owner_v1(
        provider_input: ProviderValue,
        selected_contract: CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
        invocation_authority: SelectedProviderResumeInvocationAuthorityForSessionWorkRuntimeOwnerV1,
        exact_static_child_use: crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    ) -> Self {
        Self {
            provider_input,
            admitted_contract: selected_contract
                .into_admitted_contract_tson_for_direct_run_provider_resume_owner_v1(),
            selected_output_authority,
            invocation_authority,
            exact_static_child_use,
            _seal: selected_provider_resume_host_input_private::Seal,
        }
    }

    /// Observation-only negative prefilter. A positive result grants no route:
    /// the caller must still consume `self` through the exact catalogue identity
    /// and operation transition below. This avoids materializing the builtin
    /// catalogue for unrelated provider calls.
    pub fn contract_package_export_matches_for_direct_run_routing_observation_v1(
        &self,
        package_specifier: &str,
        export_name: &str,
    ) -> bool {
        self.admitted_contract.identity().package_specifier() == package_specifier
            && self.admitted_contract.identity().export_name() == export_name
    }

    /// Select one kernel-internal command only from an exact sealed catalogue
    /// identity and its exact Contract-TSON command operation. A different
    /// identity preserves the complete request for the ordinary provider-host
    /// route; an operation mismatch on the exact identity refuses typed.
    pub(crate) fn select_exact_kernel_internal_command_for_direct_run_provider_resume_owner_v1(
        self,
        expected_identity: &swarm_capability_model::CapabilityContractIdentity,
        expected_operation: &str,
    ) -> Result<SelectedProviderResumeRouteForDirectRunOwnerV1, String> {
        let Self {
            provider_input,
            admitted_contract,
            selected_output_authority,
            invocation_authority,
            exact_static_child_use,
            _seal: _,
        } = self;
        match admitted_contract
            .select_exact_kernel_internal_command_for_contract_tson_owner_v1(
                expected_identity,
                expected_operation,
            )
            .map_err(|fault| fault.to_string())?
        {
            swarm_capability_contract_tson::ExactKernelInternalCommandContractSelectionForContractTsonOwnerV1::Exact(
                exact_contract,
            ) => {
                let selected_input = SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
                    provider_input,
                    invocation_authority,
                    output_settlement_authority:
                        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1 {
                            exact_contract,
                            selected_output_authority,
                            _seal: selected_kernel_internal_provider_output_settlement_private::Seal,
                        },
                    _seal: selected_kernel_internal_provider_resume_input_private::Seal,
                };
                Ok(match exact_static_child_use {
                    crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Ordinary => {
                        SelectedProviderResumeRouteForDirectRunOwnerV1::KernelInternal(
                            selected_input,
                        )
                    }
                    crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Exact(
                        crate::session::execution_kernel::executable_image::PreparedBoundProviderExactStaticChildUseAuthorityV1::ProcessLoad(
                            exact_use_authority,
                        ),
                    ) => SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessLoad(
                        SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1::from_selected_exact_use_for_session_work_runtime_owner_v1(
                            selected_input,
                            exact_use_authority,
                        ),
                    ),
                    crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Exact(
                        crate::session::execution_kernel::executable_image::PreparedBoundProviderExactStaticChildUseAuthorityV1::ProcessRestore(
                            exact_use_authority,
                        ),
                    ) => SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRestore(
                        SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1::from_selected_exact_use_for_session_work_runtime_owner_v1(
                            selected_input,
                            exact_use_authority,
                        ),
                    ),
                    crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Exact(
                        crate::session::execution_kernel::executable_image::PreparedBoundProviderExactStaticChildUseAuthorityV1::ProcessRun(
                            exact_use_authority,
                        ),
                    ) => SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessRun(
                        SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1::from_selected_exact_use_for_session_work_runtime_owner_v1(
                            selected_input,
                            exact_use_authority,
                        ),
                    ),
                    crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Exact(
                        crate::session::execution_kernel::executable_image::PreparedBoundProviderExactStaticChildUseAuthorityV1::ProcessInvoke(
                            exact_use_authority,
                        ),
                    ) => SelectedProviderResumeRouteForDirectRunOwnerV1::ProcessInvoke(
                        SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1::from_selected_exact_use_for_session_work_runtime_owner_v1(
                            selected_input,
                            exact_use_authority,
                        ),
                    ),
                })
            }
            swarm_capability_contract_tson::ExactKernelInternalCommandContractSelectionForContractTsonOwnerV1::DifferentContract(
                admitted_contract,
            ) => Ok(SelectedProviderResumeRouteForDirectRunOwnerV1::ProviderHost(Self {
                provider_input,
                admitted_contract,
                selected_output_authority,
                invocation_authority,
                exact_static_child_use,
                _seal: selected_provider_resume_host_input_private::Seal,
            })),
        }
    }

    fn into_contract_and_provider_input_for_direct_run_provider_resume_owner_v1(
        self,
    ) -> (
        swarm_capability_contract_tson::AdmittedCapabilityContractTson,
        ProviderValue,
        SelectedProviderBoundaryOutputAuthority,
    ) {
        let Self {
            provider_input,
            admitted_contract,
            selected_output_authority,
            invocation_authority,
            exact_static_child_use,
            _seal: _,
        } = self;
        let crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1::Ordinary = exact_static_child_use else {
            unreachable!(
                "exact static-child ProviderResume custody must select its Load/Run/Invoke kernel route"
            )
        };
        invocation_authority.consume_for_provider_host_route_for_session_work_runtime_owner_v1();
        (admitted_contract, provider_input, selected_output_authority)
    }

    /// One-shot admission of the selected ProviderResume host input into the
    /// host-admitted typed request. Routing between the static provider host
    /// map and the kernel-owned product-session event route happens at the
    /// direct-run drive, which consumes the declared product-session boundary
    /// predicate over this admitted request.
    pub fn admit_host_typed_request_for_direct_run_provider_resume_owner_v1(
        self,
        provider_execution_session: &swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> swarm_capability_model::CapabilitySdkResult<SelectedProviderBoundaryHostRequest> {
        let (contract, provider_input, selected_output_authority) =
            self.into_contract_and_provider_input_for_direct_run_provider_resume_owner_v1();
        provider_execution_session
            .admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1(
                contract,
                provider_input,
                selected_output_authority,
            )
    }
}

impl SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
    pub(in crate::session) fn preflight_process_run_child_launch_for_session_execution_kernel_owner_v1(
        self,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRunExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<
        PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1,
        ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
    > {
        let fault = match &self.provider_input {
            ProviderValue::Array(positional) if !(1..=2).contains(&positional.len()) => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ArgumentCardinality {
                    expected_minimum: 1,
                    expected_maximum: 2,
                    actual: positional.len(),
                },
            ),
            ProviderValue::Array(_) => None,
            _ => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::PositionalArgumentsRequired,
            ),
        };
        if let Some(fault) = fault {
            return Err(
                ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                    selected_input: self,
                    cause: ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                        exact_use_authority,
                        fault,
                    },
                },
            );
        }
        let admitted_exact_use = match exact_use_authority
            .consume_with_current_exact_scope_for_session_work_runtime_owner_v1(
                current_exact_capability_scope
                    .select_for_exact_static_child_use_for_session_work_runtime_owner_v1(),
            ) {
            Ok(admitted_exact_use) => admitted_exact_use,
            Err(refusal) => {
                return Err(
                    ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                    selected_input: self,
                    cause: ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                    },
                );
            }
        };
        Ok(PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1 {
            selected_input: self,
            admitted_exact_use,
        })
    }

    pub(in crate::session) fn preflight_process_invoke_child_launch_for_session_execution_kernel_owner_v1(
        self,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessInvokeExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<
        PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1,
        ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
    > {
        let fault = match &self.provider_input {
            ProviderValue::Array(positional) if !(2..=3).contains(&positional.len()) => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ArgumentCardinality {
                    expected_minimum: 2,
                    expected_maximum: 3,
                    actual: positional.len(),
                },
            ),
            ProviderValue::Array(positional)
                if matches!(positional.get(2), Some(options) if !matches!(options, ProviderValue::Object(fields) if fields.is_empty())) =>
            {
                Some(
                    ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::InvokeOptionsContainUnsupportedFields,
                )
            }
            ProviderValue::Array(_) => None,
            _ => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::PositionalArgumentsRequired,
            ),
        };
        if let Some(fault) = fault {
            return Err(
                ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                    selected_input: self,
                    cause: ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                        exact_use_authority,
                        fault,
                    },
                },
            );
        }
        let admitted_exact_use = match exact_use_authority
            .consume_with_current_exact_scope_for_session_work_runtime_owner_v1(
                current_exact_capability_scope
                    .select_for_exact_static_child_use_for_session_work_runtime_owner_v1(),
            ) {
            Ok(admitted_exact_use) => admitted_exact_use,
            Err(refusal) => {
                return Err(
                    ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                    selected_input: self,
                    cause: ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                    },
                );
            }
        };
        Ok(
            PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1 {
                selected_input: self,
                admitted_exact_use,
            },
        )
    }

    /// Split the checked request payload from the one-shot settlement authority
    /// and consume the corresponded invocation into a one-way fingerprint. The
    /// selected output seal cannot be paired with cargo until the latter is
    /// consumed by provider-drive-result after kernel execution completes, and
    /// direct run never receives a raw EffectRef or source-site projection.
    pub fn into_provider_input_output_settlement_and_invocation_fingerprint_for_direct_run_owner_v1(
        self,
    ) -> (
        ProviderValue,
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
        String,
    ) {
        (
            self.provider_input,
            self.output_settlement_authority,
            self.invocation_authority
                .consume_into_kernel_internal_invocation_fingerprint_for_session_work_runtime_owner_v1(),
        )
    }
}

fn preflight_process_load_child_launch_for_session_execution_kernel_owner_v1(
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    exact_use_authority: crate::session::execution_kernel::executable_image::ProcessLoadExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
    selected_current_scope: crate::session::execution_kernel::executable_image::SelectedCurrentExactCapabilityScopeForExactStaticChildOwnerV1,
) -> Result<
    PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1,
    ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
> {
    let input_fault = match &selected_input.provider_input {
        ProviderValue::Array(positional) if positional.len() != 1 => Some(
            ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ArgumentCardinality {
                expected_minimum: 1,
                expected_maximum: 1,
                actual: positional.len(),
            },
        ),
        ProviderValue::Array(positional) => match positional.get(0) {
            Some(ProviderValue::Object(load))
                if load.len() == 1 && load.contains_key("program") =>
            {
                None
            }
            Some(ProviderValue::Object(_)) => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ProcessLoadProgramFieldRequired,
            ),
            _ => Some(
                ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ProcessLoadArgumentObjectRequired,
            ),
        },
        _ => Some(
            ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::PositionalArgumentsRequired,
        ),
    };
    if let Some(fault) = input_fault {
        return Err(
            ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                selected_input,
                cause: ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                    exact_use_authority,
                    selected_current_scope,
                    fault,
                },
            },
        );
    }
    let admitted_exact_use = match exact_use_authority
        .consume_with_current_exact_scope_for_session_work_runtime_owner_v1(selected_current_scope)
    {
        Ok(admitted_exact_use) => admitted_exact_use,
        Err(refusal) => {
            return Err(
                ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
                selected_input,
                cause: ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                },
            );
        }
    };
    Ok(PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1 {
        selected_input,
        admitted_exact_use,
    })
}

impl SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1 {
    pub fn preflight_kernel_internal_plain_output_settlement_for_direct_run_owner_v1(
        self,
        output: ProviderValue,
    ) -> Result<
        PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
        KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    > {
        if let Err(fault) = self
            .exact_contract
            .preflight_kernel_internal_plain_output_for_direct_run_owner_v1()
        {
            return Err(
                KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1 {
                    settlement_authority: self,
                    output,
                    fault,
                },
            );
        }
        let Self {
            exact_contract,
            selected_output_authority,
            _seal: _,
        } = self;
        Ok(
            PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1 {
                exact_contract,
                selected_output_authority,
                output,
            },
        )
    }

    pub fn admit_process_invoke_execution_output_for_direct_run_owner_v1(
        self,
        output: crate::ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
    ) -> crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1 {
        let Self {
            exact_contract,
            selected_output_authority,
            _seal: _,
        } = self;
        crate::ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1::from_exact_kernel_internal_provider_output_for_session_work_runtime_owner_v1(
            output,
            selected_output_authority,
            exact_contract
                .into_output_type_contract_authority_for_provider_drive_result_owner_v1(),
        )
    }

    pub fn admit_process_run_child_output_for_direct_run_owner_v1(
        self,
        output: crate::ProcessRunChildProviderOutputForDirectRunOwnerV1,
    ) -> crate::ProcessRunChildProviderIngressForDirectRunOwnerV1 {
        let Self {
            exact_contract,
            selected_output_authority,
            _seal: _,
        } = self;
        crate::ProcessRunChildProviderIngressForDirectRunOwnerV1::from_exact_kernel_internal_provider_output_for_session_work_runtime_owner_v1(
            output,
            selected_output_authority,
            exact_contract
                .into_output_type_contract_authority_for_provider_drive_result_owner_v1(),
        )
    }
}

impl PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1 {
    pub(crate) fn consume_into_ready_output_for_provider_drive_result_owner_v1(
        self,
    ) -> swarm_capability_model::ProviderReadyBoundaryOutput {
        let Self {
            exact_contract,
            selected_output_authority,
            output,
        } = self;
        exact_contract
            .settle_after_kernel_internal_plain_output_preflight_for_direct_run_owner_v1();
        selected_output_authority.admit_ready_output_for_provider_host_owner_v1(output)
    }

    pub(crate) fn cancel_for_direct_run_owner_v1(self) {
        let Self {
            exact_contract,
            selected_output_authority,
            output,
        } = self;
        drop((exact_contract, selected_output_authority, output));
    }
}

impl KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1 {
    pub(crate) fn retry_for_direct_run_owner_v1(
        self,
    ) -> Result<PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1, Self> {
        let Self {
            settlement_authority,
            output,
            fault: _,
        } = self;
        settlement_authority
            .preflight_kernel_internal_plain_output_settlement_for_direct_run_owner_v1(output)
    }

    pub(crate) fn cancel_into_fault_for_direct_run_owner_v1(
        self,
    ) -> swarm_capability_linker_core::CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1
    {
        let Self {
            settlement_authority,
            output,
            fault,
        } = self;
        let SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1 {
            exact_contract,
            selected_output_authority,
            _seal: _,
        } = settlement_authority;
        drop((exact_contract, selected_output_authority, output));
        fault
    }
}

impl WorkRef {
    pub(crate) fn from_handle(handle: &WorkHandle) -> Self {
        Self {
            id: handle.id.clone(),
            kind: handle.kind.clone(),
            generation: handle
                .generation
                .duplicate_for_session_work_runtime_owner_v1(),
        }
    }

    pub(crate) fn duplicate_for_session_work_runtime_owner_v1(&self) -> Self {
        Self {
            id: self.id.clone(),
            kind: self.kind.clone(),
            generation: self
                .generation
                .duplicate_for_session_work_runtime_owner_v1(),
        }
    }

    pub fn diagnostic_value(&self) -> serde_json::Value {
        serde_json::json!({
            "work_id": self.id.as_str(),
            "work_kind": self.kind,
            "work_generation": self.generation.value(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub(crate) enum WorkRetentionClass {
    LiveOnly,
    PublicContinuation,
    EffectResume,
    CheckpointRestorable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub(crate) enum WorkStatus {
    Ready,
    Running,
    WaitingOnEffect,
    WaitingOnTimer,
    WaitingOnActorDelivery,
    Yielded,
    Completed,
    Faulted,
    Retired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkAuthority {
    pub(crate) authority_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkCreationCause {
    pub(crate) cause_kind: String,
}

pub(crate) struct SessionStartWorkFrame {
    pub(crate) segmented_live_session: SegmentedLiveSessionHandle,
    pub(crate) root_input: PayloadHandle,
    pub(crate) entrypoint: InstructionCursor,
    pub(crate) root_scope_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SchedulerReawakenWorkFrame {
    pub(crate) phase_cursor: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct InstructionContinuationWorkFrame {
    pub(crate) cursor: InstructionCursor,
    pub(crate) local_stack_handle: String,
    pub(crate) dependency_frame_set_handle: String,
}

pub(crate) struct ActorTurnWorkFrame {
    pub(crate) actor_turn: PayloadHandle,
    pub(crate) actor_descriptor: ActorTurnDescriptor,
    pub(crate) handler_entry: InstructionCursor,
}

pub(crate) struct ActorCheckpointBodyWorkFrame {
    pub(crate) correlation:
        crate::privileged_hostcalls::actor_store::CheckpointActorRestoreCorrelation,
    pub(crate) payload: PayloadHandle,
}

pub(crate) struct ActorRequestReadyOkWorkFrame {
    pub(crate) request_id: ActorRequestId,
    pub(crate) result_payload: PayloadHandle,
}

pub(crate) struct ActorRequestReadyErrWorkFrame {
    pub(crate) request_id: ActorRequestId,
    pub(crate) result_payload: PayloadHandle,
}

pub(crate) struct ProviderResumeWorkFrame {
    pub(crate) effect: EffectHandle,
    pub(crate) selected_authority_custody:
        ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1,
}

pub(crate) enum ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1 {
    Pending {
        selected_contract: CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
        exact_static_child_use: crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    },
    Consumed,
}

#[must_use = "selected provider-resume host input is one-shot runnable work and must be consumed by the direct-run owner, not dropped"]
pub struct SelectedProviderResumeHostInputForDirectRunOwnerV1 {
    provider_input: ProviderValue,
    admitted_contract: swarm_capability_contract_tson::AdmittedCapabilityContractTson,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    invocation_authority: SelectedProviderResumeInvocationAuthorityForSessionWorkRuntimeOwnerV1,
    exact_static_child_use: crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    _seal: selected_provider_resume_host_input_private::Seal,
}

/// Closed execution choice for a selected direct-run ProviderResume request.
/// Exact process.run/process.invoke uses carry their role-specific sealed use;
/// ordinary kernel-internal and provider-host routes cannot receive it.
pub(crate) enum SelectedProviderResumeRouteForDirectRunOwnerV1 {
    KernelInternal(SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1),
    ProcessLoad(SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProcessRestore(SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1),
    ProcessRun(SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProcessInvoke(SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1),
    ProviderHost(SelectedProviderResumeHostInputForDirectRunOwnerV1),
}

#[must_use = "a selected kernel-internal provider input must be consumed into one corresponded ready result"]
pub struct SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
    provider_input: ProviderValue,
    invocation_authority: SelectedProviderResumeInvocationAuthorityForSessionWorkRuntimeOwnerV1,
    output_settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    _seal: selected_kernel_internal_provider_resume_input_private::Seal,
}

#[must_use = "a selected exact process.load input must join its fresh exact-use authority with current exact scope"]
pub struct SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    exact_use_authority: crate::session::execution_kernel::executable_image::ProcessLoadExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
}

#[must_use = "a selected exact process.restore input must carry its sealed Program and Checkpoint into restore execution"]
pub struct SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRestoreExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
}

#[must_use = "selected process.restore Program authority must join the loaded process open plan"]
pub struct SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1 {
    reusable_site_plan:
        std::sync::Arc<crate::direct_run::DirectRunExactStaticChildProcessRestoreReusableSitePlanV1>,
    _lexical_header_path: std::sync::Arc<
        crate::session::execution_kernel::executable_image::ExactStaticChildSiteLexicalHeaderPathForExecutableImageOwnerV1,
    >,
}

#[must_use = "selected process.restore execution input must reach checkpoint and lifecycle correspondence"]
pub(crate) struct SelectedProcessRestoreExecutionInputForDirectRunOwnerV1 {
    checkpoint: swarm_provider_value_model::ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
    program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    output_settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    invocation_fingerprint: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1 {
    PositionalArgumentsRequired,
    ArgumentCardinality { actual: usize },
    ArgumentObjectRequired,
    ExactCheckpointFieldRequired,
    CheckpointAuthorityRequired,
}

#[must_use = "a refused process.restore input retains the complete selected request"]
pub(crate) struct ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1 {
    selected_input: SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
    fault: ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1,
}

pub(crate) enum ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1 {
    Joined {
        program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    },
    Unmatched {
        program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    },
}

#[must_use = "a selected exact process.run input must be preflighted with its fresh exact-use authority"]
pub struct SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRunExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
}

#[must_use = "a selected exact process.invoke input must be preflighted with its fresh exact-use authority"]
pub struct SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    exact_use_authority: crate::session::execution_kernel::executable_image::ProcessInvokeExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
}

impl SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    pub(crate) fn from_selected_exact_use_for_session_work_runtime_owner_v1(
        selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRunExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
    ) -> Self {
        Self {
            selected_input,
            exact_use_authority,
        }
    }

    pub(in crate::session) fn preflight_for_session_execution_kernel_owner_v1(
        self,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<
        PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1,
        ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
    > {
        self.selected_input
            .preflight_process_run_child_launch_for_session_execution_kernel_owner_v1(
                self.exact_use_authority,
                current_exact_capability_scope,
            )
    }
}

impl SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    pub(crate) fn from_selected_exact_use_for_session_work_runtime_owner_v1(
        selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessLoadExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
    ) -> Self {
        Self {
            selected_input,
            exact_use_authority,
        }
    }

    pub(in crate::session) fn preflight_for_session_execution_kernel_owner_v1(
        self,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<
        PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1,
        ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
    > {
        let selected_current_scope = current_exact_capability_scope
            .select_for_exact_static_child_use_for_session_work_runtime_owner_v1();
        preflight_process_load_child_launch_for_session_execution_kernel_owner_v1(
            self.selected_input,
            self.exact_use_authority,
            selected_current_scope,
        )
    }
}

impl SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1 {
    pub(crate) fn from_selected_exact_use_for_session_work_runtime_owner_v1(
        selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRestoreExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
    ) -> Self {
        Self {
            selected_input,
            exact_use_authority,
        }
    }

    pub(crate) fn consume_into_process_restore_execution_input_for_direct_run_owner_v1(
        self,
    ) -> Result<
        SelectedProcessRestoreExecutionInputForDirectRunOwnerV1,
        ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1,
    > {
        let fault = match &self.selected_input.provider_input {
            ProviderValue::Array(positional) if positional.len() != 1 => Some(
                ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::ArgumentCardinality {
                    actual: positional.len(),
                },
            ),
            ProviderValue::Array(positional) => match positional.get(0) {
                Some(ProviderValue::Object(fields))
                    if fields.len() == 1 && fields.contains_key("checkpoint") =>
                {
                    match fields.get("checkpoint") {
                        Some(ProviderValue::ProcessCheckpoint(_)) => None,
                        _ => Some(
                            ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::CheckpointAuthorityRequired,
                        ),
                    }
                }
                Some(ProviderValue::Object(_)) => Some(
                    ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::ExactCheckpointFieldRequired,
                ),
                Some(_) => Some(
                    ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::ArgumentObjectRequired,
                ),
                None => Some(
                    ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::ArgumentCardinality {
                        actual: 0,
                    },
                ),
            },
            _ => Some(
                ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1::PositionalArgumentsRequired,
            ),
        };
        if let Some(fault) = fault {
            return Err(ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1 {
                selected_input: self,
                fault,
            });
        }
        let Self {
            selected_input:
                SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
                    provider_input,
                    invocation_authority,
                    output_settlement_authority,
                    _seal: _,
                },
            exact_use_authority:
                crate::session::execution_kernel::executable_image::ProcessRestoreExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1 {
                    plan,
                    lexical_header_path,
                },
        } = self;
        let ProviderValue::Array(mut positional) = provider_input else {
            unreachable!("process.restore positional input was preflighted")
        };
        let ProviderValue::Object(mut fields) = positional
            .pop()
            .expect("process.restore argument cardinality was preflighted")
        else {
            unreachable!("process.restore argument object was preflighted")
        };
        let ProviderValue::ProcessCheckpoint(checkpoint) = fields
            .remove("checkpoint")
            .expect("process.restore checkpoint field was preflighted")
        else {
            unreachable!("process.restore checkpoint authority was preflighted")
        };
        Ok(SelectedProcessRestoreExecutionInputForDirectRunOwnerV1 {
            checkpoint,
            program_authority: SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1 {
                reusable_site_plan: plan,
                _lexical_header_path: lexical_header_path,
            },
            output_settlement_authority,
            invocation_fingerprint: invocation_authority
                .consume_into_kernel_internal_invocation_fingerprint_for_session_work_runtime_owner_v1(),
        })
    }
}

impl ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1 {
    pub(crate) fn into_selected_input_and_fault_for_direct_run_owner_v1(
        self,
    ) -> (
        SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
        ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1,
    ) {
        (self.selected_input, self.fault)
    }
}

impl SelectedProcessRestoreExecutionInputForDirectRunOwnerV1 {
    pub(crate) fn consume_for_process_restore_owner_v1(
        self,
    ) -> (
        swarm_provider_value_model::ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
        SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
        String,
    ) {
        (
            self.checkpoint,
            self.program_authority,
            self.output_settlement_authority,
            self.invocation_fingerprint,
        )
    }
}

impl SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1 {
    pub(crate) fn try_join_loaded_process_open_plan_for_direct_run_process_restore_owner_v1(
        self,
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    ) -> ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1 {
        if self
            .reusable_site_plan
            .corresponds_to_loaded_process_open_plan_for_direct_run_process_restore_owner_v1(
                &open_plan,
            )
        {
            ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1::Joined {
                program_authority: self,
                open_plan,
            }
        } else {
            ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1::Unmatched {
                program_authority: self,
                open_plan,
            }
        }
    }
}

impl SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1 {
    pub(crate) fn from_selected_exact_use_for_session_work_runtime_owner_v1(
        selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessInvokeExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
    ) -> Self {
        Self {
            selected_input,
            exact_use_authority,
        }
    }

    pub(in crate::session) fn preflight_for_session_execution_kernel_owner_v1(
        self,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<
        PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1,
        ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1,
    > {
        self.selected_input
            .preflight_process_invoke_child_launch_for_session_execution_kernel_owner_v1(
                self.exact_use_authority,
                current_exact_capability_scope,
            )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {
    PositionalArgumentsRequired,
    ArgumentCardinality {
        expected_minimum: usize,
        expected_maximum: usize,
        actual: usize,
    },
    ProcessLoadArgumentObjectRequired,
    ProcessLoadProgramFieldRequired,
    InvokeOptionsContainUnsupportedFields,
    ExactStaticChildScope(
        crate::session::execution_kernel::executable_image::ExactStaticChildCapabilityScopeAdmissionFaultForSessionWorkRuntimeOwnerV1,
    ),
}

impl std::fmt::Display for ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "process-child launch input admission failed: {self:?}"
        )
    }
}

impl std::error::Error for ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {}

#[must_use = "a refused process.run exact launch preflight retains selected input and exact-use authority"]
pub(in crate::session) struct ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1
{
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    cause: ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1,
}

#[must_use = "a refused process.load exact launch preflight retains selected work, exact use, and selected current scope"]
pub(in crate::session) struct ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1
{
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    cause: ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1,
}

enum ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1 {
    Input {
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessLoadExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
        selected_current_scope: crate::session::execution_kernel::executable_image::SelectedCurrentExactCapabilityScopeForExactStaticChildOwnerV1,
        fault: ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    Scope(
        crate::session::ProcessLoadExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    ),
}

enum ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1 {
    Input {
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessRunExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
        fault: ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    Scope(
        crate::session::ProcessRunExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    ),
}

#[must_use = "a refused process.invoke exact launch preflight retains selected input and exact-use authority"]
pub(in crate::session) struct ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1
{
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    cause: ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1,
}

enum ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1 {
    Input {
        exact_use_authority: crate::session::execution_kernel::executable_image::ProcessInvokeExactStaticChildUseAuthorityForSessionWorkRuntimeOwnerV1,
        fault: ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    },
    Scope(
        crate::session::ProcessInvokeExactStaticChildUseAdmissionRefusalForSessionWorkRuntimeOwnerV1,
    ),
}

#[must_use = "a preflighted process.run launch must commit its fresh exact-use authority"]
pub(in crate::session) struct PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    admitted_exact_use:
        crate::session::AdmittedProcessRunExactStaticChildUseForSessionWorkRuntimeOwnerV1,
}

#[must_use = "a preflighted process.load launch must commit its exact use into loaded-process lifecycle custody"]
pub(in crate::session) struct PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    admitted_exact_use:
        crate::session::AdmittedProcessLoadExactStaticChildUseForSessionWorkRuntimeOwnerV1,
}

#[must_use = "a preflighted process.invoke launch must commit its fresh exact-use authority"]
pub(in crate::session) struct PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1 {
    selected_input: SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    admitted_exact_use:
        crate::session::AdmittedProcessInvokeExactStaticChildUseForSessionWorkRuntimeOwnerV1,
}

#[must_use = "an admitted process.run child launch must be consumed by direct-run registration"]
pub struct AdmittedProcessRunChildLaunchForDirectRunOwnerV1 {
    child_open_plan: crate::direct_run::DirectRunAdmittedProcessRunExactStaticChildOpenPlanV1,
    program_input: ProviderValue,
    options: ProviderValue,
    output_settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
}

#[must_use = "an admitted process.load child launch must be consumed by loaded-process lifecycle registration"]
pub struct AdmittedProcessLoadChildLaunchForDirectRunOwnerV1 {
    child_open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    output_settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
}

impl AdmittedProcessLoadChildLaunchForDirectRunOwnerV1 {
    pub(crate) fn commit_into_process_lifecycle_registration_for_direct_run_owner_v1(
        self,
    ) -> crate::direct_run::DirectRunAdmittedProcessLoadLifecycleRegistrationV1 {
        crate::direct_run::DirectRunAdmittedProcessLoadLifecycleRegistrationV1::from_exact_load_scope_admission_for_session_work_runtime_owner_v1(
            self.child_open_plan,
            self.output_settlement_authority,
        )
    }
}

impl PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn commit_after_complete_preflight_for_session_runtime_owner_v1(
        self,
    ) -> AdmittedProcessLoadChildLaunchForDirectRunOwnerV1 {
        let Self {
            selected_input,
            admitted_exact_use,
        } = self;
        let SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
            provider_input,
            invocation_authority,
            output_settlement_authority,
            _seal: _,
        } = selected_input;
        invocation_authority
            .consume_for_exact_static_child_route_for_session_work_runtime_owner_v1();
        let ProviderValue::Array(mut positional) = provider_input else {
            unreachable!("process.load exact launch payload shape was completely preflighted")
        };
        let Some(ProviderValue::Object(mut load)) = positional.pop() else {
            unreachable!("process.load exact launch argument was completely preflighted")
        };
        let _consumed_corresponded_program_input = load
            .remove("program")
            .expect("process.load exact launch Program field presence was completely preflighted");
        AdmittedProcessLoadChildLaunchForDirectRunOwnerV1 {
            child_open_plan: admitted_exact_use
                .consume_for_loaded_process_lifecycle_registration_owner_v1(),
            output_settlement_authority,
        }
    }
}

impl PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn commit_after_complete_preflight_for_session_runtime_owner_v1(
        self,
    ) -> AdmittedProcessRunChildLaunchForDirectRunOwnerV1 {
        let Self {
            selected_input,
            admitted_exact_use,
        } = self;
        let SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
            provider_input,
            invocation_authority,
            output_settlement_authority,
            _seal: _,
        } = selected_input;
        invocation_authority
            .consume_for_exact_static_child_route_for_session_work_runtime_owner_v1();
        let ProviderValue::Array(positional) = provider_input else {
            unreachable!("process.run exact launch payload shape was completely preflighted")
        };
        let mut positional = positional.into_iter();
        let program_input = positional
            .next()
            .expect("process.run exact launch cardinality was completely preflighted");
        let options = positional.next().unwrap_or(ProviderValue::Null);
        debug_assert!(positional.next().is_none());
        AdmittedProcessRunChildLaunchForDirectRunOwnerV1 {
            child_open_plan: admitted_exact_use
                .consume_for_direct_run_child_registration_owner_v1(),
            program_input,
            options,
            output_settlement_authority,
        }
    }
}

impl PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn commit_after_complete_preflight_for_session_runtime_owner_v1(
        self,
    ) -> AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1 {
        let Self {
            selected_input,
            admitted_exact_use,
        } = self;
        let SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
            provider_input,
            invocation_authority,
            output_settlement_authority,
            _seal: _,
        } = selected_input;
        invocation_authority
            .consume_for_exact_static_child_route_for_session_work_runtime_owner_v1();
        let ProviderValue::Array(positional) = provider_input else {
            unreachable!("process.invoke exact launch payload shape was completely preflighted")
        };
        let mut positional = positional.into_iter();
        let callable_input = positional
            .next()
            .expect("process.invoke exact launch cardinality was completely preflighted");
        let input = positional
            .next()
            .expect("process.invoke exact launch cardinality was completely preflighted");
        let options = positional
            .next()
            .unwrap_or_else(|| ProviderValue::Object(std::collections::BTreeMap::new().into()));
        debug_assert!(positional.next().is_none());
        AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1 {
            child_open_plan: admitted_exact_use
                .consume_for_direct_run_child_registration_owner_v1(),
            callable_input,
            input,
            options,
            output_settlement_authority,
        }
    }
}

impl ProcessRunExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn retry_for_session_runtime_owner_v1(
        self,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1, Self> {
        let Self {
            selected_input,
            cause,
        } = self;
        match cause {
            ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority,
                fault: _,
            } => selected_input.preflight_process_run_child_launch_for_session_execution_kernel_owner_v1(
                exact_use_authority,
                current_exact_capability_scope,
            ),
            ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => match refusal.retry_for_session_work_runtime_owner_v1() {
                Ok(admitted_exact_use) => Ok(PreflightedProcessRunChildLaunchForSessionRuntimeOwnerV1 {
                    selected_input,
                    admitted_exact_use,
                }),
                Err(refusal) => Err(Self {
                    selected_input,
                    cause: ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                }),
            },
        }
    }

    pub(in crate::session) fn cancel_for_session_runtime_owner_v1(
        self,
    ) -> ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {
        let Self {
            selected_input: _,
            cause,
        } = self;
        match cause {
            ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority: _,
                fault,
            } => fault,
            ProcessRunExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ExactStaticChildScope(
                refusal.cancel_for_session_work_runtime_owner_v1(),
            ),
        }
    }
}

impl ProcessLoadExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn retry_for_session_runtime_owner_v1(
        self,
    ) -> Result<PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1, Self> {
        let Self {
            selected_input,
            cause,
        } = self;
        match cause {
            ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority,
                selected_current_scope,
                fault: _,
            } => preflight_process_load_child_launch_for_session_execution_kernel_owner_v1(
                selected_input,
                exact_use_authority,
                selected_current_scope,
            ),
            ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => match refusal.retry_for_session_work_runtime_owner_v1() {
                Ok(admitted_exact_use) => {
                    Ok(PreflightedProcessLoadChildLaunchForSessionRuntimeOwnerV1 {
                        selected_input,
                        admitted_exact_use,
                    })
                }
                Err(refusal) => Err(Self {
                    selected_input,
                    cause: ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                }),
            },
        }
    }

    pub(in crate::session) fn cancel_for_session_runtime_owner_v1(
        self,
    ) -> ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {
        match self.cause {
            ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority: _,
                selected_current_scope: _,
                fault,
            } => fault,
            ProcessLoadExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ExactStaticChildScope(
                refusal.cancel_for_session_work_runtime_owner_v1(),
            ),
        }
    }
}

impl ProcessInvokeExactStaticChildLaunchPreflightRefusalForSessionRuntimeOwnerV1 {
    pub(in crate::session) fn retry_for_session_runtime_owner_v1(
        self,
        current_exact_capability_scope: &crate::session::CurrentExactCapabilityScopeAuthority,
    ) -> Result<PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1, Self> {
        let Self {
            selected_input,
            cause,
        } = self;
        match cause {
            ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority,
                fault: _,
            } => selected_input.preflight_process_invoke_child_launch_for_session_execution_kernel_owner_v1(
                exact_use_authority,
                current_exact_capability_scope,
            ),
            ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => match refusal.retry_for_session_work_runtime_owner_v1() {
                Ok(admitted_exact_use) => Ok(PreflightedProcessInvokeChildLaunchForSessionRuntimeOwnerV1 {
                    selected_input,
                    admitted_exact_use,
                }),
                Err(refusal) => Err(Self {
                    selected_input,
                    cause: ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(refusal),
                }),
            },
        }
    }

    pub(in crate::session) fn cancel_for_session_runtime_owner_v1(
        self,
    ) -> ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1 {
        let Self {
            selected_input: _,
            cause,
        } = self;
        match cause {
            ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Input {
                exact_use_authority: _,
                fault,
            } => fault,
            ProcessInvokeExactStaticChildLaunchPreflightRefusalCauseForSessionRuntimeOwnerV1::Scope(
                refusal,
            ) => ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1::ExactStaticChildScope(
                refusal.cancel_for_session_work_runtime_owner_v1(),
            ),
        }
    }
}

#[must_use = "an admitted process.invoke child launch must be consumed by direct-run registration"]
pub struct AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1 {
    child_open_plan: crate::direct_run::DirectRunAdmittedProcessInvokeExactStaticChildOpenPlanV1,
    callable_input: ProviderValue,
    input: ProviderValue,
    options: ProviderValue,
    output_settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
}

impl AdmittedProcessRunChildLaunchForDirectRunOwnerV1 {
    pub(crate) fn consume_for_direct_run_process_kernel_owner_v1(
        self,
    ) -> (
        crate::direct_run::DirectRunAdmittedProcessRunExactStaticChildOpenPlanV1,
        ProviderValue,
        ProviderValue,
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    ) {
        (
            self.child_open_plan,
            self.program_input,
            self.options,
            self.output_settlement_authority,
        )
    }
}

impl AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1 {
    pub(crate) fn consume_for_direct_run_process_kernel_owner_v1(
        self,
    ) -> (
        crate::direct_run::DirectRunAdmittedProcessInvokeExactStaticChildOpenPlanV1,
        ProviderValue,
        ProviderValue,
        ProviderValue,
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    ) {
        (
            self.child_open_plan,
            self.callable_input,
            self.input,
            self.options,
            self.output_settlement_authority,
        )
    }
}

#[must_use = "an exact kernel-internal output settlement must consume Contract-TSON output authority and the selected boundary seal together"]
pub struct SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1 {
    exact_contract: swarm_capability_contract_tson::AdmittedExactKernelInternalCommandContractForContractTsonOwnerV1,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    _seal: selected_kernel_internal_provider_output_settlement_private::Seal,
}

/// One exact kernel-internal output whose complete contract classification has
/// succeeded while the contract, selected boundary half, and concrete output
/// remain together. Only provider-drive-result may consume this product into a
/// ready result; route owners may retain or explicitly cancel it on refusal.
#[must_use = "a preflighted kernel-internal plain output must commit once or be explicitly cancelled"]
pub struct PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1 {
    exact_contract: swarm_capability_contract_tson::AdmittedExactKernelInternalCommandContractForContractTsonOwnerV1,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    output: ProviderValue,
}

#[must_use = "retry or explicitly cancel a refused kernel-internal plain-output settlement"]
pub struct KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1 {
    settlement_authority:
        SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    output: ProviderValue,
    fault: swarm_capability_linker_core::CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1,
}

mod selected_provider_resume_host_input_private {
    #[derive(Debug)]
    pub(super) struct Seal;
}

mod selected_kernel_internal_provider_resume_input_private {
    #[derive(Debug)]
    pub(super) struct Seal;
}

mod selected_kernel_internal_provider_output_settlement_private {
    #[derive(Debug)]
    pub(super) struct Seal;
}

impl std::fmt::Debug for SelectedProviderResumeHostInputForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = &self.provider_input;
        formatter
            .write_str("SelectedProviderResumeHostInputForDirectRunOwnerV1 { input: <sealed> }")
    }
}

impl std::fmt::Debug for SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = (&self.provider_input, &self.output_settlement_authority);
        formatter.write_str(
            "SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1 { input: <sealed> }",
        )
    }
}

impl std::fmt::Debug
    for SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            "SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1(<sealed>)",
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct EventAppendWorkFrame {
    pub(crate) effect: EffectHandle,
    pub(crate) resume_cursor: InstructionCursor,
}

#[must_use = "the event-append application completion ticket authorizes resume work and must be consumed, not dropped"]
pub struct EventAppendApplicationCompletionTicket {
    effect_ref: EffectRef,
    resume_work: WorkHandle,
}

#[must_use = "selected event-append publication work is one-shot and must be consumed by the direct-run owner, not dropped"]
pub struct SelectedEventAppendPublicationWorkForDirectRunOwnerV1 {
    completion_ticket: EventAppendApplicationCompletionTicket,
    payload: SessionRuntimeMaterializedSwarmEventPublishPayloadProduct,
    _seal: selected_event_append_publication_work_private::Seal,
}

#[must_use = "selected event-append publication append input must be consumed by the durable-execution owner, not dropped"]
pub struct SelectedEventAppendPublicationAppendInputForDurableExecutionOwnerV1 {
    completion_ticket: EventAppendApplicationCompletionTicket,
    payload: SessionRuntimeSwarmEventPublishPayloadForDurableExecutionOwnerV1,
    _seal: selected_event_append_publication_append_input_private::Seal,
}

mod selected_event_append_publication_work_private {
    #[derive(Debug)]
    pub(super) struct Seal;
}

mod selected_event_append_publication_append_input_private {
    #[derive(Debug)]
    pub(super) struct Seal;
}

impl fmt::Debug for SelectedEventAppendPublicationWorkForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = (&self.completion_ticket, &self.payload);
        formatter
            .write_str("SelectedEventAppendPublicationWorkForDirectRunOwnerV1 { work: <sealed> }")
    }
}

impl EventAppendApplicationCompletionTicket {
    pub(crate) fn from_selected_event_append_work_for_session_work_runtime_owner_v1(
        effect_ref: EffectRef,
        resume_work: WorkHandle,
    ) -> Self {
        Self {
            effect_ref,
            resume_work,
        }
    }

    pub(crate) fn into_parts_for_session_work_runtime_owner_v1(self) -> (EffectRef, WorkHandle) {
        (self.effect_ref, self.resume_work)
    }
}

impl SelectedEventAppendPublicationWorkForDirectRunOwnerV1 {
    pub(crate) fn from_session_work_runtime_owner_v1(
        completion_ticket: EventAppendApplicationCompletionTicket,
        payload: SessionRuntimeMaterializedSwarmEventPublishPayloadProduct,
    ) -> Self {
        Self {
            completion_ticket,
            payload,
            _seal: selected_event_append_publication_work_private::Seal,
        }
    }

    pub fn into_append_input_for_durable_execution_owner_v1(
        self,
    ) -> Result<SelectedEventAppendPublicationAppendInputForDurableExecutionOwnerV1, String> {
        let payload = self
            .payload
            .into_durable_execution_payload_for_event_journal_store_owner_v1()
            .map_err(|error| error.to_string())?;
        Ok(
            SelectedEventAppendPublicationAppendInputForDurableExecutionOwnerV1 {
                completion_ticket: self.completion_ticket,
                payload,
                _seal: selected_event_append_publication_append_input_private::Seal,
            },
        )
    }
}

impl SelectedEventAppendPublicationAppendInputForDurableExecutionOwnerV1 {
    pub fn into_parts_for_durable_execution_owner_v1(
        self,
    ) -> (
        EventAppendApplicationCompletionTicket,
        SessionRuntimeSwarmEventPublishPayloadForDurableExecutionOwnerV1,
    ) {
        (self.completion_ticket, self.payload)
    }
}

pub(crate) struct ProjectionWorkFrame {
    pub(crate) source: WorkHandle,
    pub(crate) projection_kind: ProjectionKind,
    pub(crate) cursor: ProjectionCursor,
}

pub(crate) struct ExternalIngressWorkFrame {
    pub(crate) ingress_payload: PayloadHandle,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TimerWakeWorkFrame {
    pub(crate) deadline_id: String,
}

pub(crate) enum WorkFrame {
    SessionStart(SessionStartWorkFrame),
    SchedulerReawaken(SchedulerReawakenWorkFrame),
    SchedulerRunnable,
    InstructionContinuation(InstructionContinuationWorkFrame),
    ActorTurn(ActorTurnWorkFrame),
    ActorCheckpointBodyWork(ActorCheckpointBodyWorkFrame),
    ActorRequestReadyOkResult(ActorRequestReadyOkWorkFrame),
    ActorRequestReadyErrResult(ActorRequestReadyErrWorkFrame),
    EventWaitProducer(InstructionContinuationWorkFrame),
    ProviderResume(ProviderResumeWorkFrame),
    EventAppend(EventAppendWorkFrame),
    Projection(ProjectionWorkFrame),
    ExternalIngress(ExternalIngressWorkFrame),
    TimerWake(TimerWakeWorkFrame),
}

pub(crate) struct WorkRecord {
    pub(crate) handle: WorkHandle,
    pub(crate) prepared: Option<PreparedRuntimeHandle>,
    pub(crate) authority: WorkAuthority,
    pub(crate) frame: WorkFrame,
    pub(crate) retention: WorkRetentionClass,
    pub(crate) status: WorkStatus,
    pub(crate) created_by: WorkCreationCause,
}
