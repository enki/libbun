#[path = "owner_lane_ready_file_execution.rs"]
mod owner_lane_ready_file_execution;
#[path = "ready_file_execution.rs"]
mod ready_file_execution;
#[cfg(test)]
pub(super) use ready_file_execution::exact_terminal_seed_for_non_executing_case;
#[path = "ready_file_node_outcome.rs"]
mod ready_file_node_outcome;
#[path = "runtime_execution_domain/runtime_file_settlement.rs"]
mod runtime_file_settlement;

use self::ready_file_node_outcome::{
    SsTestReadyFileNodeOutcome, SsTestReadyFileResultProjectionDependencies,
};
use self::runtime_file_settlement::SsRuntimeExecutionDomainPreparedWorkerFileSettlement;
pub(super) use self::runtime_file_settlement::SsRuntimeExecutionDomainWorkerFileSettlement;
use super::body_authority_registry::{
    SsTestProviderSettlementPool, TestReadyFileBodyDispatchAuthority,
};
use super::{SsCollectedTestFile, SsReadyFileExecutionFile};
use crate::test_runner::{SsTestFileWorkStage, SsTestProfilePhase, SsTestProfileSpanContext};
use crate::{SsError, SsResult};
use owner_lane_ready_file_execution::SsTestOwnerLaneReadyFileWorkItem;
use serde_json::{Value, json};
use ss_runtime_source_compiler_owner::SsTestSourceWorkSetReceipt;
use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

struct SsRuntimeExecutionDomainReadyFileAdmission {
    kind: SsRuntimeExecutionDomainReadyFileAdmissionKind,
}

struct AdmittedSsTestWorkItem {
    path: String,
    projection_dependencies: SsTestReadyFileResultProjectionDependencies,
    file: SsReadyFileExecutionFile,
}

enum SsRuntimeExecutionDomainReadyFileAdmissionKind {
    OwnerOutcome {
        settlement: SsRuntimeExecutionDomainWorkerFileSettlement,
    },
    DeferredOwnerLane(SsTestOwnerLaneReadyFileWorkItem),
}

pub(super) struct SsRuntimeExecutionDomainOwner {
    provider_settlement_pool: SsTestProviderSettlementPool,
}

pub(super) struct SsRuntimeExecutionDomainState {
    deferred_owner_lane_ready_files: SsRuntimeExecutionDomainOwnerLaneQueue,
}

pub(super) struct SsReadyFileExecutionFileAdmission {
    _private: (),
}

pub(in crate::test_runner) struct SsRuntimeExecutionDomainReadyFileGraphSettlement {
    kind: SsRuntimeExecutionDomainReadyFileGraphSettlementKind,
}

/// One-shot parent settlement minted by consuming the exact dispatched
/// selected-source readiness through authenticated Running-slot staging.
pub(in crate::test_runner) struct SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
    dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    preflighted: SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement,
}

struct SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement {
    outcome: SsTestReadyFileNodeOutcome,
    prepared_settlement: SsRuntimeExecutionDomainPreparedWorkerFileSettlement,
}

pub(in crate::test_runner) struct SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal {
    pub(in crate::test_runner) dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    pub(in crate::test_runner) error: SsError,
}

impl std::fmt::Debug for SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal")
            .field("error", &self.error.to_string())
            .finish_non_exhaustive()
    }
}

impl SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
    pub(super) fn cancel_for_parent_owner_v1(
        self,
    ) -> super::SsPoolDispatchedSelectedSourceTestFile {
        self.dispatched_source
    }
}

/// Move-only normalized runtime-file cargo minted by consuming a graph
/// settlement only after the graph has committed that file.
pub(in crate::test_runner) struct SsRuntimeExecutionDomainCommittedFileCandidate {
    projected_result: super::SsReadyFileProjectedResult,
    exact_terminal_observations:
        super::exact_terminal_observation_carriage::SsTestCommittedFileTerminalObservations,
    projection_dependencies: SsTestReadyFileResultProjectionDependencies,
}

enum SsRuntimeExecutionDomainReadyFileGraphSettlementKind {
    OwnerSettlement {
        settlement: SsRuntimeExecutionDomainWorkerFileSettlement,
    },
    AdjudicatedCommittedFile {
        candidate: SsRuntimeExecutionDomainCommittedFileCandidate,
    },
}

pub(super) struct SsRuntimeExecutionDomainBodyAuthorityOwner<'a> {
    artifact_execution_state: &'a mut super::SsTestArtifactExecutionState,
}

struct SsRuntimeExecutionDomainOwnerLaneQueue {
    deferred: VecDeque<SsTestOwnerLaneReadyFileWorkItem>,
}

impl SsRuntimeExecutionDomainOwner {
    pub(super) fn new() -> Self {
        Self {
            provider_settlement_pool: SsTestProviderSettlementPool::new(),
        }
    }

    fn execute_owner_lane_ready_file(
        &mut self,
        work_item: SsTestOwnerLaneReadyFileWorkItem,
        artifact_execution_state: &mut super::SsTestArtifactExecutionState,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> crate::SsResult<SsRuntimeExecutionDomainWorkerFileSettlement> {
        let mut body_authority_owner =
            SsRuntimeExecutionDomainBodyAuthorityOwner::new(artifact_execution_state);
        self.execute_owner_lane_ready_file_with_body_authority(
            work_item,
            &mut body_authority_owner,
            session,
            spawned_worker_child_liveness_deadline,
        )
    }

    fn execute_owner_lane_ready_file_with_body_authority(
        &mut self,
        work_item: SsTestOwnerLaneReadyFileWorkItem,
        body_authority_owner: &mut SsRuntimeExecutionDomainBodyAuthorityOwner<'_>,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> crate::SsResult<SsRuntimeExecutionDomainWorkerFileSettlement> {
        let _ = spawned_worker_child_liveness_deadline;
        work_item.execute(
            body_authority_owner,
            &mut self.provider_settlement_pool,
            session,
        )
    }

    pub(super) fn shutdown(
        &mut self,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
    ) -> crate::SsResult<()> {
        let started = Instant::now();
        let shutdown_result = self.provider_settlement_pool.shutdown();
        session.record_profile_span(
            SsTestProfilePhase::ProviderHostPoolShutdown,
            started.elapsed(),
            SsTestProfileSpanContext::counters(json!({
                "status": if shutdown_result.is_ok() { "shutdown" } else { "failed" },
            })),
        );
        shutdown_result
    }
}

impl SsReadyFileExecutionFileAdmission {
    fn runtime_domain() -> Self {
        Self { _private: () }
    }
}

impl SsRuntimeExecutionDomainState {
    pub(super) fn new() -> Self {
        Self {
            deferred_owner_lane_ready_files: SsRuntimeExecutionDomainOwnerLaneQueue::new(),
        }
    }

    pub(super) fn admit_ready_file_from_collected_file(
        &mut self,
        file: SsCollectedTestFile,
        artifact_execution_state: &mut super::SsTestArtifactExecutionState,
        provider_settlement_pool: Option<&mut SsRuntimeExecutionDomainOwner>,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Option<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        let node = AdmittedSsTestWorkItem::admit_ready_file(file)?;
        self.admit_ready_file(
            node,
            artifact_execution_state,
            provider_settlement_pool,
            session,
            spawned_worker_child_liveness_deadline,
        )
    }

    fn admit_ready_file(
        &mut self,
        node: AdmittedSsTestWorkItem,
        artifact_execution_state: &mut super::SsTestArtifactExecutionState,
        provider_settlement_pool: Option<&mut SsRuntimeExecutionDomainOwner>,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Option<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        let admission = admit_ready_file_execution_domain(
            node,
            artifact_execution_state,
            provider_settlement_pool,
            session,
            spawned_worker_child_liveness_deadline,
        )?;
        self.admit_ready_file_admission(admission)
    }

    pub(super) fn drain_deferred_owner_lane_ready_file_leases(
        &mut self,
        artifact_execution_state: &mut super::SsTestArtifactExecutionState,
        provider_settlement_pool: &mut SsRuntimeExecutionDomainOwner,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        let mut graph_settlements = Vec::new();
        for settlement in self.deferred_owner_lane_ready_files.drain(
            artifact_execution_state,
            provider_settlement_pool,
            session,
            spawned_worker_child_liveness_deadline,
        )? {
            graph_settlements.push(self.admit_ready_file_owner_settlement(settlement));
        }
        Ok(graph_settlements)
    }

