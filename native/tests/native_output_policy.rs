use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, ExportCallResult, ProviderCallResult, SinkPolicy,
    StructuralValue,
};
use libbun_native::NativeBunRuntime;
use serde_json::json;

#[test]
fn native_runtime_honors_drop_output_policy() {
    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let path = tempdir.keep();
    let mut config = BunRuntimeConfig::new("native-output-policy-test-host", path);
    config.stdout = SinkPolicy::Drop;
    config.stderr = SinkPolicy::Drop;

    let mut host = BunHost::<NativeBunRuntime>::initialize(config).expect("host initializes");
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "output-policy".to_string(),
            source: r#"
                export function run(input) {
                    console.log("dropped stdout");
                    console.error("dropped stderr");
                    return input;
                }
            "#
            .to_string(),
        })
        .expect("module loads");

    let result = host
        .call_export(&module, "run", StructuralValue(json!({ "ok": true })))
        .expect("export call succeeds");

    assert_eq!(
        result,
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(
            json!({ "ok": true })
        )))
    );
    assert!(host.captured_output().is_empty());
}
