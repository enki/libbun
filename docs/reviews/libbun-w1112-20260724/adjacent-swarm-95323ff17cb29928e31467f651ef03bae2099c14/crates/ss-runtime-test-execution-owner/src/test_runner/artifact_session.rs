#[path = "artifact_session/error.rs"]
mod error;
#[path = "artifact_session/preparation_failure.rs"]
mod preparation_failure;
#[path = "artifact_session/selected_source_dispatch_custody.rs"]
mod selected_source_dispatch_custody;
#[path = "artifact_session/work_graph.rs"]
mod work_graph;

pub(in crate::test_runner) use self::selected_source_dispatch_custody::SsTestParentSelectedSourceDispatchAdmission;
use self::work_graph::SsTestSourceWorkSetRuntimePlanLiveFeedSession;
pub(in crate::test_runner) use self::work_graph::{
    SourceWorkSetRuntimePlanWorkerExecutionAuthorities,
    SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    SsRuntimeExecutionDomainCommittedFileCandidate,
    SsRuntimeExecutionDomainReadyFileGraphSettlement,
};
pub(in crate::test_runner) use self::work_graph::{
    SsTestCompilerWorkerPhaseObservation, SsTestPoolWorkerParentObservedFrame,
    SsTestPoolWorkerParentPool, SsTestPoolWorkerParentPreparedSettlementCargo,
    SsTestPoolWorkerRuntimeRefusalKind, read_child_frame_for_pool_worker_parent_v1,
};
pub(crate) use self::work_graph::{
    encode_run_frame_for_pool_harness_observation_v1,
    encode_shutdown_frame_for_pool_harness_observation_v1,
    read_child_frame_for_pool_harness_observation_v1,
    run_pool_worker_child_session_for_pool_worker_child_owner_v1,
};
use super::{
    SsTestFileWorkStage, SsTestProfilePhase, SsTestProfileSpanContext, SsTestResultStatus,
    SsTestRunnerSession, ss_test_file_preparation_failure_diagnostic,
};
use crate::{SsError, SsResult};
use serde::Serialize;
use serde_json::Value;
use ss_runtime_source_compiler_owner::{
    SsSourceWorkSetSelectedTestSourceRuntimePlanFeedEmissionForSsTestExecutionOwnerV1,
    SsTestSourceWorkSetRuntimePlanFeedAdmissionForSsTestExecutionOwnerV1,
    test_declaration::SsCollectedTestCase,
};
use ss_runtime_test_runtime_plan_owner::SsRuntimePlanFailureDetail;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

#[path = "artifact_session/result_projection.rs"]
mod result_projection;
use self::result_projection::project_non_executing_test_case;
use super::SsTestRuntimePlanOwnerSession;
use super::state::{
    SsTestExecutionGraphRuntimeFileExecutionSession, SsTestSourceWorkSetAdmissionProfile,
    TestRunFinished,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::test_runner) struct SsCollectedTestFile {
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    tests: Vec<SsCollectedTestCase>,
    #[serde(skip_serializing)]
    preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
}

/// One selected source admitted directly from the compiler-owned source-work-set
/// feed. The graph may retain or dispatch this value, but cannot project the
/// source selector or rebuild parent-compiled runtime artifacts from it.
pub(in crate::test_runner) struct SsSelectedSourceTestFile {
    dispatch_custody: selected_source_dispatch_custody::SsTestParentSelectedSourceDispatchCustody,
    source_path: PathBuf,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
}

/// Exact selected-source custody after the authenticated pool Run frame has
/// been issued successfully. Terminal admission consumes this carrier.
pub(in crate::test_runner) struct SsPoolDispatchedSelectedSourceTestFile {
    selected_source: SsSelectedSourceTestFile,
}

#[must_use = "a failed selected-source pool dispatch retains the exact readiness for retry or cancellation"]
pub(in crate::test_runner) struct SsSelectedSourcePoolDispatchRefusal {
    selected_source: SsSelectedSourceTestFile,
    error: SsError,
}

impl SsSelectedSourcePoolDispatchRefusal {
    pub(in crate::test_runner) fn into_retry_for_execution_graph_owner(
        self,
    ) -> (SsSelectedSourceTestFile, SsError) {
        (self.selected_source, self.error)
    }
}

impl SsCollectedTestFile {
    pub(in crate::test_runner) fn take_selected_negative_terminal_custody_for_execution_graph_owner(
        &mut self,
    ) -> Option<super::preparation_terminal::SsTestPreparationTerminalSeed> {
        self.preparation_terminal.take()
    }

    pub(in crate::test_runner) fn restore_selected_negative_terminal_custody_for_execution_graph_owner(
        &mut self,
        custody: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) {
        debug_assert!(self.preparation_terminal.is_none());
        self.preparation_terminal = custody;
    }
}

