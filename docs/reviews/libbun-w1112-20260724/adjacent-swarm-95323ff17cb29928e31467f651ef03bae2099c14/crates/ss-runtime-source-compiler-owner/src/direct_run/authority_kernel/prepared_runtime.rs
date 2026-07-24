use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    sync::{Arc, atomic::AtomicU64},
};

use crate::DirectRunExactStaticChildDispatchInstalledPreparedRuntimeForPreparedRuntimeOwnerV1 as EngineInstalledPreparedSessionRuntimeV1;
use crate::DirectSessionLirCompilerRuntimePreparedProgramForDirectRunOwnerV1;
use crate::SealedPreparedRuntime as EnginePreparedSessionRuntimeV1;
use crate::direct_run::{
    DirectRunAdmittedSourceProgramHandle, DirectRunKernelStartPackageGraphTemplate,
    DirectRunLaunchDurabilityPolicyAuthority, DirectRunLaunchProductStoreBindingAuthority,
    DirectRunPreparedRuntimeStaticChildRuntimeHandles,
    DirectRunPreparedSourceProgramImageInstallTransactionIdentityV1,
    DirectRunPreparedSourceProgramRegistryInstallV1,
    DirectRunPreparedStaticChildModuleRunArtifactReceipt,
    DirectRunPreparedStaticChildModuleRunRuntimeHandle,
    DirectRunPreparedStaticChildSelectedEntryArtifactReceipt,
    DirectRunPreparedStaticChildSelectedEntryRuntimeHandle, DirectRunProcessSessionDriveFaultV1,
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1, DirectRunProgramAdmissionReceipt,
    DirectRunSourceProgramAuthorityOwner, DirectSwarmScriptRunCurrentProcess,
    DirectSwarmScriptRunKernelState, direct_run_runtime_authority_refs, require_non_empty,
    require_run_namespace, require_run_suffix, require_timestamp,
    with_direct_run_thread_local_cell, with_direct_run_thread_local_cell_mut,
};
use crate::direct_run::{
    DirectSwarmScriptRunLinkedCapabilityProduct, DirectSwarmScriptRunPreparedSourceProgram,
    DirectSwarmScriptRunPreparedSourceProgramHandle,
    DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    DirectSwarmScriptRunPreparedSourceProgramMaterializationReceipt,
};
use crate::{
    PreparedStaticChildRuntimeTemplateForDirectRunOwnerV1 as EnginePreparedStaticChildRuntimeTemplateV1,
    ProcessSessionV0 as EngineLiveProcessSessionV1,
    open_process_child_session_v0_from_sealed_prepared_runtime_for_direct_run_owner_v1,
};
use libswarm_package_graph_contract_source_admission::{
    PackageGraphPreparedRuntimeContractTsonDerivationInput,
    PackageGraphRuntimeContractTsonDerivationInput,
    PackageGraphRuntimeContractTsonDerivationInputArtifactSidecarForPackageGraphContractSourceOwner,
};
use libswarm_package_graph_model::{CapabilityContractResolutionRecord, ProviderImportIdentity};
use libswarm_package_graph_provider_requirements::{
    PackageGraphImplementationDeclaration,
    PackageGraphStaticProviderContractForProviderRequirementsOwner,
};
use libswarm_package_graph_source_session::{
    PackageGraphSourceSessionManifestReceiptProduct, PackageGraphSourceSessionReceiptProduct,
};
use prepared_runtime_image_manifest_model::{
    PreparedRuntimeProviderImportAuthoritySet,
    PreparedRuntimeProviderImportExecutionStartAdmissionSet,
};
use serde_json::{Value, json};

include!("prepared_runtime/static_child_module_runs_and_sessions.rs");
include!("prepared_runtime/static_child_selected_entries.rs");

mod image_install_transaction;
pub(crate) use image_install_transaction::DirectRunPreparedSourceProgramImageInstallationRefusalV1;

#[path = "prepared_runtime/artifacts/mod.rs"]
mod registries_prepared_artifacts;

pub(in crate::direct_run) use registries_prepared_artifacts::observe_stored_image_bytes_for_artifact_owner_v1;
pub use registries_prepared_artifacts::{
    DirectRunColdMaterializationEvidenceForPreparedRuntimeOwnerV1,
    direct_run_cold_materialization_evidence_from_front_pass_admission_v1,
};

pub(crate) struct DirectRunPreparedRuntimeAuthorityOwner;

#[must_use = "the sealed module-run template must be consumed by static-child image preparation"]
pub(crate) struct DirectRunPreparedStaticChildModuleRunRuntimeTemplateV1 {
    prepared_runtime_template: EnginePreparedStaticChildRuntimeTemplateV1,
}

#[must_use = "the sealed selected-entry template must be consumed by static-child image preparation"]
pub(crate) struct DirectRunPreparedStaticChildSelectedEntryRuntimeTemplateV1 {
    prepared_runtime_template: EnginePreparedStaticChildRuntimeTemplateV1,
    selected_function_params: Vec<String>,
}

impl DirectRunPreparedStaticChildSelectedEntryRuntimeTemplateV1 {
    pub(in crate::direct_run) fn final_selected_function_params_observation_for_prepared_runtime_artifact_owner_v1(
        &self,
    ) -> Vec<String> {
        self.selected_function_params.clone()
    }
}

impl DirectRunPreparedRuntimeAuthorityOwner {
    pub(crate) fn install_compiler_prepared_source_program_image_for_source_entrypoint_owner_v1(
        direct_run_source_program_authority:
            libswarm_package_graph_executable_program_model::AdmittedEntryExecutableSourceClosureDirectRunSourceProgramAuthority,
        prepared_runtime_program: DirectSessionLirCompilerRuntimePreparedProgramForDirectRunOwnerV1,
        cold_materialization_evidence: DirectRunColdMaterializationEvidenceForPreparedRuntimeOwnerV1,
    ) -> Result<
        DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
        DirectRunPreparedSourceProgramImageInstallationRefusalV1,
    > {
        Self::install_compiler_prepared_source_program_image_v1(
            direct_run_source_program_authority,
            prepared_runtime_program,
            cold_materialization_evidence,
        )
    }

    pub(crate) fn seal_module_run_static_child_runtime_template_for_source_entrypoint_executable_runtime_owner_v1(
        prepared_runtime_template: EnginePreparedStaticChildRuntimeTemplateV1,
    ) -> DirectRunPreparedStaticChildModuleRunRuntimeTemplateV1 {
        DirectRunPreparedStaticChildModuleRunRuntimeTemplateV1 {
            prepared_runtime_template,
        }
    }

    pub(crate) fn seal_selected_entry_static_child_runtime_template_for_source_entrypoint_executable_runtime_owner_v1(
        prepared_runtime_template: EnginePreparedStaticChildRuntimeTemplateV1,
        selected_function_params: Vec<String>,
    ) -> DirectRunPreparedStaticChildSelectedEntryRuntimeTemplateV1 {
        DirectRunPreparedStaticChildSelectedEntryRuntimeTemplateV1 {
            prepared_runtime_template,
            selected_function_params,
        }
    }
}

pub(crate) struct DirectRunPreparedRuntimeImageStartFault {
    message: String,
}

impl DirectRunPreparedRuntimeImageStartFault {
    fn from_registry_message(message: String) -> Self {
        Self { message }
    }

    pub(crate) fn from_process_session_open_fault(
        fault: crate::session::ProcessSessionOpenError,
    ) -> Self {
        Self {
            message: fault.to_string(),
        }
    }
}

impl std::fmt::Debug for DirectRunPreparedRuntimeImageStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DirectRunPreparedRuntimeImageStartFault")
            .field("message", &self.message)
            .finish()
    }
}

impl std::fmt::Display for DirectRunPreparedRuntimeImageStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DirectRunPreparedRuntimeImageStartFault {}

