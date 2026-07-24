use super::{SsTestRunnerSession, TestRunFinished, TestRunState};
use crate::test_runner::artifact_session::{
    SourceWorkSetRuntimePlanWorkerExecutionAuthorities, SsCollectedTestFile,
    SsPoolDispatchedSelectedSourceTestFile, SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    SsRuntimeExecutionDomainReadyFileGraphSettlement, SsSelectedSourceTestFile,
    SsTestCompilerWorkerPhaseObservation, SsTestParentSelectedSourceDispatchAdmission,
    SsTestPoolWorkerParentObservedFrame, SsTestPoolWorkerParentPool,
    SsTestPoolWorkerParentPreparedSettlementCargo, SsTestPoolWorkerRuntimeRefusalKind,
    SsTestSourceWorkSetRuntimePlanAdmissionFeed,
    SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner,
    read_child_frame_for_pool_worker_parent_v1,
};
use crate::test_runner::{
    SsCliTestInvocation, SsTestFileWorkStage, SsTestProfilePhase, SsTestProfileSpanContext,
    SsTestSchedulerWidthRequest, SsTestTimeoutRequest, ss_test_plural,
    ss_test_progress_duration_suffix,
};
use crate::{
    SsError, SsResult,
    source_work_set::{
        SsSourceWorkSetAdmittedDependencyGraph,
        SsSourceWorkSetAdmittedSourceInventoryForSsTestExecutionOwnerV1,
        SsSourceWorkSetCheckerError,
        SsSourceWorkSetClosureReadySelectedTestSourceForSsTestExecutionOwnerV1,
        SsSourceWorkSetExecutableFrontPassAdmissionForSsTestExecutionOwnerV1,
        SsSourceWorkSetLeasedSourceFactStepForSsTestExecutionOwnerV1,
        SsSourceWorkSetNonTerminalCheckerError,
        SsSourceWorkSetRefusedSelectedTestSourceForSsTestExecutionOwnerV1,
        SsSourceWorkSetSelectedTestSourceFeedAndWorkAdmissionForSsTestExecutionOwnerV1,
        SsSourceWorkSetSelectedTestSourceRuntimePlanCoverageForSsTestExecutionOwnerV1,
        SsSourceWorkSetSourceFactsBundle,
        SsSourceWorkSetStreamingFactAdmissionForSsTestExecutionOwnerV1,
        SsSourceWorkSetStreamingFactApplicationForSsTestExecutionOwnerV1,
        SsSourceWorkSetUnadmittedTestSourceForSsTestExecutionOwnerV1,
    },
};
use libswarm_package_graph_source_model::PackageGraphPackageUniverseAdmission;
use serde_json::{Value, json};
use ss_runtime_source_compiler_owner::{
    SsTestSourceWorkSetGeneration, SsTestSourceWorkSetReceiptFileCount,
};
use ss_runtime_test_plan_owner::{
    SsTestPlanError, SsTestRunPlanDiscoveredTestFileForSsTestExecutionOwnerV1,
    SsTestRunPlanDiscoveredTestFileSinkForSsTestExecutionOwnerV1,
    SsTestRunPlanDiscoveryWalkForSsTestExecutionOwnerV1,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc,
};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

mod execution_graph_coordination;
mod execution_graph_session;
mod runtime_file_commit_publication;
mod source_work_set_admission_coordinator;
mod source_work_set_runtime_dispatch;
use runtime_file_commit_publication::SsTestExecutionGraphRuntimeFileExecutionLease;
pub(in crate::test_runner) use runtime_file_commit_publication::{
    SsTestExecutionGraphRuntimeFileCacheHitSettlement,
    SsTestExecutionGraphRuntimeFileExecutionSession,
    SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement,
};
#[cfg(test)]
pub(in crate::test_runner) use runtime_file_commit_publication::{
    publish_cache_hit_runtime_file_fixture_through_committed_carrier_for_streaming_acceptance_v1,
    publish_captured_runtime_file_fixture_through_committed_carrier_for_streaming_acceptance_v1,
};

const SS_TEST_EXECUTION_GRAPH_RUNTIME_PLAN_BACKGROUND_LIVENESS_DEADLINE: Duration =
    Duration::from_secs(30);
const SS_TEST_EXECUTION_GRAPH_SPAWNED_WORKER_CHILD_LIVENESS_DEADLINE: Duration =
    Duration::from_secs(30);
const SS_TEST_EXECUTION_GRAPH_RUNTIME_FILE_POOL_CONFORMANCE_TIMEOUT_DEADLINE: Duration =
    Duration::from_secs(30);
const SS_TEST_EXECUTION_GRAPH_RUNTIME_FILE_POOL_CONFORMANCE_TIMEOUT_OBSERVATION: &str = "30s";
const SS_TEST_EXECUTION_GRAPH_RUNTIME_FILE_POOL_DEFAULT_TIMEOUT_DEADLINE: Duration =
    Duration::from_secs(60);
const SS_TEST_EXECUTION_GRAPH_RUNTIME_FILE_POOL_DEFAULT_TIMEOUT_OBSERVATION: &str = "60s";
const SS_TEST_EXECUTION_GRAPH_QUIET_PROGRESS_INTERVAL: Duration = Duration::from_secs(10);
const SS_TEST_COMPILER_WORKER_RECENT_PHASE_LIMIT: usize = 7;
static SS_TEST_EXECUTION_GRAPH_NEXT_WORKER_LEASE_CELL_ID: AtomicU64 = AtomicU64::new(1);

struct SsTestSourceWorkSetLiveAdmissionInventory {
    source_work_set_inventory:
        Option<SsSourceWorkSetAdmittedSourceInventoryForSsTestExecutionOwnerV1>,
    selected_source_count: usize,
}

#[derive(Clone, Debug)]
pub(in crate::test_runner) struct SsTestSourceWorkSetAdmissionProfile {
    stage_records: Vec<Value>,
}

pub(in crate::test_runner) fn source_work_set_non_terminal_checker_error_to_ss_error(
    error: SsSourceWorkSetNonTerminalCheckerError,
) -> SsError {
    let source = error.into_fault_observation_for_ss_test_execution_owner_v1();
    SsError::Fault(json!({
        "schema": "swarm.ss.test.source_work_set_checker_error.v1",
        "code": "ss_test_source_work_set_checker_error",
        "reason": "source-work-set checker faults must be admitted at the live test execution owner; the source-test bridge may not depend on the live checker just to provide a broad error conversion",
        "source": source,
    }))
}

