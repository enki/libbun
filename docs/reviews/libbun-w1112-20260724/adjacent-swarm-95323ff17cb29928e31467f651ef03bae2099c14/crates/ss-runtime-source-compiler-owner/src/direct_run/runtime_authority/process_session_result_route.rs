use serde_json::json;
use swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary;

use super::super::{
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    DirectRunProcessSessionTerminalPublicOutputProductV1,
};
use super::process_liveness_boundary_owner::{
    DirectRunProcessLivenessBoundaryAdmissionV1, DirectRunProcessLivenessBoundaryOwnerOutcome,
};
use super::process_session_owner_execution_substrate::{
    DirectRunProcessSessionOwnerExecutionSubstrate, DirectRunProcessSessionOwnerExecutionToken,
};
use super::typed_continuation::{
    DirectRunProcessSessionReawakenContinuationToken, DirectRunProcessSessionResultRouteAuthority,
    DirectRunProcessSessionStartContinuationToken,
};
use super::{DirectRunKernelStateRefOwnerKind, DirectRunPublicApertureKernelStateRef};
use crate::direct_run::direct_process_session_result_projection_kind;
use crate::direct_run::direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner;
use crate::direct_run::{
    DirectProcessSessionResultProjection, DirectSwarmScriptRunHostInteraction,
};

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn continue_after_process_session_start_result_with_typed_authority(
    process_session_start_token: DirectRunProcessSessionStartContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    projection: Box<DirectProcessSessionResultProjection>,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    continue_after_process_session_result_with_typed_route_authority(
        DirectRunProcessSessionResultRouteAuthority::Start(process_session_start_token),
        execution_substrate,
        Vec::new(),
        projection,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn continue_after_process_session_reawaken_result_with_typed_authority(
    process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
    execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    projection: Box<DirectProcessSessionResultProjection>,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    continue_after_process_session_result_with_typed_route_authority(
        DirectRunProcessSessionResultRouteAuthority::Reawaken(process_session_reawaken_token),
        execution_substrate,
        Vec::new(),
        projection,
    )
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) fn continue_after_process_session_result_with_typed_route_authority(
    route_authority: DirectRunProcessSessionResultRouteAuthority,
    mut execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    host_interactions: Vec<DirectSwarmScriptRunHostInteraction>,
    projection: Box<DirectProcessSessionResultProjection>,
) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
    let projection_kind = direct_process_session_result_projection_kind(&projection);
    if !host_interactions.is_empty() {
        let error = json!({
            "kind": "process_session_result_host_interactions_require_typed_host_activity_product",
            "reason": "converted process-session result routing may not carry host-interaction projection cargo through the generic result route; host activity effects need the typed host-activity descriptor/product owner",
            "route_owner": route_authority.owner_label(),
            "typed_route_authority": route_authority.diagnostic_value(),
            "projection_kind": projection_kind,
            "host_interaction_count": host_interactions.len(),
        })
        .to_string();
        cancel_process_liveness_wait_after_route_refusal(projection);
        return Err(error);
    }
    let owner_token = match execution_substrate
        .consume_owner_token_for_result_route("process_session_result.route")
    {
        Ok(owner_token) => owner_token,
        Err(error) => {
            cancel_process_liveness_wait_after_route_refusal(projection);
            return Err(error);
        }
    };
    if let Err(error) = require_process_session_result_route_authority_matches_owner_token(
        &route_authority,
        owner_token,
        projection_kind,
    ) {
        cancel_process_liveness_wait_after_route_refusal(projection);
        return Err(error);
    }
    match *projection {
        DirectProcessSessionResultProjection::Terminal(projection) => {
            let terminal_finalization = projection
                .into_terminal_finalization_product_for_direct_run_process_session_terminal_finalization_owner_v1(
                    "process_session_result_route.terminal_finalization",
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
        DirectProcessSessionResultProjection::WaitingOnLiveness(mut projection) => {
            let liveness_wait = projection.liveness_wait_boundary_product.take().ok_or_else(|| {
                json!({
                    "kind": "process_session_liveness_projection_missing_boundary_product",
                    "reason": "converted process-session liveness wait must carry the process-liveness boundary owner product before route resumption",
                    "route_owner": route_authority.owner_label(),
                    "typed_route_authority": route_authority.diagnostic_value(),
                    "projection_kind": projection_kind,
                })
                .to_string()
            })?;
            let owner_outcome =
                DirectRunProcessLivenessBoundaryAdmissionV1::from_typed_process_session_boundary(
                    route_authority,
                    execution_substrate,
                    projection,
                    liveness_wait,
                )
                .admit();
            match owner_outcome {
                DirectRunProcessLivenessBoundaryOwnerOutcome::ProcessLivenessDrain(storage) => {
                    let kernel_state_ref =
                        DirectRunRuntimeAuthorityOwner::admit_process_liveness_drain_kernel_state_seal(
                            "seal",
                            "process_liveness_drain_public_aperture_route",
                            Some("process_liveness_drain_route_owner_storage"),
                            storage,
                            "process_liveness_drain_public_aperture_route",
                        )
                        .map_err(|refusal| refusal.cancel_into_final_diagnostic())?;
                    Ok(DirectRunProcessSessionPublicApertureRouteOutput::NextStep(
                        DirectRunProcessSessionPublicApertureNextStepOutputProductV1::from_process_liveness_drain_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
                            kernel_state_ref,
                        ),
                    ))
                }
            }
        }
        DirectProcessSessionResultProjection::HostBoundaryIngress(projection_poison) => {
            let _ = (route_authority, execution_substrate, projection_kind);
            match projection_poison {}
        }
        DirectProcessSessionResultProjection::ForbiddenBoundary(_) => Err(json!({
            "kind": "process_session_result_projection_route_requires_specific_owner_product",
            "reason": "converted process-session result routing cannot resume forbidden-boundary projections through generic transport; the owning boundary must mint a typed continuation product",
            "route_owner": route_authority.owner_label(),
            "typed_route_authority": route_authority.diagnostic_value(),
            "projection_kind": projection_kind,
        })
        .to_string()),
    }
}

fn cancel_process_liveness_wait_after_route_refusal(
    projection: Box<DirectProcessSessionResultProjection>,
) {
    let DirectProcessSessionResultProjection::WaitingOnLiveness(mut projection) = *projection
    else {
        return;
    };
    let Some(liveness_wait) = projection.liveness_wait_boundary_product.take() else {
        return;
    };
    let transition = liveness_wait.into_store();
    let _cancellation_receipt = transition
        .wait_store_receipt
        .cancel_for_process_liveness_wait_store_owner_v1();
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) enum DirectRunProcessSessionPublicApertureRouteOutput
{
    NextStep(DirectRunProcessSessionPublicApertureNextStepOutputProductV1),
    HostResourceFinalization(DirectRunHostResourceFinalizationNextStepV1),
    ProcessInvokeAwaitExecution(DirectRunProcessInvokeAwaitExecutionNextStepV1),
    ProcessRunDriveTerminal(DirectRunProcessRunDriveTerminalNextStepV1),
    ProcessControl(DirectRunProcessControlNextStepV1),
    TerminalPublicOutput(DirectRunProcessSessionTerminalPublicOutputProductV1),
}

pub(in crate::direct_run::direct_run_runtime_authority_refs) enum DirectRunProcessKernelBoundaryParentRouteV1
{
    Start {
        process_session_start_token: DirectRunProcessSessionStartContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    },
    Reawaken {
        process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    },
    ProviderResume {
        provider_resume_token: super::typed_continuation::DirectRunProviderResumeContinuationToken,
        provider_resume_private_storage: super::process_session_owner_execution_substrate::DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
    },
}

pub(in crate::direct_run) struct DirectRunProcessInvokeAwaitExecutionNextStepV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
}

pub(in crate::direct_run) struct DirectRunProcessRunDriveTerminalNextStepV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
}

