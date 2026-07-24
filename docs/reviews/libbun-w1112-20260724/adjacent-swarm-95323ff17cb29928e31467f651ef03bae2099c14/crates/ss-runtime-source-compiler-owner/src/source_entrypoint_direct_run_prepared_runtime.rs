use crate::direct_run::{
    DirectRunColdMaterializationEvidenceForPreparedRuntimeOwnerV1,
    DirectRunPreparedSourceProgramImageInstallationRefusalV1,
    DirectSessionLirCompilerRuntimePreparedProgramForDirectRunOwnerV1,
    DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    direct_run_cold_materialization_evidence_from_front_pass_admission_v1,
};
use crate::source_entrypoint_compiler_admission_session::{
    checked_runtime_artifact_demand::{
        ProviderEffectExecutableImageTransitionFaultForRuntimeProgramArtifactOwnerV1,
        checked_runtime_program_artifact_demands_from_source_work_set_runtime_artifact_body_inputs_for_runtime_program_artifact_owner_v1,
    },
    public_surface::CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1,
    source_work_set::{
        CompilerPreparedExactOccurrenceSelectedStaticChildWorkDispositionForRuntimeProgramArtifactOwnerV1,
        CompilerSelectedExactOccurrenceStaticChildRoleTokenForRuntimeProgramArtifactOwnerV1,
        SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    },
};

