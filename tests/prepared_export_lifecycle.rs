use libbun::{
    BackendShutdownTerminal, BunProviderBackend, DriveControl, DriveInterrupt, MechanicalTerminal,
    PreparedExport, ProviderRequest, ProviderSettleOptions, ShutdownControl,
};

fn open_exact_contained_backend(
    config: libbun::BunRuntimeConfig,
) -> libbun::LibbunResult<BunProviderBackend> {
    BunProviderBackend::open(config)
}

fn consume_one_prepared_export(
    prepared: PreparedExport,
    control: DriveControl,
) -> MechanicalTerminal {
    prepared.drive(control)
}

fn consume_backend_shutdown(
    backend: BunProviderBackend,
    control: ShutdownControl,
) -> BackendShutdownTerminal {
    backend.shutdown(control)
}

fn request_typed_interrupt(interrupt: &DriveInterrupt) {
    interrupt.request();
}

fn consume_selected_request_into_prepared_export(
    backend: BunProviderBackend,
    request: ProviderRequest,
    options: ProviderSettleOptions,
) -> PreparedExport {
    backend.prepare_selected_request(request, options)
}

fn consume_ready_terminal_into_next_prepared_export(
    terminal: MechanicalTerminal,
    request: ProviderRequest,
    options: ProviderSettleOptions,
) -> Result<PreparedExport, MechanicalTerminal> {
    terminal.prepare_next_selected_request(request, options)
}

#[test]
fn retained_owner_surface_is_affine_and_mechanically_closed() {
    let _ = open_exact_contained_backend;
    let _ = consume_one_prepared_export;
    let _ = consume_backend_shutdown;
    let _ = request_typed_interrupt;
    let _ = consume_selected_request_into_prepared_export;
    let _ = consume_ready_terminal_into_next_prepared_export;
}
