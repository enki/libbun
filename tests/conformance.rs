use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use libbun::BackendState;
use libbun::BunAsyncHandle;
use libbun::BunEmbeddingRuntime;
use libbun::BunHost;
use libbun::BunModuleHandle;
use libbun::BunModuleSpec;
use libbun::BunProviderBackend;
use libbun::BunRuntimeConfig;
use libbun::ExportCallResult;
use libbun::InvocationOutputPolicy;
use libbun::LibbunError;
use libbun::LibbunResult;
use libbun::LowLevelBunHost;
use libbun::OutputRecord;
use libbun::OutputStream;
use libbun::PreparedBundleModuleV1;
use libbun::PreparedBundleV1;
use libbun::ProviderCallResult;
use libbun::ProviderContractIdentity;
use libbun::ProviderDeadline;
use libbun::ProviderDiagnosticEvent;
use libbun::ProviderDiagnosticEventKind;
use libbun::ProviderDiagnosticPhase;
use libbun::ProviderDomainClass;
use libbun::ProviderError;
use libbun::ProviderExecutionOperation;
use libbun::ProviderInvocationDescriptor;
use libbun::ProviderRequest;
use libbun::ProviderRuntimeState;
use libbun::ProviderSettleOptions;
use libbun::ProviderSettlementPhase;
use libbun::PumpBudget;
use libbun::PumpOutcome;
use libbun::SettledProviderReceipt;
use libbun::SinkPolicy;
use libbun::StructuralValue;
use serde_json::json;

#[derive(Debug)]
struct ConformanceRuntime {
    modules: BTreeMap<String, ModuleBehavior>,
    async_results: BTreeMap<String, PendingResult>,
    output: Vec<OutputRecord>,
    late_output_after_empty_drain: Option<OutputRecord>,
    next_async: u64,
    shutdown: bool,
}

#[derive(Debug, Clone)]
enum ModuleBehavior {
    Echo,
    AsyncEcho,
    BlockingEcho,
    NeverSettles,
    ProviderError,
    PromiseReject,
    PromiseRejectWithOutput,
    LoadOutputEcho,
    LateAfterSettle,
    InvalidCallable,
}

