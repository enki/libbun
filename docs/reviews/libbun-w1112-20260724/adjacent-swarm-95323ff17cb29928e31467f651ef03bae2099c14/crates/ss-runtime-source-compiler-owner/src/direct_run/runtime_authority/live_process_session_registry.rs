use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use super::EngineProcessSessionRunResultV1;
use crate::direct_run::event::DirectRunEventPublicationBackendOutputDrainObservationBundle;
use crate::{
    DirectRunProcessSessionPublicApertureProgressProductV1,
    DirectRunProcessSessionRunResultProductV1, ProcessControlResumeProductForDirectRunOwnerV1,
    ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    ProcessRunChildProviderIngressForDirectRunOwnerV1,
    ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    ProcessSessionV0 as EngineLiveProcessSessionV1,
    SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    SelectedProviderResumeHostInputForDirectRunOwnerV1,
};
use swarmvm_runtime_types::VmBoundaryValue;

use super::super::{
    DirectRunActorStateProductOutboxHandoffRoute, DirectRunLiveProcessSessionRef,
    DirectRunLiveProcessSessionRegistryEntry, DirectSwarmScriptRunKernelState,
    clear_direct_run_mesh_control_loopback_hosts_for_root,
    process_creation_export_readiness_for_live_process_session_owner_v1, require_non_empty,
};
use super::kernel_state_substrate::{
    DirectRunProcessSessionPublicApertureRouteOutput, DirectRunProcessSessionStartDriveAuthority,
    admit_direct_run_process_session_start_owner_transition,
};
use super::process_session_public_aperture::session_route_lifecycle::route_engine_process_session_result_for_public_aperture_start_owner;
use super::{
    DirectRunProcessSessionDriveFaultV1, DirectRunRuntimeAuthorityOwner,
    with_direct_run_thread_local_cell, with_direct_run_thread_local_cell_mut,
};

static DIRECT_RUN_LIVE_PROCESS_SESSION_NEXT_ID: AtomicU64 = AtomicU64::new(1);
const DIRECT_RUN_RUST_SDK_PUBLISH_EVENT_RUN_LIMIT: usize = 64;

thread_local! {
    static DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY:
        RefCell<HashMap<String, DirectRunLiveProcessSessionRegistryEntry>> =
            RefCell::new(HashMap::new());
}

pub(in crate::direct_run) enum DirectRunProcessInvokeLiveSessionResumeFaultV1 {
    Registry {
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
        failure: String,
    },
    Transition(crate::ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1),
}

pub(in crate::direct_run) enum DirectRunProcessRunLiveSessionResumeFaultV1 {
    Registry {
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
        failure: String,
    },
    Transition(crate::ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1),
}

pub(in crate::direct_run) struct DirectRunPendingProcessChildEffectsAppendFaultV1 {
    pub(in crate::direct_run) observations:
        DirectRunEventPublicationBackendOutputDrainObservationBundle,
    pub(in crate::direct_run) process_output_records:
        Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
    pub(in crate::direct_run) failure: String,
}

pub(in crate::direct_run) struct DirectRunProcessSessionStartAdmissionRefusalV1 {
    opened_session: EngineLiveProcessSessionV1,
    failure: String,
    cleanup_failure: Option<String>,
}

impl DirectRunProcessSessionStartAdmissionRefusalV1 {
    pub(super) fn cancel_for_direct_run_boundary_owner_v1(
        mut self,
    ) -> Result<DirectRunProcessSessionStartCancellationReceiptV1, Self> {
        match self
            .opened_session
            .settle_pending_actor_handler_entry_for_session_teardown_owner_v1()
        {
            Ok(()) => Ok(DirectRunProcessSessionStartCancellationReceiptV1 {
                failure: self.failure,
            }),
            Err(cleanup_failure) => {
                self.cleanup_failure = Some(cleanup_failure.to_string());
                Err(self)
            }
        }
    }
}

pub(super) struct DirectRunProcessSessionStartCancellationReceiptV1 {
    failure: String,
}

impl DirectRunProcessSessionStartCancellationReceiptV1 {
    pub(super) fn consume_into_generic_message_for_direct_run_boundary_owner_v1(self) -> String {
        self.failure
    }
}

struct DirectRunProcessSessionStartRegistryCleanupAuthorityV1 {
    live_process_session_id: String,
    root_scope_id: String,
}

enum DirectRunProcessSessionStartDriveRefusalCustodyV1 {
    BeforeRoute {
        start_authority: DirectRunProcessSessionStartDriveAuthority,
        detached_session: Option<DirectRunLiveProcessSessionRegistryEntry>,
    },
    AfterRoute {
        cleanup_authority: DirectRunProcessSessionStartRegistryCleanupAuthorityV1,
        detached_session: Option<DirectRunLiveProcessSessionRegistryEntry>,
    },
}