    pub(super) fn require_empty_for_projection(&self) -> SsResult<()> {
        if !self.deferred_owner_lane_ready_files.is_empty() {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.work_set_runtime_plan_fault.v1",
                    "code": "ss_test_work_set_runtime_plan_owner_lane_ready_file_undrained",
                    "reason": "runtime-plan projection requires provider-affine owner-lane ready-file leases to settle through the provider owner lane before deterministic projection",
                    "remainingOwnerLaneReadyFileLeaseCount": self.deferred_owner_lane_ready_files.len(),
                })
                .to_string(),
            ));
        }
        Ok(())
    }

    pub(super) fn project_settled_outcomes(
        &mut self,
        executor: &mut super::SsTestArtifactPlanExecutor,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        timeout_observation: Option<String>,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        events: Vec<Value>,
        file_order: Vec<String>,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<Value> {
        self.require_empty_for_projection()?;
        project_ready_file_node_outcomes_from_runtime_execution_domain(
            executor,
            session,
            timeout_observation,
            source_work_set_receipt,
            events,
            file_order,
            graph_settlements,
        )
    }

    fn admit_ready_file_admission(
        &mut self,
        admission: SsRuntimeExecutionDomainReadyFileAdmission,
    ) -> SsResult<Option<SsRuntimeExecutionDomainReadyFileGraphSettlement>> {
        match admission.kind {
            SsRuntimeExecutionDomainReadyFileAdmissionKind::OwnerOutcome { settlement } => {
                Ok(Some(self.admit_ready_file_owner_settlement(settlement)))
            }
            SsRuntimeExecutionDomainReadyFileAdmissionKind::DeferredOwnerLane(admission) => {
                self.deferred_owner_lane_ready_files.defer(admission);
                Ok(None)
            }
        }
    }

    fn admit_ready_file_owner_settlement(
        &mut self,
        settlement: SsRuntimeExecutionDomainWorkerFileSettlement,
    ) -> SsRuntimeExecutionDomainReadyFileGraphSettlement {
        SsRuntimeExecutionDomainReadyFileGraphSettlement::owner_settlement(settlement)
    }
}

impl AdmittedSsTestWorkItem {
    fn path(&self) -> &str {
        &self.path
    }

    fn admit_ready_file(file: SsCollectedTestFile) -> SsResult<Self> {
        let projection_dependencies = SsTestReadyFileResultProjectionDependencies::empty();
        let path = file.path.clone();
        Ok(Self {
            path,
            projection_dependencies,
            file: SsReadyFileExecutionFile {
                _runtime_execution_domain_admission:
                    SsReadyFileExecutionFileAdmission::runtime_domain(),
                path: file.path,
                package_root: file.package_root,
                package_graph_session_fingerprint: file.package_graph_session_fingerprint,
                package_graph_manifest_fingerprint: file.package_graph_manifest_fingerprint,
                source_work_set_generation_id: file.source_work_set_generation_id,
                tests: file.tests,
                preparation_terminal: file.preparation_terminal,
            },
        })
    }

    fn into_ready_file_execution_parts(
        self,
    ) -> (
        String,
        SsTestReadyFileResultProjectionDependencies,
        SsReadyFileExecutionFile,
    ) {
        (self.path, self.projection_dependencies, self.file)
    }
}

impl SsRuntimeExecutionDomainReadyFileGraphSettlement {
    fn owner_settlement(settlement: SsRuntimeExecutionDomainWorkerFileSettlement) -> Self {
        Self {
            kind: SsRuntimeExecutionDomainReadyFileGraphSettlementKind::OwnerSettlement {
                settlement,
            },
        }
    }

    pub(in crate::test_runner) fn into_committed_file_candidate_after_graph_commit(
        self,
        _settlement_frame_sink: Option<
            &crate::test_runner::executed_file_frame_sink::SsTestExecutedFileSettlementFrameSink,
        >,
    ) -> SsResult<SsRuntimeExecutionDomainCommittedFileCandidate> {
        match self.kind {
            SsRuntimeExecutionDomainReadyFileGraphSettlementKind::OwnerSettlement {
                settlement,
            } => Ok(
                SsRuntimeExecutionDomainCommittedFileCandidate::from_owner_projection_parts(
                    settlement.into_normalized_projection_parts_after_graph_commit_for_runtime_execution_domain_owner_v1(),
                ),
            ),
            SsRuntimeExecutionDomainReadyFileGraphSettlementKind::AdjudicatedCommittedFile {
                candidate,
            } => Ok(candidate),
        }
    }

    pub(in crate::test_runner) fn retain_adjudicated_committed_file_after_publication(
        candidate: SsRuntimeExecutionDomainCommittedFileCandidate,
    ) -> Self {
        Self {
            kind: SsRuntimeExecutionDomainReadyFileGraphSettlementKind::AdjudicatedCommittedFile {
                candidate,
            },
        }
    }

    #[cfg(test)]
    pub(in crate::test_runner) fn inert_fixture_for_execution_graph_owner_v1(
        path: impl Into<String>,
    ) -> SsResult<Self> {
        let path = path.into();
        let outcome = SsTestReadyFileNodeOutcome::admit_owner_lane_execution(
            super::SsExecutedTestFile {
                _ready_file_case_outcome_admission:
                    super::ready_file_case_outcome::SsExecutedTestFileAdmission::admitted_executed_file_settlement_frame(),
                path: path.clone(),
                package_root: "/tmp".to_owned(),
                package_graph_session_fingerprint: "fixture-package-session".to_owned(),
                package_graph_manifest_fingerprint: "fixture-package-manifest".to_owned(),
                source_work_set_generation_id: "fixture-source-work-set".to_owned(),
                status: crate::test_runner::SsTestResultStatus::Passed,
                passed: 0,
                failed: 0,
                skipped: 0,
                todo: 0,
                events: Vec::new(),
                tests: Vec::new(),
                exact_terminal_seeds: Vec::new(),
                profile_spans: Vec::new(),
                process_captured_stdio: Vec::new(),
            },
            SsTestReadyFileResultProjectionDependencies::empty(),
        );
        Ok(Self::owner_settlement(
            SsRuntimeExecutionDomainWorkerFileSettlement::admit_owner_lane_execution(
                path, outcome,
            )?,
        ))
    }
}

#[cfg(test)]
#[path = "runtime_execution_domain/streaming_acceptance_fixture.rs"]
mod streaming_acceptance_fixture;

impl SsRuntimeExecutionDomainCommittedFileCandidate {
    fn from_owner_projection_parts(
        (mut projected_result, projection_dependencies): (
            super::SsReadyFileProjectedResult,
            SsTestReadyFileResultProjectionDependencies,
        ),
    ) -> Self {
        let exact_terminal_observations =
            super::exact_terminal_observation_carriage::committed_file_terminal_observations_for_run_plan_owner_v1(
                &projected_result.path,
                std::mem::take(&mut projected_result.exact_terminal_seeds),
            );
        Self {
            projected_result,
            exact_terminal_observations,
            projection_dependencies,
        }
    }

    pub(in crate::test_runner) fn source_path(&self) -> &str {
        &self.projected_result.path
    }

    /// Consumes the exact committed candidate through the one-run expectation
    /// ledger while retaining all runtime-owned metadata and graph projection
    /// dependencies. There is deliberately no generic callback/parts surface:
    /// only this named transition may rewrite observable fields after graph
    /// commit and before publication.
    pub(in crate::test_runner) fn adjudicate_for_test_run_plan_owner_v1(
        self,
        run_plan: &mut ss_runtime_test_plan_owner::SsTestRunPlan,
    ) -> SsResult<Self> {
        let Self {
            projected_result,
            mut exact_terminal_observations,
            projection_dependencies,
        } = self;
        let super::SsReadyFileProjectedResult {
            _executed_file_projection_admission,
            path,
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            status,
            passed,
            failed,
            skipped,
            todo,
            events,
            tests,
            exact_terminal_seeds,
            profile_spans,
            process_captured_stdio,
        } = projected_result;
        let _raw_status = status;
        let canonical_path = PathBuf::from(&path);
        let exact_expectations = run_plan
            .take_exact_expectations_for_file_for_ss_test_execution_owner_v1(canonical_path.clone())
            .map_err(|error| SsError::Cli(error.to_string()))?;
        let exact_decisions = exact_expectations
            .into_iter()
            .map(|expectation| {
                exact_terminal_observations.adjudicate_exact_expectation_for_run_plan_owner_v1(
                    expectation.classify_identity_for_ss_test_execution_owner_v1(),
                )
            })
            .collect();
        let adjudicated = run_plan
            .adjudicate_file_with_exact_terminal_decisions_for_ss_test_execution_owner_v1(
                canonical_path,
                tests,
                events,
                passed,
                failed,
                skipped,
                todo,
                exact_decisions,
            )
            .map_err(|error| SsError::Cli(error.to_string()))?;
        let (canonical_path, tests, events, passed, failed, skipped, todo, counts_as_failed_file) =
            adjudicated.into_parts_for_ss_test_execution_owner_v1();
        let adjudicated_path = canonical_path.display().to_string();
        if adjudicated_path != path {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_committed_file_fault.v1",
                    "code": "ss_test_committed_file_adjudication_path_mismatch",
                    "reason": "run-plan expectation adjudication must return the exact canonical path retained by the committed runtime-file candidate",
                    "retainedPath": path,
                    "adjudicatedPath": adjudicated_path,
                })
                .to_string(),
            ));
        }
        Ok(Self {
            projected_result: super::SsReadyFileProjectedResult {
                _executed_file_projection_admission,
                path,
                package_root,
                package_graph_session_fingerprint,
                package_graph_manifest_fingerprint,
                source_work_set_generation_id,
                status: if counts_as_failed_file {
                    crate::test_runner::SsTestResultStatus::Failed
                } else {
                    crate::test_runner::SsTestResultStatus::Passed
                },
                passed,
                failed,
                skipped,
                todo,
                events,
                tests,
                exact_terminal_seeds,
                profile_spans,
                process_captured_stdio,
            },
            exact_terminal_observations,
            projection_dependencies,
        })
    }

    /// Derives the exact post-commit live cargo without owning reporter
    /// authority. Parent-captured pool stdio is ordered before in-result
    /// stdout/stderr, followed separately by typed settlement events.
    pub(in crate::test_runner) fn observation_events_for_committed_file_v1(
        &self,
    ) -> (Vec<Value>, Vec<Value>) {
        let output_events = self
            .projected_result
            .process_captured_stdio
            .iter()
            .chain(self.projected_result.events.iter())
            .filter(|event| {
                is_output_observation_event_for_runtime_execution_domain_owner_v1(event)
            })
            .cloned()
            .collect();
        let typed_settlement_events = self
            .projected_result
            .events
            .iter()
            .filter(|event| is_typed_settlement_event_for_runtime_execution_domain_owner_v1(event))
            .cloned()
            .collect();
        (output_events, typed_settlement_events)
    }

    fn into_projection_parts_for_runtime_execution_domain_owner_v1(
        self,
    ) -> (
        super::SsReadyFileProjectedResult,
        SsTestReadyFileResultProjectionDependencies,
    ) {
        (self.projected_result, self.projection_dependencies)
    }
}