#[derive(Debug, Clone)]
struct PendingResult {
    remaining_ticks: u32,
    result: ProviderCallResult,
    output_on_ready: Vec<OutputRecord>,
    emitted_ready_output: bool,
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
            late_output_after_empty_drain: None,
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
                    "export:blocking_echo" => ModuleBehavior::BlockingEcho,
                    "export:never" => ModuleBehavior::NeverSettles,
                    "export:error" => ModuleBehavior::ProviderError,
                    "export:reject" => ModuleBehavior::PromiseReject,
                    "export:reject_with_output" => ModuleBehavior::PromiseRejectWithOutput,
                    "export:load_output_echo" => {
                        self.output.push(OutputRecord {
                            stream: OutputStream::Log,
                            text: "module load output".to_string(),
                        });
                        ModuleBehavior::LoadOutputEcho
                    }
                    "export:late_after_settle" => ModuleBehavior::LateAfterSettle,
                    "export:invalid_callable" => ModuleBehavior::InvalidCallable,
                    "export:import_error" => {
                        return Err(LibbunError::module_load(
                            "import stack: Error: test import boom\n    at source:test",
                        ));
                    }
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
                "missing export `{export}`"
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
                self.output.push(OutputRecord {
                    stream: OutputStream::Stdout,
                    text: "async before await".to_string(),
                });
                let handle = BunAsyncHandle {
                    id: format!("async-{}", self.next_async),
                };
                self.next_async += 1;
                self.async_results.insert(
                    handle.id.clone(),
                    PendingResult {
                        remaining_ticks: 1,
                        result: ProviderCallResult::Ok(input),
                        output_on_ready: vec![
                            OutputRecord {
                                stream: OutputStream::Stderr,
                                text: "async after await".to_string(),
                            },
                            OutputRecord {
                                stream: OutputStream::Log,
                                text: "async settled log".to_string(),
                            },
                        ],
                        emitted_ready_output: false,
                    },
                );
                Ok(ExportCallResult::Pending(handle))
            }
            ModuleBehavior::BlockingEcho => {
                std::thread::sleep(Duration::from_millis(200));
                Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
            }
            ModuleBehavior::NeverSettles => {
                let handle = BunAsyncHandle {
                    id: format!("async-{}", self.next_async),
                };
                self.next_async += 1;
                self.async_results.insert(
                    handle.id.clone(),
                    PendingResult {
                        remaining_ticks: u32::MAX,
                        result: ProviderCallResult::Ok(input),
                        output_on_ready: Vec::new(),
                        emitted_ready_output: false,
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
            ModuleBehavior::PromiseReject => {
                let handle = BunAsyncHandle {
                    id: format!("async-{}", self.next_async),
                };
                self.next_async += 1;
                self.async_results.insert(
                    handle.id.clone(),
                    PendingResult {
                        remaining_ticks: 1,
                        result: ProviderCallResult::Err(ProviderError {
                            code: "provider_rejected".to_string(),
                            message: "Error: async provider boom\n    at provider.test".to_string(),
                        }),
                        output_on_ready: Vec::new(),
                        emitted_ready_output: false,
                    },
                );
                Ok(ExportCallResult::Pending(handle))
            }
            ModuleBehavior::PromiseRejectWithOutput => {
                self.output.push(OutputRecord {
                    stream: OutputStream::Stdout,
                    text: "reject before await".to_string(),
                });
                let handle = BunAsyncHandle {
                    id: format!("async-{}", self.next_async),
                };
                self.next_async += 1;
                self.async_results.insert(
                    handle.id.clone(),
                    PendingResult {
                        remaining_ticks: 1,
                        result: ProviderCallResult::Err(ProviderError {
                            code: "provider_rejected".to_string(),
                            message: "Error: async provider boom\n    at provider.test".to_string(),
                        }),
                        output_on_ready: vec![OutputRecord {
                            stream: OutputStream::Stderr,
                            text: "reject async output".to_string(),
                        }],
                        emitted_ready_output: false,
                    },
                );
                Ok(ExportCallResult::Pending(handle))
            }
            ModuleBehavior::LoadOutputEcho => {
                self.output.push(OutputRecord {
                    stream: OutputStream::Stdout,
                    text: "called default".to_string(),
                });
                Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
            }
            ModuleBehavior::LateAfterSettle => {
                self.late_output_after_empty_drain = Some(OutputRecord {
                    stream: OutputStream::Log,
                    text: "late output after settled receipt".to_string(),
                });
                Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
            }
            ModuleBehavior::InvalidCallable => {
                Err(LibbunError::export_call("export `default` is not callable"))
            }
        }
    }

    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
        let mut ticks = 0;
        while ticks < budget.max_ticks {
            ticks += 1;
            for pending in self.async_results.values_mut() {
                pending.remaining_ticks = pending.remaining_ticks.saturating_sub(1);
                if pending.remaining_ticks == 0 && !pending.emitted_ready_output {
                    self.output.append(&mut pending.output_on_ready);
                    pending.emitted_ready_output = true;
                }
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

    fn drain_captured_output(&mut self) -> Vec<OutputRecord> {
        if self.output.is_empty() {
            if let Some(record) = self.late_output_after_empty_drain.take() {
                self.output.push(record);
                return Vec::new();
            }
        }
        std::mem::take(&mut self.output)
    }

    fn shutdown(&mut self) -> LibbunResult<()> {
        self.shutdown = true;
        Ok(())
    }
}

fn host() -> BunHost<ConformanceRuntime> {
    BunHost::initialize(BunRuntimeConfig::new("test-host", "/tmp")).expect("host initializes")
}

fn low_level_host() -> LowLevelBunHost<ConformanceRuntime> {
    LowLevelBunHost::initialize(BunRuntimeConfig::new("test-host", "/tmp"))
        .expect("host initializes")
}

fn contract() -> ProviderContractIdentity {
    ProviderContractIdentity {
        package: "@test/provider".to_string(),
        capability: "test/capability".to_string(),
        contract_fingerprint: "fingerprint".to_string(),
    }
}

fn settle_options() -> ProviderSettleOptions {
    ProviderSettleOptions::new(ProviderDeadline::from_millis(1_000))
}

fn invocation_profile_phases(finished: &libbun::FinishedInvocation) -> Vec<&str> {
    assert_eq!(
        finished.profile.schema,
        "libbun.invocation_profile.ledger.v1"
    );
    assert_eq!(finished.profile.invocation_id, finished.invocation_id);
    assert!(
        !finished.profile.spans.is_empty(),
        "finished invocation must carry libbun-owned profile spans"
    );
    for span in finished.profile.spans.iter() {
        assert_eq!(span.schema, "libbun.invocation_profile.span.v1");
    }
    finished
        .profile
        .spans
        .iter()
        .map(|span| span.phase.as_str())
        .collect()
}

fn assert_profile_contains(phases: &[&str], expected: &str) {
    assert!(
        phases.contains(&expected),
        "expected retained invocation profile to contain phase `{expected}`, got {phases:?}"
    );
}

fn source_request(source: &str, input: StructuralValue) -> ProviderRequest {
    ProviderRequest {
        contract: contract(),
        domain: ProviderDomainClass::JavaScriptExternalTransport,
        module: BunModuleSpec::Source {
            module_id: source.to_string(),
            source: source.to_string(),
        },
        export: "default".to_string(),
        input,
    }
}

#[test]
fn settled_provider_call_returns_structural_result_and_captures_output() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request("export:echo", StructuralValue(json!({ "ok": true }))),
            settle_options(),
        )
        .expect("provider call succeeds");

    match receipt {
        SettledProviderReceipt::Ready {
            result,
            artifact,
            output,
            settlement,
            ..
        } => {
            assert_eq!(
                result,
                ProviderCallResult::Ok(StructuralValue(json!({ "ok": true })))
            );
            assert_eq!(artifact.bun_revision, env!("LIBBUN_BUN_SOURCE_COMMIT"));
            assert_eq!(
                settlement.operation,
                ProviderExecutionOperation::ProviderCallableInvoke
            );
            assert!(
                settlement.call_id.is_some(),
                "settled receipts must carry a generated call id"
            );
            assert!(output.iter().any(|record| {
                record.stream == OutputStream::Stdout && record.text == "called default"
            }));
        }
        SettledProviderReceipt::Failed(failure) => panic!("expected ready receipt: {failure:?}"),
    }
    assert!(
        host.captured_output()
            .iter()
            .any(|record| record.stream == OutputStream::Stdout && record.text == "called default")
    );
}

