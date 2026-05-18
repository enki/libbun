use libbun::{
    BunHost, BunModuleHandle, BunModuleSpec, BunRuntimeConfig, ProviderCallResult,
    ProviderContractIdentity, ProviderDomainClass, ProviderHostReceipt, ProviderRequest,
    PumpBudget, StructuralValue, OutputStream,
};
use libbun_native::NativeBunRuntime;
use serde_json::json;

fn host() -> BunHost<NativeBunRuntime> {
    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let path = tempdir.keep();
    BunHost::initialize(BunRuntimeConfig::new("native-test-host", path)).expect("host initializes")
}

fn contract() -> ProviderContractIdentity {
    ProviderContractIdentity {
        package: "@test/native-provider".to_string(),
        capability: "test/native".to_string(),
        contract_fingerprint: "native-test".to_string(),
    }
}

fn assert_sync_export(host: &mut BunHost<NativeBunRuntime>, module: &BunModuleHandle) {
    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::JavaScriptExternalTransport,
            module: module.clone(),
            export: "sync".to_string(),
            input: StructuralValue(json!({ "value": 42 })),
        })
        .expect("provider call succeeds");

    match receipt {
        ProviderHostReceipt::Ready(ready) => assert_eq!(
            ready.result,
            ProviderCallResult::Ok(StructuralValue(json!({
                "ok": true,
                "input": { "value": 42 }
            })))
        ),
        ProviderHostReceipt::Parked(_) => panic!("expected ready receipt"),
    }

    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text.contains("native stdout 42")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text.contains("native stderr")
    }));
}

fn assert_async_export(host: &mut BunHost<NativeBunRuntime>, module: &BunModuleHandle) {
    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module: module.clone(),
            export: "asyncExport".to_string(),
            input: StructuralValue(json!({ "async": true })),
        })
        .expect("provider call parks");

    let handle = match receipt {
        ProviderHostReceipt::Parked(parked) => parked.handle,
        ProviderHostReceipt::Ready(_) => panic!("expected parked async receipt"),
    };

    for _ in 0..8 {
        if let Some(result) = host.resolve_async(&handle).expect("async poll succeeds") {
            assert_eq!(
                result,
                ProviderCallResult::Ok(StructuralValue(json!({ "async": true })))
            );
            return;
        }
        host.pump_event_loop(PumpBudget { max_ticks: 1 })
            .expect("event loop pumps");
    }

    panic!("async export did not resolve");
}

fn assert_structured_provider_error(host: &mut BunHost<NativeBunRuntime>, module: &BunModuleHandle) {
    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module: module.clone(),
            export: "throws".to_string(),
            input: StructuralValue::null(),
        })
        .expect("provider throw is structural");

    match receipt {
        ProviderHostReceipt::Ready(ready) => match ready.result {
            ProviderCallResult::Err(error) => {
                assert_eq!(error.code, "provider_rejected");
                assert!(error.message.contains("provider boom"));
            }
            ProviderCallResult::Ok(_) => panic!("expected provider error"),
        },
        ProviderHostReceipt::Parked(_) => panic!("expected ready error receipt"),
    }
}

#[test]
fn native_runtime_provider_flows() {
    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "flows".to_string(),
            source: r#"
                export function sync(input) {
                    console.log("native stdout", input.value);
                    console.error("native stderr");
                    return { ok: true, input };
                }

                export async function asyncExport(input) {
                    await Promise.resolve();
                    return { async: input.async };
                }

                export function throws() {
                    throw new Error("provider boom");
                }
            "#
            .to_string(),
        })
        .expect("module loads");

    assert_sync_export(&mut host, &module);
    assert_async_export(&mut host, &module);
    assert_structured_provider_error(&mut host, &module);
}