fn is_output_observation_event_for_runtime_execution_domain_owner_v1(event: &Value) -> bool {
    matches!(
        event.get("kind").and_then(Value::as_str),
        Some("stdout" | "stderr")
    )
}

fn is_typed_settlement_event_for_runtime_execution_domain_owner_v1(event: &Value) -> bool {
    matches!(
        event.get("kind").and_then(Value::as_str),
        Some(
            "file_started"
                | "file_finished"
                | "test_started"
                | "test_passed"
                | "test_failed"
                | "test_skipped"
                | "test_todo"
                | "test_expected_failed"
                | "test_unexpected_passed"
                | "test_drift_walled"
                | "diagnostic"
        )
    )
}

impl<'a> SsRuntimeExecutionDomainBodyAuthorityOwner<'a> {
    pub(super) fn new(
        artifact_execution_state: &'a mut super::SsTestArtifactExecutionState,
    ) -> Self {
        Self {
            artifact_execution_state,
        }
    }

    fn materialize_registered_test_body(
        &mut self,
        path: &str,
    ) -> SsResult<TestReadyFileBodyDispatchAuthority> {
        self.artifact_execution_state
            .take_next_registered_body_authority(path)
    }
}

const SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA: &str =
    "swarm.ss.test.runtime_execution_domain.executed_file_payload.v1";
const SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_IPC_FRAME_MAX_BYTES: usize = 16 * 1024 * 1024;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SsRuntimeExecutionDomainExecutedFileWire {
    schema: String,
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    status: String,
    passed: u64,
    failed: u64,
    skipped: u64,
    todo: u64,
    events: Vec<Value>,
    tests: Vec<Value>,
    exact_terminal_seeds:
        Vec<super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeedWire>,
    profile_spans: Vec<Value>,
}

pub(in crate::test_runner) struct SsRuntimeExecutionDomainExecutedFilePayload {
    payload_bytes: Vec<u8>,
}

fn executed_file_wire_status_for_runtime_execution_domain_owner_v1(
    path: &str,
    status: crate::test_runner::SsTestResultStatus,
) -> SsResult<String> {
    use crate::test_runner::SsTestResultStatus;
    let wire_status = match status {
        SsTestResultStatus::Passed => "passed",
        SsTestResultStatus::Failed => "failed",
        SsTestResultStatus::Skipped => "skipped",
        SsTestResultStatus::Todo => "todo",
        other => {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_status_not_settleable",
                    "reason": "executed-file settlement frames may carry only terminal file statuses",
                    "activeFile": path,
                    "status": format!("{other:?}"),
                })
                .to_string(),
            ));
        }
    };
    Ok(wire_status.to_owned())
}

fn admit_executed_file_wire_status_for_runtime_execution_domain_owner_v1(
    path: &str,
    status: &str,
) -> SsResult<crate::test_runner::SsTestResultStatus> {
    use crate::test_runner::SsTestResultStatus;
    match status {
        "passed" => Ok(SsTestResultStatus::Passed),
        "failed" => Ok(SsTestResultStatus::Failed),
        "skipped" => Ok(SsTestResultStatus::Skipped),
        "todo" => Ok(SsTestResultStatus::Todo),
        other => Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                "code": "ss_test_executed_file_payload_status_invalid",
                "reason": "executed-file settlement frame status failed admission",
                "activeFile": path,
                "status": other,
            })
            .to_string(),
        )),
    }
}

fn admit_executed_file_wire_count_for_runtime_execution_domain_owner_v1(
    path: &str,
    field: &'static str,
    count: u64,
) -> SsResult<usize> {
    usize::try_from(count).map_err(|_| {
        SsError::Cli(
            json!({
                "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                "code": "ss_test_executed_file_payload_count_invalid",
                "reason": "executed-file settlement frame count failed admission",
                "activeFile": path,
                "field": field,
                "count": count,
            })
            .to_string(),
        )
    })
}

impl SsRuntimeExecutionDomainExecutedFilePayload {
    pub(super) fn admit_authenticated_pool_worker_payload_for_runtime_execution_domain_owner_v1(
        payload_bytes: Vec<u8>,
    ) -> SsResult<Self> {
        Self::admit_payload_bytes_for_runtime_execution_domain_owner_v1(payload_bytes)
    }

    pub(super) fn consume_into_pool_worker_encoded_settlement_cargo_for_runtime_execution_domain_owner_v1(
        self,
    ) -> super::pool_worker_transport_credential::SsTestPoolWorkerEncodedSettlementCargo {
        super::pool_worker_transport_credential::SsTestPoolWorkerEncodedSettlementCargo::from_executed_file_payload_for_child_final_observation_owner_v1(
            self.payload_bytes,
            Vec::new(),
            false,
            Vec::new(),
            false,
        )
    }