#[test]
fn settled_async_provider_work_does_not_require_host_polling() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request(
                "export:async_echo",
                StructuralValue(json!({ "async": true })),
            ),
            settle_options(),
        )
        .expect("provider call settles");

    match receipt {
        SettledProviderReceipt::Ready {
            result,
            output,
            settlement,
            ..
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
                output
                    .iter()
                    .any(|record| record.text == "async before await")
            );
            assert!(
                output
                    .iter()
                    .any(|record| record.text == "async after await")
            );
            assert!(
                output
                    .iter()
                    .any(|record| record.text == "async settled log")
            );
        }
        SettledProviderReceipt::Failed(failure) => panic!("expected async success: {failure:?}"),
    }
}

#[test]
fn provider_error_is_structured_and_does_not_terminate_process() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request("export:error", StructuralValue::null()),
            settle_options(),
        )
        .expect("provider errors are returned structurally");

    match receipt {
        SettledProviderReceipt::Ready { result, .. } => {
            assert_eq!(
                result,
                ProviderCallResult::Err(ProviderError {
                    code: "provider_failed".to_string(),
                    message: "provider returned a structured error".to_string(),
                })
            );
        }
        SettledProviderReceipt::Failed(failure) => {
            panic!("provider domain errors should be ready results: {failure:?}")
        }
    }

    host.shutdown()
        .expect("host remains alive after provider error");
}

#[test]
fn rust_substrate_exports_are_rejected_before_provider_execution() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "@proving/agent".to_string(),
                    capability: "capability:advanceTurnSource".to_string(),
                    contract_fingerprint: "substrate".to_string(),
                },
                domain: ProviderDomainClass::RustSubstrateAuthority,
                module: BunModuleSpec::Source {
                    module_id: "substrate".to_string(),
                    source: "export:echo".to_string(),
                },
                export: "default".to_string(),
                input: StructuralValue(json!({ "mustNotRun": true })),
            },
            settle_options(),
        )
        .expect("substrate rejection is structural");

    match receipt {
        SettledProviderReceipt::Ready { result, .. } => match result {
            ProviderCallResult::Err(error) => {
                assert_eq!(error.code, "rust_substrate_authority_rejected");
            }
            ProviderCallResult::Ok(_) => panic!("substrate export should not execute"),
        },
        SettledProviderReceipt::Failed(failure) => {
            panic!("substrate rejection should be a ready provider error: {failure:?}")
        }
    }

    assert!(
        !host
            .captured_output()
            .iter()
            .any(|record| record.stream == OutputStream::Stdout)
    );
}

