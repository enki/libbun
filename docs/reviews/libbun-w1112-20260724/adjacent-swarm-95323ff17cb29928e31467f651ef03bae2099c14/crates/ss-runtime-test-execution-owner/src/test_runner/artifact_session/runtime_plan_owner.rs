#[path = "runtime_plan_owner/body_authority_registry.rs"]
mod body_authority_registry;
#[path = "runtime_plan_owner/child_local_execution_custody.rs"]
mod child_local_execution_custody;
#[path = "runtime_plan_owner/ready_file_case_outcome.rs"]
mod ready_file_case_outcome;
use self::body_authority_registry::SsTestArtifactExecutionState;
use self::ready_file_case_outcome::{
    SsReadyFileCaseExecution, SsReadyFileExecutionSettlementOwner,
};
#[path = "runtime_plan_owner/pool_worker_child.rs"]
mod pool_worker_child;
#[path = "runtime_plan_owner/pool_worker_transport_credential.rs"]
mod pool_worker_transport_credential;
#[path = "runtime_plan_owner/runtime_file_worker_execution_lease_registry.rs"]
mod runtime_file_worker_execution_lease_registry;
pub(in crate::test_runner) use pool_worker_child::{
    SsTestCompilerWorkerPhaseObservation, SsTestPoolWorkerParentObservedFrame,
    read_child_frame_for_pool_worker_parent_v1,
};
pub(crate) use pool_worker_child::{
    encode_run_frame_for_pool_harness_observation_v1,
    encode_shutdown_frame_for_pool_harness_observation_v1,
    read_child_frame_for_pool_harness_observation_v1,
    run_pool_worker_child_session_for_pool_worker_child_owner_v1,
};
pub(in crate::test_runner) use pool_worker_transport_credential::{
    SsTestPoolWorkerParentPreparedSettlementCargo, SsTestPoolWorkerRuntimeRefusalKind,
};
#[path = "runtime_plan_owner/pool_worker_parent.rs"]
mod pool_worker_parent;
pub(in crate::test_runner) use pool_worker_parent::SsTestPoolWorkerParentPool;
#[path = "runtime_plan_owner/exact_terminal_observation_carriage.rs"]
mod exact_terminal_observation_carriage;
#[path = "runtime_plan_owner/phase_trace_projection.rs"]
mod phase_trace_projection;
#[path = "runtime_plan_owner/runtime_execution_domain.rs"]
mod runtime_execution_domain;
#[path = "runtime_plan_owner/source_work_set_artifact_dag.rs"]
mod source_work_set_artifact_dag;
#[path = "runtime_plan_owner/source_work_set_worker_execution.rs"]
mod source_work_set_worker_execution;
use super::super::preparation_failure::ss_collected_file_from_preparation_failure;
use super::super::{
    SsCollectedTestCase, SsCollectedTestFile, SsPoolDispatchedSelectedSourceTestFile,
    SsSelectedSourcePoolDispatchRefusal, SsSelectedSourceTestFile,
    SsTestFileFailureCollectionReceipt, SsTestFileFailureReceipt, SsTestFileWorkStage,
    SsTestResultStatus,
};
use crate::test_runner::state::{
    SsTestExecutionGraphRuntimeFileExecutionSession,
    SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement,
    SsTestExecutionGraphRuntimeFileFailureFeedAdmission,
    SsTestExecutionGraphRuntimeFileReadyWorkAdmission, SsTestProfileSpan,
};
use crate::test_runner::{
    SsTestProfilePhase, SsTestProfileSpanContext, SsTestRunnerSession,
    SsTestRuntimePlanOwnerSession,
};
use crate::test_runner::{
    SsTestRunEvent, SsTestRunSummary, SsTestTarget, duration_nanos_u64, scheduler_width_projection,
    ss_test_no_tests_matched_diagnostic,
};
use crate::{SsError, SsResult};
use serde_json::{Value, json};
pub(in crate::test_runner::artifact_session) use source_work_set_artifact_dag::SsTestSourceWorkSetArtifactDagNodeId;
use source_work_set_artifact_dag::{
    SsTestArtifactDagAuthority, SsTestSourceWorkSetArtifactDag,
    SsTestSourceWorkSetArtifactDagSnapshot, map_artifact_dag_result,
};
use source_work_set_worker_execution::SourceWorkSetRuntimePlanExecutionSession;
pub(in crate::test_runner) use source_work_set_worker_execution::SourceWorkSetRuntimePlanWorkerExecutionAuthorities;
use ss_runtime_source_compiler_owner::SsTestSourceWorkSetReceipt;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use runtime_execution_domain::SsRuntimeExecutionDomainOwner;
pub(in crate::test_runner) use runtime_execution_domain::{
    SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    SsRuntimeExecutionDomainCommittedFileCandidate,
    SsRuntimeExecutionDomainReadyFileGraphSettlement,
};

