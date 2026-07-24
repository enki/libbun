use crate::session::execution_kernel::executable_value::{
    SessionRuntimeMaterializedActivityInputPayloadProduct,
    SessionRuntimeMaterializedSwarmEventPublishPayloadProduct,
};
use swarmvm_host_sql_authority::live_primitives::LivePrimitiveTaskTerminalSelectedOutputForLivePrimitiveOwnerV1;
use swarmvm_isa_types::HostActivityResultMode;
use swarmvm_isa_types::authority_ids as swarmvm_isa;
use swarmvm_isa_types::authority_ids::{ActivityAttemptId, ActorRequestId};
use swarmvm_runtime_types::{SemanticTypeRefValue, VmBoundaryValue};

use super::checkpoint_manifest::CheckpointManifest;
use super::descriptors::{PayloadDescriptor, PayloadSizeClass};
pub use super::effect_ledger::PendingActivityEffectFrame;
use super::effect_ledger::{
    EffectDescriptor, EffectHandle, EffectLedger, EffectRecord, EffectRef, EffectResumeFrame,
    EffectState, EventAppendPublicationEffectRecordInput,
    PreflightedProviderResumeHostInputEffectRecordForSessionWorkRuntimeOwnerV1,
    PublicApertureLiveExecutionFrontier, effect_ref_from_handle, effect_ref_matches_handle,
};
use super::ids::{EffectKind, RuntimeHandleGeneration, WorkId};
use super::payload_store::{
    ActivityInputPayloadProduct, ActorRequestReadyErrPayloadProduct,
    ActorRequestReadyOkPayloadProduct, ActorTurnPayloadProduct,
    EventAppendPublicationPayloadProduct, PayloadHandle, PayloadRetentionClass, PayloadStore,
};
use super::prepared_store::PreparedRuntimeStore;
use super::scheduler_queues::{
    SchedulerCommitSequence, SchedulerExecutionProfileSelection,
    SchedulerIncidentalExecutionSchedule, SchedulerQueues, SchedulerWorkAdmissionClass,
    SchedulerWorkAdmissionFault, SchedulerWorkExecutionFault, SchedulerWorkTerminalInput,
    SelectedSchedulerRunnableWork,
};

mod types;
use super::payload_store::ActorCheckpointBodyPayloadProduct;
pub use types::EventAppendApplicationCompletionTicket;
pub(crate) use types::SelectedProviderResumeRouteForDirectRunOwnerV1;
pub(crate) use types::{
    ActorCheckpointBodyWorkFrame, ActorRequestReadyErrWorkFrame, ActorRequestReadyOkWorkFrame,
    ActorTurnWorkFrame, ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1,
    ProviderResumeWorkFrame, StoreOwnedWorkSettlementFault, WorkAuthority, WorkCreationCause,
    WorkFrame, WorkHandle, WorkKind, WorkRecord, WorkRef, WorkRetentionClass, WorkStatus,
};
pub(crate) use types::{ActorRequestReadyErrWorkHandle, ActorRequestReadyOkWorkHandle};
pub use types::{
    AdmittedProcessInvokeChildLaunchForDirectRunOwnerV1,
    AdmittedProcessLoadChildLaunchForDirectRunOwnerV1,
    AdmittedProcessRunChildLaunchForDirectRunOwnerV1,
    KernelInternalPlainOutputSettlementPreflightRefusalForDirectRunOwnerV1,
    PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    ProcessChildLaunchInputAdmissionFaultForDirectRunOwnerV1,
    SelectedEventAppendPublicationAppendInputForDurableExecutionOwnerV1,
    SelectedEventAppendPublicationWorkForDirectRunOwnerV1,
    SelectedKernelInternalProviderOutputSettlementAuthorityForDirectRunOwnerV1,
    SelectedKernelInternalProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessInvokeExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessLoadExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRestoreProgramAuthorityForDirectRunOwnerV1,
    SelectedProcessRestoreProviderResumeInputForDirectRunOwnerV1,
    SelectedProcessRunExactStaticChildProviderResumeInputForDirectRunOwnerV1,
    SelectedProviderResumeHostInputForDirectRunOwnerV1,
};
pub(crate) use types::{
    ProcessRestoreInputAdmissionFaultForDirectRunOwnerV1,
    ProcessRestoreInputAdmissionRefusalForDirectRunOwnerV1,
    ProcessRestoreProgramOpenPlanJoinForDirectRunOwnerV1,
    SelectedProcessRestoreExecutionInputForDirectRunOwnerV1,
};
#[cfg(test)]
#[path = "work_store_tests.rs"]
mod work_store_tests;

#[derive(Default)]
pub(crate) struct WorkStore {
    records: std::collections::BTreeMap<WorkId, WorkRecord>,
    consumed_actor_request_ready_result_work:
        std::collections::BTreeMap<WorkId, RuntimeHandleGeneration>,
    next_scheduler_runnable_sequence: u64,
    next_checkpoint_actor_restore_sequence: u64,
}

struct SchedulerExecutingStoreWork {
    work_id: WorkId,
    record: WorkRecord,
    selected: SelectedSchedulerRunnableWork,
    terminal: SchedulerWorkTerminalInput,
}

struct SchedulerCompletedStoreWork {
    work_id: WorkId,
    record: WorkRecord,
    selected: SelectedSchedulerRunnableWork,
    terminal: SchedulerWorkTerminalInput,
}

struct PreflightedProviderResumeHostInputWorkFrameForSessionWorkRuntimeOwnerV1 {
    work_id: WorkId,
}

impl WorkStore {
    pub(crate) fn record_count(&self) -> usize {
        self.records.len()
    }

    pub(crate) fn contains_handle(&self, handle: &WorkHandle) -> bool {
        self.records
            .get(&handle.id)
            .is_some_and(|record| record.handle.matches_session_work_runtime_owner_v1(handle))
    }
}

pub struct WorkRuntimeStores {
    prepared: PreparedRuntimeStore,
    payloads: PayloadStore,
    work: WorkStore,
    effects: EffectLedger,
    scheduler: SchedulerQueues,
}

/// Opens the work-runtime owner's empty store state for one process session.
///
/// The returned store is opaque. Executable-image state never crosses this
/// owner boundary, and callers receive no raw store parts or insertion surface.
pub fn open_work_runtime_stores_for_session_execution_kernel_owner_v1() -> WorkRuntimeStores {
    WorkRuntimeStores::new_for_swarmvm_session_runtime_open_owner_v1()
}

#[cfg(test)]
impl Default for WorkRuntimeStores {
    fn default() -> Self {
        Self {
            prepared: PreparedRuntimeStore::default(),
            payloads: PayloadStore::default(),
            work: WorkStore::default(),
            effects: EffectLedger::default(),
            scheduler: SchedulerQueues::default(),
        }
    }
}

include!("work_runtime_stores_impl.rs");