struct SsTestLiveSourceWorkSetRuntimePlanEmissionSession {
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: Option<String>,
    source_work_set_generation_id: String,
    source_work_set_feed_admission:
        SsTestSourceWorkSetRuntimePlanFeedAdmissionForSsTestExecutionOwnerV1,
    source_work_set_admission_profile: Option<SsTestSourceWorkSetAdmissionProfile>,
    source_work_set_failure_count: usize,
    authored_file_order: Vec<String>,
    selected_source_ready: VecDeque<SsSelectedSourceTestFile>,
    live_feed_session: SsTestSourceWorkSetRuntimePlanLiveFeedSession,
}

pub(in crate::test_runner) struct SsTestSourceWorkSetRuntimePlanAdmissionFeed {
    emission: SsTestLiveSourceWorkSetRuntimePlanEmissionSession,
    feed_emission_authority: SsTestSourceWorkSetFeedEmissionAuthority,
}

pub(in crate::test_runner) struct SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner
{
    result: Value,
}

struct SsTestSourceWorkSetFeedEmissionAuthority {
    _private: (),
}

impl SsTestSourceWorkSetFeedEmissionAuthority {
    fn admit_source_work_set_runtime_plan_feed_owner() -> Self {
        Self { _private: () }
    }
}

impl SsTestSourceWorkSetRuntimePlanAdmissionFeed {
    pub(in crate::test_runner) fn admit(
        source_work_set_feed_admission:
            SsTestSourceWorkSetRuntimePlanFeedAdmissionForSsTestExecutionOwnerV1,
        total_file_count: usize,
        _candidate_set_observation: Option<Value>,
        worker_limit: Option<std::num::NonZeroUsize>,
        runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
        runtime_plan_background_liveness_deadline: Duration,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Self> {
        Ok(Self {
            emission: SsTestLiveSourceWorkSetRuntimePlanEmissionSession::admit(
                source_work_set_feed_admission,
                total_file_count,
                _candidate_set_observation,
                worker_limit,
                runtime_plan_owner_session,
                runtime_plan_background_liveness_deadline,
                spawned_worker_child_liveness_deadline,
            )?,
            feed_emission_authority:
                SsTestSourceWorkSetFeedEmissionAuthority::admit_source_work_set_runtime_plan_feed_owner(
                ),
        })
    }

    pub(in crate::test_runner) fn admit_source_work_set_admission_profile(
        &mut self,
        profile: SsTestSourceWorkSetAdmissionProfile,
    ) {
        let _authority = &self.feed_emission_authority;
        self.emission.source_work_set_admission_profile = Some(profile);
    }

    pub(in crate::test_runner) fn admit_source_work_set_failure_feed_emission(
        &mut self,
        path: PathBuf,
        package_root: String,
        error: SsError,
        preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) -> SsResult<()> {
        let _authority = &self.feed_emission_authority;
        self.emission
            .admit_source_work_set_failure(path, package_root, error, preparation_terminal)
    }

    pub(in crate::test_runner) fn admit_selected_source_feed_emission(
        &mut self,
        selected_source: SsTestParentSelectedSourceDispatchAdmission,
    ) {
        let _authority = &self.feed_emission_authority;
        self.emission.admit_selected_source(selected_source);
    }

    pub(in crate::test_runner) fn take_next_selected_source_readiness_for_execution_graph_owner(
        &mut self,
    ) -> Option<SsSelectedSourceTestFile> {
        self.emission.selected_source_ready.pop_front()
    }

    pub(in crate::test_runner) fn admit_file_product_failure_feed_emission(
        &mut self,
        path: PathBuf,
        package_root: String,
        stage: SsTestFileWorkStage,
        error: SsError,
        preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) -> SsResult<()> {
        let _authority = &self.feed_emission_authority;
        self.emission.admit_file_product_failure(
            path,
            package_root,
            stage,
            error,
            preparation_terminal,
        )
    }

    pub(in crate::test_runner) fn duplicate_receipt_for_file_product_admission(
        &self,
    ) -> ss_runtime_source_compiler_owner::SsTestSourceWorkSetReceipt {
        let _authority = &self.feed_emission_authority;
        self.emission
            .source_work_set_feed_admission
            .duplicate_receipt_for_source_work_set_owner_v1()
    }

    pub(in crate::test_runner) fn close_file_failure_feed_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<()> {
        self.emission
            .live_feed_session
            .close_file_failure_feed_for_execution_graph_owner(session)
    }

    pub(in crate::test_runner) fn admit_next_runtime_file_ready_work_for_execution_graph_owner(
        &mut self,
        runtime_file_execution_session: &mut SsTestExecutionGraphRuntimeFileExecutionSession,
    ) -> SsResult<super::state::SsTestExecutionGraphRuntimeFileReadyWorkAdmission> {
        self.emission
            .live_feed_session
            .admit_next_runtime_file_ready_work_for_execution_graph_owner(
                runtime_file_execution_session,
            )
    }

    pub(in crate::test_runner) fn admit_next_file_failure_to_live_source_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<super::state::SsTestExecutionGraphRuntimeFileFailureFeedAdmission> {
        self.emission
            .live_feed_session
            .admit_next_file_failure_to_live_source_for_execution_graph_owner(session)
    }

    pub(in crate::test_runner) fn execute_runtime_file_worker_input_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        worker_input: super::state::SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    ) -> SsResult<super::state::SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement> {
        self.emission
            .live_feed_session
            .execute_runtime_file_worker_input_for_execution_graph_owner(session, worker_input)
    }