#[must_use = "the prepared source-entrypoint runtime must be consumed by a compiler-owned direct-run operation"]
pub struct SourceEntrypointDirectRunPreparedRuntime {
    prepared_source_program_image_authority:
        DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    _module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    _runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

impl SourceEntrypointDirectRunPreparedRuntime {
    pub(crate) fn observe_post_compiler_success_cache_publication_for_compiler_owner_v1(
        self,
        success: &crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionCachePublicationSuccessForCompilerOwnerV1,
    ) -> Self {
        let Self {
            prepared_source_program_image_authority,
            _module_interface_runtime_custodies: module_interface_runtime_custodies,
            _runtime_artifact_observations: runtime_artifact_observations,
        } = self;
        let module_interface_runtime_custodies = module_interface_runtime_custodies
            .into_iter()
            .map(|custody| {
                custody
                    .observe_post_compiler_success_cache_publication_for_compiler_owner_v1(success)
            })
            .collect();
        Self {
            prepared_source_program_image_authority,
            _module_interface_runtime_custodies: module_interface_runtime_custodies,
            _runtime_artifact_observations: runtime_artifact_observations,
        }
    }
}

#[cfg(test)]
impl SourceEntrypointDirectRunPreparedRuntime {
    pub(crate) fn h7_contract_witness_section_payloads_for_compiler_root_test_v1(
        &self,
    ) -> Result<Vec<(String, serde_json::Value)>, serde_json::Error> {
        let mut payloads = Vec::new();
        for custody in &self._module_interface_runtime_custodies {
            payloads.extend(
                custody
                    .h7_contract_witness_section_payloads_for_module_interface_artifact_observation_v1()?,
            );
        }
        Ok(payloads)
    }
}

#[must_use = "the sealed ss-test dispatch must be consumed by the compiler-owned body-work operation"]
pub struct SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1 {
    prepared_source_program_image_authority:
        DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

#[must_use = "the compiler-owned ss-test body work must be consumed by the ss-test runtime owner"]
pub struct SsTestDirectRunBodyWorkMaterializationForCompilerOwnerV1 {
    _body_work: crate::DirectRunSsTestBodyWorkMaterializationAuthority,
    _module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    _runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

impl SsTestDirectRunBodyWorkMaterializationForCompilerOwnerV1 {
    pub(crate) fn consume_with_work_set_projection_for_preselected_image_owner_v1(
        self,
        work_set_projection: crate::source_entrypoint_compiler_admission_session::source_work_set::SsReadyTestFileWorkSetProjectionAuthority,
    ) -> crate::DirectRunSsTestExecutedFileResultAuthority {
        let Self {
            _body_work: body_work,
            _module_interface_runtime_custodies: module_interface_runtime_custodies,
            _runtime_artifact_observations: runtime_artifact_observations,
        } = self;
        body_work.consume_preselected_runtime_image_for_ss_test_result_owner_v1(
            work_set_projection,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
        )
    }
}

pub struct SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
    custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1,
}

enum SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1 {
    CompileAdmission {
        source: SourceEntrypointDirectRunCompileAdmissionRefusal,
        _transaction_root: crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
    },
    PreparedImage {
        source: CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1,
        _transaction_root: crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
    },
    LaunchValues {
        dispatch_product: SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1,
        provider_host_set: swarm_provider_host_set::ProviderHostSet,
        selected_body_launch: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSelectedTestBodyLaunchAuthority,
        source: crate::direct_run::DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1,
    },
    PreparedRuntimeStart {
        dispatch_product: SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1,
        launch_values: crate::direct_run::DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
        provider_host_set: swarm_provider_host_set::ProviderHostSet,
        selected_body_launch: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSelectedTestBodyLaunchAuthority,
        fault: crate::direct_run::DirectRunPreparedRuntimeImageStartFault,
    },
    ProviderAdmission {
        refusal:
            crate::direct_run::DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
        registered_case_launch_demand_set: crate::test_declaration::SsTestRegisteredCaseBodyLaunchDemandSetForDirectRunOwnerV1,
        module_interface_runtime_custodies:
            Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    },
    ProcessSessionDriveCleanup {
        fault: crate::direct_run::DirectRunProcessSessionDriveFaultV1,
        registered_case_launch_demand_set: crate::test_declaration::SsTestRegisteredCaseBodyLaunchDemandSetForDirectRunOwnerV1,
        module_interface_runtime_custodies:
            Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    },
}

pub struct SsTestDirectRunBodyWorkMaterializationCancellationForCompilerOwnerV1 {
    _custody: SsTestDirectRunBodyWorkMaterializationCancellationCustodyForCompilerOwnerV1,
}

enum SsTestDirectRunBodyWorkMaterializationCancellationCustodyForCompilerOwnerV1 {
    Retained(SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1),
    ProcessSessionDrive {
        registered_case_launch_demand_set:
            crate::test_declaration::SsTestRegisteredCaseBodyLaunchDemandSetForDirectRunOwnerV1,
        module_interface_runtime_custodies:
            Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    },
}

pub struct SourceEntrypointDirectRunPreparedRuntimeProcessStartCancellation {
    _custody: SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody,
}

#[must_use = "the admitted source-entrypoint direct run must be driven to its terminal product"]
pub struct SourceEntrypointDirectRunPreparedRuntimeProcessStart {
    admitted: crate::AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

#[must_use = "the compiler-owned terminal product must be settled into its final source-run observation"]
pub struct SourceEntrypointDirectRunTerminalForCompilerOwnerV1 {
    output: crate::DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

pub struct SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
    fault: crate::direct_run::DirectRunProcessSessionDriveFaultV1,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

pub struct SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {
    message: String,
    _module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    _runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

#[must_use = "the retained prepared-runtime start inputs must be explicitly cancelled"]
pub struct SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
    custody: SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody,
}

enum SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody {
    LaunchValues {
        prepared_runtime: SourceEntrypointDirectRunPreparedRuntime,
        provider_host_set: swarm_provider_host_set::ProviderHostSet,
        source: crate::direct_run::DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1,
    },
    PreparedImage {
        prepared_runtime: SourceEntrypointDirectRunPreparedRuntime,
        launch_values: crate::direct_run::DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
        provider_host_set: swarm_provider_host_set::ProviderHostSet,
        cause: crate::direct_run::DirectRunPreparedRuntimeImageStartFault,
    },
    ProviderAdmission {
        refusal:
            crate::direct_run::DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
        module_interface_runtime_custodies:
            Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    },
}

pub(crate) struct SourceEntrypointRuntimeArtifactObservationCustody {
    _protocol_declarations: crate::protocol_declaration_authority::AdmittedDeclarationTable,
}

pub struct SourceEntrypointDirectRunPreparationRefusal {
    cancellation: SourceEntrypointDirectRunPreparationCancellation,
}

#[cfg(test)]
impl SourceEntrypointDirectRunPreparationRefusal {
    pub(super) fn assert_injected_exact_image_settlement_refusal_cancelled_for_test(&self) {
        let SourceEntrypointDirectRunPreparationCancellation::InjectedExactImageSettlementRefusal {
            _custody_fault,
            ..
        } = &self.cancellation
        else {
            panic!("the fault injection must retain its exact-image preparation custody")
        };
        assert!(
            _custody_fault.is_none(),
            "cancelling the freshly minted Root ticket must succeed",
        );
    }
}

pub(crate) enum SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1 {
    Production,
    #[cfg(test)]
    RefuseExactImageSettlement,
    #[cfg(test)]
    DropRootAfterExactImageSettlement,
    #[cfg(test)]
    PanicAfterDroppingRootAfterExactImageSettlement,
}

enum SourceEntrypointDirectRunPreparationCancellation {
    CompileAdmission(SourceEntrypointDirectRunCompileAdmissionRefusal),
    PreparedImage {
        source: CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1,
        _custody_fault: Option<swarm_affine_custody_observation::CustodyOperationFault>,
    },
    CustodyBeforePreparedImage {
        _prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
        source: swarm_affine_custody_observation::CustodyOperationFault,
    },
    CustodyAfterPreparedImage {
        _receipt:
            CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1,
        source: swarm_affine_custody_observation::CustodyOperationFault,
    },
    #[cfg(test)]
    InjectedExactImageSettlementRefusal {
        _prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
        _custody_fault: Option<swarm_affine_custody_observation::CustodyOperationFault>,
    },
}

pub struct SourceEntrypointDirectRunPreparationCancellationForCompilerOwnerV1 {
    _custody: SourceEntrypointDirectRunPreparationCancellation,
}

pub struct SourceEntrypointDirectRunCompileAdmissionRefusal {
    cancellation: SourceEntrypointDirectRunCompileAdmissionCancellation,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

struct CompilerTransactionProgramStagingRefusalV1 {
    cancellation: SourceEntrypointDirectRunCompileAdmissionCancellation,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

pub struct SourceEntrypointDirectRunCompileAdmissionCancellationForCompilerOwnerV1 {
    _refusal: SourceEntrypointDirectRunCompileAdmissionRefusal,
}

enum SourceEntrypointDirectRunCompileAdmissionCancellation {
    ProviderEffectImage {
        source: ProviderEffectExecutableImageTransitionFaultForRuntimeProgramArtifactOwnerV1,
        _checked_process_lifecycle_argument_evidence: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedProcessLifecycleArgumentEvidenceForProcessLifecycleArgumentEvidenceOwnerV1,
        _tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
    },
    ProcessLifecycle {
        source: swarmscript_process_lifecycle_sidecar_authority::ProcessLifecyclePayloadCarrierAdmissionError,
        _selected_executable_image: crate::SourceEntrypointExecutableImage,
        _settled_static_child_work: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerSettledExactOccurrenceSelectedStaticChildWorkBatchForRuntimeProgramArtifactOwnerV1,
        _tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
    },
    ExecutableImage {
        source: crate::ExecutableSessionImagePreparationFault,
        _settled_static_child_work: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerSettledExactOccurrenceSelectedStaticChildWorkBatchForRuntimeProgramArtifactOwnerV1,
        _process_lifecycle_artifact_sidecar_segment:
            swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
        _tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
    },
    StaticChildCompilation {
        source: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceSelectedStaticChildWorkPreparationRefusalForRuntimeProgramArtifactOwnerV1,
        _prepared_runtime_entry: crate::SourceEntrypointExecutableRuntime,
        _process_lifecycle_artifact_sidecar_segment:
            swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
        _tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
    },
    StaticChildAcceptance {
        source: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceSelectedStaticChildRecursivePreparationAcceptanceRefusalForRuntimeProgramArtifactOwnerV1,
        _prepared_runtime_entry: crate::SourceEntrypointExecutableRuntime,
        _process_lifecycle_artifact_sidecar_segment:
            swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
        _tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
        _transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
    },
}

pub(crate) struct CompilerPreparationTailCustodyForSourceEntrypointOwnerV1 {
    boundary_decode_plan_set:
        swarmscript_types::BoundaryDecodePlanReDerivationSetForSwarmvmImageOwnerV1,
    direct_run_source_program_authority:
        libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
    front_pass_materialization_admission: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSourceWorkSetFrontPassPreparedRuntimeMaterializationAdmissionForDirectRunOwnerV1,
    capability_requirement_inventory:
        crate::CompilerSettledCapabilityRequirementInventoryForPreparedRuntimeOwnerV1,
}

impl std::fmt::Debug for SourceEntrypointDirectRunPreparationRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceEntrypointDirectRunPreparationRefusal")
            .field("message", &self.to_string())
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for SourceEntrypointDirectRunPreparationRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cancellation {
            SourceEntrypointDirectRunPreparationCancellation::CompileAdmission(source) => {
                source.fmt(formatter)
            }
            SourceEntrypointDirectRunPreparationCancellation::PreparedImage { source, .. } => {
                source.fmt(formatter)
            }
            SourceEntrypointDirectRunPreparationCancellation::CustodyBeforePreparedImage {
                source,
                ..
            }
            | SourceEntrypointDirectRunPreparationCancellation::CustodyAfterPreparedImage {
                source,
                ..
            } => write!(
                formatter,
                "source-entrypoint exact checked-image custody operation failed: {source:?}"
            ),
            #[cfg(test)]
            SourceEntrypointDirectRunPreparationCancellation::InjectedExactImageSettlementRefusal {
                ..
            } => formatter.write_str("injected exact checked-image settlement refusal"),
        }
    }
}

impl std::error::Error for SourceEntrypointDirectRunPreparationRefusal {}

impl std::fmt::Debug for SourceEntrypointDirectRunCompileAdmissionRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceEntrypointDirectRunCompileAdmissionRefusal")
            .field("message", &self.to_string())
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for SourceEntrypointDirectRunCompileAdmissionRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cancellation {
            SourceEntrypointDirectRunCompileAdmissionCancellation::ProviderEffectImage {
                source,
                ..
            } => source.fmt(formatter),
            SourceEntrypointDirectRunCompileAdmissionCancellation::ProcessLifecycle {
                source,
                ..
            } => source.fmt(formatter),
            SourceEntrypointDirectRunCompileAdmissionCancellation::ExecutableImage {
                source,
                ..
            } => source.fmt(formatter),
            SourceEntrypointDirectRunCompileAdmissionCancellation::StaticChildCompilation {
                source,
                ..
            } => source.fmt(formatter),
            SourceEntrypointDirectRunCompileAdmissionCancellation::StaticChildAcceptance {
                ..
            } => formatter
                .write_str("compiler static-child preparation acceptance correlation refused"),
        }
    }
}

impl std::error::Error for SourceEntrypointDirectRunCompileAdmissionRefusal {}

impl std::fmt::Debug for SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1")
            .field("message", &self.to_string())
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.custody {
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::CompileAdmission { source, .. } => source.fmt(formatter),
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::PreparedImage { source, .. } => source.fmt(formatter),
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::LaunchValues { source, .. } => source.fmt(formatter),
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::PreparedRuntimeStart { fault, .. } => fault.fmt(formatter),
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProviderAdmission { refusal, .. } => refusal.fmt(formatter),
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProcessSessionDriveCleanup { .. } => formatter.write_str("ss-test direct-run process-session cleanup refused"),
        }
    }
}

impl std::error::Error for SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {}

impl SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
    pub fn cancel_for_compiler_owner_v1(
        self,
    ) -> Result<SsTestDirectRunBodyWorkMaterializationCancellationForCompilerOwnerV1, Self> {
        match self.custody {
            SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProcessSessionDriveCleanup {
                fault,
                registered_case_launch_demand_set,
                module_interface_runtime_custodies,
                runtime_artifact_observations,
            } => match fault.cancel_into_generic_message_for_direct_run_boundary_owner_v1() {
                Ok(_message) => Ok(SsTestDirectRunBodyWorkMaterializationCancellationForCompilerOwnerV1 {
                    _custody: SsTestDirectRunBodyWorkMaterializationCancellationCustodyForCompilerOwnerV1::ProcessSessionDrive {
                        registered_case_launch_demand_set,
                        module_interface_runtime_custodies,
                        runtime_artifact_observations,
                    },
                }),
                Err(fault) => Err(Self {
                    custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProcessSessionDriveCleanup {
                        fault,
                        registered_case_launch_demand_set,
                        module_interface_runtime_custodies,
                        runtime_artifact_observations,
                    },
                }),
            },
            custody => Ok(SsTestDirectRunBodyWorkMaterializationCancellationForCompilerOwnerV1 {
                _custody: SsTestDirectRunBodyWorkMaterializationCancellationCustodyForCompilerOwnerV1::Retained(custody),
            }),
        }
    }
}

