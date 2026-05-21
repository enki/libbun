use libbun::{
    BunHost, BunModuleSpec, BunRuntimeConfig, OutputStream, ProviderCallResult,
    ProviderContractIdentity, ProviderDeadline, ProviderDomainClass, ProviderExecutionOperation,
    ProviderRequest, ProviderSettleOptions, SettledProviderReceipt, StructuralValue,
};
use libbun_native::NativeBunRuntime;
use serde_json::json;

const OVERLAY_ENV_KEY: &str = "LIBBUN_NATIVE_OVERLAY_TEST";

fn host() -> BunHost<NativeBunRuntime> {
    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let path = tempdir.keep();
    let config = BunRuntimeConfig::new("native-test-host", path)
        .with_environment_overlay([(OVERLAY_ENV_KEY, "overlay-value")]);
    BunHost::initialize(config).expect("host initializes")
}

fn contract() -> ProviderContractIdentity {
    ProviderContractIdentity {
        package: "@test/native-provider".to_string(),
        capability: "test/native".to_string(),
        contract_fingerprint: "native-test".to_string(),
    }
}

fn settle_options() -> ProviderSettleOptions {
    ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000))
}

fn provider_request(source: &str, export: &str, input: StructuralValue) -> ProviderRequest {
    ProviderRequest {
        contract: contract(),
        domain: ProviderDomainClass::ApplicationIo,
        module: BunModuleSpec::Source {
            module_id: "flows".to_string(),
            source: source.to_string(),
        },
        export: export.to_string(),
        input,
    }
}

fn assert_sync_export(host: &mut BunHost<NativeBunRuntime>, source: &str) {
    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                domain: ProviderDomainClass::JavaScriptExternalTransport,
                ..provider_request(source, "sync", StructuralValue(json!({ "value": 42 })))
            },
            settle_options(),
        )
        .expect("provider call succeeds");

    match receipt {
        SettledProviderReceipt::Ready { result, .. } => assert_eq!(
            result,
            ProviderCallResult::Ok(StructuralValue(json!({
                "ok": true,
                "input": { "value": 42 }
            })))
        ),
        SettledProviderReceipt::Failed(failure) => panic!("expected ready receipt: {failure:?}"),
    }

    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text.contains("native stdout 42")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text.contains("native stderr")
    }));
}

fn assert_async_export(host: &mut BunHost<NativeBunRuntime>, source: &str) {
    let receipt = host
        .call_provider_until_settled(
            provider_request(source, "asyncExport", StructuralValue(json!({ "async": true }))),
            settle_options(),
        )
        .expect("provider call settles");

    match receipt {
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
        }
        SettledProviderReceipt::Failed(failure) => panic!("async export failed: {failure:?}"),
    }
}

fn assert_structured_provider_error(host: &mut BunHost<NativeBunRuntime>, source: &str) {
    let receipt = host
        .call_provider_until_settled(
            provider_request(source, "throws", StructuralValue::null()),
            settle_options(),
        )
        .expect("provider throw is structural");

    match receipt {
        SettledProviderReceipt::Failed(failure) => {
            assert_eq!(
                failure.operation,
                ProviderExecutionOperation::ProviderCallableInvoke
            );
            assert!(
                failure
                    .js_error_message
                    .expect("JS error message")
                    .contains("provider boom")
            );
        }
        SettledProviderReceipt::Ready { result, .. } => panic!("expected provider error: {result:?}"),
    }
}

fn assert_environment_overlay(host: &mut BunHost<NativeBunRuntime>, source: &str) {
    let receipt = host
        .call_provider_until_settled(
            provider_request(source, "readEnv", StructuralValue::null()),
            settle_options(),
        )
        .expect("env export succeeds");
    let SettledProviderReceipt::Ready { result, .. } = receipt else {
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
}

#[test]
fn native_runtime_provider_flows() {
    assert!(
        std::env::var_os(OVERLAY_ENV_KEY).is_none(),
        "test requires {OVERLAY_ENV_KEY} to be unset in the process environment"
    );

    let mut host = host();
    let source = r#"
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

                export function readEnv() {
                    return {
                        processEnv: process.env.LIBBUN_NATIVE_OVERLAY_TEST,
                        bunEnv: Bun.env.LIBBUN_NATIVE_OVERLAY_TEST,
                    };
                }
            "#;

    assert_sync_export(&mut host, source);
    assert_async_export(&mut host, source);
    assert_structured_provider_error(&mut host, source);
    assert_environment_overlay(&mut host, source);
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Log && record.text.contains("loading module module-1")
    }));
}

#[test]
fn source_module_execution_does_not_create_runtime_artifacts() {
    let tempdir = tempfile::tempdir().expect("tempdir creates");
    let mut host = BunHost::<NativeBunRuntime>::initialize(BunRuntimeConfig::new(
        "native-file-free-test-host",
        tempdir.path(),
    ))
    .expect("host initializes");

    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: contract(),
                domain: ProviderDomainClass::ApplicationIo,
                module: BunModuleSpec::Source {
                    module_id: "file-free".to_string(),
                    source: r#"
                        export function run() {
                            console.log("file free stdout");
                            console.error("file free stderr");
                            return { ok: true };
                        }
                    "#
                    .to_string(),
                },
                export: "run".to_string(),
                input: StructuralValue::null(),
            },
            settle_options(),
        )
        .expect("source module export runs");
    let SettledProviderReceipt::Ready { result, .. } = receipt else {
        panic!("expected file-free provider success");
    };

    assert_eq!(
        result,
        ProviderCallResult::Ok(StructuralValue(json!({
            "ok": true
        })))
    );
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text.contains("file free stdout")
    }));
    assert!(host.captured_output().iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text.contains("file free stderr")
    }));

    let artifacts = std::fs::read_dir(tempdir.path())
        .expect("working directory readable")
        .collect::<Result<Vec<_>, _>>()
        .expect("working directory entries readable");
    assert!(
        artifacts.is_empty(),
        "source-module runtime must not create files in working directory: {artifacts:?}"
    );
}

#[test]
fn second_native_runtime_fails_instead_of_blocking() {
    let first_dir = tempfile::tempdir().expect("first tempdir creates");
    let _first = BunHost::<NativeBunRuntime>::initialize(BunRuntimeConfig::new(
        "native-single-runtime-first",
        first_dir.path(),
    ))
    .expect("first native runtime initializes");

    let second_dir = tempfile::tempdir().expect("second tempdir creates");
    let error = BunHost::<NativeBunRuntime>::initialize(BunRuntimeConfig::new(
        "native-single-runtime-second",
        second_dir.path(),
    ))
    .expect_err("second native runtime must be rejected");

    assert!(
        error
            .to_string()
            .contains("another native Bun runtime is already active"),
        "unexpected error: {error}"
    );
}