/// The complete, preflighted install of one prepared source-program image.
/// This product owns every registry entry and image/session value until its
/// infallible consuming commit publishes the correlated set exactly once.
#[must_use = "a prepared source-program image install must be consumed by its commit"]
struct DirectRunPreparedSourceProgramImageInstallV1 {
    source_program_commit:
        crate::direct_run::authority_kernel::source_program::DirectRunPreparedSourceProgramRegistryCommitV1,
    prepared_source_program: DirectSwarmScriptRunPreparedSourceProgram,
    _provider_lineage:
        libswarm_package_graph_provider_requirements::PackageGraphProviderLineage,
    _cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner:
        swarmscript_types::BoundaryDecodePlanReDerivationSetForSwarmvmImageOwnerV1,
}

#[must_use = "prepared source-program image staging must be consumed by the install owner"]
pub(in crate::direct_run) struct DirectRunPreparedSourceProgramImageStagingV1 {
    prepared_source_program: DirectSwarmScriptRunPreparedSourceProgram,
    install_transaction_identity: DirectRunPreparedSourceProgramImageInstallTransactionIdentityV1,
    provider_lineage: libswarm_package_graph_provider_requirements::PackageGraphProviderLineage,
    cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner:
        swarmscript_types::BoundaryDecodePlanReDerivationSetForSwarmvmImageOwnerV1,
}

enum DirectRunPreparedSourceProgramImageRegistryStagingRefusalCustodyV1 {
    Unpreflighted {
        source_program_installs: Vec<DirectRunPreparedSourceProgramRegistryInstallV1>,
        image_staging: DirectRunPreparedSourceProgramImageStagingV1,
    },
    PreparedRuntimeImagePreflight {
        source_program_commit:
            crate::direct_run::authority_kernel::source_program::DirectRunPreparedSourceProgramRegistryCommitV1,
        prepared_source_program: DirectSwarmScriptRunPreparedSourceProgram,
        provider_lineage: libswarm_package_graph_provider_requirements::PackageGraphProviderLineage,
        cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner:
            swarmscript_types::BoundaryDecodePlanReDerivationSetForSwarmvmImageOwnerV1,
    },
}

/// A preparation refusal retains every linear value that had been staged.
/// Diagnostics borrow the refusal; only explicit retry or cancellation may
/// consume its private custody, which no registry has observed.
enum DirectRunPreparedSourceProgramImageRegistryStagingFaultV1 {
    SourceProgramRegistryPreflight,
    PreparedRuntimeImageRegistryPreflight,
}

struct DirectRunPreparedSourceProgramImageRegistryStagingRefusalV1 {
    fault: DirectRunPreparedSourceProgramImageRegistryStagingFaultV1,
    custody: DirectRunPreparedSourceProgramImageRegistryStagingRefusalCustodyV1,
}

pub(in crate::direct_run) struct DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority
{
    source: DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionSourceV1,
}

#[must_use = "cancelled process.run registry admission retains the whole prepared execution custody"]
pub(in crate::direct_run) struct DirectRunPreparedStaticChildModuleRunRegistryCommitCancellationV1 {
    _execution: DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority,
}

enum DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionSourceV1 {
    ExactProcessRun {
        open_plan: crate::direct_run::DirectRunAdmittedProcessRunExactStaticChildOpenPlanV1,
        program_input: swarm_provider_value_model::ProviderValue,
        options: swarm_provider_value_model::ProviderValue,
    },
}

/// One loaded Process lifecycle's exact, one-shot activation execution.
///
/// The selected loaded Process remains in this authority while its duplicated
/// session-open identity and root input enter the child-open transaction. That
/// lets every refusal retain the lifecycle identity without reconstructing it
/// from raw process fields.
#[must_use = "a prepared process.activate execution must open, remain in refusal custody, or be cancelled"]
pub(in crate::direct_run) struct DirectRunPreparedProcessActivateExactStaticChildExecutionAuthorityV1
{
    reusable_site_plan:
        Arc<crate::direct_run::DirectRunExactStaticChildProcessLoadReusableSitePlanV1>,
    prepared_runtime: Arc<EnginePreparedSessionRuntimeV1>,
    initial_input: crate::ProcessSessionInitialInputForDirectRunOwnerV1,
    process_identity: crate::ProcessSessionProcessIdentityForDirectRunLaunchOwnerV1,
    admitted_child_scope:
        crate::session::AdmittedExactStaticChildCapabilityScopeForChildSessionOpenOwnerV1,
    loaded_process: DirectSwarmScriptRunCurrentProcess,
}

/// Preparation refusal before the admitted exact child scope has entered the
/// child-open construction. Retry or cancellation therefore retains the exact
/// original lifecycle-ready pair unchanged.
#[must_use = "a process.activate preparation refusal retains lifecycle-ready custody"]
pub(in crate::direct_run) struct DirectRunProcessActivateExactStaticChildPreparationRefusalV1 {
    open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    loaded_process: DirectSwarmScriptRunCurrentProcess,
    fault: DirectRunPreparedRuntimeImageStartFault,
}

/// Explicit cancellation before child-open transition. The lifecycle owner
/// may consume this once to restore its Ready row; no raw selector escapes.
#[must_use = "cancelled process.activate preparation retains the exact ready lifecycle pair"]
pub(in crate::direct_run) struct DirectRunCancelledProcessActivateLifecycleOpenV1 {
    open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    loaded_process: DirectSwarmScriptRunCurrentProcess,
    _fault: DirectRunPreparedRuntimeImageStartFault,
}

/// Explicit cancellation after preparation and before registry/open commit.
/// The fresh runtime/input staging is torn down while the exact admitted plan
/// and loaded Process return to lifecycle Ready custody.
#[must_use = "cancelled prepared process.activate execution retains the exact ready lifecycle pair"]
pub(in crate::direct_run) struct DirectRunCancelledPreparedProcessActivateLifecycleOpenV1 {
    open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
    loaded_process: DirectSwarmScriptRunCurrentProcess,
}

/// Retry custody after the admitted child scope has entered session-open
/// preflight. The scope stays inside the lower refusal; the reusable site and
/// loaded Process remain beside it as the transitioned lifecycle state.
#[must_use = "a process.activate open refusal must retry, remain in lifecycle custody, or cancel"]
pub(in crate::direct_run) struct DirectRunProcessActivateExactStaticChildOpenRefusalV1 {
    reusable_site_plan:
        Arc<crate::direct_run::DirectRunExactStaticChildProcessLoadReusableSitePlanV1>,
    loaded_process: DirectSwarmScriptRunCurrentProcess,
    refusal: crate::session::ProcessChildSessionOpenRefusalForDirectRunOwnerV1,
}

/// Terminal teardown after an activation open crossed into lower preflight.
/// This is deliberately distinct from Ready cancellation: the one-shot scope
/// was consumed by the lower construction and cannot be reminted.
#[must_use = "cancelled transitioned process.activate custody must settle at the lifecycle owner"]
pub(in crate::direct_run) struct DirectRunCancelledTransitionedProcessActivateLifecycleOpenV1 {
    _reusable_site_plan:
        Arc<crate::direct_run::DirectRunExactStaticChildProcessLoadReusableSitePlanV1>,
    _loaded_process: DirectSwarmScriptRunCurrentProcess,
    _fault: crate::ProcessSessionOpenError,
}

pub(in crate::direct_run) struct DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority
{
    open_plan: crate::direct_run::DirectRunAdmittedProcessInvokeExactStaticChildOpenPlanV1,
    callable_input: swarm_provider_value_model::ProviderValue,
    input: swarm_provider_value_model::ProviderValue,
    options: swarm_provider_value_model::ProviderValue,
}

#[must_use = "cancelled process.invoke registry admission retains the whole prepared execution custody"]
pub(in crate::direct_run) struct DirectRunPreparedStaticChildSelectedEntryRegistryCommitCancellationV1
{
    _execution: DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority,
}

impl DirectSwarmScriptRunPreparedStaticChildSelectedEntryExecutionAuthority {
    pub(in crate::direct_run) fn from_admitted_process_invoke_launch_for_process_kernel_owner_v1(
        open_plan: crate::direct_run::DirectRunAdmittedProcessInvokeExactStaticChildOpenPlanV1,
        callable_input: swarm_provider_value_model::ProviderValue,
        input: swarm_provider_value_model::ProviderValue,
        options: swarm_provider_value_model::ProviderValue,
    ) -> Self {
        Self {
            open_plan,
            callable_input,
            input,
            options,
        }
    }

