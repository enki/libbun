#![forbid(unsafe_code)]
#![allow(unused)]
#![allow(private_interfaces)]

pub(crate) use swarm_capability_model::{
    CapabilityContractFingerprint, CapabilityContractIdentity, CapabilityContractProjection,
    CapabilitySdkError, CapabilitySdkResult,
};
pub(crate) use swarm_rust_sdk_capability::RustSdkProviderBinding;
use thiserror::Error;

mod admitted_runner_config;
mod compiler_owned_callable_authority;
mod compiler_owned_graph_authority;
mod compiler_owned_prepared_target_correlation;
mod compiler_owned_process_replan_authority;
pub(crate) use compiler_owned_callable_authority::*;
pub(crate) use compiler_owned_graph_authority::*;
pub(crate) use compiler_owned_process_replan_authority::{
    GraphReconcileObservationCarrierForSessionRuntimeOwnerV1,
    GraphReconcileObservationSnapshotJoinForSessionRuntimeOwnerV1,
    JoinedGraphReconcileObservationSnapshotForSessionRuntimeOwnerV1,
    ProcessReconcilePlanCarrierForSessionRuntimeOwnerV1,
};
mod direct_run;
mod prepared_source_program_image_owner;
mod program_assembly;
mod protocol_declaration_authority;
mod provider_drive_result;
mod provider_messages;
mod runtime_binding_owner;
pub(crate) use runtime_binding_owner::*;
mod typed_module_summary_owner;
pub(crate) use typed_module_summary_owner::*;
mod installed_capability_implementation_owner;
#[path = "source_entrypoint_executable_runtime.rs"]
mod session;
mod source_entrypoint_cold_plan;
mod source_entrypoint_compiler_admission_session;
mod source_entrypoint_direct_run_prepared_runtime;
#[path = "source_entrypoint_compiler_admission_session/test_declaration.rs"]
pub mod test_declaration;
pub use admitted_runner_config::{
    SsAdmittedRunnerConfig, SsAdmittedRunnerConfigAdmissionFault,
    SsAdmittedRunnerConfigSourceExecutionModeMismatchForSourceEntrypointColdMaterializationOwnerV1,
};
pub use prepared_source_program_image_owner::{
    SsTestPreselectedRuntimeImageAdmissionCancellationForCompilerOwnerV1,
    SsTestPreselectedRuntimeImageAdmissionRefusalForCompilerOwnerV1,
    SsTestPreselectedRuntimeImageCancellationForCompilerOwnerV1,
    SsTestPreselectedRuntimeImageForCompilerOwnerV1,
};
pub(crate) use program_assembly::*;
pub use provider_drive_result::{
    ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
    ProviderDriveParkedContinuationForDirectRunLiveOperationOwnerV1,
    ProviderDriveProcessOutputRecordProductForProviderHostOwner, ProviderDriveResult,
    ProviderDriveSessionExecutionCommitFault,
};
pub use provider_messages::ProviderParkReceipt;
pub use source_entrypoint_cold_plan::{
    SourceEntrypointColdPlan, SourceEntrypointColdPlanAdmissionFault, SourceEntrypointColdReason,
    SourceEntrypointColdReasonSet, SourceEntrypointExecutionAdmission,
    cold_required_for_source_entrypoint_execution_owner_v1,
};
pub use source_entrypoint_compiler_admission_session::source_work_set::ss_source_work_set_consume_executable_front_pass_admission_into_preselected_runtime_image_for_ss_test_execution_owner_v1;
pub use source_entrypoint_compiler_admission_session::source_work_set::{
    SsSourceWorkSetAdmittedDependencyGraph,
    SsSourceWorkSetAdmittedSourceInventoryForSsTestExecutionOwnerV1, SsSourceWorkSetCheckerError,
    SsSourceWorkSetClosureReadySelectedTestSourceForSsTestExecutionOwnerV1,
    SsSourceWorkSetExecutableFrontPassAdmissionForSsTestExecutionOwnerV1,
    SsSourceWorkSetLeasedSourceFactStepForSsTestExecutionOwnerV1,
    SsSourceWorkSetNonTerminalCheckerError,
    SsSourceWorkSetRefusedSelectedTestSourceForSsTestExecutionOwnerV1,
    SsSourceWorkSetSelectedNegativeTerminalCustody,
    SsSourceWorkSetSelectedTestSourceFeedAndWorkAdmissionForSsTestExecutionOwnerV1,
    SsSourceWorkSetSelectedTestSourceRuntimePlanCoverageForSsTestExecutionOwnerV1,
    SsSourceWorkSetSelectedTestSourceRuntimePlanFeedEmissionForSsTestExecutionOwnerV1,
    SsSourceWorkSetSourceFactsBundle,
    SsSourceWorkSetStreamingFactAdmissionForSsTestExecutionOwnerV1,
    SsSourceWorkSetStreamingFactApplicationForSsTestExecutionOwnerV1,
    SsSourceWorkSetUnadmittedTestSourceForSsTestExecutionOwnerV1,
    SsTestExactExpectationDecisionCorrespondenceFaultForSsTestPlanOwnerV1,
    SsTestExactExpectationDecisionForSsTestExecutionOwnerV1,
    SsTestExactExpectationFinalObservationForSsTestPlanOwnerV1,
    SsTestExactExpectationIdentityForSsTestExecutionOwnerV1,
    SsTestExactExpectationIssuanceReceiptForSsTestPlanOwnerV1,
    SsTestExactExpectationTokenForSsTestExecutionOwnerV1,
    SsTestExactObservedNonFailureForSsTestExecutionOwnerV1, SsTestSourceWorkSetGeneration,
    SsTestSourceWorkSetReceipt, SsTestSourceWorkSetReceiptFileCount,
    SsTestSourceWorkSetRuntimePlanFeedAdmissionForSsTestExecutionOwnerV1,
};
pub use source_entrypoint_compiler_admission_session::{
    SourceCompilerFault, admit_source_entrypoint_executable_closure,
    collect_ss_test_declaration_index_observation_from_admitted_source_module_for_ss_runtime_test_discovery_owner_v1,
};
pub use source_entrypoint_direct_run_prepared_runtime::{
    SourceEntrypointDirectRunCompileAdmissionCancellationForCompilerOwnerV1,
    SourceEntrypointDirectRunCompileAdmissionRefusal,
    SourceEntrypointDirectRunPreparationCancellationForCompilerOwnerV1,
    SourceEntrypointDirectRunPreparationRefusal, SourceEntrypointDirectRunPreparedRuntime,
    SourceEntrypointDirectRunPreparedRuntimeProcessStart,
    SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal,
    SourceEntrypointDirectRunPreparedRuntimeProcessStartCancellation,
    SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1,
    SourceEntrypointDirectRunTerminalForCompilerOwnerV1,
    SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1,
    SsTestDirectRunBodyWorkMaterializationCancellationForCompilerOwnerV1,
    SsTestDirectRunBodyWorkMaterializationForCompilerOwnerV1,
    SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1,
    SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1,
    admit_source_entrypoint_direct_run_prepared_runtime_process_start_for_compiler_owner_v1,
    cancel_source_entrypoint_direct_run_terminal_fault_for_compiler_owner_v1,
    drive_source_entrypoint_direct_run_prepared_runtime_process_start_until_terminal_for_compiler_owner_v1,
    settle_source_entrypoint_direct_run_terminal_into_final_observation_for_compiler_owner_v1,
};
pub(crate) use source_entrypoint_direct_run_prepared_runtime::{
    direct_run_ss_test_body_work_materialization_from_process_dispatch_product_for_compiler_owner_v1,
    prepare_source_entrypoint_direct_run_runtime_for_compiler_owner_v1,
    ss_test_selected_body_process_dispatch_product_from_front_pass_products_for_compiler_owner_v1,
};
// The complete executable runtime SCC is co-located with source compilation.
// This include retains the established crate-root vocabulary while the old
// session crate becomes a downstream facade.
include!("source_entrypoint_executable_runtime/root.inc.rs");
include!("direct_run_root.inc.rs");