/// Stream-reader helpers do not own selected graph work. Selected discovery,
/// and source-fact workers are retained by the owned worker set below; only
/// pool stdout/stderr drainers use this helper.
fn spawn_detached_execution_graph_worker(
    name: String,
    worker: impl FnOnce() + Send + 'static,
) -> SsResult<()> {
    std::thread::Builder::new()
        .name(name.clone())
        .spawn(worker)
        .map(drop)
        .map_err(|error| {
            SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_worker_spawn_failed",
                    "reason": "graph coordinator could not spawn an owned worker thread",
                    "workerName": name,
                    "source": error.to_string(),
                })
                .to_string(),
            )
        })
}

struct SsTestExecutionGraphWorkerGeneration {
    serial: u64,
    active: AtomicBool,
}

impl SsTestExecutionGraphWorkerGeneration {
    fn open(serial: u64) -> Self {
        Self {
            serial,
            active: AtomicBool::new(true),
        }
    }

    fn revoke(&self) {
        self.active.store(false, Ordering::Release);
    }

    fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }
}

enum SsTestExecutionGraphWorkerLeaseCellState<O> {
    Admitted,
    Running,
    Settled(O),
    Cancelled,
    ExecutionUncertain,
}

/// Sole owner of one exact selected input across admission, worker execution,
/// settlement, cancellation, and quarantine. The worker sees only the grant
/// below; neither type exposes extraction, deref, cloning, serde, or selectors.
struct SsTestExecutionGraphWorkerLeaseCell<T, O> {
    identity: u64,
    generation: Arc<SsTestExecutionGraphWorkerGeneration>,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    input: T,
    state: Mutex<SsTestExecutionGraphWorkerLeaseCellState<O>>,
}

enum SsTestExecutionGraphWorkerTerminal<O> {
    Completed(O),
    Panicked,
    GenerationRevoked,
    SpawnRefused,
}

struct SsTestExecutionGraphWorkerExecutionGrant<T, O> {
    cell: Arc<SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>>,
    cell_identity: u64,
    generation_serial: u64,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
}

struct SsTestExecutionGraphWorkerEffectPermit {
    generation: Arc<SsTestExecutionGraphWorkerGeneration>,
    generation_serial: u64,
}

impl SsTestExecutionGraphWorkerEffectPermit {
    fn is_active(&self) -> bool {
        self.generation.serial == self.generation_serial && self.generation.is_active()
    }
}

struct SsTestExecutionGraphWorkerQuarantineAcceptanceReceipt {
    cell_identity: u64,
}

struct SsTestExecutionGraphWorkerExecutionUncertain<T> {
    input: T,
    acceptance: SsTestExecutionGraphWorkerQuarantineAcceptanceReceipt,
    late_worker_panicked: bool,
}

/// Named owner for an unfinished worker and its exact lease cell. Acceptance
/// revokes the generation before moving cancellation into execution-uncertain;
/// the handle and cell cannot separate while late execution is possible.
struct SsTestExecutionGraphWorkerQuarantineOwner<T, O> {
    handle: JoinHandle<()>,
    lease_cell: Arc<SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>>,
    acceptance: SsTestExecutionGraphWorkerQuarantineAcceptanceReceipt,
}

enum SsTestExecutionGraphWorkerBoundedJoin<T, O> {
    Joined {
        lease_cell:
            Arc<SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>>,
        observation: std::thread::Result<()>,
    },
    Quarantined(SsTestExecutionGraphWorkerQuarantineOwner<T, O>),
}

impl<T, O> SsTestExecutionGraphWorkerQuarantineOwner<T, O>
where
    T: Send + 'static,
    O: Send + 'static,
{
    fn accept(
        handle: JoinHandle<()>,
        lease_cell: Arc<
            SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>,
        >,
    ) -> Result<
        Self,
        (
            JoinHandle<()>,
            Arc<SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>>,
        ),
    > {
        lease_cell.generation.revoke();
        let cancelled = lease_cell.cancel_after_generation_revocation()
            || matches!(
                &*lease_cell
                    .state
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner()),
                SsTestExecutionGraphWorkerLeaseCellState::Cancelled
            );
        if !cancelled {
            return Err((handle, lease_cell));
        }
        let acceptance = SsTestExecutionGraphWorkerQuarantineAcceptanceReceipt {
            cell_identity: lease_cell.identity,
        };
        if acceptance.cell_identity != lease_cell.identity
            || !lease_cell.accept_cancelled_into_execution_uncertain()
        {
            return Err((handle, lease_cell));
        }
        Ok(Self {
            handle,
            lease_cell,
            acceptance,
        })
    }

    fn join_until_or_accept_quarantine(
        handle: JoinHandle<()>,
        lease_cell: Arc<
            SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>,
        >,
        deadline_at: Instant,
    ) -> SsTestExecutionGraphWorkerBoundedJoin<T, O> {
        lease_cell.generation.revoke();
        let _ = lease_cell.cancel_after_generation_revocation();
        while !handle.is_finished() && Instant::now() < deadline_at {
            std::thread::yield_now();
        }
        if handle.is_finished() {
            return SsTestExecutionGraphWorkerBoundedJoin::Joined {
                lease_cell,
                observation: handle.join(),
            };
        }
        match Self::accept(handle, lease_cell) {
            Ok(quarantine) => SsTestExecutionGraphWorkerBoundedJoin::Quarantined(quarantine),
            Err(_) => unreachable!(
                "an unfinished worker whose generation was revoked retains cancelled custody"
            ),
        }
    }

    fn into_execution_uncertain_settlement_after_late_observation(
        self,
    ) -> SsTestExecutionGraphWorkerExecutionUncertain<T> {
        let Self {
            handle,
            lease_cell,
            acceptance,
        } = self;
        assert_eq!(
            acceptance.cell_identity, lease_cell.identity,
            "quarantine acceptance remains correlated to the exact lease cell"
        );
        let late_worker_panicked = handle.join().is_err();
        let lease_cell = match Arc::try_unwrap(lease_cell) {
            Ok(lease_cell) => lease_cell,
            Err(_) => panic!("joined quarantined worker must release its execution grant"),
        };
        let state = lease_cell
            .state
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(
            matches!(
                state,
                SsTestExecutionGraphWorkerLeaseCellState::ExecutionUncertain
            ),
            "late worker observation cannot settle or replace quarantined custody"
        );
        SsTestExecutionGraphWorkerExecutionUncertain {
            input: lease_cell.input,
            acceptance,
            late_worker_panicked,
        }
    }
}