struct SsTestArtifactSession {
    authority: SsTestArtifactSessionAuthority,
}

#[derive(Debug)]
struct SsTestArtifactSessionAuthority {
    source_work_set_generation_id: String,
    artifact_dag_authority: SsTestArtifactSessionDagAuthority,
}

type SsTestArtifactSessionDagAuthority = SsTestArtifactDagAuthority;

struct SsTestArtifactPlanExecutor {
    artifact_session: SsTestArtifactSession,
    dag: SsTestSourceWorkSetArtifactDag,
}

impl SsSelectedSourceTestFile {
    pub(in crate::test_runner) fn dispatch_to_pool_worker_for_execution_graph_owner(
        self,
        pool: &mut SsTestPoolWorkerParentPool,
        worker_id: usize,
        ordered_extra_package_resolution_roots: &[PathBuf],
        test_name_pattern: Option<&str>,
        timeout_request: Option<&str>,
    ) -> Result<SsPoolDispatchedSelectedSourceTestFile, SsSelectedSourcePoolDispatchRefusal> {
        match pool.write_run_frame_for_execution_graph_owner_v1(
            worker_id,
            self.source_path.as_path(),
            ordered_extra_package_resolution_roots,
            test_name_pattern,
            timeout_request,
        ) {
            Ok(()) => Ok(SsPoolDispatchedSelectedSourceTestFile {
                selected_source: self,
            }),
            Err(error) => Err(SsSelectedSourcePoolDispatchRefusal {
                selected_source: self,
                error,
            }),
        }
    }
}

include!("runtime_plan_owner/source_work_set_live_feed_session.rs");

include!("runtime_plan_owner/artifact_session_authority.rs");

impl SsTestArtifactPlanExecutor {
    fn admit_source_work_set_file_result_projection(
        &mut self,
        source_path: &Path,
        stage: SsTestFileWorkStage,
        dependencies: Vec<SsTestSourceWorkSetArtifactDagNodeId>,
        status: SsTestResultStatus,
        passed: usize,
        failed: usize,
        skipped: usize,
        todo: usize,
        test_count: usize,
    ) -> SsResult<SsTestSourceWorkSetArtifactDagNodeId> {
        let authority = self
            .artifact_session
            .source_work_set_artifact_dag_authority();
        let summary =
            ss_runtime_test_runtime_plan_owner::SsRuntimePlanFileResultDagSummary::from_executed_file_for_runtime_plan_owner_v1(
                source_path.display().to_string(),
                stage,
                status,
                passed,
                failed,
                skipped,
                todo,
                test_count,
            );
        map_artifact_dag_result(self.dag.admit_file_result_projection(
            &authority,
            dependencies,
            summary,
        ))
    }

    fn admit_source_work_set_failed_file(
        &mut self,
        failed: SsTestFileFailureReceipt,
    ) -> SsResult<(
        SsTestFileFailureCollectionReceipt,
        SsTestSourceWorkSetArtifactDagNodeId,
    )> {
        let authority = self
            .artifact_session
            .source_work_set_artifact_dag_authority();
        let (failure_detail, collection) =
            failed.into_failure_detail_and_collection_receipt_for_runtime_plan_owner_v1();
        let dag_admission = map_artifact_dag_result(self.dag.admit_failed_file_result_envelope(
            &authority,
            failure_detail.into_file_result_failure_dag_admission_for_runtime_plan_owner_v1(),
        ))?;
        Ok((collection, dag_admission))
    }

    fn build_for_source_work_set_artifact_session(
        authority: SsTestArtifactSessionAuthority,
    ) -> SsResult<Self> {
        SsTestArtifactSession::new(authority).build_source_work_set_artifact_executor()
    }

    fn source_work_set_artifact_dag_snapshot(&self) -> SsTestSourceWorkSetArtifactDagSnapshot {
        self.dag.snapshot()
    }

    fn record_source_work_set_artifact_dag_admission_profile(
        &self,
        session: &mut SsTestRuntimePlanOwnerSession,
    ) {
        session.record_profile_span(
            SsTestProfilePhase::SourceWorkSetArtifactDagAdmission,
            Duration::ZERO,
            SsTestProfileSpanContext::counters(json!({
                    "authority": "owner_admitted_source_work_set_artifact_dag",
                    "artifact": "SourceWorkSetArtifactDag",
                    "sourceWorkSetArtifactDag": self.source_work_set_artifact_dag_snapshot(),
            })),
        );
    }

