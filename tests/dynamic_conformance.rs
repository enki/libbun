#![cfg(feature = "dynamic-loading")]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use libbun::dynamic::DynamicBunRuntime;
use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, ExportCallResult, LibbunError, OutputRecord,
    OutputStream, PreparedBundleModuleV1, PreparedBundleV1, ProviderCallResult,
    ProviderContractIdentity, ProviderDomainClass, ProviderHostReceipt, ProviderRequest,
    PumpBudget, StructuralValue,
};
use serde_json::json;

const OVERLAY_ENV_KEY: &str = "LIBBUN_DYNAMIC_OVERLAY_TEST";

fn contract() -> ProviderContractIdentity {
    ProviderContractIdentity {
        package: "@test/dynamic-provider".to_string(),
        capability: "test/dynamic".to_string(),
        contract_fingerprint: "dynamic-test".to_string(),
    }
}

#[test]
fn dynamic_plugin_facade_conformance() {
    if std::env::var_os("LIBBUN_PLUGIN_PATH").is_none() {
        eprintln!("skipping dynamic plugin conformance; LIBBUN_PLUGIN_PATH is not set");
        return;
    }
    assert!(
        std::env::var_os(OVERLAY_ENV_KEY).is_none(),
        "test requires {OVERLAY_ENV_KEY} to be unset in the process environment"
    );

    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let records = Arc::new(Mutex::new(Vec::<OutputRecord>::new()));
    let handler_records = Arc::clone(&records);
    let config = BunRuntimeConfig::new("dynamic-conformance-test-host", tempdir.path())
        .with_environment_overlay([(OVERLAY_ENV_KEY, "overlay-value")]);
    let mut host =
        BunHost::<DynamicBunRuntime>::initialize_with_output_handler(config, move |record| {
            handler_records
                .lock()
                .expect("handler records lock")
                .push(record);
        })
        .expect("host initializes");

    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "dynamic-conformance".to_string(),
            source: r#"
                export function sync(input) {
                    console.log("dynamic conformance stdout", input.value);
                    console.error("dynamic conformance stderr");
                    return { ok: true, input };
                }

                export async function asyncExport(input) {
                    await Promise.resolve();
                    return { async: input.async };
                }

                export function throws() {
                    throw new Error("dynamic provider boom");
                }

                export function readEnv() {
                    return {
                        processEnv: process.env.LIBBUN_DYNAMIC_OVERLAY_TEST,
                        bunEnv: Bun.env.LIBBUN_DYNAMIC_OVERLAY_TEST,
                    };
                }

                export function mustNotRun() {
                    console.log("dynamic substrate should not execute");
                    return { executed: true };
                }
            "#
            .to_string(),
        })
        .expect("module loads");

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
        ProviderHostReceipt::Ready(ready) => {
            assert_eq!(
                ready.result,
                ProviderCallResult::Ok(StructuralValue(json!({
                    "ok": true,
                    "input": { "value": 42 }
                })))
            );
            assert_eq!(
                ready.artifact.bun_revision,
                env!("LIBBUN_BUN_SOURCE_COMMIT")
            );
        }
        ProviderHostReceipt::Parked(_) => panic!("expected ready receipt"),
    }

    let async_receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module: module.clone(),
            export: "asyncExport".to_string(),
            input: StructuralValue(json!({ "async": true })),
        })
        .expect("provider call parks");
    let handle = match async_receipt {
        ProviderHostReceipt::Parked(parked) => parked.handle,
        ProviderHostReceipt::Ready(_) => panic!("expected parked async receipt"),
    };
    let mut resolved = false;
    for _ in 0..8 {
        if let Some(result) = host.resolve_async(&handle).expect("async poll succeeds") {
            assert_eq!(
                result,
                ProviderCallResult::Ok(StructuralValue(json!({ "async": true })))
            );
            resolved = true;
            break;
        }
        host.pump_event_loop(PumpBudget { max_ticks: 1 })
            .expect("event loop pumps");
    }
    assert!(resolved, "async export did not resolve");
    host.resolve_async(&handle)
        .expect_err("resolved handle is consumed");

    let error_receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module: module.clone(),
            export: "throws".to_string(),
            input: StructuralValue::null(),
        })
        .expect("provider throw is structural");
    match error_receipt {
        ProviderHostReceipt::Ready(ready) => match ready.result {
            ProviderCallResult::Err(error) => {
                assert_eq!(error.code, "provider_rejected");
                assert!(error.message.contains("dynamic provider boom"));
            }
            ProviderCallResult::Ok(_) => panic!("expected provider error"),
        },
        ProviderHostReceipt::Parked(_) => panic!("expected ready error receipt"),
    }

    let env_result = host
        .call_export(&module, "readEnv", StructuralValue::null())
        .expect("env export succeeds");
    assert_eq!(
        env_result,
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(json!({
            "processEnv": "overlay-value",
            "bunEnv": "overlay-value"
        }))))
    );
    assert!(std::env::var_os(OVERLAY_ENV_KEY).is_none());

    let module_load_error = host
        .load_module(BunModuleSpec::Source {
            module_id: "dynamic-conformance-throwing-import".to_string(),
            source: r#"
                throw new Error("dynamic module import diagnostic boom");
            "#
            .to_string(),
        })
        .expect_err("module import throw should fail module load");
    let module_load_message = module_load_error.to_string();
    assert!(
        module_load_message.contains("module import")
            && module_load_message.contains("specifier")
            && module_load_message.contains("dynamic module import diagnostic boom"),
        "module-load diagnostic must name the import operation, specifier, and JS exception detail; got {module_load_message}"
    );

    let substrate_receipt = host
        .call_provider(ProviderRequest {
            contract: ProviderContractIdentity {
                package: "@proving/agent".to_string(),
                capability: "capability:advanceTurnSource".to_string(),
                contract_fingerprint: "substrate".to_string(),
            },
            domain: ProviderDomainClass::RustSubstrateAuthority,
            module: module.clone(),
            export: "mustNotRun".to_string(),
            input: StructuralValue(json!({ "mustNotRun": true })),
        })
        .expect("substrate rejection is structural");
    match substrate_receipt {
        ProviderHostReceipt::Ready(ready) => match ready.result {
            ProviderCallResult::Err(error) => {
                assert_eq!(error.code, "rust_substrate_authority_rejected");
            }
            ProviderCallResult::Ok(_) => panic!("substrate export should not execute"),
        },
        ProviderHostReceipt::Parked(_) => panic!("substrate export should not park"),
    }
    assert!(
        !host
            .captured_output()
            .iter()
            .any(|record| record.text.contains("dynamic substrate should not execute"))
    );

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
    let bundle = PreparedBundleV1::source_bundle("dynamic-prepared", "entry.mjs", modules)
        .expect("bundle builds");
    let prepared_module = host
        .load_module(BunModuleSpec::PreparedBundle {
            bundle_id: "dynamic-prepared".to_string(),
            bytes: bundle.to_bytes().expect("bundle serializes"),
        })
        .expect("prepared bundle loads");
    assert_eq!(
        host.call_export(
            &prepared_module,
            "bundle",
            StructuralValue(json!({ "from": "prepared" }))
        )
        .expect("prepared export succeeds"),
        ExportCallResult::Ready(ProviderCallResult::Ok(StructuralValue(json!({
            "value": 7,
            "input": { "from": "prepared" }
        }))))
    );

    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stdout
            && record.text.contains("dynamic conformance stdout 42")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text.contains("dynamic conformance stderr")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Log && record.text.contains("loading module module-1")
    }));
    assert!(
        records
            .lock()
            .expect("handler records lock")
            .iter()
            .any(|record| {
                record.stream == OutputStream::Stdout
                    && record.text.contains("dynamic conformance stdout 42")
            })
    );
    assert!(!host.drain_captured_output().is_empty());
    assert!(host.captured_output().is_empty());

    host.shutdown().expect("shutdown succeeds");
    let error = host
        .pump_event_loop(PumpBudget { max_ticks: 1 })
        .expect_err("post-shutdown pump fails");
    assert!(matches!(error, LibbunError::RuntimeShutdown));
}