    pub(in crate::direct_run) fn open_child_session_for_process_invoke_owner_v1(
        self,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<EngineLiveProcessSessionV1, String> {
        let Self {
            open_plan,
            callable_input,
            input,
            options,
        } = self;
        match open_plan.open_child_session_for_process_invoke_owner_v1(input, current_process) {
            Ok(session) => {
                let _consumed_authored_invocation = (callable_input, options);
                Ok(session)
            }
            Err(refusal) => {
                let _cancellation = refusal.cancel_for_process_invoke_owner_v1();
                let _consumed_authored_invocation = (callable_input, options);
                Err("process.invoke exact child open was explicitly cancelled after retaining refusal".to_owned())
            }
        }
    }

    pub(in crate::direct_run) fn cancel_before_process_invoke_registry_commit_for_process_kernel_owner_v1(
        self,
    ) -> DirectRunPreparedStaticChildSelectedEntryRegistryCommitCancellationV1 {
        DirectRunPreparedStaticChildSelectedEntryRegistryCommitCancellationV1 { _execution: self }
    }
}

impl DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionAuthority {
    pub(in crate::direct_run) fn from_admitted_process_run_launch_for_process_kernel_owner_v1(
        open_plan: crate::direct_run::DirectRunAdmittedProcessRunExactStaticChildOpenPlanV1,
        program_input: swarm_provider_value_model::ProviderValue,
        options: swarm_provider_value_model::ProviderValue,
    ) -> Self {
        Self {
            source:
                DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionSourceV1::ExactProcessRun {
                    open_plan,
                    program_input,
                    options,
                },
        }
    }

    pub(in crate::direct_run) fn open_child_session_for_process_run_owner_v1(
        self,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<EngineLiveProcessSessionV1, String> {
        match self.source {
            DirectSwarmScriptRunPreparedStaticChildModuleRunExecutionSourceV1::ExactProcessRun {
                open_plan,
                program_input,
                options,
            } => match open_plan
                .open_child_session_for_process_run_owner_v1(options, current_process)
            {
                Ok(session) => {
                    let _consumed_authored_program = program_input;
                    Ok(session)
                }
                Err(refusal) => {
                    let _cancellation = refusal.cancel_for_process_run_owner_v1();
                    let _consumed_authored_program = program_input;
                    Err("process.run exact child open was explicitly cancelled after retaining refusal".to_owned())
                }
            },
        }
    }

    pub(in crate::direct_run) fn cancel_before_process_run_registry_commit_for_process_kernel_owner_v1(
        self,
    ) -> DirectRunPreparedStaticChildModuleRunRegistryCommitCancellationV1 {
        DirectRunPreparedStaticChildModuleRunRegistryCommitCancellationV1 { _execution: self }
    }
}

impl DirectRunPreparedProcessActivateExactStaticChildExecutionAuthorityV1 {
    pub(in crate::direct_run) fn from_process_activate_loaded_lifecycle_for_process_kernel_owner_v1(
        open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        loaded_process: DirectSwarmScriptRunCurrentProcess,
    ) -> Result<Self, DirectRunProcessActivateExactStaticChildPreparationRefusalV1> {
        let preparation_refusal = |open_plan, loaded_process, message| {
            DirectRunProcessActivateExactStaticChildPreparationRefusalV1 {
                open_plan,
                loaded_process,
                fault: DirectRunPreparedRuntimeImageStartFault::from_registry_message(message),
            }
        };
        let current_process_root = match crate::direct_run::direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::static_child_current_process_root_input_authority(&loaded_process) {
            Ok(current_process_root) => current_process_root,
            Err(message) => {
                return Err(preparation_refusal(open_plan, loaded_process, message));
            }
        };
        let process_identity =
            match loaded_process.process_session_identity_for_session_runtime_open_owner_v1() {
                Ok(process_identity) => process_identity,
                Err(message) => {
                    return Err(preparation_refusal(open_plan, loaded_process, message));
                }
            };
        let module_run_ticket = match load_direct_run_prepared_static_child_module_run(
            &open_plan
                .reusable_site_plan
                .staged_child_runtime_handle
                .live_entry,
        ) {
            Ok(module_run_ticket) => module_run_ticket,
            Err(message) => {
                return Err(preparation_refusal(open_plan, loaded_process, message));
            }
        };
        let DirectRunPreparedStaticChildModuleRunInvocationTicket {
            prepared_runtime,
            program_admission_root_input,
        } = module_run_ticket;
        let root_input = BTreeMap::from([
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_ARGS_FIELD.to_owned(),
                swarmvm_runtime_types::process_runtime_args_boundary_value_for_swarmvm_session_runtime_owner_v1(Vec::new()),
            ),
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_CURRENT_PROCESS_FIELD
                    .to_owned(),
                current_process_root,
            ),
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_PROGRAM_ADMISSION_FIELD
                    .to_owned(),
                program_admission_root_input,
            ),
        ]);
        let crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1 {
            reusable_site_plan,
            admitted_child_scope,
        } = open_plan;
        Ok(Self {
            reusable_site_plan,
            prepared_runtime: Arc::new(prepared_runtime),
            initial_input:
                crate::ProcessSessionInitialInputForDirectRunOwnerV1::from_root_input_for_direct_run_owner_v1(
                    root_input,
                ),
            process_identity,
            admitted_child_scope,
            loaded_process,
        })
    }

    pub(in crate::direct_run) fn open_child_session_for_process_activate_owner_v1(
        self,
    ) -> Result<EngineLiveProcessSessionV1, DirectRunProcessActivateExactStaticChildOpenRefusalV1>
    {
        let Self {
            reusable_site_plan,
            prepared_runtime,
            initial_input,
            process_identity,
            admitted_child_scope,
            loaded_process,
        } = self;
        open_process_child_session_v0_from_sealed_prepared_runtime_for_direct_run_owner_v1(
            prepared_runtime,
            initial_input,
            swarmvm_session_runtime_model::ProcessSessionDurabilityPolicyV0::default(),
            process_identity,
            admitted_child_scope,
        )
        .map_err(
            |refusal| DirectRunProcessActivateExactStaticChildOpenRefusalV1 {
                reusable_site_plan,
                loaded_process,
                refusal,
            },
        )
    }

    pub(in crate::direct_run) fn cancel_before_process_activate_registry_commit_for_process_lifecycle_owner_v1(
        self,
    ) -> DirectRunCancelledPreparedProcessActivateLifecycleOpenV1 {
        let Self {
            reusable_site_plan,
            prepared_runtime: _,
            initial_input: _,
            process_identity: _,
            admitted_child_scope,
            loaded_process,
        } = self;
        DirectRunCancelledPreparedProcessActivateLifecycleOpenV1 {
            open_plan: crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1 {
                reusable_site_plan,
                admitted_child_scope,
            },
            loaded_process,
        }
    }
}

impl DirectRunProcessActivateExactStaticChildPreparationRefusalV1 {
    pub(in crate::direct_run) fn retry_for_process_activate_owner_v1(
        self,
    ) -> Result<DirectRunPreparedProcessActivateExactStaticChildExecutionAuthorityV1, Self> {
        let Self {
            open_plan,
            loaded_process,
            fault: _,
        } = self;
        DirectRunPreparedProcessActivateExactStaticChildExecutionAuthorityV1::from_process_activate_loaded_lifecycle_for_process_kernel_owner_v1(
            open_plan,
            loaded_process,
        )
    }

    pub(in crate::direct_run) fn cancel_for_process_lifecycle_owner_v1(
        self,
    ) -> DirectRunCancelledProcessActivateLifecycleOpenV1 {
        let Self {
            open_plan,
            loaded_process,
            fault,
        } = self;
        DirectRunCancelledProcessActivateLifecycleOpenV1 {
            open_plan,
            loaded_process,
            _fault: fault,
        }
    }
}

impl DirectRunCancelledProcessActivateLifecycleOpenV1 {
    pub(in crate::direct_run) fn into_ready_lifecycle_custody_for_process_lifecycle_owner_v1(
        self,
    ) -> (
        crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        DirectSwarmScriptRunCurrentProcess,
    ) {
        (self.open_plan, self.loaded_process)
    }
}

