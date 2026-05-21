#![cfg(feature = "dynamic-loading")]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use libbun::dynamic::DynamicBunRuntime;
use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, LibbunError, OutputRecord, OutputStream,
    PreparedBundleModuleV1, PreparedBundleV1, ProviderCallResult, ProviderContractIdentity,
    ProviderDeadline, ProviderDomainClass, ProviderExecutionOperation, ProviderRequest,
    ProviderSettleOptions, ProviderSettlementPhase, SettledProviderReceipt, StructuralValue,
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

fn settle_options() -> ProviderSettleOptions {
    ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000))
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

    let provider_source = r#"
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
            "#;
    let provider_module = || BunModuleSpec::Source {
        module_id: "dynamic-conformance".to_string(),
        source: provider_source.to_string(),
    };

    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::JavaScriptExternalTransport,
                module: provider_module(),
                export: "sync".to_string(),
                input: StructuralValue(json!({ "value": 42 })),
            },
            settle_options(),
        )
        .expect("provider call succeeds");
    match receipt {
        SettledProviderReceipt::Ready {
            result, artifact, ..
        } => {
            assert_eq!(
                result,
                ProviderCallResult::Ok(StructuralValue(json!({
                    "ok": true,
                    "input": { "value": 42 }
                })))
            );
            assert_eq!(artifact.bun_revision, env!("LIBBUN_BUN_SOURCE_COMMIT"));
        }
        SettledProviderReceipt::Failed(failure) => panic!("expected ready receipt: {failure:?}"),
    }

    let async_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: provider_module(),
                export: "asyncExport".to_string(),
                input: StructuralValue(json!({ "async": true })),
            },
            settle_options(),
        )
        .expect("provider call settles");
    match async_receipt {
        SettledProviderReceipt::Ready {
            result, settlement, ..
        } => {
            assert_eq!(
                result,
                ProviderCallResult::Ok(StructuralValue(json!({ "async": true })))
            );
            assert_eq!(
                settlement.operation,
                ProviderExecutionOperation::ProviderPromiseSettle
            );
            assert!(
                settlement
                    .trace
                    .iter()
                    .any(|event| event.phase == ProviderSettlementPhase::ResolveAsync)
            );
            assert_eq!(
                settlement.trace.last().map(|event| event.phase),
                Some(ProviderSettlementPhase::Complete)
            );
        }
        SettledProviderReceipt::Failed(failure) => panic!("expected async success: {failure:?}"),
    }

    let error_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: provider_module(),
                export: "throws".to_string(),
                input: StructuralValue::null(),
            },
            settle_options(),
        )
        .expect("provider throw is structural");
    match error_receipt {
        SettledProviderReceipt::Failed(failure) => {
            assert_eq!(
                failure.operation,
                ProviderExecutionOperation::ProviderCallableInvoke
            );
            assert!(
                failure
                    .js_error_message
                    .expect("JS error message")
                    .contains("dynamic provider boom")
            );
        }
        SettledProviderReceipt::Ready { result, .. } => {
            panic!("expected provider failure, got {result:?}")
        }
    }

    let env_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: provider_module(),
                export: "readEnv".to_string(),
                input: StructuralValue::null(),
            },
            settle_options(),
        )
        .expect("env export succeeds");
    let SettledProviderReceipt::Ready { result, .. } = env_receipt else {
        panic!("expected env success");
    };
    assert_eq!(
        result,
        ProviderCallResult::Ok(StructuralValue(json!({
            "processEnv": "overlay-value",
            "bunEnv": "overlay-value"
        })))
    );
    assert!(std::env::var_os(OVERLAY_ENV_KEY).is_none());

    let module_load_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::Source {
                    module_id: "dynamic-conformance-throwing-import".to_string(),
                    source: r#"
                        throw new Error("dynamic module import diagnostic boom");
                    "#
                    .to_string(),
                },
                export: "default".to_string(),
                input: StructuralValue::null(),
            },
            settle_options(),
        )
        .expect("module import throw should be a settled failure");
    let SettledProviderReceipt::Failed(module_load_failure) = module_load_receipt else {
        panic!("expected module import failure");
    };
    let module_load_message = module_load_failure
        .js_error_message
        .as_deref()
        .unwrap_or_default();
    assert!(
        module_load_failure.operation == ProviderExecutionOperation::ProviderModuleImport
            && module_load_failure
                .module_specifier_or_url
                .contains("dynamic-conformance-throwing-import")
            && module_load_message.contains("dynamic module import diagnostic boom"),
        "module-load diagnostic must name the import operation, specifier, and JS exception detail; got {module_load_failure:?}"
    );

    let substrate_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "@proving/agent".to_string(),
                    capability: "capability:advanceTurnSource".to_string(),
                    contract_fingerprint: "substrate".to_string(),
                },
                domain: ProviderDomainClass::RustSubstrateAuthority,
                module: provider_module(),
                export: "mustNotRun".to_string(),
                input: StructuralValue(json!({ "mustNotRun": true })),
            },
            settle_options(),
        )
        .expect("substrate rejection is structural");
    match substrate_receipt {
        SettledProviderReceipt::Ready { result, .. } => match result {
            ProviderCallResult::Err(error) => {
                assert_eq!(error.code, "rust_substrate_authority_rejected");
            }
            ProviderCallResult::Ok(_) => panic!("substrate export should not execute"),
        },
        SettledProviderReceipt::Failed(failure) => {
            panic!("substrate rejection should be ready: {failure:?}")
        }
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
    let prepared_receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::PreparedBundle {
                    bundle_id: "dynamic-prepared".to_string(),
                    bytes: bundle.to_bytes().expect("bundle serializes"),
                },
                export: "bundle".to_string(),
                input: StructuralValue(json!({ "from": "prepared" })),
            },
            settle_options(),
        )
        .expect("prepared export succeeds");
    let SettledProviderReceipt::Ready { result, .. } = prepared_receipt else {
        panic!("expected prepared bundle success");
    };
    assert_eq!(
        result,
        ProviderCallResult::Ok(StructuralValue(json!({
            "value": 7,
            "input": { "from": "prepared" }
        })))
    );

    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stdout
            && record.text.contains("dynamic conformance stdout 42")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text.contains("dynamic conformance stderr")
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
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: provider_module(),
                export: "sync".to_string(),
                input: StructuralValue::null(),
            },
            settle_options(),
        )
        .expect_err("post-shutdown provider call fails");
    assert!(matches!(error, LibbunError::RuntimeShutdown));
}