    fn encode_for_runtime_execution_domain_owner_v1(
        executed_file: super::SsExecutedTestFile,
    ) -> SsResult<Self> {
        let super::SsExecutedTestFile {
            _ready_file_case_outcome_admission: _,
            path,
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            status,
            passed,
            failed,
            skipped,
            todo,
            events,
            tests,
            exact_terminal_seeds,
            profile_spans,
            // Parent-side capture observation: the parent already holds these
            // bytes, so the bounded settlement codec never carries them.
            process_captured_stdio: _,
        } = executed_file;
        let status =
            executed_file_wire_status_for_runtime_execution_domain_owner_v1(&path, status)?;
        let profile_spans = profile_spans
            .into_iter()
            .map(|span| {
                serde_json::to_value(&span).map_err(|error| {
                    SsError::Cli(
                        json!({
                            "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                            "code": "ss_test_executed_file_payload_profile_span_encode_failed",
                            "reason": "executed-file settlement frame profile span failed observation encoding",
                            "activeFile": path,
                            "source": error.to_string(),
                        })
                        .to_string(),
                    )
                })
            })
            .collect::<SsResult<Vec<Value>>>()?;
        if exact_terminal_seeds.len() != tests.len()
            || exact_terminal_seeds
                .iter()
                .zip(&tests)
                .any(|(seed, test)| !seed.corresponds_to_projected_case_observation(test))
        {
            return Err(SsError::Fault(json!({
                "schema": "swarm.ss.test.exact_terminal_carriage_fault.v1",
                "code": "ss_test_exact_terminal_seed_preencode_case_correlation_mismatch",
                "reason": "executed-file settlement may encode only after every typed terminal seed exactly correlates with its committed test id, name, status, and ordinal",
                "activeFile": path,
                "seedCount": exact_terminal_seeds.len(),
                "caseCount": tests.len(),
            })));
        }
        let exact_terminal_seeds = exact_terminal_seeds
            .into_iter()
            .map(|seed| seed.into_wire_for_runtime_execution_domain_owner_v1())
            .collect();
        let wire = SsRuntimeExecutionDomainExecutedFileWire {
            schema: SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA.to_owned(),
            path: path.clone(),
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            status,
            passed: passed as u64,
            failed: failed as u64,
            skipped: skipped as u64,
            todo: todo as u64,
            events,
            tests,
            exact_terminal_seeds,
            profile_spans,
        };
        let payload_bytes = rmp_serde::to_vec_named(&wire).map_err(|error| {
            SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_encode_failed",
                    "reason": "executed-file settlement must encode as a bounded binary payload before worker transfer",
                    "activeFile": path,
                    "source": error.to_string(),
                })
                .to_string(),
            )
        })?;
        Self::admit_payload_bytes_for_runtime_execution_domain_owner_v1(payload_bytes)
    }

    fn admit_payload_bytes_for_runtime_execution_domain_owner_v1(
        payload_bytes: Vec<u8>,
    ) -> SsResult<Self> {
        if payload_bytes.is_empty() {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_empty",
                    "reason": "executed-file settlement payload cannot be empty",
                })
                .to_string(),
            ));
        }
        if payload_bytes.len() > SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_IPC_FRAME_MAX_BYTES {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_too_large",
                    "reason": "executed-file settlement payload exceeded the owner bounded IPC frame limit",
                    "payloadBytes": payload_bytes.len(),
                    "maxPayloadBytes": SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_IPC_FRAME_MAX_BYTES,
                })
                .to_string(),
            ));
        }
        Ok(Self { payload_bytes })
    }

    pub(super) fn write_bounded_ipc_frame_for_runtime_execution_domain_owner_v1<
        W: std::io::Write,
    >(
        self,
        mut writer: W,
    ) -> SsResult<()> {
        let payload_len = self.payload_bytes.len();
        writer
            .write_all(&(payload_len as u64).to_be_bytes())
            .and_then(|()| writer.write_all(&self.payload_bytes))
            .and_then(|()| writer.flush())
            .map_err(|error| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                        "code": "ss_test_executed_file_ipc_frame_write_failed",
                        "reason": "executed-file settlement IPC frame write failed",
                        "source": error.to_string(),
                    })
                    .to_string(),
                )
            })
    }

    pub(super) fn read_bounded_ipc_frame_for_runtime_execution_domain_owner_v1<R: std::io::Read>(
        mut reader: R,
    ) -> SsResult<Self> {
        let mut length_bytes = [0_u8; 8];
        reader.read_exact(&mut length_bytes).map_err(|error| {
            SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_ipc_frame_length_read_failed",
                    "reason": "executed-file settlement IPC frame length read failed",
                    "source": error.to_string(),
                })
                .to_string(),
            )
        })?;
        let payload_len = u64::from_be_bytes(length_bytes);
        if payload_len == 0
            || payload_len > SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_IPC_FRAME_MAX_BYTES as u64
        {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_ipc_frame_payload_length_invalid",
                    "reason": "executed-file settlement IPC frame payload length failed bounded admission",
                    "payloadBytes": payload_len,
                    "maxPayloadBytes": SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_IPC_FRAME_MAX_BYTES,
                })
                .to_string(),
            ));
        }
        let mut payload_bytes = vec![0_u8; payload_len as usize];
        reader.read_exact(&mut payload_bytes).map_err(|error| {
            SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_ipc_frame_payload_read_failed",
                    "reason": "executed-file settlement IPC frame payload read failed",
                    "source": error.to_string(),
                })
                .to_string(),
            )
        })?;
        Self::admit_payload_bytes_for_runtime_execution_domain_owner_v1(payload_bytes)
    }

    fn decode_into_executed_file_for_runtime_execution_domain_owner_v1(
        &self,
    ) -> SsResult<super::SsExecutedTestFile> {
        let wire: SsRuntimeExecutionDomainExecutedFileWire =
            rmp_serde::from_slice(&self.payload_bytes).map_err(|error| {
                SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_decode_failed",
                    "reason": "executed-file settlement payload failed binary admission",
                    "source": error.to_string(),
                })
                .to_string(),
            )
            })?;
        if wire.schema != SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_schema_mismatch",
                    "reason": "executed-file settlement payload schema mismatch",
                    "expectedSchema": SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA,
                    "observedSchema": wire.schema,
                })
                .to_string(),
            ));
        }
        if !wire.profile_spans.is_empty() {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                    "code": "ss_test_executed_file_payload_profile_spans_unsupported",
                    "reason": "executed-file settlement frames carry no profile spans at the owner-lane seam; span admission widens with a lawful span owner operation, never serde rehydration",
                    "activeFile": wire.path,
                    "profileSpanCount": wire.profile_spans.len(),
                })
                .to_string(),
            ));
        }
        let status = admit_executed_file_wire_status_for_runtime_execution_domain_owner_v1(
            &wire.path,
            &wire.status,
        )?;
        let passed = admit_executed_file_wire_count_for_runtime_execution_domain_owner_v1(
            &wire.path,
            "passed",
            wire.passed,
        )?;
        let failed = admit_executed_file_wire_count_for_runtime_execution_domain_owner_v1(
            &wire.path,
            "failed",
            wire.failed,
        )?;
        let skipped = admit_executed_file_wire_count_for_runtime_execution_domain_owner_v1(
            &wire.path,
            "skipped",
            wire.skipped,
        )?;
        let todo = admit_executed_file_wire_count_for_runtime_execution_domain_owner_v1(
            &wire.path, "todo", wire.todo,
        )?;
        let exact_terminal_seeds = wire
            .exact_terminal_seeds
            .into_iter()
            .map(|wire| {
                super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed::admit_wire_for_runtime_execution_domain_owner_v1(wire)
            })
            .collect::<SsResult<Vec<_>>>()?;
        if exact_terminal_seeds.len() != wire.tests.len() {
            return Err(SsError::Fault(json!({
                "schema": "swarm.ss.test.exact_terminal_carriage_fault.v1",
                "code": "ss_test_exact_terminal_seed_wire_case_count_mismatch",
                "reason": "pool settlement must carry exactly one typed terminal seed for every projected committed case row",
                "activeFile": wire.path,
                "seedCount": exact_terminal_seeds.len(),
                "caseCount": wire.tests.len(),
            })));
        }
        for (seed, test) in exact_terminal_seeds.iter().zip(&wire.tests) {
            if !seed.corresponds_to_projected_case_observation(test) {
                return Err(SsError::Fault(json!({
                    "schema": "swarm.ss.test.exact_terminal_carriage_fault.v1",
                    "code": "ss_test_exact_terminal_seed_wire_case_correlation_mismatch",
                    "reason": "each pool-settled typed terminal seed must retain the exact test id, name, status, and ordinal of its committed projected case row",
                    "activeFile": wire.path,
                })));
            }
        }
        Ok(super::SsExecutedTestFile {
            _ready_file_case_outcome_admission:
                super::ready_file_case_outcome::SsExecutedTestFileAdmission::admitted_executed_file_settlement_frame(),
            path: wire.path,
            package_root: wire.package_root,
            package_graph_session_fingerprint: wire.package_graph_session_fingerprint,
            package_graph_manifest_fingerprint: wire.package_graph_manifest_fingerprint,
            source_work_set_generation_id: wire.source_work_set_generation_id,
            status,
            passed,
            failed,
            skipped,
            todo,
            events: wire.events,
            tests: wire.tests,
            exact_terminal_seeds,
            profile_spans: Vec::new(),
            process_captured_stdio: Vec::new(),
        })
    }
}

pub(super) fn project_executed_file_settlement_payload_for_pool_harness_observation_v1(
    payload_bytes: Vec<u8>,
) -> SsResult<Value> {
    let executed_file =
        SsRuntimeExecutionDomainExecutedFilePayload::admit_payload_bytes_for_runtime_execution_domain_owner_v1(
            payload_bytes,
        )?
        .decode_into_executed_file_for_runtime_execution_domain_owner_v1()?;
    let status = executed_file_wire_status_for_runtime_execution_domain_owner_v1(
        &executed_file.path,
        executed_file.status,
    )?;
    Ok(json!({
        "schema": "swarm.ss.test.pool_worker.executed_file_observation.v1",
        "path": executed_file.path,
        "status": status,
        "passed": executed_file.passed,
        "failed": executed_file.failed,
        "skipped": executed_file.skipped,
        "todo": executed_file.todo,
        "eventCount": executed_file.events.len(),
        "testCount": executed_file.tests.len(),
    }))
}

pub(super) fn round_trip_executed_file_settlement_frame_for_runtime_execution_domain_owner_v1(
    executed_file: super::SsExecutedTestFile,
    settlement_frame_sink: Option<
        &crate::test_runner::executed_file_frame_sink::SsTestExecutedFileSettlementFrameSink,
    >,
) -> SsResult<super::SsExecutedTestFile> {
    let payload =
        SsRuntimeExecutionDomainExecutedFilePayload::encode_for_runtime_execution_domain_owner_v1(
            executed_file,
        )?;
    if let Some(sink) = settlement_frame_sink {
        sink.push_settlement_payload_bytes_for_pool_worker_child_owner_v1(
            payload.payload_bytes.clone(),
        )?;
    }
    let mut frame = Vec::new();
    payload.write_bounded_ipc_frame_for_runtime_execution_domain_owner_v1(&mut frame)?;
    SsRuntimeExecutionDomainExecutedFilePayload::read_bounded_ipc_frame_for_runtime_execution_domain_owner_v1(
        std::io::Cursor::new(frame),
    )?
    .decode_into_executed_file_for_runtime_execution_domain_owner_v1()
}

