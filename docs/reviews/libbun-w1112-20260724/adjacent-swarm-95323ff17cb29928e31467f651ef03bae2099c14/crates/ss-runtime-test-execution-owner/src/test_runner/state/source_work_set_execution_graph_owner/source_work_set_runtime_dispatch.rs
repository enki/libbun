use super::*;

const SS_TEST_COMPILER_WORKER_PHASE_HANDSHAKE_PATH_ENV: &str =
    "SWARM_SS_TEST_COMPILER_WORKER_PHASE_HANDSHAKE_PATH";

fn record_test_only_compiler_phase_handshake(
    pool: &SsTestPoolWorkerParentPool,
    worker_id: usize,
    phase: SsTestCompilerWorkerPhaseObservation,
) {
    use std::io::Write as _;

    let Some(path) = std::env::var_os(SS_TEST_COMPILER_WORKER_PHASE_HANDSHAKE_PATH_ENV) else {
        return;
    };
    let Some(child_pid) = pool.child_pid_observation_for_execution_graph_owner_v1(worker_id) else {
        return;
    };
    let record = json!({
        "schema": "swarm.ss.test.compiler_worker_phase_handshake.v1",
        "workerId": worker_id,
        "childPid": child_pid,
        "phase": phase.as_str(),
        "phaseOrdinal": phase.ordinal(),
    });
    let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    else {
        return;
    };
    // This file is an integration-test observation seam only. Failure to
    // retain it must never affect worker liveness, termination, or settlement.
    let _ = writeln!(file, "{record}");
}

#[allow(clippy::too_many_arguments)]
fn compiler_worker_fault_for_dispatched_file(
    dispatch_work: &SsTestExecutionGraphRuntimeFilePoolDispatchWork,
    code: &str,
    kind: &str,
    reason: &str,
    worker_id: usize,
    in_flight_count: usize,
    pending_count: usize,
    termination: Value,
    cause: Value,
) -> Value {
    let observed_at = Instant::now();
    json!({
        "schema": "swarm.ss.test.compiler_worker_fault.v1",
        "code": code,
        "kind": kind,
        "reason": reason,
        "workerId": worker_id,
        "selectedWork": {
            "workerTicketSerial": dispatch_work.worker_ticket.serial,
        },
        "compilerPhaseObservation": dispatch_work.compiler_worker_phase_observation_for_execution_graph_owner(
            observed_at,
            in_flight_count,
            pending_count,
        ),
        "termination": termination,
        "cause": cause,
    })
}

