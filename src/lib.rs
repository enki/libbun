//! Hostable Bun embedding facade.
//!
//! This crate owns the stable Rust boundary for hosting JavaScript and
//! TypeScript providers through Bun. It deliberately does not call Bun CLI
//! entrypoints and does not expose raw JSC handles across its public API.

use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettleOptions {
    pub deadline: ProviderDeadline,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<ProviderCallId>,
}

impl ProviderSettleOptions {
    pub fn new(deadline: ProviderDeadline) -> Self {
        Self {
            deadline,
            call_id: None,
        }
    }

    pub fn with_call_id(mut self, call_id: impl Into<String>) -> Self {
        self.call_id = Some(ProviderCallId(call_id.into()));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProviderCallId(pub String);

pub const PROVIDER_DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;

pub trait ProviderDiagnosticSink: Send + Sync + 'static {
    fn provider_event(&self, event: ProviderDiagnosticEvent);
}

impl<F> ProviderDiagnosticSink for F
where
    F: Fn(ProviderDiagnosticEvent) + Send + Sync + 'static,
{
    fn provider_event(&self, event: ProviderDiagnosticEvent) {
        self(event);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDiagnosticEvent {
    pub schema_version: u32,
    pub call_id: ProviderCallId,
    pub sequence: u64,
    pub span_id: u64,
    pub parent_span_id: Option<u64>,
    pub wall_time_unix_ms: u64,
    pub elapsed_ms: u64,
    pub kind: ProviderDiagnosticEventKind,
    pub phase: ProviderDiagnosticPhase,
    pub operation: ProviderExecutionOperation,
    pub contract: ProviderContractIdentity,
    pub provider_domain_class: ProviderDomainClass,
    pub module_specifier_or_url: String,
    pub export_name: String,
    pub deadline_ms: u64,
    pub pending_async_task_count: Option<usize>,
    pub captured_output_record_count: Option<usize>,
    pub libbun_version: String,
    pub libbun_abi_version: u32,
    pub bun_revision: String,
    pub dynamic_plugin_path: Option<PathBuf>,
    pub dynamic_plugin_sha256: Option<String>,
    pub runtime_instance_id: String,
    pub process_id: u32,
    pub thread_id: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDiagnosticEventKind {
    CallStart,
    PhaseEnter,
    PhaseExit,
    DeadlineElapsed,
    CallComplete,
    CallFailed,
    OutputCaptured,
    RuntimeSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDiagnosticPhase {
    RuntimeInitialize,
    ModuleLoad,
    CallExport,
    ResolveAsync,
    PumpEventLoop,
    DrainOutput,
    Shutdown,
    DeadlineElapsed,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRuntimeSnapshot {
    pub active_call: Option<ProviderCallSnapshot>,
    pub recent_events: Vec<ProviderDiagnosticEvent>,
    pub captured_output_count: usize,
    pub runtime_state: ProviderRuntimeState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCallSnapshot {
    pub call_id: ProviderCallId,
    pub contract: ProviderContractIdentity,
    pub provider_domain_class: ProviderDomainClass,
    pub module_specifier_or_url: String,
    pub export_name: String,
    pub deadline_ms: u64,
    pub started_wall_time_unix_ms: u64,
    pub latest_event: Option<ProviderDiagnosticEvent>,
    pub unmatched_phase_enters: Vec<ProviderDiagnosticEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRuntimeState {
    Initializing,
    Ready,
    InProviderCall,
    ShuttingDown,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRuntimeDiagnosticMetadata {
    pub runtime_instance_id: String,
    pub libbun_version: String,
    pub libbun_abi_version: u32,
    pub bun_revision: String,
    pub dynamic_plugin_path: Option<PathBuf>,
    pub dynamic_plugin_sha256: Option<String>,
}

impl Default for ProviderRuntimeDiagnosticMetadata {
    fn default() -> Self {
        Self {
            runtime_instance_id: next_runtime_instance_id(),
            libbun_version: env!("CARGO_PKG_VERSION").to_string(),
            libbun_abi_version: LIBBUN_ABI_VERSION,
            bun_revision: env!("LIBBUN_BUN_SOURCE_COMMIT").to_string(),
            dynamic_plugin_path: None,
            dynamic_plugin_sha256: None,
        }
    }
}

#[derive(Clone)]
pub struct ProviderDiagnosticsHandle {
    state: Arc<ProviderDiagnosticsState>,
}

struct ProviderDiagnosticsState {
    inner: Mutex<ProviderDiagnosticsInner>,
    sequence: AtomicU64,
    span: AtomicU64,
    sink: Option<Arc<dyn ProviderDiagnosticSink>>,
}

#[derive(Debug)]
struct ProviderDiagnosticsInner {
    metadata: ProviderRuntimeDiagnosticMetadata,
    runtime_state: ProviderRuntimeState,
    active_call: Option<ProviderCallSnapshot>,
    recent_events: VecDeque<ProviderDiagnosticEvent>,
    open_spans: BTreeMap<u64, ProviderDiagnosticEvent>,
    captured_output_count: usize,
}

#[derive(Debug, Clone)]
struct ProviderDiagnosticCallContext {
    call_id: ProviderCallId,
    request: ProviderRequest,
    options: ProviderSettleOptions,
    started_at: Instant,
    started_wall_time_unix_ms: u64,
}

impl ProviderDiagnosticsHandle {
    pub fn new() -> Self {
        Self::from_optional_sink(None)
    }

    pub fn new_with_sink(sink: impl ProviderDiagnosticSink) -> Self {
        Self::from_optional_sink(Some(Arc::new(sink)))
    }

    fn from_optional_sink(sink: Option<Arc<dyn ProviderDiagnosticSink>>) -> Self {
        Self {
            state: Arc::new(ProviderDiagnosticsState {
                inner: Mutex::new(ProviderDiagnosticsInner {
                    metadata: ProviderRuntimeDiagnosticMetadata::default(),
                    runtime_state: ProviderRuntimeState::Initializing,
                    active_call: None,
                    recent_events: VecDeque::new(),
                    open_spans: BTreeMap::new(),
                    captured_output_count: 0,
                }),
                sequence: AtomicU64::new(1),
                span: AtomicU64::new(1),
                sink,
            }),
        }
    }

    pub fn snapshot(&self) -> ProviderRuntimeSnapshot {
        let inner = self
            .state
            .inner
            .lock()
            .expect("provider diagnostics state poisoned");
        ProviderRuntimeSnapshot {
            active_call: inner.active_call.clone(),
            recent_events: inner.recent_events.iter().cloned().collect(),
            captured_output_count: inner.captured_output_count,
            runtime_state: inner.runtime_state,
        }
    }

    fn set_metadata(&self, metadata: ProviderRuntimeDiagnosticMetadata) {
        self.state
            .inner
            .lock()
            .expect("provider diagnostics state poisoned")
            .metadata = metadata;
    }

    fn set_runtime_state(&self, runtime_state: ProviderRuntimeState) {
        self.state
            .inner
            .lock()
            .expect("provider diagnostics state poisoned")
            .runtime_state = runtime_state;
    }

    fn add_captured_output(&self, count: usize) {
        if count == 0 {
            return;
        }
        self.state
            .inner
            .lock()
            .expect("provider diagnostics state poisoned")
            .captured_output_count += count;
    }

    fn start_call(
        &self,
        request: &ProviderRequest,
        options: ProviderSettleOptions,
        started_at: Instant,
    ) -> ProviderDiagnosticCallContext {
        let call_id = options
            .call_id
            .clone()
            .unwrap_or_else(next_provider_call_id);
        let context = ProviderDiagnosticCallContext {
            call_id: call_id.clone(),
            request: request.clone(),
            options,
            started_at,
            started_wall_time_unix_ms: wall_time_unix_ms(),
        };
        {
            let mut inner = self
                .state
                .inner
                .lock()
                .expect("provider diagnostics state poisoned");
            inner.runtime_state = ProviderRuntimeState::InProviderCall;
            inner.open_spans.clear();
            inner.active_call = Some(ProviderCallSnapshot {
                call_id,
                contract: request.contract.clone(),
                provider_domain_class: request.domain,
                module_specifier_or_url: module_specifier_or_url(&request.module),
                export_name: request.export.clone(),
                deadline_ms: context.options.deadline.deadline_ms,
                started_wall_time_unix_ms: context.started_wall_time_unix_ms,
                latest_event: None,
                unmatched_phase_enters: Vec::new(),
            });
        }
        self.record(
            &context,
            ProviderDiagnosticEventKind::CallStart,
            ProviderDiagnosticPhase::RuntimeInitialize,
            ProviderExecutionOperation::RuntimeInitialize,
            0,
            None,
            None,
            None,
        );
        context
    }

    fn enter_phase(
        &self,
        context: &ProviderDiagnosticCallContext,
        phase: ProviderDiagnosticPhase,
        operation: ProviderExecutionOperation,
        pending_async_task_count: usize,
    ) -> u64 {
        let span_id = self.state.span.fetch_add(1, Ordering::Relaxed);
        self.record(
            context,
            ProviderDiagnosticEventKind::PhaseEnter,
            phase,
            operation,
            span_id,
            None,
            Some(pending_async_task_count),
            None,
        );
        span_id
    }

    fn exit_phase(
        &self,
        context: &ProviderDiagnosticCallContext,
        phase: ProviderDiagnosticPhase,
        operation: ProviderExecutionOperation,
        span_id: u64,
        pending_async_task_count: usize,
        output_record_count: usize,
    ) {
        self.record(
            context,
            ProviderDiagnosticEventKind::PhaseExit,
            phase,
            operation,
            span_id,
            None,
            Some(pending_async_task_count),
            Some(output_record_count),
        );
    }

    fn finish_call(
        &self,
        context: &ProviderDiagnosticCallContext,
        kind: ProviderDiagnosticEventKind,
        phase: ProviderDiagnosticPhase,
        operation: ProviderExecutionOperation,
        pending_async_task_count: usize,
        output_record_count: usize,
        detail: Option<String>,
    ) {
        self.record(
            context,
            kind,
            phase,
            operation,
            0,
            detail,
            Some(pending_async_task_count),
            Some(output_record_count),
        );
        let mut inner = self
            .state
            .inner
            .lock()
            .expect("provider diagnostics state poisoned");
        inner.runtime_state = ProviderRuntimeState::Ready;
        inner.active_call = None;
        inner.open_spans.clear();
    }

    #[allow(clippy::too_many_arguments)]
    fn record(
        &self,
        context: &ProviderDiagnosticCallContext,
        kind: ProviderDiagnosticEventKind,
        phase: ProviderDiagnosticPhase,
        operation: ProviderExecutionOperation,
        span_id: u64,
        detail: Option<String>,
        pending_async_task_count: Option<usize>,
        captured_output_record_count: Option<usize>,
    ) {
        let sequence = self.state.sequence.fetch_add(1, Ordering::Relaxed);
        let metadata = {
            self.state
                .inner
                .lock()
                .expect("provider diagnostics state poisoned")
                .metadata
                .clone()
        };
        let event = ProviderDiagnosticEvent {
            schema_version: PROVIDER_DIAGNOSTIC_SCHEMA_VERSION,
            call_id: context.call_id.clone(),
            sequence,
            span_id,
            parent_span_id: None,
            wall_time_unix_ms: wall_time_unix_ms(),
            elapsed_ms: elapsed_ms(context.started_at),
            kind,
            phase,
            operation,
            contract: context.request.contract.clone(),
            provider_domain_class: context.request.domain,
            module_specifier_or_url: module_specifier_or_url(&context.request.module),
            export_name: context.request.export.clone(),
            deadline_ms: context.options.deadline.deadline_ms,
            pending_async_task_count,
            captured_output_record_count,
            libbun_version: metadata.libbun_version,
            libbun_abi_version: metadata.libbun_abi_version,
            bun_revision: metadata.bun_revision,
            dynamic_plugin_path: metadata.dynamic_plugin_path,
            dynamic_plugin_sha256: metadata.dynamic_plugin_sha256,
            runtime_instance_id: metadata.runtime_instance_id,
            process_id: std::process::id(),
            thread_id: format!("{:?}", std::thread::current().id()),
            detail,
        };
        let sink = {
            let mut inner = self
                .state
                .inner
                .lock()
                .expect("provider diagnostics state poisoned");
            const MAX_RECENT_EVENTS: usize = 128;
            if inner.recent_events.len() == MAX_RECENT_EVENTS {
                inner.recent_events.pop_front();
            }
            if kind == ProviderDiagnosticEventKind::PhaseEnter && span_id != 0 {
                inner.open_spans.insert(span_id, event.clone());
            } else if kind == ProviderDiagnosticEventKind::PhaseExit && span_id != 0 {
                inner.open_spans.remove(&span_id);
            }
            inner.recent_events.push_back(event.clone());
            let unmatched: Vec<_> = inner.open_spans.values().cloned().collect();
            if let Some(active_call) = inner.active_call.as_mut() {
                active_call.latest_event = Some(event.clone());
                active_call.unmatched_phase_enters = unmatched;
            }
            self.state.sink.clone()
        };
        if let Some(sink) = sink {
            sink.provider_event(event);
        }
    }
}

impl Default for ProviderDiagnosticsHandle {
    fn default() -> Self {
        Self::new()
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
    pub call_id: Option<ProviderCallId>,
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
    pub call_id: Option<ProviderCallId>,
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
        options: &ProviderSettleOptions,
        started_at: Instant,
        pending_async_task_count: usize,
        output: Vec<OutputRecord>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let (js_error_name, js_error_stack) = js_error_fields(&message);
        Self {
            call_id: options.call_id.clone(),
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
        options: &ProviderSettleOptions,
        started_at: Instant,
        pending_async_task_count: usize,
        output_record_count: usize,
    ) -> Self {
        Self {
            call_id: options.call_id.clone(),
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

fn wall_time_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn next_provider_call_id() -> ProviderCallId {
    static NEXT_PROVIDER_CALL: AtomicU64 = AtomicU64::new(1);
    ProviderCallId(format!(
        "provider-call-{}",
        NEXT_PROVIDER_CALL.fetch_add(1, Ordering::Relaxed)
    ))
}

fn next_runtime_instance_id() -> String {
    static NEXT_RUNTIME_INSTANCE: AtomicU64 = AtomicU64::new(1);
    format!(
        "runtime-{}",
        NEXT_RUNTIME_INSTANCE.fetch_add(1, Ordering::Relaxed)
    )
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
    options: &ProviderSettleOptions,
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

fn ensure_provider_call_id(mut options: ProviderSettleOptions) -> ProviderSettleOptions {
    if options.call_id.is_none() {
        options.call_id = Some(next_provider_call_id());
    }
    options
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

    fn diagnostic_metadata(&self) -> ProviderRuntimeDiagnosticMetadata {
        ProviderRuntimeDiagnosticMetadata::default()
    }

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
        call_provider_until_settled_observed(self, request, options, None)
    }

    fn captured_output(&self) -> &[OutputRecord];

    fn drain_captured_output(&mut self) -> Vec<OutputRecord>;

    fn shutdown(&mut self) -> LibbunResult<()>;
}

fn call_provider_until_settled_observed<R: BunEmbeddingRuntime + ?Sized>(
    runtime: &mut R,
    request: ProviderRequest,
    options: ProviderSettleOptions,
    diagnostics: Option<&ProviderDiagnosticsHandle>,
) -> LibbunResult<SettledProviderReceipt> {
    let options = ensure_provider_call_id(options);
    let started_at = Instant::now();
    let diagnostic_context =
        diagnostics.map(|handle| handle.start_call(&request, options.clone(), started_at));
    let mut output = runtime.drain_captured_output();
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
        let receipt = SettledProviderReceipt::Ready {
            contract: request.contract.clone(),
            artifact: artifact_fingerprint(),
            result: ProviderCallResult::Err(ProviderError {
                code: "rust_substrate_authority_rejected".to_string(),
                message: "libbun cannot execute Rust-substrate provider exports".to_string(),
            }),
            settlement: ProviderSettlementDiagnostics::from_request(
                ProviderExecutionOperation::ProviderCallableValidate,
                &request,
                &options,
                started_at,
                pending_async_task_count,
                output.len(),
            )
            .with_trace(trace),
            output,
        };
        finish_diagnostic_call(
            diagnostics,
            diagnostic_context.as_ref(),
            &receipt,
            ProviderDiagnosticEventKind::CallComplete,
            ProviderDiagnosticPhase::Complete,
            ProviderExecutionOperation::ProviderCallableValidate,
        );
        return Ok(receipt);
    }

    push_settlement_trace(
        &mut trace,
        ProviderSettlementPhase::ModuleLoad,
        started_at,
        pending_async_task_count,
    );
    let module_span = enter_diagnostic_phase(
        diagnostics,
        diagnostic_context.as_ref(),
        ProviderDiagnosticPhase::ModuleLoad,
        ProviderExecutionOperation::ProviderModuleImport,
        pending_async_task_count,
    );
    let module = match runtime.load_module(request.module.clone()) {
        Ok(module) => {
            output.extend(runtime.drain_captured_output());
            exit_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::ModuleLoad,
                ProviderExecutionOperation::ProviderModuleImport,
                module_span,
                pending_async_task_count,
                output.len(),
            );
            module
        }
        Err(err) => {
            output.extend(runtime.drain_captured_output());
            exit_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::ModuleLoad,
                ProviderExecutionOperation::ProviderModuleImport,
                module_span,
                pending_async_task_count,
                output.len(),
            );
            let receipt = SettledProviderReceipt::Failed(
                ProviderExecutionFailure::new(
                    ProviderExecutionOperation::ProviderModuleImport,
                    &request,
                    &options,
                    started_at,
                    pending_async_task_count,
                    output,
                    err.to_string(),
                )
                .with_trace(trace),
            );
            finish_diagnostic_call(
                diagnostics,
                diagnostic_context.as_ref(),
                &receipt,
                ProviderDiagnosticEventKind::CallFailed,
                ProviderDiagnosticPhase::ModuleLoad,
                ProviderExecutionOperation::ProviderModuleImport,
            );
            return Ok(receipt);
        }
    };

    push_settlement_trace(
        &mut trace,
        ProviderSettlementPhase::CallExport,
        started_at,
        pending_async_task_count,
    );
    let export_span = enter_diagnostic_phase(
        diagnostics,
        diagnostic_context.as_ref(),
        ProviderDiagnosticPhase::CallExport,
        ProviderExecutionOperation::ProviderCallableInvoke,
        pending_async_task_count,
    );
    let export_result = match runtime.call_export(&module, &request.export, request.input.clone()) {
        Ok(result) => {
            output.extend(runtime.drain_captured_output());
            exit_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::CallExport,
                ProviderExecutionOperation::ProviderCallableInvoke,
                export_span,
                pending_async_task_count,
                output.len(),
            );
            result
        }
        Err(err) => {
            output.extend(runtime.drain_captured_output());
            let operation = export_failure_operation(&err);
            exit_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::CallExport,
                operation,
                export_span,
                pending_async_task_count,
                output.len(),
            );
            let receipt = SettledProviderReceipt::Failed(
                ProviderExecutionFailure::new(
                    operation,
                    &request,
                    &options,
                    started_at,
                    pending_async_task_count,
                    output,
                    err.to_string(),
                )
                .with_trace(trace),
            );
            finish_diagnostic_call(
                diagnostics,
                diagnostic_context.as_ref(),
                &receipt,
                ProviderDiagnosticEventKind::CallFailed,
                ProviderDiagnosticPhase::CallExport,
                operation,
            );
            return Ok(receipt);
        }
    };

    let receipt = match export_result {
        ExportCallResult::Ready(result) => {
            push_settlement_trace(
                &mut trace,
                ProviderSettlementPhase::Complete,
                started_at,
                pending_async_task_count,
            );
            settled_ready_or_failure(
                request,
                &options,
                started_at,
                pending_async_task_count,
                output,
                result,
                ProviderExecutionOperation::ProviderCallableInvoke,
            )
            .map(|receipt| receipt.with_trace(trace))?
        }
        ExportCallResult::Pending(handle) => loop {
            if deadline_elapsed(started_at, options.deadline) {
                output.extend(runtime.drain_captured_output());
                push_settlement_trace(
                    &mut trace,
                    ProviderSettlementPhase::DeadlineElapsed,
                    started_at,
                    pending_async_task_count.max(1),
                );
                let receipt = SettledProviderReceipt::Failed(
                    ProviderExecutionFailure::new(
                        ProviderExecutionOperation::ProviderDeadlineElapsed,
                        &request,
                        &options,
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
                );
                break receipt;
            }

            push_settlement_trace(
                &mut trace,
                ProviderSettlementPhase::ResolveAsync,
                started_at,
                pending_async_task_count,
            );
            let resolve_span = enter_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::ResolveAsync,
                ProviderExecutionOperation::ProviderPromiseSettle,
                pending_async_task_count,
            );
            match runtime.resolve_async(&handle) {
                Ok(Some(result)) => {
                    output.extend(runtime.drain_captured_output());
                    exit_diagnostic_phase(
                        diagnostics,
                        diagnostic_context.as_ref(),
                        ProviderDiagnosticPhase::ResolveAsync,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                        resolve_span,
                        pending_async_task_count,
                        output.len(),
                    );
                    push_settlement_trace(
                        &mut trace,
                        ProviderSettlementPhase::Complete,
                        started_at,
                        pending_async_task_count,
                    );
                    break settled_ready_or_failure(
                        request,
                        &options,
                        started_at,
                        pending_async_task_count,
                        output,
                        result,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                    )
                    .map(|receipt| receipt.with_trace(trace))?;
                }
                Ok(None) => {
                    output.extend(runtime.drain_captured_output());
                    exit_diagnostic_phase(
                        diagnostics,
                        diagnostic_context.as_ref(),
                        ProviderDiagnosticPhase::ResolveAsync,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                        resolve_span,
                        pending_async_task_count,
                        output.len(),
                    );
                }
                Err(err) => {
                    output.extend(runtime.drain_captured_output());
                    exit_diagnostic_phase(
                        diagnostics,
                        diagnostic_context.as_ref(),
                        ProviderDiagnosticPhase::ResolveAsync,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                        resolve_span,
                        pending_async_task_count,
                        output.len(),
                    );
                    break SettledProviderReceipt::Failed(
                        ProviderExecutionFailure::new(
                            ProviderExecutionOperation::ProviderPromiseSettle,
                            &request,
                            &options,
                            started_at,
                            pending_async_task_count,
                            output,
                            err.to_string(),
                        )
                        .with_trace(trace),
                    );
                }
            }

            push_settlement_trace(
                &mut trace,
                ProviderSettlementPhase::PumpEventLoop,
                started_at,
                pending_async_task_count,
            );
            let pump_span = enter_diagnostic_phase(
                diagnostics,
                diagnostic_context.as_ref(),
                ProviderDiagnosticPhase::PumpEventLoop,
                ProviderExecutionOperation::ProviderPromiseSettle,
                pending_async_task_count,
            );
            match runtime.pump_event_loop(PumpBudget { max_ticks: 1 }) {
                Ok(outcome) => {
                    pending_async_task_count = outcome.pending_async_work;
                    output.extend(runtime.drain_captured_output());
                    exit_diagnostic_phase(
                        diagnostics,
                        diagnostic_context.as_ref(),
                        ProviderDiagnosticPhase::PumpEventLoop,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                        pump_span,
                        pending_async_task_count,
                        output.len(),
                    );
                }
                Err(err) => {
                    output.extend(runtime.drain_captured_output());
                    exit_diagnostic_phase(
                        diagnostics,
                        diagnostic_context.as_ref(),
                        ProviderDiagnosticPhase::PumpEventLoop,
                        ProviderExecutionOperation::ProviderPromiseSettle,
                        pump_span,
                        pending_async_task_count,
                        output.len(),
                    );
                    break SettledProviderReceipt::Failed(
                        ProviderExecutionFailure::new(
                            ProviderExecutionOperation::ProviderPromiseSettle,
                            &request,
                            &options,
                            started_at,
                            pending_async_task_count,
                            output,
                            err.to_string(),
                        )
                        .with_trace(trace),
                    );
                }
            }
        },
    };

    let (kind, phase, operation) = match &receipt {
        SettledProviderReceipt::Ready { settlement, .. } => (
            ProviderDiagnosticEventKind::CallComplete,
            ProviderDiagnosticPhase::Complete,
            settlement.operation,
        ),
        SettledProviderReceipt::Failed(failure)
            if failure.operation == ProviderExecutionOperation::ProviderDeadlineElapsed =>
        {
            (
                ProviderDiagnosticEventKind::DeadlineElapsed,
                ProviderDiagnosticPhase::DeadlineElapsed,
                failure.operation,
            )
        }
        SettledProviderReceipt::Failed(failure) => (
            ProviderDiagnosticEventKind::CallFailed,
            ProviderDiagnosticPhase::Complete,
            failure.operation,
        ),
    };
    finish_diagnostic_call(
        diagnostics,
        diagnostic_context.as_ref(),
        &receipt,
        kind,
        phase,
        operation,
    );
    Ok(receipt)
}

fn enter_diagnostic_phase(
    diagnostics: Option<&ProviderDiagnosticsHandle>,
    context: Option<&ProviderDiagnosticCallContext>,
    phase: ProviderDiagnosticPhase,
    operation: ProviderExecutionOperation,
    pending_async_task_count: usize,
) -> u64 {
    match (diagnostics, context) {
        (Some(handle), Some(context)) => {
            handle.enter_phase(context, phase, operation, pending_async_task_count)
        }
        _ => 0,
    }
}

fn exit_diagnostic_phase(
    diagnostics: Option<&ProviderDiagnosticsHandle>,
    context: Option<&ProviderDiagnosticCallContext>,
    phase: ProviderDiagnosticPhase,
    operation: ProviderExecutionOperation,
    span_id: u64,
    pending_async_task_count: usize,
    output_record_count: usize,
) {
    if let (Some(handle), Some(context)) = (diagnostics, context) {
        handle.exit_phase(
            context,
            phase,
            operation,
            span_id,
            pending_async_task_count,
            output_record_count,
        );
    }
}

fn finish_diagnostic_call(
    diagnostics: Option<&ProviderDiagnosticsHandle>,
    context: Option<&ProviderDiagnosticCallContext>,
    receipt: &SettledProviderReceipt,
    kind: ProviderDiagnosticEventKind,
    phase: ProviderDiagnosticPhase,
    operation: ProviderExecutionOperation,
) {
    let (pending_async_task_count, output_record_count, detail) = match receipt {
        SettledProviderReceipt::Ready { settlement, .. } => (
            settlement.pending_async_task_count,
            settlement.output_record_count,
            None,
        ),
        SettledProviderReceipt::Failed(failure) => (
            failure.pending_async_task_count,
            failure.output_record_count,
            failure.js_error_message.clone(),
        ),
    };
    if let (Some(handle), Some(context)) = (diagnostics, context) {
        handle.finish_call(
            context,
            kind,
            phase,
            operation,
            pending_async_task_count,
            output_record_count,
            detail,
        );
    }
}

pub struct BunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    output: CapturedOutput,
    output_handler: Option<OutputHandler>,
    output_policies: OutputPolicies,
    diagnostics: ProviderDiagnosticsHandle,
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
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
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
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: Some(Box::new(output_handler)),
            output_policies,
            diagnostics,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn initialize_with_diagnostics(
        config: BunRuntimeConfig,
        diagnostic_sink: impl ProviderDiagnosticSink,
    ) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let diagnostics = ProviderDiagnosticsHandle::new_with_sink(diagnostic_sink);
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn from_runtime(config: BunRuntimeConfig, runtime: R) -> Self {
        let output_policies = OutputPolicies::from_config(&config);
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
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
        let result = call_provider_until_settled_observed(
            &mut self.runtime,
            request,
            options,
            Some(&self.diagnostics),
        );
        if let Ok(receipt) = &result {
            self.collect_returned_output(receipt.output());
        }
        self.collect_output();
        result
    }

    pub fn diagnostics_handle(&self) -> ProviderDiagnosticsHandle {
        self.diagnostics.clone()
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
        self.diagnostics.add_captured_output(1);
    }
}

pub struct LowLevelBunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    output: CapturedOutput,
    output_handler: Option<OutputHandler>,
    output_policies: OutputPolicies,
    diagnostics: ProviderDiagnosticsHandle,
    shutdown: bool,
}

impl<R: BunEmbeddingRuntime> LowLevelBunHost<R> {
    pub fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
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
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: Some(Box::new(output_handler)),
            output_policies,
            diagnostics,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn initialize_with_diagnostics(
        config: BunRuntimeConfig,
        diagnostic_sink: impl ProviderDiagnosticSink,
    ) -> LibbunResult<Self> {
        let output_policies = OutputPolicies::from_config(&config);
        let runtime = R::initialize(config)?;
        let diagnostics = ProviderDiagnosticsHandle::new_with_sink(diagnostic_sink);
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn from_runtime(config: BunRuntimeConfig, runtime: R) -> Self {
        let output_policies = OutputPolicies::from_config(&config);
        let diagnostics = ProviderDiagnosticsHandle::new();
        diagnostics.set_metadata(runtime.diagnostic_metadata());
        diagnostics.set_runtime_state(ProviderRuntimeState::Ready);
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            output_policies,
            diagnostics,
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
        let result = call_provider_until_settled_observed(
            &mut self.runtime,
            request,
            options,
            Some(&self.diagnostics),
        );
        if let Ok(receipt) = &result {
            self.collect_returned_output(receipt.output());
        }
        self.collect_output();
        result
    }

    pub fn diagnostics_handle(&self) -> ProviderDiagnosticsHandle {
        self.diagnostics.clone()
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
        self.diagnostics.add_captured_output(1);
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
