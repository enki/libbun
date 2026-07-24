use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

fn check_fixture(bin: &str) -> Output {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = repository.join("tests/fixtures/public_api_boundary/Cargo.toml");
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("target"))
        .join("external-public-api-boundary");

    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args([
            "check",
            "--locked",
            "--offline",
            "--color",
            "never",
            "--manifest-path",
        ])
        .arg(manifest)
        .args(["--bin", bin])
        .env("CARGO_TARGET_DIR", target)
        .output()
        .expect("external public-API fixture cargo check must launch")
}

#[test]
fn raw_installer_is_absent_from_external_import_and_call_surfaces() {
    let control = check_fixture("adjacent-public-controls");
    assert!(
        control.status.success(),
        "adjacent public controls must remain available:\n{}",
        String::from_utf8_lossy(&control.stderr)
    );

    for (bin, intended_diagnostic) in [
        ("import-raw-installer", "unresolved import"),
        ("call-raw-installer", "cannot find function"),
    ] {
        let output = check_fixture(bin);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !output.status.success(),
            "{bin} unexpectedly accessed the deleted raw installer"
        );
        assert!(
            stderr.contains("install_prepared_export") && stderr.contains(intended_diagnostic),
            "{bin} failed for an unintended reason:\n{stderr}"
        );
    }
}