#[test]
fn provider_promise_rejection_returns_structured_failure_with_stack() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request("export:reject", StructuralValue::null()),
            settle_options(),
        )
        .expect("provider call settles");

    let SettledProviderReceipt::Failed(failure) = receipt else {
        panic!("expected structured rejection failure");
    };
    assert_eq!(
        failure.operation,
        ProviderExecutionOperation::ProviderPromiseSettle
    );
    let message = failure.js_error_message.expect("JS error message");
    assert!(message.contains("async provider boom"));
    assert!(message.contains("provider.test"));
    assert_eq!(failure.module_specifier_or_url, "source:export:reject");
    assert_eq!(failure.export_name, "default");
}

#[test]
fn module_import_failure_returns_structured_failure_with_module_identity() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request("export:import_error", StructuralValue::null()),
            settle_options(),
        )
        .expect("module import failure is a settled receipt");

    let SettledProviderReceipt::Failed(failure) = receipt else {
        panic!("expected module import failure");
    };
    assert_eq!(
        failure.operation,
        ProviderExecutionOperation::ProviderModuleImport
    );
    assert_eq!(
        failure.module_specifier_or_url,
        "source:export:import_error"
    );
    assert!(
        failure
            .js_error_message
            .expect("message")
            .contains("test import boom")
    );
}

#[test]
fn missing_export_and_invalid_callable_are_structured_failures() {
    let mut host = host();
    let missing = host
        .call_provider_until_settled(
            ProviderRequest {
                export: "missing".to_string(),
                ..source_request("export:echo", StructuralValue::null())
            },
            settle_options(),
        )
        .expect("missing export is structural");
    let SettledProviderReceipt::Failed(failure) = missing else {
        panic!("expected missing export failure");
    };
    assert_eq!(
        failure.operation,
        ProviderExecutionOperation::ProviderExportLookup
    );

    let invalid = host
        .call_provider_until_settled(
            source_request("export:invalid_callable", StructuralValue::null()),
            settle_options(),
        )
        .expect("invalid callable is structural");
    let SettledProviderReceipt::Failed(failure) = invalid else {
        panic!("expected invalid callable failure");
    };
    assert_eq!(
        failure.operation,
        ProviderExecutionOperation::ProviderCallableValidate
    );
}

#[test]
fn deadline_expiry_returns_pending_async_diagnostics() {
    let mut host = host();
    let receipt = host
        .call_provider_until_settled(
            source_request("export:never", StructuralValue::null()),
            ProviderSettleOptions::new(ProviderDeadline::from_millis(0)),
        )
        .expect("deadline failure is structural");
    let SettledProviderReceipt::Failed(failure) = receipt else {
        panic!("expected deadline failure");
    };
    assert_eq!(
        failure.operation,
        ProviderExecutionOperation::ProviderDeadlineElapsed
    );
    assert_eq!(failure.deadline_ms, 0);
    assert!(
        failure.call_id.is_some(),
        "settled failures must carry a generated call id"
    );
    assert!(failure.pending_async_task_count >= 1);
    assert!(
        failure
            .trace
            .iter()
            .any(|event| event.phase == ProviderSettlementPhase::CallExport)
    );
    assert_eq!(
        failure.trace.last().map(|event| event.phase),
        Some(ProviderSettlementPhase::DeadlineElapsed)
    );
}

#[test]
fn low_level_async_controls_are_quarantined_on_named_host() {
    let mut host = low_level_host();
    let module = host
        .load_module(BunModuleSpec::Source {
            module_id: "async".to_string(),
            source: "export:async_echo".to_string(),
        })
        .expect("module loads");

    let result = host
        .call_export(
            &module,
            "default",
            StructuralValue(json!({ "async": true })),
        )
        .expect("export starts");
    let ExportCallResult::Pending(handle) = result else {
        panic!("expected low-level parked handle");
    };

    assert_eq!(host.resolve_async(&handle).expect("resolve polls"), None);
    let pump = host
        .pump_event_loop(PumpBudget { max_ticks: 1 })
        .expect("event loop pumps");
    assert_eq!(pump.ticks, 1);
    assert_eq!(
        host.resolve_async(&handle).expect("async result resolves"),
        Some(ProviderCallResult::Ok(StructuralValue(
            json!({ "async": true })
        )))
    );
}

#[test]
fn shutdown_is_deterministic_and_blocks_later_calls() {
    let mut host = host();
    host.shutdown().expect("shutdown succeeds");
    let error = host
        .call_provider_until_settled(
            source_request("export:echo", StructuralValue::null()),
            settle_options(),
        )
        .expect_err("post-shutdown call fails");
    assert!(matches!(error, LibbunError::RuntimeShutdown));
}

