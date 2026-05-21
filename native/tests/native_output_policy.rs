use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, ProviderCallResult, ProviderContractIdentity,
    ProviderDeadline, ProviderDomainClass, ProviderRequest, ProviderSettleOptions, SettledProviderReceipt,
    SinkPolicy, StructuralValue,
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
    config.log = SinkPolicy::Drop;

    let mut host = BunHost::<NativeBunRuntime>::initialize(config).expect("host initializes");
    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "@test/native-output-policy".to_string(),
                    capability: "test/output-policy".to_string(),
                    contract_fingerprint: "native-output-policy-test".to_string(),
                },
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::Source {
                    module_id: "output-policy".to_string(),
                    source: r#"
                        export function run(input) {
                            console.log("dropped stdout");
                            console.error("dropped stderr");
                            return input;
                        }
                    "#
                    .to_string(),
                },
                export: "run".to_string(),
                input: StructuralValue(json!({ "ok": true })),
            },
            ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000)),
        )
        .expect("export call succeeds");
    let SettledProviderReceipt::Ready { result, .. } = receipt else {
        panic!("expected output policy success");
    };

    assert_eq!(
        result,
        ProviderCallResult::Ok(StructuralValue(json!({ "ok": true })))
    );
    assert!(host.captured_output().is_empty());
}
