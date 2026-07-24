use super::super::super::error::artifact_session_error;
#[path = "live_source.rs"]
mod live_source;

use self::live_source::{
    SsTestSourceWorkSetRuntimePlanLiveSourceEmitter, SsTestSourceWorkSetRuntimePlanLiveSourcePair,
    SsTestSourceWorkSetRuntimePlanLiveSourceReceiver,
    SsTestSourceWorkSetRuntimePlanLiveSourceTryBatchPoll,
};
use super::runtime_execution_domain::{
    SsRuntimeExecutionDomainOwner, SsRuntimeExecutionDomainReadyFileGraphSettlement,
    SsRuntimeExecutionDomainState,
};
use super::{
    SsCollectedTestFile, SsTestArtifactExecutionState, SsTestArtifactPlanExecutor,
    SsTestArtifactSessionAuthority, SsTestFileFailureReceipt, SsTestFileWorkStage,
};
use crate::test_runner::state::{
    SsTestExecutionGraphRuntimeFileExecutionSession,
    SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement,
    SsTestExecutionGraphRuntimeFileFailureFeedAdmission,
    SsTestExecutionGraphRuntimeFileReadyWorkAdmission,
};
use crate::test_runner::{
    SsTestProfilePhase, SsTestProfileSpanContext, SsTestRuntimePlanOwnerSession,
    ss_test_file_preparation_failure_diagnostic,
};
use crate::{SsError, SsResult};
use serde_json::{Value, json};
use ss_runtime_source_compiler_owner::SsTestSourceWorkSetReceipt;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::Duration;

#[cfg(test)]
const SOURCE_WORK_SET_RUNTIME_PLAN_BACKGROUND_LIVENESS_DEADLINE: Duration = Duration::from_secs(30);

pub(super) struct SsTestRuntimePlanOwner {
    runtime_execution_domain: SsRuntimeExecutionDomainState,
}

struct SourceWorkSetRuntimePlanOwnerSession {
    runtime_plan: SsTestRuntimePlanOwner,
}

struct SourceWorkSetRuntimePlanFeedSession<'a> {
    failure_feed_receiver: &'a mut SourceWorkSetRuntimePlanFileFailureFeedReceiver,
    live_source_emitter: &'a mut Option<SsTestSourceWorkSetRuntimePlanLiveSourceEmitter>,
    live_source_receiver: &'a mut SsTestSourceWorkSetRuntimePlanLiveSourceReceiver,
    worker_limit: NonZeroUsize,
    liveness_deadline: Duration,
}

include!("source_work_set_file_failure_feed.rs");

struct SourceWorkSetRuntimePlanArtifactDagOwner {
    executor: SsTestArtifactPlanExecutor,
    artifact_execution_state: SsTestArtifactExecutionState,
}

pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) struct SourceWorkSetRuntimePlanExecutionSession
{
    artifact_dag_owner: SourceWorkSetRuntimePlanArtifactDagOwner,
    runtime_plan_failure_feed_emitter: Option<SourceWorkSetRuntimePlanFileFailureFeedEmitter>,
    runtime_plan_failure_feed_receiver: SourceWorkSetRuntimePlanFileFailureFeedReceiver,
    runtime_plan_live_source_emitter: Option<SsTestSourceWorkSetRuntimePlanLiveSourceEmitter>,
    runtime_plan_live_source_receiver: SsTestSourceWorkSetRuntimePlanLiveSourceReceiver,
    owner_session: SourceWorkSetRuntimePlanOwnerSession,
    source_work_set_generation_id: String,
    expected_authored_file_count: usize,
    worker_limit: Option<NonZeroUsize>,
    runtime_plan_background_liveness_deadline: Duration,
    spawned_worker_child_liveness_deadline: Duration,
}

struct SourceWorkSetRuntimePlanBackgroundProgressEmitter {
    source_work_set_generation_id: String,
    sender: mpsc::SyncSender<SourceWorkSetRuntimePlanBackgroundWorkerFrame>,
    next_cursor: usize,
}

pub(in crate::test_runner) struct SourceWorkSetRuntimePlanWorkerExecutionAuthorities<'a> {
    artifact_dag_owner: &'a mut SourceWorkSetRuntimePlanArtifactDagOwner,
    session: &'a mut SsTestRuntimePlanOwnerSession,
    source_work_set_generation_id: &'a str,
    owner_session: &'a mut SourceWorkSetRuntimePlanOwnerSession,
    provider_settlement_pool: &'a mut SsRuntimeExecutionDomainOwner,
    background_progress: Option<&'a mut SourceWorkSetRuntimePlanBackgroundProgressEmitter>,
    spawned_worker_child_liveness_deadline: Duration,
}