/// Pool worker loss: settle the dispatched file's selected cases as typed
/// worker-loss failures through the ordinary owner-lane settlement path (codec
/// round trip included) so the run continues and the file renders failed.
pub(in crate::test_runner) fn settle_pool_worker_loss_for_execution_graph_owner_v1(
    dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    worker_loss_fault: &Value,
    session: &crate::test_runner::SsTestRuntimePlanOwnerSession,
) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
    let super::SsPoolDispatchedSelectedSourceTestFile { selected_source } = dispatched_source;
    let super::SsSelectedSourceTestFile {
        dispatch_custody,
        source_path,
        package_root,
        package_graph_session_fingerprint,
        package_graph_manifest_fingerprint,
        source_work_set_generation_id,
    } = selected_source;
    let diagnostic = json!({
        "schema": "swarm.ss.test.pool_worker_loss_fault.v1",
        "code": "ss_test_selected_source_pool_worker_lost",
        "reason": "the authenticated worker was lost after selected-source dispatch",
        "source": worker_loss_fault,
    });
    let file = super::super::super::preparation_failure::ss_collected_file_from_runtime_failure(
        source_path.display().to_string(),
        package_root,
        package_graph_session_fingerprint,
        package_graph_manifest_fingerprint,
        source_work_set_generation_id,
        crate::test_runner::SsTestFileWorkStage::TestBodyMaterialization,
        diagnostic,
        None,
        session.invocation(),
        Vec::new(),
    )
    .ok_or_else(|| SsError::Fault(json!({
        "schema": "swarm.ss.test.pool_worker_loss_fault.v1",
        "code": "ss_test_selected_source_pool_worker_loss_filtered_without_terminal",
        "reason": "a dispatched selected source cannot be filtered after worker loss without a terminal settlement",
    })))?;
    let settlement =
        ready_file_execution::settle_pool_worker_loss_owner_lane_for_execution_graph_owner_v1(
            file,
            worker_loss_fault,
            SsTestReadyFileResultProjectionDependencies::empty(),
            session,
        )?;
    dispatch_custody.finish_for_execution_graph_owner_v1(
        crate::test_runner::artifact_session::selected_source_dispatch_custody::SsTestParentSelectedSourceDispatchTerminalDisposition::Cancelled,
    )?;
    Ok(SsRuntimeExecutionDomainReadyFileGraphSettlement::owner_settlement(settlement))
}

/// Complete fallible settlement staging while the exact process-registry slot
/// remains `Running`. This is the sole mint: it consumes the retained file and
/// terminal custody, and every refusal returns both unchanged.
pub(in crate::test_runner) fn admit_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
    dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    payload: &SsRuntimeExecutionDomainExecutedFilePayload,
    captured_stdout: &[u8],
    captured_stdout_truncated: bool,
    captured_stderr: &[u8],
    captured_stderr_truncated: bool,
) -> Result<
    SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal,
> {
    match preflight_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
        &dispatched_source,
        payload,
        captured_stdout,
        captured_stdout_truncated,
        captured_stderr,
        captured_stderr_truncated,
    ) {
        Ok(preflighted) => Ok(SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
            dispatched_source,
            preflighted,
        }),
        Err(error) => Err(
            SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal {
                dispatched_source,
                error,
            },
        ),
    }
}

fn preflight_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
    dispatched_source: &super::SsPoolDispatchedSelectedSourceTestFile,
    payload: &SsRuntimeExecutionDomainExecutedFilePayload,
    captured_stdout: &[u8],
    captured_stdout_truncated: bool,
    captured_stderr: &[u8],
    captured_stderr_truncated: bool,
) -> SsResult<SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement> {
    let selected_source = &dispatched_source.selected_source;
    let mut executed_file =
        payload.decode_into_executed_file_for_runtime_execution_domain_owner_v1()?;
    if Path::new(&executed_file.path) != selected_source.source_path {
        return Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.test.runtime_execution_domain_settlement_codec_fault.v1",
                "code": "ss_test_pool_worker_settlement_path_mismatch",
                "reason": "pool worker settlement payload must belong to the dispatched file",
                "expectedPath": selected_source.source_path,
                "observedPath": executed_file.path,
            })
            .to_string(),
        ));
    }
    let expected_path = selected_source.source_path.display().to_string();
    // Test declarations and exact terminal seeds are authored by the
    // authenticated worker-local compile. The parent retains only the exact
    // selected-source metadata needed for final observation attribution.
    executed_file.package_root = selected_source.package_root.clone();
    executed_file.package_graph_session_fingerprint =
        selected_source.package_graph_session_fingerprint.clone();
    executed_file.package_graph_manifest_fingerprint =
        selected_source.package_graph_manifest_fingerprint.clone();
    executed_file.source_work_set_generation_id =
        selected_source.source_work_set_generation_id.clone();
    // Parent-captured child stdio remains attributed to this exact file inside
    // the uncommitted graph settlement. It is neither emitted nor dropped by
    // worker-payload admission.
    let mut captured_events = Vec::new();
    for (kind, bytes, truncated) in [
        ("stdout", captured_stdout, captured_stdout_truncated),
        ("stderr", captured_stderr, captured_stderr_truncated),
    ] {
        if !bytes.is_empty() {
            captured_events.push(json!({
                "kind": kind,
                "path": expected_path,
                "text": String::from_utf8_lossy(bytes),
                "truncated": truncated,
            }));
        }
    }
    executed_file.process_captured_stdio = captured_events;
    let outcome = SsTestReadyFileNodeOutcome::admit_owner_lane_execution(
        executed_file,
        SsTestReadyFileResultProjectionDependencies::empty(),
    );
    let prepared_settlement =
        SsRuntimeExecutionDomainWorkerFileSettlement::prepare_owner_lane_execution(
            &expected_path,
            &outcome,
        )?;
    Ok(SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement {
        outcome,
        prepared_settlement,
    })
}

pub(in crate::test_runner) fn commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1(
    admitted: SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
    let SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
        dispatched_source,
        preflighted,
    } = admitted;
    let SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement {
        outcome,
        prepared_settlement,
    } = preflighted;
    let super::SsPoolDispatchedSelectedSourceTestFile { selected_source } = dispatched_source;
    selected_source.dispatch_custody.finish_for_execution_graph_owner_v1(
        crate::test_runner::artifact_session::selected_source_dispatch_custody::SsTestParentSelectedSourceDispatchTerminalDisposition::Settled,
    )?;
    let settlement =
        SsRuntimeExecutionDomainWorkerFileSettlement::commit_prepared_owner_lane_execution(
            prepared_settlement,
            outcome,
        );
    Ok(SsRuntimeExecutionDomainReadyFileGraphSettlement::owner_settlement(settlement))
}

fn project_ready_file_node_outcomes_from_runtime_execution_domain(
    executor: &mut super::SsTestArtifactPlanExecutor,
    session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
    timeout_observation: Option<String>,
    source_work_set_receipt: SsTestSourceWorkSetReceipt,
    mut events: Vec<Value>,
    file_order: Vec<String>,
    graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
) -> SsResult<Value> {
    let settlement_frame_sink = session.executed_file_settlement_frame_sink().cloned();
    let committed_file_candidates =
        reconcile_committed_file_candidates_with_planned_file_order_for_runtime_execution_domain_owner_v1(
            file_order,
            graph_settlements,
            settlement_frame_sink.as_ref(),
        )?;
    let mut executed_files = Vec::with_capacity(committed_file_candidates.len());
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;
    let mut todo = 0usize;

    for candidate in committed_file_candidates {
        let (executed_file, projection_dependencies) =
            candidate.into_projection_parts_for_runtime_execution_domain_owner_v1();
        session.append_worker_profile_spans(executed_file.profile_spans);
        events.extend(executed_file.events.clone());
        passed += executed_file.passed;
        failed += executed_file.failed;
        skipped += executed_file.skipped;
        todo += executed_file.todo;

        let file_result_projection_dag_node = executor
            .admit_source_work_set_file_result_projection(
                Path::new(&executed_file.path),
                SsTestFileWorkStage::TestBodyMaterialization,
                projection_dependencies.into_dependencies(),
                executed_file.status,
                executed_file.passed,
                executed_file.failed,
                executed_file.skipped,
                executed_file.todo,
                executed_file.tests.len(),
            )?;
        session.record_profile_span(
            SsTestProfilePhase::SourceWorkSetArtifactDagAdmission,
            Duration::ZERO,
            SsTestProfileSpanContext::path_package_root(
                executed_file.path.clone(),
                executed_file.package_root.clone(),
                json!({
                    "authority": "file result projected from runtime-domain admitted ready-file outcome receipts",
                    "sourceWorkSetArtifactDagNodeId": file_result_projection_dag_node,
                }),
            ),
        );
        // Per-fixture phase-checkpoint trace (swarm.ss.test.phase_trace.v1):
        // assembled here, at the single deterministic projection point every
        // planned fixture crosses, from the typed boundary documents its case
        // results already carry. Observation cargo only.
        let phase_trace = super::phase_trace_projection::phase_trace_observation_for_file_result_v1(
            &executed_file.path,
            &executed_file.source_work_set_generation_id,
            &executed_file.tests,
        );
        let mut executed_file_entry = json!({
            "path": executed_file.path,
            "packageRoot": executed_file.package_root,
            "packageGraphSessionFingerprint": executed_file.package_graph_session_fingerprint,
            "packageGraphManifestFingerprint": executed_file.package_graph_manifest_fingerprint,
            "sourceWorkSetGenerationId": executed_file.source_work_set_generation_id,
            "tests": executed_file.tests,
            "phaseTrace": phase_trace,
        });
        if !executed_file.process_captured_stdio.is_empty() {
            executed_file_entry
                .as_object_mut()
                .expect("executed file entry is an object")
                .insert(
                    "capturedProcessStdio".to_owned(),
                    Value::Array(executed_file.process_captured_stdio),
                );
        }
        executed_files.push(executed_file_entry);
    }

    Ok(executor.project_executed_source_work_set_result(
        session,
        timeout_observation,
        source_work_set_receipt,
        events,
        executed_files,
        passed,
        failed,
        skipped,
        todo,
    ))
}

