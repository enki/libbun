use crate::{SsError, SsExternalCapabilityProviderHost, SsResult};
use durable_direct_run_kernel::{
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    DirectRunSsTestBodyWorkMaterializationRootAuthority,
};
use serde_json::{Value, json};

#[path = "provider_settlement_lane.rs"]
mod provider_settlement_lane;

pub(super) use provider_settlement_lane::{
    SsTestProviderSettlementPool, SsTestReadyFileBodyDispatchOwner,
};

pub(super) struct SsTestArtifactExecutionState {
    _obsolete_body_registry: (),
}

pub(super) struct TestReadyFileBodyDispatchAuthority {
    _private: (),
}

pub(super) struct SsTestBodyWorkerLaunchAuthority {
    active_file: String,
    test_id: String,
    test_name: String,
}

pub(super) struct SsTestReadyFileBodyDispatchSettlement {
    body_work_root_authority: DirectRunSsTestBodyWorkMaterializationRootAuthority,
    profile_counters: Value,
}

impl SsTestArtifactExecutionState {
    pub(super) fn new() -> Self {
        Self {
            _obsolete_body_registry: (),
        }
    }

    pub(super) fn take_next_registered_body_authority(
        &mut self,
        path: &str,
    ) -> SsResult<TestReadyFileBodyDispatchAuthority> {
        Err(obsolete_body_registry_fault(
            path,
            "ss_test_body_authority_registry_obsolete_for_direct_run_ready_file",
            "registered ss-test bodies execute only inside the authenticated worker-local compile and runtime transaction; the parent runtime-plan body registry is no longer an authority source",
        ))
    }
}

impl TestReadyFileBodyDispatchAuthority {
    pub(super) fn materialization_profile_counters(&self, authority: &'static str) -> Value {
        json!({
            "schema": "swarm.ss.test.body_materialization_profile.v1",
            "authority": authority,
            "obsoleteBodyRegistry": true,
        })
    }

    pub(super) fn into_body_worker_launch_authority_for_ss_test_worker_owner_v1(
        self,
        active_file: &str,
        test: &crate::test_runner::artifact_session::SsCollectedTestCase,
    ) -> SsTestBodyWorkerLaunchAuthority {
        SsTestBodyWorkerLaunchAuthority {
            active_file: active_file.to_owned(),
            test_id: test.test_id().to_owned(),
            test_name: test.name().to_owned(),
        }
    }
}

impl SsTestBodyWorkerLaunchAuthority {
    pub(super) fn external_capability_provider_enabled(&self) -> bool {
        false
    }

    pub(super) fn libbun_working_directory_for_ready_file_body_dispatch_owner_v1(
        &self,
    ) -> std::path::PathBuf {
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    }

    pub(super) fn execute_for_ready_file_body_dispatch_owner_v1(
        self,
        _external_capability_provider: Option<&mut SsExternalCapabilityProviderHost>,
        _provider_checkout_initialized: Option<bool>,
    ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
        Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.test.body_authority_registry_fault.v1",
                "code": "ss_test_obsolete_body_worker_launch_authority_consumed",
                "reason": "old body worker launch authority is no longer executable; selected ss-test work must run through direct-run selected body launch authority",
                "activeFile": self.active_file,
                "testId": self.test_id,
                "testName": self.test_name,
            })
            .to_string(),
        ))
    }
}

impl SsTestReadyFileBodyDispatchSettlement {
    pub(super) fn admit_terminal_product_for_ss_test_execution_owner_v1(
        terminal_product: DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
        profile_counters: Value,
    ) -> Self {
        let body_work_root_authority = terminal_product
            .into_ss_test_body_work_materialization_root_authority_for_direct_run_ss_test_body_work_owner_v1();
        Self {
            body_work_root_authority,
            profile_counters,
        }
    }

    pub(super) fn profile_counters_for_ready_file_execution_owner_v1(&self) -> Value {
        self.profile_counters.clone()
    }

    pub(super) fn into_body_work_materialization_root_authority_for_ready_file_execution_owner_v1(
        self,
    ) -> DirectRunSsTestBodyWorkMaterializationRootAuthority {
        self.body_work_root_authority
    }
}

fn obsolete_body_registry_fault(path: &str, code: &'static str, reason: &'static str) -> SsError {
    SsError::Cli(
        json!({
            "schema": "swarm.ss.test.body_authority_registry_fault.v1",
            "code": code,
            "reason": reason,
            "sourcePath": path,
        })
        .to_string(),
    )
}
