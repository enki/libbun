#![cfg(feature = "dynamic-loading")]

use libbun::dynamic::DynamicBunRuntime;
use libbun::{
    BunEmbeddingRuntime, BunModuleSpec, BunRuntimeConfig, ExportCallResult, ProviderCallResult,
    StructuralValue,
};
use serde_json::json;

#[test]
fn dynamic_plugin_provider_flow() {
    let Some(plugin_path) = std::env::var_os("LIBBUN_PLUGIN_PATH") else {
        eprintln!("skipping dynamic plugin flow; LIBBUN_PLUGIN_PATH is not set");
        return;
    };

    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let mut runtime = DynamicBunRuntime::load(
        plugin_path,
        BunRuntimeConfig::new("dynamic-plugin-test-host", tempdir.path()),
    )
    .expect("dynamic plugin runtime loads");

    let module = runtime
        .load_module(BunModuleSpec::Source {
            module_id: "dynamic-flow".to_string(),
            source: r#"
                export function run(input) {
                    console.log("dynamic stdout", input.value);
                    return { ok: true, input };
                }
            "#
            .to_string(),
        })
        .expect("module loads through dynamic plugin");

    let result = runtime
        .call_export(&module, "run", StructuralValue(json!({ "value": 7 })))
        .expect("export call succeeds through dynamic plugin");

    assert_eq!(
        result,
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(json!({
            "ok": true,
            "input": { "value": 7 }
        }))))
    );
    assert!(
        runtime
            .captured_output()
            .iter()
            .any(|record| record.text.contains("dynamic stdout 7"))
    );
}