fn reconcile_committed_file_candidates_with_planned_file_order_for_runtime_execution_domain_owner_v1(
    file_order: Vec<String>,
    graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    settlement_frame_sink: Option<
        &crate::test_runner::executed_file_frame_sink::SsTestExecutedFileSettlementFrameSink,
    >,
) -> SsResult<Vec<SsRuntimeExecutionDomainCommittedFileCandidate>> {
    let mut committed_file_candidates = BTreeMap::new();
    for settlement in graph_settlements {
        let candidate =
            settlement.into_committed_file_candidate_after_graph_commit(settlement_frame_sink)?;
        let path = candidate.source_path().to_owned();
        if committed_file_candidates
            .insert(path.clone(), candidate)
            .is_some()
        {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.work_set_runtime_plan_fault.v1",
                    "code": "ss_test_work_set_runtime_plan_committed_file_candidate_duplicate",
                    "reason": "runtime-plan ready-file graph settlements must normalize into exactly one committed-file candidate per source path before deterministic projection",
                    "sourcePath": path,
                })
                .to_string(),
            ));
        }
    }
    let mut ordered_candidates = Vec::with_capacity(file_order.len());

    for path in file_order {
        let candidate = committed_file_candidates.remove(&path).ok_or_else(|| {
            SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_fault.v1",
                        "code": "ss_test_ready_file_execution_output_missing",
                        "reason": "runtime execution-domain deterministic result projection requires one normalized committed-file candidate for every planned file",
                        "filePath": path,
                    })
                    .to_string(),
                )
        })?;
        ordered_candidates.push(candidate);
    }

    if !committed_file_candidates.is_empty() {
        let extra_source_paths = committed_file_candidates
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        return Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.test.work_set_runtime_plan_fault.v1",
                "code": "ss_test_work_set_runtime_plan_committed_file_candidate_extra",
                "reason": "runtime execution-domain deterministic result projection requires every normalized committed-file candidate to consume one exact planned-file entry; remaining candidates are unplanned extra settlements",
                "extraCommittedFileCandidateCount": extra_source_paths.len(),
                "extraSourcePaths": extra_source_paths,
            })
            .to_string(),
        ));
    }

    Ok(ordered_candidates)
}

#[cfg(test)]
mod committed_file_candidate_reconciliation_tests {
    use super::{
        SsRuntimeExecutionDomainReadyFileGraphSettlement,
        reconcile_committed_file_candidates_with_planned_file_order_for_runtime_execution_domain_owner_v1,
    };

    fn fixture_settlement(path: &str) -> SsRuntimeExecutionDomainReadyFileGraphSettlement {
        SsRuntimeExecutionDomainReadyFileGraphSettlement::inert_fixture_for_execution_graph_owner_v1(
            path,
        )
        .expect("inert committed-file settlement fixture should admit")
    }

    fn reconcile(
        planned_paths: &[&str],
        settled_paths: &[&str],
    ) -> crate::SsResult<Vec<super::SsRuntimeExecutionDomainCommittedFileCandidate>> {
        reconcile_committed_file_candidates_with_planned_file_order_for_runtime_execution_domain_owner_v1(
            planned_paths.iter().map(|path| (*path).to_owned()).collect(),
            settled_paths
                .iter()
                .map(|path| fixture_settlement(path))
                .collect(),
            None,
        )
    }

    fn reconciliation_fault(planned_paths: &[&str], settled_paths: &[&str]) -> String {
        match reconcile(planned_paths, settled_paths) {
            Ok(_) => panic!("committed-file candidate set unexpectedly reconciled"),
            Err(error) => error.to_string(),
        }
    }

    #[test]
    fn exact_committed_file_candidate_set_preserves_planned_order() {
        let candidates = reconcile(
            &["/tmp/planned-b.test.ss", "/tmp/planned-a.test.ss"],
            &["/tmp/planned-a.test.ss", "/tmp/planned-b.test.ss"],
        )
        .expect("the exact committed candidate set should reconcile");
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.source_path())
                .collect::<Vec<_>>(),
            vec!["/tmp/planned-b.test.ss", "/tmp/planned-a.test.ss"]
        );
    }

    #[test]
    fn duplicate_committed_file_candidate_is_a_typed_fault() {
        let rendered = reconciliation_fault(
            &["/tmp/duplicate.test.ss"],
            &["/tmp/duplicate.test.ss", "/tmp/duplicate.test.ss"],
        );
        assert!(
            rendered.contains("ss_test_work_set_runtime_plan_committed_file_candidate_duplicate"),
            "duplicate-settlement fault must retain its typed code: {rendered}"
        );
        assert!(
            rendered.contains("/tmp/duplicate.test.ss"),
            "duplicate-settlement fault must name the duplicated path: {rendered}"
        );
    }

    #[test]
    fn missing_committed_file_candidate_is_a_typed_fault() {
        let rendered = reconciliation_fault(&["/tmp/missing.test.ss"], &[]);
        assert!(
            rendered.contains("ss_test_ready_file_execution_output_missing"),
            "missing-settlement fault must retain its typed code: {rendered}"
        );
        assert!(
            rendered.contains("/tmp/missing.test.ss"),
            "missing-settlement fault must name the missing path: {rendered}"
        );
    }

    #[test]
    fn extra_committed_file_candidate_is_a_typed_fault() {
        let rendered = reconciliation_fault(
            &["/tmp/planned.test.ss"],
            &["/tmp/planned.test.ss", "/tmp/unplanned-extra.test.ss"],
        );
        assert!(
            rendered.contains("ss_test_work_set_runtime_plan_committed_file_candidate_extra"),
            "extra-settlement fault must carry its typed code: {rendered}"
        );
        assert!(
            rendered.contains("/tmp/unplanned-extra.test.ss"),
            "extra-settlement fault must name the unplanned path: {rendered}"
        );
        assert!(
            rendered.contains("\"extraCommittedFileCandidateCount\":1"),
            "extra-settlement fault must report exact surplus cardinality: {rendered}"
        );
    }
}

impl SsRuntimeExecutionDomainReadyFileAdmission {
    fn owner_settlement(settlement: SsRuntimeExecutionDomainWorkerFileSettlement) -> Self {
        Self {
            kind: SsRuntimeExecutionDomainReadyFileAdmissionKind::OwnerOutcome { settlement },
        }
    }

    fn deferred_owner_lane(work_item: SsTestOwnerLaneReadyFileWorkItem) -> Self {
        Self {
            kind: SsRuntimeExecutionDomainReadyFileAdmissionKind::DeferredOwnerLane(work_item),
        }
    }
}

impl SsRuntimeExecutionDomainOwnerLaneQueue {
    pub(super) fn new() -> Self {
        Self {
            deferred: VecDeque::new(),
        }
    }

    pub(super) fn defer(&mut self, work_item: SsTestOwnerLaneReadyFileWorkItem) {
        self.deferred.push_back(work_item);
    }

