use std::io;
use std::io::Read;
use std::io::Write;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::BunAsyncHandle;
use crate::BunModuleHandle;
use crate::BunModuleSpec;
use crate::BunRuntimeConfig;
use crate::ExportCallResult;
use crate::OutputRecord;
use crate::ProviderCallResult;
use crate::ProviderRequest;
use crate::ProviderSettleOptions;
use crate::PumpBudget;
use crate::PumpOutcome;
use crate::SettledProviderReceipt;
use crate::StructuralValue;
use crate::plugin_abi::LIBBUN_PLUGIN_ABI_VERSION;

pub const LIBBUN_HELPER_PROTOCOL_VERSION: u32 = 1;
pub const LIBBUN_RUNTIME_NATIVE_PATH_ENV: &str = "LIBBUN_RUNTIME_NATIVE_PATH";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelperHello {
    pub plugin_abi_version: u32,
    pub helper_protocol_version: u32,
    pub target: String,
    pub libbun_version: String,
    pub bun_revision: String,
    pub helper_sha256: Option<String>,
}

impl HelperHello {
    pub fn current(target: impl Into<String>) -> Self {
        Self {
            plugin_abi_version: LIBBUN_PLUGIN_ABI_VERSION,
            helper_protocol_version: LIBBUN_HELPER_PROTOCOL_VERSION,
            target: target.into(),
            libbun_version: env!("CARGO_PKG_VERSION").to_string(),
            bun_revision: env!("LIBBUN_BUN_SOURCE_COMMIT").to_string(),
            helper_sha256: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelperRequest {
    pub id: u64,
    pub payload: HelperRequestPayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum HelperRequestPayload {
    Hello(HelperHello),
    Create {
        config: BunRuntimeConfig,
    },
    LoadModule {
        spec: BunModuleSpec,
    },
    CallExport {
        module: BunModuleHandle,
        export: String,
        input: StructuralValue,
    },
    PumpEventLoop {
        budget: PumpBudget,
    },
    ResolveAsync {
        handle: BunAsyncHandle,
    },
    CallProviderUntilSettled {
        request: ProviderRequest,
        options: ProviderSettleOptions,
    },
    DrainOutput,
    Shutdown,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelperResponse {
    pub id: u64,
    pub result: Result<HelperResponsePayload, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum HelperResponsePayload {
    Hello(HelperHello),
    Unit,
    Module(BunModuleHandle),
    Export(ExportCallResult),
    Pump(PumpOutcome),
    Resolve(Option<ProviderCallResult>),
    SettledProvider(SettledProviderReceipt),
    Output(Vec<OutputRecord>),
}

pub fn write_frame<W, T>(writer: &mut W, value: &T) -> io::Result<()>
where
    W: Write,
    T: Serialize,
{
    let bytes = serde_json::to_vec(value).map_err(invalid_data)?;
    let len = u32::try_from(bytes.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "helper protocol frame exceeds u32 length",
        )
    })?;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&bytes)?;
    writer.flush()
}

pub fn read_frame<R, T>(reader: &mut R) -> io::Result<Option<T>>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut len = [0_u8; 4];
    match reader.read_exact(&mut len) {
        Ok(()) => {}
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(err) => return Err(err),
    }

    let len = u32::from_be_bytes(len) as usize;
    let mut bytes = vec![0_u8; len];
    reader.read_exact(&mut bytes)?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(invalid_data)
}

fn invalid_data(error: impl std::error::Error + Send + Sync + 'static) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}