impl DirectRunCancelledPreparedProcessActivateLifecycleOpenV1 {
    pub(in crate::direct_run) fn into_ready_lifecycle_custody_for_process_lifecycle_owner_v1(
        self,
    ) -> (
        crate::direct_run::DirectRunAdmittedProcessLoadExactStaticChildOpenPlanV1,
        DirectSwarmScriptRunCurrentProcess,
    ) {
        (self.open_plan, self.loaded_process)
    }
}

impl DirectRunProcessActivateExactStaticChildOpenRefusalV1 {
    pub(in crate::direct_run) fn retry_for_process_activate_owner_v1(
        self,
    ) -> Result<EngineLiveProcessSessionV1, Self> {
        let Self {
            reusable_site_plan,
            loaded_process,
            refusal,
        } = self;
        refusal
            .retry_for_direct_run_child_open_owner_v1()
            .map_err(|refusal| Self {
                reusable_site_plan,
                loaded_process,
                refusal,
            })
    }

    pub(in crate::direct_run) fn cancel_for_process_lifecycle_owner_v1(
        self,
    ) -> DirectRunCancelledTransitionedProcessActivateLifecycleOpenV1 {
        let Self {
            reusable_site_plan,
            loaded_process,
            refusal,
        } = self;
        DirectRunCancelledTransitionedProcessActivateLifecycleOpenV1 {
            _reusable_site_plan: reusable_site_plan,
            _loaded_process: loaded_process,
            _fault: refusal.cancel_for_direct_run_child_open_owner_v1(),
        }
    }
}

impl DirectRunCancelledTransitionedProcessActivateLifecycleOpenV1 {
    pub(in crate::direct_run) fn into_fault_after_process_lifecycle_teardown_owner_v1(
        self,
    ) -> crate::ProcessSessionOpenError {
        self._fault
    }
}

impl crate::direct_run::DirectRunAdmittedProcessRunExactStaticChildOpenPlanV1 {
    pub(in crate::direct_run) fn open_child_session_for_process_run_owner_v1(
        self,
        options: swarm_provider_value_model::ProviderValue,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<
        EngineLiveProcessSessionV1,
        crate::direct_run::DirectRunProcessRunExactStaticChildOpenRefusalV1,
    > {
        let preparation_fault = |open_plan, options, message| {
            crate::direct_run::DirectRunProcessRunExactStaticChildOpenRefusalV1::Preparation {
                open_plan,
                options,
                fault: DirectRunPreparedRuntimeImageStartFault::from_registry_message(message),
            }
        };
        let module_run_ticket = match load_direct_run_prepared_static_child_module_run(
            &self
                .reusable_site_plan
                .staged_child_runtime_handle
                .live_entry,
        ) {
            Ok(ticket) => ticket,
            Err(message) => return Err(preparation_fault(self, options, message)),
        };
        let args_option = match &options {
            swarm_provider_value_model::ProviderValue::Object(options) => options.get("args"),
            swarm_provider_value_model::ProviderValue::Null => None,
            _ => {
                return Err(preparation_fault(
                    self,
                    options,
                    "process.run child options must be an admitted object or null".to_owned(),
                ));
            }
        };
        let args = match args_option {
            None | Some(swarm_provider_value_model::ProviderValue::Null) => Vec::new(),
            Some(swarm_provider_value_model::ProviderValue::Array(args)) => {
                let mut admitted = Vec::with_capacity(args.len());
                for value in args {
                    let swarm_provider_value_model::ProviderValue::String(value) = value else {
                        return Err(preparation_fault(
                            self,
                            options,
                            "process.run args must contain only strings".to_owned(),
                        ));
                    };
                    admitted.push(value.clone());
                }
                admitted
            }
            Some(_) => {
                return Err(preparation_fault(
                    self,
                    options,
                    "process.run args must be an array of strings".to_owned(),
                ));
            }
        };
        let current_process_root = match crate::direct_run::direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::static_child_current_process_root_input_authority(current_process) {
            Ok(current_process_root) => current_process_root,
            Err(message) => return Err(preparation_fault(self, options, message)),
        };
        let process_identity =
            match current_process.process_session_identity_for_session_runtime_open_owner_v1() {
                Ok(process_identity) => process_identity,
                Err(message) => return Err(preparation_fault(self, options, message)),
            };
        let DirectRunPreparedStaticChildModuleRunInvocationTicket {
            prepared_runtime,
            program_admission_root_input,
        } = module_run_ticket;
        let root_input = BTreeMap::from([
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_ARGS_FIELD.to_owned(),
                swarmvm_runtime_types::process_runtime_args_boundary_value_for_swarmvm_session_runtime_owner_v1(args),
            ),
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_CURRENT_PROCESS_FIELD.to_owned(),
                current_process_root,
            ),
            (
                swarmvm_host_abi::process_runtime::PROCESS_RUNTIME_PARAM_PROGRAM_ADMISSION_FIELD.to_owned(),
                program_admission_root_input,
            ),
        ]);
        let Self {
            reusable_site_plan,
            admitted_child_scope,
        } = self;
        crate::session::open_process_child_session_v0_from_sealed_prepared_runtime_for_direct_run_owner_v1(
            Arc::new(prepared_runtime),
            crate::ProcessSessionInitialInputForDirectRunOwnerV1::from_root_input_for_direct_run_owner_v1(root_input),
            swarmvm_session_runtime_model::ProcessSessionDurabilityPolicyV0::default(),
            process_identity,
            admitted_child_scope,
        )
        .map_err(|refusal| {
            crate::direct_run::DirectRunProcessRunExactStaticChildOpenRefusalV1::SessionOpen {
                reusable_site_plan,
                refusal,
            }
        })
    }
}

impl crate::direct_run::DirectRunProcessRunExactStaticChildOpenRefusalV1 {
    pub(in crate::direct_run) fn retry_for_process_run_owner_v1(
        self,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<EngineLiveProcessSessionV1, Self> {
        match self {
            Self::Preparation {
                open_plan,
                options,
                fault: _,
            } => open_plan.open_child_session_for_process_run_owner_v1(options, current_process),
            Self::SessionOpen {
                reusable_site_plan,
                refusal,
            } => refusal
                .retry_for_direct_run_child_open_owner_v1()
                .map_err(|refusal| Self::SessionOpen {
                    reusable_site_plan,
                    refusal,
                }),
        }
    }

    pub(in crate::direct_run) fn cancel_for_process_run_owner_v1(
        self,
    ) -> crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1 {
        match self {
            Self::Preparation { fault, .. } => {
                crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1::Preparation(
                    fault,
                )
            }
            Self::SessionOpen { refusal, .. } => {
                crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1::SessionOpen(
                    refusal.cancel_for_direct_run_child_open_owner_v1(),
                )
            }
        }
    }
}

impl crate::direct_run::DirectRunAdmittedProcessInvokeExactStaticChildOpenPlanV1 {
    pub(in crate::direct_run) fn open_child_session_for_process_invoke_owner_v1(
        self,
        input: swarm_provider_value_model::ProviderValue,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<
        EngineLiveProcessSessionV1,
        crate::direct_run::DirectRunProcessInvokeExactStaticChildOpenRefusalV1,
    > {
        let preparation_fault = |open_plan, input, message| {
            crate::direct_run::DirectRunProcessInvokeExactStaticChildOpenRefusalV1::Preparation {
                open_plan,
                input,
                fault: DirectRunPreparedRuntimeImageStartFault::from_registry_message(message),
            }
        };
        let selected_entry_ticket = match load_direct_run_prepared_static_child_selected_entry(
            &self
                .reusable_site_plan
                .staged_child_runtime_handle
                .live_entry,
        ) {
            Ok(ticket) => ticket,
            Err(message) => return Err(preparation_fault(self, input, message)),
        };
        if selected_entry_ticket.selected_function_params.len() != 1 {
            return Err(preparation_fault(
                self,
                input,
                "process.invoke exact child entry must accept one authored input".to_owned(),
            ));
        }
        let selected_parameter_name = selected_entry_ticket.selected_function_params[0].clone();
        let process_identity =
            match current_process.process_session_identity_for_session_runtime_open_owner_v1() {
                Ok(process_identity) => process_identity,
                Err(message) => return Err(preparation_fault(self, input, message)),
            };
        let initial_input = match crate::ProcessSessionInitialInputForDirectRunOwnerV1::from_selected_entry_single_parameter_for_process_invoke_owner_v1(
            selected_parameter_name,
            input,
        ) {
            Ok(initial_input) => initial_input,
            Err(refusal) => {
                return Err(refusal
                    .settle_into_exact_static_child_open_refusal_for_direct_run_owner_v1(self));
            }
        };
        let DirectRunPreparedStaticChildSelectedEntryInvocationTicket {
            prepared_runtime,
            selected_function_params: _,
        } = selected_entry_ticket;
        let Self {
            reusable_site_plan,
            admitted_child_scope,
        } = self;
        crate::session::open_process_child_session_v0_from_sealed_prepared_runtime_for_direct_run_owner_v1(
            Arc::new(prepared_runtime),
            initial_input,
            swarmvm_session_runtime_model::ProcessSessionDurabilityPolicyV0::default(),
            process_identity,
            admitted_child_scope,
        )
        .map_err(|refusal| {
            crate::direct_run::DirectRunProcessInvokeExactStaticChildOpenRefusalV1::SessionOpen {
                reusable_site_plan,
                refusal,
            }
        })
    }
}

impl crate::direct_run::DirectRunProcessInvokeExactStaticChildOpenRefusalV1 {
    pub(in crate::direct_run) fn retry_for_process_invoke_owner_v1(
        self,
        current_process: &DirectSwarmScriptRunCurrentProcess,
    ) -> Result<EngineLiveProcessSessionV1, Self> {
        match self {
            Self::Preparation {
                open_plan,
                input,
                fault: _,
            } => open_plan.open_child_session_for_process_invoke_owner_v1(input, current_process),
            Self::SessionOpen {
                reusable_site_plan,
                refusal,
            } => refusal
                .retry_for_direct_run_child_open_owner_v1()
                .map_err(|refusal| Self::SessionOpen {
                    reusable_site_plan,
                    refusal,
                }),
        }
    }