    fn project_executed_source_work_set_result(
        &self,
        session: &SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        mut events: Vec<Value>,
        executed_files: Vec<Value>,
        passed: usize,
        failed: usize,
        skipped: usize,
        todo: usize,
    ) -> Value {
        let status = SsTestResultStatus::from_failed_count(failed);
        events.push(
            serde_json::to_value(SsTestRunEvent::RunFinished {
                schema: "swarm.ss.test.event.v1",
                runner: "ss",
                status,
                diagnostic: None,
            })
            .expect("ss test run finish event should serialize"),
        );

        let test_file_count = executed_files.len();
        let test_count = exact_terminal_test_case_count(&executed_files);
        SsTestRunSummary::admit(
            status,
            session.invocation().reporter_mode().to_owned(),
            session.invocation().serial(),
            timeout_observation,
            session.invocation().scheduler_width(),
            session.invocation().profile_enabled(),
            session.invocation().profile_mode_str(),
            session.invocation().cloned_test_name_pattern(),
            source_work_set_receipt,
            json!(self.source_work_set_artifact_dag_snapshot()),
            test_count,
            test_file_count,
            duration_nanos_u64(session.profile_started_elapsed()),
            passed,
            failed,
            skipped,
            todo,
            events,
            executed_files,
            session.live_reporter_enabled(),
            None,
        )
        .into_value()
    }

    fn collect_source_work_set_failed_file(
        &mut self,
        session: &mut SsTestRuntimePlanOwnerSession,
        source_work_set_generation_id: &str,
        failed: SsTestFileFailureCollectionReceipt,
        failed_file_dag_node: SsTestSourceWorkSetArtifactDagNodeId,
    ) -> SsResult<Option<SsCollectedTestFile>> {
        let started = Instant::now();
        let stage = failed.stage;
        let failed_file = ss_collected_file_from_preparation_failure(failed, session.invocation());
        let Some(failed_file) = failed_file else {
            return Ok(None);
        };
        session.record_profile_span(
                    SsTestProfilePhase::TestFileCollection,
                    started.elapsed(),
                    SsTestProfileSpanContext::path_package_root(
                        failed_file.path.clone(),
                        failed_file.package_root.clone(),
                        json!({
                            "testCount": failed_file.tests.len(),
                            "registeredBodyCount": 0,
                            "status": "failed",
                            "stage": "file_result_admission",
                            "packageGraphSessionFingerprint": failed_file.package_graph_session_fingerprint.clone(),
                            "packageGraphManifestFingerprint": failed_file.package_graph_manifest_fingerprint.clone(),
                            "sourceWorkSetGenerationId": source_work_set_generation_id,
                        }),
                    ),
                );
        let node_id = self.admit_source_work_set_file_result_projection(
            Path::new(&failed_file.path),
            stage,
            vec![failed_file_dag_node],
            SsTestResultStatus::Failed,
            0,
            failed_file.tests.len(),
            0,
            0,
            failed_file.tests.len(),
        )?;
        session.record_profile_span(
                    SsTestProfilePhase::SourceWorkSetArtifactDagAdmission,
                    Duration::ZERO,
                    SsTestProfileSpanContext::path_package_root(
                        failed_file.path.clone(),
                        failed_file.package_root.clone(),
                        json!({
                            "authority": "file-local failure projected from source-work-set artifact DAG receipt by runtime-plan owner",
                            "sourceWorkSetArtifactDagNodeId": node_id,
                        }),
                    ),
                );
        Ok(Some(failed_file))
    }

    fn project_list_only_source_work_set_collection(
        &self,
        session: &SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        files: Vec<Value>,
        events: Vec<Value>,
        test_count: usize,
    ) -> SsResult<Value> {
        let test_file_count = files.len();
        Ok(json!({
            "schema": "swarm.ss.test.collection.v1",
            "runner": "ss",
            "status": "listed",
            "executionMode": "native_ss_test",
            "reporter": session.invocation().reporter_mode(),
            "serial": session.invocation().serial(),
            "timeout": timeout_observation,
            "schedulerWidth": scheduler_width_projection(session.invocation().scheduler_width()),
            "profileEnabled": session.invocation().profile_enabled(),
            "profileMode": session.invocation().profile_mode_str(),
            "events": events,
            "fileFilters": session.invocation().targets().iter().filter_map(|target| match target {
                SsTestTarget::ExplicitPath(_) => None,
                SsTestTarget::FileFilter(filter) => Some(json!(filter)),
            }).collect::<Vec<_>>(),
            "testNamePattern": session.invocation().cloned_test_name_pattern(),
            "sourceWorkSet": source_work_set_receipt,
            "sourceWorkSetArtifactDag": self.source_work_set_artifact_dag_snapshot(),
            "testFileCount": test_file_count,
            "testCaseCount": test_count,
            "testFiles": files,
            "nextRequiredSubstrate": Value::Null,
        }))
    }