#[test]
fn output_handler_receives_records_without_polling_runtime() {
    let records = Arc::new(Mutex::new(Vec::new()));
    let handler_records = Arc::clone(&records);
    let mut host = BunHost::<ConformanceRuntime>::initialize_diagnostic_with_output_handler(
        BunRuntimeConfig::new("test-host", "/tmp"),
        move |record| {
            handler_records
                .lock()
                .expect("handler records lock")
                .push(record);
        },
    )
    .expect("host initializes");

    host.call_provider_until_settled(
        source_request("export:echo", StructuralValue(json!({ "ok": true }))),
        settle_options(),
    )
    .expect("export succeeds");

    let records = records.lock().expect("handler records lock");
    assert!(records.iter().any(|record| {
        record.stream == OutputStream::Log && record.text == "initialized test-host"
    }));
    assert!(records.iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text == "called default"
    }));
}

#[test]
fn host_can_drain_captured_output_history() {
    let mut host = host();
    let initial = host.drain_captured_output();
    assert_eq!(initial.len(), 1);
    assert!(host.captured_output().is_empty());
}

#[test]
fn bun_host_post_settlement_output_without_invocation_ledger_fails_closed() {
    let mut host = host();
    let _ = host.drain_captured_output();

    let error = match host.call_provider_until_settled(
        source_request("export:late_after_settle", StructuralValue::null()),
        settle_options(),
    ) {
        Ok(_) => panic!("post-settlement output must not be silently retained outside a ledger"),
        Err(error) => error,
    };
    assert!(matches!(error, LibbunError::BackendState { .. }));
    assert!(
        error
            .to_string()
            .contains("bun_host_post_settlement_output_without_invocation_ledger_forbidden")
    );
    let captured = host.drain_captured_output();
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].text, "late output after settled receipt");
}

#[test]
fn retained_backend_invocation_ledger_excludes_startup_output_and_reuses_backend() {
    let mut backend = BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new(
        "retained-test-host",
        "/tmp",
    ))
    .expect("retained backend opens");
    assert_eq!(backend.state(), BackendState::Ready);
    assert_eq!(backend.startup_output().len(), 1);
    assert_eq!(
        backend.startup_output()[0].text,
        "initialized retained-test-host"
    );

    let first = backend
        .begin_invocation(ProviderInvocationDescriptor::new("retained-call-1"))
        .expect("first invocation begins")
        .settle_provider(
            source_request("export:echo", StructuralValue(json!({ "first": true }))),
            settle_options(),
        )
        .expect("first invocation settles")
        .finish()
        .expect("first invocation finishes");
    assert_eq!(backend.state(), BackendState::Ready);
    assert_eq!(first.output.invocation_id, "retained-call-1");
    assert_eq!(first.output.record_count, 1);
    assert_eq!(first.output.records[0].text, "called default");
    let first_phases = invocation_profile_phases(&first);
    assert_profile_contains(&first_phases, "backend_begin_invocation");
    assert_profile_contains(&first_phases, "provider_module_load");
    assert_profile_contains(&first_phases, "provider_call_dispatch");
    assert_profile_contains(&first_phases, "provider_call_settlement");
    assert_profile_contains(&first_phases, "backend_finish_invocation");
    assert_profile_contains(&first_phases, "output_ledger_finish");

    let second = backend
        .begin_invocation(ProviderInvocationDescriptor::new("retained-call-2"))
        .expect("second invocation begins")
        .settle_provider(
            source_request(
                "export:async_echo",
                StructuralValue(json!({ "second": true })),
            ),
            settle_options(),
        )
        .expect("second invocation settles")
        .finish()
        .expect("second invocation finishes");
    assert_eq!(backend.state(), BackendState::Ready);
    assert_eq!(second.output.invocation_id, "retained-call-2");
    assert_eq!(second.output.record_count, 3);
    assert!(second.output.records.iter().any(|record| {
        record.stream == OutputStream::Stdout && record.text == "async before await"
    }));
    assert!(second.output.records.iter().any(|record| {
        record.stream == OutputStream::Stderr && record.text == "async after await"
    }));
    assert!(second.output.records.iter().any(|record| {
        record.stream == OutputStream::Log && record.text == "async settled log"
    }));
    let second_phases = invocation_profile_phases(&second);
    assert_profile_contains(&second_phases, "provider_settlement");
    assert_profile_contains(&second_phases, "provider_event_loop_pump");
}