    pub(in crate::direct_run) fn cancel_for_process_invoke_owner_v1(
        self,
    ) -> crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1 {
        match self {
            Self::Preparation { fault, .. } => {
                crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1::Preparation(
                    fault,
                )
            }
            Self::SessionOpen { refusal, .. } => {
                crate::direct_run::DirectRunExactStaticChildOpenCancellationFaultV1::SessionOpen(
                    refusal.cancel_for_direct_run_child_open_owner_v1(),
                )
            }
        }
    }
}

pub(in crate::direct_run::authority_kernel::prepared_runtime) struct DirectRunPreparedRuntimeImageStartParts {
    prepared_source_program_ref: String,
    source_path: String,
    entry_executable_source_closure_fingerprint: Option<String>,
    package_resolution_roots: Vec<String>,
    package_graph_session_receipt: PackageGraphSourceSessionReceiptProduct,
    package_graph_session_manifest_receipt: PackageGraphSourceSessionManifestReceiptProduct,
    package_graph_implementation_declarations: Vec<PackageGraphImplementationDeclaration>,
    package_graph_static_provider_contracts:
        Vec<PackageGraphStaticProviderContractForProviderRequirementsOwner>,
    package_graph_contract_resolution_records: Vec<CapabilityContractResolutionRecord>,
    package_graph_prepared_runtime_contract_tson_derivation_inputs:
        Vec<PackageGraphPreparedRuntimeContractTsonDerivationInput>,
    package_graph_runtime_contract_tson_derivation_inputs:
        Vec<PackageGraphRuntimeContractTsonDerivationInput>,
    package_graph_runtime_contract_tson_derivation_input_sidecar:
        PackageGraphRuntimeContractTsonDerivationInputArtifactSidecarForPackageGraphContractSourceOwner,
    package_graph_provider_import_records: BTreeMap<String, ProviderImportIdentity>,
    prepared_runtime_executable_image: EngineInstalledPreparedSessionRuntimeV1,
    program_admission: DirectRunProgramAdmissionReceipt,
    has_process_lifecycle_payload_carrier: bool,
    capability_provider_import_identities:
        Option<PreparedRuntimeProviderImportExecutionStartAdmissionSet>,
    rust_internal_capability_bindings: Vec<DirectSwarmScriptRunLinkedCapabilityProduct>,
    admitted_static_child_source_programs: Vec<DirectRunAdmittedSourceProgramHandle>,
    prepared_static_child_runtime_handles: DirectRunPreparedRuntimeStaticChildRuntimeHandles,
}

pub(crate) struct DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority {
    inner: DirectRunPreparedRuntimeImageStartParts,
}

pub(crate) struct DirectSwarmScriptRunPublicApertureProcessStartDrive {
    inner: DirectRunPreparedRuntimeImageStartParts,
}

pub(crate) struct DirectRunProcessStartLaunchValuesForCompilerOwnerV1 {
    run_namespace: String,
    run_suffix: String,
    started_at: String,
    program_args: Vec<String>,
    launch_cwd: Option<String>,
}

pub(crate) struct DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1 {
    _values: DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
    message: String,
}

impl std::fmt::Display
    for DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl DirectRunProcessStartLaunchValuesForCompilerOwnerV1 {
    pub(crate) fn new(
        run_namespace: String,
        run_suffix: String,
        started_at: String,
        program_args: Vec<String>,
        launch_cwd: Option<String>,
    ) -> Self {
        Self {
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        }
    }
}

pub(crate) struct DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand {
    process_start_drive: DirectSwarmScriptRunPublicApertureProcessStartDrive,
    launch_values: DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
}

#[must_use = "prepared-runtime process-start admission input must be atomically admitted or retained by its typed refusal"]
pub(crate) struct DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1 {
    command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
    provider_execution_session: swarm_provider_host_set::ProviderHostExecutionSession,
}

#[must_use = "an admitted prepared-runtime process start must be driven by the public-aperture runtime owner"]
pub(crate) struct AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1 {
    command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
    provider_execution_session: swarm_provider_host_set::ProviderHostExecutionSession,
}

#[must_use = "a refused prepared-runtime process-start admission retains the complete start command custody beside the host-set refusal"]
pub(crate) struct DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1 {
    command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
    host_refusal: swarm_provider_host_set::ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1,
}

struct DirectRunPublicAperturePreparedRuntimeProcessStartCommandParts {
    start_parts: DirectRunPreparedRuntimeImageStartParts,
    run_namespace: String,
    run_suffix: String,
    started_at: String,
    program_args: Vec<String>,
    launch_cwd: Option<String>,
}

struct DirectRunPreparedRuntimeProcessStartKernelInputs {
    package_graph_template: DirectRunKernelStartPackageGraphTemplate,
    prepared_source_program_ref: String,
    program_admission: DirectRunProgramAdmissionReceipt,
    has_process_lifecycle_payload_carrier: bool,
    capability_link_bindings: Vec<DirectSwarmScriptRunLinkedCapabilityProduct>,
    admitted_static_child_source_programs: Vec<DirectRunAdmittedSourceProgramHandle>,
    prepared_static_child_runtime_handles: DirectRunPreparedRuntimeStaticChildRuntimeHandles,
    run_namespace: String,
    run_suffix: String,
    started_at: String,
    program_args: Vec<String>,
    launch_cwd: Option<String>,
}

impl DirectRunPreparedRuntimeProcessStartKernelInputs {
    fn into_kernel_state_for_direct_run_runtime_authority_owner_v1(
        self,
    ) -> Result<DirectSwarmScriptRunKernelState, String> {
        let Self {
            package_graph_template,
            prepared_source_program_ref,
            program_admission,
            has_process_lifecycle_payload_carrier,
            capability_link_bindings,
            admitted_static_child_source_programs,
            prepared_static_child_runtime_handles,
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        } = self;
        let current_process = Some(
            DirectSwarmScriptRunCurrentProcess::admitted_for_direct_run_root_process_owner_v1(
                direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::process_id_projection(
                    &run_namespace,
                    &run_suffix,
                ),
                direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::root_scope_id_projection(
                    &run_namespace,
                    &run_suffix,
                ),
                direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::host_id_projection(
                    &run_namespace,
                    &run_suffix,
                ),
                program_admission.program_id().to_owned(),
            )?,
        );
        let mut kernel_state =
            crate::direct_run::direct_run_kernel_state_from_package_graph_template_with_launch_durability_policy_and_store_binding(
                run_namespace,
                run_suffix,
                DirectRunLaunchDurabilityPolicyAuthority::Volatile,
                package_graph_template,
                started_at,
                current_process,
                None,
                program_args,
                launch_cwd,
                DirectRunLaunchProductStoreBindingAuthority::bind_fresh_product_stores(),
            )?;
        kernel_state.prepared_source_program_execution_ref = Some(prepared_source_program_ref);
        kernel_state.program_admission = Some(program_admission);
        kernel_state.has_process_lifecycle_payload_carrier = has_process_lifecycle_payload_carrier;
        kernel_state.capability_link_bindings = capability_link_bindings;
        kernel_state.admitted_static_child_source_programs = admitted_static_child_source_programs;
        kernel_state.prepared_static_child_runtime_handles = prepared_static_child_runtime_handles;
        Ok(kernel_state)
    }
}

pub(in crate::direct_run) struct DirectRunPublicAperturePreparedRuntimeProcessStartProduct {
    kernel_inputs: DirectRunPreparedRuntimeProcessStartKernelInputs,
    prepared_runtime_executable_image: EngineInstalledPreparedSessionRuntimeV1,
}

impl DirectRunPublicAperturePreparedRuntimeProcessStartProduct {
    pub(in crate::direct_run) fn into_kernel_state_and_session_open_inputs_for_direct_run_runtime_authority_owner_v1(
        self,
    ) -> Result<
        (
            DirectSwarmScriptRunKernelState,
            EngineInstalledPreparedSessionRuntimeV1,
        ),
        String,
    > {
        let Self {
            kernel_inputs,
            prepared_runtime_executable_image,
        } = self;
        let kernel_state =
            kernel_inputs.into_kernel_state_for_direct_run_runtime_authority_owner_v1()?;
        Ok((kernel_state, prepared_runtime_executable_image))
    }
}

impl DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority {
    fn new(inner: DirectRunPreparedRuntimeImageStartParts) -> Self {
        Self { inner }
    }

