use std::collections::BTreeMap;

use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, PreparedBundleModuleV1, PreparedBundleV1,
    ProviderCallResult, ProviderContractIdentity, ProviderDeadline, ProviderDomainClass,
    ProviderRequest, ProviderSettleOptions, SettledProviderReceipt, StructuralValue,
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
    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "@test/native-prepared".to_string(),
                    capability: "test/prepared".to_string(),
                    contract_fingerprint: "native-prepared-test".to_string(),
                },
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::PreparedBundle {
                    bundle_id: "native-prepared".to_string(),
                    bytes: bundle.to_bytes().expect("bundle serializes"),
                },
                export: "bundle".to_string(),
                input: StructuralValue(json!({ "from": "prepared" })),
            },
            ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000)),
        )
        .expect("export call succeeds");
    let SettledProviderReceipt::Ready { result, .. } = receipt else {
        panic!("expected prepared bundle success");
    };

    assert_eq!(
        result,
        ProviderCallResult::Ok(StructuralValue(json!({
            "value": 7,
            "input": { "from": "prepared" }
        })))
    );
}
