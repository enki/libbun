pub(in crate::test_runner::artifact_session) struct SsTestSourceWorkSetRuntimePlanLiveFeedSession {
    runtime_plan_execution_session: SourceWorkSetRuntimePlanExecutionSession,
    runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
    source_work_set_receipt: SsTestSourceWorkSetReceipt,
    runtime_execution_domain_owner: SsRuntimeExecutionDomainOwner,
}

impl SsTestSourceWorkSetRuntimePlanLiveFeedSession {
    pub(in crate::test_runner::artifact_session) fn open(
        source_work_set_generation_id: &str,
        _package_graph_session_fingerprint: &str,
        _package_graph_manifest_fingerprint: Option<&str>,
        total_admission_file_count: usize,
        worker_limit: Option<NonZeroUsize>,
        runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
        source_work_set_receipt: SsTestSourceWorkSetReceipt,
        runtime_plan_background_liveness_deadline: Duration,
        spawned_worker_child_liveness_deadline: Duration,
    ) -> SsResult<Self> {
        let artifact_session_authority =
            SsTestArtifactSessionAuthority::from_source_work_set_receipt(
                source_work_set_generation_id.to_owned(),
                &source_work_set_receipt,
            );
        let runtime_plan_execution_session = SourceWorkSetRuntimePlanExecutionSession::new(
            artifact_session_authority,
            total_admission_file_count,
            source_work_set_generation_id.to_owned(),
            worker_limit,
            runtime_plan_background_liveness_deadline,
            spawned_worker_child_liveness_deadline,
        )?;
        Ok(Self {
            runtime_plan_execution_session,
            runtime_plan_owner_session,
            source_work_set_receipt,
            runtime_execution_domain_owner: SsRuntimeExecutionDomainOwner::new(),
        })
    }

    pub(in crate::test_runner::artifact_session) fn admit_live_file_product_emission(
        &mut self,
        file: SsTestFileFailureReceipt,
    ) -> SsResult<()> {
        self.runtime_plan_execution_session
            .admit_live_file_product_emission(file)
    }

    pub(in crate::test_runner::artifact_session) fn close_file_failure_feed_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<()> {
        self.runtime_plan_execution_session
            .close_file_failure_feed_for_execution_graph_owner(
                &mut self.runtime_plan_owner_session,
            )?;
        let _ = session;
        Ok(())
    }

    pub(in crate::test_runner::artifact_session) fn admit_next_runtime_file_ready_work_for_execution_graph_owner(
        &mut self,
        runtime_file_execution_session: &mut SsTestExecutionGraphRuntimeFileExecutionSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileReadyWorkAdmission> {
        self.runtime_plan_execution_session
            .admit_next_runtime_file_ready_work_for_execution_graph_owner(
                runtime_file_execution_session,
            )
    }

    pub(in crate::test_runner::artifact_session) fn admit_next_file_failure_to_live_source_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileFailureFeedAdmission> {
        let admission = self
            .runtime_plan_execution_session
            .admit_next_file_failure_to_live_source_for_execution_graph_owner(
                &mut self.runtime_plan_owner_session,
            )?;
        let _ = session;
        Ok(admission)
    }

    pub(in crate::test_runner::artifact_session) fn commit_admitted_pool_worker_settlement_for_execution_graph_owner(
        &mut self,
        admitted: runtime_execution_domain::SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        runtime_execution_domain::commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1(
            admitted,
        )
    }

    pub(in crate::test_runner::artifact_session) fn settle_pool_worker_loss_for_execution_graph_owner(
        &mut self,
        dispatched_source: SsPoolDispatchedSelectedSourceTestFile,
        worker_loss_fault: &serde_json::Value,
    ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
        runtime_execution_domain::settle_pool_worker_loss_for_execution_graph_owner_v1(
            dispatched_source,
            worker_loss_fault,
            &self.runtime_plan_owner_session,
        )
    }

    pub(in crate::test_runner::artifact_session) fn execute_runtime_file_worker_input_for_execution_graph_owner(
        &mut self,
        session: &mut SsTestRunnerSession,
        worker_input: SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    ) -> SsResult<SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement> {
        let worker_settlement = self
            .runtime_plan_execution_session
            .execute_runtime_file_worker_input_for_execution_graph_owner(
                &mut self.runtime_plan_owner_session,
                &mut self.runtime_execution_domain_owner,
                worker_input,
            );
        let _ = session;
        worker_settlement
    }

    pub(in crate::test_runner::artifact_session) fn close_for_execution_graph_owner(
        self,
        session: &mut SsTestRunnerSession,
        authored_file_order: Vec<String>,
        graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
    ) -> SsResult<Value> {
        let mut runtime_plan_owner_session = self.runtime_plan_owner_session;
        let result = self
            .runtime_plan_execution_session
            .close_runtime_plan_feed_for_execution_graph_owner(
                &mut runtime_plan_owner_session,
                self.source_work_set_receipt,
                authored_file_order,
                graph_settlements,
            );
        session
            .append_runtime_plan_owner_observations(runtime_plan_owner_session.into_observations());
        result
    }
}
