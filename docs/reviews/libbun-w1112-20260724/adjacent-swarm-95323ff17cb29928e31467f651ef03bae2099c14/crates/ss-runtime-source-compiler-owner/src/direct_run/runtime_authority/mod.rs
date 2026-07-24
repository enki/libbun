use std::cell::RefCell;

pub(crate) enum DirectRunProcessSessionDriveFaultV1 {
    RuntimeTerminal {
        observation: crate::DirectRunProcessSessionRuntimeTerminalFaultObservationV1,
        context: &'static str,
    },
    ProcessChild(
        process_session_public_aperture::process_child_lifecycle::DirectRunProcessChildDriveFailureV1,
    ),
    ProcessChildStage(kernel_state_substrate::DirectRunProcessChildStageFaultV1),
    ProcessChildParentResume(
        process_session_public_aperture::process_child_lifecycle::DirectRunProcessChildParentResumeFaultV1,
    ),
    ProcessLoad(
        process_session_public_aperture::provider_resume_lifecycle::DirectRunProcessLoadExecutionRefusalV1,
    ),
    ProcessCheckpoint(
        process_session_public_aperture::provider_resume_lifecycle::DirectRunProcessCheckpointExecutionRefusalV1,
    ),
    ProcessRestore(
        process_session_public_aperture::provider_resume_lifecycle::DirectRunProcessRestoreExecutionRefusalV1,
    ),
    ProcessSessionStartAdmission(
        live_process_session_registry::DirectRunProcessSessionStartAdmissionRefusalV1,
    ),
    ProcessSessionStartDrive(
        live_process_session_registry::DirectRunProcessSessionStartDriveRefusalV1,
    ),
    Generic(String),
}

impl From<String> for DirectRunProcessSessionDriveFaultV1 {
    fn from(message: String) -> Self {
        Self::Generic(message)
    }
}

impl DirectRunProcessSessionDriveFaultV1 {
    pub(crate) fn from_session_run_error_for_direct_run_owner_v1(
        error: crate::ProcessSessionRunError,
    ) -> Self {
        match error {
            crate::ProcessSessionRunError::RuntimeTerminal {
                observation,
                context,
            } => Self::RuntimeTerminal {
                observation,
                context,
            },
            error => Self::Generic(error.to_string()),
        }
    }

    pub(crate) fn cancel_into_generic_message_for_direct_run_boundary_owner_v1(
        self,
    ) -> Result<String, Self> {
        match self {
            Self::RuntimeTerminal {
                observation,
                context,
            } => {
                let (code, message) =
                    observation.consume_into_generic_fault_for_direct_run_boundary_owner_v1();
                Ok(format!(
                    "process session actor scheduler failed: {}",
                    serde_json::json!({
                        "kind": code,
                        "reason": message,
                        "context": context,
                    })
                ))
            }
            Self::ProcessChild(refusal) => {
                Ok(refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessChildStage(fault) => {
                Ok(fault.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessChildParentResume(fault) => {
                Ok(fault.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessLoad(refusal) => {
                Ok(refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessCheckpoint(refusal) => {
                Ok(refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessRestore(refusal) => {
                Ok(refusal.consume_into_final_diagnostic_for_direct_run_boundary_owner_v1())
            }
            Self::ProcessSessionStartAdmission(refusal) => refusal
                .cancel_for_direct_run_boundary_owner_v1()
                .map(|receipt| {
                    receipt.consume_into_generic_message_for_direct_run_boundary_owner_v1()
                })
                .map_err(Self::ProcessSessionStartAdmission),
            Self::ProcessSessionStartDrive(refusal) => refusal
                .cancel_for_direct_run_boundary_owner_v1()
                .map(|receipt| {
                    receipt.consume_into_generic_message_for_direct_run_boundary_owner_v1()
                })
                .map_err(Self::ProcessSessionStartDrive),
            Self::Generic(message) => Ok(message),
        }
    }
}

mod base_refs;
mod child_terminal_persist_refs;
mod kernel_state_substrate;
pub(super) mod live_persist_receipts;
mod live_process_session_registry;
pub(super) mod live_source_resume;
mod process_child_launch;
mod process_invoke_child_settlement;
mod process_kernel_boundary;
mod process_session_public_aperture;
mod process_session_result_authority;
mod public_abi_boundary;
mod runtime_operation;
mod typed_continuation_token_admission;
mod volatile_runtime_state;

pub use self::kernel_state_substrate::DirectRunProcessSessionPublicApertureDriveOutputV1;
pub(in crate::direct_run) use self::kernel_state_substrate::DirectRunProcessSessionPublicApertureNextStepOutputProductV1;
pub(in crate::direct_run) use child_terminal_persist_refs::DirectRunProcessRunChildExecutionAuthority;
#[cfg(test)]
pub(in crate::direct_run) use live_persist_receipts::known_rust_internal_provider_target;
pub(in crate::direct_run) use live_persist_receipts::{
    known_rust_internal_provider_target_for_authored_facade_binding,
    known_rust_internal_provider_target_for_selected_operation,
};
pub use process_session_result_authority::{
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    DirectRunSsTestBodyWorkMaterializationAuthority,
    DirectRunSsTestBodyWorkMaterializationRootAuthority,
    DirectRunSsTestCaseTerminalObservationAuthority, DirectRunSsTestExecutedFileResultAuthority,
    DirectRunSsTestExecutedFileTerminalObservationCursor,
    DirectRunSsTestExecutedFileTerminalObservationProductAuthority,
    DirectRunSsTestTerminalObservationFaultV1,
};
pub(in crate::direct_run) use process_session_result_authority::{
    DirectRunProcessSessionTerminalFinalizationProductV1,
    DirectRunProcessSessionTerminalFinalizationReceiptV1,
    DirectRunProcessSessionTerminalPublicOutputProductV1, EngineProcessSessionRunResultV1,
    EngineProcessSessionTerminalResultProductV1,
};
pub use public_abi_boundary::enter_direct_run_public_abi_boundary;
pub(in crate::direct_run) use runtime_operation::{
    DirectRunContinuationSealOwnerRequest, DirectRunEventPublicationResumeContinuationHandle,
    DirectRunRuntimeAuthorityOwner,
};
fn with_direct_run_thread_local_cell<T, R>(
    cell: &'static std::thread::LocalKey<RefCell<T>>,
    label: &'static str,
    body: impl FnOnce(&T) -> R,
) -> Result<R, String> {
    cell.try_with(|cell| {
        let value = cell
            .try_borrow()
            .map_err(|_| format!("direct-run {label} is already borrowed"))?;
        Ok(body(&value))
    })
    .map_err(|_| format!("direct-run {label} is unavailable"))?
}

fn with_direct_run_thread_local_cell_mut<T, R>(
    cell: &'static std::thread::LocalKey<RefCell<T>>,
    label: &'static str,
    body: impl FnOnce(&mut T) -> R,
) -> Result<R, String> {
    cell.try_with(|cell| {
        let mut value = cell
            .try_borrow_mut()
            .map_err(|_| format!("direct-run {label} is already borrowed"))?;
        Ok(body(&mut value))
    })
    .map_err(|_| format!("direct-run {label} is unavailable"))?
}