enum SourceWorkSetRuntimePlanBackgroundWorkerFrame {
    Progress(SourceWorkSetRuntimePlanBackgroundProgress),
}

struct SourceWorkSetRuntimePlanBackgroundProgress {
    cursor: usize,
    kind: SourceWorkSetRuntimePlanBackgroundProgressKind,
    active_file: Option<String>,
    detail: Value,
}

enum SourceWorkSetRuntimePlanBackgroundProgressKind {
    WorkerStarted,
    CollectingFailureFeed,
    LiveFileBatchAdmitted,
    ReadyFileExecutionStarted,
    ReadyFileExecutionSettled,
    WaitingForFileFailure,
    LiveSourceClosed,
}

impl SourceWorkSetRuntimePlanBackgroundProgressEmitter {
    fn emit(
        &mut self,
        kind: SourceWorkSetRuntimePlanBackgroundProgressKind,
        active_file: Option<String>,
        detail: Value,
    ) -> SsResult<()> {
        let progress = SourceWorkSetRuntimePlanBackgroundProgress {
            cursor: self.next_cursor,
            kind,
            active_file,
            detail,
        };
        self.next_cursor += 1;
        self.sender
            .send(SourceWorkSetRuntimePlanBackgroundWorkerFrame::Progress(
                progress,
            ))
            .map_err(|_| {
                artifact_session_error(
                    "ss_test_source_work_set_runtime_plan_background_worker_progress_channel_closed",
                    "runtime-plan background worker could not send typed progress because the coordinator-side worker protocol channel closed before terminal settlement",
                    json!({
                        "sourceWorkSetGenerationId": self.source_work_set_generation_id,
                    }),
                )
            })
    }
}

impl<'a> SourceWorkSetRuntimePlanWorkerExecutionAuthorities<'a> {
    fn admit(
        artifact_dag_owner: &'a mut SourceWorkSetRuntimePlanArtifactDagOwner,
        session: &'a mut SsTestRuntimePlanOwnerSession,
        source_work_set_generation_id: &'a str,
        owner_session: &'a mut SourceWorkSetRuntimePlanOwnerSession,
        provider_settlement_pool: &'a mut SsRuntimeExecutionDomainOwner,
        background_progress: Option<&'a mut SourceWorkSetRuntimePlanBackgroundProgressEmitter>,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> Self {
        Self {
            artifact_dag_owner,
            session,
            source_work_set_generation_id,
            owner_session,
            provider_settlement_pool,
            background_progress,
            spawned_worker_child_liveness_deadline,
        }
    }

    pub(in crate::test_runner) fn execute_runtime_file_for_execution_graph_owner(
        &mut self,
        file: SsCollectedTestFile,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        self.owner_session
            .execute_runtime_file_for_execution_graph_owner(
                self.artifact_dag_owner,
                self.session,
                self.source_work_set_generation_id,
                self.provider_settlement_pool,
                self.background_progress.as_deref_mut(),
                file,
                self.spawned_worker_child_liveness_deadline,
            )
    }
}

impl SourceWorkSetRuntimePlanBackgroundProgress {
    fn into_diagnostic_value(self) -> Value {
        json!({
            "cursor": self.cursor,
            "kind": self.kind.as_str(),
            "activeFile": self.active_file,
            "detail": self.detail,
        })
    }
}

impl SourceWorkSetRuntimePlanBackgroundProgressKind {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::WorkerStarted => "runtime_plan_background_worker_started",
            Self::CollectingFailureFeed => "runtime_plan_background_worker_collecting_failure_feed",
            Self::LiveFileBatchAdmitted => "runtime_plan_live_file_batch_admitted",
            Self::ReadyFileExecutionStarted => "runtime_plan_ready_file_execution_started",
            Self::ReadyFileExecutionSettled => "runtime_plan_ready_file_execution_settled",
            Self::WaitingForFileFailure => "runtime_plan_waiting_for_file_failure",
            Self::LiveSourceClosed => "runtime_plan_live_source_closed",
        }
    }
}

