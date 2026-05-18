//! Hostable Bun embedding facade.
//!
//! This crate owns the stable Rust boundary for hosting JavaScript and
//! TypeScript providers through Bun. It deliberately does not call Bun CLI
//! entrypoints and does not expose raw JSC handles across its public API.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

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
}

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

    fn shutdown(&mut self) -> LibbunResult<()>;
}

#[derive(Debug)]
pub struct BunHost<R: BunEmbeddingRuntime> {
    runtime: R,
    shutdown: bool,
}

impl<R: BunEmbeddingRuntime> BunHost<R> {
    pub fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let runtime = R::initialize(config)?;
        Ok(Self {
            runtime,
            shutdown: false,
        })
    }

    pub fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
        self.ensure_live()?;
        self.runtime.load_module(spec)
    }

    pub fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: impl Into<StructuralValue>,
    ) -> LibbunResult<ExportCallResult> {
        self.ensure_live()?;
        self.runtime.call_export(module, export, input.into())
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

        match self
            .runtime
            .call_export(&request.module, &request.export, request.input)?
        {
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
        self.runtime.pump_event_loop(budget)
    }

    pub fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>> {
        self.ensure_live()?;
        self.runtime.resolve_async(handle)
    }

    pub fn captured_output(&self) -> &[OutputRecord] {
        self.runtime.captured_output()
    }

    pub fn shutdown(&mut self) -> LibbunResult<()> {
        if self.shutdown {
            return Ok(());
        }
        self.runtime.shutdown()?;
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
