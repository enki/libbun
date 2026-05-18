use std::collections::BTreeMap;

use libbun::BunAsyncHandle;
use libbun::BunEmbeddingRuntime;
use libbun::BunHost;
use libbun::BunModuleHandle;
use libbun::BunModuleSpec;
use libbun::BunRuntimeConfig;
use libbun::ExportCallResult;
use libbun::LibbunError;
use libbun::LibbunResult;
use libbun::OutputRecord;
use libbun::OutputStream;
use libbun::ProviderCallResult;
use libbun::ProviderContractIdentity;
use libbun::ProviderDomainClass;
use libbun::ProviderError;
use libbun::ProviderHostReceipt;
use libbun::ProviderRequest;
use libbun::PumpBudget;
use libbun::PumpOutcome;
use libbun::StructuralValue;
use serde_json::json;

#[derive(Debug)]
struct ConformanceRuntime {
    modules: BTreeMap<String, ModuleBehavior>,
    async_results: BTreeMap<String, PendingResult>,
    output: Vec<OutputRecord>,
    next_async: u64,
    shutdown: bool,
}

#[derive(Debug, Clone)]
enum ModuleBehavior {
    Echo,
    AsyncEcho,
    ProviderError,
}

#[derive(Debug, Clone)]
struct PendingResult {
    remaining_ticks: u32,
    result: ProviderCallResult,
}

impl BunEmbeddingRuntime for ConformanceRuntime {
    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
        Ok(Self {
            modules: BTreeMap::new(),
            async_results: BTreeMap::new(),
            output: vec![OutputRecord {
                stream: OutputStream::Log,
                text: format!("initialized {}", config.host_id),
            }],
            next_async: 1,
            shutdown: false,
        })
    }

    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
        let (id, behavior) = match spec {
            BunModuleSpec::Source { module_id, source } => {
                let behavior = match source.as_str() {
                    "export:echo" => ModuleBehavior::Echo,
                    "export:async_echo" => ModuleBehavior::AsyncEcho,
                    "export:error" => ModuleBehavior::ProviderError,
                    other => {
                        return Err(LibbunError::module_load(format!(
                            "unknown test source `{other}`"
                        )));
                    }
                };
                (module_id, behavior)
            }
            BunModuleSpec::Path { path } => {
                (path.to_string_lossy().to_string(), ModuleBehavior::Echo)
            }
            BunModuleSpec::PreparedBundle { bundle_id, .. } => (bundle_id, ModuleBehavior::Echo),
        };
        self.modules.insert(id.clone(), behavior);
        Ok(BunModuleHandle { id })
    }

    fn call_export(
        &mut self,
        module: &BunModuleHandle,
        export: &str,
        input: StructuralValue,
    ) -> LibbunResult<ExportCallResult> {
        if self.shutdown {
            return Err(LibbunError::RuntimeShutdown);
        }
        if export != "default" {
            return Err(LibbunError::export_call(format!(
                "unknown export `{export}`"
            )));
        }

        match self
            .modules
            .get(&module.id)
            .ok_or_else(|| LibbunError::module_load("unknown module handle"))?
        {
            ModuleBehavior::Echo => {
                self.output.push(OutputRecord {
                    stream: OutputStream::Stdout,
                    text: "called default".to_string(),
                });
                Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
            }
            ModuleBehavior::AsyncEcho => {
                let handle = BunAsyncHandle {
                    id: format!("async-{}", self.next_async),
                };
                self.next_async += 1;
                self.async_results.insert(
                    handle.id.clone(),
                    PendingResult {
                        remaining_ticks: 1,
                        result: ProviderCallResult::Ok(input),
                    },
                );
                Ok(ExportCallResult::Pending(handle))
            }
            ModuleBehavior::ProviderError => Ok(ExportCallResult::Ready(ProviderCallResult::Err(
                ProviderError {
                    code: "provider_failed".to_string(),
                    message: "provider returned a structured error".to_string(),
                },
            ))),
        }
    }

    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
        let mut ticks = 0;
        while ticks < budget.max_ticks {
            ticks += 1;
            for pending in self.async_results.values_mut() {
                pending.remaining_ticks = pending.remaining_ticks.saturating_sub(1);
            }
        }
        Ok(PumpOutcome {
            ticks,
            pending_async_work: self
                .async_results
                .values()
                .filter(|pending| pending.remaining_ticks > 0)
                .count(),
        })
    }

    fn resolve_async(
        &mut self,
        handle: &BunAsyncHandle,
    ) -> LibbunResult<Option<ProviderCallResult>> {
        let Some(pending) = self.async_results.get(&handle.id) else {
            return Err(LibbunError::UnknownAsyncHandle {
                handle: handle.id.clone(),
            });
        };
        if pending.remaining_ticks > 0 {
            return Ok(None);
        }
        let result = self
            .async_results
            .remove(&handle.id)
            .expect("pending result exists after readiness check")
            .result;
        Ok(Some(result))
    }

    fn captured_output(&self) -> &[OutputRecord] {
        &self.output
    }

    fn shutdown(&mut self) -> LibbunResult<()> {
        self.shutdown = true;
        Ok(())
    }
}