impl SourceEntrypointDirectRunPreparationRefusal {
    pub fn cancel_for_compiler_owner_v1(
        self,
    ) -> SourceEntrypointDirectRunPreparationCancellationForCompilerOwnerV1 {
        SourceEntrypointDirectRunPreparationCancellationForCompilerOwnerV1 {
            _custody: self.cancellation,
        }
    }
}

impl SourceEntrypointDirectRunCompileAdmissionRefusal {
    pub fn cancel_for_compiler_owner_v1(
        self,
    ) -> SourceEntrypointDirectRunCompileAdmissionCancellationForCompilerOwnerV1 {
        SourceEntrypointDirectRunCompileAdmissionCancellationForCompilerOwnerV1 { _refusal: self }
    }
}

impl std::fmt::Debug for SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal")
            .field("message", &self.to_string())
            .field("retained_inputs", &"<opaque>")
            .finish()
    }
}

impl std::fmt::Display for SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.custody {
            SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::LaunchValues {
                source,
                ..
            } => source.fmt(formatter),
            SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::PreparedImage {
                cause,
                ..
            } => write!(formatter, "prepared source-entrypoint runtime start refused: {cause}"),
            SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::ProviderAdmission {
                refusal,
                ..
            } => refusal.fmt(formatter),
        }
    }
}

impl std::error::Error for SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {}

impl std::fmt::Debug for SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1")
            .field("fault", &"<opaque typed runtime fault>")
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("source-entrypoint direct run reached a typed runtime fault")
    }
}

impl std::error::Error for SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {}

impl std::fmt::Debug for SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1")
            .field("message", &self.message)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {}

impl SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
    pub fn cancel_for_compiler_owner_v1(
        self,
    ) -> SourceEntrypointDirectRunPreparedRuntimeProcessStartCancellation {
        SourceEntrypointDirectRunPreparedRuntimeProcessStartCancellation {
            _custody: self.custody,
        }
    }
}

pub(crate) struct CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
    prepared_runtime_program: DirectSessionLirCompilerRuntimePreparedProgramForDirectRunOwnerV1,
    direct_run_source_program_authority:
        libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
    cold_materialization_evidence: DirectRunColdMaterializationEvidenceForPreparedRuntimeOwnerV1,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
}

/// Linear correspondence minted only by the compiler transaction root.  The
/// exact selected child role travels with that child's front pass, frame, and
/// prepared program; parent acceptance consumes it instead of reconstructing
/// correspondence from stack position or projected identity.
pub(crate) enum CompilerTransactionProgramCorrespondenceV1 {
    Root,
    StaticChild {
        role: CompilerSelectedExactOccurrenceStaticChildRoleTokenForRuntimeProgramArtifactOwnerV1,
    },
    AcceptedStaticChild,
}

pub(crate) struct CompilerTransactionSelectedStaticChildFrontPassV1 {
    front_pass: SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    correspondence: CompilerTransactionProgramCorrespondenceV1,
}

impl CompilerTransactionSelectedStaticChildFrontPassV1 {
    pub(crate) fn from_root_selected_child_for_compiler_owner_v1(
        role: CompilerSelectedExactOccurrenceStaticChildRoleTokenForRuntimeProgramArtifactOwnerV1,
        front_pass: SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    ) -> Self {
        Self {
            front_pass,
            correspondence: CompilerTransactionProgramCorrespondenceV1::StaticChild { role },
        }
    }

    fn consume_for_compiler_transaction_root_v1(
        self,
    ) -> (
        SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
        CompilerTransactionProgramCorrespondenceV1,
    ){
        (self.front_pass, self.correspondence)
    }
}

/// Private staging for the session-root exact-call settlement. Each accepted
/// child becomes the existing direct-run child product exactly once while its
/// authored-order ledger tree remains paired until the root image commit.
#[must_use = "exact-call image settlement staging must be finished by the session owner"]
pub(crate) struct CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1 {
    prepared_runtime_entry: crate::SourceEntrypointExecutableRuntime,
    process_lifecycle_artifact_sidecar_segment:
        swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
    tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
    prepared_children: Vec<
        crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildForRuntimeProgramArtifactOwnerV1,
    >,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
    transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
}

/// One authored program suspended on the compiler transaction root's private
/// heap stack.  A parent frame retains its exact child-role continuation while
/// the child frame is compiled; completed child products return only to that
/// parent frame.
pub(crate) struct CompilerTransactionProgramWorkFrameV1 {
    staging: CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1,
    state: CompilerTransactionProgramWorkStateV1,
}

enum CompilerTransactionProgramWorkStateV1 {
    Ready(
        crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerPreparedExactOccurrenceSelectedStaticChildWorkBatchForRuntimeProgramArtifactOwnerV1,
    ),
    AwaitingChild {
        continuation: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceSelectedStaticChildRecursivePreparationContinuationForRuntimeProgramArtifactOwnerV1,
    },
}

enum CompilerTransactionProgramWorkActionV1 {
    Complete(CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1),
    PrepareChild {
        parent: CompilerTransactionProgramWorkFrameV1,
        role: CompilerSelectedExactOccurrenceStaticChildRoleTokenForRuntimeProgramArtifactOwnerV1,
        child_front_pass: SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    },
}

#[must_use = "the committed exact-call image receipt must enter its selected runtime path"]
pub(crate) struct CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1
{
    prepared_source_program_image_authority:
        DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

#[must_use = "the exact-call image refusal retains settlement custody until explicit cancellation"]
pub(crate) struct CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1
{
    source: DirectRunPreparedSourceProgramImageInstallationRefusalV1,
    _module_interface_runtime_custodies:
        Vec<CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1>,
    _runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
}

impl std::fmt::Debug
    for CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(
                "CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1",
            )
            .field("message", &self.to_string())
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display
    for CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(formatter)
    }
}

impl std::error::Error
    for CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1
{
}

impl CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1 {
    pub(crate) fn begin_for_source_entrypoint_owner_v1(
        prepared_runtime_entry: crate::SourceEntrypointExecutableRuntime,
        process_lifecycle_artifact_sidecar_segment: swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
        tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
        module_interface_runtime_custodies: Vec<
            CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1,
        >,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
        transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
    ) -> Self {
        Self {
            prepared_runtime_entry,
            process_lifecycle_artifact_sidecar_segment,
            tail,
            prepared_children: Vec::new(),
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence,
        }
    }