enum DirectRunProcessSessionStartDriveFailureV1 {
    Registry(String),
    Runtime(crate::ProcessSessionRunError),
    Route {
        failure: String,
        cleanup_failure: Option<String>,
    },
}

pub(in crate::direct_run) struct DirectRunProcessSessionStartDriveRefusalV1 {
    custody: DirectRunProcessSessionStartDriveRefusalCustodyV1,
    failure: DirectRunProcessSessionStartDriveFailureV1,
    cleanup_failure: Option<String>,
}

impl DirectRunProcessSessionStartDriveRefusalV1 {
    pub(super) fn cancel_for_direct_run_boundary_owner_v1(
        self,
    ) -> Result<DirectRunProcessSessionStartCancellationReceiptV1, Self> {
        let Self {
            custody,
            failure,
            cleanup_failure: _,
        } = self;
        let cancellation = match custody {
            DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                start_authority,
                detached_session: Some(detached_session),
            } => settle_detached_process_session_start_refusal_for_cancellation(detached_session)
                .map_err(|(detached_session, cleanup_failure)| {
                    (
                        DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                            start_authority,
                            detached_session: Some(detached_session),
                        },
                        cleanup_failure,
                    )
                }),
            DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                start_authority,
                detached_session: None,
            } => {
                let live_process_session_id =
                    start_authority.token().live_process_session_id().to_owned();
                let root_scope_id = start_authority.token().root_scope_id().to_owned();
                settle_registered_process_session_start_refusal_for_cancellation(
                    &live_process_session_id,
                    &root_scope_id,
                )
                .map_err(|cleanup_failure| {
                    (
                        DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                            start_authority,
                            detached_session: None,
                        },
                        cleanup_failure,
                    )
                })
            }
            DirectRunProcessSessionStartDriveRefusalCustodyV1::AfterRoute {
                cleanup_authority,
                detached_session: Some(detached_session),
            } => settle_detached_process_session_start_refusal_for_cancellation(detached_session)
                .map_err(|(detached_session, cleanup_failure)| {
                    (
                        DirectRunProcessSessionStartDriveRefusalCustodyV1::AfterRoute {
                            cleanup_authority,
                            detached_session: Some(detached_session),
                        },
                        cleanup_failure,
                    )
                }),
            DirectRunProcessSessionStartDriveRefusalCustodyV1::AfterRoute {
                cleanup_authority,
                detached_session: None,
            } => settle_registered_process_session_start_refusal_for_cancellation(
                &cleanup_authority.live_process_session_id,
                &cleanup_authority.root_scope_id,
            )
            .map_err(|cleanup_failure| {
                (
                    DirectRunProcessSessionStartDriveRefusalCustodyV1::AfterRoute {
                        cleanup_authority,
                        detached_session: None,
                    },
                    cleanup_failure,
                )
            }),
        };
        match cancellation {
            Ok(()) => Ok(DirectRunProcessSessionStartCancellationReceiptV1 {
                failure: failure.into_generic_message_for_direct_run_boundary_owner_v1(),
            }),
            Err((custody, cleanup_failure)) => Err(Self {
                custody,
                failure,
                cleanup_failure: Some(cleanup_failure),
            }),
        }
    }
}

impl DirectRunProcessSessionStartDriveFailureV1 {
    fn into_generic_message_for_direct_run_boundary_owner_v1(self) -> String {
        match self {
            DirectRunProcessSessionStartDriveFailureV1::Registry(failure) => failure,
            DirectRunProcessSessionStartDriveFailureV1::Runtime(failure) => failure.to_string(),
            DirectRunProcessSessionStartDriveFailureV1::Route {
                failure,
                cleanup_failure: None,
            } => failure,
            DirectRunProcessSessionStartDriveFailureV1::Route {
                failure,
                cleanup_failure: Some(cleanup_failure),
            } => format!(
                "{failure}; process-session start refusal retained cleanup authority after cleanup failed: {cleanup_failure}"
            ),
        }
    }
}

fn retire_terminal_registration_from_registry_for_direct_run_owner_v1<T>(
    registry: &mut HashMap<String, T>,
    session_id: &str,
    expected_root_scope_id: &str,
    root_scope_id: impl for<'entry> Fn(&'entry T) -> &'entry str,
) -> Result<bool, String> {
    let entry = registry.get(session_id).ok_or_else(|| {
        format!("live process-session id '{session_id}' is not present in the Rust registry")
    })?;
    let observed_root_scope_id = root_scope_id(entry);
    if observed_root_scope_id != expected_root_scope_id {
        return Err(format!(
            "live process-session id '{session_id}' belongs to root scope '{observed_root_scope_id}', expected '{expected_root_scope_id}'"
        ));
    }
    Ok(registry.remove(session_id).is_some())
}