pub(in crate::direct_run) struct DirectRunProcessControlNextStepV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessControlBoundaryForDirectRunOwnerV1,
}

macro_rules! process_kernel_boundary_next_step_constructors {
    ($type:ty, $boundary:ty) => {
        impl $type {
            pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_start_owner_v1(
                process_session_start_token: DirectRunProcessSessionStartContinuationToken,
                execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
                selected_boundary: $boundary,
            ) -> Self {
                Self {
                    parent_route: DirectRunProcessKernelBoundaryParentRouteV1::Start {
                        process_session_start_token,
                        execution_substrate,
                    },
                    selected_boundary,
                }
            }

            pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_reawaken_owner_v1(
                process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
                execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
                selected_boundary: $boundary,
            ) -> Self {
                Self {
                    parent_route: DirectRunProcessKernelBoundaryParentRouteV1::Reawaken {
                        process_session_reawaken_token,
                        execution_substrate,
                    },
                    selected_boundary,
                }
            }

            pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_provider_resume_owner_v1(
                provider_resume_token: super::typed_continuation::DirectRunProviderResumeContinuationToken,
                provider_resume_private_storage: super::process_session_owner_execution_substrate::DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
                selected_boundary: $boundary,
            ) -> Self {
                Self {
                    parent_route: DirectRunProcessKernelBoundaryParentRouteV1::ProviderResume {
                        provider_resume_token,
                        provider_resume_private_storage,
                    },
                    selected_boundary,
                }
            }
        }
    };
}