    pub(in crate::test_runner) fn settle_pool_worker_loss_for_execution_graph_owner(
        &mut self,
        dispatched_source: SsPoolDispatchedSelectedSourceTestFile,
        worker_loss_fault: &serde_json::Value,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        self.emission
            .live_feed_session
            .settle_pool_worker_loss_for_execution_graph_owner(dispatched_source, worker_loss_fault)
    }

    pub(in crate::test_runner) fn commit_admitted_pool_worker_settlement_for_execution_graph_owner(
        &mut self,
        admitted: SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        self.emission
            .live_feed_session
            .commit_admitted_pool_worker_settlement_for_execution_graph_owner(admitted)
    }

    pub(in crate::test_runner) fn close_for_execution_graph_owner(
        self,
        session: &mut SsTestRunnerSession,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner> {
        let SsTestLiveSourceWorkSetRuntimePlanEmissionSession {
            live_feed_session,
            authored_file_order,
            ..
        } = self.emission;
        let result = live_feed_session.close_for_execution_graph_owner(
            session,
            authored_file_order,
            graph_settlements,
        )?;
        Ok(SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner::admit(result))
    }
}

impl SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner {
    fn admit(result: Value) -> Self {
        Self { result }
    }

    pub(in crate::test_runner) fn into_finished_for_execution_graph_owner(self) -> TestRunFinished {
        TestRunFinished::admit(self.result)
    }
}

impl SsTestLiveSourceWorkSetRuntimePlanEmissionSession {
    fn admit(
        source_work_set_feed_admission:
            SsTestSourceWorkSetRuntimePlanFeedAdmissionForSsTestExecutionOwnerV1,
        total_file_count: usize,
        _candidate_set_observation: Option<Value>,
        worker_limit: Option<std::num::NonZeroUsize>,
        runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
        runtime_plan_background_liveness_deadline: Duration,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Self> {
        let package_graph_session_fingerprint = source_work_set_feed_admission
            .package_graph_session_fingerprint_for_ss_test_execution_owner_v1()
            .to_owned();
        let package_graph_manifest_fingerprint = source_work_set_feed_admission
            .package_graph_manifest_fingerprint_for_ss_test_execution_owner_v1()
            .map(str::to_owned);
        let source_work_set_generation_id = source_work_set_feed_admission
            .source_work_set_generation_id_for_ss_test_execution_owner_v1()
            .to_owned();
        let live_feed_session = SsTestSourceWorkSetRuntimePlanLiveFeedSession::open(
            &source_work_set_generation_id,
            &package_graph_session_fingerprint,
            package_graph_manifest_fingerprint.as_deref(),
            total_file_count,
            worker_limit,
            runtime_plan_owner_session,
            source_work_set_feed_admission.duplicate_receipt_for_source_work_set_owner_v1(),
            runtime_plan_background_liveness_deadline,
            spawned_worker_child_liveness_deadline,
        )?;
        Ok(Self {
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            source_work_set_feed_admission,
            source_work_set_admission_profile: None,
            source_work_set_failure_count: 0,
            authored_file_order: Vec::with_capacity(total_file_count),
            selected_source_ready: VecDeque::new(),
            live_feed_session,
        })
    }

    fn admit_selected_source(
        &mut self,
        selected_source: SsTestParentSelectedSourceDispatchAdmission,
    ) {
        let (selected_source, dispatch_custody) =
            selected_source.consume_into_runtime_plan_admission_for_execution_graph_owner_v1();
        let (source_path, package_root) = selected_source
            .consume_into_runtime_plan_file_emission_input_for_ss_test_execution_owner_v1();
        self.authored_file_order
            .push(source_path.display().to_string());
        self.selected_source_ready
            .push_back(SsSelectedSourceTestFile {
                dispatch_custody,
                source_path,
                package_root,
                package_graph_session_fingerprint: self.package_graph_session_fingerprint.clone(),
                package_graph_manifest_fingerprint: self
                    .package_graph_manifest_fingerprint
                    .clone()
                    .unwrap_or_else(|| "<not-admitted>".to_owned()),
                source_work_set_generation_id: self.source_work_set_generation_id.clone(),
            });
    }

    fn admit_source_work_set_failure(
        &mut self,
        path: PathBuf,
        package_root: String,
        error: SsError,
        preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) -> SsResult<()> {
        let authored_source_path = path.display().to_string();
        let diagnostic = ss_test_file_preparation_failure_diagnostic(
            &path,
            &package_root,
            &self.package_graph_session_fingerprint,
            self.package_graph_manifest_fingerprint
                .as_deref()
                .unwrap_or("<not-admitted>"),
            &self.source_work_set_generation_id,
            SsTestFileWorkStage::SourceWorkSetAdmission,
            &error,
        );
        let failure_detail =
            SsRuntimePlanFailureDetail::admit_file_preparation_failure_for_runtime_plan_owner_v1(
                &path,
                SsTestFileWorkStage::SourceWorkSetAdmission,
                &package_root,
                &self.package_graph_session_fingerprint,
                self.package_graph_manifest_fingerprint
                    .as_deref()
                    .unwrap_or("<not-admitted>"),
                &self.source_work_set_generation_id,
                &error,
            );
        self.source_work_set_failure_count += 1;
        self.live_feed_session
            .admit_live_file_product_emission(SsTestFileFailureReceipt {
                path,
                package_root,
                package_graph_session_fingerprint: self.package_graph_session_fingerprint.clone(),
                package_graph_manifest_fingerprint: self
                    .package_graph_manifest_fingerprint
                    .clone()
                    .unwrap_or_else(|| "<not-admitted>".to_owned()),
                source_work_set_generation_id: self.source_work_set_generation_id.clone(),
                stage: SsTestFileWorkStage::SourceWorkSetAdmission,
                diagnostic,
                failure_detail,
                preparation_terminal,
            })?;
        self.authored_file_order.push(authored_source_path);
        Ok(())
    }

    fn admit_file_product_failure(
        &mut self,
        path: PathBuf,
        package_root: String,
        stage: SsTestFileWorkStage,
        error: SsError,
        preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) -> SsResult<()> {
        let authored_source_path = path.display().to_string();
        let diagnostic = ss_test_file_preparation_failure_diagnostic(
            &path,
            &package_root,
            &self.package_graph_session_fingerprint,
            self.package_graph_manifest_fingerprint
                .as_deref()
                .unwrap_or("<not-admitted>"),
            &self.source_work_set_generation_id,
            stage,
            &error,
        );
        let failure_detail =
            SsRuntimePlanFailureDetail::admit_file_preparation_failure_for_runtime_plan_owner_v1(
                &path,
                stage,
                &package_root,
                &self.package_graph_session_fingerprint,
                self.package_graph_manifest_fingerprint
                    .as_deref()
                    .unwrap_or("<not-admitted>"),
                &self.source_work_set_generation_id,
                &error,
            );
        self.live_feed_session
            .admit_live_file_product_emission(SsTestFileFailureReceipt {
                path,
                package_root,
                package_graph_session_fingerprint: self.package_graph_session_fingerprint.clone(),
                package_graph_manifest_fingerprint: self
                    .package_graph_manifest_fingerprint
                    .clone()
                    .unwrap_or_else(|| "<not-admitted>".to_owned()),
                source_work_set_generation_id: self.source_work_set_generation_id.clone(),
                stage,
                diagnostic,
                failure_detail,
                preparation_terminal,
            })?;
        self.authored_file_order.push(authored_source_path);
        Ok(())
    }
}

struct SsTestFileFailureReceipt {
    path: PathBuf,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    stage: SsTestFileWorkStage,
    diagnostic: Value,
    failure_detail: SsRuntimePlanFailureDetail,
    preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
}

struct SsTestFileFailureCollectionReceipt {
    path: PathBuf,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    stage: SsTestFileWorkStage,
    diagnostic: Value,
    preparation_terminal: Option<super::preparation_terminal::SsTestPreparationTerminalSeed>,
}

impl SsTestFileFailureReceipt {
    fn into_failure_detail_and_collection_receipt_for_runtime_plan_owner_v1(
        self,
    ) -> (
        SsRuntimePlanFailureDetail,
        SsTestFileFailureCollectionReceipt,
    ) {
        let Self {
            path,
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            stage,
            diagnostic,
            failure_detail,
            preparation_terminal,
        } = self;
        (
            failure_detail,
            SsTestFileFailureCollectionReceipt {
                path,
                package_root,
                package_graph_session_fingerprint,
                package_graph_manifest_fingerprint,
                source_work_set_generation_id,
                stage,
                diagnostic,
                preparation_terminal,
            },
        )
    }
}