impl<T, O> SsTestExecutionGraphWorkerLeaseCell<T, SsTestExecutionGraphWorkerTerminal<O>>
where
    T: Send + 'static,
    O: Send + 'static,
{
    fn admit(
        input: T,
        generation: Arc<SsTestExecutionGraphWorkerGeneration>,
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    ) -> Arc<Self> {
        Arc::new(Self {
            identity: SS_TEST_EXECUTION_GRAPH_NEXT_WORKER_LEASE_CELL_ID
                .fetch_add(1, Ordering::Relaxed),
            generation,
            worker_ticket,
            input,
            state: Mutex::new(SsTestExecutionGraphWorkerLeaseCellState::Admitted),
        })
    }

    fn execution_grant(cell: &Arc<Self>) -> SsTestExecutionGraphWorkerExecutionGrant<T, O> {
        SsTestExecutionGraphWorkerExecutionGrant {
            cell: Arc::clone(cell),
            cell_identity: cell.identity,
            generation_serial: cell.generation.serial,
            worker_ticket: cell.worker_ticket.clone(),
        }
    }

    fn consume_unstarted_admission(cell: Arc<Self>) -> Result<T, Arc<Self>> {
        let cell = match Arc::try_unwrap(cell) {
            Ok(cell) => cell,
            Err(cell) => return Err(cell),
        };
        let state = cell
            .state
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if matches!(state, SsTestExecutionGraphWorkerLeaseCellState::Admitted) {
            Ok(cell.input)
        } else {
            Err(Arc::new(Self {
                identity: cell.identity,
                generation: cell.generation,
                worker_ticket: cell.worker_ticket,
                input: cell.input,
                state: Mutex::new(state),
            }))
        }
    }

    fn consume_joined_settlement(
        cell: Arc<Self>,
    ) -> Result<(T, SsTestExecutionGraphWorkerTerminal<O>), Arc<Self>> {
        let cell = match Arc::try_unwrap(cell) {
            Ok(cell) => cell,
            Err(cell) => return Err(cell),
        };
        let state = cell
            .state
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match state {
            SsTestExecutionGraphWorkerLeaseCellState::Settled(terminal) => {
                Ok((cell.input, terminal))
            }
            state => {
                let cell = Arc::new(Self {
                    identity: cell.identity,
                    generation: cell.generation,
                    worker_ticket: cell.worker_ticket,
                    input: cell.input,
                    state: Mutex::new(state),
                });
                Err(cell)
            }
        }
    }

    fn cancel_after_generation_revocation(&self) -> bool {
        if self.generation.is_active() {
            return false;
        }
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match &*state {
            SsTestExecutionGraphWorkerLeaseCellState::Admitted
            | SsTestExecutionGraphWorkerLeaseCellState::Running => {
                *state = SsTestExecutionGraphWorkerLeaseCellState::Cancelled;
                true
            }
            _ => false,
        }
    }

    fn settle_spawn_refusal(&self) -> bool {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if matches!(&*state, SsTestExecutionGraphWorkerLeaseCellState::Admitted) {
            *state = SsTestExecutionGraphWorkerLeaseCellState::Settled(
                SsTestExecutionGraphWorkerTerminal::SpawnRefused,
            );
            true
        } else {
            false
        }
    }

    fn accept_cancelled_into_execution_uncertain(&self) -> bool {
        if self.generation.is_active() {
            return false;
        }
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if matches!(&*state, SsTestExecutionGraphWorkerLeaseCellState::Cancelled) {
            *state = SsTestExecutionGraphWorkerLeaseCellState::ExecutionUncertain;
            true
        } else {
            false
        }
    }
}

impl<T, O> SsTestExecutionGraphWorkerExecutionGrant<T, O>
where
    T: Send + 'static,
    O: Send + 'static,
{
    fn stage_terminal(
        self,
        operation: impl FnOnce(&T, &SsTestExecutionGraphWorkerEffectPermit) -> O,
    ) {
        let valid_grant = self.cell.identity == self.cell_identity
            && self.cell.generation.serial == self.generation_serial
            && self.cell.worker_ticket == self.worker_ticket;
        let mut state = self
            .cell
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(&*state, SsTestExecutionGraphWorkerLeaseCellState::Admitted) {
            return;
        }
        *state = SsTestExecutionGraphWorkerLeaseCellState::Running;
        if !valid_grant || !self.cell.generation.is_active() {
            *state = SsTestExecutionGraphWorkerLeaseCellState::Settled(
                SsTestExecutionGraphWorkerTerminal::GenerationRevoked,
            );
            return;
        }
        drop(state);
        let effect_permit = SsTestExecutionGraphWorkerEffectPermit {
            generation: Arc::clone(&self.cell.generation),
            generation_serial: self.generation_serial,
        };
        let terminal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            operation(&self.cell.input, &effect_permit)
        }))
        .map(SsTestExecutionGraphWorkerTerminal::Completed)
        .unwrap_or(SsTestExecutionGraphWorkerTerminal::Panicked);
        let mut state = self
            .cell
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if matches!(&*state, SsTestExecutionGraphWorkerLeaseCellState::Running) {
            *state = if self.cell.generation.is_active() {
                SsTestExecutionGraphWorkerLeaseCellState::Settled(terminal)
            } else {
                SsTestExecutionGraphWorkerLeaseCellState::Settled(
                    SsTestExecutionGraphWorkerTerminal::GenerationRevoked,
                )
            };
        }
    }
}

struct SsTestExecutionGraphOwnedSourceFactWorker {
    handle: Option<JoinHandle<()>>,
    lease_cell: Arc<
        SsTestExecutionGraphWorkerLeaseCell<
            SsSourceWorkSetLeasedSourceFactStepForSsTestExecutionOwnerV1,
            SsTestExecutionGraphWorkerTerminal<()>,
        >,
    >,
}

struct SsTestExecutionGraphDiscoveryWorkerInput {
    walk: SsTestRunPlanDiscoveryWalkForSsTestExecutionOwnerV1,
    current_dir: PathBuf,
}

struct SsTestExecutionGraphDiscoveryWorkerTerminal {
    candidates: Vec<SsTestRunPlanDiscoveredTestFileForSsTestExecutionOwnerV1>,
    walk_result: Result<usize, SsTestPlanError>,
}

struct SsTestExecutionGraphOwnedDiscoveryWorker {
    handle: Option<JoinHandle<()>>,
    lease_cell: Arc<
        SsTestExecutionGraphWorkerLeaseCell<
            SsTestExecutionGraphDiscoveryWorkerInput,
            SsTestExecutionGraphWorkerTerminal<SsTestExecutionGraphDiscoveryWorkerTerminal>,
        >,
    >,
}

#[derive(Default)]
struct SsTestExecutionGraphOwnedWorkerSet {
    source_fact_workers: BTreeMap<u64, SsTestExecutionGraphOwnedSourceFactWorker>,
    discovery_workers: BTreeMap<u64, SsTestExecutionGraphOwnedDiscoveryWorker>,
}