    pub(crate) fn accept_exact_module_run_for_source_entrypoint_owner_v1(
        mut self,
        target: crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceStaticChildModuleRunTargetForRuntimeProgramArtifactOwnerV1,
        site_plan_reservation: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceStaticChildSitePlanReservationForRuntimeProgramArtifactOwnerV1,
        source_module: swarmscript_source::SourceRuntimeAdmittedModuleInput,
        prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    ) -> Self {
        let CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence: _,
        } = prepared_program;
        self.prepared_children.push(
            crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildForRuntimeProgramArtifactOwnerV1::from_settled_work_and_prepared_child_for_source_entrypoint_owner_v1(
                crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceSelectedStaticChildWorkForRuntimeProgramArtifactOwnerV1::ModuleRun {
                    target,
                    site_plan_reservation,
                    source_module,
                },
                crate::DirectRunRungMStaticChildPreparedChildProgramForPreparedRuntimeOwnerV1::from_prepared_runtime_program_for_source_compiler_owner_v1(
                direct_run_source_program_authority,
                prepared_runtime_program,
                cold_materialization_evidence,
            ),
            ),
        );
        self.module_interface_runtime_custodies
            .extend(module_interface_runtime_custodies);
        self.runtime_artifact_observations
            .extend(runtime_artifact_observations);
        self
    }

    pub(crate) fn accept_exact_process_load_for_source_entrypoint_owner_v1(
        mut self,
        target: crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceStaticChildProcessLoadTargetForRuntimeProgramArtifactOwnerV1,
        site_plan_reservation: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceStaticChildSitePlanReservationForRuntimeProgramArtifactOwnerV1,
        source_module: swarmscript_source::SourceRuntimeAdmittedModuleInput,
        prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    ) -> Self {
        let CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence: _,
        } = prepared_program;
        self.prepared_children.push(
            crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildForRuntimeProgramArtifactOwnerV1::from_settled_work_and_prepared_child_for_source_entrypoint_owner_v1(
                crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceSelectedStaticChildWorkForRuntimeProgramArtifactOwnerV1::ProcessLoad {
                    target,
                    site_plan_reservation,
                    source_module,
                },
                crate::DirectRunRungMStaticChildPreparedChildProgramForPreparedRuntimeOwnerV1::from_prepared_runtime_program_for_source_compiler_owner_v1(
                    direct_run_source_program_authority,
                    prepared_runtime_program,
                    cold_materialization_evidence,
                ),
            ),
        );
        self.module_interface_runtime_custodies
            .extend(module_interface_runtime_custodies);
        self.runtime_artifact_observations
            .extend(runtime_artifact_observations);
        self
    }

    pub(crate) fn accept_exact_process_restore_for_source_entrypoint_owner_v1(
        mut self,
        target: crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceStaticChildProcessRestoreTargetForRuntimeProgramArtifactOwnerV1,
        site_plan_reservation: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceStaticChildSitePlanReservationForRuntimeProgramArtifactOwnerV1,
        source_module: swarmscript_source::SourceRuntimeAdmittedModuleInput,
        prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    ) -> Self {
        let CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence: _,
        } = prepared_program;
        self.prepared_children.push(
            crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildForRuntimeProgramArtifactOwnerV1::from_settled_work_and_prepared_child_for_source_entrypoint_owner_v1(
                crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceSelectedStaticChildWorkForRuntimeProgramArtifactOwnerV1::ProcessRestore {
                    target,
                    site_plan_reservation,
                    source_module,
                },
                crate::DirectRunRungMStaticChildPreparedChildProgramForPreparedRuntimeOwnerV1::from_prepared_runtime_program_for_source_compiler_owner_v1(
                    direct_run_source_program_authority,
                    prepared_runtime_program,
                    cold_materialization_evidence,
                ),
            ),
        );
        self.module_interface_runtime_custodies
            .extend(module_interface_runtime_custodies);
        self.runtime_artifact_observations
            .extend(runtime_artifact_observations);
        self
    }

    pub(crate) fn accept_exact_selected_entry_for_source_entrypoint_owner_v1(
        mut self,
        target: crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceStaticChildSelectedEntryTargetForRuntimeProgramArtifactOwnerV1,
        site_plan_reservation: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerExactOccurrenceStaticChildSitePlanReservationForRuntimeProgramArtifactOwnerV1,
        selected_entry_name: swarmscript_bind::ValueName,
        source_module: swarmscript_source::SourceRuntimeAdmittedModuleInput,
        prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    ) -> Self {
        let CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence: _,
        } = prepared_program;
        self.prepared_children.push(
            crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildForRuntimeProgramArtifactOwnerV1::from_settled_work_and_prepared_child_for_source_entrypoint_owner_v1(
                crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrenceSelectedStaticChildWorkForRuntimeProgramArtifactOwnerV1::SelectedEntry {
                    target,
                    site_plan_reservation,
                    source_module,
                    selected_entry_name,
                },
                crate::DirectRunRungMStaticChildPreparedChildProgramForPreparedRuntimeOwnerV1::from_prepared_runtime_program_for_source_compiler_owner_v1(
                direct_run_source_program_authority,
                prepared_runtime_program,
                cold_materialization_evidence,
            ),
            ),
        );
        self.module_interface_runtime_custodies
            .extend(module_interface_runtime_custodies);
        self.runtime_artifact_observations
            .extend(runtime_artifact_observations);
        self
    }

    pub(crate) fn finish_for_source_entrypoint_owner_v1(
        self,
        settled_exact_occurrence_ledger: crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerExactOccurrenceProviderRouteLedgerForTypecheckOwnerV1,
    ) -> CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
        let Self {
            prepared_runtime_entry,
            process_lifecycle_artifact_sidecar_segment,
            tail,
            prepared_children,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence,
        } = self;
        let CompilerPreparationTailCustodyForSourceEntrypointOwnerV1 {
            boundary_decode_plan_set,
            direct_run_source_program_authority,
            front_pass_materialization_admission,
            capability_requirement_inventory,
        } = tail;
        let prepared_runtime_program = crate::direct_run::direct_session_lir_compiler_runtime_prepared_program_for_source_compiler_owner_v1(
            prepared_runtime_entry,
            process_lifecycle_artifact_sidecar_segment,
            boundary_decode_plan_set,
            crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::CompilerExactCheckedCallLedgerCommitCustodyForSourceEntrypointOwnerV1::from_settled_root_for_source_entrypoint_owner_v1(
                crate::session::execution_kernel::executable_image::source_plan_preparation::checked_body_plan::CompilerSettledExactOccurrencePreparedStaticChildDemandForRuntimeProgramArtifactOwnerV1::reunite_after_recursive_child_preparation_for_runtime_program_artifact_owner_v1(
                    settled_exact_occurrence_ledger,
                    prepared_children,
                ),
            ),
            capability_requirement_inventory,
        );
        let cold_materialization_evidence =
            direct_run_cold_materialization_evidence_from_front_pass_admission_v1(
                front_pass_materialization_admission,
            );
        CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence,
        }
    }
}

impl CompilerTransactionProgramWorkFrameV1 {
    fn new_for_compiler_transaction_root_v1(
        prepared_runtime_entry: crate::SourceEntrypointExecutableRuntime,
        process_lifecycle_artifact_sidecar_segment: swarmscript_process_lifecycle_sidecar_authority::ProcessLifecycleArtifactSidecarSegmentForSessionRuntimeOwnerV1,
        tail: CompilerPreparationTailCustodyForSourceEntrypointOwnerV1,
        prepared_static_child_work: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerPreparedExactOccurrenceSelectedStaticChildWorkBatchForRuntimeProgramArtifactOwnerV1,
        module_interface_runtime_custodies: Vec<
            CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1,
        >,
        runtime_artifact_observations: Vec<SourceEntrypointRuntimeArtifactObservationCustody>,
        transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
    ) -> Self {
        Self {
            staging: CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1::begin_for_source_entrypoint_owner_v1(
                prepared_runtime_entry,
                process_lifecycle_artifact_sidecar_segment,
                tail,
                module_interface_runtime_custodies,
                runtime_artifact_observations,
                transaction_correspondence,
            ),
            state: CompilerTransactionProgramWorkStateV1::Ready(prepared_static_child_work),
        }
    }

    fn advance_for_compiler_transaction_root_v1(self) -> CompilerTransactionProgramWorkActionV1 {
        let Self { staging, state } = self;
        let CompilerTransactionProgramWorkStateV1::Ready(prepared_static_child_work) = state else {
            unreachable!(
                "the current compiler transaction frame cannot advance while awaiting its exact child"
            )
        };
        match prepared_static_child_work.take_next_for_runtime_program_artifact_owner_v1() {
            CompilerPreparedExactOccurrenceSelectedStaticChildWorkDispositionForRuntimeProgramArtifactOwnerV1::Complete(completed) => {
                let completed = completed
                    .consume_into_exact_checked_call_executable_image_settlement_for_source_entrypoint_owner_v1();
                let CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1 {
                    prepared_runtime_entry,
                    process_lifecycle_artifact_sidecar_segment,
                    tail,
                    prepared_children: _,
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                    transaction_correspondence,
                } = staging;
                CompilerTransactionProgramWorkActionV1::Complete(
                    completed.consume_with_root_runtime_and_publication_custody_for_source_entrypoint_owner_v1(
                        prepared_runtime_entry,
                        process_lifecycle_artifact_sidecar_segment,
                        tail,
                        module_interface_runtime_custodies,
                        runtime_artifact_observations,
                        transaction_correspondence,
                    ),
                )
            }
            CompilerPreparedExactOccurrenceSelectedStaticChildWorkDispositionForRuntimeProgramArtifactOwnerV1::Next {
                current,
                remaining,
            } => {
                let (role, child_front_pass) =
                    current.consume_for_runtime_program_artifact_owner_v1();
                CompilerTransactionProgramWorkActionV1::PrepareChild {
                    parent: Self {
                        staging,
                        state: CompilerTransactionProgramWorkStateV1::AwaitingChild {
                            continuation: remaining,
                        },
                    },
                    role,
                    child_front_pass,
                }
            }
        }
    }