fn host() -> BunHost<ConformanceRuntime> {
    BunHost::initialize(BunRuntimeConfig::new("test-host", "/tmp")).expect("host initializes")
}

fn contract() -> ProviderContractIdentity {
    ProviderContractIdentity {
        package: "@test/provider".to_string(),
        capability: "test/capability".to_string(),
        contract_fingerprint: "fingerprint".to_string(),
    }
}

#[test]
fn provider_call_returns_structural_result_and_captures_output() {
    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "echo".to_string(),
            source: "export:echo".to_string(),
        })
        .expect("module loads");

    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::JavaScriptExternalTransport,
            module,
            export: "default".to_string(),
            input: StructuralValue(json!({ "ok": true })),
        })
        .expect("provider call succeeds");

    match receipt {
        ProviderHostReceipt::Ready(ready) => {
            assert_eq!(
                ready.result,
                ProviderCallResult::Ok(StructuralValue(json!({ "ok": true })))
            );
            assert_eq!(
                ready.artifact.bun_revision,
                env!("LIBBUN_BUN_SOURCE_COMMIT")
            );
        }
        ProviderHostReceipt::Parked(_) => panic!("expected ready receipt"),
    }
    assert!(
        host.captured_output()
            .iter()
            .any(|record| record.stream == OutputStream::Stdout && record.text == "called default")
    );
}

#[test]
fn async_provider_work_is_driven_by_explicit_pump() {
    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "async".to_string(),
            source: "export:async_echo".to_string(),
        })
        .expect("module loads");

    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module,
            export: "default".to_string(),
            input: StructuralValue(json!({ "async": true })),
        })
        .expect("provider call starts");
    let handle = match receipt {
        ProviderHostReceipt::Parked(parked) => parked.handle,
        ProviderHostReceipt::Ready(_) => panic!("expected parked async receipt"),
    };

    assert_eq!(host.resolve_async(&handle).expect("resolve polls"), None);
    let pump = host
        .pump_event_loop(PumpBudget { max_ticks: 1 })
        .expect("event loop pumps");
    assert_eq!(pump.ticks, 1);
    assert_eq!(pump.pending_async_work, 0);
    assert_eq!(
        host.resolve_async(&handle).expect("async result resolves"),
        Some(ProviderCallResult::Ok(StructuralValue(
            json!({ "async": true })
        )))
    );
}

#[test]
fn provider_error_is_structured_and_does_not_terminate_process() {
    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "error".to_string(),
            source: "export:error".to_string(),
        })
        .expect("module loads");

    let receipt = host
        .call_provider(ProviderRequest {
            contract: contract(),
            domain: ProviderDomainClass::ApplicationIo,
            module,
            export: "default".to_string(),
            input: StructuralValue::null(),
        })
        .expect("provider errors are returned structurally");

    match receipt {
        ProviderHostReceipt::Ready(ready) => {
            assert_eq!(
                ready.result,
                ProviderCallResult::Err(ProviderError {
                    code: "provider_failed".to_string(),
                    message: "provider returned a structured error".to_string(),
                })
            );
        }
        ProviderHostReceipt::Parked(_) => panic!("expected ready error receipt"),
    }

    host.shutdown()
        .expect("host remains alive after provider error");
}

#[test]
fn rust_substrate_exports_are_rejected_before_provider_execution() {
    let mut host = host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "echo".to_string(),
            source: "export:echo".to_string(),
        })
        .expect("module loads");

    let receipt = host
        .call_provider(ProviderRequest {
            contract: ProviderContractIdentity {
                package: "@proving/agent".to_string(),
                capability: "capability:advanceTurnSource".to_string(),
                contract_fingerprint: "substrate".to_string(),
            },
            domain: ProviderDomainClass::RustSubstrateAuthority,
            module,
            export: "default".to_string(),
            input: StructuralValue(json!({ "mustNotRun": true })),
        })
        .expect("substrate rejection is structural");

    match receipt {
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
            .any(|record| record.stream == OutputStream::Stdout)
    );
}

#[test]
fn shutdown_is_deterministic_and_blocks_later_calls() {
    let mut host = host();
    host.shutdown().expect("shutdown succeeds");
    let error = host
        .pump_event_loop(PumpBudget { max_ticks: 1 })
        .expect_err("post-shutdown pump fails");
    assert!(matches!(error, LibbunError::RuntimeShutdown));
}