    pub fn into_public_aperture_process_start_drive_for_direct_run_public_aperture_owner_v1(
        self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> DirectSwarmScriptRunPublicApertureProcessStartDrive {
        let _ = self;
        match poison {}
    }
}

impl DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand {
    pub(in crate::direct_run) fn into_start_product_for_direct_run_public_aperture_owner_v1(
        self,
    ) -> Result<DirectRunPublicAperturePreparedRuntimeProcessStartProduct, String> {
        let Self {
            process_start_drive,
            launch_values,
        } = self;
        let DirectSwarmScriptRunPublicApertureProcessStartDrive { inner: start_parts } =
            process_start_drive;
        let DirectRunProcessStartLaunchValuesForCompilerOwnerV1 {
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        } = launch_values;
        DirectRunPublicAperturePreparedRuntimeProcessStartCommandParts {
            start_parts,
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        }
        .into_start_product_for_direct_run_public_aperture_owner_v1()
    }
}

impl DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1 {
    pub(in crate::direct_run) fn new_for_direct_run_public_aperture_owner_v1(
        command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
        provider_execution_session: swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Self {
        Self {
            command,
            provider_execution_session,
        }
    }

    pub(in crate::direct_run) fn admit_for_direct_run_public_aperture_owner_v1(
        self,
    ) -> Result<
        AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1,
        DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
    > {
        let Self {
            mut command,
            provider_execution_session,
        } = self;
        let Some(provider_imports) = command
            .process_start_drive
            .inner
            .capability_provider_import_identities
            .take()
        else {
            return Ok(
                AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1 {
                    command,
                    provider_execution_session,
                },
            );
        };
        match provider_execution_session
            .admit_prepared_runtime_provider_import_execution_start_for_direct_run_owner_v1(
                provider_imports,
            ) {
            Ok(provider_execution_session) => Ok(
                AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1 {
                    command,
                    provider_execution_session,
                },
            ),
            Err(host_refusal) => Err(
                DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1 {
                    command,
                    host_refusal,
                },
            ),
        }
    }
}

impl std::fmt::Debug
    for DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(
            "DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1",
        )
        .field("host_refusal", &self.host_refusal)
        .field("hidden_process_start_command", &"redacted")
        .finish()
    }
}

impl std::fmt::Display
    for DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "prepared-runtime process-start admission refused: {}",
            self.host_refusal
        )
    }
}

impl std::error::Error
    for DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1
{
}

impl AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1 {
    pub(in crate::direct_run) fn drive_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
        self,
    ) -> Result<
        DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
        DirectRunProcessSessionDriveFaultV1,
    > {
        let Self {
            command,
            mut provider_execution_session,
        } = self;
        direct_run_runtime_authority_refs::DirectRunRuntimeAuthorityOwner::drive_prepared_runtime_process_start_command_public_aperture_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
            command,
            &mut provider_execution_session,
        )
    }
}

impl DirectRunPublicAperturePreparedRuntimeProcessStartCommandParts {
    pub(in crate::direct_run) fn into_start_product_for_direct_run_public_aperture_owner_v1(
        self,
    ) -> Result<DirectRunPublicAperturePreparedRuntimeProcessStartProduct, String> {
        let Self {
            start_parts,
            run_namespace,
            run_suffix,
            started_at,
            program_args,
            launch_cwd,
        } = self;
        let DirectRunPreparedRuntimeImageStartParts {
            prepared_runtime_executable_image,
            prepared_source_program_ref,
            source_path,
            entry_executable_source_closure_fingerprint,
            package_resolution_roots,
            package_graph_session_receipt,
            package_graph_session_manifest_receipt,
            package_graph_implementation_declarations,
            package_graph_static_provider_contracts,
            package_graph_contract_resolution_records,
            package_graph_prepared_runtime_contract_tson_derivation_inputs,
            package_graph_runtime_contract_tson_derivation_inputs,
            package_graph_runtime_contract_tson_derivation_input_sidecar,
            package_graph_provider_import_records,
            program_admission,
            has_process_lifecycle_payload_carrier,
            capability_provider_import_identities: _provider_imports_moved_to_execution_session,
            rust_internal_capability_bindings,
            admitted_static_child_source_programs,
            prepared_static_child_runtime_handles,
        } = start_parts;
        Ok(DirectRunPublicAperturePreparedRuntimeProcessStartProduct {
            kernel_inputs: DirectRunPreparedRuntimeProcessStartKernelInputs {
                package_graph_template: DirectRunKernelStartPackageGraphTemplate {
                    source_path,
                    entry_executable_source_closure_fingerprint,
                    package_resolution_roots,
                    session_receipt: package_graph_session_receipt,
                    manifest_receipt: package_graph_session_manifest_receipt,
                    implementation_declarations: package_graph_implementation_declarations,
                    static_provider_contracts: package_graph_static_provider_contracts,
                    contract_resolution_records: package_graph_contract_resolution_records,
                    prepared_runtime_contract_tson_derivation_inputs:
                        package_graph_prepared_runtime_contract_tson_derivation_inputs,
                    runtime_contract_tson_derivation_inputs:
                        package_graph_runtime_contract_tson_derivation_inputs,
                    runtime_contract_tson_derivation_input_sidecar:
                        package_graph_runtime_contract_tson_derivation_input_sidecar,
                    provider_import_records: package_graph_provider_import_records,
                },
                prepared_source_program_ref,
                program_admission,
                has_process_lifecycle_payload_carrier,
                capability_link_bindings: rust_internal_capability_bindings,
                admitted_static_child_source_programs,
                prepared_static_child_runtime_handles,
                run_namespace,
                run_suffix,
                started_at,
                program_args,
                launch_cwd,
            },
            prepared_runtime_executable_image,
        })
    }
}