impl SourceWorkSetRuntimePlanArtifactDagOwner {
    fn build(artifact_session_authority: SsTestArtifactSessionAuthority) -> SsResult<Self> {
        Ok(Self {
            executor: SsTestArtifactPlanExecutor::build_for_source_work_set_artifact_session(
                artifact_session_authority,
            )?,
            artifact_execution_state: SsTestArtifactExecutionState::new(),
        })
    }

    fn admit_live_file_product_emission(
        &mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
        source_work_set_generation_id: &str,
        file: SsTestFileFailureReceipt,
    ) -> SsResult<Option<SsCollectedTestFile>> {
        let (file, failed_file_dag_node) = self.executor.admit_source_work_set_failed_file(file)?;
        self.executor.collect_source_work_set_failed_file(
            session,
            source_work_set_generation_id,
            file,
            failed_file_dag_node,
        )
    }
}

impl SsTestRuntimePlanOwner {
    fn new() -> Self {
        Self {
            runtime_execution_domain: SsRuntimeExecutionDomainState::new(),
        }
    }

    fn drain_deferred_owner_lane_ready_file_leases(
        &mut self,
        artifact_execution_state: &mut SsTestArtifactExecutionState,
        provider_settlement_pool: &mut SsRuntimeExecutionDomainOwner,
        session: &mut SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        self.runtime_execution_domain
            .drain_deferred_owner_lane_ready_file_leases(
                artifact_execution_state,
                provider_settlement_pool,
                session,
                spawned_worker_child_liveness_deadline,
            )
    }

    fn require_ready_for_projection(&self) -> SsResult<()> {
        self.runtime_execution_domain.require_empty_for_projection()
    }

    fn project_settled_ready_file_node_outcomes(
        &mut self,
        executor: &mut super::SsTestArtifactPlanExecutor,
        session: &mut SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        events: Vec<Value>,
        file_order: Vec<String>,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<Value> {
        self.require_ready_for_projection()?;
        self.runtime_execution_domain.project_settled_outcomes(
            executor,
            session,
            timeout_observation,
            source_work_set_receipt,
            events,
            file_order,
            graph_settlements,
        )
    }
}

impl SourceWorkSetRuntimePlanOwnerSession {
    fn new() -> Self {
        Self {
            runtime_plan: SsTestRuntimePlanOwner::new(),
        }
    }

    fn require_ready_for_projection(&self) -> SsResult<()> {
        self.runtime_plan.require_ready_for_projection()
    }