    fn accept_prepared_child_for_compiler_transaction_root_v1(
        self,
        prepared_program: CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    ) -> Result<Self, CompilerTransactionProgramStagingRefusalV1> {
        let Self { staging, state } = self;
        let CompilerTransactionProgramWorkStateV1::AwaitingChild { continuation } = state else {
            unreachable!("only the exact parent frame may accept a completed child")
        };
        let (role, prepared_program) =
            prepared_program.consume_static_child_correspondence_for_compiler_transaction_root_v1();
        match continuation
            .accept_prepared_child_for_runtime_program_artifact_owner_v1(role, prepared_program)
        {
            Ok(prepared_static_child_work) => Ok(Self {
                staging,
                state: CompilerTransactionProgramWorkStateV1::Ready(prepared_static_child_work),
            }),
            Err(source) => {
                let CompilerExactCheckedCallExecutableImageSettlementStagingForSourceEntrypointOwnerV1 {
                    prepared_runtime_entry,
                    process_lifecycle_artifact_sidecar_segment,
                    tail,
                    prepared_children: _,
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                    transaction_correspondence,
                } = staging;
                Err(CompilerTransactionProgramStagingRefusalV1 {
                    cancellation: SourceEntrypointDirectRunCompileAdmissionCancellation::StaticChildAcceptance {
                        source,
                        _prepared_runtime_entry: prepared_runtime_entry,
                        _process_lifecycle_artifact_sidecar_segment:
                            process_lifecycle_artifact_sidecar_segment,
                        _tail: tail,
                        _transaction_correspondence: transaction_correspondence,
                    },
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                })
            }
        }
    }
}

impl CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1 {
    fn consume_static_child_correspondence_for_compiler_transaction_root_v1(
        mut self,
    ) -> (
        CompilerSelectedExactOccurrenceStaticChildRoleTokenForRuntimeProgramArtifactOwnerV1,
        Self,
    ) {
        let correspondence = std::mem::replace(
            &mut self.transaction_correspondence,
            CompilerTransactionProgramCorrespondenceV1::AcceptedStaticChild,
        );
        let CompilerTransactionProgramCorrespondenceV1::StaticChild { role } = correspondence
        else {
            unreachable!("only a root-minted static-child program may return to a parent")
        };
        (role, self)
    }

    pub(crate) fn consume_into_exact_checked_call_executable_image_settlement_for_source_entrypoint_owner_v1(
        self,
    ) -> Result<
        CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1,
        CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1,
    > {
        let Self {
            prepared_runtime_program,
            direct_run_source_program_authority,
            cold_materialization_evidence,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence,
        } = self;
        let CompilerTransactionProgramCorrespondenceV1::Root = transaction_correspondence else {
            unreachable!("only the compiler transaction root program may install the final image")
        };
        match crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::install_compiler_prepared_source_program_image_for_source_entrypoint_owner_v1(
            direct_run_source_program_authority,
            prepared_runtime_program,
            cold_materialization_evidence,
        ) {
            Ok(prepared_source_program_image_authority) => Ok(
                CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1 {
                    prepared_source_program_image_authority,
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                },
            ),
            Err(source) => Err(
                CompilerExactCheckedCallExecutableImageSettlementRefusalForSourceEntrypointOwnerV1 {
                    source,
                    _module_interface_runtime_custodies: module_interface_runtime_custodies,
                    _runtime_artifact_observations: runtime_artifact_observations,
                },
            ),
        }
    }
}

pub(crate) fn ss_test_selected_body_process_dispatch_product_from_front_pass_products_for_compiler_owner_v1(
    runtime_artifact_body_inputs: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedRuntimeArtifactBodyInputsForExecutableBodyLoweringOwnerV1,
    checked_process_lifecycle_argument_evidence: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedProcessLifecycleArgumentEvidenceForProcessLifecycleArgumentEvidenceOwnerV1,
    protocol_declaration_table: crate::protocol_declaration_authority::AdmittedDeclarationTable,
    selected_provider_command_evidence_batch: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerSelectedProviderCommandEvidenceBatchForExecutableImageOwnerV1,
    direct_run_source_program_authority: libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
    front_pass_materialization_admission: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSourceWorkSetFrontPassPreparedRuntimeMaterializationAdmissionForDirectRunOwnerV1,
    capability_requirement_inventory:
        crate::CompilerSettledCapabilityRequirementInventoryForPreparedRuntimeOwnerV1,
    transaction_root: crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
) -> Result<
    SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1,
    SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1,
> {
    let mut transaction_root = transaction_root;
    let prepared_program =
        match prepare_compiler_runtime_program_from_consumed_products_for_source_entrypoint_owner_v1(
            runtime_artifact_body_inputs,
            checked_process_lifecycle_argument_evidence,
            protocol_declaration_table,
            selected_provider_command_evidence_batch,
            direct_run_source_program_authority,
            front_pass_materialization_admission,
            capability_requirement_inventory,
            Vec::new(),
            &mut transaction_root,
        ) {
            Ok(prepared_program) => prepared_program,
            Err(source) => {
                return Err(SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
                custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::CompileAdmission {
                    source,
                    _transaction_root: transaction_root,
                },
            });
            }
        };
    let CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1 {
        prepared_source_program_image_authority,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::settle_exact_checked_call_executable_image_for_source_entrypoint_owner_v1(
        prepared_program,
    )
    .map_err(
        |source| SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
            custody:
                SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::PreparedImage {
                    source,
                    _transaction_root: transaction_root,
                },
        },
    )?;
    Ok(SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1 {
        prepared_source_program_image_authority,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    })
}

pub(crate) fn direct_run_ss_test_body_work_materialization_from_process_dispatch_product_for_compiler_owner_v1(
    dispatch_product: SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1,
    run_namespace: String,
    run_suffix: String,
    started_at: String,
    program_args: Vec<String>,
    launch_cwd: Option<String>,
    provider_host_set: swarm_provider_host_set::ProviderHostSet,
    selected_body_launch: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSelectedTestBodyLaunchAuthority,
) -> Result<
    SsTestDirectRunBodyWorkMaterializationForCompilerOwnerV1,
    SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1,