impl SsTestExecutionGraphOwnedWorkerSet {
    fn insert_source_fact(
        &mut self,
        worker_ticket: &SsTestExecutionGraphWorkerTicket,
        worker: SsTestExecutionGraphOwnedSourceFactWorker,
    ) {
        assert!(
            self.source_fact_workers
                .insert(worker_ticket.serial, worker)
                .is_none(),
            "one owned source-fact worker per graph ticket"
        );
    }

    fn take_source_fact(
        &mut self,
        worker_ticket: &SsTestExecutionGraphWorkerTicket,
    ) -> Option<SsTestExecutionGraphOwnedSourceFactWorker> {
        self.source_fact_workers.remove(&worker_ticket.serial)
    }

    fn insert_discovery(
        &mut self,
        worker_ticket: &SsTestExecutionGraphWorkerTicket,
        worker: SsTestExecutionGraphOwnedDiscoveryWorker,
    ) {
        assert!(
            self.discovery_workers
                .insert(worker_ticket.serial, worker)
                .is_none(),
            "one owned discovery worker per graph ticket"
        );
    }

    fn take_discovery(
        &mut self,
        worker_ticket: &SsTestExecutionGraphWorkerTicket,
    ) -> Option<SsTestExecutionGraphOwnedDiscoveryWorker> {
        self.discovery_workers.remove(&worker_ticket.serial)
    }
}

struct SsTestSourceWorkSetAdmissionSession {
    graph_session: SsTestExecutionGraphSession,
    candidate_set_observation: Option<Value>,
    admission_profile: SsTestSourceWorkSetAdmissionProfile,
}

pub(in crate::test_runner) struct SsTestExecutionGraphOpenAdmission {
    graph_session: SsTestExecutionGraphSession,
}

/// One runtime-file work item consumed into a pool dispatch: the lease that
/// will settle it, the Run-frame observation facts, and the graph-owned
/// liveness deadline. The child rebuilds the file from source; no prepared
/// authority crosses the process boundary.
struct SsTestExecutionGraphRuntimeFilePoolDispatchWork {
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    selected_source: Option<SsSelectedSourceTestFile>,
    dispatched_source: Option<SsPoolDispatchedSelectedSourceTestFile>,
    test_name_pattern: Option<String>,
    timeout_request: Option<SsTestTimeoutRequest>,
    default_timeout: SsTestExecutionGraphDefaultTimeout,
    timeout_deadline: SsTestExecutionGraphEffectiveTimeoutDeadline,
    pending_dispatch_liveness: SsTestExecutionGraphRuntimeFilePoolPendingDispatchLiveness,
    dispatch_deadline_at: Instant,
    dispatch_started_at: Option<Instant>,
    last_heartbeat_at: Option<Instant>,
    heartbeat_count: u64,
    compiler_phases: SsTestCompilerWorkerPhaseLedger,
}