#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub enum ProviderResumeBoundaryOwnerClass {
    CompletedProviderResultPayloadStoreAdmission,
    TypedLivenessContinuationOwner,
    TypedStreamContinuationOwner,
    TypedDeadlineContinuationOwner,
    TypedCancellationContinuationOwner,
}

impl ProviderResumeBoundaryOwnerClass {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::CompletedProviderResultPayloadStoreAdmission => {
                "completed_provider_result_payload_store_admission"
            }
            Self::TypedLivenessContinuationOwner => "typed_liveness_continuation_owner",
            Self::TypedStreamContinuationOwner => "typed_provider_stream_continuation_owner",
            Self::TypedDeadlineContinuationOwner => "typed_provider_deadline_continuation_owner",
            Self::TypedCancellationContinuationOwner => {
                "typed_provider_cancellation_continuation_owner"
            }
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::CompletedProviderResultPayloadStoreAdmission => {
                "completed ProviderResult payload-store admission"
            }
            Self::TypedLivenessContinuationOwner => {
                "typed parked-provider liveness continuation owner"
            }
            Self::TypedStreamContinuationOwner => "typed provider stream continuation owner",
            Self::TypedDeadlineContinuationOwner => "typed provider deadline continuation owner",
            Self::TypedCancellationContinuationOwner => {
                "typed provider cancellation continuation owner"
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum SsError {
    #[error("{0}")]
    Cli(String),
}

pub type SsResult<T> = Result<T, SsError>;