process_kernel_boundary_next_step_constructors!(
    DirectRunProcessInvokeAwaitExecutionNextStepV1,
    crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1
);

impl DirectRunProcessInvokeAwaitExecutionNextStepV1 {
    pub(in crate::direct_run) fn into_child_drive_stage_for_process_kernel_owner_v1(
        self,
    ) -> DirectRunProcessInvokeChildDriveStageV1 {
        DirectRunProcessInvokeChildDriveStageV1 {
            parent_route: self.parent_route,
            selected_boundary: self.selected_boundary,
        }
    }
}

impl DirectRunProcessRunDriveTerminalNextStepV1 {
    pub(in crate::direct_run) fn into_child_drive_stage_for_process_kernel_owner_v1(
        self,
    ) -> DirectRunProcessRunChildDriveStageV1 {
        DirectRunProcessRunChildDriveStageV1 {
            parent_route: self.parent_route,
            selected_boundary: self.selected_boundary,
        }
    }
}

impl DirectRunProcessControlNextStepV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_for_process_kernel_owner_v1(
        self,
    ) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
        super::super::process_session_public_aperture::process_child_lifecycle::drive_process_control_and_route_parent_for_process_kernel_owner_v1(
            self.parent_route,
            self.selected_boundary,
        )
    }
}

/// Closed stage that retains the exact parent route while the selected invoke
/// boundary is matched, opened, and driven.  The parent route is never
/// returned independently of the matching resume product.
pub(in crate::direct_run) struct DirectRunProcessInvokeChildDriveStageV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
}

pub(in crate::direct_run) struct DirectRunProcessRunChildDriveStageV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    selected_boundary: crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
}

pub(in crate::direct_run) struct DirectRunDrivenProcessInvokeChildProductV1 {
    resume: crate::ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    event_publication_backend_output_drain_observations:
        crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
    process_output_records:
        Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
}

pub(in crate::direct_run) struct DirectRunDrivenProcessRunChildProductV1 {
    resume: crate::ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    process_output_records:
        Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
}

pub(in crate::direct_run) struct DirectRunProcessInvokeParentResumeStageV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    driven_child: DirectRunDrivenProcessInvokeChildProductV1,
}

pub(in crate::direct_run) struct DirectRunProcessRunParentResumeStageV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    driven_child: DirectRunDrivenProcessRunChildProductV1,
}