    fn project_settled_ready_file_node_outcomes(
        &mut self,
        executor: &mut super::SsTestArtifactPlanExecutor,
        session: &mut SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        events: Vec<Value>,
        file_order: Vec<String>,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<Value> {
        self.runtime_plan.project_settled_ready_file_node_outcomes(
            executor,
            session,
            timeout_observation,
            source_work_set_receipt,
            events,
            file_order,
            graph_settlements,
        )
    }

    fn execute_runtime_file_for_execution_graph_owner(
        &mut self,
        artifact_dag_owner: &mut SourceWorkSetRuntimePlanArtifactDagOwner,
        session: &mut SsTestRuntimePlanOwnerSession,
        source_work_set_generation_id: &str,
        provider_settlement_pool: &mut SsRuntimeExecutionDomainOwner,
        mut background_progress: Option<&mut SourceWorkSetRuntimePlanBackgroundProgressEmitter>,
        file: SsCollectedTestFile,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        if let Some(progress) = background_progress.as_deref_mut() {
            progress.emit(
                SourceWorkSetRuntimePlanBackgroundProgressKind::ReadyFileExecutionStarted,
                None,
                json!({
                    "sourceWorkSetGenerationId": source_work_set_generation_id,
                    "stage": SsTestFileWorkStage::TestBodyMaterialization.as_str(),
                }),
            )?;
        }
        let result = dispatch_or_queue_ready_file_from_collected_file(
            artifact_dag_owner,
            session,
            file,
            &mut self.runtime_plan,
            Some(provider_settlement_pool),
            spawned_worker_child_liveness_deadline,
        );
        if result.is_ok()
            && let Some(progress) = background_progress.as_deref_mut()
        {
            progress.emit(
                SourceWorkSetRuntimePlanBackgroundProgressKind::ReadyFileExecutionSettled,
                None,
                json!({
                    "sourceWorkSetGenerationId": source_work_set_generation_id,
                    "stage": SsTestFileWorkStage::TestBodyMaterialization.as_str(),
                }),
            )?;
        }
        result
    }

    fn drain_deferred_owner_lane_ready_file_leases(
        &mut self,
        artifact_dag_owner: &mut SourceWorkSetRuntimePlanArtifactDagOwner,
        session: &mut SsTestRuntimePlanOwnerSession,
        provider_settlement_pool: &mut SsRuntimeExecutionDomainOwner,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        self.runtime_plan
            .drain_deferred_owner_lane_ready_file_leases(
                &mut artifact_dag_owner.artifact_execution_state,
                provider_settlement_pool,
                session,
                spawned_worker_child_liveness_deadline,
            )
    }
}

fn dispatch_or_queue_ready_file_from_collected_file(
    artifact_dag_owner: &mut SourceWorkSetRuntimePlanArtifactDagOwner,
    session: &mut SsTestRuntimePlanOwnerSession,
    file: SsCollectedTestFile,
    runtime_plan: &mut SsTestRuntimePlanOwner,
    provider_settlement_pool: Option<&mut SsRuntimeExecutionDomainOwner>,
    spawned_worker_child_liveness_deadline: Duration,
) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
    let graph_settlement = runtime_plan
        .runtime_execution_domain
        .admit_ready_file_from_collected_file(
            file,
            &mut artifact_dag_owner.artifact_execution_state,
            provider_settlement_pool,
            session,
            spawned_worker_child_liveness_deadline,
        )?;
    graph_settlement.ok_or_else(|| {
        artifact_session_error(
            "ss_test_runtime_file_graph_settlement_missing",
            "runtime-file graph execution may not settle successfully without a graph-owned ready-file settlement",
            json!({}),
        )
    })
}

impl<'a> SourceWorkSetRuntimePlanFeedSession<'a> {
    fn open(
        failure_feed_receiver: &'a mut SourceWorkSetRuntimePlanFileFailureFeedReceiver,
        live_source_emitter: &'a mut Option<SsTestSourceWorkSetRuntimePlanLiveSourceEmitter>,
        live_source_receiver: &'a mut SsTestSourceWorkSetRuntimePlanLiveSourceReceiver,
        worker_limit: NonZeroUsize,
        liveness_deadline: Duration,
    ) -> Self {
        Self {
            failure_feed_receiver,
            live_source_emitter,
            live_source_receiver,
            worker_limit,
            liveness_deadline,
        }
    }