#[test]
fn retained_backend_provider_rejection_is_terminal_receipt_not_backend_poison() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    let finished = backend
        .begin_invocation(ProviderInvocationDescriptor::new("rejected-call"))
        .expect("invocation begins")
        .settle_provider(
            source_request("export:reject", StructuralValue::null()),
            settle_options(),
        )
        .expect("provider rejection still produces a terminal receipt")
        .finish()
        .expect("rejected invocation finishes");

    assert_eq!(backend.state(), BackendState::Ready);
    assert!(matches!(
        finished.receipt,
        SettledProviderReceipt::Failed(_)
    ));
    let phases = invocation_profile_phases(&finished);
    assert_profile_contains(&phases, "provider_call_settlement");
    assert_profile_contains(&phases, "backend_finish_invocation");
}

#[test]
fn retained_backend_invocation_ledger_captures_all_provider_output_phases_once() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    let module_load_and_call = backend
        .begin_invocation(ProviderInvocationDescriptor::new("module-load-and-call"))
        .expect("invocation begins")
        .settle_provider(
            source_request("export:load_output_echo", StructuralValue::null()),
            settle_options(),
        )
        .expect("invocation settles")
        .finish()
        .expect("invocation finishes");
    assert_eq!(backend.state(), BackendState::Ready);
    assert_eq!(module_load_and_call.output.record_count, 2);
    assert_eq!(
        module_load_and_call
            .output
            .records
            .iter()
            .map(|record| record.text.as_str())
            .collect::<Vec<_>>(),
        vec!["module load output", "called default"]
    );
    let after = backend
        .begin_invocation(ProviderInvocationDescriptor::new(
            "after-module-load-ledger",
        ))
        .expect("backend remains reusable after module-load output ledger")
        .settle_provider(
            source_request("export:echo", StructuralValue::null()),
            settle_options(),
        )
        .expect("second invocation settles")
        .finish()
        .expect("second invocation finishes");
    assert_eq!(after.output.record_count, 1);
}

#[test]
fn retained_backend_invocation_ledger_captures_async_and_rejection_output_once() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    let async_finished = backend
        .begin_invocation(ProviderInvocationDescriptor::new("async-output"))
        .expect("invocation begins")
        .settle_provider(
            source_request("export:async_echo", StructuralValue::null()),
            settle_options(),
        )
        .expect("invocation settles")
        .finish()
        .expect("invocation finishes");
    assert_eq!(async_finished.output.record_count, 3);
    assert_eq!(
        async_finished
            .output
            .records
            .iter()
            .map(|record| record.text.as_str())
            .collect::<Vec<_>>(),
        vec![
            "async before await",
            "async after await",
            "async settled log"
        ]
    );

    let rejected = backend
        .begin_invocation(ProviderInvocationDescriptor::new("rejected-output"))
        .expect("invocation begins")
        .settle_provider(
            source_request("export:reject_with_output", StructuralValue::null()),
            settle_options(),
        )
        .expect("provider rejection still produces a terminal receipt")
        .finish()
        .expect("rejected invocation finishes");
    assert_eq!(backend.state(), BackendState::Ready);
    assert!(matches!(
        rejected.receipt,
        SettledProviderReceipt::Failed(_)
    ));
    assert_eq!(rejected.output.record_count, 2);
    assert_eq!(
        rejected
            .output
            .records
            .iter()
            .map(|record| record.text.as_str())
            .collect::<Vec<_>>(),
        vec!["reject before await", "reject async output"]
    );
}

#[test]
fn retained_backend_drop_output_policy_preserves_diagnostic_record_count() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    let finished = backend
        .begin_invocation(
            ProviderInvocationDescriptor::new("drop-output")
                .with_output_policy(InvocationOutputPolicy::Drop),
        )
        .expect("invocation begins")
        .settle_provider(
            source_request("export:echo", StructuralValue::null()),
            settle_options(),
        )
        .expect("invocation settles")
        .finish()
        .expect("invocation finishes");

    assert_eq!(backend.state(), BackendState::Ready);
    assert_eq!(finished.output.record_count, 1);
    assert!(finished.output.records.is_empty());
}

