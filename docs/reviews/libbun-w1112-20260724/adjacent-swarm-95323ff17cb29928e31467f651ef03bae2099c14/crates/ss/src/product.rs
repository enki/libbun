#![forbid(unsafe_code)]

use serde_json::{Value, json};
use ss_runtime_provider_host_set_owner::SsRuntimeProviderHostSet;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use swarm_rust_sdk_static_provider_host::{
    RustSdkStaticProviderHostOwner, RustSdkStaticProviderHostSet,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SsError {
    #[error(transparent)]
    ProviderListing(#[from] ss_runtime_provider_listing_owner::SsProviderListingError),
    #[error(transparent)]
    SourceCommandOwner(#[from] ss_runtime_source_command_owner::SsSourceCommandError),
    #[error(transparent)]
    TestOwner(#[from] ss_runtime_test_owner::SsError),
    #[error(transparent)]
    TestPlanOwner(#[from] ss_runtime_test_plan_owner::SsTestPlanError),
    #[error(transparent)]
    CommandModel(#[from] ss_command_model::SsCommandModelError),
    #[error("{0}")]
    Cli(String),
}

pub type SsResult<T> = Result<T, SsError>;

pub(crate) fn run_cli_for_product_binary(
    args: impl IntoIterator<Item = String>,
) -> Result<SsCliExecutionOutcome, SsCliExecutionFailure> {
    let args = args.into_iter().collect::<Vec<_>>();
    let mut profile = SsCliExecutionProfile::new();
    let parse_started = Instant::now();
    let invocation =
        match ss_command_model::parse_ss_command_invocation(args, product_ss_artifact_cache_root())
        {
            Ok(invocation) => invocation,
            Err(error) => {
                return Err(SsCliExecutionFailure::without_source_profile(
                    error.into(),
                    profile,
                ));
            }
        };
    let command = ss_command_invocation_name(&invocation);
    let source_profile_out = source_profile_out_from_invocation(&invocation);
    profile.record_span(
        "cli_parse",
        parse_started.elapsed(),
        json!({
            "schema": "swarm.ss.cli_execution.profile.parse.v1",
            "command": command,
        }),
    );
    let run_started = Instant::now();
    let command_result = match invocation {
        ss_command_model::SsCommandInvocation::Providers {
            libbun_external_capability_provider_enabled,
        } => run_providers_operation(libbun_external_capability_provider_enabled),
        ss_command_model::SsCommandInvocation::Source {
            request,
            profile,
            profile_out,
        } => match builtin_runtime_provider_host_set_for_product_binary_owner_v1() {
            Ok(provider_host_set) => {
                run_source_command_request(request, provider_host_set, profile, profile_out)
            }
            Err(error) => Err(error),
        },
        ss_command_model::SsCommandInvocation::Test { invocation } => {
            match test_runtime_provider_environment_for_product_binary_owner_v1() {
                Ok(runtime_provider_environment) => {
                    run_test_invocation_request(invocation, runtime_provider_environment)
                }
                Err(error) => Err(error),
            }
        }
    };
    let mut value = match command_result {
        Ok(value) => value,
        Err(error) => {
            profile.record_span(
                "command_execution",
                run_started.elapsed(),
                json!({
                    "schema": "swarm.ss.cli_execution.profile.command_execution.v1",
                    "command": command,
                    "outcome": "error",
                }),
            );
            return Err(SsCliExecutionFailure::from_error(
                error,
                source_profile_out,
                profile,
            ));
        }
    };
    profile.record_span(
        "command_execution",
        run_started.elapsed(),
        json!({
            "schema": "swarm.ss.cli_execution.profile.command_execution.v1",
            "command": command,
        }),
    );
    let source_profile = source_profile_out
        .as_ref()
        .and_then(|_| take_source_profile_for_profile_out(&mut value));
    Ok(SsCliExecutionOutcome {
        value,
        source_profile_out,
        source_profile,
        cli_profile: profile,
    })
}

fn builtin_runtime_provider_host_set_for_product_binary_owner_v1()
-> SsResult<SsRuntimeProviderHostSet> {
    let working_directory = std::env::current_dir().map_err(|error| {
        product_external_provider_host_set_error(format!(
            "failed to read current working directory: {error}"
        ))
    })?;
    let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
        .map_err(product_provider_host_set_error)?;
    let admissions =
        ss_runtime_native_provider_composition_owner::admit_native_provider_admission_bundle_for_ss_product_environment_owner_v1(
            &owner,
        )
        .map_err(product_provider_host_set_error)?
        .into_builtin_static_provider_host_admission_set_for_product_environment_owner_v1(&owner)
        .map_err(product_provider_host_set_error)?;
    let static_manifest_provider_bridge = owner
        .admit_product_binary_static_manifest_provider_bridge_for_package_graph_owner_v1()
        .map_err(product_provider_host_set_error)?;
    let provider_host_set = runtime_provider_host_set_from_static_provider_host_admissions_and_manifest_bridge_for_product_binary_owner_v1(
        admissions,
        static_manifest_provider_bridge,
    )?;
    ss_runtime_external_capability_provider_owner::install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
        provider_host_set,
        &working_directory,
    )
    .map_err(product_external_provider_host_set_error)
}

fn test_runtime_provider_environment_for_product_binary_owner_v1()
-> SsResult<ss_runtime_test_owner::SsTestRuntimeProviderExecutionEnvironment> {
    let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
        .map_err(product_provider_host_set_error)?;
    let admissions =
        ss_runtime_native_provider_composition_owner::admit_native_provider_admission_bundle_for_ss_product_environment_owner_v1(
            &owner,
        )
        .map_err(product_provider_host_set_error)?
        .into_test_mode_static_provider_host_admission_set_for_product_environment_owner_v1(&owner)
        .map_err(product_provider_host_set_error)?;
    let provider_host_set =
        runtime_provider_host_set_from_static_provider_host_admissions_for_product_binary_owner_v1(
            admissions,
        )?;
    let builtin_static_manifest_provider_bridge = owner
        .admit_product_binary_test_static_manifest_provider_bridge_for_package_graph_owner_v1()
        .map_err(product_provider_host_set_error)?;
    let static_test_manifest_provider_bridge = owner
        .admit_test_mode_manifest_provider_bridge_for_package_graph_owner_v1()
        .map_err(product_provider_host_set_error)?;
    Ok(
        ss_runtime_test_owner::SsTestRuntimeProviderExecutionEnvironment::admit_from_product_binary_test_provider_authority_for_ss_test_execution_owner_v1(
            provider_host_set,
            builtin_static_manifest_provider_bridge,
            static_test_manifest_provider_bridge,
        ),
    )
}

fn runtime_provider_host_set_from_static_provider_host_admissions_for_product_binary_owner_v1(
    admissions: swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostAdmissionSet,
) -> SsResult<SsRuntimeProviderHostSet> {
    let rust_sdk =
        RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(admissions)
            .map_err(product_provider_host_set_error)?;
    SsRuntimeProviderHostSet::from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
        rust_sdk,
    )
    .map_err(product_provider_host_set_error)
}

fn runtime_provider_host_set_from_static_provider_host_admissions_and_manifest_bridge_for_product_binary_owner_v1(
    admissions: swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostAdmissionSet,
    static_manifest_provider_bridge: swarm_rust_sdk_static_provider_host::RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
) -> SsResult<SsRuntimeProviderHostSet> {
    let rust_sdk =
        RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(admissions)
            .map_err(product_provider_host_set_error)?;
    SsRuntimeProviderHostSet::from_rust_sdk_static_provider_host_set_and_manifest_bridge_for_ss_product_environment_owner_v1(
        rust_sdk,
        static_manifest_provider_bridge,
    )
    .map_err(product_provider_host_set_error)
}

fn product_provider_host_set_error(error: impl ToString) -> SsError {
    SsError::Cli(
        json!({
            "schema": "swarm.ss.product_binary.provider_host_set_admission_failed.v1",
            "reason": "ss product-binary startup must mint provider-host-set authority from admitted static-provider host-set evidence",
            "source": error.to_string(),
        })
        .to_string(),
    )
}

fn product_external_provider_host_set_error(error: impl ToString) -> SsError {
    SsError::Cli(
        json!({
            "schema": "swarm.ss.product_binary.external_provider_host_set_admission_failed.v1",
            "reason": "ss product-binary startup must install its external-transport provider host before direct source-command execution",
            "source": error.to_string(),
        })
        .to_string(),
    )
}

fn product_ss_artifact_cache_root() -> Option<PathBuf> {
    ss_runtime_source_path_owner::swarm_test_artifact_cache_root()
}

fn run_source_command_request(
    request: ss_command_model::SsSourceCommandRequest,
    provider_host_set: SsRuntimeProviderHostSet,
    profile: ss_command_model::SsSourceProfileMode,
    profile_out: Option<String>,
) -> SsResult<Value> {
    ss_runtime_source_command_owner::run_source_command_request_for_ss_product_owner_v1(
        request,
        provider_host_set,
        profile,
        profile_out,
    )
    .map_err(Into::into)
}

fn run_providers_operation(libbun_external_capability_provider_enabled: bool) -> SsResult<Value> {
    ss_runtime_provider_listing_owner::providers_with_libbun(
        libbun_external_capability_provider_enabled,
    )
    .map_err(Into::into)
}

fn run_test_invocation_request(
    invocation: ss_command_model::SsCliTestInvocation,
    runtime_provider_environment: ss_runtime_test_owner::SsTestRuntimeProviderExecutionEnvironment,
) -> SsResult<Value> {
    let current_dir = std::env::current_dir()
        .map_err(|error| SsError::Cli(format!("failed to read current directory: {error}")))?;
    if invocation.list_suites() {
        return ss_runtime_test_plan_owner::list_ss_test_suites_for_ss_cli_observation_v1(
            &current_dir,
        )
        .map_err(Into::into);
    }
    let run_plan =
        ss_runtime_test_plan_owner::admit_ss_test_run_plan_for_ss_runtime_test_discovery_owner_v1(
            &invocation,
            &current_dir,
        )?;
    ss_runtime_test_owner::run_admitted_cli_test_invocation_with_runtime_provider_environment_for_ss_test_execution_owner_v1(
        invocation,
        current_dir,
        run_plan,
        runtime_provider_environment,
    )
    .map_err(Into::into)
}

pub fn render_cli_result_for_stdout(value: &Value) -> String {
    let default_report = render_default_ss_test_result(value);
    if default_report.is_empty() {
        ss_cli_output::render_cli_result_for_stdout(value)
    } else {
        default_report
    }
}

fn render_default_ss_test_result(value: &Value) -> String {
    ss_runtime_test_owner::render_default_ss_test_result(value)
}

pub(crate) fn write_cli_result_to_stdout_with_receipt(
    value: &Value,
) -> SsResult<SsCliStdoutWriteReceipt> {
    if ss_cli_output::cli_result_was_streamed_to_terminal(value) {
        return Ok(SsCliStdoutWriteReceipt {
            streamed_to_terminal: true,
            bytes: 0,
            render_elapsed: Duration::ZERO,
            write_elapsed: Duration::ZERO,
        });
    }
    let render_started = Instant::now();
    let mut output = render_cli_result_for_stdout(value).into_bytes();
    let render_elapsed = render_started.elapsed();
    output.push(b'\n');
    let bytes = output.len();
    let write_started = Instant::now();
    std::io::Write::write_all(&mut std::io::stdout().lock(), &output)
        .map_err(|error| SsError::Cli(error.to_string()))?;
    Ok(SsCliStdoutWriteReceipt {
        streamed_to_terminal: false,
        bytes,
        render_elapsed,
        write_elapsed: write_started.elapsed(),
    })
}

pub fn cli_result_requires_failure_exit(value: &Value) -> bool {
    ss_cli_output::cli_result_requires_failure_exit(value)
}

pub fn should_print_cli_result(args: impl IntoIterator<Item = String>) -> bool {
    ss_cli_output::should_print_cli_result(args)
}

pub struct SsCliExecutionOutcome {
    value: Value,
    source_profile_out: Option<String>,
    source_profile: Option<Value>,
    cli_profile: SsCliExecutionProfile,
}

pub struct SsCliExecutionFailure {
    error: SsError,
    source_profile_out: Option<String>,
    source_profile: Option<Value>,
    cli_profile: SsCliExecutionProfile,
}

impl SsCliExecutionOutcome {
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn record_stdout_write(&mut self, receipt: SsCliStdoutWriteReceipt) {
        self.cli_profile.record_span(
            "stdout_render",
            receipt.render_elapsed,
            json!({
                "schema": "swarm.ss.cli_execution.profile.stdout_render.v1",
                "streamedToTerminal": receipt.streamed_to_terminal,
                "bytes": receipt.bytes,
            }),
        );
        self.cli_profile.record_span(
            "stdout_write",
            receipt.write_elapsed,
            json!({
                "schema": "swarm.ss.cli_execution.profile.stdout_write.v1",
                "streamedToTerminal": receipt.streamed_to_terminal,
                "bytes": receipt.bytes,
            }),
        );
    }

    pub fn record_stdout_skipped(&mut self, reason: &'static str) {
        self.cli_profile.record_span(
            "stdout_skipped",
            Duration::ZERO,
            json!({
                "schema": "swarm.ss.cli_execution.profile.stdout_skipped.v1",
                "reason": reason,
            }),
        );
    }

    pub fn write_profile_out_if_requested(&self) -> SsResult<()> {
        write_profile_out_if_requested(
            self.source_profile_out.as_deref(),
            self.source_profile.as_ref(),
            &self.cli_profile,
        )
    }
}

impl SsCliExecutionFailure {
    fn without_source_profile(error: SsError, cli_profile: SsCliExecutionProfile) -> Self {
        Self {
            error,
            source_profile_out: None,
            source_profile: None,
            cli_profile,
        }
    }

    fn from_error(
        error: SsError,
        source_profile_out: Option<String>,
        cli_profile: SsCliExecutionProfile,
    ) -> Self {
        let source_profile = source_profile_from_error_for_profile_out(&error);
        Self {
            error,
            source_profile_out,
            source_profile,
            cli_profile,
        }
    }

    pub fn render_error_for_stderr(&self) -> String {
        render_cli_failure_for_stderr(&self.error)
    }

    pub fn write_profile_out_if_requested(&self) -> SsResult<()> {
        write_profile_out_if_requested(
            self.source_profile_out.as_deref(),
            self.source_profile.as_ref(),
            &self.cli_profile,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SsCliStdoutWriteReceipt {
    streamed_to_terminal: bool,
    bytes: usize,
    render_elapsed: Duration,
    write_elapsed: Duration,
}

struct SsCliExecutionProfile {
    started: Instant,
    spans: Vec<SsCliExecutionProfileSpan>,
}

struct SsCliExecutionProfileSpan {
    phase: &'static str,
    elapsed: Duration,
    context: Value,
}

impl SsCliExecutionProfile {
    fn new() -> Self {
        Self {
            started: Instant::now(),
            spans: Vec::new(),
        }
    }

    fn record_span(&mut self, phase: &'static str, elapsed: Duration, context: Value) {
        self.spans.push(SsCliExecutionProfileSpan {
            phase,
            elapsed,
            context,
        });
    }

    fn to_value(&self) -> Value {
        json!({
            "schema": "swarm.ss.cli_execution.profile.v1",
            "elapsedMs": duration_millis_u64(self.started.elapsed()),
            "spanCount": self.spans.len(),
            "phaseBuckets": self.phase_buckets_value(),
            "spans": self.spans.iter().map(SsCliExecutionProfileSpan::to_value).collect::<Vec<_>>(),
        })
    }

    fn phase_buckets_value(&self) -> Vec<Value> {
        let mut buckets = std::collections::BTreeMap::<String, (u64, u64)>::new();
        for span in &self.spans {
            let bucket = buckets.entry(span.phase.to_owned()).or_insert((0, 0));
            bucket.0 = bucket.0.saturating_add(duration_millis_u64(span.elapsed));
            bucket.1 = bucket.1.saturating_add(1);
        }
        buckets
            .into_iter()
            .map(|(bucket, (elapsed_ms, span_count))| {
                json!({
                    "schema": "swarm.ss.cli_execution.profile.phase_bucket.v1",
                    "bucket": bucket,
                    "elapsedMs": elapsed_ms,
                    "spanCount": span_count,
                })
            })
            .collect()
    }
}

impl SsCliExecutionProfileSpan {
    fn to_value(&self) -> Value {
        json!({
            "schema": "swarm.ss.cli_execution.profile.span.v1",
            "phase": self.phase,
            "elapsedMs": duration_millis_u64(self.elapsed),
            "context": self.context,
        })
    }
}

fn ss_command_invocation_name(invocation: &ss_command_model::SsCommandInvocation) -> &'static str {
    match invocation {
        ss_command_model::SsCommandInvocation::Providers { .. } => "providers",
        ss_command_model::SsCommandInvocation::Source { request, .. } => request.command().as_str(),
        ss_command_model::SsCommandInvocation::Test { .. } => "test",
    }
}

fn source_profile_out_from_invocation(
    invocation: &ss_command_model::SsCommandInvocation,
) -> Option<String> {
    match invocation {
        ss_command_model::SsCommandInvocation::Source { profile_out, .. } => profile_out.clone(),
        _ => None,
    }
}

fn take_source_profile_for_profile_out(value: &mut Value) -> Option<Value> {
    value
        .as_object_mut()
        .and_then(|object| object.remove("profile"))
}

fn source_profile_from_error_for_profile_out(error: &SsError) -> Option<Value> {
    match error {
        SsError::SourceCommandOwner(source) => {
            source.source_profile_for_ss_product_owner_v1().cloned()
        }
        _ => None,
    }
}

fn write_profile_out_if_requested(
    source_profile_out: Option<&str>,
    source_profile: Option<&Value>,
    cli_profile: &SsCliExecutionProfile,
) -> SsResult<()> {
    let Some(profile_out) = source_profile_out else {
        return Ok(());
    };
    let Some(source_profile) = source_profile else {
        return Err(SsError::Cli(
            json!({
                "schema": "swarm.ss.source_command.profile_fault.v1",
                "code": "ss_source_command_profile_missing_for_profile_out",
                "reason": "ss source-command profile-out requires the source owner to return a profile projection to the CLI finalization owner",
                "path": profile_out,
            })
            .to_string(),
        ));
    };
    let mut profile_value = source_profile.clone();
    attach_cli_profile_to_source_profile(&mut profile_value, cli_profile.to_value());
    let profile_text = serde_json::to_string_pretty(&profile_value).map_err(|source| {
        SsError::Cli(
            json!({
                "schema": "swarm.ss.source_command.profile_fault.v1",
                "code": "ss_source_command_profile_encode_failed",
                "reason": "ss source-command profile output must serialize before it can be written to --profile-out",
                "source": source.to_string(),
            })
            .to_string(),
        )
    })?;
    fs::write(profile_out, profile_text).map_err(|source| {
        SsError::Cli(
            json!({
                "schema": "swarm.ss.source_command.profile_fault.v1",
                "code": "ss_source_command_profile_out_write_failed",
                "reason": "ss source-command profile output could not be written to --profile-out",
                "path": profile_out,
                "source": source.to_string(),
            })
            .to_string(),
        )
    })
}

fn attach_cli_profile_to_source_profile(source_profile: &mut Value, cli_profile: Value) {
    if let Some(object) = source_profile.as_object_mut() {
        object.insert("cliProfile".to_owned(), cli_profile);
        return;
    }
    let source_profile_value = std::mem::take(source_profile);
    *source_profile = json!({
        "schema": "swarm.ss.source_command.profile_with_cli_execution.v1",
        "sourceProfile": source_profile_value,
        "cliProfile": cli_profile,
    });
}

fn duration_millis_u64(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn render_cli_failure_for_stderr(error: &SsError) -> String {
    source_entrypoint_substrate_failure_projection(error)
        .map(|projection| projection.render())
        .unwrap_or_else(|| error.to_string())
}

struct SourceEntrypointSubstrateFailureStderrProjection {
    code: String,
    stage: String,
    file: Option<String>,
    mode: Option<String>,
    work: Option<String>,
    cause: String,
}

impl SourceEntrypointSubstrateFailureStderrProjection {
    fn render(self) -> String {
        let mut lines = vec![
            format!("ss substrate fault [{}]", self.code),
            format!("stage: {}", self.stage),
        ];
        if let Some(file) = self.file {
            lines.push(format!("file: {file}"));
        }
        if let Some(mode) = self.mode {
            lines.push(format!("mode: {mode}"));
        }
        if let Some(work) = self.work {
            lines.push(format!("work: {work}"));
        }
        lines.push(format!("cause: {}", self.cause));
        lines.join("\n")
    }
}

fn source_entrypoint_substrate_failure_projection(
    error: &SsError,
) -> Option<SourceEntrypointSubstrateFailureStderrProjection> {
    match error {
        SsError::SourceCommandOwner(error) => source_command_substrate_failure_projection(error),
        _ => None,
    }
}

fn source_command_substrate_failure_projection(
    error: &ss_runtime_source_command_owner::SsSourceCommandError,
) -> Option<SourceEntrypointSubstrateFailureStderrProjection> {
    match error {
        ss_runtime_source_command_owner::SsSourceCommandError::ProfiledSourceCommandFailed {
            error,
            ..
        } => source_command_substrate_failure_projection(error),
        ss_runtime_source_command_owner::SsSourceCommandError::Cli(message) => {
            substrate_failure_projection_from_text(&message.to_string())
        }
        ss_runtime_source_command_owner::SsSourceCommandError::RunFailed(message) => {
            substrate_failure_projection_from_text(&message.to_string())
        }
        ss_runtime_source_command_owner::SsSourceCommandError::SourceOwner(error) => {
            substrate_failure_projection_from_text(&error.to_string())
        }
        ss_runtime_source_command_owner::SsSourceCommandError::SourceEntrypointExecution(error) => {
            substrate_failure_projection_from_text(&error.to_string())
        }
        ss_runtime_source_command_owner::SsSourceCommandError::Libswarm(error) => {
            substrate_failure_projection_from_text(&error.to_string())
        }
        ss_runtime_source_command_owner::SsSourceCommandError::CapabilitySdk(_)
        | ss_runtime_source_command_owner::SsSourceCommandError::RunnerConfig(_)
        | ss_runtime_source_command_owner::SsSourceCommandError::PackageUniverse(_) => None,
    }
}

fn substrate_failure_projection_from_text(
    text: &str,
) -> Option<SourceEntrypointSubstrateFailureStderrProjection> {
    let value = SubstrateDiagnosticJson::parse(text)?;
    substrate_failure_projection_from_value(value.value())
}

fn substrate_failure_projection_from_value(
    value: &Value,
) -> Option<SourceEntrypointSubstrateFailureStderrProjection> {
    if !is_source_entrypoint_substrate_fault_value(value) {
        return None;
    }
    Some(SourceEntrypointSubstrateFailureStderrProjection {
        code: substrate_fault_code(value),
        stage: substrate_fault_stage(value),
        file: first_string_field_in_tree(value, &["sourcePath", "file"]),
        mode: first_string_field_in_tree(value, &["sourceExecutionMode", "mode"]),
        work: substrate_fault_work_identity(value),
        cause: substrate_fault_cause(value),
    })
}

struct SubstrateDiagnosticJson(Value);

impl SubstrateDiagnosticJson {
    fn parse(text: &str) -> Option<Self> {
        serde_json::from_str(text).ok().map(Self)
    }

    fn value(&self) -> &Value {
        &self.0
    }
}

impl Drop for SubstrateDiagnosticJson {
    fn drop(&mut self) {
        let mut pending = vec![std::mem::take(&mut self.0)];
        while let Some(value) = pending.pop() {
            match value {
                Value::Array(items) => pending.extend(items),
                Value::Object(object) => pending.extend(object.into_values()),
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }
    }
}

fn is_source_entrypoint_substrate_fault_value(value: &Value) -> bool {
    let schema = value.get("schema").and_then(Value::as_str);
    if schema.is_some_and(|schema| {
        schema.starts_with("swarm.ss.source_entrypoint.")
            || schema == "swarm.ss.source_command.stored_image_run_fault.v1"
            || schema == "swarm.durable_execution.projection.fault.v1"
    }) {
        return true;
    }
    value
        .get("kind")
        .and_then(Value::as_str)
        .is_some_and(|kind| {
            kind == "process_session_terminal_failed" || kind.starts_with("process_session_")
        })
}

fn substrate_fault_code(value: &Value) -> String {
    first_string_field(value, &["code", "kind", "diagnostic_code", "schema"])
        .unwrap_or_else(|| "source_entrypoint_substrate_fault".to_owned())
}

fn substrate_fault_stage(value: &Value) -> String {
    first_string_field(value, &["stage", "phase", "executionPath"])
        .or_else(|| {
            value
                .get("schema")
                .and_then(Value::as_str)
                .map(stage_from_schema)
        })
        .unwrap_or_else(|| "source_entrypoint_runtime".to_owned())
}

fn substrate_fault_work_identity(value: &Value) -> Option<String> {
    first_string_field(
        value,
        &["selectedWorkIdentity", "workIdentity", "operation"],
    )
    .or_else(|| {
        value
            .get("coldPlan")
            .and_then(|cold_plan| cold_plan.get("reason"))
            .and_then(Value::as_str)
            .map(|reason| format!("source_entrypoint_cold_plan:{reason}"))
    })
}

fn substrate_fault_cause(value: &Value) -> String {
    first_string_field(value, &["reason", "message", "source", "error"])
        .and_then(|cause| nested_json_cause(&cause).or(Some(cause)))
        .map(compact_stderr_text)
        .unwrap_or_else(|| "source-entrypoint substrate fault".to_owned())
}

fn nested_json_cause(text: &str) -> Option<String> {
    let value = SubstrateDiagnosticJson::parse(text)?;
    first_string_field(value.value(), &["reason", "message", "source", "error"])
}

fn first_string_field(value: &Value, fields: &[&str]) -> Option<String> {
    fields
        .iter()
        .find_map(|field| value.get(*field).and_then(Value::as_str).map(str::to_owned))
}

fn first_string_field_in_tree(value: &Value, fields: &[&str]) -> Option<String> {
    let mut pending = vec![value];
    while let Some(candidate) = pending.pop() {
        if let Some(found) = first_string_field(candidate, fields) {
            return Some(found);
        }
        match candidate {
            Value::Object(object) => pending.extend(object.values().rev()),
            Value::Array(items) => pending.extend(items.iter().rev()),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
    None
}

fn stage_from_schema(schema: &str) -> String {
    schema
        .strip_prefix("swarm.")
        .unwrap_or(schema)
        .strip_suffix(".v1")
        .unwrap_or(schema.strip_prefix("swarm.").unwrap_or(schema))
        .replace('.', "_")
}

fn compact_stderr_text(text: String) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    const MAX_CAUSE_LEN: usize = 600;
    if compact.chars().count() <= MAX_CAUSE_LEN {
        compact
    } else {
        format!(
            "{}...",
            compact.chars().take(MAX_CAUSE_LEN).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SsError, SubstrateDiagnosticJson, first_string_field_in_tree,
        render_cli_failure_for_stderr, substrate_failure_projection_from_value,
    };
    use serde_json::{Map, Value};
    use std::process::Command;

    const DIAGNOSTIC_JSON_STACK_PROBE_ENV: &str = "SWARM_SS_DIAGNOSTIC_JSON_STACK_PROBE";
    const HOSTILE_DIAGNOSTIC_JSON_DEPTH: usize = 20_000;
    const TINY_DIAGNOSTIC_JSON_STACK_BYTES: usize = 128 * 1024;

    fn nested_diagnostic_json(mut value: Value) -> Value {
        for depth in 0..HOSTILE_DIAGNOSTIC_JSON_DEPTH {
            value = if depth % 2 == 0 {
                Value::Array(vec![value])
            } else {
                let mut object = Map::new();
                object.insert("next".to_owned(), value);
                Value::Object(object)
            };
        }
        value
    }

    fn run_diagnostic_json_stack_probe(test_name: &str, probe_name: &str, action: fn()) {
        if std::env::var(DIAGNOSTIC_JSON_STACK_PROBE_ENV).as_deref() == Ok(probe_name) {
            std::thread::Builder::new()
                .name(format!("ss-diagnostic-json-{probe_name}-128k"))
                .stack_size(TINY_DIAGNOSTIC_JSON_STACK_BYTES)
                .spawn(action)
                .expect("the diagnostic JSON tiny-stack worker must spawn")
                .join()
                .expect("the diagnostic JSON tiny-stack worker must terminate normally");
            return;
        }

        let output = Command::new(std::env::current_exe().expect("test executable must resolve"))
            .args(["--exact", test_name, "--nocapture"])
            .env(DIAGNOSTIC_JSON_STACK_PROBE_ENV, probe_name)
            .output()
            .expect("the process-isolated diagnostic JSON probe must spawn");
        assert!(
            output.status.success(),
            "diagnostic JSON probe `{probe_name}` aborted or failed (status {}):\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    fn run_valid_diagnostic_json_probe() {
        let mut leaf = Map::new();
        leaf.insert(
            "file".to_owned(),
            Value::String("field-order-loser.ss".to_owned()),
        );
        leaf.insert("sourcePath".to_owned(), Value::String("deep.ss".to_owned()));
        leaf.insert(
            "mode".to_owned(),
            Value::String("mode-order-loser".to_owned()),
        );
        leaf.insert(
            "sourceExecutionMode".to_owned(),
            Value::String("ss_run".to_owned()),
        );

        let mut root = Map::new();
        root.insert(
            "schema".to_owned(),
            Value::String("swarm.ss.source_entrypoint.execution_owner_fault.v1".to_owned()),
        );
        root.insert(
            "code".to_owned(),
            Value::String("deep_diagnostic_fault".to_owned()),
        );
        root.insert(
            "reason".to_owned(),
            Value::String("  compact\n diagnostic   cause  ".to_owned()),
        );
        root.insert(
            "diagnostic".to_owned(),
            nested_diagnostic_json(Value::Object(leaf)),
        );
        let diagnostic = SubstrateDiagnosticJson(Value::Object(root));

        let projection = substrate_failure_projection_from_value(diagnostic.value())
            .expect("the valid substrate diagnostic must project");
        assert_eq!(
            projection.render(),
            "ss substrate fault [deep_diagnostic_fault]\nstage: ss_source_entrypoint_execution_owner_fault\nfile: deep.ss\nmode: ss_run\ncause: compact diagnostic cause"
        );
        drop(diagnostic);
    }

    fn run_refused_diagnostic_json_probe() {
        let mut root = Map::new();
        root.insert(
            "schema".to_owned(),
            Value::String("not.a.substrate.fault".to_owned()),
        );
        root.insert("diagnostic".to_owned(), nested_diagnostic_json(Value::Null));
        let diagnostic = SubstrateDiagnosticJson(Value::Object(root));

        assert!(first_string_field_in_tree(diagnostic.value(), &["sourcePath", "file"]).is_none());
        assert!(substrate_failure_projection_from_value(diagnostic.value()).is_none());
        drop(diagnostic);
    }

    #[test]
    fn diagnostic_json_search_preserves_field_child_and_array_order() {
        let value = serde_json::json!({
            "first": {
                "file": "field-order-loser.ss",
                "sourcePath": "object-first.ss"
            },
            "second": [
                { "sourcePath": "array-later.ss" }
            ]
        });
        assert_eq!(
            first_string_field_in_tree(&value, &["sourcePath", "file"]).as_deref(),
            Some("object-first.ss")
        );

        let array = serde_json::json!([
            { "sourcePath": "array-first.ss" },
            { "sourcePath": "array-second.ss" }
        ]);
        assert_eq!(
            first_string_field_in_tree(&array, &["sourcePath"]).as_deref(),
            Some("array-first.ss")
        );
    }

    #[test]
    fn valid_diagnostic_json_search_render_and_teardown_are_tiny_stack_safe_at_20000_depth() {
        run_diagnostic_json_stack_probe(
            "product::tests::valid_diagnostic_json_search_render_and_teardown_are_tiny_stack_safe_at_20000_depth",
            "valid-20000",
            run_valid_diagnostic_json_probe,
        );
    }

    #[test]
    fn refused_diagnostic_json_search_and_teardown_are_tiny_stack_safe_at_20000_depth() {
        run_diagnostic_json_stack_probe(
            "product::tests::refused_diagnostic_json_search_and_teardown_are_tiny_stack_safe_at_20000_depth",
            "refused-20000",
            run_refused_diagnostic_json_probe,
        );
    }

    #[test]
    fn renders_source_entrypoint_substrate_json_as_compact_stderr() {
        let error = SsError::SourceCommandOwner(
            ss_runtime_source_command_owner::SsSourceCommandError::Cli(
                serde_json::json!({
                    "schema": "swarm.ss.source_entrypoint.execution_owner_fault.v1",
                    "code": "source_entrypoint_execution_cache_root_unavailable",
                    "reason": "source-entrypoint execution owner cannot return a cold publish plan without a real admitted artifact cache root",
                    "sourcePath": "sample.ss",
                    "sourceExecutionMode": "ss_run",
                    "coldPlan": {
                        "reason": "miss"
                    }
                })
                .to_string(),
            ),
        );

        let rendered = render_cli_failure_for_stderr(&error);

        assert!(
            rendered.contains(
                "ss substrate fault [source_entrypoint_execution_cache_root_unavailable]"
            )
        );
        assert!(rendered.contains("stage: ss_source_entrypoint_execution_owner_fault"));
        assert!(rendered.contains("file: sample.ss"));
        assert!(rendered.contains("mode: ss_run"));
        assert!(rendered.contains("work: source_entrypoint_cold_plan:miss"));
        assert!(rendered.contains("cause: source-entrypoint execution owner cannot return"));
    }

    #[test]
    fn renders_process_session_runtime_failure_without_raw_json() {
        let error = SsError::SourceCommandOwner(
            ss_runtime_source_command_owner::SsSourceCommandError::Cli(
                serde_json::json!({
                    "kind": "process_session_terminal_failed",
                    "code": "process_session_turn_limit_exceeded",
                    "message": "runtime turn limit reached"
                })
                .to_string(),
            ),
        );

        let rendered = render_cli_failure_for_stderr(&error);

        assert!(rendered.contains("ss substrate fault [process_session_turn_limit_exceeded]"));
        assert!(rendered.contains("stage: source_entrypoint_runtime"));
        assert!(rendered.contains("cause: runtime turn limit reached"));
        assert!(!rendered.contains("\"kind\""));
    }

    #[test]
    fn leaves_non_substrate_cli_errors_unchanged() {
        let error = SsError::Cli("usage: swarm <file>".to_owned());

        assert_eq!(render_cli_failure_for_stderr(&error), "usage: swarm <file>");
    }
}
