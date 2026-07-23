use std::io;

use libbun::BunEmbeddingRuntime;
use libbun::BunModuleSpec;
use libbun::BunRuntimeConfig;
use libbun::ExportCallResult;
use libbun::PreparedBundleV1;
use libbun::ProviderCallResult;
use libbun::PumpBudget;
use libbun::SinkPolicy;
use libbun::StructuralValue;
use libbun_native::NativeBunRuntime;
use libbun_prepared_export_wire::DriveRequest;
use libbun_prepared_export_wire::WorkerFaultKind;

#[cfg(target_os = "linux")]
use bun_platform as _;

fn main() {
    if let Err(error) = run_one_drive() {
        eprintln!("libbun one-shot worker failed: {error}");
        std::process::exit(1);
    }
}

fn run_one_drive() -> io::Result<()> {
    let request = match libbun_prepared_export_wire::read_drive_request(&mut io::stdin().lock()) {
        Ok(request) => request,
        Err(error) => {
            return libbun_prepared_export_wire::write_fault(
                &mut io::stdout().lock(),
                WorkerFaultKind::Internal,
                &format!("private drive request admission failed: {error}"),
            );
        }
    };

    match drive(request) {
        Ok(cargo) => {
            libbun_prepared_export_wire::write_cargo(&mut io::stdout().lock(), &cargo)?;
        }
        Err(failure) => {
            libbun_prepared_export_wire::write_fault(
                &mut io::stdout().lock(),
                failure.kind,
                &failure.diagnostic,
            )?;
        }
    }
    Ok(())
}

fn drive(request: DriveRequest) -> Result<Vec<u8>, WorkerFailure> {
    let input: serde_json::Value =
        serde_json::from_slice(&request.opaque_invocation).map_err(|error| {
            WorkerFailure::new(
                WorkerFaultKind::InputLowering,
                format!("opaque invocation lowering failed: {error}"),
            )
        })?;
    let bundle = PreparedBundleV1::from_bytes(&request.prepared_artifact).map_err(|error| {
        WorkerFailure::new(
            WorkerFaultKind::Preparation,
            format!("prepared artifact admission failed: {error}"),
        )
    })?;
    let bundle_id = bundle.bundle_id.clone();

    let working_directory = std::env::current_dir().map_err(|error| {
        WorkerFailure::new(
            WorkerFaultKind::Preparation,
            format!("worker current-directory admission failed: {error}"),
        )
    })?;
    let mut config = BunRuntimeConfig::new("libbun-one-shot-worker", working_directory);
    config.stdout = SinkPolicy::Drop;
    config.stderr = SinkPolicy::Drop;
    config.log = SinkPolicy::Drop;
    let mut runtime = NativeBunRuntime::initialize(config).map_err(|error| {
        WorkerFailure::new(
            WorkerFaultKind::Preparation,
            format!("fresh Bun VM admission failed: {error}"),
        )
    })?;
    let module = runtime
        .load_module(BunModuleSpec::PreparedBundle {
            bundle_id,
            bytes: request.prepared_artifact,
        })
        .map_err(|error| {
            WorkerFailure::new(
                WorkerFaultKind::Preparation,
                format!("prepared module load failed: {error}"),
            )
        })?;
    let result = runtime
        .call_export(&module, &request.selected_export, StructuralValue(input))
        .map_err(|error| {
            WorkerFailure::new(
                WorkerFaultKind::Preparation,
                format!("prepared export call admission failed: {error}"),
            )
        })?;

    settle_result(&mut runtime, result)
}

fn settle_result(
    runtime: &mut NativeBunRuntime,
    mut result: ExportCallResult,
) -> Result<Vec<u8>, WorkerFailure> {
    loop {
        match result {
            ExportCallResult::Ready(ProviderCallResult::Ok(value)) => {
                return serde_json::to_vec(&value.0).map_err(|error| {
                    WorkerFailure::new(
                        WorkerFaultKind::CargoExtraction,
                        format!("opaque cargo extraction failed: {error}"),
                    )
                });
            }
            ExportCallResult::Ready(ProviderCallResult::Err(error)) => {
                return Err(WorkerFailure::new(
                    WorkerFaultKind::JavaScriptRejection,
                    error.message,
                ));
            }
            ExportCallResult::Pending(handle) => {
                runtime
                    .pump_event_loop(PumpBudget { max_ticks: 256 })
                    .map_err(|error| {
                        WorkerFailure::new(
                            WorkerFaultKind::CargoExtraction,
                            format!("private event-loop drive failed: {error}"),
                        )
                    })?;
                if let Some(settled) = runtime.resolve_async(&handle).map_err(|error| {
                    WorkerFailure::new(
                        WorkerFaultKind::CargoExtraction,
                        format!("private promise observation failed: {error}"),
                    )
                })? {
                    result = ExportCallResult::Ready(settled);
                } else {
                    result = ExportCallResult::Pending(handle);
                }
            }
        }
    }
}

struct WorkerFailure {
    kind: WorkerFaultKind,
    diagnostic: String,
}

impl WorkerFailure {
    fn new(kind: WorkerFaultKind, diagnostic: impl Into<String>) -> Self {
        let mut diagnostic = diagnostic.into();
        if diagnostic.len() > 4096 {
            let mut boundary = 4096;
            while !diagnostic.is_char_boundary(boundary) {
                boundary -= 1;
            }
            diagnostic.truncate(boundary);
        }
        Self { kind, diagnostic }
    }
}
