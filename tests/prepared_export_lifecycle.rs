use libbun::{
    BackendShutdownTerminal, BunProviderBackend, DriveControl, DriveInterrupt, MechanicalTerminal,
    PreparedExport, ShutdownControl,
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

#[test]
fn retained_owner_surface_is_affine_and_mechanically_closed() {
    let _ = open_exact_contained_backend;
    let _ = consume_one_prepared_export;
    let _ = consume_backend_shutdown;
    let _ = request_typed_interrupt;
}