    fn project_empty_source_work_set_run(
        &self,
        session: &SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        mut events: Vec<Value>,
    ) -> Value {
        let empty_configured_suite = session.allows_empty_configured_suite();
        let (status, diagnostic) = if empty_configured_suite {
            (SsTestResultStatus::Passed, None)
        } else {
            (
                SsTestResultStatus::Failed,
                Some(ss_test_no_tests_matched_diagnostic(session.invocation())),
            )
        };
        events.push(
            serde_json::to_value(SsTestRunEvent::RunFinished {
                schema: "swarm.ss.test.event.v1",
                runner: "ss",
                status,
                diagnostic: diagnostic.clone(),
            })
            .expect("ss test run finish event should serialize"),
        );
        SsTestRunSummary::admit(
            status,
            session.invocation().reporter_mode().to_owned(),
            session.invocation().serial(),
            timeout_observation,
            session.invocation().scheduler_width(),
            session.invocation().profile_enabled(),
            session.invocation().profile_mode_str(),
            session.invocation().cloned_test_name_pattern(),
            source_work_set_receipt,
            json!(self.source_work_set_artifact_dag_snapshot()),
            0,
            0,
            duration_nanos_u64(session.profile_started_elapsed()),
            0,
            0,
            0,
            0,
            events,
            Vec::new(),
            session.live_reporter_enabled(),
            diagnostic,
        )
        .into_value()
    }
}

fn exact_terminal_test_case_count(executed_files: &[Value]) -> usize {
    executed_files.iter().fold(0usize, |count, file| {
        let file_test_count = file
            .get("tests")
            .and_then(Value::as_array)
            .expect("owner-projected executed test file must carry its exact terminal tests")
            .len();
        count
            .checked_add(file_test_count)
            .expect("exact terminal test-case cardinality must not overflow")
    })
}

#[cfg(test)]
mod exact_terminal_test_case_count_tests {
    use super::exact_terminal_test_case_count;
    use serde_json::json;

    #[test]
    fn final_run_cardinality_is_derived_from_exact_terminal_file_projections() {
        let executed_files = vec![
            json!({"path": "a.test.ss", "tests": [{"testId": "a"}, {"testId": "b"}]}),
            json!({"path": "b.test.ss", "tests": [{"testId": "c"}]}),
        ];

        assert_eq!(exact_terminal_test_case_count(&executed_files), 3);
    }
}

struct SsReadyFileExecutionFile {
    _runtime_execution_domain_admission:
        runtime_execution_domain::SsReadyFileExecutionFileAdmission,
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    tests: Vec<SsCollectedTestCase>,
    preparation_terminal:
        Option<crate::test_runner::preparation_terminal::SsTestPreparationTerminalSeed>,
}

struct SsExecutedTestFile {
    _ready_file_case_outcome_admission: ready_file_case_outcome::SsExecutedTestFileAdmission,
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    status: SsTestResultStatus,
    passed: usize,
    failed: usize,
    skipped: usize,
    todo: usize,
    events: Vec<Value>,
    tests: Vec<Value>,
    exact_terminal_seeds: Vec<exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed>,
    profile_spans: Vec<SsTestProfileSpan>,
    /// Parent-side process-captured child stdio for pool-dispatched files
    /// (`{kind, text, truncated}` observation entries). Never wire cargo: the
    /// parent captures these bytes itself, so the settlement codec neither
    /// carries nor rehydrates them. Empty for in-process lanes, whose stream
    /// output already rides `events`.
    process_captured_stdio: Vec<Value>,
}

struct SsReadyFileProjectedResult {
    _executed_file_projection_admission: ready_file_case_outcome::SsExecutedTestFileAdmission,
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    status: SsTestResultStatus,
    passed: usize,
    failed: usize,
    skipped: usize,
    todo: usize,
    events: Vec<Value>,
    tests: Vec<Value>,
    exact_terminal_seeds: Vec<exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed>,
    profile_spans: Vec<SsTestProfileSpan>,
    process_captured_stdio: Vec<Value>,
}

impl SsExecutedTestFile {
    fn into_projected_result(self) -> SsReadyFileProjectedResult {
        SsReadyFileProjectedResult {
            _executed_file_projection_admission: self._ready_file_case_outcome_admission,
            path: self.path,
            package_root: self.package_root,
            package_graph_session_fingerprint: self.package_graph_session_fingerprint,
            package_graph_manifest_fingerprint: self.package_graph_manifest_fingerprint,
            source_work_set_generation_id: self.source_work_set_generation_id,
            status: self.status,
            passed: self.passed,
            failed: self.failed,
            skipped: self.skipped,
            todo: self.todo,
            events: self.events,
            tests: self.tests,
            exact_terminal_seeds: self.exact_terminal_seeds,
            profile_spans: self.profile_spans,
            process_captured_stdio: self.process_captured_stdio,
        }
    }
}

impl SsTestArtifactPlanExecutor {}