> {
    let launch_values = match crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::admit_process_start_launch_values_for_compiler_owner_v1(
        crate::direct_run::DirectRunProcessStartLaunchValuesForCompilerOwnerV1::new(
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        ),
    ) {
        Ok(launch_values) => launch_values,
        Err(source) => {
            return Err(SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
                custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::LaunchValues {
                    dispatch_product,
                    provider_host_set,
                    selected_body_launch,
                    source,
                },
            });
        }
    };
    let SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1 {
        prepared_source_program_image_authority,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = dispatch_product;
    let execution_authority = match crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::execution_authority_from_prepared_source_program_image_authority(
        prepared_source_program_image_authority,
    ) {
        Ok(execution_authority) => execution_authority,
        Err((prepared_source_program_image_authority, fault)) => {
            return Err(SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
                custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::PreparedRuntimeStart {
                    dispatch_product: SsTestSelectedBodyProcessDispatchProductForCompilerOwnerV1 {
                        prepared_source_program_image_authority,
                        module_interface_runtime_custodies,
                        runtime_artifact_observations,
                    },
                    launch_values,
                    provider_host_set,
                    selected_body_launch,
                    fault,
                },
            });
        }
    };
    let selected_callable_body_result_binding_handoff = selected_body_launch
        .consume_into_callable_body_result_binding_handoff_for_direct_run_ss_test_body_launch_owner_v1();
    let (process_start_drive, registered_case_launch_demand_set) =
        crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::ss_test_public_aperture_process_start_drive_from_execution_authority_and_callable_body_result_binding_handoff(
            execution_authority,
            selected_callable_body_result_binding_handoff,
        );
    let command = crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::public_aperture_prepared_runtime_process_start_command(
        process_start_drive,
        launch_values,
    );
    let admission_input =
        crate::direct_run::direct_run_public_aperture_prepared_runtime_process_start_admission_input_v1(
            command,
            provider_host_set.begin_provider_execution_session_v1(),
        );
    let admitted =
        match crate::direct_run::admit_direct_run_public_aperture_prepared_runtime_process_start_v1(
            admission_input,
        ) {
            Ok(admitted) => admitted,
            Err(refusal) => {
                return Err(SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
                custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProviderAdmission {
                    refusal,
                    registered_case_launch_demand_set,
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                },
            });
            }
        };
    let root = match crate::direct_run::drive_direct_run_public_aperture_prepared_runtime_process_start_command_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
        admitted,
    ) {
        Ok(terminal) => terminal
            .into_ss_test_body_work_materialization_root_authority_for_direct_run_ss_test_body_work_owner_v1(),
        Err(crate::direct_run::DirectRunProcessSessionDriveFaultV1::RuntimeTerminal {
            observation,
            context,
        }) => crate::direct_run::DirectRunSsTestBodyWorkMaterializationRootAuthority::from_runtime_terminal_observation_for_direct_run_ss_test_body_work_owner_v1(
            observation,
            context,
        ),
        Err(crate::direct_run::DirectRunProcessSessionDriveFaultV1::Generic(message)) => {
            crate::direct_run::DirectRunSsTestBodyWorkMaterializationRootAuthority::from_runtime_projection_fault_for_direct_run_ss_test_body_work_owner_v1(
                crate::DurableExecutionProjectionJsonBoundaryFault::Projection {
                    operation: "direct_run_ss_test_body_work_materialization_from_process_dispatch_product_for_compiler_owner_v1",
                    message,
                },
            )
        }
        Err(fault) => {
            let message = match fault
                .cancel_into_generic_message_for_direct_run_boundary_owner_v1()
            {
                Ok(message) => message,
                Err(fault) => {
                    return Err(SsTestDirectRunBodyWorkMaterializationRefusalForCompilerOwnerV1 {
                        custody: SsTestDirectRunBodyWorkMaterializationRefusalCustodyForCompilerOwnerV1::ProcessSessionDriveCleanup {
                            fault,
                            registered_case_launch_demand_set,
                            module_interface_runtime_custodies,
                            runtime_artifact_observations,
                        },
                    });
                }
            };
            crate::direct_run::DirectRunSsTestBodyWorkMaterializationRootAuthority::from_runtime_projection_fault_for_direct_run_ss_test_body_work_owner_v1(
                crate::DurableExecutionProjectionJsonBoundaryFault::Projection {
                    operation: "direct_run_ss_test_body_work_materialization_from_process_dispatch_product_for_compiler_owner_v1",
                    message,
                },
            )
        }
    };
    let body_work = crate::direct_run::DirectRunSsTestBodyWorkMaterializationAuthority::from_root_authority_and_registered_case_launch_demands_for_direct_run_ss_test_body_work_owner_v1(
        root,
        registered_case_launch_demand_set,
    );
    Ok(SsTestDirectRunBodyWorkMaterializationForCompilerOwnerV1 {
        _body_work: body_work,
        _module_interface_runtime_custodies: module_interface_runtime_custodies,
        _runtime_artifact_observations: runtime_artifact_observations,
    })
}

pub(crate) fn prepare_source_entrypoint_direct_run_runtime_for_compiler_owner_v1(
    front_pass_products: SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    transaction_root: &mut crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
    exact_checked_image_settlement_family: &crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::SourceEntrypointExactCheckedImageSettlementFamilyEpochV1,
    exact_checked_image_custody_fault_injection: SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1,
) -> Result<SourceEntrypointDirectRunPreparedRuntime, SourceEntrypointDirectRunPreparationRefusal> {
    let prepared_program =
        prepare_compiler_runtime_program_from_front_pass_products_for_source_entrypoint_owner_v1(
            front_pass_products,
            transaction_root,
        )
        .map_err(|source| SourceEntrypointDirectRunPreparationRefusal {
            cancellation: SourceEntrypointDirectRunPreparationCancellation::CompileAdmission(
                source,
            ),
        })?;
    transaction_root.mark_program_forest_prepared_for_compiler_owner_v1();
    let exact_checked_image_settlement_ticket = match crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::mint_source_entrypoint_exact_checked_image_settlement_root_v1(
        exact_checked_image_settlement_family,
    ) {
        Ok(ticket) => ticket,
        Err(source) => {
            return Err(SourceEntrypointDirectRunPreparationRefusal {
                cancellation:
                    SourceEntrypointDirectRunPreparationCancellation::CustodyBeforePreparedImage {
                        _prepared_program: prepared_program,
                        source,
                    },
            });
        }
    };
    #[cfg(test)]
    if matches!(
        &exact_checked_image_custody_fault_injection,
        SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1::RefuseExactImageSettlement
    ) {
        let custody_fault = crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::cancel_source_entrypoint_exact_checked_image_settlement_root_v1(
            exact_checked_image_settlement_ticket,
        )
        .err();
        return Err(SourceEntrypointDirectRunPreparationRefusal {
            cancellation:
                SourceEntrypointDirectRunPreparationCancellation::InjectedExactImageSettlementRefusal {
                    _prepared_program: prepared_program,
                    _custody_fault: custody_fault,
                },
        });
    }
    let exact_checked_image_settlement_receipt =
        match crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::settle_exact_checked_call_executable_image_for_source_entrypoint_owner_v1(
            prepared_program,
        ) {
            Ok(receipt) => receipt,
            Err(source) => {
                let custody_fault = crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::cancel_source_entrypoint_exact_checked_image_settlement_root_v1(
                    exact_checked_image_settlement_ticket,
                )
                .err();
                return Err(SourceEntrypointDirectRunPreparationRefusal {
                    cancellation: SourceEntrypointDirectRunPreparationCancellation::PreparedImage {
                        source,
                        _custody_fault: custody_fault,
                    },
                });
            }
        };
    #[cfg(test)]
    match exact_checked_image_custody_fault_injection {
        SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1::DropRootAfterExactImageSettlement => {
            drop(exact_checked_image_settlement_ticket);
            return Ok(source_entrypoint_direct_run_prepared_runtime_from_exact_image_receipt_for_compiler_owner_v1(
                exact_checked_image_settlement_receipt,
            ));
        }
        SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1::PanicAfterDroppingRootAfterExactImageSettlement => {
            drop(exact_checked_image_settlement_ticket);
            panic!("semantic compiler panic after exact-image settlement");
        }
        SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1::Production
        | SourceEntrypointExactCheckedImageCustodyFaultInjectionForCompilerOwnerV1::RefuseExactImageSettlement => {}
    }
    if let Err(source) = crate::source_entrypoint_compiler_admission_session::exact_checked_call_executable_image_settlement_owner::settle_source_entrypoint_exact_checked_image_settlement_root_v1(
        exact_checked_image_settlement_ticket,
    ) {
        return Err(SourceEntrypointDirectRunPreparationRefusal {
            cancellation:
                SourceEntrypointDirectRunPreparationCancellation::CustodyAfterPreparedImage {
                    _receipt: exact_checked_image_settlement_receipt,
                    source,
            },
        });
    }
    Ok(source_entrypoint_direct_run_prepared_runtime_from_exact_image_receipt_for_compiler_owner_v1(
        exact_checked_image_settlement_receipt,
    ))
}

