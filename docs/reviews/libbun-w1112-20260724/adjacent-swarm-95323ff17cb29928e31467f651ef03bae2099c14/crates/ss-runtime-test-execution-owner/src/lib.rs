#![forbid(unsafe_code)]
#![allow(dead_code)]

pub(crate) use ss_runtime_external_capability_provider_owner::SsExternalCapabilityProviderHost;
use ss_runtime_external_capability_provider_owner::SsRuntimeProviderHostSet;
pub(crate) use ss_runtime_source_compiler_owner as source_work_set;
pub(crate) use ss_runtime_source_runner_config_model::SsRunnerConfig;
pub(crate) use ss_runtime_test_runtime_plan_owner::SsRunner;
pub use ss_runtime_test_runtime_plan_owner::{SsError, SsResult};
use swarm_rust_sdk_static_provider_host::{
    RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
    RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner,
};

mod test_runner;

pub struct SsTestRuntimeProviderExecutionEnvironment {
    provider_host_set: SsRuntimeProviderHostSet,
    builtin_static_manifest_provider_bridge:
        RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
    static_test_manifest_provider_bridge:
        RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner,
}

impl SsTestRuntimeProviderExecutionEnvironment {
    pub fn admit_from_product_binary_test_provider_authority_for_ss_test_execution_owner_v1(
        provider_host_set: SsRuntimeProviderHostSet,
        builtin_static_manifest_provider_bridge:
            RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
        static_test_manifest_provider_bridge:
            RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner,
    ) -> Self {
        Self {
            provider_host_set,
            builtin_static_manifest_provider_bridge,
            static_test_manifest_provider_bridge,
        }
    }

    pub(crate) fn into_parts_for_ss_test_execution_owner_v1(
        self,
    ) -> (
        SsRuntimeProviderHostSet,
        RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
        RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner,
    ) {
        (
            self.provider_host_set,
            self.builtin_static_manifest_provider_bridge,
            self.static_test_manifest_provider_bridge,
        )
    }

    pub(crate) fn install_libbun_external_provider_for_ss_test_execution_owner_v1(
        self,
        working_directory: &std::path::Path,
    ) -> SsResult<Self> {
        let Self {
            provider_host_set,
            builtin_static_manifest_provider_bridge,
            static_test_manifest_provider_bridge,
        } = self;
        let provider_host_set = ss_runtime_external_capability_provider_owner::install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
            provider_host_set,
            working_directory,
        )
        .map_err(ss_test_child_provider_host_set_error)?;
        Ok(Self {
            provider_host_set,
            builtin_static_manifest_provider_bridge,
            static_test_manifest_provider_bridge,
        })
    }
}

fn ss_test_child_provider_host_set_error(error: impl ToString) -> SsError {
    SsError::Cli(
        serde_json::json!({
            "schema": "swarm.ss.test.spawned_worker_runtime_environment_fault.v1",
            "code": "ss_test_spawned_worker_provider_host_set_admission_failed",
            "reason": "ss-test spawned-worker body dispatch must mint a fresh test-mode provider-host-set from product-binary static-provider authority",
            "source": error.to_string(),
        })
        .to_string(),
    )
}

pub fn ss_test_runner_config_for_source_path(
    source_path: &std::path::Path,
    extra_package_resolution_roots: &[std::path::PathBuf],
) -> SsResult<SsRunnerConfig> {
    ss_runtime_source_test_runner_config_owner::ss_test_runner_config_for_source_path(
        source_path,
        extra_package_resolution_roots,
    )
    .map_err(|error| SsError::Cli(error.to_string()))
}

/// ss-test pool worker child session: serve Run/Shutdown frames over the
/// process's original stdin/stdout, executing one test file per Run frame
/// with process-level stdio capture. The lawful worker leaf of the Stage B
/// pool; never a parent fallback lane.
pub fn run_ss_test_pool_worker_child_session_for_ss_test_execution_owner_v1() -> SsResult<()> {
    test_runner::run_pool_worker_child_session_for_pool_worker_child_owner_v1()
}

/// Pool harness observation surface: encode a Run frame for a pool worker
/// child. Grants no authority beyond invoking `ss test <path>` directly.
pub fn encode_ss_test_pool_worker_run_frame_for_pool_harness_observation_v1(
    path: &str,
    extra_package_resolution_roots: &[std::path::PathBuf],
    test_name_pattern: Option<&str>,
) -> SsResult<Vec<u8>> {
    test_runner::encode_run_frame_for_pool_harness_observation_v1(
        path,
        extra_package_resolution_roots,
        test_name_pattern,
    )
}

/// Pool harness observation surface: encode a Shutdown frame.
pub fn encode_ss_test_pool_worker_shutdown_frame_for_pool_harness_observation_v1()
-> SsResult<Vec<u8>> {
    test_runner::encode_shutdown_frame_for_pool_harness_observation_v1()
}

/// Pool harness observation surface: read and project one child frame as an
/// observation value ({kind: ready|settlement|file_fault, ...}).
pub fn read_ss_test_pool_worker_child_frame_for_pool_harness_observation_v1(
    reader: &mut dyn std::io::Read,
) -> SsResult<serde_json::Value> {
    test_runner::read_child_frame_for_pool_harness_observation_v1(reader)
}

pub fn run_admitted_cli_test_invocation_with_runtime_provider_environment_for_ss_test_execution_owner_v1(
    invocation: ss_command_model::SsCliTestInvocation,
    current_dir: std::path::PathBuf,
    run_plan: ss_runtime_test_plan_owner::SsTestRunPlan,
    runtime_provider_environment: SsTestRuntimeProviderExecutionEnvironment,
) -> SsResult<serde_json::Value> {
    test_runner::run_admitted_cli_test_invocation_with_runtime_provider_environment_for_ss_test_execution_owner_v1(
        invocation,
        current_dir,
        run_plan,
        runtime_provider_environment,
    )
}
