//! Hostable Bun embedding facade.
//!
//! This crate owns the stable Rust boundary for hosting JavaScript and
//! TypeScript providers through Bun. It deliberately does not call Bun CLI
//! entrypoints and does not expose raw JSC handles across its public API.

use std::collections::BTreeMap;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

#[cfg(feature = "dynamic-loading")]
pub mod dynamic;
pub mod helper_protocol;
pub mod plugin_checksums;
pub mod release;

pub type LibbunResult<T> = Result<T, LibbunError>;

pub mod plugin_abi {
    pub const LIBBUN_PLUGIN_ABI_VERSION: u32 = 2;

    pub const LIBBUN_PLUGIN_STATUS_OK: u32 = 0;
    pub const LIBBUN_PLUGIN_STATUS_ERROR: u32 = 1;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct LibbunPluginBuffer {
        pub data: *mut u8,
        pub len: usize,
    }

    impl LibbunPluginBuffer {
        pub const fn empty() -> Self {
            Self {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct LibbunPluginStatus {
        pub code: u32,
        pub payload: LibbunPluginBuffer,
    }

    impl LibbunPluginStatus {
        pub const fn ok(payload: LibbunPluginBuffer) -> Self {
            Self {
                code: LIBBUN_PLUGIN_STATUS_OK,
                payload,
            }
        }

        pub const fn error(payload: LibbunPluginBuffer) -> Self {
            Self {
                code: LIBBUN_PLUGIN_STATUS_ERROR,
                payload,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BunRuntimeConfig {
    pub host_id: String,
    pub bun_revision: String,
    pub working_directory: PathBuf,
    pub environment: BTreeMap<String, String>,
    pub stdout: SinkPolicy,
    pub stderr: SinkPolicy,
    pub log: SinkPolicy,
}

impl BunRuntimeConfig {
    pub fn new(host_id: impl Into<String>, working_directory: impl Into<PathBuf>) -> Self {
        Self {
            host_id: host_id.into(),
            bun_revision: env!("LIBBUN_BUN_SOURCE_COMMIT").to_string(),
            working_directory: working_directory.into(),
            environment: BTreeMap::new(),
            stdout: SinkPolicy::Capture,
            stderr: SinkPolicy::Capture,
            log: SinkPolicy::Capture,
        }
    }

    pub fn with_environment_overlay(
        mut self,
        environment: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.environment = environment
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SinkPolicy {
    Capture,
    Drop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BunArtifactFingerprint {
    pub bun_revision: String,
    pub libbun_abi_version: u32,
}

pub const LIBBUN_ABI_VERSION: u32 = 1;

pub fn artifact_fingerprint() -> BunArtifactFingerprint {
    BunArtifactFingerprint {
        bun_revision: env!("LIBBUN_BUN_SOURCE_COMMIT").to_string(),
        libbun_abi_version: LIBBUN_ABI_VERSION,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BunModuleSpec {
    Source { module_id: String, source: String },
    Path { path: PathBuf },
    PreparedBundle { bundle_id: String, bytes: Vec<u8> },
}

pub const LIBBUN_PREPARED_BUNDLE_FORMAT: &str = "libbun.preparedBundle";
pub const LIBBUN_PREPARED_BUNDLE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedBundleV1 {
    pub format: String,
    pub format_version: u32,
    pub bundle_id: String,
    pub bun_revision: String,
    pub libbun_abi_version: u32,
    pub entry_module: String,
    pub modules: BTreeMap<String, PreparedBundleModuleV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedBundleModuleV1 {
    pub source: String,
}

impl PreparedBundleV1 {
    pub fn source_bundle(
        bundle_id: impl Into<String>,
        entry_module: impl Into<String>,
        modules: BTreeMap<String, PreparedBundleModuleV1>,
    ) -> LibbunResult<Self> {
        let bundle = Self {
            format: LIBBUN_PREPARED_BUNDLE_FORMAT.to_string(),
            format_version: LIBBUN_PREPARED_BUNDLE_FORMAT_VERSION,
            bundle_id: bundle_id.into(),
            bun_revision: env!("LIBBUN_BUN_SOURCE_COMMIT").to_string(),
            libbun_abi_version: LIBBUN_ABI_VERSION,
            entry_module: entry_module.into(),
            modules,
        };
        bundle.validate()?;
        Ok(bundle)
    }

    pub fn from_bytes(bytes: &[u8]) -> LibbunResult<Self> {
        let bundle: Self = serde_json::from_slice(bytes).map_err(|err| {
            LibbunError::module_load(format!("prepared bundle JSON decode failed: {err}"))
        })?;
        bundle.validate()?;
        Ok(bundle)
    }

    pub fn to_bytes(&self) -> LibbunResult<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec(self).map_err(|err| {
            LibbunError::module_load(format!("prepared bundle JSON encode failed: {err}"))
        })
    }

    pub fn fingerprint(&self) -> LibbunResult<String> {
        let bytes = self.to_bytes()?;
        let digest = sha2::Sha256::digest(bytes);
        Ok(format!("sha256:{}", hex_lower(&digest)))
    }

    pub fn validate_for_current_runtime(&self, expected_bundle_id: &str) -> LibbunResult<()> {
        self.validate()?;
        if self.bundle_id != expected_bundle_id {
            return Err(LibbunError::module_load(format!(
                "prepared bundle id mismatch: spec requested `{expected_bundle_id}`, artifact contains `{}`",
                self.bundle_id
            )));
        }
        if self.bun_revision != env!("LIBBUN_BUN_SOURCE_COMMIT") {
            return Err(LibbunError::module_load(format!(
                "prepared bundle Bun revision `{}` is incompatible with runtime `{}`",
                self.bun_revision,
                env!("LIBBUN_BUN_SOURCE_COMMIT")
            )));
        }
        if self.libbun_abi_version != LIBBUN_ABI_VERSION {
            return Err(LibbunError::module_load(format!(
                "prepared bundle libbun ABI `{}` is incompatible with runtime `{}`",
                self.libbun_abi_version, LIBBUN_ABI_VERSION
            )));
        }
        Ok(())
    }

    pub fn validate(&self) -> LibbunResult<()> {
        if self.format != LIBBUN_PREPARED_BUNDLE_FORMAT {
            return Err(LibbunError::module_load(format!(
                "unsupported prepared bundle format `{}`",
                self.format
            )));
        }
        if self.format_version != LIBBUN_PREPARED_BUNDLE_FORMAT_VERSION {
            return Err(LibbunError::module_load(format!(
                "unsupported prepared bundle format version `{}`",
                self.format_version
            )));
        }
        if self.bundle_id.trim().is_empty() {
            return Err(LibbunError::module_load("prepared bundle id is empty"));
        }
        if self.bun_revision.trim().is_empty() {
            return Err(LibbunError::module_load(
                "prepared bundle Bun revision is empty",
            ));
        }
        validate_bundle_module_path(&self.entry_module)?;
        if !self.modules.contains_key(&self.entry_module) {
            return Err(LibbunError::module_load(format!(
                "prepared bundle entry module `{}` is missing",
                self.entry_module
            )));
        }
        if self.modules.is_empty() {
            return Err(LibbunError::module_load("prepared bundle has no modules"));
        }
        for (module_path, module) in &self.modules {
            validate_bundle_module_path(module_path)?;
            if module.source.is_empty() {
                return Err(LibbunError::module_load(format!(
                    "prepared bundle module `{module_path}` is empty"
                )));
            }
        }
        Ok(())
    }
}

impl PreparedBundleModuleV1 {
    pub fn source(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

fn validate_bundle_module_path(path: &str) -> LibbunResult<()> {
    if path.trim().is_empty() {
        return Err(LibbunError::module_load(
            "prepared bundle module path is empty",
        ));
    }
    if path.contains('\\') {
        return Err(LibbunError::module_load(format!(
            "prepared bundle module path `{path}` must use forward slashes"
        )));
    }
    if path
        .split('/')
        .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(LibbunError::module_load(format!(
            "prepared bundle module path `{path}` must not contain empty, current, or parent segments"
        )));
    }
    let path = Path::new(path);
    if path.is_absolute() {
        return Err(LibbunError::module_load(format!(
            "prepared bundle module path `{}` must be relative",
            path.display()
        )));
    }

    let mut normal_components = 0;
    for component in path.components() {
        match component {
            Component::Normal(_) => normal_components += 1,
            _ => {
                return Err(LibbunError::module_load(format!(
                    "prepared bundle module path `{}` must not contain parent, root, or prefix components",
                    path.display()
                )));
            }
        }
    }
    if normal_components == 0 {
        return Err(LibbunError::module_load(format!(
            "prepared bundle module path `{}` has no file component",
            path.display()
        )));
    }
    Ok(())
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BunModuleHandle {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BunAsyncHandle {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StructuralValue(pub serde_json::Value);

impl StructuralValue {
    pub fn null() -> Self {
        Self(serde_json::Value::Null)
    }
}

impl From<serde_json::Value> for StructuralValue {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderContractIdentity {
    pub package: String,
    pub capability: String,
    pub contract_fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProviderDomainClass {
    JavaScriptExternalTransport,
    ApplicationIo,
    RustSubstrateAuthority,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRequest {
    pub contract: ProviderContractIdentity,
    pub domain: ProviderDomainClass,
    pub module: BunModuleSpec,
    pub export: String,
    pub input: StructuralValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDeadline {
    pub deadline_ms: u64,
}

impl ProviderDeadline {
    pub fn from_millis(deadline_ms: u64) -> Self {
        Self { deadline_ms }
    }

    pub fn after(duration: Duration) -> Self {
        Self {
            deadline_ms: duration.as_millis().try_into().unwrap_or(u64::MAX),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettleOptions {
    pub deadline: ProviderDeadline,
}

impl ProviderSettleOptions {
    pub fn new(deadline: ProviderDeadline) -> Self {
        Self { deadline }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettledProviderReceipt {
    Ready {
        contract: ProviderContractIdentity,
        artifact: BunArtifactFingerprint,
        result: ProviderCallResult,
        output: Vec<OutputRecord>,
        settlement: ProviderSettlementDiagnostics,
    },
    Failed(ProviderExecutionFailure),
}

impl SettledProviderReceipt {
    pub fn output(&self) -> &[OutputRecord] {
        match self {
            Self::Ready { output, .. } => output,
            Self::Failed(failure) => &failure.output,
        }
    }

    fn with_trace(mut self, trace: Vec<ProviderSettlementTraceEvent>) -> Self {
        match &mut self {
            Self::Ready { settlement, .. } => {
                settlement.trace = trace;
            }
            Self::Failed(failure) => {
                failure.trace = trace;
            }
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettlementDiagnostics {
    pub operation: ProviderExecutionOperation,
    pub module_specifier_or_url: String,
    pub export_name: String,
    pub provider_domain_class: ProviderDomainClass,
    pub elapsed_ms: u64,
    pub deadline_ms: u64,
    pub pending_async_task_count: usize,
    pub output_record_count: usize,
    pub trace: Vec<ProviderSettlementTraceEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettlementTraceEvent {
    pub phase: ProviderSettlementPhase,
    pub elapsed_ms: u64,
    pub pending_async_task_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSettlementPhase {
    Start,
    ModuleLoad,
    CallExport,
    ResolveAsync,
    PumpEventLoop,
    DeadlineElapsed,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderExecutionFailure {
    pub operation: ProviderExecutionOperation,
    pub module_specifier_or_url: String,
    pub export_name: String,
    pub provider_domain_class: ProviderDomainClass,
    pub js_error_name: Option<String>,
    pub js_error_message: Option<String>,
    pub js_error_stack: Option<String>,
    pub detail_extraction_failed: bool,
    pub pending_async_task_count: usize,
    pub elapsed_ms: u64,
    pub deadline_ms: u64,
    pub output_record_count: usize,
    pub output: Vec<OutputRecord>,
    pub trace: Vec<ProviderSettlementTraceEvent>,
}

impl ProviderExecutionFailure {
    fn new(
        operation: ProviderExecutionOperation,
        request: &ProviderRequest,
        options: ProviderSettleOptions,
        started_at: Instant,
        pending_async_task_count: usize,
        output: Vec<OutputRecord>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let (js_error_name, js_error_stack) = js_error_fields(&message);
        Self {
            operation,
            module_specifier_or_url: module_specifier_or_url(&request.module),
            export_name: request.export.clone(),
            provider_domain_class: request.domain,
            js_error_name,
            js_error_message: Some(message),
            js_error_stack,
            detail_extraction_failed: false,
            pending_async_task_count,
            elapsed_ms: elapsed_ms(started_at),
            deadline_ms: options.deadline.deadline_ms,
            output_record_count: output.len(),
            output,
            trace: Vec::new(),
        }
    }

    fn with_trace(mut self, trace: Vec<ProviderSettlementTraceEvent>) -> Self {
        self.trace = trace;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderExecutionOperation {
    RuntimeInitialize,
    AdapterModuleLoad,
    ProviderModuleImport,
    ProviderExportLookup,
    ProviderFactoryValidate,
    ProviderFactoryInvoke,
    ProviderCallableValidate,
    ProviderCallableInvoke,
    ProviderPromiseSettle,
    ProviderDeadlineElapsed,
}

impl ProviderSettlementDiagnostics {
    fn from_request(
        operation: ProviderExecutionOperation,
        request: &ProviderRequest,
        options: ProviderSettleOptions,
        started_at: Instant,
        pending_async_task_count: usize,
        output_record_count: usize,
    ) -> Self {
        Self {
            operation,
            module_specifier_or_url: module_specifier_or_url(&request.module),
            export_name: request.export.clone(),
            provider_domain_class: request.domain,
            elapsed_ms: elapsed_ms(started_at),
            deadline_ms: options.deadline.deadline_ms,
            pending_async_task_count,
            output_record_count,
            trace: Vec::new(),
        }
    }

    fn with_trace(mut self, trace: Vec<ProviderSettlementTraceEvent>) -> Self {
        self.trace = trace;
        self
    }
}

fn push_settlement_trace(
    trace: &mut Vec<ProviderSettlementTraceEvent>,
    phase: ProviderSettlementPhase,
    started_at: Instant,
    pending_async_task_count: usize,
) {
    const MAX_TRACE_EVENTS: usize = 64;
    if trace.len() == MAX_TRACE_EVENTS {
        trace.remove(0);
    }
    trace.push(ProviderSettlementTraceEvent {
        phase,
        elapsed_ms: elapsed_ms(started_at),
        pending_async_task_count,
    });
}

fn module_specifier_or_url(module: &BunModuleSpec) -> String {
    match module {
        BunModuleSpec::Source { module_id, .. } => format!("source:{module_id}"),
        BunModuleSpec::Path { path } => path.display().to_string(),
        BunModuleSpec::PreparedBundle { bundle_id, .. } => format!("prepared-bundle:{bundle_id}"),
    }
}

fn elapsed_ms(started_at: Instant) -> u64 {
    started_at
        .elapsed()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn deadline_elapsed(started_at: Instant, deadline: ProviderDeadline) -> bool {
    started_at.elapsed() >= Duration::from_millis(deadline.deadline_ms)
}

fn export_failure_operation(error: &LibbunError) -> ProviderExecutionOperation {
    let message = error.to_string();
    if message.contains("missing export") {
        ProviderExecutionOperation::ProviderExportLookup
    } else if message.contains("not callable") {
        ProviderExecutionOperation::ProviderCallableValidate
    } else {
        ProviderExecutionOperation::ProviderCallableInvoke
    }
}

fn js_error_fields(message: &str) -> (Option<String>, Option<String>) {
    let name = message
        .split_once(':')
        .map(|(name, _)| name.trim())
        .filter(|name| !name.is_empty() && !name.contains(' ') && !name.contains('\n'))
        .map(str::to_string);
    let stack = if message.contains("\n    at ") || message.contains("\nat ") {
        Some(message.to_string())
    } else {
        None
    };
    (name, stack)
}

fn settled_ready_or_failure(
    request: ProviderRequest,
    options: ProviderSettleOptions,
    started_at: Instant,
    pending_async_task_count: usize,
    output: Vec<OutputRecord>,
    result: ProviderCallResult,
    operation: ProviderExecutionOperation,
) -> LibbunResult<SettledProviderReceipt> {
    if let ProviderCallResult::Err(error) = &result {
        if error.code == "provider_rejected" {
            return Ok(SettledProviderReceipt::Failed(
                ProviderExecutionFailure::new(
                    operation,
                    &request,
                    options,
                    started_at,
                    pending_async_task_count,
                    output,
                    error.message.clone(),
                ),
            ));
        }
    }

    Ok(SettledProviderReceipt::Ready {
        contract: request.contract.clone(),
        artifact: artifact_fingerprint(),
        result,
        settlement: ProviderSettlementDiagnostics::from_request(
            operation,
            &request,
            options,
            started_at,
            pending_async_task_count,
            output.len(),
        ),
        output,
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProviderCallResult {
    Ok(StructuralValue),
    Err(ProviderError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportCallResult {
    Ready(ProviderCallResult),
    Pending(BunAsyncHandle),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PumpBudget {
    pub max_ticks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PumpOutcome {
    pub ticks: u32,
    pub pending_async_work: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputRecord {
    pub stream: OutputStream,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputStream {
    Stdout,
    Stderr,
    Log,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapturedOutput {
    records: Vec<OutputRecord>,
}

impl CapturedOutput {
    pub fn push(&mut self, record: OutputRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[OutputRecord] {
        &self.records
    }

    pub fn drain(&mut self) -> Vec<OutputRecord> {
        std::mem::take(&mut self.records)
    }
}

pub type OutputHandler = Box<dyn FnMut(OutputRecord) + Send + 'static>;

pub trait BunEmbeddingRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self>
    where
        Self: Sized;

    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle>;

    fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: StructuralValue,
    ) -> LibbunResult<ExportCallResult>;

    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome>;

    fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>>;

    fn call_provider_until_settled(
        &mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledProviderReceipt> {
        let started_at = Instant::now();
        let mut output = self.drain_captured_output();
        let mut pending_async_task_count = 0;
        let mut trace = Vec::new();
        push_settlement_trace(
            &mut trace,
            ProviderSettlementPhase::Start,
            started_at,
            pending_async_task_count,
        );

        if request.domain == ProviderDomainClass::RustSubstrateAuthority {
            push_settlement_trace(
                &mut trace,
                ProviderSettlementPhase::Complete,
                started_at,
                pending_async_task_count,
            );
            return Ok(SettledProviderReceipt::Ready {
                contract: request.contract.clone(),
                artifact: artifact_fingerprint(),
                result: ProviderCallResult::Err(ProviderError {
                    code: "rust_substrate_authority_rejected".to_string(),
                    message: "libbun cannot execute Rust-substrate provider exports".to_string(),
                }),
                settlement: ProviderSettlementDiagnostics::from_request(
                    ProviderExecutionOperation::ProviderCallableValidate,
                    &request,
                    options,
                    started_at,
                    pending_async_task_count,
                    output.len(),
                )
                .with_trace(trace),
                output,
            });
        }

        push_settlement_trace(
            &mut trace,
            ProviderSettlementPhase::ModuleLoad,
            started_at,
            pending_async_task_count,
        );
        let module = match self.load_module(request.module.clone()) {
            Ok(module) => {
                output.extend(self.drain_captured_output());
                module
            }
            Err(err) => {
                output.extend(self.drain_captured_output());
                return Ok(SettledProviderReceipt::Failed(
                    ProviderExecutionFailure::new(
                        ProviderExecutionOperation::ProviderModuleImport,
                        &request,
                        options,
                        started_at,
                        pending_async_task_count,
                        output,
                        err.to_string(),
                    )
                    .with_trace(trace),
                ));
            }
        };

        push_settlement_trace(
            &mut trace,
            ProviderSettlementPhase::CallExport,
            started_at,
            pending_async_task_count,
        );
        let export_result = match self.call_export(&module, &request.export, request.input.clone())
        {
            Ok(result) => {
                output.extend(self.drain_captured_output());
                result
            }
            Err(err) => {
                output.extend(self.drain_captured_output());
                return Ok(SettledProviderReceipt::Failed(
                    ProviderExecutionFailure::new(
                        export_failure_operation(&err),
                        &request,
                        options,
                        started_at,
                        pending_async_task_count,
                        output,
                        err.to_string(),
                    )
                    .with_trace(trace),
                ));
            }
        };

        match export_result {
            ExportCallResult::Ready(result) => {
                push_settlement_trace(
                    &mut trace,
                    ProviderSettlementPhase::Complete,
                    started_at,
                    pending_async_task_count,
                );
                settled_ready_or_failure(
                    request,
                    options,
                    started_at,
                    pending_async_task_count,
                    output,
                    result,
                    ProviderExecutionOperation::ProviderCallableInvoke,
                )
                .map(|receipt| receipt.with_trace(trace))
            }
            ExportCallResult::Pending(handle) => loop {
                if deadline_elapsed(started_at, options.deadline) {
                    output.extend(self.drain_captured_output());
                    push_settlement_trace(
                        &mut trace,
                        ProviderSettlementPhase::DeadlineElapsed,
                        started_at,
                        pending_async_task_count.max(1),
                    );
                    return Ok(SettledProviderReceipt::Failed(
                        ProviderExecutionFailure::new(
                            ProviderExecutionOperation::ProviderDeadlineElapsed,
                            &request,
                            options,
                            started_at,
                            pending_async_task_count.max(1),
                            output,
                            format!(
                                "provider deadline elapsed while settling `{}` after {} ms",
                                request.export,
                                elapsed_ms(started_at)
                            ),
                        )
                        .with_trace(trace),
                    ));
                }

                push_settlement_trace(
                    &mut trace,
                    ProviderSettlementPhase::ResolveAsync,
                    started_at,
                    pending_async_task_count,
                );
                match self.resolve_async(&handle) {
                    Ok(Some(result)) => {
                        output.extend(self.drain_captured_output());
                        push_settlement_trace(
                            &mut trace,
                            ProviderSettlementPhase::Complete,
                            started_at,
                            pending_async_task_count,
                        );
                        break settled_ready_or_failure(
                            request,
                            options,
                            started_at,
                            pending_async_task_count,
                            output,
                            result,
                            ProviderExecutionOperation::ProviderPromiseSettle,
                        )
                        .map(|receipt| receipt.with_trace(trace));
                    }
                    Ok(None) => {
                        output.extend(self.drain_captured_output());
                    }
                    Err(err) => {
                        output.extend(self.drain_captured_output());
                        break Ok(SettledProviderReceipt::Failed(
                            ProviderExecutionFailure::new(
                                ProviderExecutionOperation::ProviderPromiseSettle,
                                &request,
                                options,
                                started_at,
                                pending_async_task_count,
                                output,
                                err.to_string(),
                            )
                            .with_trace(trace),
                        ));
                    }
                }

                push_settlement_trace(
                    &mut trace,
                    ProviderSettlementPhase::PumpEventLoop,
                    started_at,
                    pending_async_task_count,
                );
                match self.pump_event_loop(PumpBudget { max_ticks: 1 }) {
                    Ok(outcome) => {
                        pending_async_task_count = outcome.pending_async_work;
                        output.extend(self.drain_captured_output());
                    }
                    Err(err) => {
                        output.extend(self.drain_captured_output());
                        break Ok(SettledProviderReceipt::Failed(
                            ProviderExecutionFailure::new(
                                ProviderExecutionOperation::ProviderPromiseSettle,
                                &request,
                                options,
                                started_at,
                                pending_async_task_count,
                                output,
                                err.to_string(),
                            )
                            .with_trace(trace),
                        ));
                    }
                }
            },
        }
    }

    fn captured_output(&self) -> &[OutputRecord];

    fn drain_captured_output(&mut self) -> Vec<OutputRecord>;

    fn shutdown(&mut self) -> LibbunResult<()>;
}

pub struct BunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    output: CapturedOutput,
    output_handler: Option<OutputHandler>,
    output_policies: OutputPolicies,
    shutdown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OutputPolicies {
    stdout: SinkPolicy,
    stderr: SinkPolicy,
    log: SinkPolicy,
}

impl OutputPolicies {
    fn from_config(config: &BunRuntimeConfig) -> Self {
        Self {
            stdout: config.stdout,
            stderr: config.stderr,
            log: config.log,
        }
    }

    fn captures(self, stream: OutputStream) -> bool {
        match stream {
            OutputStream::Stdout => self.stdout == SinkPolicy::Capture,
            OutputStream::Stderr => self.stderr == SinkPolicy::Capture,
            OutputStream::Log => self.log == SinkPolicy::Capture,
        }
    }
}

impl<R: BunEmbeddingRuntime> BunHost<R> {
    pub fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn initialize_with_output_handler(
        config: BunRuntimeConfig,
        output_handler: impl FnMut(OutputRecord) + Send + 'static,
    ) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: Some(Box::new(output_handler)),
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn from_runtime(config: BunRuntimeConfig, runtime: R) -> Self {
        let output_policies = OutputPolicies::from_config(&config);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        host
    }

    pub fn call_provider_until_settled(
        &mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledProviderReceipt> {
        self.ensure_live()?;
        let result = self.runtime.call_provider_until_settled(request, options);
        if let Ok(receipt) = &result {
            self.collect_returned_output(receipt.output());
        }
        self.collect_output();
        result
    }

    pub fn captured_output(&self) -> &[OutputRecord] {
        self.output.records()
    }

    pub fn drain_captured_output(&mut self) -> Vec<OutputRecord> {
        self.output.drain()
    }

    pub fn shutdown(&mut self) -> LibbunResult<()> {
        if self.shutdown {
            return Ok(());
        }
        let result = self.runtime.shutdown();
        self.collect_output();
        result?;
        self.shutdown = true;
        Ok(())
    }

    fn ensure_live(&self) -> LibbunResult<()> {
        if self.shutdown {
            Err(LibbunError::RuntimeShutdown)
        } else {
            Ok(())
        }
    }

    fn collect_output(&mut self) {
        for record in self.runtime.drain_captured_output() {
            self.collect_output_record(record);
        }
    }

    fn collect_returned_output(&mut self, records: &[OutputRecord]) {
        for record in records {
            self.collect_output_record(record.clone());
        }
    }

    fn collect_output_record(&mut self, record: OutputRecord) {
        if !self.output_policies.captures(record.stream) {
            return;
        }
        if let Some(handler) = self.output_handler.as_mut() {
            handler(record.clone());
        }
        self.output.push(record);
    }
}

pub struct LowLevelBunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    output: CapturedOutput,
    output_handler: Option<OutputHandler>,
    output_policies: OutputPolicies,
    shutdown: bool,
}

impl<R: BunEmbeddingRuntime> LowLevelBunHost<R> {
    pub fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn initialize_with_output_handler(
        config: BunRuntimeConfig,
        output_handler: impl FnMut(OutputRecord) + Send + 'static,
    ) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: Some(Box::new(output_handler)),
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn from_runtime(config: BunRuntimeConfig, runtime: R) -> Self {
        let output_policies = OutputPolicies::from_config(&config);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            shutdown: false,
        };
        host.collect_output();
        host
    }

    pub fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
        self.ensure_live()?;
        let result = self.runtime.load_module(spec);
        self.collect_output();
        result
    }

    pub fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: impl Into<StructuralValue>,
    ) -> LibbunResult<ExportCallResult> {
        self.ensure_live()?;
        let result = self.runtime.call_export(module, export, input.into());
        self.collect_output();
        result
    }

    pub fn call_provider_until_settled(
        &mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledProviderReceipt> {
        self.ensure_live()?;
        let result = self.runtime.call_provider_until_settled(request, options);
        if let Ok(receipt) = &result {
            self.collect_returned_output(receipt.output());
        }
        self.collect_output();
        result
    }

    pub fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
        self.ensure_live()?;
        let result = self.runtime.pump_event_loop(budget);
        self.collect_output();
        result
    }

    pub fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>> {
        self.ensure_live()?;
        let result = self.runtime.resolve_async(handle);
        self.collect_output();
        result
    }

    pub fn captured_output(&self) -> &[OutputRecord] {
        self.output.records()
    }

    pub fn drain_captured_output(&mut self) -> Vec<OutputRecord> {
        self.output.drain()
    }

    pub fn shutdown(&mut self) -> LibbunResult<()> {
        if self.shutdown {
            return Ok(());
        }
        let result = self.runtime.shutdown();
        self.collect_output();
        result?;
        self.shutdown = true;
        Ok(())
    }

    fn ensure_live(&self) -> LibbunResult<()> {
        if self.shutdown {
            Err(LibbunError::RuntimeShutdown)
        } else {
            Ok(())
        }
    }

    fn collect_output(&mut self) {
        for record in self.runtime.drain_captured_output() {
            self.collect_output_record(record);
        }
    }

    fn collect_returned_output(&mut self, records: &[OutputRecord]) {
        for record in records {
            self.collect_output_record(record.clone());
        }
    }

    fn collect_output_record(&mut self, record: OutputRecord) {
        if !self.output_policies.captures(record.stream) {
            return;
        }
        if let Some(handler) = self.output_handler.as_mut() {
            handler(record.clone());
        }
        self.output.push(record);
    }
}

impl<R> std::fmt::Debug for BunHost<R>
where
    R: BunEmbeddingRuntime + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BunHost")
            .field("runtime", &self.runtime)
            .field("output", &self.output)
            .field("output_policies", &self.output_policies)
            .field("shutdown", &self.shutdown)
            .finish_non_exhaustive()
    }
}

impl<R> std::fmt::Debug for LowLevelBunHost<R>
where
    R: BunEmbeddingRuntime + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LowLevelBunHost")
            .field("runtime", &self.runtime)
            .field("output", &self.output)
            .field("output_policies", &self.output_policies)
            .field("shutdown", &self.shutdown)
            .finish_non_exhaustive()
    }
}

impl<R> Drop for BunHost<R>
where
    R: BunEmbeddingRuntime,
{
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

impl<R> Drop for LowLevelBunHost<R>
where
    R: BunEmbeddingRuntime,
{
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LibbunError {
    #[error("runtime initialization failed: {message}")]
    Initialize { message: String },
    #[error("module load failed: {message}")]
    ModuleLoad { message: String },
    #[error("export call failed before provider code returned: {message}")]
    ExportCall { message: String },
    #[error("async handle `{handle}` is unknown")]
    UnknownAsyncHandle { handle: String },
    #[error("event loop pump failed: {message}")]
    EventLoopPump { message: String },
    #[error("runtime has already shut down")]
    RuntimeShutdown,
    #[error("shutdown failed: {message}")]
    Shutdown { message: String },
}

impl LibbunError {
    pub fn initialize(message: impl Into<String>) -> Self {
        Self::Initialize {
            message: message.into(),
        }
    }

    pub fn module_load(message: impl Into<String>) -> Self {
        Self::ModuleLoad {
            message: message.into(),
        }
    }

    pub fn export_call(message: impl Into<String>) -> Self {
        Self::ExportCall {
            message: message.into(),
        }
    }

    pub fn event_loop_pump(message: impl Into<String>) -> Self {
        Self::EventLoopPump {
            message: message.into(),
        }
    }

    pub fn shutdown(message: impl Into<String>) -> Self {
        Self::Shutdown {
            message: message.into(),
        }
    }
}