    fn admit_next_file_failure_to_live_source(
        &mut self,
        artifact_dag_owner: &mut SourceWorkSetRuntimePlanArtifactDagOwner,
        session: &mut SsTestRuntimePlanOwnerSession,
        source_work_set_generation_id: &str,
    ) -> SsResult<bool> {
        match self
            .failure_feed_receiver
            .wait_next_file_failure(self.liveness_deadline)?
        {
            SourceWorkSetRuntimePlanFileFailureFeedPoll::Ready(file) => {
                admit_file_failure_to_live_source(
                    artifact_dag_owner,
                    session,
                    source_work_set_generation_id,
                    self.live_source_emitter,
                    file,
                )?;
                Ok(true)
            }
            SourceWorkSetRuntimePlanFileFailureFeedPoll::Closed => {
                close_live_source_once(self.live_source_emitter)?;
                Ok(false)
            }
        }
    }
}

fn admit_file_failure_to_live_source(
    artifact_dag_owner: &mut SourceWorkSetRuntimePlanArtifactDagOwner,
    session: &mut SsTestRuntimePlanOwnerSession,
    source_work_set_generation_id: &str,
    live_source_emitter: &mut Option<SsTestSourceWorkSetRuntimePlanLiveSourceEmitter>,
    file: SsTestFileFailureReceipt,
) -> SsResult<()> {
    let live_source_emitter = live_source_emitter.as_mut().ok_or_else(|| {
        artifact_session_error(
            "ss_test_source_work_set_runtime_plan_live_source_emit_after_close",
            "runtime-plan owner may collect file failures into the live source only before closeout",
            json!({
                "sourceWorkSetGenerationId": source_work_set_generation_id,
            }),
        )
    })?;
    if let Some(file) = artifact_dag_owner.admit_live_file_product_emission(
        session,
        source_work_set_generation_id,
        file,
    )? {
        live_source_emitter.admit_live_collected_file_emission(file)?;
    }
    Ok(())
}

fn close_live_source_once(
    live_source_emitter: &mut Option<SsTestSourceWorkSetRuntimePlanLiveSourceEmitter>,
) -> SsResult<()> {
    let Some(live_source_emitter) = live_source_emitter.take() else {
        return Ok(());
    };
    live_source_emitter.close_live_file_product_emission()
}

impl SourceWorkSetRuntimePlanExecutionSession {
    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn new(
        artifact_session_authority: SsTestArtifactSessionAuthority,
        total_admission_file_count: usize,
        source_work_set_generation_id: String,
        worker_limit: Option<NonZeroUsize>,
        runtime_plan_background_liveness_deadline: Duration,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Self> {
        let (runtime_plan_live_source_emitter, runtime_plan_live_source_receiver) =
            SsTestSourceWorkSetRuntimePlanLiveSourcePair::open(
                total_admission_file_count,
                source_work_set_generation_id.clone(),
            )?
            .split();
        let (runtime_plan_failure_feed_emitter, runtime_plan_failure_feed_receiver) =
            SourceWorkSetRuntimePlanFileFailureFeedPair::open().split();
        Ok(Self {
            artifact_dag_owner: SourceWorkSetRuntimePlanArtifactDagOwner::build(
                artifact_session_authority,
            )?,
            runtime_plan_failure_feed_emitter: Some(runtime_plan_failure_feed_emitter),
            runtime_plan_failure_feed_receiver,
            runtime_plan_live_source_emitter: Some(runtime_plan_live_source_emitter),
            runtime_plan_live_source_receiver,
            owner_session: SourceWorkSetRuntimePlanOwnerSession::new(),
            source_work_set_generation_id,
            expected_authored_file_count: total_admission_file_count,
            worker_limit,
            runtime_plan_background_liveness_deadline,
            spawned_worker_child_liveness_deadline,
        })
    }

    fn take_failure_feed_emitter(
        &mut self,
    ) -> SsResult<SourceWorkSetRuntimePlanFileFailureFeedEmitter> {
        self.runtime_plan_failure_feed_emitter
            .take()
            .ok_or_else(|| {
                artifact_session_error(
                    "ss_test_source_work_set_runtime_plan_failure_feed_closed_twice",
                    "runtime-plan execution session owns file-failure feed closeout and may close exactly once",
                    json!({
                        "sourceWorkSetGenerationId": self.source_work_set_generation_id,
                    }),
                )
            })
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn admit_live_file_product_emission(
        &mut self,
        file: SsTestFileFailureReceipt,
    ) -> SsResult<()> {
        let runtime_plan_failure_feed_emitter = self
            .runtime_plan_failure_feed_emitter
            .as_mut()
            .ok_or_else(|| {
                artifact_session_error(
                    "ss_test_source_work_set_runtime_plan_failure_feed_emit_after_close",
                    "runtime-plan file-failure feed emission cannot continue after graph closeout",
                    json!({
                        "sourceWorkSetGenerationId": self.source_work_set_generation_id,
                    }),
                )
            })?;
        runtime_plan_failure_feed_emitter.admit_file_failure_emission(file)
    }

    fn collect_failure_feed_until_closed(
        &mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
    ) -> SsResult<()> {
        loop {
            match self
                .runtime_plan_failure_feed_receiver
                .wait_next_file_failure(self.runtime_plan_background_liveness_deadline)?
            {
                SourceWorkSetRuntimePlanFileFailureFeedPoll::Ready(file) => {
                    admit_file_failure_to_live_source(
                        &mut self.artifact_dag_owner,
                        session,
                        &self.source_work_set_generation_id,
                        &mut self.runtime_plan_live_source_emitter,
                        file,
                    )?;
                }
                SourceWorkSetRuntimePlanFileFailureFeedPoll::Closed => {
                    close_live_source_once(&mut self.runtime_plan_live_source_emitter)?;
                    return Ok(());
                }
            }
        }
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn close_file_failure_feed_for_execution_graph_owner(
        &mut self,
        _session: &mut SsTestRuntimePlanOwnerSession,
    ) -> SsResult<()> {
        self.take_failure_feed_emitter()?
            .close_file_failure_emission()
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn admit_next_file_failure_to_live_source_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileFailureFeedAdmission> {
        match self
            .runtime_plan_failure_feed_receiver
            .try_next_file_failure()?
        {
            SourceWorkSetRuntimePlanFileFailureFeedTryPoll::Ready(file) => {
                admit_file_failure_to_live_source(
                    &mut self.artifact_dag_owner,
                    session,
                    &self.source_work_set_generation_id,
                    &mut self.runtime_plan_live_source_emitter,
                    file,
                )?;
                Ok(
                    SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Admitted {
                        admitted_count: 1,
                    },
                )
            }
            SourceWorkSetRuntimePlanFileFailureFeedTryPoll::Pending => {
                Ok(SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Pending)
            }
            SourceWorkSetRuntimePlanFileFailureFeedTryPoll::Closed => {
                close_live_source_once(&mut self.runtime_plan_live_source_emitter)?;
                Ok(SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Closed)
            }
        }
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn admit_next_runtime_file_ready_work_for_execution_graph_owner(
        &mut self,
        runtime_file_execution_session: &mut SsTestExecutionGraphRuntimeFileExecutionSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileReadyWorkAdmission> {
        let worker_limit = NonZeroUsize::new(1).expect("one runtime-file node is a nonzero batch");
        match self
            .runtime_plan_live_source_receiver
            .try_live_file_batch(worker_limit)?
        {
            SsTestSourceWorkSetRuntimePlanLiveSourceTryBatchPoll::Ready(files) => {
                let mut admitted_count = 0usize;
                for file in files {
                    admitted_count = runtime_file_execution_session
                        .admit_runtime_file_ready_work_for_execution_graph_owner(file);
                }
                Ok(SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Admitted { admitted_count })
            }
            SsTestSourceWorkSetRuntimePlanLiveSourceTryBatchPoll::Pending => {
                Ok(SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Pending)
            }
            SsTestSourceWorkSetRuntimePlanLiveSourceTryBatchPoll::Closed => {
                Ok(SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Closed)
            }
        }
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn execute_runtime_file_worker_input_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
        provider_settlement_pool: &mut SsRuntimeExecutionDomainOwner,
        worker_input: SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement> {
        let mut worker_authorities = SourceWorkSetRuntimePlanWorkerExecutionAuthorities::admit(
            &mut self.artifact_dag_owner,
            session,
            &self.source_work_set_generation_id,
            &mut self.owner_session,
            provider_settlement_pool,
            None,
            self.spawned_worker_child_liveness_deadline,
        );
        Ok(
            worker_input.execute_with_runtime_plan_worker_authorities_for_execution_graph_owner(
                &mut worker_authorities,
            ),
        )
    }

    pub(in crate::test_runner::artifact_session::work_graph::runtime_plan_owner) fn close_runtime_plan_feed_for_execution_graph_owner(
        mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        authored_file_order: Vec<String>,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<Value> {
        if authored_file_order.len() != self.expected_authored_file_count {
            return Err(artifact_session_error(
                "ss_test_source_work_set_runtime_plan_authored_file_order_incomplete",
                "runtime-plan closeout requires one owner-recorded authored source entry for every admitted selected-source or failure-feed emission",
                json!({
                    "sourceWorkSetGenerationId": self.source_work_set_generation_id,
                    "expectedFileCount": self.expected_authored_file_count,
                    "authoredFileCount": authored_file_order.len(),
                }),
            ));
        }
        self.artifact_dag_owner
            .executor
            .record_source_work_set_artifact_dag_admission_profile(session);
        let closeout_projection = self.runtime_plan_live_source_receiver.closeout_projection(
            &self.source_work_set_generation_id,
            session.invocation().list_only(),
        )?;
        if session.invocation().list_only() {
            let (events, test_count, projection_files) = closeout_projection.into_list_projection();
            self.artifact_dag_owner
                .executor
                .project_list_only_source_work_set_collection(
                    session,
                    None,
                    source_work_set_receipt,
                    projection_files,
                    events,
                    test_count,
                )
        } else {
            self.owner_session.require_ready_for_projection()?;
            let events = closeout_projection.into_execution_projection();
            if authored_file_order.is_empty() && graph_settlements.is_empty() {
                Ok(self
                    .artifact_dag_owner
                    .executor
                    .project_empty_source_work_set_run(
                        session,
                        None,
                        source_work_set_receipt,
                        events,
                    ))
            } else {
                self.owner_session.project_settled_ready_file_node_outcomes(
                    &mut self.artifact_dag_owner.executor,
                    session,
                    None,
                    source_work_set_receipt,
                    events,
                    authored_file_order,
                    graph_settlements,
                )
            }
        }
    }
}

fn shutdown_runtime_execution_domain_owner(
    runtime_execution_domain_owner: &mut SsRuntimeExecutionDomainOwner,
    session: &mut SsTestRuntimePlanOwnerSession,
) -> SsResult<()> {
    runtime_execution_domain_owner.shutdown(session)
}