impl DirectRunPreparedRuntimeAuthorityOwner {
    pub(crate) fn admit_process_start_launch_values_for_compiler_owner_v1(
        values: DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
    ) -> Result<
        DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
        DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1,
    > {
        let validation = require_run_namespace(&values.run_namespace)
            .and_then(|()| require_run_suffix(&values.run_suffix))
            .and_then(|()| {
                require_timestamp(
                    &values.started_at,
                    "public_aperture_process_start.started_at",
                )
            })
            .and_then(|()| match values.launch_cwd.as_ref() {
                Some(launch_cwd) => {
                    require_non_empty(launch_cwd, "public_aperture_process_start.launch_cwd")
                }
                None => Ok(()),
            });
        if let Err(message) = validation {
            return Err(
                DirectRunProcessStartLaunchValuesAdmissionRefusalForCompilerOwnerV1 {
                    _values: values,
                    message,
                },
            );
        }
        Ok(values)
    }

    pub(crate) fn public_aperture_prepared_runtime_process_start_command(
        process_start_drive: DirectSwarmScriptRunPublicApertureProcessStartDrive,
        launch_values: DirectRunProcessStartLaunchValuesForCompilerOwnerV1,
    ) -> DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand {
        DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand {
            process_start_drive,
            launch_values,
        }
    }

    pub(crate) fn public_aperture_process_start_drive_from_execution_authority(
        execution_authority: DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority,
    ) -> DirectSwarmScriptRunPublicApertureProcessStartDrive {
        DirectSwarmScriptRunPublicApertureProcessStartDrive {
            inner: execution_authority.inner,
        }
    }

    // R41058: the handoff consume stops voiding cargo — the ordered
    // registered-case launch demand set survives beside the drive; the plan's
    // selection evidence still dies here (final observation owner unchanged).
    pub(crate) fn ss_test_public_aperture_process_start_drive_from_execution_authority_and_callable_body_result_binding_handoff(
        execution_authority: DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority,
        callable_body_result_binding_handoff:
            crate::source_entrypoint_compiler_admission_session::source_work_set::SsSelectedCallableBodyResultBindingHandoffForDirectRunOwnerV1,
    ) -> (
        DirectSwarmScriptRunPublicApertureProcessStartDrive,
        crate::test_declaration::SsTestRegisteredCaseBodyLaunchDemandSetForDirectRunOwnerV1,
    ){
        let registered_case_launch_demand_set = callable_body_result_binding_handoff
            .consume_for_direct_run_ss_test_body_launch_owner_v1();
        (
            DirectSwarmScriptRunPublicApertureProcessStartDrive {
                inner: execution_authority.inner,
            },
            registered_case_launch_demand_set,
        )
    }

    fn take_prepared_runtime_image_start_parts(
        handle: DirectSwarmScriptRunPreparedSourceProgramHandle,
    ) -> Result<DirectRunPreparedRuntimeImageStartParts, String> {
        let (runtime_image, prepared_runtime_executable_image) =
            take_direct_run_prepared_runtime_image_and_session(handle)?;
        Ok(Self::start_parts_from_preflighted_runtime_image(
            runtime_image,
            prepared_runtime_executable_image,
        ))
    }

    fn take_prepared_runtime_image_start_parts_from_thread_transfer_authority(
        transfer: DirectRunPreparedRuntimeImageThreadTransferAuthority,
    ) -> Result<DirectRunPreparedRuntimeImageStartParts, String> {
        let (runtime_image, prepared_runtime_executable_image) =
            take_direct_run_prepared_runtime_image_from_thread_transfer_authority(transfer);
        Ok(Self::start_parts_from_preflighted_runtime_image(
            runtime_image,
            prepared_runtime_executable_image,
        ))
    }

    fn start_parts_from_preflighted_runtime_image(
        mut runtime_image: DirectSwarmScriptRunPreparedRuntimeImage,
        prepared_runtime_executable_image: EngineInstalledPreparedSessionRuntimeV1,
    ) -> DirectRunPreparedRuntimeImageStartParts {
        let prepared_static_child_runtime_handles =
            Self::commit_preflighted_static_child_receipts_from_runtime_image(&mut runtime_image);
        let program_admission = runtime_image.program_admission;
        DirectRunPreparedRuntimeImageStartParts {
            prepared_source_program_ref: runtime_image.prepared_source_program_ref,
            source_path: runtime_image.source_path,
            entry_executable_source_closure_fingerprint: runtime_image
                .entry_executable_source_closure_fingerprint,
            package_resolution_roots: runtime_image.package_resolution_roots,
            package_graph_session_receipt: runtime_image.package_graph_session_receipt,
            package_graph_session_manifest_receipt: runtime_image
                .package_graph_session_manifest_receipt,
            package_graph_implementation_declarations: runtime_image
                .package_graph_implementation_declarations,
            package_graph_static_provider_contracts: runtime_image
                .package_graph_static_provider_contracts,
            package_graph_contract_resolution_records: runtime_image
                .package_graph_contract_resolution_records,
            package_graph_prepared_runtime_contract_tson_derivation_inputs: runtime_image
                .package_graph_prepared_runtime_contract_tson_derivation_inputs,
            package_graph_runtime_contract_tson_derivation_inputs: runtime_image
                .package_graph_runtime_contract_tson_derivation_inputs,
            package_graph_runtime_contract_tson_derivation_input_sidecar: runtime_image
                .package_graph_runtime_contract_tson_derivation_input_sidecar,
            package_graph_provider_import_records: runtime_image
                .package_graph_provider_import_records,
            program_admission,
            prepared_runtime_executable_image,
            has_process_lifecycle_payload_carrier: runtime_image
                .has_process_lifecycle_payload_carrier,
            capability_provider_import_identities: runtime_image
                .capability_provider_import_identities
                .map(PreparedRuntimeProviderImportAuthoritySet::into_execution_start_admission_for_direct_run_owner_v1),
            rust_internal_capability_bindings: runtime_image.rust_internal_capability_bindings,
            admitted_static_child_source_programs: runtime_image
                .admitted_static_child_source_programs,
            prepared_static_child_runtime_handles,
        }
    }

    pub(crate) fn take_prepared_runtime_image_thread_transfer_authority(
        handle: DirectSwarmScriptRunPreparedSourceProgramHandle,
    ) -> Result<DirectRunPreparedRuntimeImageThreadTransferAuthority, String> {
        take_direct_run_prepared_runtime_image_thread_transfer_authority(handle)
    }

    pub(crate) fn execution_authority_from_prepared_source_program_image_authority(
        image_authority: DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
    ) -> Result<
        DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority,
        (
            DirectSwarmScriptRunPreparedSourceProgramImageAuthority,
            DirectRunPreparedRuntimeImageStartFault,
        ),
    > {
        let DirectSwarmScriptRunPreparedSourceProgramImageAuthority { handle } = image_authority;
        match try_take_direct_run_prepared_runtime_image_and_session(handle) {
            Ok((runtime_image, prepared_runtime_executable_image)) => Ok(
                DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority::new(
                    Self::start_parts_from_preflighted_runtime_image(
                        runtime_image,
                        prepared_runtime_executable_image,
                    ),
                ),
            ),
            Err((handle, message)) => Err((
                DirectSwarmScriptRunPreparedSourceProgramImageAuthority { handle },
                message,
            )),
        }
    }

    pub(crate) fn execution_authority_from_thread_transfer_authority(
        transfer: DirectRunPreparedRuntimeImageThreadTransferAuthority,
    ) -> Result<DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority, String> {
        Self::take_prepared_runtime_image_start_parts_from_thread_transfer_authority(transfer)
            .map(DirectSwarmScriptRunPreparedRuntimeImageExecutionAuthority::new)
    }

    pub(crate) fn observe_canonical_ssc_stored_image_from_bytes(
        stored_image_bytes: &[u8],
    ) -> Result<Value, String> {
        observe_stored_image_bytes_for_artifact_owner_v1(stored_image_bytes)
    }