#[test]
fn retained_backend_post_settlement_output_poisons_before_reuse() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    let error = match backend
        .begin_invocation(ProviderInvocationDescriptor::new("late-output"))
        .expect("invocation begins")
        .settle_provider(
            source_request("export:late_after_settle", StructuralValue::null()),
            settle_options(),
        ) {
        Ok(_) => panic!("late output after the invocation ledger settles must poison backend"),
        Err(error) => error,
    };

    assert!(matches!(error, LibbunError::BackendState { .. }));
    assert_eq!(backend.state(), BackendState::Poisoned);
    let poison = backend
        .poison_diagnostic()
        .expect("late output poison diagnostic is retained");
    assert_eq!(
        poison.code,
        "retained_backend_post_settlement_host_output_poisoned"
    );
    assert_eq!(poison.output.len(), 1);
    assert_eq!(poison.output[0].text, "late output after settled receipt");
    let profile = poison
        .profile
        .as_ref()
        .expect("poison diagnostic must retain libbun-owned invocation profile");
    assert_eq!(profile.schema, "libbun.invocation_profile.ledger.v1");
    assert_eq!(profile.invocation_id, "late-output");
    let poison_phases = profile
        .spans
        .iter()
        .map(|span| span.phase.as_str())
        .collect::<Vec<_>>();
    assert_profile_contains(&poison_phases, "provider_call_settlement");
    assert_profile_contains(&poison_phases, "backend_poison");
    let reuse_error =
        match backend.begin_invocation(ProviderInvocationDescriptor::new("after-late-output")) {
            Ok(_) => panic!("poisoned backend must not be reused"),
            Err(error) => error,
        };
    assert!(matches!(reuse_error, LibbunError::BackendState { .. }));
}

#[test]
fn retained_backend_shutdown_consumes_backend_cleanly() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    backend.shutdown().expect("shutdown succeeds");
    assert_eq!(backend.state(), BackendState::Shutdown);
    backend.shutdown().expect("shutdown is idempotent");

    let error = match backend.begin_invocation(ProviderInvocationDescriptor::new("after-shutdown"))
    {
        Ok(_) => panic!("shutdown backend must not begin invocations"),
        Err(error) => error,
    };
    assert!(matches!(error, LibbunError::BackendState { .. }));
    assert!(
        error
            .to_string()
            .contains("retained_backend_begin_invocation_after_shutdown_forbidden")
    );
}

#[test]
fn retained_backend_dropped_invocation_lease_poisons_reuse() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    {
        let _lease = backend
            .begin_invocation(ProviderInvocationDescriptor::new("dropped-lease"))
            .expect("invocation begins");
    }

    assert_eq!(backend.state(), BackendState::Poisoned);
    let poison = backend
        .poison_diagnostic()
        .expect("dropped lease should leave poison diagnostic");
    assert_eq!(
        poison.code,
        "retained_backend_invocation_lease_dropped_without_settlement_poisoned"
    );
    let error = match backend.begin_invocation(ProviderInvocationDescriptor::new("after-poison")) {
        Ok(_) => panic!("poisoned backend cannot begin another invocation"),
        Err(error) => error,
    };
    assert!(matches!(error, LibbunError::BackendState { .. }));
}

#[test]
fn retained_backend_dropped_settled_outcome_poisons_reuse() {
    let mut backend =
        BunProviderBackend::<ConformanceRuntime>::open(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("retained backend opens");

    {
        let _outcome = backend
            .begin_invocation(ProviderInvocationDescriptor::new("dropped-outcome"))
            .expect("invocation begins")
            .settle_provider(
                source_request("export:echo", StructuralValue::null()),
                settle_options(),
            )
            .expect("invocation settles");
    }

    assert_eq!(backend.state(), BackendState::Poisoned);
    let poison = backend
        .poison_diagnostic()
        .expect("dropped outcome should leave poison diagnostic");
    assert_eq!(
        poison.code,
        "retained_backend_settled_outcome_dropped_without_finish_poisoned"
    );
}

#[test]
fn provider_diagnostics_emit_live_phase_events() {
    let events: Arc<Mutex<Vec<ProviderDiagnosticEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink_events = Arc::clone(&events);
    let mut host = BunHost::<ConformanceRuntime>::initialize_with_diagnostics(
        BunRuntimeConfig::new("test-host", "/tmp"),
        move |event| {
            sink_events.lock().expect("events lock").push(event);
        },
    )
    .expect("host initializes");

    let receipt = host
        .call_provider_until_settled(
            source_request("export:async_echo", StructuralValue(json!({ "ok": true }))),
            settle_options().with_call_id("diagnostic-call"),
        )
        .expect("provider succeeds");
    assert!(matches!(receipt, SettledProviderReceipt::Ready { .. }));

    let events = events.lock().expect("events lock");
    assert!(
        events
            .iter()
            .any(|event| event.kind == ProviderDiagnosticEventKind::CallStart)
    );
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::PhaseEnter
            && event.phase == ProviderDiagnosticPhase::ModuleLoad
    }));
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::PhaseExit
            && event.phase == ProviderDiagnosticPhase::ModuleLoad
    }));
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::PhaseEnter
            && event.phase == ProviderDiagnosticPhase::CallExport
    }));
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::PhaseEnter
            && event.phase == ProviderDiagnosticPhase::ResolveAsync
    }));
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::PhaseEnter
            && event.phase == ProviderDiagnosticPhase::PumpEventLoop
    }));
    assert!(events.iter().any(|event| {
        event.kind == ProviderDiagnosticEventKind::CallComplete
            && event.phase == ProviderDiagnosticPhase::Complete
    }));
    assert!(
        events
            .iter()
            .all(|event| event.call_id.0 == "diagnostic-call")
    );
}