impl SsTestSourceWorkSetAdmissionSession {
    pub(super) fn capture_graph_coordinator_event_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        graph_event: SsTestExecutionGraphCoordinatorEvent,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        match graph_event {
            SsTestExecutionGraphCoordinatorEvent::SourceFactWorkerSettled(worker_settlement) => {
                self.capture_source_fact_worker_settlement_for_execution_graph_owner(
                    coordinator_context,
                    worker_settlement,
                )
            }
            SsTestExecutionGraphCoordinatorEvent::DiscoveryWorkerObserved(discovery_event) => self
                .capture_discovery_worker_event_for_execution_graph_owner(
                    coordinator_context,
                    discovery_event,
                ),
            SsTestExecutionGraphCoordinatorEvent::RuntimeFilePoolWorkerFrame {
                worker_id,
                observation,
            } => self.capture_runtime_file_pool_worker_frame_for_execution_graph_owner(
                session,
                coordinator_context,
                worker_id,
                observation,
            ),
            SsTestExecutionGraphCoordinatorEvent::RuntimeFilePoolWorkerEventLivenessElapsed {
                worker_ticket,
            } => self.settle_runtime_file_pool_liveness_deadline_for_execution_graph_owner(
                session,
                coordinator_context,
                worker_ticket,
            ),
            SsTestExecutionGraphCoordinatorEvent::RuntimeFilePoolWorkerTimeoutElapsed {
                worker_ticket,
            } => self.settle_runtime_file_pool_timeout_deadline_for_execution_graph_owner(
                session,
                coordinator_context,
                worker_ticket,
            ),
        }
    }

    /// Contain a runtime-file pool node whose graph-owned liveness deadline
    /// elapsed: recover its dispatch work by worker ticket from the dispatched
    /// map (a live child is executing it) or, defensively, from the pending
    /// queue (never dispatched), mint a typed per-file liveness fault carrying
    /// the settle-with-cause context, and route it through the same pool
    /// worker-loss settle lane the channel-closed path uses. A dispatched
    /// worker is killed and quarantined so its in-flight run cannot produce a
    /// second settlement; the run continues to terminal projection.
    pub(super) fn settle_runtime_file_pool_liveness_deadline_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let liveness_deadline_secs =
            SS_TEST_EXECUTION_GRAPH_SPAWNED_WORKER_CHILD_LIVENESS_DEADLINE.as_secs();
        let configured_width = self.graph_worker_limit().map(NonZeroUsize::get);
        let in_flight_count = coordinator_context.runtime_file_pool_dispatched.len();
        let pending_count = coordinator_context.runtime_file_pool_pending_dispatch.len();

        // Dispatched: a live child is still executing this node.
        if let Some(worker_id) = coordinator_context
            .runtime_file_pool_dispatched
            .iter()
            .find(|(_, dispatch_work)| dispatch_work.worker_ticket == worker_ticket)
            .map(|(worker_id, _)| *worker_id)
        {
            let Some(dispatch_work) = coordinator_context
                .runtime_file_pool_dispatched
                .remove(&worker_id)
            else {
                return Err(SsError::Fault(json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_runtime_file_pool_liveness_dispatch_unavailable",
                    "reason": "runtime-file liveness settlement lost the dispatched work before typed per-file settlement",
                    "workerId": worker_id,
                    "workerTicketSerial": worker_ticket.serial,
                })));
            };
            let runtime_observation =
                dispatch_work.runtime_observation_for_execution_graph_owner(Instant::now());
            // Kill and quarantine the still-running child: it must not re-lease
            // or produce a second settlement. A late frame it already flushed is
            // recorded as a ledger observation (see the frame capture entry).
            let termination = coordinator_context
                .runtime_file_pool
                .remove_worker_for_execution_graph_owner_v1(worker_id);
            coordinator_context
                .runtime_file_pool_idle_workers
                .retain(|idle_worker_id| *idle_worker_id != worker_id);
            coordinator_context
                .runtime_file_pool_liveness_lost_workers
                .insert(worker_id);
            let liveness_fault = compiler_worker_fault_for_dispatched_file(
                &dispatch_work,
                "ss_test_compiler_worker_watchdog_exhausted",
                "admitted_watchdog_exhaustion",
                "compiler worker reached its graph-owned liveness watchdog without a boundary observation; the worker's selected file settles as a typed fault and unrelated admitted work continues",
                worker_id,
                in_flight_count,
                pending_count,
                termination,
                json!({
                    "workerTicketSerial": worker_ticket.serial,
                    "dispatchState": "dispatched",
                    "livenessDeadlineSecs": liveness_deadline_secs,
                    "configuredPoolWidth": configured_width,
                    "runtimeObservation": runtime_observation,
                }),
            );
            return self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                session,
                coordinator_context,
                dispatch_work,
                &liveness_fault,
            );
        }

        // The obligation's worker ticket must name exactly one live dispatch
        // work item; if neither map holds it the coordinator's obligation set
        // has drifted from its dispatch state, which is a kernel invariant
        // violation.
        Err(SsError::Fault(json!({
            "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
            "code": "ss_test_execution_graph_runtime_file_pool_liveness_ticket_unmatched",
            "reason": "runtime-file pool liveness elapse named a worker ticket held by neither the dispatched map nor the pending queue; the coordinator obligation set drifted from its dispatch state",
            "workerTicketSerial": worker_ticket.serial,
        })))
    }

    pub(super) fn settle_runtime_file_pool_timeout_deadline_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        worker_ticket: SsTestExecutionGraphWorkerTicket,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let configured_width = self.graph_worker_limit().map(NonZeroUsize::get);
        let in_flight_count = coordinator_context.runtime_file_pool_dispatched.len();
        let pending_count = coordinator_context.runtime_file_pool_pending_dispatch.len();
        if let Some(worker_id) = coordinator_context
            .runtime_file_pool_dispatched
            .iter()
            .find(|(_, dispatch_work)| dispatch_work.worker_ticket == worker_ticket)
            .map(|(worker_id, _)| *worker_id)
        {
            let Some(dispatch_work) = coordinator_context
                .runtime_file_pool_dispatched
                .remove(&worker_id)
            else {
                return Err(SsError::Fault(json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_runtime_file_pool_timeout_dispatch_unavailable",
                    "reason": "runtime-file timeout settlement lost the dispatched work before typed per-file settlement",
                    "workerId": worker_id,
                    "workerTicketSerial": worker_ticket.serial,
                })));
            };
            let timeout_observation = dispatch_work
                .timeout_deadline
                .timeout_observation_for_execution_graph_owner()
                .map(str::to_owned);
            let runtime_observation =
                dispatch_work.runtime_observation_for_execution_graph_owner(Instant::now());
            let termination = coordinator_context
                .runtime_file_pool
                .remove_worker_for_execution_graph_owner_v1(worker_id);
            coordinator_context
                .runtime_file_pool_idle_workers
                .retain(|idle_worker_id| *idle_worker_id != worker_id);
            coordinator_context
                .runtime_file_pool_liveness_lost_workers
                .insert(worker_id);
            let timeout_fault = compiler_worker_fault_for_dispatched_file(
                &dispatch_work,
                "ss_test_compiler_worker_invocation_watchdog_exhausted",
                "admitted_watchdog_exhaustion",
                "compiler/test worker reached its admitted invocation watchdog before terminal settlement; the worker's selected file settles as a typed fault and unrelated admitted work continues",
                worker_id,
                in_flight_count,
                pending_count,
                termination,
                json!({
                    "workerTicketSerial": worker_ticket.serial,
                    "dispatchState": "dispatched",
                    "timeout": timeout_observation,
                    "configuredPoolWidth": configured_width,
                    "runtimeObservation": runtime_observation,
                }),
            );
            return self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                session,
                coordinator_context,
                dispatch_work,
                &timeout_fault,
            );
        }

        Err(SsError::Fault(json!({
            "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
            "code": "ss_test_execution_graph_runtime_file_pool_timeout_ticket_unmatched",
            "reason": "runtime-file pool timeout elapse named a worker ticket held by no dispatched worker; timeout authority must only be armed at pool dispatch",
            "workerTicketSerial": worker_ticket.serial,
        })))
    }

    /// One pool worker frame becomes exactly one coordinator receipt: Ready
    /// marks the worker idle and immediately pull-dispatches pending work;
    /// an authenticated worker settlement consumes exact dispatched-source
    /// custody into admitted result or typed-refusal settlement; a channel
    /// error consumes that custody through typed worker-loss settlement and
    /// removes the worker (the pool respawns lazily while ready work remains).
    pub(super) fn capture_runtime_file_pool_worker_frame_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        worker_id: usize,
        observation: SsTestExecutionGraphRuntimeFilePoolWorkerObservation,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        // Late-frame containment: this worker's dispatched node was already
        // settled as liveness-lost and the worker was killed. Any frame that
        // still arrives (a buffered settlement, a Ready, a heartbeat, or the
        // channel-close error) is recorded and dropped — never a second
        // settlement, never a re-lease, never a silent drop. Keep the worker
        // quarantined out of the idle set.
        if coordinator_context
            .runtime_file_pool_liveness_lost_workers
            .contains(&worker_id)
        {
            coordinator_context
                .runtime_file_pool_idle_workers
                .retain(|idle_worker_id| *idle_worker_id != worker_id);
            return Ok(
                SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolLivenessLostLateFrameObserved {
                    worker_id,
                },
            );
        }
        match observation {
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::Claim(claim),
            ) => {
                if let Err(error) = coordinator_context
                    .runtime_file_pool
                    .admit_claim_and_write_receipt_for_execution_graph_owner_v1(worker_id, claim)
                {
                    let termination = coordinator_context
                        .runtime_file_pool
                        .remove_worker_for_execution_graph_owner_v1(worker_id);
                    coordinator_context
                        .runtime_file_pool_liveness_lost_workers
                        .insert(worker_id);
                    let dispatch_work = coordinator_context
                        .runtime_file_pool_dispatched
                        .remove(&worker_id)
                        .ok_or_else(|| {
                            SsError::Fault(json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_claim_refusal_without_dispatch",
                                "reason": "a Claim refusal must consume its exact dispatched reservation through worker-loss settlement",
                                "workerId": worker_id,
                            }))
                        })?;
                    let worker_loss_fault = json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_pool_worker_claim_or_receipt_refused",
                        "reason": "the pool worker Claim admission or ClaimReceipt write failed; the worker was terminated and its dispatched reservation retained for typed loss settlement",
                        "cause": error.into_fault_observation_value(),
                        "termination": termination,
                    });
                    return self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                        session,
                        coordinator_context,
                        dispatch_work,
                        &worker_loss_fault,
                    );
                }
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerHeartbeatObserved {
                        worker_id,
                    },
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::Running(running),
            ) => {
                if let Err(error) = coordinator_context
                    .runtime_file_pool
                    .admit_running_for_execution_graph_owner_v1(worker_id, running)
                {
                    let termination = coordinator_context
                        .runtime_file_pool
                        .remove_worker_for_execution_graph_owner_v1(worker_id);
                    coordinator_context
                        .runtime_file_pool_liveness_lost_workers
                        .insert(worker_id);
                    let dispatch_work = coordinator_context
                        .runtime_file_pool_dispatched
                        .remove(&worker_id)
                        .ok_or_else(|| {
                            SsError::Fault(json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_running_refusal_without_dispatch",
                                "reason": "a Running refusal must consume its exact dispatched reservation through worker-loss settlement",
                                "workerId": worker_id,
                            }))
                        })?;
                    let worker_loss_fault = json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_pool_worker_running_refused",
                        "reason": "the pool worker Running admission failed; the worker was terminated and its dispatched reservation retained for typed loss settlement",
                        "cause": error.into_fault_observation_value(),
                        "termination": termination,
                    });
                    return self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                        session,
                        coordinator_context,
                        dispatch_work,
                        &worker_loss_fault,
                    );
                }
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerHeartbeatObserved {
                        worker_id,
                    },
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::CompilerPhase(phase),
            ) => {
                let observed_for_dispatch = if let Some(dispatch_work) = coordinator_context
                    .runtime_file_pool_dispatched
                    .get_mut(&worker_id)
                {
                    dispatch_work
                        .observe_compiler_phase_for_execution_graph_owner(phase, Instant::now());
                    true
                } else {
                    false
                };
                if observed_for_dispatch {
                    record_test_only_compiler_phase_handshake(
                        &coordinator_context.runtime_file_pool,
                        worker_id,
                        phase,
                    );
                }
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolCompilerPhaseObserved {
                        worker_id,
                    },
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::Heartbeat,
            ) => {
                // The worker is alive and still executing its dispatched file.
                // Re-arm that node's liveness deadline so a slow-but-alive file
                // under contention is never falsely settled as liveness-lost.
                if let Some(dispatch_work) = coordinator_context
                    .runtime_file_pool_dispatched
                    .get_mut(&worker_id)
                {
                    let observed_at = Instant::now();
                    dispatch_work.dispatch_deadline_at = observed_at
                        + SS_TEST_EXECUTION_GRAPH_SPAWNED_WORKER_CHILD_LIVENESS_DEADLINE;
                    dispatch_work.last_heartbeat_at = Some(observed_at);
                    dispatch_work.heartbeat_count = dispatch_work.heartbeat_count.saturating_add(1);
                }
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerHeartbeatObserved {
                        worker_id,
                    },
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::Ready,
            ) => {
                coordinator_context
                    .runtime_file_pool_idle_workers
                    .push_back(worker_id);
                if let Some(receipt) =
                    Self::pump_runtime_file_pool_dispatch_for_execution_graph_owner(
                        coordinator_context,
                        self.graph_worker_limit(),
                    )?
                {
                    return Ok(receipt);
                }
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerReady {
                        worker_id,
                    },
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                SsTestPoolWorkerParentObservedFrame::AuthenticatedSettlement(settlement),
            ) => {
                let dispatch_work = coordinator_context
                    .runtime_file_pool_dispatched
                    .get_mut(&worker_id)
                    .ok_or_else(|| {
                        SsError::Fault(json!({
                            "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                            "code": "ss_test_execution_graph_runtime_file_pool_terminal_without_dispatch",
                            "reason": "terminal staging requires the exact retained dispatched selected-source carrier while its registry slot remains Running",
                            "workerId": worker_id,
                        }))
                    })?;
                let dispatched_source = dispatch_work.dispatched_source.take().ok_or_else(|| {
                    SsError::Fault(json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_pool_terminal_reentered",
                        "reason": "authenticated terminal staging must consume the dispatched selected-source carrier exactly once",
                        "workerId": worker_id,
                    }))
                })?;
                let terminal = match coordinator_context
                    .runtime_file_pool
                    .admit_settlement_for_execution_graph_owner_v1(
                        worker_id,
                        settlement,
                        dispatched_source,
                    ) {
                    Ok(terminal) => terminal,
                    Err(refusal) => {
                        let dispatch_work = coordinator_context
                            .runtime_file_pool_dispatched
                            .get_mut(&worker_id)
                            .expect(
                                "the exact dispatch remains installed through terminal refusal",
                            );
                        dispatch_work.dispatched_source = Some(refusal.dispatched_source);
                        let termination = coordinator_context
                            .runtime_file_pool
                            .remove_worker_for_execution_graph_owner_v1(worker_id);
                        coordinator_context
                            .runtime_file_pool_liveness_lost_workers
                            .insert(worker_id);
                        let dispatch_work = coordinator_context
                            .runtime_file_pool_dispatched
                            .remove(&worker_id)
                            .ok_or_else(|| {
                                SsError::Fault(json!({
                                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                    "code": "ss_test_execution_graph_runtime_file_pool_refused_terminal_without_dispatch",
                                    "reason": "a refused authenticated terminal must consume its exact dispatched selected-source carrier through the worker-loss lane",
                                    "workerId": worker_id,
                                }))
                            })?;
                        let worker_loss_fault = json!({
                            "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                            "code": "ss_test_pool_worker_authenticated_terminal_refused",
                            "reason": "the pool worker terminal failed exact authenticated registry or cargo admission",
                            "cause": refusal.error.into_fault_observation_value(),
                            "termination": termination,
                        });
                        return self
                            .settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                                session,
                                coordinator_context,
                                dispatch_work,
                                &worker_loss_fault,
                            );
                    }
                };
                self.settle_runtime_file_pool_frame_for_execution_graph_owner(
                    session,
                    coordinator_context,
                    worker_id,
                    terminal,
                )
            }
            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::FrameReadRefused => {
                let in_flight_count = coordinator_context.runtime_file_pool_dispatched.len();
                let pending_count = coordinator_context.runtime_file_pool_pending_dispatch.len();
                let termination = coordinator_context
                    .runtime_file_pool
                    .remove_worker_for_execution_graph_owner_v1(worker_id);
                coordinator_context
                    .runtime_file_pool_idle_workers
                    .retain(|idle_worker_id| *idle_worker_id != worker_id);
                let Some(dispatch_work) = coordinator_context
                    .runtime_file_pool_dispatched
                    .remove(&worker_id)
                else {
                    return Ok(
                        SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerLost {
                            worker_id,
                        },
                    );
                };
                coordinator_context.runtime_file_pool_unexpected_termination_settlement_count =
                    coordinator_context
                        .runtime_file_pool_unexpected_termination_settlement_count
                        .saturating_add(1);
                let owner_settlement_ordinal =
                    coordinator_context.runtime_file_pool_unexpected_termination_settlement_count;
                let worker_ticket_serial = dispatch_work.worker_ticket.serial;
                let mut worker_loss_fault = compiler_worker_fault_for_dispatched_file(
                    &dispatch_work,
                    "ss_test_compiler_worker_unexpected_termination",
                    "unexpected_termination_or_signal",
                    "compiler/test worker channel closed before its selected file settled; its bounded recent compiler phase observations are retained and unrelated admitted work continues",
                    worker_id,
                    in_flight_count,
                    pending_count,
                    termination,
                    json!({
                        "kind": "pool_worker_frame_channel_closed",
                        "workerId": worker_id,
                    }),
                );
                worker_loss_fault["ownerUnexpectedTerminationSettlement"] = json!({
                    "ordinal": owner_settlement_ordinal,
                    "workerTicketSerial": worker_ticket_serial,
                });
                self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                    session,
                    coordinator_context,
                    dispatch_work,
                    &worker_loss_fault,
                )
            }
        }
    }

    pub(super) fn settle_runtime_file_pool_frame_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        worker_id: usize,
        terminal: SsTestPoolWorkerParentPreparedSettlementCargo,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let mut dispatch_work = coordinator_context
            .runtime_file_pool_dispatched
            .remove(&worker_id)
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_pool_settlement_without_dispatch",
                        "reason": "pool worker settlement frames must correspond to exactly one dispatched selected source",
                        "workerId": worker_id,
                    })
                    .to_string(),
                )
            })?;
        let admitted = match terminal {
            SsTestPoolWorkerParentPreparedSettlementCargo::RuntimeRefusal {
                refusal,
                dispatched_source,
            } => {
                dispatch_work.dispatched_source = Some(dispatched_source);
                let (code, reason) = match refusal.kind {
                    SsTestPoolWorkerRuntimeRefusalKind::SettlementCardinality => (
                        "ss_test_pool_worker_settlement_cardinality_refused",
                        "worker-local ss-test execution produced an invalid settlement cardinality",
                    ),
                    SsTestPoolWorkerRuntimeRefusalKind::ChildExecution => (
                        "ss_test_pool_worker_child_execution_refused",
                        "worker-local package admission, compilation, or execution refused",
                    ),
                    SsTestPoolWorkerRuntimeRefusalKind::ChildPanic => (
                        "ss_test_pool_worker_child_execution_panicked",
                        "worker-local package admission, compilation, or execution panicked",
                    ),
                };
                let worker_loss_fault = json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": code,
                    "reason": reason,
                    "cause": refusal.observation,
                    "capturedStdout": String::from_utf8_lossy(&refusal.captured_stdout),
                    "capturedStdoutTruncated": refusal.captured_stdout_truncated,
                    "capturedStderr": String::from_utf8_lossy(&refusal.captured_stderr),
                    "capturedStderrTruncated": refusal.captured_stderr_truncated,
                });
                return self.settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
                    session,
                    coordinator_context,
                    dispatch_work,
                    &worker_loss_fault,
                );
            }
            SsTestPoolWorkerParentPreparedSettlementCargo::ExecutedFile(admitted) => admitted,
        };
        let result = coordinator_context
            .live_runtime_plan_feed_mut_for_execution_graph_owner()?
            .commit_admitted_pool_worker_settlement_for_execution_graph_owner(admitted);
        self.commit_selected_source_graph_settlement_for_execution_graph_owner(
            session,
            coordinator_context,
            dispatch_work.worker_ticket,
            result,
            false,
        )
    }

    pub(super) fn settle_runtime_file_pool_worker_loss_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        mut dispatch_work: SsTestExecutionGraphRuntimeFilePoolDispatchWork,
        worker_loss_fault: &serde_json::Value,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let dispatched_source = dispatch_work.dispatched_source.take().ok_or_else(|| {
            SsError::Fault(json!({
                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                "code": "ss_test_execution_graph_worker_loss_without_dispatched_source",
                "reason": "selected-source worker loss must consume the exact authenticated dispatched-source carrier",
                "workerTicketSerial": dispatch_work.worker_ticket.serial,
            }))
        })?;
        let result = coordinator_context
            .live_runtime_plan_feed_mut_for_execution_graph_owner()?
            .settle_pool_worker_loss_for_execution_graph_owner(
                dispatched_source,
                worker_loss_fault,
            );
        self.commit_selected_source_graph_settlement_for_execution_graph_owner(
            session,
            coordinator_context,
            dispatch_work.worker_ticket,
            result,
            true,
        )
    }

    fn commit_selected_source_graph_settlement_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        worker_ticket: SsTestExecutionGraphWorkerTicket,
        result: SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
        failed: bool,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let graph_settlement = result?;
        let adjudicated_settlement =
            session.publish_committed_runtime_file_for_execution_graph_owner(graph_settlement)?;
        coordinator_context
            .runtime_file_execution_session
            .as_mut()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_session_consumed",
                        "reason": "selected-source graph settlement must be retained before runtime-file closeout consumes the session",
                    })
                    .to_string(),
                )
            })?
            .retain_committed_runtime_file_settlement_after_publication_for_execution_graph_owner(
                adjudicated_settlement,
            )?;
        let outcome = if failed {
            coordinator_context.selected_source_failed_count = coordinator_context
                .selected_source_failed_count
                .saturating_add(1);
            SsTestExecutionGraphNodeOutcome::RuntimeFileExecutionFailed {
                failed_count: coordinator_context.selected_source_failed_count,
            }
        } else {
            coordinator_context.selected_source_settled_count = coordinator_context
                .selected_source_settled_count
                .saturating_add(1);
            SsTestExecutionGraphNodeOutcome::RuntimeFileExecutionSettled {
                settled_count: coordinator_context.selected_source_settled_count,
            }
        };
        self.graph_session
            .settle_running_observed_node(worker_ticket.clone(), outcome)?;
        Ok(
            SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileOutcomeCommittedObserved {
                worker_ticket,
                settled_runtime_file_count: coordinator_context.selected_source_settled_count,
                failed_runtime_file_count: coordinator_context.selected_source_failed_count,
            },
        )
    }

    pub(super) fn runtime_file_pool_spawn_ceiling_for_execution_graph_owner(
        graph_worker_limit: Option<NonZeroUsize>,
        outstanding_runtime_work: NonZeroUsize,
    ) -> NonZeroUsize {
        let configured_limit = graph_worker_limit.unwrap_or(NonZeroUsize::MIN);
        configured_limit.min(outstanding_runtime_work)
    }

    /// Pull dispatch: pair pending runtime-file work with an idle worker's
    /// standing Ready, or lazily spawn one more worker when pending work
    /// exists, no worker is idle, and the pool is below scheduler width.
    pub(super) fn pump_runtime_file_pool_dispatch_for_execution_graph_owner(
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        graph_worker_limit: Option<NonZeroUsize>,
    ) -> SsResult<Option<SsTestExecutionGraphCoordinatorStepReceipt>> {
        if coordinator_context
            .runtime_file_pool_pending_dispatch
            .is_empty()
        {
            return Ok(None);
        }
        if let Some(worker_id) = coordinator_context
            .runtime_file_pool_idle_workers
            .pop_front()
        {
            if coordinator_context
                .runtime_file_pool_dispatched
                .contains_key(&worker_id)
            {
                return Err(SsError::Fault(json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_idle_pool_worker_already_dispatched",
                    "reason": "an idle worker cannot receive another selected-source offer while it still owns dispatched work",
                    "workerId": worker_id,
                })));
            }
            let Some(dispatch_work) = coordinator_context
                .runtime_file_pool_pending_dispatch
                .front_mut()
            else {
                return Ok(None);
            };
            let extra_package_resolution_roots =
                Arc::clone(&coordinator_context.source_fact_extra_package_resolution_roots);
            let selected_source = dispatch_work.selected_source.take().ok_or_else(|| {
                SsError::Fault(json!({
                    "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                    "code": "ss_test_execution_graph_pending_selected_source_missing",
                    "reason": "pending pool dispatch must retain the exact selected-source readiness until authenticated offer dispatch succeeds",
                    "workerId": worker_id,
                }))
            })?;
            let dispatched_source = match selected_source
                .dispatch_to_pool_worker_for_execution_graph_owner(
                    &mut coordinator_context.runtime_file_pool,
                    worker_id,
                    extra_package_resolution_roots.as_slice(),
                    dispatch_work.test_name_pattern.as_deref(),
                    dispatch_work.timeout_request_observation_for_execution_graph_owner(),
                ) {
                Ok(dispatched_source) => dispatched_source,
                Err(refusal) => {
                    let (selected_source, write_fault) =
                        refusal.into_retry_for_execution_graph_owner();
                    dispatch_work.selected_source = Some(selected_source);
                    let termination = coordinator_context
                        .runtime_file_pool
                        .remove_worker_for_execution_graph_owner_v1(worker_id);
                    return Err(SsError::Fault(json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_run_offer_write_failed",
                        "reason": "the Run offer write failed; worker termination is recorded and the exact selected-source readiness remains at the pending queue front",
                        "workerId": worker_id,
                        "writeFault": write_fault.into_fault_observation_value(),
                        "workerTermination": termination,
                        "pendingReadinessRetained": true,
                    })));
                }
            };
            dispatch_work.dispatched_source = Some(dispatched_source);
            dispatch_work.arm_timeout_deadline_for_execution_graph_owner()?;
            dispatch_work.dispatch_deadline_at =
                Instant::now() + SS_TEST_EXECUTION_GRAPH_SPAWNED_WORKER_CHILD_LIVENESS_DEADLINE;
            let dispatch_work = coordinator_context
                .runtime_file_pool_pending_dispatch
                .pop_front()
                .ok_or_else(|| {
                    SsError::Fault(json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_pending_dispatch_missing_after_run_write",
                        "reason": "a successfully written selected-source offer must atomically move its retained pending reservation into the dispatched map",
                        "workerId": worker_id,
                    }))
                })?;
            match coordinator_context
                .runtime_file_pool_dispatched
                .entry(worker_id)
            {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(dispatch_work);
                }
                std::collections::btree_map::Entry::Occupied(_) => {
                    coordinator_context
                        .runtime_file_pool_pending_dispatch
                        .push_front(dispatch_work);
                    return Err(SsError::Fault(json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_pool_dispatch_replaced_live_reservation",
                        "reason": "installing a selected-source dispatch must never replace another live reservation",
                        "workerId": worker_id,
                    })));
                }
            }
            return Ok(Some(
                SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolRunDispatched {
                    worker_id,
                },
            ));
        }
        let Some(outstanding_runtime_work) = NonZeroUsize::new(
            coordinator_context
                .runtime_file_pool_pending_dispatch
                .len()
                .saturating_add(coordinator_context.runtime_file_pool_dispatched.len()),
        ) else {
            return Ok(None);
        };
        let pool_width = Self::runtime_file_pool_spawn_ceiling_for_execution_graph_owner(
            graph_worker_limit,
            outstanding_runtime_work,
        );
        if coordinator_context
            .runtime_file_pool
            .spawned_worker_count_for_execution_graph_owner_v1()
            < pool_width.get()
        {
            let (worker_id, child_stdout, child_stderr) = coordinator_context
                .runtime_file_pool
                .spawn_worker_for_execution_graph_owner_v1()?;
            let event_sender = coordinator_context.graph_event_sender.clone();
            spawn_detached_execution_graph_worker(
                format!("ss-test-pool-stdout-{worker_id}"),
                move || {
                    let mut child_stdout = child_stdout;
                    loop {
                        match read_child_frame_for_pool_worker_parent_v1(&mut child_stdout) {
                        Ok(frame) => {
                            if event_sender
                                .send(
                                    SsTestExecutionGraphCoordinatorEvent::RuntimeFilePoolWorkerFrame {
                                        worker_id,
                                        observation:
                                            SsTestExecutionGraphRuntimeFilePoolWorkerObservation::Frame(
                                                frame,
                                            ),
                                    },
                                )
                                .is_err()
                            {
                                return;
                            }
                        }
                        Err(_error) => {
                            let _ = event_sender.send(
                                SsTestExecutionGraphCoordinatorEvent::RuntimeFilePoolWorkerFrame {
                                    worker_id,
                                    observation:
                                        SsTestExecutionGraphRuntimeFilePoolWorkerObservation::FrameReadRefused,
                                },
                            );
                            return;
                        }
                    }
                    }
                },
            )?;
            spawn_detached_execution_graph_worker(
                format!("ss-test-pool-stderr-{worker_id}"),
                move || {
                    let mut child_stderr = child_stderr;
                    let _ = std::io::copy(&mut child_stderr, &mut std::io::sink());
                },
            )?;
            return Ok(Some(
                SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolWorkerSpawned {
                    worker_id,
                },
            ));
        }
        Ok(None)
    }

    pub(super) fn capture_discovery_worker_event_for_execution_graph_owner(
        &mut self,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        discovery_event: SsTestExecutionGraphDiscoveryWorkerEvent,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        match discovery_event {
            SsTestExecutionGraphDiscoveryWorkerEvent::OwnedTerminalReady { worker_ticket } => {
                let (discovery_input, terminal) =
                    Self::take_owned_discovery_worker_terminal_for_execution_graph_owner(
                        coordinator_context,
                        &worker_ticket,
                    )?;
                let terminal = match terminal {
                    SsTestExecutionGraphWorkerTerminal::Completed(terminal) => terminal,
                    SsTestExecutionGraphWorkerTerminal::Panicked => {
                        return Err(SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_discovery_walk_worker_panicked",
                                "reason": "discovery graph worker panicked while the exact discovery input remained in its graph-owned lease cell",
                                "workerTicketSerial": worker_ticket.serial,
                            })
                            .to_string(),
                        ));
                    }
                    SsTestExecutionGraphWorkerTerminal::GenerationRevoked => {
                        return Err(SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_discovery_worker_generation_revoked",
                                "reason": "discovery worker terminal publication was fenced after its graph generation was revoked",
                                "workerTicketSerial": worker_ticket.serial,
                            })
                            .to_string(),
                        ));
                    }
                    SsTestExecutionGraphWorkerTerminal::SpawnRefused => {
                        return Err(SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_discovery_worker_spawn_failed",
                                "reason": "graph coordinator could not spawn the owned discovery worker while the exact discovery input remained retained",
                                "workerTicketSerial": worker_ticket.serial,
                            })
                            .to_string(),
                        ));
                    }
                };
                for candidate in terminal.candidates {
                    let _ = self.capture_discovery_worker_event_for_execution_graph_owner(
                        coordinator_context,
                        SsTestExecutionGraphDiscoveryWorkerEvent::CandidateDiscovered {
                            worker_ticket: worker_ticket.clone(),
                            candidate,
                        },
                    )?;
                }
                let discovered_file_count = terminal.walk_result.map_err(SsError::from)?;
                drop(discovery_input);
                self.capture_discovery_worker_event_for_execution_graph_owner(
                    coordinator_context,
                    SsTestExecutionGraphDiscoveryWorkerEvent::WalkClosed {
                        worker_ticket,
                        discovered_file_count,
                    },
                )
            }
            SsTestExecutionGraphDiscoveryWorkerEvent::CandidateDiscovered {
                worker_ticket,
                candidate,
            } => {
                self.graph_session
                    .renew_discovery_worker_event_obligation(&worker_ticket)?;
                coordinator_context.discovered_candidate_count = coordinator_context
                    .discovered_candidate_count
                    .saturating_add(1);
                let candidate_selection_session = coordinator_context
                    .candidate_selection_session
                    .as_mut()
                    .ok_or_else(|| {
                        SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_candidate_selection_session_missing",
                                "reason": "discovered candidates must be selected by the run-plan candidate selection session while the discovery walk is open",
                            })
                            .to_string(),
                        )
                    })?;
                let selection = candidate_selection_session
                    .select_discovered_candidate_for_ss_test_execution_owner_v1(candidate)
                    .map_err(|error| SsError::Cli(error.to_string()))?;
                let selected_test_source_admitted = match selection {
                    ss_runtime_test_plan_owner::SsTestRunPlanCandidateSelectionForSsTestExecutionOwnerV1::Selected(
                        canonical_seed,
                    ) => {
                        self.seed_selected_source_work_path_for_execution_graph_owner(
                            coordinator_context,
                            canonical_seed,
                        )?;
                        true
                    }
                    ss_runtime_test_plan_owner::SsTestRunPlanCandidateSelectionForSsTestExecutionOwnerV1::Excluded
                    | ss_runtime_test_plan_owner::SsTestRunPlanCandidateSelectionForSsTestExecutionOwnerV1::Parked => {
                        false
                    }
                };
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::DiscoveredCandidateObserved {
                        selected_test_source_admitted,
                    },
                )
            }
            SsTestExecutionGraphDiscoveryWorkerEvent::WalkClosed {
                worker_ticket,
                discovered_file_count,
            } => {
                self.graph_session
                    .remove_discovery_worker_event_obligation(&worker_ticket)?;
                self.graph_session.capture_running_observed_node(
                    SsTestExecutionGraphNodeFamily::Discovery,
                    worker_ticket.clone(),
                )?;
                let candidate_selection_session = coordinator_context
                    .candidate_selection_session
                    .take()
                    .ok_or_else(|| {
                        SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_candidate_selection_session_missing_for_close",
                                "reason": "discovery walk close must consume the run-plan candidate selection session exactly once",
                            })
                            .to_string(),
                        )
                    })?;
                let selection_close = candidate_selection_session
                    .close_for_ss_test_execution_owner_v1()
                    .map_err(|error| SsError::Cli(error.to_string()))?;
                let (late_selected, candidate_set_observation, selection_counts) = selection_close
                    .consume_into_late_selected_seeds_and_observation_for_ss_test_execution_owner_v1();
                for canonical_seed in late_selected {
                    self.seed_selected_source_work_path_for_execution_graph_owner(
                        coordinator_context,
                        canonical_seed,
                    )?;
                }
                let candidate_selection_counts = candidate_set_observation
                    .is_some()
                    .then_some(selection_counts);
                self.candidate_set_observation = candidate_set_observation;
                self.graph_session.settle_candidate_nodes_admitted(
                    coordinator_context.selected_candidate_count,
                )?;
                coordinator_context.discovery_walk_closed = true;
                self.graph_session.settle_outcome_captured_node(
                    worker_ticket,
                    SsTestExecutionGraphNodeOutcome::DiscoveryClosed {
                        discovered_file_count,
                    },
                )?;
                Ok(
                    SsTestExecutionGraphCoordinatorStepReceipt::DiscoveryFamilyClosedObserved {
                        discovered_file_count,
                        discovery_elapsed: coordinator_context.discovery_started_at.elapsed(),
                        candidate_selection_counts,
                    },
                )
            }
        }
    }

    pub(super) fn seed_selected_source_work_path_for_execution_graph_owner(
        &mut self,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        canonical_seed: ss_runtime_test_plan_owner::SsTestRunPlanCanonicalSelectedSourceWorkPathForSsTestExecutionOwnerV1,
    ) -> SsResult<()> {
        let streaming_admission = coordinator_context
            .source_fact_streaming_admission
            .as_mut()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_source_fact_streaming_admission_missing_for_seed",
                        "reason": "discovered selected source paths must seed the graph-coordinator-owned streaming fact admission while the discovery walk is open; the source-fact family cannot close before discovery closes",
                    })
                    .to_string(),
                )
            })?;
        let (source_path, selected_test_source) =
            canonical_seed.consume_into_source_work_seed_for_ss_test_execution_owner_v1();
        let source_runner_config = ss_test_runner_config_for_source_path(
            &source_path,
            coordinator_context
                .source_fact_extra_package_resolution_roots
                .as_slice(),
        )?;
        streaming_admission
            .seed_discovered_selected_source_path_for_ss_test_execution_owner_v1(
                source_path,
                selected_test_source,
                source_runner_config,
            )
            .map_err(source_work_set_non_terminal_checker_error_to_ss_error)?;
        if selected_test_source {
            coordinator_context.selected_candidate_count = coordinator_context
                .selected_candidate_count
                .saturating_add(1);
        }
        Ok(())
    }

    pub(super) fn mint_feed_emission_obligations_for_closure_ready_admission(
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        closure_ready_selected_test_sources: &[SsSourceWorkSetClosureReadySelectedTestSourceForSsTestExecutionOwnerV1],
    ) {
        if closure_ready_selected_test_sources.is_empty() {
            return;
        }
        let feed_emission_obligations = coordinator_context
            .feed_emission_obligations
            .as_mut()
            .expect("feed-emission ledger was admitted before closure-ready custody commit");
        for closure_ready in closure_ready_selected_test_sources {
            feed_emission_obligations.mint_for_selected_test_source(
                closure_ready.source_path_for_ss_test_execution_owner_observation_v1(),
            );
        }
    }

    /// Settles quarantined test-source failures from a closure-ready take:
    /// each quarantined file mints its feed-emission obligation and settles
    /// as a pre-generation source-work-set failure graph node, so its typed
    /// refusal reaches the runtime-plan feed as exactly one per-file failure
    /// emission instead of aborting the run. Unadmitted files (derive-step
    /// quarantine) are additionally counted so runtime-plan coverage extends
    /// past the applied selected-test-source inventory.
    pub(super) fn prepare_quarantined_test_source_failures_for_execution_graph_owner(
        &mut self,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        quarantine_count: usize,
    ) -> SsResult<
        Vec<execution_graph_session::SsTestExecutionGraphPreparedPreGenerationFailureAdmission>,
    > {
        coordinator_context
            .feed_emission_obligations
            .as_ref()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_feed_emission_obligation_ledger_consumed_before_mint",
                        "reason": "quarantined selected-source settlement must retain its complete take until the feed-emission ledger and graph receipts are admitted",
                    })
                    .to_string(),
                )
            })?;
        let mut prepared = Vec::with_capacity(quarantine_count);
        for _ in 0..quarantine_count {
            prepared.push(
                self.graph_session
                    .prepare_pre_generation_failure_admission()?,
            );
        }
        Ok(prepared)
    }

    pub(super) fn commit_prepared_quarantined_test_source_failures_for_execution_graph_owner(
        &mut self,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        refused_selected_test_sources: Vec<
            SsSourceWorkSetRefusedSelectedTestSourceForSsTestExecutionOwnerV1,
        >,
        unadmitted_test_sources: Vec<SsSourceWorkSetUnadmittedTestSourceForSsTestExecutionOwnerV1>,
        prepared: Vec<
            execution_graph_session::SsTestExecutionGraphPreparedPreGenerationFailureAdmission,
        >,
    ) {
        let mut prepared = prepared.into_iter();
        for refused in refused_selected_test_sources {
            let (path, package_root, error) = refused
                .consume_into_quarantined_test_source_failure_for_ss_test_execution_owner_v1();
            self.commit_prepared_quarantined_test_source_failure_for_execution_graph_owner(
                coordinator_context,
                path,
                package_root,
                error,
                prepared
                    .next()
                    .expect("one pre-admitted graph receipt per quarantined selected source"),
            );
        }
        for unadmitted in unadmitted_test_sources {
            let (path, package_root, error) = unadmitted
                .consume_into_quarantined_test_source_failure_for_ss_test_execution_owner_v1();
            self.commit_prepared_quarantined_test_source_failure_for_execution_graph_owner(
                coordinator_context,
                path,
                package_root,
                error,
                prepared
                    .next()
                    .expect("one pre-admitted graph receipt per quarantined selected source"),
            );
            coordinator_context.unadmitted_test_source_failure_count = coordinator_context
                .unadmitted_test_source_failure_count
                .saturating_add(1);
        }
        debug_assert!(prepared.next().is_none());
    }

    pub(super) fn commit_prepared_quarantined_test_source_failure_for_execution_graph_owner(
        &mut self,
        coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        path: PathBuf,
        package_root: String,
        error: SsSourceWorkSetCheckerError,
        prepared: execution_graph_session::SsTestExecutionGraphPreparedPreGenerationFailureAdmission,
    ) {
        let feed_emission_obligations = coordinator_context
            .feed_emission_obligations
            .as_mut()
            .expect("feed-emission ledger was admitted before quarantine custody commit");
        let source_path = path.display().to_string();
        feed_emission_obligations.mint_for_selected_test_source(&source_path);
        let (cause, preparation_terminal) =
            error.into_selected_negative_terminal_custody_for_ss_test_execution_owner_v1();
        self.graph_session
            .commit_prepared_pre_generation_failure_admission(
            prepared,
            PackageGraphTestFileSourceWorkSetFailure::admit_with_preparation_terminal(
                SsTestExecutionGraphRuntimePlanFileEmissionInput { path, package_root },
                SsError::Fault(json!({
                    "schema": "swarm.ss.test.execution_dag_node_settlement_fault.v1",
                    "code": "ss_test_selected_test_source_admission_node_settled_failed",
                    "reason": "ss test source-work-set admission refused the selected test source's own import closure; the selected source settles as a source-work-set execution graph node; callers may not abort the run on a per-file admission refusal",
                    "sourcePath": source_path,
                    "cause": cause,
                })),
                preparation_terminal,
            ),
        );
    }

    pub(super) fn commit_runtime_file_cache_hit_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        graph_coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
        cache_hit_settlement: SsTestExecutionGraphRuntimeFileCacheHitSettlement,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let mut runtime_file_execution_session = graph_coordinator_context
            .runtime_file_execution_session
            .take()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_session_missing_for_cache_hit_commit",
                        "reason": "runtime-file cache-hit settlement must be committed by the graph coordinator while the runtime-file graph session is live",
                    })
                    .to_string(),
                )
            })?;
        let committed_publication = runtime_file_execution_session
            .commit_runtime_file_cache_hit_for_execution_graph_owner(cache_hit_settlement);
        let committed_receipt = committed_publication.and_then(|committed_publication| {
            let (cache_hit_count, graph_settlement) =
                committed_publication.into_parts_for_execution_graph_owner();
            let adjudicated_settlement = session
                .publish_committed_runtime_file_for_execution_graph_owner(graph_settlement)?;
            runtime_file_execution_session
                .retain_committed_runtime_file_settlement_after_publication_for_execution_graph_owner(
                    adjudicated_settlement,
                )?;
            Ok(cache_hit_count)
        });
        graph_coordinator_context.runtime_file_execution_session =
            Some(runtime_file_execution_session);
        let cache_hit_count = committed_receipt?;
        Ok(
            SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileCacheHitCommittedObserved {
                cache_hit_count,
            },
        )
    }

    pub(super) fn produce_runtime_file_execution_receipt_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        graph_coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
    ) -> SsResult<Option<SsTestExecutionGraphCoordinatorStepReceipt>> {
        let mut runtime_file_execution_session = graph_coordinator_context
            .runtime_file_execution_session
            .take()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_session_consumed",
                        "reason": "runtime-file graph session may be consumed exactly once by graph-owned closeout",
                    })
                    .to_string(),
                )
            })?;
        let graph_worker_limit = self.graph_worker_limit();
        let receipt = (|| {
            if runtime_file_execution_session
                .has_runtime_file_commit_progress_for_execution_graph_owner()
            {
                let Some(committed_publication) = runtime_file_execution_session
                    .commit_next_runtime_file_captured_outcome_for_execution_graph_owner()?
                else {
                    return Ok(None);
                };
                let (
                    worker_ticket,
                    settled_runtime_file_count,
                    failed_runtime_file_count,
                    graph_settlement,
                ) = committed_publication.into_parts_for_execution_graph_owner();
                let adjudicated_settlement = session
                    .publish_committed_runtime_file_for_execution_graph_owner(graph_settlement)?;
                runtime_file_execution_session
                    .retain_committed_runtime_file_settlement_after_publication_for_execution_graph_owner(
                        adjudicated_settlement,
                    )?;
                return Ok(Some(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileOutcomeCommittedObserved {
                        worker_ticket,
                        settled_runtime_file_count,
                        failed_runtime_file_count,
                    },
                ));
            }

            if self.graph_session.has_available_worker_slot() {
                if let Some(selected_source) = graph_coordinator_context
                    .live_runtime_plan_feed_mut_for_execution_graph_owner()?
                    .take_next_selected_source_readiness_for_execution_graph_owner()
                {
                    let worker_ticket = self.graph_session.admit_selected_source_pool_dispatch()?;
                    graph_coordinator_context
                        .runtime_file_pool_pending_dispatch
                        .push_back(
                            SsTestExecutionGraphRuntimeFilePoolDispatchWork::admit_selected_source_readiness(
                                worker_ticket,
                                selected_source,
                                session.invocation().cloned_test_name_pattern(),
                                session
                                    .invocation()
                                    .timeout_request_for_ss_test_execution_graph_owner_v1(),
                                SsTestExecutionGraphDefaultTimeout::for_invocation_for_execution_graph_owner(
                                    session.invocation(),
                                ),
                            ),
                        );
                    if let Some(receipt) =
                        Self::pump_runtime_file_pool_dispatch_for_execution_graph_owner(
                            graph_coordinator_context,
                            graph_worker_limit,
                        )?
                    {
                        return Ok(Some(receipt));
                    }
                    return Ok(Some(
                        SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFilePoolDispatchQueued,
                    ));
                }
            }

            if let Some(receipt) = Self::pump_runtime_file_pool_dispatch_for_execution_graph_owner(
                graph_coordinator_context,
                graph_worker_limit,
            )? {
                return Ok(Some(receipt));
            }

            if let Some(worker_input) = graph_coordinator_context
                .runtime_file_started_work
                .pop_front()
            {
                let worker_settlement = graph_coordinator_context
                    .live_runtime_plan_feed_mut_for_execution_graph_owner()?
                    .execute_runtime_file_worker_input_for_execution_graph_owner(
                        session,
                        worker_input,
                    )?;
                let worker_ticket = runtime_file_execution_session
                    .capture_runtime_file_execution_worker_settlement_for_execution_graph_owner(
                        worker_settlement,
                    )?;
                return Ok(Some(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileWorkerOutcomeCaptured {
                        worker_ticket,
                    },
                ));
            }

            if let Some(worker_input) = graph_coordinator_context
                .runtime_file_leased_work
                .pop_front()
            {
                let (worker_ticket, worker_input) = runtime_file_execution_session
                    .observe_runtime_file_worker_input_started_for_execution_graph_owner(
                        worker_input,
                    )?;
                graph_coordinator_context
                    .runtime_file_started_work
                    .push_back(worker_input);
                return Ok(Some(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileWorkStartedObserved {
                        worker_ticket,
                    },
                ));
            }

            if let Some((worker_ticket, worker_input)) = runtime_file_execution_session
                .lease_next_runtime_file_execution_worker_input_for_execution_graph_owner()?
            {
                graph_coordinator_context
                    .runtime_file_leased_work
                    .push_back(worker_input);
                return Ok(Some(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileWorkLeased {
                        worker_ticket,
                    },
                ));
            }

            if graph_coordinator_context.runtime_file_live_source_closed {
                // The live source is closed: only commit/pump/lease branches
                // above can still produce receipts on re-selection.
                return Ok(None);
            }

            if !graph_coordinator_context.runtime_file_failure_feed_closed {
                graph_coordinator_context
                    .live_runtime_plan_feed_mut_for_execution_graph_owner()?
                    .close_file_failure_feed_for_execution_graph_owner(session)?;
                graph_coordinator_context.runtime_file_failure_feed_closed = true;
                return Ok(Some(
                    SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileFailureFeedClosedObserved,
                ));
            }

            if !graph_coordinator_context.runtime_file_failure_feed_drained {
                match graph_coordinator_context
                    .live_runtime_plan_feed_mut_for_execution_graph_owner()?
                    .admit_next_file_failure_to_live_source_for_execution_graph_owner(session)?
                {
                    SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Admitted {
                        admitted_count,
                    } => {
                        return Ok(Some(
                            SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileFailureAdmitted {
                                admitted_count,
                            },
                        ));
                    }
                    SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Pending => {}
                    SsTestExecutionGraphRuntimeFileFailureFeedAdmission::Closed => {
                        graph_coordinator_context.runtime_file_failure_feed_drained = true;
                        return Ok(Some(
                            SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileFailureFeedDrainedObserved,
                        ));
                    }
                }
            }

            match graph_coordinator_context
                .live_runtime_plan_feed_mut_for_execution_graph_owner()?
                .admit_next_runtime_file_ready_work_for_execution_graph_owner(
                    &mut runtime_file_execution_session,
                )? {
                SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Admitted { admitted_count } => {
                    Ok(Some(
                        SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileReadyWorkAdmitted {
                            admitted_count,
                        },
                    ))
                }
                SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Pending => {
                    if graph_coordinator_context.runtime_file_failure_feed_drained {
                        Err(SsError::Cli(
                            json!({
                                "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                                "code": "ss_test_execution_graph_runtime_file_live_source_pending_after_feed_drain",
                                "reason": "runtime-file failure readiness cannot be pending after the graph coordinator has closed and drained file-failure feed admission",
                            })
                            .to_string(),
                        ))
                    } else {
                        Ok(None)
                    }
                }
                SsTestExecutionGraphRuntimeFileReadyWorkAdmission::Closed => {
                    if !graph_coordinator_context.runtime_file_failure_feed_drained {
                        return Ok(None);
                    }
                    graph_coordinator_context.runtime_file_live_source_closed = true;
                    Ok(Some(
                        SsTestExecutionGraphCoordinatorStepReceipt::RuntimeFileLiveSourceClosedObserved,
                    ))
                }
            }
        })();
        graph_coordinator_context.runtime_file_execution_session =
            Some(runtime_file_execution_session);
        receipt
    }

    pub(super) fn produce_graph_close_receipt_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        graph_coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
    ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
        let mut runtime_file_execution_session = graph_coordinator_context
            .runtime_file_execution_session
            .take()
            .ok_or_else(|| {
                SsError::Cli(
                    json!({
                        "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
                        "code": "ss_test_execution_graph_runtime_file_session_consumed",
                        "reason": "runtime-file graph session may be consumed exactly once by graph-owned closeout",
                    })
                    .to_string(),
                )
            })?;
        if !runtime_file_execution_session.is_terminal_runtime_file_work_for_execution_graph_owner()
        {
            let blocked_receipt =
                runtime_file_execution_session.runtime_file_close_blocked_receipt();
            graph_coordinator_context.runtime_file_execution_session =
                Some(runtime_file_execution_session);
            return Ok(blocked_receipt);
        }
        graph_coordinator_context
            .runtime_file_pool
            .shutdown_and_reap_for_execution_graph_owner_v1()?;
        let graph_settlements = runtime_file_execution_session
            .consume_ready_file_graph_settlements_for_graph_closeout_projection()?;
        let runtime_plan_closeout_ticket = self.graph_session.admit_runtime_plan_closeout()?;
        let live_runtime_plan_feed =
            graph_coordinator_context.consume_live_runtime_plan_feed_for_execution_graph_owner()?;
        match live_runtime_plan_feed.close_for_execution_graph_owner(session, graph_settlements) {
            Ok(terminal_summary) => {
                self.graph_session
                    .consume_runtime_file_execution_session(runtime_file_execution_session)?;
                let finished = self.graph_session.settle_runtime_plan_closeout_succeeded(
                    runtime_plan_closeout_ticket,
                    terminal_summary,
                )?;
                Ok(SsTestExecutionGraphCoordinatorStepReceipt::GraphClosed { finished })
            }
            Err(error) => {
                graph_coordinator_context.runtime_file_execution_session =
                    Some(runtime_file_execution_session);
                self.graph_session
                    .settle_runtime_plan_closeout_failed(runtime_plan_closeout_ticket)?;
                Err(error)
            }
        }
    }
}
