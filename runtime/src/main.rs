use std::io;

use libbun::helper_protocol::HelperHello;
use libbun::helper_protocol::HelperRequest;
use libbun::helper_protocol::HelperRequestPayload;
use libbun::helper_protocol::HelperResponse;
use libbun::helper_protocol::HelperResponsePayload;
use libbun::helper_protocol::LIBBUN_HELPER_PROTOCOL_VERSION;
use libbun::helper_protocol::read_frame;
use libbun::helper_protocol::write_frame;
use libbun::plugin_abi::LIBBUN_PLUGIN_ABI_VERSION;
use libbun::{BunHost, LibbunError};
use libbun_native::NativeBunRuntime;

#[cfg(target_os = "linux")]
use bun_platform as _;

fn main() {
    if let Err(err) = run() {
        eprintln!("libbun-runtime-native failed: {err}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    let mut state = HelperState::default();

    while let Some(request) = read_frame::<_, HelperRequest>(&mut reader)? {
        let id = request.id;
        let exit = matches!(request.payload, HelperRequestPayload::Exit);
        let result = state.handle(request.payload).map_err(|err| err.to_string());
        write_frame(&mut writer, &HelperResponse { id, result })?;
        if exit {
            break;
        }
    }
    Ok(())
}

#[derive(Default)]
struct HelperState {
    host: Option<BunHost<NativeBunRuntime>>,
}

impl HelperState {
    fn handle(
        &mut self,
        payload: HelperRequestPayload,
    ) -> libbun::LibbunResult<HelperResponsePayload> {
        match payload {
            HelperRequestPayload::Hello(hello) => self.hello(hello),
            HelperRequestPayload::Create { config } => {
                self.host = Some(BunHost::<NativeBunRuntime>::initialize(config)?);
                Ok(HelperResponsePayload::Unit)
            }
            HelperRequestPayload::LoadModule { spec } => self
                .host_mut()?
                .load_module(spec)
                .map(HelperResponsePayload::Module),
            HelperRequestPayload::CallExport {
                module,
                export,
                input,
            } => self
                .host_mut()?
                .call_export(&module, &export, input)
                .map(HelperResponsePayload::Export),
            HelperRequestPayload::PumpEventLoop { budget } => self
                .host_mut()?
                .pump_event_loop(budget)
                .map(HelperResponsePayload::Pump),
            HelperRequestPayload::ResolveAsync { handle } => self
                .host_mut()?
                .resolve_async(&handle)
                .map(HelperResponsePayload::Resolve),
            HelperRequestPayload::DrainOutput => {
                let records = self
                    .host
                    .as_mut()
                    .map(BunHost::drain_captured_output)
                    .unwrap_or_default();
                Ok(HelperResponsePayload::Output(records))
            }
            HelperRequestPayload::Shutdown => {
                if let Some(host) = self.host.as_mut() {
                    host.shutdown()?;
                }
                Ok(HelperResponsePayload::Unit)
            }
            HelperRequestPayload::Exit => {
                if let Some(mut host) = self.host.take() {
                    host.shutdown()?;
                }
                Ok(HelperResponsePayload::Unit)
            }
        }
    }

    fn hello(&self, hello: HelperHello) -> libbun::LibbunResult<HelperResponsePayload> {
        if hello.plugin_abi_version != LIBBUN_PLUGIN_ABI_VERSION {
            return Err(LibbunError::initialize(format!(
                "plugin ABI {} is incompatible with helper ABI {}",
                hello.plugin_abi_version, LIBBUN_PLUGIN_ABI_VERSION
            )));
        }
        if hello.helper_protocol_version != LIBBUN_HELPER_PROTOCOL_VERSION {
            return Err(LibbunError::initialize(format!(
                "plugin helper protocol {} is incompatible with helper protocol {}",
                hello.helper_protocol_version, LIBBUN_HELPER_PROTOCOL_VERSION
            )));
        }
        Ok(HelperResponsePayload::Hello(HelperHello::current(
            std::env::consts::ARCH,
        )))
    }

    fn host_mut(&mut self) -> libbun::LibbunResult<&mut BunHost<NativeBunRuntime>> {
        self.host.as_mut().ok_or_else(|| {
            LibbunError::initialize("Linux native helper runtime has not been created")
        })
    }
}