/// Private, fixed-retention process-boundary phase attribution. Worker-id
/// dispatch ownership associates these observations with the selected file;
/// the observations themselves make no path, fingerprint, or identity claim.
struct SsTestCompilerWorkerPhaseLedger {
    last_observed_at: Option<Instant>,
    observed_count: u8,
    sequence_anomaly_count: u8,
    last_sequence_anomaly: Option<SsTestCompilerWorkerPhaseSequenceAnomaly>,
    recent: VecDeque<SsTestCompilerWorkerPhaseObservation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SsTestCompilerWorkerPhaseSequenceAnomaly {
    preceding: Option<SsTestCompilerWorkerPhaseObservation>,
    observed: SsTestCompilerWorkerPhaseObservation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SsTestExecutionGraphRuntimeFilePoolPendingDispatchLiveness {
    deadline_at: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SsTestExecutionGraphDefaultTimeout {
    deadline: Duration,
    observation: &'static str,
}

struct SsTestExecutionGraphSourceFactWorkerEventObligation {
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    liveness_deadline_at: Instant,
}

struct SsTestExecutionGraphSourceFactWorkerSettlement {
    worker_ticket: SsTestExecutionGraphWorkerTicket,
}

enum SsTestExecutionGraphEventWaitObligation {
    RuntimeFilePool {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        deadline_at: Instant,
    },
    RuntimeFilePoolTimeout {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        deadline_at: Instant,
    },
    SourceFact {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        deadline_at: Instant,
    },
    Discovery {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        deadline_at: Instant,
    },
}

impl SsTestExecutionGraphEventWaitObligation {
    fn deadline_at(&self) -> Instant {
        match self {
            Self::RuntimeFilePool { deadline_at, .. }
            | Self::RuntimeFilePoolTimeout { deadline_at, .. }
            | Self::SourceFact { deadline_at, .. }
            | Self::Discovery { deadline_at, .. } => *deadline_at,
        }
    }
}

struct SsTestExecutionGraphDiscoveryWorkerEventObligation {
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    liveness_deadline_at: Instant,
}

enum SsTestExecutionGraphDiscoveryWorkerEvent {
    CandidateDiscovered {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        candidate: SsTestRunPlanDiscoveredTestFileForSsTestExecutionOwnerV1,
    },
    WalkClosed {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        discovered_file_count: usize,
    },
    OwnedTerminalReady {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
}

enum SsTestExecutionGraphRuntimeFilePoolWorkerObservation {
    Frame(SsTestPoolWorkerParentObservedFrame),
    FrameReadRefused,
}

/// Sendable worker observations only. Broad `SsError` remains in the
/// coordinator because its runtime variant may retain provider-session
/// authority that belongs to the owning thread.
enum SsTestExecutionGraphCoordinatorEvent {
    SourceFactWorkerSettled(SsTestExecutionGraphSourceFactWorkerSettlement),
    DiscoveryWorkerObserved(SsTestExecutionGraphDiscoveryWorkerEvent),
    RuntimeFilePoolWorkerFrame {
        worker_id: usize,
        observation: SsTestExecutionGraphRuntimeFilePoolWorkerObservation,
    },
    /// A dispatched (or, defensively, pending) runtime-file pool node reached
    /// its graph-owned liveness deadline. Contained per-node exactly like a
    /// worker-channel loss: the coordinator recovers the dispatch work by
    /// worker ticket and settles that one file through the pool worker-loss
    /// lane, then the run continues to terminal projection. Never whole-run
    /// fatal.
    RuntimeFilePoolWorkerEventLivenessElapsed {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    RuntimeFilePoolWorkerTimeoutElapsed {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
}

struct SsTestExecutionGraphCoordinatorContext {
    worker_generation: Arc<SsTestExecutionGraphWorkerGeneration>,
    owned_workers: SsTestExecutionGraphOwnedWorkerSet,
    live_runtime_plan_feed: Option<SsTestExecutionGraphLiveRuntimePlanFeed>,
    source_work_set_live_admission_inventory: Option<SsTestSourceWorkSetLiveAdmissionInventory>,
    batch_source_count: usize,
    package_resolution_root_count: usize,
    source_work_set_inventory:
        Option<Arc<SsSourceWorkSetAdmittedSourceInventoryForSsTestExecutionOwnerV1>>,
    source_fact_streaming_admission:
        Option<SsSourceWorkSetStreamingFactAdmissionForSsTestExecutionOwnerV1>,
    source_fact_extra_package_resolution_roots: Arc<Vec<PathBuf>>,
    selected_source_feed_emissions: VecDeque<SsTestParentSelectedSourceDispatchAdmission>,
    runtime_file_leased_work: VecDeque<SsTestExecutionGraphRuntimeFileExecutionWorkerInput>,
    runtime_file_started_work: VecDeque<SsTestExecutionGraphRuntimeFileExecutionWorkerInput>,
    runtime_file_pool: SsTestPoolWorkerParentPool,
    runtime_file_pool_idle_workers: VecDeque<usize>,
    runtime_file_pool_pending_dispatch: VecDeque<SsTestExecutionGraphRuntimeFilePoolDispatchWork>,
    runtime_file_pool_dispatched: BTreeMap<usize, SsTestExecutionGraphRuntimeFilePoolDispatchWork>,
    runtime_file_pool_unexpected_termination_settlement_count: usize,
    selected_source_settled_count: usize,
    selected_source_failed_count: usize,
    runtime_file_last_progress_emitted_at: Instant,
    /// Pool worker ids whose dispatched node was already settled as
    /// liveness-lost. The worker was killed at settle time, but bytes it had
    /// already flushed can still surface as one late frame; those frames are
    /// recorded as a non-fatal ledger observation and never re-settled.
    runtime_file_pool_liveness_lost_workers: BTreeSet<usize>,
    runtime_file_execution_session: Option<SsTestExecutionGraphRuntimeFileExecutionSession>,
    runtime_file_execution_requested: bool,
    runtime_file_failure_feed_closed: bool,
    runtime_file_failure_feed_drained: bool,
    runtime_file_live_source_closed: bool,
    graph_event_sender: mpsc::Sender<SsTestExecutionGraphCoordinatorEvent>,
    graph_event_receiver: mpsc::Receiver<SsTestExecutionGraphCoordinatorEvent>,
    discovery_walk_closed: bool,
    discovery_started_at: Instant,
    candidate_selection_session: Option<
        ss_runtime_test_plan_owner::SsTestRunPlanCandidateSelectionSessionForSsTestExecutionOwnerV1,
    >,
    discovered_candidate_count: usize,
    selected_candidate_count: usize,
    // Quarantined test files that failed source-fact derivation and never
    // joined the applied selected-test-source inventory; each one still
    // settles as exactly one failure feed emission, so this count extends
    // runtime-plan coverage at source-fact family close.
    unadmitted_test_source_failure_count: usize,
    feed_emission_obligations: Option<SsTestExecutionGraphFeedEmissionObligationLedger>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SsTestExecutionGraphCoordinatorWorkSourceKind {
    SourceFactDerivation,
    SourceWorkSetFailureCommit,
    QueuedGraphEvent,
    RuntimeFileExecution,
    GraphClose,
    SourceFactWorkerEventObligationMissing,
    DiscoveryWorkerEventObligationMissing,
    AwaitGraphEvent,
    NoProgress,
}

enum SsTestExecutionGraphCoordinatorWorkSource {
    SourceFactDerivation,
    SourceWorkSetFailureCommit,
    QueuedGraphEvent(SsTestExecutionGraphCoordinatorEvent),
    RuntimeFileExecution,
    GraphClose,
    SourceFactWorkerEventObligationMissing {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    DiscoveryWorkerEventObligationMissing {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    AwaitGraphEvent {
        wait_obligation: SsTestExecutionGraphEventWaitObligation,
    },
    NoProgress,
}

impl SsTestExecutionGraphCoordinatorWorkSource {
    fn kind(&self) -> SsTestExecutionGraphCoordinatorWorkSourceKind {
        match self {
            Self::SourceFactDerivation => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::SourceFactDerivation
            }
            Self::SourceWorkSetFailureCommit => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::SourceWorkSetFailureCommit
            }
            Self::QueuedGraphEvent(_) => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::QueuedGraphEvent
            }
            Self::RuntimeFileExecution => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::RuntimeFileExecution
            }
            Self::GraphClose => SsTestExecutionGraphCoordinatorWorkSourceKind::GraphClose,
            Self::SourceFactWorkerEventObligationMissing { .. } => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::SourceFactWorkerEventObligationMissing
            }
            Self::DiscoveryWorkerEventObligationMissing { .. } => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::DiscoveryWorkerEventObligationMissing
            }
            Self::AwaitGraphEvent { .. } => {
                SsTestExecutionGraphCoordinatorWorkSourceKind::AwaitGraphEvent
            }
            Self::NoProgress => SsTestExecutionGraphCoordinatorWorkSourceKind::NoProgress,
        }
    }
}

enum SsTestExecutionGraphCoordinatorStepReceipt {
    SourceFactWorkStartedObserved {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    SourceFactWorkerOutcomeCaptured {
        closure_ready_admitted_count: usize,
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        pending_source_fact_count: usize,
    },
    SourceFactFamilyClosedObserved {
        source_file_count: usize,
        package_resolution_root_count: usize,
        dependency_edge_count: usize,
        dependency_cyclic_component_count: usize,
    },
    DiscoveredCandidateObserved {
        selected_test_source_admitted: bool,
    },
    DiscoveryFamilyClosedObserved {
        discovered_file_count: usize,
        discovery_elapsed: Duration,
        candidate_selection_counts:
            Option<ss_runtime_test_plan_owner::SsTestRunPlanCandidateSelectionCountsForSsTestExecutionOwnerV1>,
    },
    SourceWorkSetFailureCommittedObserved {
        receipt: SsTestExecutionGraphTransactionReceipt,
    },
    RuntimeFileReadyWorkAdmitted {
        admitted_count: usize,
    },
    RuntimeFileFailureFeedClosedObserved,
    RuntimeFileFailureAdmitted {
        admitted_count: usize,
    },
    RuntimeFileFailureFeedDrainedObserved,
    RuntimeFileWorkLeased {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    RuntimeFileWorkStartedObserved {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    RuntimeFilePoolWorkerSpawned {
        worker_id: usize,
    },
    RuntimeFilePoolDispatchQueued,
    RuntimeFilePoolWorkerReady {
        worker_id: usize,
    },
    RuntimeFilePoolRunDispatched {
        worker_id: usize,
    },
    RuntimeFilePoolWorkerLost {
        worker_id: usize,
    },
    /// A dispatched worker proved it is still alive; the coordinator re-armed
    /// that node's liveness deadline. Progress observation only.
    RuntimeFilePoolWorkerHeartbeatObserved {
        worker_id: usize,
    },
    /// A process-isolated selected-file worker reported one truthful coarse
    /// phase. It is retained in the dispatch's bounded diagnostic ring but
    /// does not renew liveness; only the periodic heartbeat does that.
    RuntimeFilePoolCompilerPhaseObserved {
        worker_id: usize,
    },
    /// A late frame arrived from a worker whose node was already settled as
    /// liveness-lost. Recorded and dropped without a second settlement.
    RuntimeFilePoolLivenessLostLateFrameObserved {
        worker_id: usize,
    },
    RuntimeFileWorkerOutcomeCaptured {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    },
    RuntimeFileOutcomeCommittedObserved {
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        settled_runtime_file_count: usize,
        failed_runtime_file_count: usize,
    },
    RuntimeFileCacheHitCommittedObserved {
        cache_hit_count: usize,
    },
    RuntimeFileLiveSourceClosedObserved,
    GraphCloseBlocked {
        active_worker_count: usize,
        ready_runtime_file_queue_count: usize,
        ready_node_count: usize,
        leased_node_count: usize,
        running_node_count: usize,
        running_observed_node_count: usize,
        outcome_captured_node_count: usize,
        commit_pending_node_count: usize,
        captured_runtime_file_outcome_count: usize,
    },
    GraphClosed {
        finished: TestRunFinished,
    },
}

struct SsTestExecutionGraphSourceWorkSetFailureCommitPending {
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    kind: SsTestExecutionGraphSourceWorkSetFailureTransactionKind,
    failure: PackageGraphTestFileSourceWorkSetFailure,
}

struct SsTestExecutionGraphCommitPendingNode {
    family: SsTestExecutionGraphNodeFamily,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    kind: SsTestExecutionGraphSourceWorkSetFailureTransactionKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphSourceWorkSetFailureTransactionKind {
    PreGenerationFailure,
}

enum SsTestExecutionGraphTransactionReceipt {
    SourceWorkSetFailureSettledObserved { settled_failure_count: usize },
}

struct SsTestExecutionGraphTransactionKernel;

struct SsTestExecutionGraphSession {
    opened_at: Instant,
    scheduler: SsTestExecutionGraphSchedulerState,
    nodes: Vec<SsTestExecutionGraphNodeState>,
    source_work_set_failure_commit_pending:
        VecDeque<SsTestExecutionGraphSourceWorkSetFailureCommitPending>,
    observed_source_work_set_failure_count: usize,
    discovery_worker_event_obligations:
        VecDeque<SsTestExecutionGraphDiscoveryWorkerEventObligation>,
    source_fact_worker_event_obligations:
        VecDeque<SsTestExecutionGraphSourceFactWorkerEventObligation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SsTestExecutionGraphSchedulerPolicy {
    execution_mode: SsTestExecutionGraphSchedulerExecutionMode,
    runtime_file_affinity: SsTestExecutionGraphRuntimeFileAffinityPolicy,
    liveness: SsTestExecutionGraphLivenessPolicy,
    timeout: SsTestExecutionGraphTimeoutPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SsTestExecutionGraphLivenessPolicy {
    runtime_plan_background_deadline: Duration,
    spawned_worker_child_deadline: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphTimeoutPolicy {
    InvocationTimeout {
        request: SsTestTimeoutRequest,
    },
    DefaultTimeout {
        deadline: Duration,
        observation: &'static str,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphEffectiveTimeoutDeadline {
    NoTimeoutConfigured,
    Deadline {
        started_at: Instant,
        deadline_at: Instant,
        request: SsTestTimeoutRequest,
    },
    DefaultDeadline {
        started_at: Instant,
        deadline_at: Instant,
        observation: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphSchedulerExecutionMode {
    SerialOwner,
    BoundedWorkerPool { worker_limit: NonZeroUsize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphRuntimeFileAffinityPolicy {
    OwnerLaneOnly,
    GraphWorkerEligible,
}

struct SsTestExecutionGraphSchedulerState {
    policy: SsTestExecutionGraphSchedulerPolicy,
    next_worker_ticket: u64,
    active_worker_count: usize,
}

enum SsTestExecutionGraphNodeState {
    Waiting(SsTestExecutionGraphWaitingNode),
    Ready(SsTestExecutionGraphReadyNode),
    Leased(SsTestExecutionGraphLeasedNode),
    Running(SsTestExecutionGraphRunningNode),
    RunningObserved(SsTestExecutionGraphRunningObservedNode),
    OutcomeCaptured(SsTestExecutionGraphOutcomeCapturedNode),
    CommitPending(SsTestExecutionGraphCommitPendingNode),
    Settled(SsTestExecutionGraphSettledNode),
}

struct SsTestExecutionGraphWaitingNode {
    family: SsTestExecutionGraphNodeFamily,
    dependency_count: usize,
}

struct SsTestExecutionGraphReadyNode {
    family: SsTestExecutionGraphNodeFamily,
}

struct SsTestExecutionGraphLeasedNode {
    family: SsTestExecutionGraphNodeFamily,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
}

struct SsTestExecutionGraphRunningNode {
    family: SsTestExecutionGraphNodeFamily,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
}

struct SsTestExecutionGraphRunningObservedNode {
    family: SsTestExecutionGraphNodeFamily,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
    timeout_deadline: SsTestExecutionGraphEffectiveTimeoutDeadline,
}

struct SsTestExecutionGraphOutcomeCapturedNode {
    family: SsTestExecutionGraphNodeFamily,
    worker_ticket: SsTestExecutionGraphWorkerTicket,
}

struct SsTestExecutionGraphSettledNode {
    family: SsTestExecutionGraphNodeFamily,
    outcome: SsTestExecutionGraphNodeOutcome,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SsTestExecutionGraphWorkerTicket {
    serial: u64,
    scheduler_policy: SsTestExecutionGraphSchedulerPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SsTestExecutionGraphNodeFamily {
    Discovery,
    CandidateAdmission,
    SourceFacts,
    PreGenerationFailure,
    PackageReadiness,
    DependencyReadiness,
    RuntimeFileExecution,
    RunCloseout,
}

enum SsTestExecutionGraphNodeOutcome {
    DiscoveryClosed {
        discovered_file_count: usize,
    },
    CandidateNodesAdmitted {
        selected_candidate_count: usize,
    },
    SourceFactDerivedApplied {
        enqueued_work_count: usize,
    },
    SourceFactTestFileFailureRecorded,
    SourceFactDependencyFailureRecorded,
    PreGenerationFailed {
        settled_failure_count: usize,
    },
    PackageReadinessAdmitted {
        source_file_count: usize,
        package_resolution_root_count: usize,
    },
    DependencyReadinessAdmitted {
        source_file_count: usize,
        edge_count: usize,
        cyclic_component_count: usize,
    },
    RuntimeFileExecutionSettled {
        settled_count: usize,
    },
    RuntimeFileExecutionCacheHit {
        cache_hit_count: usize,
    },
    RuntimeFileExecutionFailed {
        failed_count: usize,
    },
    RunCloseoutSucceeded {
        terminal_summary:
            Option<SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner>,
    },
    RunCloseoutFailed,
}

struct SsTestExecutionGraphRuntimePlanFileEmissionInput {
    path: PathBuf,
    package_root: String,
}

fn ss_test_runner_config_for_source_path(
    path: &Path,
    extra_package_resolution_roots: &[PathBuf],
) -> SsResult<crate::SsRunnerConfig> {
    Ok(crate::ss_test_runner_config_for_source_path(
        path,
        extra_package_resolution_roots,
    )?)
}

pub(super) fn execute_discovered_source_work_set_execution_graph_for_execution_graph_owner(
    session: SsTestRunnerSession,
    graph_open_admission: SsTestExecutionGraphOpenAdmission,
) -> SsResult<TestRunState<TestRunFinished>> {
    let mut session = session;
    let finished = SsTestSourceWorkSetAdmissionSession::execute_discovered_source_work_set_execution_graph_for_execution_graph_owner(
        &mut session,
        graph_open_admission,
    )?;
    Ok(TestRunState::admit(session, finished))
}

pub(in crate::test_runner) fn open_streaming_discovery_execution_graph_for_ss_test_execution_owner(
    session: &SsTestRunnerSession,
) -> SsTestExecutionGraphOpenAdmission {
    SsTestExecutionGraphOpenAdmission {
        graph_session: SsTestExecutionGraphSession::open(
            Instant::now(),
            session
                .invocation()
                .scheduler_width()
                .map(SsTestSchedulerWidthRequest::worker_limit),
            session
                .invocation()
                .timeout_request_for_ss_test_execution_graph_owner_v1(),
            SsTestExecutionGraphDefaultTimeout::for_invocation_for_execution_graph_owner(
                session.invocation(),
            ),
        ),
    }
}

struct SsTestExecutionGraphDiscoveryWalkSink<'a> {
    candidates: Vec<SsTestRunPlanDiscoveredTestFileForSsTestExecutionOwnerV1>,
    effect_permit: &'a SsTestExecutionGraphWorkerEffectPermit,
}

impl SsTestRunPlanDiscoveredTestFileSinkForSsTestExecutionOwnerV1
    for SsTestExecutionGraphDiscoveryWalkSink<'_>
{
    fn admit_discovered_test_file_for_ss_test_execution_owner_v1(
        &mut self,
        candidate: SsTestRunPlanDiscoveredTestFileForSsTestExecutionOwnerV1,
    ) -> Result<(), SsTestPlanError> {
        if !self.effect_permit.is_active() {
            return Err(SsTestPlanError::Cli(
                "discovery walk generation was revoked before candidate publication".to_owned(),
            ));
        }
        self.candidates.push(candidate);
        Ok(())
    }
}

impl SsTestSourceWorkSetAdmissionProfile {
    fn new() -> Self {
        Self {
            stage_records: Vec::new(),
        }
    }

    pub(in crate::test_runner) fn to_projection_value(&self) -> Value {
        json!({
            "schema": "swarm.ss.test.source_work_set.admission_profile.v1",
            "stages": self.stage_records,
        })
    }
}

struct SsTestExecutionGraphRuntimePlanSourceCoverage {
    total_file_count: usize,
    source_work_set_receipt_file_count: SsTestSourceWorkSetReceiptFileCount,
}

struct SsTestExecutionGraphLiveRuntimePlanFeed {
    live_runtime_plan_emission_session: SsTestSourceWorkSetRuntimePlanAdmissionFeed,
    feed_emission_obligations: SsTestExecutionGraphFeedEmissionObligationLedger,
}

/// Settlement-obligation ledger replacing the retired whole-set feed
/// cardinality conservation check: one obligation is minted per selected
/// test source at closure-ready admission and consumed by the exactly-one
/// feed emission (prepared, file-product failure, or source-work-set
/// failure) that settles that file. Feed close requires an empty ledger, so
/// a selected file that never reaches the feed is a typed fault naming the
/// file instead of a count drift.
#[derive(Default)]
struct SsTestExecutionGraphFeedEmissionObligationLedger {
    outstanding: BTreeSet<String>,
    consumed_emission_count: usize,
}

impl SsTestExecutionGraphFeedEmissionObligationLedger {
    fn mint_for_selected_test_source(&mut self, source_path: &str) {
        self.outstanding.insert(source_path.to_owned());
    }

    fn consume_for_feed_emission(&mut self, source_path: &Path) -> SsResult<()> {
        let key = source_path.display().to_string();
        if !self.outstanding.remove(&key) {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_feed_emission_obligation_missing_for_emission",
                    "reason": "every runtime-plan feed emission must consume the feed-emission obligation minted for that selected test source at closure-ready admission; an emission without an outstanding obligation is a double settlement or an unselected-source emission",
                    "sourcePath": key,
                })
                .to_string(),
            ));
        }
        self.consumed_emission_count += 1;
        Ok(())
    }

    fn close_check_for_feed_close(&self) -> SsResult<()> {
        if self.outstanding.is_empty() {
            return Ok(());
        }
        let outstanding_sample = self.outstanding.iter().take(8).cloned().collect::<Vec<_>>();
        Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                "code": "ss_test_execution_graph_feed_emission_obligation_outstanding",
                "reason": "runtime-plan feed close requires every feed-emission obligation to be consumed; an outstanding obligation means a selected test source was admitted but never settled into exactly one feed emission",
                "outstandingObligationCount": self.outstanding.len(),
                "consumedEmissionCount": self.consumed_emission_count,
                "outstandingSourcePaths": outstanding_sample,
            })
            .to_string(),
        ))
    }
}

impl SsTestExecutionGraphRuntimePlanSourceCoverage {
    fn open_live_runtime_plan_feed_for_execution_graph_owner(
        self,
        source_work_set_generation: SsTestSourceWorkSetGeneration,
        candidate_set_observation: Option<Value>,
        worker_limit: Option<NonZeroUsize>,
        runtime_plan_owner_session: crate::test_runner::SsTestRuntimePlanOwnerSession,
        runtime_plan_background_liveness_deadline: Duration,
        spawned_worker_child_liveness_deadline: Duration,
        feed_emission_obligations: SsTestExecutionGraphFeedEmissionObligationLedger,
    ) -> SsResult<SsTestExecutionGraphLiveRuntimePlanFeed> {
        let source_work_set_feed_admission = source_work_set_generation
            .consume_into_runtime_plan_feed_admission_for_ss_test_execution_owner_v1(
                self.source_work_set_receipt_file_count,
            )
            .map_err(|source| SsError::Cli(source.to_string()))?;
        let live_runtime_plan_emission_session =
            SsTestSourceWorkSetRuntimePlanAdmissionFeed::admit(
                source_work_set_feed_admission,
                self.total_file_count,
                candidate_set_observation,
                worker_limit,
                runtime_plan_owner_session,
                runtime_plan_background_liveness_deadline,
                spawned_worker_child_liveness_deadline,
            )?;
        Ok(SsTestExecutionGraphLiveRuntimePlanFeed {
            live_runtime_plan_emission_session,
            feed_emission_obligations,
        })
    }
}

impl SsTestExecutionGraphLiveRuntimePlanFeed {
    fn admit_selected_source_feed_emission_for_execution_graph_owner(
        &mut self,
        selected_source: SsTestParentSelectedSourceDispatchAdmission,
    ) {
        self.live_runtime_plan_emission_session
            .admit_selected_source_feed_emission(selected_source);
    }

    fn take_next_selected_source_readiness_for_execution_graph_owner(
        &mut self,
    ) -> Option<SsSelectedSourceTestFile> {
        self.live_runtime_plan_emission_session
            .take_next_selected_source_readiness_for_execution_graph_owner()
    }

    fn admit_source_work_set_failure_for_execution_graph_owner(
        &mut self,
        failure: PackageGraphTestFileSourceWorkSetFailure,
    ) -> SsResult<()> {
        let (path, package_root, error, preparation_terminal) = failure.into_parts();
        self.feed_emission_obligations
            .consume_for_feed_emission(&path)?;
        self.live_runtime_plan_emission_session
            .admit_source_work_set_failure_feed_emission(
                path,
                package_root,
                error,
                preparation_terminal,
            )
    }

    fn admit_source_work_set_admission_profile_for_execution_graph_owner(
        &mut self,
        profile: SsTestSourceWorkSetAdmissionProfile,
    ) {
        self.live_runtime_plan_emission_session
            .admit_source_work_set_admission_profile(profile);
    }

    fn close_file_failure_feed_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<()> {
        self.live_runtime_plan_emission_session
            .close_file_failure_feed_for_execution_graph_owner(session)
    }

    fn admit_next_file_failure_to_live_source_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileFailureFeedAdmission> {
        self.live_runtime_plan_emission_session
            .admit_next_file_failure_to_live_source_for_execution_graph_owner(session)
    }

    fn admit_next_runtime_file_ready_work_for_execution_graph_owner(
        &mut self,
        runtime_file_execution_session: &mut SsTestExecutionGraphRuntimeFileExecutionSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileReadyWorkAdmission> {
        self.live_runtime_plan_emission_session
            .admit_next_runtime_file_ready_work_for_execution_graph_owner(
                runtime_file_execution_session,
            )
    }

    fn execute_runtime_file_worker_input_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        worker_input: SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement> {
        self.live_runtime_plan_emission_session
            .execute_runtime_file_worker_input_for_execution_graph_owner(session, worker_input)
    }

    fn commit_admitted_pool_worker_settlement_for_execution_graph_owner(
        &mut self,
        admitted: SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        self.live_runtime_plan_emission_session
            .commit_admitted_pool_worker_settlement_for_execution_graph_owner(admitted)
    }

    fn settle_pool_worker_loss_for_execution_graph_owner(
        &mut self,
        dispatched_source: SsPoolDispatchedSelectedSourceTestFile,
        worker_loss_fault: &serde_json::Value,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        self.live_runtime_plan_emission_session
            .settle_pool_worker_loss_for_execution_graph_owner(dispatched_source, worker_loss_fault)
    }

    fn close_for_execution_graph_owner(
        self,
        session: &mut SsTestRunnerSession,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner> {
        let summary = self
            .live_runtime_plan_emission_session
            .close_for_execution_graph_owner(session, graph_settlements)?;
        self.feed_emission_obligations
            .close_check_for_feed_close()?;
        Ok(summary)
    }
}

pub(in crate::test_runner) enum SsTestExecutionGraphRuntimeFileReadyWorkAdmission {
    Admitted { admitted_count: usize },
    Pending,
    Closed,
}

pub(in crate::test_runner) enum SsTestExecutionGraphRuntimeFileFailureFeedAdmission {
    Admitted { admitted_count: usize },
    Pending,
    Closed,
}

struct PackageGraphTestFileSourceWorkSetFailure {
    path: PathBuf,
    package_root: String,
    error: SsError,
    preparation_terminal:
        Option<crate::test_runner::preparation_terminal::SsTestPreparationTerminalSeed>,
}

struct SsTestExecutionGraphSelectedSourceFailureNode {
    source_work_set_failure: PackageGraphTestFileSourceWorkSetFailure,
}

impl SsTestExecutionGraphSelectedSourceFailureNode {
    fn admit_source_work_set_failure(
        file_emission_input: SsTestExecutionGraphRuntimePlanFileEmissionInput,
        error: SsError,
    ) -> Self {
        Self {
            source_work_set_failure: PackageGraphTestFileSourceWorkSetFailure::admit(
                file_emission_input,
                error,
            ),
        }
    }

    fn into_source_work_set_failure(self) -> PackageGraphTestFileSourceWorkSetFailure {
        self.source_work_set_failure
    }
}

impl PackageGraphTestFileSourceWorkSetFailure {
    fn admit(
        file_emission_input: SsTestExecutionGraphRuntimePlanFileEmissionInput,
        error: SsError,
    ) -> Self {
        let SsTestExecutionGraphRuntimePlanFileEmissionInput { path, package_root } =
            file_emission_input;
        Self {
            path,
            package_root,
            error,
            preparation_terminal: None,
        }
    }

    fn admit_with_preparation_terminal(
        file_emission_input: SsTestExecutionGraphRuntimePlanFileEmissionInput,
        error: SsError,
        preparation_terminal: Option<
            crate::test_runner::preparation_terminal::SsTestPreparationTerminalSeed,
        >,
    ) -> Self {
        let SsTestExecutionGraphRuntimePlanFileEmissionInput { path, package_root } =
            file_emission_input;
        Self {
            path,
            package_root,
            error,
            preparation_terminal,
        }
    }

    fn into_parts(
        self,
    ) -> (
        PathBuf,
        String,
        SsError,
        Option<crate::test_runner::preparation_terminal::SsTestPreparationTerminalSeed>,
    ) {
        (
            self.path,
            self.package_root,
            self.error,
            self.preparation_terminal,
        )
    }
}

#[cfg(test)]
mod tests;