    pub(super) fn drain(
        &mut self,
        artifact_execution_state: &mut super::SsTestArtifactExecutionState,
        runtime_execution_domain_owner: &mut SsRuntimeExecutionDomainOwner,
        session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> crate::SsResult<Vec<SsRuntimeExecutionDomainWorkerFileSettlement>> {
        let mut settlements = Vec::new();
        while let Some(work_item) = self.deferred.pop_front() {
            let settlement = runtime_execution_domain_owner.execute_owner_lane_ready_file(
                work_item,
                artifact_execution_state,
                session,
                spawned_worker_child_liveness_deadline,
            )?;
            settlements.push(settlement);
        }
        Ok(settlements)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.deferred.is_empty()
    }

    pub(super) fn len(&self) -> usize {
        self.deferred.len()
    }
}

fn admit_ready_file_execution_domain(
    node: AdmittedSsTestWorkItem,
    artifact_execution_state: &mut super::SsTestArtifactExecutionState,
    runtime_execution_domain_owner: Option<&mut SsRuntimeExecutionDomainOwner>,
    session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
    spawned_worker_child_liveness_deadline: Duration,
) -> crate::SsResult<SsRuntimeExecutionDomainReadyFileAdmission> {
    let mut body_authority_owner =
        SsRuntimeExecutionDomainBodyAuthorityOwner::new(artifact_execution_state);
    let work_item = SsTestOwnerLaneReadyFileWorkItem::admit(node)?;
    match runtime_execution_domain_owner {
        Some(runtime_execution_domain_owner) => {
            let settlement = runtime_execution_domain_owner
                .execute_owner_lane_ready_file_with_body_authority(
                    work_item,
                    &mut body_authority_owner,
                    session,
                    spawned_worker_child_liveness_deadline,
                )?;
            Ok(SsRuntimeExecutionDomainReadyFileAdmission::owner_settlement(settlement))
        }
        None => Ok(SsRuntimeExecutionDomainReadyFileAdmission::deferred_owner_lane(work_item)),
    }
}

#[cfg(test)]
pub(super) fn ready_file_execution_file_for_test(
    path: String,
    package_root: String,
    package_graph_session_fingerprint: String,
    package_graph_manifest_fingerprint: String,
    source_work_set_generation_id: String,
    tests: Vec<super::SsCollectedTestCase>,
) -> super::SsReadyFileExecutionFile {
    super::SsReadyFileExecutionFile {
        _runtime_execution_domain_admission: SsReadyFileExecutionFileAdmission::runtime_domain(),
        path,
        package_root,
        package_graph_session_fingerprint,
        package_graph_manifest_fingerprint,
        source_work_set_generation_id,
        tests,
        preparation_terminal: None,
    }
}

#[cfg(test)]
mod executed_file_wire_tests {
    use super::super::ready_file_case_outcome::SsReadyFileExecutionSettlementOwner;
    use super::ready_file_node_outcome::SsTestReadyFileResultProjectionDependencies;
    use super::{
        SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA,
        SsRuntimeExecutionDomainCommittedFileCandidate,
        SsRuntimeExecutionDomainExecutedFilePayload, SsRuntimeExecutionDomainExecutedFileWire,
        admit_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1,
        commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1,
        round_trip_executed_file_settlement_frame_for_runtime_execution_domain_owner_v1,
    };
    use crate::test_runner::SsTestResultStatus;
    use serde_json::{Value, json};
    use ss_runtime_source_compiler_owner::test_declaration::{
        SsCollectedTestCase, SsCollectedTestCaseAdmission, SsTestCaseDisposition,
        admit_child_command_collected_test_case_for_ss_test_execution_owner_v1,
    };
    use std::path::PathBuf;
    use std::time::Duration;

    fn executed_fixture_file() -> super::super::SsExecutedTestFile {
        let file = super::ready_file_execution_file_for_test(
            "/tmp/executed-file-wire.test.ss".to_owned(),
            "/tmp".to_owned(),
            "package-session:test".to_owned(),
            "manifest:test".to_owned(),
            "source-work-set:test".to_owned(),
            vec![collected_test("case.wire", "wire case")],
        );
        let super::super::SsReadyFileExecutionFile {
            path,
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            tests,
            ..
        } = file;
        let mut owner =
            SsReadyFileExecutionSettlementOwner::admit_ready_file_execution(&path, tests);
        let case_execution = owner
            .admit_next_case_start(&path)
            .expect("planned wire case start should admit")
            .expect("planned wire case should exist");
        let result = json!({
            "schema": "swarm.ss.test.case_result.v1",
            "testId": case_execution.test().test_id(),
            "name": case_execution.test().name(),
            "status": "passed",
            "applicationIoOutputRecords": [
                { "stream": "stdout", "text": "wire hello\n" }
            ],
        });
        owner
            .admit_executed_case_result(
                &path,
                case_execution,
                result,
                super::super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed::case_non_failure(
                    "case.wire",
                    "wire case",
                    crate::test_runner::SsTestResultStatus::Passed,
                )
                .expect("passed terminal seed"),
                Duration::from_nanos(23),
            )
            .expect("executed case should settle");
        owner
            .into_executed_file(
                path,
                package_root,
                package_graph_session_fingerprint,
                package_graph_manifest_fingerprint,
                source_work_set_generation_id,
                Vec::new(),
            )
            .expect("exact wire case set should finalize")
    }

    fn dispatched_selected_source_fixture()
    -> crate::test_runner::artifact_session::SsPoolDispatchedSelectedSourceTestFile {
        crate::test_runner::artifact_session::SsPoolDispatchedSelectedSourceTestFile {
            selected_source: crate::test_runner::artifact_session::SsSelectedSourceTestFile {
                dispatch_custody: crate::test_runner::artifact_session::selected_source_dispatch_custody::SsTestParentSelectedSourceDispatchCustody::for_test_fixture_owner_v1(),
                source_path: PathBuf::from("/tmp/executed-file-wire.test.ss"),
                package_root: "/tmp".to_owned(),
                package_graph_session_fingerprint: "package-session:test".to_owned(),
                package_graph_manifest_fingerprint: "manifest:test".to_owned(),
                source_work_set_generation_id: "source-work-set:test".to_owned(),
            },
        }
    }

    fn executed_exact_failure_fixture_file() -> super::super::SsExecutedTestFile {
        let file = super::ready_file_execution_file_for_test(
            "/tmp/executed-file-wire.test.ss".to_owned(),
            "/tmp".to_owned(),
            "package-session:test".to_owned(),
            "manifest:test".to_owned(),
            "source-work-set:test".to_owned(),
            vec![collected_test("case.wire", "wire case")],
        );
        let super::super::SsReadyFileExecutionFile {
            path,
            package_root,
            package_graph_session_fingerprint,
            package_graph_manifest_fingerprint,
            source_work_set_generation_id,
            tests,
            ..
        } = file;
        let mut owner =
            SsReadyFileExecutionSettlementOwner::admit_ready_file_execution(&path, tests);
        let case_execution = owner
            .admit_next_case_start(&path)
            .expect("planned exact wire case start should admit")
            .expect("planned exact wire case should exist");
        owner
            .admit_executed_case_result(
                &path,
                case_execution,
                json!({
                    "schema": "swarm.ss.test.case_result.v1",
                    "status": "failed",
                }),
                super::super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed::case_failed_exact(
                    "case.wire",
                    "wire case",
                    ss_runtime_test_model::SsTestPhaseTracePhase::Effect,
                    "provider_refused",
                    "provider_call",
                )
                .expect("exact failure terminal seed should admit"),
                Duration::from_nanos(29),
            )
            .expect("exact failed case should settle");
        owner
            .into_executed_file(
                path,
                package_root,
                package_graph_session_fingerprint,
                package_graph_manifest_fingerprint,
                source_work_set_generation_id,
                Vec::new(),
            )
            .expect("exact failed wire case set should finalize")
    }

    fn collected_test(test_id: &str, name: &str) -> SsCollectedTestCase {
        admit_child_command_collected_test_case_for_ss_test_execution_owner_v1(
            SsCollectedTestCaseAdmission {
                test_id: test_id.to_owned(),
                name: name.to_owned(),
                disposition: SsTestCaseDisposition::Registered,
                file_path: "/tmp/executed-file-wire.test.ss".to_owned(),
                package_root: "/tmp".to_owned(),
                source_work_set_generation_id: "source-work-set:test".to_owned(),
                provider_id: "@swarm/test:test".to_owned(),
                contract: json!({ "schema": "swarm.test.contract.v1" }),
                admission_diagnostic: None,
                options: Value::Null,
            },
        )
        .expect("fixture collected test should admit")
    }

    fn passed_seed_wire(
        test_id: &str,
        name: &str,
    ) -> super::super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeedWire {
        super::super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeed::case_non_failure(
            test_id,
            name,
            SsTestResultStatus::Passed,
        )
        .expect("passed seed should admit")
        .into_wire_for_runtime_execution_domain_owner_v1()
    }

    fn passed_wire(
        tests: Vec<Value>,
        exact_terminal_seeds: Vec<
            super::super::exact_terminal_observation_carriage::SsTestCaseExactTerminalSeedWire,
        >,
    ) -> SsRuntimeExecutionDomainExecutedFileWire {
        let passed = tests.len() as u64;
        SsRuntimeExecutionDomainExecutedFileWire {
            schema: SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA.to_owned(),
            path: "/tmp/executed-file-wire.test.ss".to_owned(),
            package_root: "/tmp".to_owned(),
            package_graph_session_fingerprint: "package-session:test".to_owned(),
            package_graph_manifest_fingerprint: "manifest:test".to_owned(),
            source_work_set_generation_id: "source-work-set:test".to_owned(),
            status: "passed".to_owned(),
            passed,
            failed: 0,
            skipped: 0,
            todo: 0,
            events: Vec::new(),
            tests,
            exact_terminal_seeds,
            profile_spans: Vec::new(),
        }
    }

