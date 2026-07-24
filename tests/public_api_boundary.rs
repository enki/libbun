use std::path::PathBuf;
use std::process::Command;

fn fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/prepared_export_privacy/Cargo.toml")
}

fn assert_compile_fails(bin: &str, expected: &[&str]) {
    let target = tempfile::tempdir().expect("privacy target directory");
    let output = Command::new(env!("CARGO"))
        .args([
            "check",
            "--locked",
            "--offline",
            "--manifest-path",
            fixture_manifest().to_str().expect("fixture path is UTF-8"),
            "--bin",
            bin,
        ])
        .env("CARGO_TARGET_DIR", target.path())
        .output()
        .expect("privacy fixture cargo check launches");
    assert!(
        !output.status.success(),
        "privacy fixture `{bin}` unexpectedly compiled"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        expected.iter().any(|term| stderr.contains(term)),
        "privacy fixture `{bin}` failed for an unexpected reason:\n{stderr}"
    );
}

#[test]
fn selected_work_cannot_be_constructed_or_cloned_by_a_sibling_crate() {
    assert_compile_fails(
        "construct_selected_work",
        &["private field", "cannot construct"],
    );
    assert_compile_fails(
        "clone_selected_work",
        &["mismatched types", "does not implement `Clone`"],
    );
}

#[test]
fn prepared_export_cannot_be_cloned_and_old_lease_family_is_absent() {
    assert_compile_fails(
        "clone_prepared_export",
        &["mismatched types", "does not implement `Clone`"],
    );
    assert_compile_fails(
        "import_old_lease_family",
        &["unresolved imports", "no `ProviderInvocationLease`"],
    );
}