    pub(crate) fn install_prepared_runtime_image_thread_transfer_authority(
        transfer: DirectRunPreparedRuntimeImageThreadTransferAuthority,
    ) -> Result<
        DirectRunPreparedRuntimeImageThreadTransferInstallReceipt,
        DirectRunPreparedRuntimeImageThreadTransferInstallRefusal,
    > {
        install_direct_run_prepared_runtime_image_thread_transfer_authority(transfer)
    }
    pub(in crate::direct_run) fn prepared_runtime_registry_diagnostic_counts_value_v1() -> Value {
        let prepared_session_runtime_counts =
            direct_run_prepared_session_runtime_registry_kind_counts().unwrap_or_default();
        json!({
            "schema": "swarm.semantic_kernel.direct_run.prepared_runtime.registry_diagnostic_counts.v2",
            "prepared_static_child_custody": "prepared_runtime_receipt_and_runtime_handle",
            "prepared_source_programs": with_direct_run_thread_local_cell(
                &DIRECT_RUN_PREPARED_SOURCE_PROGRAM_REGISTRY,
                "prepared source program registry",
                |registry| registry.len(),
            ).unwrap_or_default(),
            "prepared_runtime_images": with_direct_run_thread_local_cell(
                &DIRECT_RUN_PREPARED_RUNTIME_IMAGE_REGISTRY,
                "prepared runtime image registry",
                |registry| registry.len(),
            ).unwrap_or_default(),
            "prepared_session_runtimes": prepared_session_runtime_counts.total(),
            "prepared_session_runtime_entry_kinds": {
                "materialized": prepared_session_runtime_counts.materialized,
            },
        })
    }

    pub(crate) fn require_prepared_source_program_handle(
        handle: &DirectSwarmScriptRunPreparedSourceProgramHandle,
    ) -> Result<(), String> {
        registries_prepared_artifacts::require_direct_run_prepared_source_program_handle(handle)
    }

    fn commit_preflighted_static_child_receipts_from_runtime_image(
        runtime_image: &mut DirectSwarmScriptRunPreparedRuntimeImage,
    ) -> DirectRunPreparedRuntimeStaticChildRuntimeHandles {
        let prepared_static_child_module_runs =
            std::mem::take(&mut runtime_image.prepared_static_child_module_runs);
        let prepared_static_child_selected_entries =
            std::mem::take(&mut runtime_image.prepared_static_child_selected_entries);
        registries_prepared_artifacts::commit_preflighted_direct_run_prepared_runtime_image_static_child_receipts(
            prepared_static_child_module_runs,
            prepared_static_child_selected_entries,
        )
    }

    fn prepare_source_program_image_install_v1(
        source_program_installs: Vec<DirectRunPreparedSourceProgramRegistryInstallV1>,
        image_staging: DirectRunPreparedSourceProgramImageStagingV1,
    ) -> Result<
        DirectRunPreparedSourceProgramImageInstallV1,
        DirectRunPreparedSourceProgramImageRegistryStagingRefusalV1,
    > {
        let DirectRunPreparedSourceProgramImageStagingV1 {
            prepared_source_program,
            install_transaction_identity,
            provider_lineage,
            cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner,
        } = image_staging;
        let source_program_commit = match DirectRunSourceProgramAuthorityOwner::preflight_prepared_source_program_registry_installs_for_prepared_runtime_owner_v1(
            source_program_installs,
            install_transaction_identity,
        ) {
            Ok(source_program_commit) => source_program_commit,
            Err((_, source_program_installs, install_transaction_identity)) => {
                return Err(DirectRunPreparedSourceProgramImageRegistryStagingRefusalV1 {
                    fault: DirectRunPreparedSourceProgramImageRegistryStagingFaultV1::SourceProgramRegistryPreflight,
                    custody: DirectRunPreparedSourceProgramImageRegistryStagingRefusalCustodyV1::Unpreflighted {
                        source_program_installs,
                        image_staging: DirectRunPreparedSourceProgramImageStagingV1 {
                            prepared_source_program,
                            install_transaction_identity,
                            provider_lineage,
                            cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner,
                        },
                    },
                });
            }
        };
        if let Err(fault) =
            registries_prepared_artifacts::preflight_direct_run_prepared_runtime_image_install(
                &prepared_source_program,
            )
        {
            return Err(DirectRunPreparedSourceProgramImageRegistryStagingRefusalV1 {
                fault,
                custody:
                    DirectRunPreparedSourceProgramImageRegistryStagingRefusalCustodyV1::PreparedRuntimeImagePreflight {
                        source_program_commit,
                        prepared_source_program,
                        provider_lineage,
                        cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner,
                    },
            });
        }
        Ok(DirectRunPreparedSourceProgramImageInstallV1 {
            source_program_commit,
            prepared_source_program,
            _provider_lineage: provider_lineage,
            _cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner:
                cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner,
        })
    }

    fn commit_source_program_image_install_v1(
        install: DirectRunPreparedSourceProgramImageInstallV1,
    ) -> DirectSwarmScriptRunPreparedSourceProgramImageAuthority {
        let DirectRunPreparedSourceProgramImageInstallV1 {
            source_program_commit,
            prepared_source_program,
            _provider_lineage: _,
            _cold_materialization_boundary_decode_plan_set_for_swarmvm_image_owner: _,
        } = install;
        let crate::direct_run::authority_kernel::source_program::DirectRunPreparedSourceProgramRegistryCommitV1 {
            replacement_registry,
            source_program_installs: _,
            image_install_transaction_identity: _,
        } = source_program_commit;
        let (handle, prepared_session_runtime_ref, prepared_runtime, runtime_image) =
            registries_prepared_artifacts::stage_preflighted_direct_run_prepared_runtime_image_install(
                prepared_source_program,
            );
        let prepared_source_program_ref = runtime_image.prepared_source_program_ref.clone();
        crate::direct_run::authority_kernel::source_program::DIRECT_RUN_ADMITTED_SOURCE_PROGRAM_REGISTRY.with(|source_program_registry| {
            DIRECT_RUN_PREPARED_SESSION_RUNTIME_REGISTRY.with(|prepared_session_runtime_registry| {
                DIRECT_RUN_PREPARED_RUNTIME_IMAGE_REGISTRY.with(|prepared_runtime_image_registry| {
                    // Acquire every registry borrow before the first write. All
                    // allocation, validation, correspondence, and image staging
                    // has already completed; publication below is one infallible
                    // owner commit with no callback or fallible operation.
                    let mut source_program_registry = source_program_registry.borrow_mut();
                    let mut prepared_session_runtime_registry =
                        prepared_session_runtime_registry.borrow_mut();
                    let mut prepared_runtime_image_registry =
                        prepared_runtime_image_registry.borrow_mut();
                    prepared_session_runtime_registry.reserve(1);
                    prepared_runtime_image_registry.reserve(1);
                    *source_program_registry = replacement_registry;
                    prepared_session_runtime_registry.insert(
                        prepared_session_runtime_ref,
                        DirectRunPreparedSessionRuntimeRegistryEntry::Materialized(
                            prepared_runtime,
                        ),
                    );
                    prepared_runtime_image_registry
                        .insert(prepared_source_program_ref, runtime_image);
                });
            });
        });
        DirectSwarmScriptRunPreparedSourceProgramImageAuthority::mint_from_source_program_owner_prepared_image_v1(handle)
    }
}

#[cfg(test)]
mod process_start_admission_custody_tests {
    use super::*;

    const _: fn() = || {
        trait AmbiguousIfClone<A> {
            fn probe() {}
        }
        impl<T: ?Sized> AmbiguousIfClone<()> for T {}
        impl<T: ?Sized + Clone> AmbiguousIfClone<u8> for T {}
        let _ = <DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1 as AmbiguousIfClone<_>>::probe;
        let _ = <AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1 as AmbiguousIfClone<_>>::probe;
        let _ = <DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1 as AmbiguousIfClone<_>>::probe;
    };

    #[test]
    fn durable_refusal_keeps_command_beside_the_typed_host_refusal() {
        fn retain_complete_refusal(
            refusal: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
        ) {
            let DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1 {
                command,
                host_refusal,
            } = refusal;
            let _retained_custody = (command, host_refusal);
        }

        let _typed_contract: fn(
            DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
        ) = retain_complete_refusal;
    }
}
