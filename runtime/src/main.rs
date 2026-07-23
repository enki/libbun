use std::io;

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
                libbun_prepared_export_wire::WorkerFaultKind::Internal,
                &format!("private drive request admission failed: {error}"),
            );
        }
    };

    match libbun_native::drive_prepared_export(request) {
        Ok(cargo) => {
            libbun_prepared_export_wire::write_cargo(&mut io::stdout().lock(), &cargo)?;
        }
        Err(failure) => {
            libbun_prepared_export_wire::write_fault(
                &mut io::stdout().lock(),
                failure.kind(),
                failure.diagnostic(),
            )?;
        }
    }
    Ok(())
}