fn source_entrypoint_direct_run_prepared_runtime_from_exact_image_receipt_for_compiler_owner_v1(
    exact_checked_image_settlement_receipt: CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1,
) -> SourceEntrypointDirectRunPreparedRuntime {
    let CompilerExactCheckedCallExecutableImageSettlementReceiptForSourceEntrypointOwnerV1 {
        prepared_source_program_image_authority,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = exact_checked_image_settlement_receipt;
    SourceEntrypointDirectRunPreparedRuntime {
        prepared_source_program_image_authority,
        _module_interface_runtime_custodies: module_interface_runtime_custodies,
        _runtime_artifact_observations: runtime_artifact_observations,
    }
}

pub fn admit_source_entrypoint_direct_run_prepared_runtime_process_start_for_compiler_owner_v1(
    prepared_runtime: SourceEntrypointDirectRunPreparedRuntime,
    run_namespace: String,
    run_suffix: String,
    started_at: String,
    program_args: Vec<String>,
    launch_cwd: Option<String>,
    provider_host_set: swarm_provider_host_set::ProviderHostSet,
) -> Result<
    SourceEntrypointDirectRunPreparedRuntimeProcessStart,
    SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal,
> {
    let launch_values = match crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::admit_process_start_launch_values_for_compiler_owner_v1(
        crate::direct_run::DirectRunProcessStartLaunchValuesForCompilerOwnerV1::new(
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        ),
    ) {
        Ok(launch_values) => launch_values,
        Err(source) => {
            return Err(SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
                custody: SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::LaunchValues {
                    prepared_runtime,
                    provider_host_set,
                    source,
                },
            });
        }
    };
    let SourceEntrypointDirectRunPreparedRuntime {
        prepared_source_program_image_authority,
        _module_interface_runtime_custodies: module_interface_runtime_custodies,
        _runtime_artifact_observations: runtime_artifact_observations,
    } = prepared_runtime;
    let execution_authority = match crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::execution_authority_from_prepared_source_program_image_authority(
        prepared_source_program_image_authority,
    ) {
        Ok(execution_authority) => execution_authority,
        Err((prepared_source_program_image_authority, cause)) => {
            return Err(SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
                custody: SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::PreparedImage {
                    prepared_runtime: SourceEntrypointDirectRunPreparedRuntime {
                        prepared_source_program_image_authority,
                        _module_interface_runtime_custodies: module_interface_runtime_custodies,
                        _runtime_artifact_observations: runtime_artifact_observations,
                    },
                    launch_values,
                    provider_host_set,
                    cause,
                },
            });
        }
    };
    let process_start_drive = crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::public_aperture_process_start_drive_from_execution_authority(
        execution_authority,
    );
    let command = crate::direct_run::DirectRunPreparedRuntimeAuthorityOwner::public_aperture_prepared_runtime_process_start_command(
        process_start_drive,
        launch_values,
    );
    let admission_input =
        crate::direct_run::direct_run_public_aperture_prepared_runtime_process_start_admission_input_v1(
            command,
            provider_host_set.begin_provider_execution_session_v1(),
        );
    match crate::direct_run::admit_direct_run_public_aperture_prepared_runtime_process_start_v1(
        admission_input,
    ) {
        Ok(admitted) => Ok(SourceEntrypointDirectRunPreparedRuntimeProcessStart {
            admitted,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
        }),
        Err(refusal) => Err(
            SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusal {
                custody: SourceEntrypointDirectRunPreparedRuntimeProcessStartAdmissionRefusalCustody::ProviderAdmission {
                    refusal,
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                },
            },
        ),
    }
}

pub fn drive_source_entrypoint_direct_run_prepared_runtime_process_start_until_terminal_for_compiler_owner_v1(
    start: SourceEntrypointDirectRunPreparedRuntimeProcessStart,
) -> Result<
    SourceEntrypointDirectRunTerminalForCompilerOwnerV1,
    SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1,
> {
    let SourceEntrypointDirectRunPreparedRuntimeProcessStart {
        admitted,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = start;
    match crate::direct_run::drive_direct_run_public_aperture_prepared_runtime_process_start_command_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
        admitted,
    ) {
        Ok(output) => Ok(SourceEntrypointDirectRunTerminalForCompilerOwnerV1 {
            output,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
        }),
        Err(fault) => Err(SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
            fault,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
        }),
    }
}

pub fn cancel_source_entrypoint_direct_run_terminal_fault_for_compiler_owner_v1(
    fault: SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1,
) -> Result<
    SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1,
    SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1,
> {
    let SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
        fault,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = fault;
    match fault.cancel_into_generic_message_for_direct_run_boundary_owner_v1() {
        Ok(message) => Ok(
            SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {
                message,
                _module_interface_runtime_custodies: module_interface_runtime_custodies,
                _runtime_artifact_observations: runtime_artifact_observations,
            },
        ),
        Err(fault) => Err(SourceEntrypointDirectRunTerminalFaultForCompilerOwnerV1 {
            fault,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
        }),
    }
}

pub fn settle_source_entrypoint_direct_run_terminal_into_final_observation_for_compiler_owner_v1(
    terminal: SourceEntrypointDirectRunTerminalForCompilerOwnerV1,
) -> Result<serde_json::Value, SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1>
{
    let SourceEntrypointDirectRunTerminalForCompilerOwnerV1 {
        output,
        module_interface_runtime_custodies,
        runtime_artifact_observations,
    } = terminal;
    output
        .into_settled_source_execution_projection_for_source_run_result_owner_v1()
        .map_err(
            |message| SourceEntrypointDirectRunTerminalObservationFaultForCompilerOwnerV1 {
                message,
                _module_interface_runtime_custodies: module_interface_runtime_custodies,
                _runtime_artifact_observations: runtime_artifact_observations,
            },
        )
}

fn prepare_compiler_runtime_program_from_front_pass_products_for_source_entrypoint_owner_v1(
    front_pass_products: SsSourceEntrypointColdMaterializationFrontPassProductsForSourceEntrypointColdMaterializationOwnerV1,
    transaction_root: &mut crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
) -> Result<
    CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    SourceEntrypointDirectRunCompileAdmissionRefusal,
> {
    let (
        runtime_artifact_body_inputs,
        checked_process_lifecycle_argument_evidence,
        protocol_declarations,
        selected_provider_command_evidence_batch,
        direct_run_source_program_authority,
        front_pass_materialization_admission,
        module_interface_runtime_custody,
        capability_requirement_inventory,
    ) = front_pass_products.consume_for_source_entrypoint_direct_run_prepared_runtime_owner_v1();

    prepare_compiler_runtime_program_from_consumed_products_for_source_entrypoint_owner_v1(
        runtime_artifact_body_inputs,
        checked_process_lifecycle_argument_evidence,
        protocol_declarations,
        selected_provider_command_evidence_batch,
        direct_run_source_program_authority,
        front_pass_materialization_admission,
        capability_requirement_inventory,
        vec![module_interface_runtime_custody],
        transaction_root,
    )
}

fn stage_compiler_runtime_program_from_front_pass_products_for_transaction_root_v1(
    selected_child_front_pass: CompilerTransactionSelectedStaticChildFrontPassV1,
    transaction_root: &mut crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
) -> Result<CompilerTransactionProgramWorkFrameV1, CompilerTransactionProgramStagingRefusalV1> {
    let (front_pass_products, transaction_correspondence) =
        selected_child_front_pass.consume_for_compiler_transaction_root_v1();
    let (
        runtime_artifact_body_inputs,
        checked_process_lifecycle_argument_evidence,
        protocol_declarations,
        selected_provider_command_evidence_batch,
        direct_run_source_program_authority,
        front_pass_materialization_admission,
        module_interface_runtime_custody,
        capability_requirement_inventory,
    ) = front_pass_products.consume_for_source_entrypoint_direct_run_prepared_runtime_owner_v1();
    stage_compiler_runtime_program_from_consumed_products_for_transaction_root_v1(
        runtime_artifact_body_inputs,
        checked_process_lifecycle_argument_evidence,
        protocol_declarations,
        selected_provider_command_evidence_batch,
        direct_run_source_program_authority,
        front_pass_materialization_admission,
        capability_requirement_inventory,
        vec![module_interface_runtime_custody],
        transaction_root,
        transaction_correspondence,
    )
}

fn prepare_compiler_runtime_program_from_consumed_products_for_source_entrypoint_owner_v1(
    runtime_artifact_body_inputs: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedRuntimeArtifactBodyInputsForExecutableBodyLoweringOwnerV1,
    checked_process_lifecycle_argument_evidence: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedProcessLifecycleArgumentEvidenceForProcessLifecycleArgumentEvidenceOwnerV1,
    protocol_declarations: crate::protocol_declaration_authority::AdmittedDeclarationTable,
    selected_provider_command_evidence_batch: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerSelectedProviderCommandEvidenceBatchForExecutableImageOwnerV1,
    direct_run_source_program_authority: libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
    front_pass_materialization_admission: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSourceWorkSetFrontPassPreparedRuntimeMaterializationAdmissionForDirectRunOwnerV1,
    capability_requirement_inventory:
        crate::CompilerSettledCapabilityRequirementInventoryForPreparedRuntimeOwnerV1,
    mut module_interface_runtime_custodies: Vec<
        CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1,
    >,
    transaction_root: &mut crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
) -> Result<
    CompilerPreparedRuntimeProgramForSourceEntrypointOwnerV1,
    SourceEntrypointDirectRunCompileAdmissionRefusal,