#[test]
fn provider_diagnostics_snapshot_observes_unmatched_phase_during_call() {
    let mut host =
        BunHost::<ConformanceRuntime>::initialize(BunRuntimeConfig::new("test-host", "/tmp"))
            .expect("host initializes");
    let diagnostics = host.diagnostics_handle();

    let worker = std::thread::spawn(move || {
        host.call_provider_until_settled(
            source_request(
                "export:blocking_echo",
                StructuralValue(json!({ "ok": true })),
            ),
            settle_options().with_call_id("blocking-diagnostic-call"),
        )
        .expect("provider succeeds")
    });

    let mut observed = None;
    for _ in 0..50 {
        let snapshot = diagnostics.snapshot();
        if let Some(active_call) = snapshot.active_call {
            if active_call.unmatched_phase_enters.iter().any(|event| {
                event.kind == ProviderDiagnosticEventKind::PhaseEnter
                    && event.phase == ProviderDiagnosticPhase::CallExport
            }) {
                observed = Some(active_call);
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    let active_call = observed.expect("snapshot observes unmatched call_export phase");
    assert_eq!(active_call.call_id.0, "blocking-diagnostic-call");
    assert_eq!(
        active_call.latest_event.as_ref().map(|event| event.phase),
        Some(ProviderDiagnosticPhase::CallExport)
    );

    let receipt = worker.join().expect("worker joins");
    assert!(matches!(receipt, SettledProviderReceipt::Ready { .. }));
    let final_snapshot = diagnostics.snapshot();
    assert_eq!(final_snapshot.runtime_state, ProviderRuntimeState::Ready);
    assert!(final_snapshot.active_call.is_none());
}

#[test]
fn host_enforces_log_drop_policy_for_all_runtimes() {
    let mut config = BunRuntimeConfig::new("test-host", "/tmp");
    config.log = SinkPolicy::Drop;
    let host = BunHost::<ConformanceRuntime>::initialize(config).expect("host initializes");

    assert!(host.captured_output().is_empty());
}

#[test]
fn prepared_bundle_artifact_is_versioned_and_fingerprinted() {
    let mut modules = BTreeMap::new();
    modules.insert(
        "entry.mjs".to_string(),
        PreparedBundleModuleV1::source("export { value } from './lib/value.mjs';"),
    );
    modules.insert(
        "lib/value.mjs".to_string(),
        PreparedBundleModuleV1::source("export const value = 42;"),
    );

    let bundle = PreparedBundleV1::source_bundle("bundle-test", "entry.mjs", modules)
        .expect("bundle is valid");
    let bytes = bundle.to_bytes().expect("bundle serializes");
    let decoded = PreparedBundleV1::from_bytes(&bytes).expect("bundle decodes");

    assert_eq!(decoded, bundle);
    assert_eq!(decoded.fingerprint().expect("fingerprint hashes").len(), 71);
    decoded
        .validate_for_current_runtime("bundle-test")
        .expect("runtime metadata matches");
}

#[test]
fn prepared_bundle_rejects_unsafe_module_paths() {
    let mut modules = BTreeMap::new();
    modules.insert(
        "../escape.mjs".to_string(),
        PreparedBundleModuleV1::source("export const value = 1;"),
    );

    let error = PreparedBundleV1::source_bundle("bundle-test", "../escape.mjs", modules)
        .expect_err("unsafe path is rejected");
    assert!(matches!(error, LibbunError::ModuleLoad { .. }));
}