pub(crate) struct DirectRunProcessChildStageFaultV1 {
    parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
    failure: super::super::process_session_public_aperture::process_child_lifecycle::DirectRunProcessChildDriveFailureV1,
}

impl DirectRunProcessChildStageFaultV1 {
    fn from_child_drive_for_process_kernel_owner_v1(
        parent_route: DirectRunProcessKernelBoundaryParentRouteV1,
        failure: super::super::process_session_public_aperture::process_child_lifecycle::DirectRunProcessChildDriveFailureV1,
    ) -> Self {
        Self {
            parent_route,
            failure,
        }
    }

    pub(in crate::direct_run) fn consume_into_final_diagnostic_for_direct_run_boundary_owner_v1(
        self,
    ) -> String {
        let _retained_parent_route = self.parent_route;
        self.failure
            .consume_into_final_diagnostic_for_direct_run_boundary_owner_v1()
    }
}

impl DirectRunDrivenProcessInvokeChildProductV1 {
    pub(in crate::direct_run) fn from_matching_child_drive_for_process_kernel_owner_v1(
        resume: crate::ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
        event_publication_backend_output_drain_observations: crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
    ) -> Self {
        Self {
            resume,
            event_publication_backend_output_drain_observations,
            process_output_records,
        }
    }
}

impl DirectRunDrivenProcessRunChildProductV1 {
    pub(in crate::direct_run) fn from_matching_child_drive_for_process_kernel_owner_v1(
        resume: crate::ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
    ) -> Self {
        Self {
            resume,
            process_output_records,
        }
    }
}

impl DirectRunProcessInvokeChildDriveStageV1 {
    pub(in crate::direct_run) fn drive_matching_child_for_process_kernel_owner_v1(
        self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessInvokeParentResumeStageV1,
        crate::direct_run::DirectRunProcessSessionDriveFaultV1,
    > {
        let Self {
            parent_route,
            selected_boundary,
        } = self;
        let driven_child = match super::super::process_session_public_aperture::process_child_lifecycle::drive_selected_process_invoke_child_to_matching_resume_for_process_kernel_owner_v1(
            selected_boundary,
            provider_execution_session,
        ) {
            Ok(driven_child) => driven_child,
            Err(failure) => {
                return Err(crate::direct_run::DirectRunProcessSessionDriveFaultV1::ProcessChildStage(
                    DirectRunProcessChildStageFaultV1::from_child_drive_for_process_kernel_owner_v1(
                        parent_route,
                        failure,
                    ),
                ));
            }
        };
        Ok(DirectRunProcessInvokeParentResumeStageV1 {
            parent_route,
            driven_child,
        })
    }
}

impl DirectRunProcessRunChildDriveStageV1 {
    pub(in crate::direct_run) fn drive_matching_child_for_process_kernel_owner_v1(
        self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Result<
        DirectRunProcessRunParentResumeStageV1,
        crate::direct_run::DirectRunProcessSessionDriveFaultV1,
    > {
        let Self {
            parent_route,
            selected_boundary,
        } = self;
        let driven_child = match super::super::process_session_public_aperture::process_child_lifecycle::drive_selected_process_run_child_to_matching_resume_for_process_kernel_owner_v1(
            selected_boundary,
            provider_execution_session,
        ) {
            Ok(driven_child) => driven_child,
            Err(failure) => {
                return Err(crate::direct_run::DirectRunProcessSessionDriveFaultV1::ProcessChildStage(
                    DirectRunProcessChildStageFaultV1::from_child_drive_for_process_kernel_owner_v1(
                        parent_route,
                        failure,
                    ),
                ));
            }
        };
        Ok(DirectRunProcessRunParentResumeStageV1 {
            parent_route,
            driven_child,
        })
    }
}

impl DirectRunProcessInvokeParentResumeStageV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn commit_and_route_parent_for_process_kernel_owner_v1(
        self,
    ) -> Result<
        DirectRunProcessSessionPublicApertureRouteOutput,
        crate::direct_run::DirectRunProcessSessionDriveFaultV1,
    > {
        let DirectRunDrivenProcessInvokeChildProductV1 {
            resume,
            event_publication_backend_output_drain_observations,
            process_output_records,
        } = self.driven_child;
        super::super::process_session_public_aperture::process_child_lifecycle::commit_process_invoke_resume_and_route_parent_for_process_kernel_owner_v1(
            self.parent_route,
            resume,
            event_publication_backend_output_drain_observations,
            process_output_records,
        )
    }
}

impl DirectRunProcessRunParentResumeStageV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn commit_and_route_parent_for_process_kernel_owner_v1(
        self,
    ) -> Result<
        DirectRunProcessSessionPublicApertureRouteOutput,
        crate::direct_run::DirectRunProcessSessionDriveFaultV1,
    > {
        let DirectRunDrivenProcessRunChildProductV1 {
            resume,
            process_output_records,
        } = self.driven_child;
        super::super::process_session_public_aperture::process_child_lifecycle::commit_process_run_resume_and_route_parent_for_process_kernel_owner_v1(
            self.parent_route,
            resume,
            process_output_records,
        )
    }
}
process_kernel_boundary_next_step_constructors!(
    DirectRunProcessRunDriveTerminalNextStepV1,
    crate::SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1
);
process_kernel_boundary_next_step_constructors!(
    DirectRunProcessControlNextStepV1,
    crate::SelectedProcessControlBoundaryForDirectRunOwnerV1
);

