use std::collections::BTreeMap;

use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, ExportCallResult, PreparedBundleModuleV1,
    PreparedBundleV1, ProviderCallResult, StructuralValue,
};
use libbun_native::NativeBunRuntime;
use serde_json::json;

fn host() -> BunHost<NativeBunRuntime> {
    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let path = tempdir.keep();
    BunHost::initialize(BunRuntimeConfig::new("native-prepared-test-host", path))
        .expect("host initializes")
}

#[test]
fn native_runtime_loads_prepared_source_bundle() {
    let mut modules = BTreeMap::new();
    modules.insert(
        "entry.mjs".to_string(),
        PreparedBundleModuleV1::source(
            r#"
                import { value } from "./dep/value.mjs";

                export function bundle(input) {
                    return { value, input };
                }
            "#,
        ),
    );
    modules.insert(
        "dep/value.mjs".to_string(),
        PreparedBundleModuleV1::source("export const value = 7;"),
    );
    let bundle = PreparedBundleV1::source_bundle("native-prepared", "entry.mjs", modules)
        .expect("bundle builds");

    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::PreparedBundle {
            bundle_id: "native-prepared".to_string(),
            bytes: bundle.to_bytes().expect("bundle serializes"),
        })
        .expect("prepared bundle loads");

    let result = host
        .call_export(
            &module,
            "bundle",
            StructuralValue(json!({ "from": "prepared" })),
        )
        .expect("export call succeeds");

    assert_eq!(
        result,
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(json!({
            "value": 7,
            "input": { "from": "prepared" }
        }))))
    );
}
