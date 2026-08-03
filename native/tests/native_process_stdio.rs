use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, OutputStream, ProviderContractIdentity,
    ProviderDeadline, ProviderDomainClass, ProviderRequest, ProviderSettleOptions,
    SettledProviderReceipt, StructuralValue,
};
use libbun_native::NativeBunRuntime;
use serde_json::json;

#[test]
fn raw_process_stdio_is_attached_to_the_exact_provider_receipt() {
    let working_directory = tempfile::tempdir().expect("working directory creates");
    let config = BunRuntimeConfig::new("native-process-stdio-test", working_directory.path())
        .with_process_stdio_capture();
    let mut host = BunHost::<NativeBunRuntime>::initialize(config).expect("host initializes");
    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "@test/native-process-output".to_owned(),
                    capability: "test/native-process-output".to_owned(),
                    contract_fingerprint: "native-process-output-test".to_owned(),
                },
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::Source {
                    module_id: "native-process-output".to_owned(),
                    source: r#"
                        export function emit() {
                            process.stdout.write("raw provider stdout\n");
                            process.stderr.write("raw provider stderr\n");
                            return { kind: "ok" };
                        }
                    "#
                    .to_owned(),
                },
                export: "emit".to_owned(),
                input: StructuralValue(json!({})),
            },
            ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000)),
        )
        .expect("provider call settles");
    host.shutdown().expect("host restores process stdio");

    let SettledProviderReceipt::Ready { output, .. } = receipt else {
        panic!("provider call must settle ready");
    };
    assert!(output.iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text == "raw provider stdout\n"
    }));
    assert!(output.iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text == "raw provider stderr\n"
    }));
}