pub(in crate::direct_run) struct DirectRunHostResourceFinalizationNextStepV1 {
    route: DirectRunHostResourceFinalizationRouteV1,
    selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
}

enum DirectRunHostResourceFinalizationRouteV1 {
    Start {
        process_session_start_token: DirectRunProcessSessionStartContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    },
    Reawaken {
        process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
    },
    ProviderResume {
        provider_resume_token: super::typed_continuation::DirectRunProviderResumeContinuationToken,
        provider_resume_private_storage: super::process_session_owner_execution_substrate::DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
    },
}

impl DirectRunHostResourceFinalizationNextStepV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_start_owner_v1(
        process_session_start_token: DirectRunProcessSessionStartContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
        selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Self {
        Self {
            route: DirectRunHostResourceFinalizationRouteV1::Start {
                process_session_start_token,
                execution_substrate,
            },
            selected_boundary,
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_reawaken_owner_v1(
        process_session_reawaken_token: DirectRunProcessSessionReawakenContinuationToken,
        execution_substrate: DirectRunProcessSessionOwnerExecutionSubstrate,
        selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Self {
        Self {
            route: DirectRunHostResourceFinalizationRouteV1::Reawaken {
                process_session_reawaken_token,
                execution_substrate,
            },
            selected_boundary,
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_provider_resume_owner_v1(
        provider_resume_token: super::typed_continuation::DirectRunProviderResumeContinuationToken,
        provider_resume_private_storage: super::process_session_owner_execution_substrate::DirectRunProviderResumeHostBoundaryPrivateExecutionStorage,
        selected_boundary: crate::SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Self {
        Self {
            route: DirectRunHostResourceFinalizationRouteV1::ProviderResume {
                provider_resume_token,
                provider_resume_private_storage,
            },
            selected_boundary,
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn drive_for_direct_run_owner_v1(
        self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Result<DirectRunProcessSessionPublicApertureRouteOutput, String> {
        match self.route {
            DirectRunHostResourceFinalizationRouteV1::Start {
                process_session_start_token,
                execution_substrate,
            } => super::super::process_session_public_aperture::session_route_lifecycle::drive_start_route_host_resource_finalization_for_owner_v1(
                process_session_start_token,
                execution_substrate,
                self.selected_boundary,
                provider_execution_session,
            ),
            DirectRunHostResourceFinalizationRouteV1::Reawaken {
                process_session_reawaken_token,
                execution_substrate,
            } => super::super::process_session_public_aperture::session_route_lifecycle::drive_reawaken_route_host_resource_finalization_for_owner_v1(
                process_session_reawaken_token,
                execution_substrate,
                self.selected_boundary,
                provider_execution_session,
            ),
            DirectRunHostResourceFinalizationRouteV1::ProviderResume {
                provider_resume_token,
                provider_resume_private_storage,
            } => super::super::process_session_public_aperture::session_route_lifecycle::drive_provider_resume_route_host_resource_finalization_for_owner_v1(
                provider_resume_token,
                provider_resume_private_storage,
                self.selected_boundary,
                provider_execution_session,
            ),
        }
    }
}

enum DirectRunProcessSessionPublicApertureDriveOutputKindV1 {
    NextStep(DirectRunProcessSessionPublicApertureNextStepOutputProductV1),
    HostResourceFinalization(DirectRunHostResourceFinalizationNextStepV1),
    ProcessInvokeAwaitExecution(DirectRunProcessInvokeAwaitExecutionNextStepV1),
    ProcessRunDriveTerminal(DirectRunProcessRunDriveTerminalNextStepV1),
    ProcessControl(DirectRunProcessControlNextStepV1),
    Terminal(DirectRunProcessSessionPublicApertureOutputEmissionProductV1),
}

pub(in crate::direct_run) enum DirectRunProcessSessionPublicApertureClosedDriveStateV1 {
    NextStep(DirectRunProcessSessionPublicApertureNextStepOutputProductV1),
    HostResourceFinalization(DirectRunHostResourceFinalizationNextStepV1),
    ProcessInvokeAwaitExecution(DirectRunProcessInvokeAwaitExecutionNextStepV1),
    ProcessRunDriveTerminal(DirectRunProcessRunDriveTerminalNextStepV1),
    ProcessControl(DirectRunProcessControlNextStepV1),
    Terminal(DirectRunProcessSessionPublicApertureOutputEmissionProductV1),
}

pub struct DirectRunProcessSessionPublicApertureDriveOutputV1 {
    kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1,
}

impl std::fmt::Debug for DirectRunProcessSessionPublicApertureDriveOutputV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match &self.kind {
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::NextStep(_) => "next_step",
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::HostResourceFinalization(_) => {
                "host_resource_finalization"
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessInvokeAwaitExecution(
                _,
            ) => "process_invoke_await_execution",
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessRunDriveTerminal(_) => {
                "process_run_drive_terminal"
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessControl(_) => {
                "process_control"
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::Terminal(_) => "terminal",
        };
        formatter
            .debug_struct("DirectRunProcessSessionPublicApertureDriveOutputV1")
            .field("kind", &kind)
            .finish()
    }
}

enum DirectRunProcessSessionPublicApertureNextStepOutputKindV1 {
    KernelStateRef(DirectRunPublicApertureKernelStateRef),
}

pub(in crate::direct_run) struct DirectRunProcessSessionPublicApertureNextStepOutputProductV1 {
    kind: DirectRunProcessSessionPublicApertureNextStepOutputKindV1,
}

impl DirectRunProcessSessionPublicApertureNextStepOutputProductV1 {
    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_live_primitive_source_advance_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
        kernel_state_ref: DirectRunPublicApertureKernelStateRef,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureNextStepOutputKindV1::KernelStateRef(
                kernel_state_ref,
            ),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_process_liveness_drain_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
        kernel_state_ref: DirectRunPublicApertureKernelStateRef,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureNextStepOutputKindV1::KernelStateRef(
                kernel_state_ref,
            ),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn from_provider_resume_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
        kernel_state_ref: DirectRunPublicApertureKernelStateRef,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureNextStepOutputKindV1::KernelStateRef(
                kernel_state_ref,
            ),
        }
    }

    pub(in crate::direct_run::direct_run_runtime_authority_refs) fn owner_kind_for_direct_run_process_session_public_aperture_owner_v1(
        &self,
        operation: &'static str,
    ) -> Result<DirectRunKernelStateRefOwnerKind, String> {
        match &self.kind {
            DirectRunProcessSessionPublicApertureNextStepOutputKindV1::KernelStateRef(
                kernel_state_ref,
            ) => kernel_state_ref.owner_kind(operation),
        }
    }

    pub(in crate::direct_run) fn into_kernel_state_ref_for_direct_run_process_session_public_aperture_owner_v1(
        self,
    ) -> DirectRunPublicApertureKernelStateRef {
        match self.kind {
            DirectRunProcessSessionPublicApertureNextStepOutputKindV1::KernelStateRef(
                kernel_state_ref,
            ) => kernel_state_ref,
        }
    }
}

impl DirectRunProcessSessionPublicApertureDriveOutputV1 {
    pub(in crate::direct_run) fn from_next_step_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunProcessSessionPublicApertureNextStepOutputProductV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1::NextStep(output),
        }
    }

    pub(in crate::direct_run) fn from_host_resource_finalization_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunHostResourceFinalizationNextStepV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1::HostResourceFinalization(
                output,
            ),
        }
    }

    pub(in crate::direct_run) fn from_process_invoke_await_execution_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunProcessInvokeAwaitExecutionNextStepV1,
    ) -> Self {
        Self {
            kind:
                DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessInvokeAwaitExecution(
                    output,
                ),
        }
    }

    pub(in crate::direct_run) fn from_process_run_drive_terminal_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunProcessRunDriveTerminalNextStepV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessRunDriveTerminal(
                output,
            ),
        }
    }

    pub(in crate::direct_run) fn from_process_control_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunProcessControlNextStepV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessControl(output),
        }
    }

    pub(in crate::direct_run) fn from_terminal_output_for_direct_run_process_session_public_aperture_owner_v1(
        output: DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    ) -> Self {
        Self {
            kind: DirectRunProcessSessionPublicApertureDriveOutputKindV1::Terminal(output),
        }
    }

    pub fn is_next_step_for_libswarm_runtime_owner_v1(
        &self,
        poison: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> ProjectionCargoForbiddenAtAuthorityBoundary {
        match poison {}
    }

    pub fn into_next_step_for_libswarm_runtime_owner_v1(
        self,
        poison: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<ProjectionCargoForbiddenAtAuthorityBoundary, String> {
        match poison {}
    }

    pub(in crate::direct_run) fn into_closed_drive_state_for_direct_run_runtime_execution_owner_v1(
        self,
    ) -> DirectRunProcessSessionPublicApertureClosedDriveStateV1 {
        match self.kind {
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::NextStep(output) => {
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::NextStep(output)
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::HostResourceFinalization(
                output,
            ) => DirectRunProcessSessionPublicApertureClosedDriveStateV1::HostResourceFinalization(
                output,
            ),
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessInvokeAwaitExecution(
                output,
            ) => {
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessInvokeAwaitExecution(
                    output,
                )
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessRunDriveTerminal(
                output,
            ) => DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessRunDriveTerminal(
                output,
            ),
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::ProcessControl(output) => {
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::ProcessControl(output)
            }
            DirectRunProcessSessionPublicApertureDriveOutputKindV1::Terminal(output) => {
                DirectRunProcessSessionPublicApertureClosedDriveStateV1::Terminal(output)
            }
        }
    }
}

fn require_process_session_result_route_authority_matches_owner_token(
    route_authority: &DirectRunProcessSessionResultRouteAuthority,
    owner_token: DirectRunProcessSessionOwnerExecutionToken,
    projection_kind: &'static str,
) -> Result<(), String> {
    owner_token.require_matches_result_route_authority(route_authority, projection_kind)
}

#[cfg(test)]
#[path = "process_session_result_route/next_step_output_fixture_tests.rs"]
mod next_step_output_fixture_tests;
