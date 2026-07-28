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

pub const LIBBUN_HELPER_PROTOCOL_VERSION: u32 = 2;
pub const LIBBUN_RUNTIME_NATIVE_PATH_ENV: &str = "LIBBUN_RUNTIME_NATIVE_PATH";
pub const LIBBUN_HELPER_RESPONSE_FD_ENV: &str = "LIBBUN_HELPER_RESPONSE_FD";

const MAX_HELPER_FRAME_BYTES: usize = 64 * 1024 * 1024;

#[cfg(target_os = "linux")]
pub fn take_response_writer() -> io::Result<std::fs::File> {
    let raw = std::env::var(LIBBUN_HELPER_RESPONSE_FD_ENV).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("{LIBBUN_HELPER_RESPONSE_FD_ENV} is absent"),
        )
    })?;
    // SAFETY: helper startup is single-threaded, and removing the transport capability before
    // provider initialization prevents authored code from discovering it through the environment.
    unsafe { std::env::remove_var(LIBBUN_HELPER_RESPONSE_FD_ENV) };
    let fd = raw.parse::<i32>().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{LIBBUN_HELPER_RESPONSE_FD_ENV} is not a descriptor: {error}"),
        )
    })?;
    seal_response_writer(fd)
}

#[cfg(target_os = "linux")]
fn seal_response_writer(fd: i32) -> io::Result<std::fs::File> {
    use std::os::fd::FromRawFd;

    if fd <= 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{LIBBUN_HELPER_RESPONSE_FD_ENV} must identify a private descriptor"),
        ));
    }
    // The launcher cleared CLOEXEC only for Bubblewrap transit. Restore it before any authored
    // provider can spawn descendants, so no descendant can retain or write the control endpoint.
    if unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) } == -1 {
        return Err(io::Error::other(format!(
            "failed to seal {LIBBUN_HELPER_RESPONSE_FD_ENV} against descendant inheritance: {}",
            io::Error::last_os_error()
        )));
    }
    // SAFETY: the launcher transfers exclusive ownership of this inherited descriptor to the
    // helper. It is validated above and converted exactly once.
    Ok(unsafe { std::fs::File::from_raw_fd(fd) })
}

#[cfg(not(target_os = "linux"))]
pub fn take_response_writer() -> io::Result<std::fs::File> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "the contained helper response channel is available only on Linux",
    ))
}

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
    if len > MAX_HELPER_FRAME_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "helper protocol announced a {len}-byte frame, exceeding the {MAX_HELPER_FRAME_BYTES}-byte limit"
            ),
        ));
    }
    let mut bytes = vec![0_u8; len];
    reader.read_exact(&mut bytes)?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(invalid_data)
}

fn invalid_data(error: impl std::error::Error + Send + Sync + 'static) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::os::fd::{AsRawFd, IntoRawFd};

    use super::*;

    #[test]
    fn response_writer_seals_control_descriptor_against_descendants() {
        let (reader, writer) = io::pipe().expect("response pipe is allocated");
        let fd = writer.as_raw_fd();
        // SAFETY: this test owns the descriptor and models the launcher's required inheritance
        // state before transferring it once to seal_response_writer.
        assert_ne!(unsafe { libc::fcntl(fd, libc::F_SETFD, 0) }, -1);
        let fd = writer.into_raw_fd();
        let writer = seal_response_writer(fd).expect("response descriptor is admitted and sealed");
        let flags = unsafe { libc::fcntl(writer.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(flags, -1, "response descriptor flags remain readable");
        assert_ne!(
            flags & libc::FD_CLOEXEC,
            0,
            "provider descendants cannot inherit the response descriptor"
        );
        drop(writer);
        drop(reader);
    }
}