fn detach_live_process_session_entry_for_start_refusal(
    registry: &mut HashMap<String, DirectRunLiveProcessSessionRegistryEntry>,
    session_id: &str,
    expected_root_scope_id: &str,
) -> Result<Option<DirectRunLiveProcessSessionRegistryEntry>, String> {
    let Some(entry) = registry.get(session_id) else {
        return Ok(None);
    };
    if entry.root_scope_id != expected_root_scope_id {
        return Err(format!(
            "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
            entry.root_scope_id
        ));
    }
    Ok(registry.remove(session_id))
}

fn settle_detached_process_session_start_refusal_for_cancellation(
    mut entry: DirectRunLiveProcessSessionRegistryEntry,
) -> Result<(), (DirectRunLiveProcessSessionRegistryEntry, String)> {
    match entry
        .session
        .settle_pending_actor_handler_entry_for_session_teardown_owner_v1()
    {
        Ok(()) => Ok(()),
        Err(cleanup_failure) => Err((entry, cleanup_failure.to_string())),
    }
}

fn settle_registered_process_session_start_refusal_for_cancellation(
    session_id: &str,
    expected_root_scope_id: &str,
) -> Result<(), String> {
    with_direct_run_thread_local_cell_mut(
        &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
        "live process-session registry",
        |registry| {
            let Some(entry) = registry.get_mut(session_id) else {
                return Ok(());
            };
            if entry.root_scope_id != expected_root_scope_id {
                return Err(format!(
                    "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
                    entry.root_scope_id
                ));
            }
            entry
                .session
                .settle_pending_actor_handler_entry_for_session_teardown_owner_v1()
                .map_err(|cleanup_failure| cleanup_failure.to_string())?;
            registry.remove(session_id);
            Ok(())
        },
    )?
}

fn with_live_process_session<T>(
    session_id: &str,
    expected_root_scope_id: &str,
    operation: impl FnOnce(&EngineLiveProcessSessionV1) -> Result<T, String>,
) -> Result<T, String> {
    require_non_empty(session_id, "live_process_session_id")?;
    require_non_empty(
        expected_root_scope_id,
        "live_process_session.expected_root_scope_id",
    )?;
    with_direct_run_thread_local_cell(
        &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
        "live process-session registry",
        |registry| {
            let entry = registry.get(session_id).ok_or_else(|| {
                format!(
                    "live process-session id '{session_id}' is not present in the Rust registry"
                )
            })?;
            if entry.root_scope_id != expected_root_scope_id {
                return Err(format!(
                    "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
                    entry.root_scope_id
                ));
            }
            operation(&entry.session)
        },
    )?
}

fn with_live_process_session_entry_mut<T>(
    session_id: &str,
    expected_root_scope_id: &str,
    operation: impl FnOnce(&mut DirectRunLiveProcessSessionRegistryEntry) -> Result<T, String>,
) -> Result<T, String> {
    require_non_empty(session_id, "live_process_session_id")?;
    require_non_empty(
        expected_root_scope_id,
        "live_process_session.expected_root_scope_id",
    )?;
    with_direct_run_thread_local_cell_mut(
        &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
        "live process-session registry",
        |registry| {
            let entry = registry.get_mut(session_id).ok_or_else(|| {
                format!(
                    "live process-session id '{session_id}' is not present in the Rust registry"
                )
            })?;
            if entry.root_scope_id != expected_root_scope_id {
                return Err(format!(
                    "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
                    entry.root_scope_id
                ));
            }
            operation(entry)
        },
    )?
}

fn with_live_process_session_mut<T>(
    session_id: &str,
    expected_root_scope_id: &str,
    operation: impl FnOnce(&mut EngineLiveProcessSessionV1) -> Result<T, String>,
) -> Result<T, String> {
    with_live_process_session_entry_mut(session_id, expected_root_scope_id, |entry| {
        operation(&mut entry.session)
    })
}

fn with_live_process_session_mut_for_drive_owner_v1<T>(
    session_id: &str,
    expected_root_scope_id: &str,
    operation: impl FnOnce(&mut EngineLiveProcessSessionV1) -> Result<T, crate::ProcessSessionRunError>,
) -> Result<T, DirectRunProcessSessionDriveFaultV1> {
    require_non_empty(session_id, "live_process_session_id")
        .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?;
    require_non_empty(
        expected_root_scope_id,
        "live_process_session.expected_root_scope_id",
    )
    .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?;
    with_direct_run_thread_local_cell_mut(
        &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
        "live process-session registry",
        |registry| {
            let entry = registry.get_mut(session_id).ok_or_else(|| {
                DirectRunProcessSessionDriveFaultV1::Generic(format!(
                    "live process-session id '{session_id}' is not present in the Rust registry"
                ))
            })?;
            if entry.root_scope_id != expected_root_scope_id {
                return Err(DirectRunProcessSessionDriveFaultV1::Generic(format!(
                    "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
                    entry.root_scope_id
                )));
            }
            operation(&mut entry.session).map_err(
                DirectRunProcessSessionDriveFaultV1::from_session_run_error_for_direct_run_owner_v1,
            )
        },
    )
    .map_err(DirectRunProcessSessionDriveFaultV1::Generic)?
}