> {
    let initial_frame =
        match stage_compiler_runtime_program_from_consumed_products_for_transaction_root_v1(
            runtime_artifact_body_inputs,
            checked_process_lifecycle_argument_evidence,
            protocol_declarations,
            selected_provider_command_evidence_batch,
            direct_run_source_program_authority,
            front_pass_materialization_admission,
            capability_requirement_inventory,
            module_interface_runtime_custodies,
            transaction_root,
            CompilerTransactionProgramCorrespondenceV1::Root,
        ) {
            Ok(frame) => frame,
            Err(refusal) => {
                return Err(SourceEntrypointDirectRunCompileAdmissionRefusal {
                    cancellation: refusal.cancellation,
                    module_interface_runtime_custodies: refusal.module_interface_runtime_custodies,
                    runtime_artifact_observations: refusal.runtime_artifact_observations,
                });
            }
        };
    transaction_root.push_program_work_for_compiler_owner_v1(initial_frame);
    loop {
        let frame = transaction_root
            .pop_program_work_for_compiler_owner_v1()
            .expect("the compiler transaction root retains its current program frame");
        match frame.advance_for_compiler_transaction_root_v1() {
            CompilerTransactionProgramWorkActionV1::PrepareChild {
                parent,
                role,
                child_front_pass,
            } => {
                transaction_root.push_program_work_for_compiler_owner_v1(parent);
                debug_assert!(transaction_root.has_program_work_for_compiler_owner_v1());
                let child_front_pass = transaction_root
                    .seal_selected_static_child_front_pass_for_compiler_owner_v1(
                        role,
                        child_front_pass,
                    );
                let child = match stage_compiler_runtime_program_from_front_pass_products_for_transaction_root_v1(
                    child_front_pass,
                    transaction_root,
                ) {
                    Ok(child) => child,
                    Err(refusal) => {
                        debug_assert!(transaction_root.has_program_work_for_compiler_owner_v1());
                        return Err(SourceEntrypointDirectRunCompileAdmissionRefusal {
                            cancellation: refusal.cancellation,
                            module_interface_runtime_custodies:
                                refusal.module_interface_runtime_custodies,
                            runtime_artifact_observations:
                                refusal.runtime_artifact_observations,
                        });
                    }
                };
                transaction_root.push_program_work_for_compiler_owner_v1(child);
            }
            CompilerTransactionProgramWorkActionV1::Complete(prepared_program) => {
                let Some(parent) = transaction_root.pop_program_work_for_compiler_owner_v1() else {
                    debug_assert!(!transaction_root.has_program_work_for_compiler_owner_v1());
                    return Ok(prepared_program);
                };
                match parent
                    .accept_prepared_child_for_compiler_transaction_root_v1(prepared_program)
                {
                    Ok(parent) => {
                        transaction_root.push_program_work_for_compiler_owner_v1(parent);
                    }
                    Err(refusal) => {
                        return Err(SourceEntrypointDirectRunCompileAdmissionRefusal {
                            cancellation: refusal.cancellation,
                            module_interface_runtime_custodies: refusal
                                .module_interface_runtime_custodies,
                            runtime_artifact_observations: refusal.runtime_artifact_observations,
                        });
                    }
                }
            }
        }
    }
}

fn stage_compiler_runtime_program_from_consumed_products_for_transaction_root_v1(
    runtime_artifact_body_inputs: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedRuntimeArtifactBodyInputsForExecutableBodyLoweringOwnerV1,
    checked_process_lifecycle_argument_evidence: crate::source_entrypoint_compiler_admission_session::source_work_set::CheckedProcessLifecycleArgumentEvidenceForProcessLifecycleArgumentEvidenceOwnerV1,
    protocol_declarations: crate::protocol_declaration_authority::AdmittedDeclarationTable,
    selected_provider_command_evidence_batch: crate::source_entrypoint_compiler_admission_session::source_work_set::CompilerSelectedProviderCommandEvidenceBatchForExecutableImageOwnerV1,
    direct_run_source_program_authority: libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
    front_pass_materialization_admission: crate::source_entrypoint_compiler_admission_session::source_work_set::SsSourceWorkSetFrontPassPreparedRuntimeMaterializationAdmissionForDirectRunOwnerV1,
    capability_requirement_inventory:
        crate::CompilerSettledCapabilityRequirementInventoryForPreparedRuntimeOwnerV1,
    module_interface_runtime_custodies: Vec<
        CompilerOwnedModuleInterfaceRuntimeCustodyForCompilerOwnerV1,
    >,
    transaction_root: &mut crate::source_entrypoint_compiler_admission_session::SourceEntrypointCompilerTransactionRootV1,
    transaction_correspondence: CompilerTransactionProgramCorrespondenceV1,
) -> Result<CompilerTransactionProgramWorkFrameV1, CompilerTransactionProgramStagingRefusalV1> {
    let runtime_program_artifact_demands =
        checked_runtime_program_artifact_demands_from_source_work_set_runtime_artifact_body_inputs_for_runtime_program_artifact_owner_v1(
            runtime_artifact_body_inputs,
        );
    let (runtime_program_artifact_demand, boundary_decode_plan_set) =
        runtime_program_artifact_demands.consume_for_runtime_program_artifact_owner_v1();
    let tail = CompilerPreparationTailCustodyForSourceEntrypointOwnerV1 {
        boundary_decode_plan_set,
        direct_run_source_program_authority,
        front_pass_materialization_admission,
        capability_requirement_inventory,
    };
    let mut runtime_artifact_observations =
        vec![SourceEntrypointRuntimeArtifactObservationCustody {
            _protocol_declarations: protocol_declarations,
        }];
    let (selected_executable_image, settled_static_child_work) =
        match runtime_program_artifact_demand
            .consume_with_selected_provider_command_evidence_for_runtime_program_artifact_owner_v1(
                selected_provider_command_evidence_batch,
            ) {
            Ok(products) => products,
            Err(source) => {
                return Err(CompilerTransactionProgramStagingRefusalV1 {
                    cancellation:
                        SourceEntrypointDirectRunCompileAdmissionCancellation::ProviderEffectImage {
                            source,
                            _checked_process_lifecycle_argument_evidence:
                                checked_process_lifecycle_argument_evidence,
                            _tail: tail,
                        },
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                });
            }
        };
    let validated_process_lifecycle_argument_evidence = checked_process_lifecycle_argument_evidence
        .into_validated_process_lifecycle_argument_evidence_for_runtime_program_artifact_owner_v1();
    let (process_lifecycle_payload, process_lifecycle_artifact_sidecar_segment) =
        match validated_process_lifecycle_argument_evidence
            .into_process_lifecycle_payload_carrier_and_artifact_sidecar_segment_for_swarmvm_session_runtime_owner_v1()
        {
            Ok(products) => products,
            Err(source) => {
                return Err(CompilerTransactionProgramStagingRefusalV1 {
                    cancellation:
                        SourceEntrypointDirectRunCompileAdmissionCancellation::ProcessLifecycle {
                            source,
                            _selected_executable_image: selected_executable_image,
                            _settled_static_child_work: settled_static_child_work,
                            _tail: tail,
                        },
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                });
            }
        };
    let prepared_runtime_entry =
        match crate::consume_source_entrypoint_executable_image_into_runtime_owner_v1(
            selected_executable_image,
            process_lifecycle_payload,
        ) {
            Ok(entry) => entry,
            Err(source) => {
                return Err(CompilerTransactionProgramStagingRefusalV1 {
                    cancellation:
                        SourceEntrypointDirectRunCompileAdmissionCancellation::ExecutableImage {
                            source,
                            _settled_static_child_work: settled_static_child_work,
                            _process_lifecycle_artifact_sidecar_segment:
                                process_lifecycle_artifact_sidecar_segment,
                            _tail: tail,
                        },
                    module_interface_runtime_custodies,
                    runtime_artifact_observations,
                });
            }
        };

    let prepared_static_child_work = match settled_static_child_work
        .try_prepare_child_front_pass_inputs_for_runtime_program_artifact_owner_v1(transaction_root)
    {
        Ok(prepared) => prepared,
        Err(source) => {
            return Err(CompilerTransactionProgramStagingRefusalV1 {
                cancellation:
                    SourceEntrypointDirectRunCompileAdmissionCancellation::StaticChildCompilation {
                        source,
                        _prepared_runtime_entry: prepared_runtime_entry,
                        _process_lifecycle_artifact_sidecar_segment:
                            process_lifecycle_artifact_sidecar_segment,
                        _tail: tail,
                    },
                module_interface_runtime_custodies,
                runtime_artifact_observations,
            });
        }
    };
    Ok(
        CompilerTransactionProgramWorkFrameV1::new_for_compiler_transaction_root_v1(
            prepared_runtime_entry,
            process_lifecycle_artifact_sidecar_segment,
            tail,
            prepared_static_child_work,
            module_interface_runtime_custodies,
            runtime_artifact_observations,
            transaction_correspondence,
        ),
    )
}
