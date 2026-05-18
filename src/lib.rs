//! Hostable Bun embedding facade.
//!
//! This crate owns the stable Rust boundary for hosting JavaScript and
//! TypeScript providers through Bun. It deliberately does not call Bun CLI
//! entrypoints and does not expose raw JSC handles across its public API.

use std::collections::BTreeMap;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

pub type LibbunResult<T> = Result<T, LibbunError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BunRuntimeConfig {
    pub host_id: String,
    pub bun_revision: String,
    pub working_directory: PathBuf,
    #[serde(default)]
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
    pub module: BunModuleHandle,
    pub export: String,
    pub input: StructuralValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProviderHostReceipt {
    Ready(ProviderReady),
    Parked(ProviderParked),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderReady {
    pub contract: ProviderContractIdentity,
    pub artifact: BunArtifactFingerprint,
    pub result: ProviderCallResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderParked {
    pub contract: ProviderContractIdentity,
    pub artifact: BunArtifactFingerprint,
    pub handle: BunAsyncHandle,
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

    fn captured_output(&self) -> &[OutputRecord];

    fn drain_captured_output(&mut self) -> Vec<OutputRecord>;

    fn shutdown(&mut self) -> LibbunResult<()>;
}

pub struct BunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    output: CapturedOutput,
    output_handler: Option<OutputHandler>,
    shutdown: bool,
}

impl<R: BunEmbeddingRuntime> BunHost<R> {
    pub fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: None,
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
    }

    pub fn initialize_with_output_handler(
        config: BunRuntimeConfig,
        output_handler: impl FnMut(OutputRecord) + Send + 'static,
    ) -> LibbunResult<Self> {
        let runtime = R::initialize(config)?;
        let mut host = Self {
            runtime,
            output: CapturedOutput::default(),
            output_handler: Some(Box::new(output_handler)),
            shutdown: false,
        };
        host.collect_output();
        Ok(host)
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

    pub fn call_provider(&mut self, request: ProviderRequest) -> LibbunResult<ProviderHostReceipt> {
        self.ensure_live()?;
        if request.domain == ProviderDomainClass::RustSubstrateAuthority {
            return Ok(ProviderHostReceipt::Ready(ProviderReady {
                contract: request.contract,
                artifact: artifact_fingerprint(),
                result: ProviderCallResult::Err(ProviderError {
                    code: "rust_substrate_authority_rejected".to_string(),
                    message: "libbun cannot execute Rust-substrate provider exports".to_string(),
                }),
            }));
        }

        let result = self
            .runtime
            .call_export(&request.module, &request.export, request.input);
        self.collect_output();

        match result? {
            ExportCallResult::Ready(result) => Ok(ProviderHostReceipt::Ready(ProviderReady {
                contract: request.contract,
                artifact: artifact_fingerprint(),
                result,
            })),
            ExportCallResult::Pending(handle) => Ok(ProviderHostReceipt::Parked(ProviderParked {
                contract: request.contract,
                artifact: artifact_fingerprint(),
                handle,
            })),
        }
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
            if let Some(handler) = self.output_handler.as_mut() {
                handler(record.clone());
            }
            self.output.push(record);
        }
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