fn append_pending_process_session_effects_for_live_process_session_registry_owner_v1(
    entry: &mut DirectRunLiveProcessSessionRegistryEntry,
    observations: DirectRunEventPublicationBackendOutputDrainObservationBundle,
    process_output_records: Option<
        crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    >,
) {
    entry
        .pending_event_publication_backend_output_drain_observations
        .extend_for_direct_run_process_session_result_owner_v1(observations);
    let Some(process_output_records) = process_output_records else {
        return;
    };
    match &mut entry.pending_provider_process_output_records {
        Some(existing) => {
            existing.extend_for_direct_run_process_child_owner_v1(process_output_records);
        }
        None => entry.pending_provider_process_output_records = Some(process_output_records),
    }
}

impl DirectRunRuntimeAuthorityOwner {
    pub(in crate::direct_run) fn admit_process_session_start_with_live_session_for_direct_run_owner_v1(
        mut kernel_state: DirectSwarmScriptRunKernelState,
        session: EngineLiveProcessSessionV1,
        operation: &'static str,
    ) -> Result<
        DirectRunProcessSessionStartDriveAuthority,
        DirectRunProcessSessionStartAdmissionRefusalV1,
    > {
        let root_scope_id = Self::kernel_state_root_scope_id_for_route_consistency(
            &kernel_state,
            "admit_process_session_start_with_live_session.root_scope_id",
        );
        let mut opened_session = Some(session);
        let admission = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                require_non_empty(&root_scope_id, "live_process_session.root_scope_id")?;
                if Self::kernel_state_live_process_session_id_for_route_consistency(
                    &kernel_state,
                    "admit_process_session_start_with_live_session.live_session_ref",
                )
                .is_some()
                {
                    return Err(
                        "finite process-session start admission requires an unregistered kernel state"
                            .to_owned(),
                    );
                }
                let live_process_session_ref =
                    DirectRunLiveProcessSessionRef::generated_for_direct_run_live_process_session_registry_owner_v1(
                        format!(
                            "live-process-session-{}",
                            DIRECT_RUN_LIVE_PROCESS_SESSION_NEXT_ID
                                .fetch_add(1, Ordering::Relaxed)
                        ),
                    );
                if registry.contains_key(live_process_session_ref.as_str()) {
                    return Err(format!(
                        "generated live process-session id '{}' is already registered",
                        live_process_session_ref.as_str()
                    ));
                }
                let live_process_session_id = live_process_session_ref.as_str().to_owned();
                kernel_state.set_live_process_session_ref_authority(live_process_session_ref);
                let admitted = admit_direct_run_process_session_start_owner_transition(
                    Box::new(kernel_state),
                    operation,
                )?;
                let session = opened_session
                    .take()
                    .expect("opened session remains retained until the final registry commit");
                registry.insert(
                    live_process_session_id,
                    DirectRunLiveProcessSessionRegistryEntry {
                        root_scope_id,
                        session,
                        pending_event_publication_backend_output_drain_observations:
                            DirectRunEventPublicationBackendOutputDrainObservationBundle::empty_for_direct_run_event_publication_owner_v1(),
                        pending_provider_process_output_records: None,
                    },
                );
                Ok(admitted)
            },
        );
        match admission {
            Ok(Ok(admitted)) => Ok(admitted),
            Ok(Err(failure)) | Err(failure) => {
                Err(DirectRunProcessSessionStartAdmissionRefusalV1 {
                    opened_session: opened_session.expect(
                        "failed process-session start admission retains its uncommitted session",
                    ),
                    failure,
                    cleanup_failure: None,
                })
            }
        }
    }

    pub(in crate::direct_run) fn clear_live_process_session_storage_for_root(
        root_scope_id: &str,
    ) -> Result<usize, String> {
        require_non_empty(root_scope_id, "live_process_session.root_scope_id")?;
        let live_sessions_cleared = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                for entry in registry
                    .values_mut()
                    .filter(|entry| entry.root_scope_id == root_scope_id)
                {
                    entry
                        .session
                        .settle_pending_actor_handler_entry_for_session_teardown_owner_v1()
                        .map_err(|source| source.to_string())?;
                }
                let before = registry.len();
                registry.retain(|_, entry| entry.root_scope_id != root_scope_id);
                Ok::<usize, String>(before.saturating_sub(registry.len()))
            },
        )??;
        let mesh_loopback_hosts_cleared =
            clear_direct_run_mesh_control_loopback_hosts_for_root(root_scope_id)?;
        Ok(live_sessions_cleared + mesh_loopback_hosts_cleared)
    }

    pub(in crate::direct_run) fn retire_terminal_live_process_session_storage(
        session_id: &str,
        expected_root_scope_id: &str,
    ) -> Result<(), String> {
        require_non_empty(session_id, "live_process_session_id")?;
        require_non_empty(
            expected_root_scope_id,
            "live_process_session.expected_root_scope_id",
        )?;
        with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                let entry = registry.get_mut(session_id).ok_or_else(|| {
                    format!(
                        "live process-session id '{session_id}' is not present in the Rust registry"
                    )
                })?;
                if entry.root_scope_id != expected_root_scope_id {
                    return Err(format!(
                        "live process-session id '{session_id}' belongs to root scope '{}', expected '{expected_root_scope_id}'",
                        entry.root_scope_id
                    ));
                }
                entry
                    .session
                    .settle_pending_actor_handler_entry_for_session_teardown_owner_v1()
                    .map_err(|source| source.to_string())?;
                retire_terminal_registration_from_registry_for_direct_run_owner_v1(
                    registry,
                    session_id,
                    expected_root_scope_id,
                    |entry| entry.root_scope_id.as_str(),
                )?;
                Ok(())
            },
        )?
    }

    pub(in crate::direct_run) fn live_process_session_storage_count() -> usize {
        with_direct_run_thread_local_cell(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process session registry",
            |registry| registry.len(),
        )
        .unwrap_or_default()
    }

    pub(in crate::direct_run) fn append_pending_process_output_records_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
    ) -> Result<(), DirectRunPendingProcessChildEffectsAppendFaultV1> {
        Self::append_pending_process_child_effects_for_live_process_session(
            session_id,
            root_scope_id,
            DirectRunEventPublicationBackendOutputDrainObservationBundle::empty_for_direct_run_event_publication_owner_v1(),
            process_output_records,
        )
    }

    pub(in crate::direct_run) fn append_pending_process_child_effects_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        observations: DirectRunEventPublicationBackendOutputDrainObservationBundle,
        process_output_records: Option<
            crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
        >,
    ) -> Result<(), DirectRunPendingProcessChildEffectsAppendFaultV1> {
        let mut cargo = Some((observations, process_output_records));
        let registry_result = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                let entry = match registry.get_mut(session_id) {
                    Some(entry) => entry,
                    None => {
                        let (observations, process_output_records) = cargo
                            .take()
                            .expect("pending child effects remain before registry admission");
                        return Err(DirectRunPendingProcessChildEffectsAppendFaultV1 {
                            observations,
                            process_output_records,
                            failure: format!(
                                "live process-session id '{session_id}' is not present in the Rust registry"
                            ),
                        });
                    }
                };
                if entry.root_scope_id != root_scope_id {
                    let (observations, process_output_records) = cargo
                        .take()
                        .expect("pending child effects remain before root-scope admission");
                    return Err(DirectRunPendingProcessChildEffectsAppendFaultV1 {
                        observations,
                        process_output_records,
                        failure: format!(
                            "live process-session id '{session_id}' belongs to root scope '{}', expected '{root_scope_id}'",
                            entry.root_scope_id
                        ),
                    });
                }
                let (observations, process_output_records) = cargo
                    .take()
                    .expect("pending child effects commit consumes its cargo once");
                append_pending_process_session_effects_for_live_process_session_registry_owner_v1(
                    entry,
                    observations,
                    process_output_records,
                );
                Ok(())
            },
        );
        match registry_result {
            Ok(result) => result,
            Err(failure) => {
                let (observations, process_output_records) = cargo
                    .take()
                    .expect("thread-local registry refusal retains pending child effects");
                Err(DirectRunPendingProcessChildEffectsAppendFaultV1 {
                    observations,
                    process_output_records,
                    failure,
                })
            }
        }
    }

    pub(in crate::direct_run) fn take_pending_process_session_effects_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
    ) -> Result<
        (
            DirectRunEventPublicationBackendOutputDrainObservationBundle,
            Option<crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner>,
        ),
        String,
    > {
        with_live_process_session_entry_mut(session_id, root_scope_id, |entry| {
            Ok((
                std::mem::replace(
                    &mut entry.pending_event_publication_backend_output_drain_observations,
                    DirectRunEventPublicationBackendOutputDrainObservationBundle::empty_for_direct_run_event_publication_owner_v1(),
                ),
                entry.pending_provider_process_output_records.take(),
            ))
        })
    }

    pub(in crate::direct_run) fn drive_registered_process_session_start_to_first_owner_output_for_direct_run_owner_v1(
        start_authority: DirectRunProcessSessionStartDriveAuthority,
    ) -> Result<
        DirectRunProcessSessionPublicApertureRouteOutput,
        DirectRunProcessSessionStartDriveRefusalV1,
    > {
        let live_process_session_id = start_authority.token().live_process_session_id().to_owned();
        let root_scope_id = start_authority.token().root_scope_id().to_owned();
        let drive_result = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                let drive_attempt = {
                    let entry = registry.get_mut(&live_process_session_id).ok_or_else(|| {
                        format!(
                            "live process-session id '{live_process_session_id}' is not present in the Rust registry"
                        )
                    })?;
                    if entry.root_scope_id != root_scope_id {
                        return Err(format!(
                            "live process-session id '{live_process_session_id}' belongs to root scope '{}', expected '{root_scope_id}'",
                            entry.root_scope_id
                        ));
                    }
                    entry
                        .session
                        .drive_process_session_until_external_boundary_with_runtime_terminal_observation_for_direct_run_owner_v1()
                        .map(|outcome| {
                            let process_creation_export_readiness =
                                process_creation_export_readiness_for_live_process_session_owner_v1(
                                    &entry.session,
                                    "process_session_start_external_drive",
                                );
                            EngineProcessSessionRunResultV1::admitted(
                                outcome,
                                None,
                                process_creation_export_readiness,
                                "process_session_start_external_drive",
                            )
                        })
                };
                match drive_attempt {
                    Ok(engine_result) => Ok(Ok(engine_result)),
                    Err(failure) => {
                        let detached_session = registry.remove(&live_process_session_id).expect(
                            "the live process-session entry remains registered until its first drive settles",
                        );
                        Ok(Err((failure, detached_session)))
                    }
                }
            },
        );
        let engine_result = match drive_result {
            Ok(Ok(Ok(engine_result))) => engine_result,
            Ok(Ok(Err((failure, detached_session)))) => {
                return Err(DirectRunProcessSessionStartDriveRefusalV1 {
                    custody: DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                        start_authority,
                        detached_session: Some(detached_session),
                    },
                    failure: DirectRunProcessSessionStartDriveFailureV1::Runtime(failure),
                    cleanup_failure: None,
                });
            }
            Ok(Err(failure)) | Err(failure) => {
                return Err(DirectRunProcessSessionStartDriveRefusalV1 {
                    custody: DirectRunProcessSessionStartDriveRefusalCustodyV1::BeforeRoute {
                        start_authority,
                        detached_session: None,
                    },
                    failure: DirectRunProcessSessionStartDriveFailureV1::Registry(failure),
                    cleanup_failure: None,
                });
            }
        };
        let cleanup_authority = DirectRunProcessSessionStartRegistryCleanupAuthorityV1 {
            live_process_session_id,
            root_scope_id,
        };
        let (process_session_start_token, execution_substrate) =
            start_authority.into_route_inputs();
        match route_engine_process_session_result_for_public_aperture_start_owner(
            process_session_start_token,
            execution_substrate,
            engine_result,
        ) {
            Ok(output) => Ok(output),
            Err(failure) => {
                let cleanup_result = with_direct_run_thread_local_cell_mut(
                    &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
                    "live process-session registry",
                    |registry| {
                        detach_live_process_session_entry_for_start_refusal(
                            registry,
                            &cleanup_authority.live_process_session_id,
                            &cleanup_authority.root_scope_id,
                        )
                    },
                );
                let (detached_session, cleanup_failure) = match cleanup_result {
                    Ok(Ok(detached_session)) => (detached_session, None),
                    Ok(Err(cleanup_failure)) | Err(cleanup_failure) => {
                        (None, Some(cleanup_failure))
                    }
                };
                Err(DirectRunProcessSessionStartDriveRefusalV1 {
                    custody: DirectRunProcessSessionStartDriveRefusalCustodyV1::AfterRoute {
                        cleanup_authority,
                        detached_session,
                    },
                    failure: DirectRunProcessSessionStartDriveFailureV1::Route {
                        failure,
                        cleanup_failure,
                    },
                    cleanup_failure: None,
                })
            }
        }
    }

    pub(in crate::direct_run) fn take_selected_provider_resume_host_input_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        selected_boundary: SelectedProviderResumeBoundaryForDirectRunOwnerV1,
    ) -> Result<SelectedProviderResumeHostInputForDirectRunOwnerV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            live_session
                .take_selected_provider_resume_host_input_for_direct_run_owner_v1(selected_boundary)
        })
    }

    pub(in crate::direct_run) fn admit_selected_process_run_child_launch_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        selected: crate::SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<crate::AdmittedProcessRunChildLaunchForDirectRunOwnerV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            live_session
                .admit_selected_process_run_child_launch_for_direct_run_owner_v1(selected)
                .map_err(|fault| fault.to_string())
        })
    }

    pub(in crate::direct_run) fn admit_selected_process_load_child_launch_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        selected: crate::SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<crate::AdmittedProcessLoadChildLaunchForDirectRunOwnerV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            live_session
                .admit_selected_process_load_child_launch_for_direct_run_owner_v1(selected)
                .map_err(|fault| fault.to_string())
        })
    }

    pub(in crate::direct_run) fn admit_selected_process_invoke_child_launch_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        selected: crate::SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    ) -> Result<crate::AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            live_session
                .admit_selected_process_invoke_child_launch_for_direct_run_owner_v1(selected)
                .map_err(|fault| fault.to_string())
        })
    }

    pub(in crate::direct_run) fn apply_provider_drive_ready_result_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        provider_drive_result: crate::ProviderDriveResult,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_entry_mut(session_id, root_scope_id, |entry| {
            let (outcome, output_effect_drain_receipts) = provider_drive_result
                .commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1(
                    &mut entry.session,
                    "direct_provider_resume_ready_output",
                )
                .map_err(|error| error.to_string())?;
            let process_creation_export_readiness =
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    &entry.session,
                    "direct_provider_resume_ready_output",
                );
            let (output_drain_observations, process_output_records) =
                DirectRunEventPublicationBackendOutputDrainObservationBundle::from_provider_drive_output_effect_drain_receipts_and_process_output_records_for_direct_run_event_publication_owner_v1(
                    output_effect_drain_receipts,
                );
            append_pending_process_session_effects_for_live_process_session_registry_owner_v1(
                entry,
                output_drain_observations,
                Some(process_output_records),
            );
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness,
                "direct_provider_resume_ready_output",
            ))
        })
    }

    pub(in crate::direct_run) fn commit_process_invoke_execution_provider_ingress_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        ingress: ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            let outcome = live_session
                .commit_process_invoke_execution_provider_ingress_and_drive_for_direct_run_owner_v1(
                    ingress,
                )
                .map_err(|fault| fault.to_string())?;
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    live_session,
                    "direct_process_invoke_nominal_provider_ingress",
                ),
                "direct_process_invoke_nominal_provider_ingress",
            ))
        })
    }

    pub(in crate::direct_run) fn commit_process_run_child_provider_ingress_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        ingress: ProcessRunChildProviderIngressForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            let outcome = live_session
                .commit_process_run_child_provider_ingress_and_drive_for_direct_run_owner_v1(
                    ingress,
                )
                .map_err(|fault| fault.to_string())?;
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    live_session,
                    "direct_process_run_nominal_provider_ingress",
                ),
                "direct_process_run_nominal_provider_ingress",
            ))
        })
    }

    pub(in crate::direct_run) fn commit_process_invoke_await_execution_resume_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, DirectRunProcessInvokeLiveSessionResumeFaultV1>
    {
        let mut resume = Some(resume);
        for (value, field) in [
            (session_id, "live_process_session_id"),
            (root_scope_id, "live_process_session.expected_root_scope_id"),
        ] {
            if let Err(failure) = require_non_empty(value, field) {
                return Err(DirectRunProcessInvokeLiveSessionResumeFaultV1::Registry {
                    resume: resume
                        .take()
                        .expect("resume remains before registry admission"),
                    failure,
                });
            }
        }
        let registry_result = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                let entry = match registry.get_mut(session_id) {
                    Some(entry) => entry,
                    None => {
                        return Err(DirectRunProcessInvokeLiveSessionResumeFaultV1::Registry {
                            resume: resume.take().expect("resume remains before session lookup"),
                            failure: format!(
                                "live process-session id '{session_id}' is not present in the Rust registry"
                            ),
                        });
                    }
                };
                if entry.root_scope_id != root_scope_id {
                    return Err(DirectRunProcessInvokeLiveSessionResumeFaultV1::Registry {
                        resume: resume
                            .take()
                            .expect("resume remains before root-scope admission"),
                        failure: format!(
                            "live process-session id '{session_id}' belongs to root scope '{}', expected '{root_scope_id}'",
                            entry.root_scope_id
                        ),
                    });
                }
                let outcome = entry
                    .session
                    .commit_process_invoke_await_execution_resume_and_drive_for_direct_run_owner_v1(
                        resume
                            .take()
                            .expect("invoke resume commit consumes its product once"),
                    )
                    .map_err(DirectRunProcessInvokeLiveSessionResumeFaultV1::Transition)?;
                Ok(EngineProcessSessionRunResultV1::admitted(
                    outcome,
                    None,
                    process_creation_export_readiness_for_live_process_session_owner_v1(
                        &entry.session,
                        "direct_process_invoke_await_execution_resume",
                    ),
                    "direct_process_invoke_await_execution_resume",
                ))
            },
        );
        match registry_result {
            Ok(result) => result,
            Err(failure) => Err(DirectRunProcessInvokeLiveSessionResumeFaultV1::Registry {
                resume: resume
                    .take()
                    .expect("thread-local registry refusal retains invoke resume"),
                failure,
            }),
        }
    }

    pub(in crate::direct_run) fn commit_process_run_drive_terminal_resume_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, DirectRunProcessRunLiveSessionResumeFaultV1> {
        let mut resume = Some(resume);
        for (value, field) in [
            (session_id, "live_process_session_id"),
            (root_scope_id, "live_process_session.expected_root_scope_id"),
        ] {
            if let Err(failure) = require_non_empty(value, field) {
                return Err(DirectRunProcessRunLiveSessionResumeFaultV1::Registry {
                    resume: resume
                        .take()
                        .expect("resume remains before registry admission"),
                    failure,
                });
            }
        }
        let registry_result = with_direct_run_thread_local_cell_mut(
            &DIRECT_RUN_LIVE_PROCESS_SESSION_REGISTRY,
            "live process-session registry",
            |registry| {
                let entry = match registry.get_mut(session_id) {
                    Some(entry) => entry,
                    None => {
                        return Err(DirectRunProcessRunLiveSessionResumeFaultV1::Registry {
                            resume: resume.take().expect("resume remains before session lookup"),
                            failure: format!(
                                "live process-session id '{session_id}' is not present in the Rust registry"
                            ),
                        });
                    }
                };
                if entry.root_scope_id != root_scope_id {
                    return Err(DirectRunProcessRunLiveSessionResumeFaultV1::Registry {
                        resume: resume
                            .take()
                            .expect("resume remains before root-scope admission"),
                        failure: format!(
                            "live process-session id '{session_id}' belongs to root scope '{}', expected '{root_scope_id}'",
                            entry.root_scope_id
                        ),
                    });
                }
                let outcome = entry
                    .session
                    .commit_process_run_drive_terminal_resume_and_drive_for_direct_run_owner_v1(
                        resume
                            .take()
                            .expect("run resume commit consumes its product once"),
                    )
                    .map_err(DirectRunProcessRunLiveSessionResumeFaultV1::Transition)?;
                Ok(EngineProcessSessionRunResultV1::admitted(
                    outcome,
                    None,
                    process_creation_export_readiness_for_live_process_session_owner_v1(
                        &entry.session,
                        "direct_process_run_drive_terminal_resume",
                    ),
                    "direct_process_run_drive_terminal_resume",
                ))
            },
        );
        match registry_result {
            Ok(result) => result,
            Err(failure) => Err(DirectRunProcessRunLiveSessionResumeFaultV1::Registry {
                resume: resume
                    .take()
                    .expect("thread-local registry refusal retains run resume"),
                failure,
            }),
        }
    }

    pub(in crate::direct_run) fn commit_process_control_resume_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            let outcome = live_session
                .commit_process_control_resume_and_drive_for_direct_run_owner_v1(resume)
                .map_err(|fault| fault.to_string())?;
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    live_session,
                    "direct_process_control_resume",
                ),
                "direct_process_control_resume",
            ))
        })
    }

    pub(in crate::direct_run) fn commit_selected_host_resource_finalization_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
        selected_boundary: SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            let outcome = live_session
                .commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1(
                    provider_execution_session,
                    selected_boundary,
                )
                .map_err(|fault| fault.to_string())?;
            let process_creation_export_readiness =
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    live_session,
                    "direct_host_resource_finalization_commit",
                );
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness,
                "direct_host_resource_finalization_commit",
            ))
        })
    }

    pub(in crate::direct_run) fn drive_session_reawaken_to_public_aperture_boundary_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
    ) -> Result<DirectRunProcessSessionPublicApertureProgressProductV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            let engine_result = live_session
                .drive_reawaken_to_public_aperture_boundary()
                .map_err(|fault| {
                    format!("process_session_reawaken_public_aperture failed: {fault:?}")
                })?;
            Ok(engine_result)
        })
    }

    pub(in crate::direct_run) fn admit_public_aperture_boundary_outcome_for_live_process_session(
        session_id: &str,
        root_scope_id: &str,
        outcome: DirectRunProcessSessionRunResultProductV1,
        boundary_context: &'static str,
    ) -> Result<EngineProcessSessionRunResultV1, String> {
        with_live_process_session_mut(session_id, root_scope_id, |live_session| {
            Ok(EngineProcessSessionRunResultV1::admitted(
                outcome,
                None,
                process_creation_export_readiness_for_live_process_session_owner_v1(
                    live_session,
                    boundary_context,
                ),
                boundary_context,
            ))
        })
    }
}

#[cfg(test)]
mod terminal_registration_retirement_tests {
    use super::*;

    #[test]
    fn terminal_registration_retires_exact_session_without_clearing_root_siblings() {
        let mut registry = HashMap::from([
            ("terminal".to_owned(), "root-a".to_owned()),
            ("sibling".to_owned(), "root-a".to_owned()),
            ("foreign".to_owned(), "root-b".to_owned()),
        ]);

        assert_eq!(
            retire_terminal_registration_from_registry_for_direct_run_owner_v1(
                &mut registry,
                "terminal",
                "root-a",
                String::as_str,
            )
            .expect("the exact terminal registration must retire"),
            true
        );
        assert!(!registry.contains_key("terminal"));
        assert_eq!(registry.get("sibling").map(String::as_str), Some("root-a"));
        assert_eq!(registry.get("foreign").map(String::as_str), Some("root-b"));
    }
}