    fn decode_wire(
        wire: SsRuntimeExecutionDomainExecutedFileWire,
    ) -> crate::SsResult<super::super::SsExecutedTestFile> {
        let payload_bytes = rmp_serde::to_vec_named(&wire).expect("wire should encode");
        SsRuntimeExecutionDomainExecutedFilePayload::admit_payload_bytes_for_runtime_execution_domain_owner_v1(
            payload_bytes,
        )?
        .decode_into_executed_file_for_runtime_execution_domain_owner_v1()
    }

    fn decode_wire_error(wire: SsRuntimeExecutionDomainExecutedFileWire) -> crate::SsError {
        match decode_wire(wire) {
            Ok(_) => panic!("malformed executed-file wire must fail closed"),
            Err(error) => error,
        }
    }

    #[test]
    fn executed_file_round_trips_through_bounded_settlement_frame() {
        let executed = executed_fixture_file();
        let expected_events = executed.events.clone();
        let expected_tests = executed.tests.clone();
        let round_tripped =
            round_trip_executed_file_settlement_frame_for_runtime_execution_domain_owner_v1(
                executed, None,
            )
            .expect("full executed file must round-trip the bounded settlement frame");
        assert_eq!(round_tripped.path, "/tmp/executed-file-wire.test.ss");
        assert_eq!(round_tripped.package_root, "/tmp");
        assert_eq!(
            round_tripped.package_graph_session_fingerprint,
            "package-session:test"
        );
        assert_eq!(
            round_tripped.package_graph_manifest_fingerprint,
            "manifest:test"
        );
        assert_eq!(
            round_tripped.source_work_set_generation_id,
            "source-work-set:test"
        );
        assert_eq!(round_tripped.status, SsTestResultStatus::Passed);
        assert_eq!(round_tripped.passed, 1);
        assert_eq!(round_tripped.failed, 0);
        assert_eq!(round_tripped.skipped, 0);
        assert_eq!(round_tripped.todo, 0);
        assert_eq!(round_tripped.events, expected_events);
        assert_eq!(round_tripped.tests, expected_tests);
        assert!(round_tripped.profile_spans.is_empty());
        assert!(round_tripped.events.iter().any(|event| {
            event.get("kind").and_then(Value::as_str) == Some("stdout")
                && event.get("text").and_then(Value::as_str) == Some("wire hello\n")
        }));
    }

    #[test]
    fn executed_file_wire_rejects_missing_exact_terminal_seed() {
        let wire = passed_wire(
            vec![json!({
                "testId": "case.wire",
                "name": "wire case",
                "status": "passed",
            })],
            Vec::new(),
        );
        let error = decode_wire_error(wire);
        assert!(
            error
                .to_string()
                .contains("ss_test_exact_terminal_seed_wire_case_count_mismatch")
        );
    }

    #[test]
    fn executed_file_wire_rejects_extra_exact_terminal_seed() {
        let wire = passed_wire(
            vec![json!({
                "testId": "case.wire",
                "name": "wire case",
                "status": "passed",
            })],
            vec![
                passed_seed_wire("case.wire", "wire case"),
                passed_seed_wire("case.extra", "extra case"),
            ],
        );
        let error = decode_wire_error(wire);
        assert!(
            error
                .to_string()
                .contains("ss_test_exact_terminal_seed_wire_case_count_mismatch")
        );
    }

    #[test]
    fn executed_file_wire_rejects_reordered_exact_terminal_seeds() {
        let wire = passed_wire(
            vec![
                json!({
                    "testId": "case.first",
                    "name": "first case",
                    "status": "passed",
                }),
                json!({
                    "testId": "case.second",
                    "name": "second case",
                    "status": "passed",
                }),
            ],
            vec![
                passed_seed_wire("case.second", "second case"),
                passed_seed_wire("case.first", "first case"),
            ],
        );
        let error = decode_wire_error(wire);
        assert!(
            error
                .to_string()
                .contains("ss_test_exact_terminal_seed_wire_case_correlation_mismatch")
        );
    }

    #[test]
    fn executed_file_frame_rejects_schema_mismatch() {
        let wire = SsRuntimeExecutionDomainExecutedFileWire {
            schema: "swarm.ss.test.wrong_schema.v1".to_owned(),
            path: "/tmp/executed-file-wire.test.ss".to_owned(),
            package_root: "/tmp".to_owned(),
            package_graph_session_fingerprint: "package-session:test".to_owned(),
            package_graph_manifest_fingerprint: "manifest:test".to_owned(),
            source_work_set_generation_id: "source-work-set:test".to_owned(),
            status: "passed".to_owned(),
            passed: 1,
            failed: 0,
            skipped: 0,
            todo: 0,
            events: Vec::new(),
            tests: Vec::new(),
            exact_terminal_seeds: Vec::new(),
            profile_spans: Vec::new(),
        };
        let error = decode_wire_error(wire);
        assert!(
            error
                .to_string()
                .contains("ss_test_executed_file_payload_schema_mismatch")
        );
    }

    #[test]
    fn executed_file_frame_rejects_unsettleable_status() {
        let wire = SsRuntimeExecutionDomainExecutedFileWire {
            schema: SS_RUNTIME_EXECUTION_DOMAIN_EXECUTED_FILE_PAYLOAD_SCHEMA.to_owned(),
            path: "/tmp/executed-file-wire.test.ss".to_owned(),
            package_root: "/tmp".to_owned(),
            package_graph_session_fingerprint: "package-session:test".to_owned(),
            package_graph_manifest_fingerprint: "manifest:test".to_owned(),
            source_work_set_generation_id: "source-work-set:test".to_owned(),
            status: "running".to_owned(),
            passed: 0,
            failed: 0,
            skipped: 0,
            todo: 0,
            events: Vec::new(),
            tests: Vec::new(),
            exact_terminal_seeds: Vec::new(),
            profile_spans: Vec::new(),
        };
        let error = decode_wire_error(wire);
        assert!(
            error
                .to_string()
                .contains("ss_test_executed_file_payload_status_invalid")
        );
    }

    #[test]
    fn pool_payload_decode_retains_attributed_stdio_without_session_authority() {
        let payload =
            SsRuntimeExecutionDomainExecutedFilePayload::encode_for_runtime_execution_domain_owner_v1(
                executed_fixture_file(),
            )
            .expect("fixture executed file should encode");
        let dispatched_source = dispatched_selected_source_fixture();
        let admitted =
            admit_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
                dispatched_source,
                &payload,
                b"pool stdout\n",
                false,
                b"pool stderr\n",
                true,
            )
            .expect("pool payload should admit without reporter/session authority");
        let settlement =
            commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1(admitted)
                .expect("parent dispatch custody should settle before committed projection");
        let candidate = settlement
            .into_committed_file_candidate_after_graph_commit(None)
            .expect("committed pool settlement should normalize");
        let (output_events, typed_events) = candidate.observation_events_for_committed_file_v1();
        assert_eq!(
            output_events[0].get("text").and_then(Value::as_str),
            Some("pool stdout\n")
        );
        assert_eq!(
            output_events[1].get("text").and_then(Value::as_str),
            Some("pool stderr\n")
        );
        assert_eq!(
            output_events[1].get("truncated").and_then(Value::as_bool),
            Some(true)
        );
        assert!(output_events.iter().all(|event| {
            event.get("path").and_then(Value::as_str) == Some("/tmp/executed-file-wire.test.ss")
        }));
        assert!(
            output_events
                .iter()
                .any(|event| { event.get("text").and_then(Value::as_str) == Some("wire hello\n") })
        );
        assert!(!typed_events.is_empty());
        let (projected, _) =
            candidate.into_projection_parts_for_runtime_execution_domain_owner_v1();
        assert_eq!(projected.process_captured_stdio.len(), 2);
    }

    #[test]
    fn malformed_pool_success_preflight_retains_parent_file_for_valid_retry() {
        let malformed_payload = SsRuntimeExecutionDomainExecutedFilePayload::admit_authenticated_pool_worker_payload_for_runtime_execution_domain_owner_v1(
            vec![0xff, 0x00, 0x01],
        )
        .expect("non-empty malformed fixture remains authenticated encoded cargo");
        let malformed =
            admit_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
                dispatched_selected_source_fixture(),
                &malformed_payload,
                &[],
                false,
                &[],
                false,
            )
            .err()
            .expect("malformed pool success payload must return retaining refusal");
        assert!(
            malformed
                .error
                .to_string()
                .contains("ss_test_executed_file_payload_decode_failed")
        );
        let dispatched_source = malformed.dispatched_source;

        let payload = SsRuntimeExecutionDomainExecutedFilePayload::encode_for_runtime_execution_domain_owner_v1(
            executed_fixture_file(),
        )
        .expect("valid retry payload should encode");
        let admitted =
            admit_pool_worker_admitted_executed_file_payload_for_execution_graph_owner_v1(
                dispatched_source,
                &payload,
                &[],
                false,
                &[],
                false,
            )
            .expect("the refusal-retained file must admit a later valid payload");
        let settlement =
            commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1(admitted)
                .expect("refusal-retained parent dispatch custody should settle on valid retry");
        settlement
            .into_committed_file_candidate_after_graph_commit(None)
            .expect("valid retry must commit through the ordinary graph settlement");
    }
}
