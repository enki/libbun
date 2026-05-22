use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::time::Instant;

use crate::{
    BunEmbeddingRuntime, BunHost, BunRuntimeConfig, LibbunError, LibbunResult, OutputRecord,
    ProviderDiagnosticEvent, ProviderDiagnosticEventKind, ProviderDiagnosticPhase,
    ProviderExecutionOperation, ProviderRequest, ProviderRuntimeSnapshot, ProviderSettleOptions,
    SettledProviderReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendState {
    Ready,
    Active,
    Poisoned,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationOutputPolicy {
    Capture,
    Drop,
}

impl Default for InvocationOutputPolicy {
    fn default() -> Self {
        Self::Capture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationDiagnosticsPolicy {
    Snapshot,
}

impl Default for InvocationDiagnosticsPolicy {
    fn default() -> Self {
        Self::Snapshot
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LateOutputPolicy {
    Poison,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInvocationDescriptor {
    pub invocation_id: String,
    pub output_policy: InvocationOutputPolicy,
    pub diagnostics_policy: InvocationDiagnosticsPolicy,
}

impl ProviderInvocationDescriptor {
    pub fn new(invocation_id: impl Into<String>) -> Self {
        Self {
            invocation_id: invocation_id.into(),
            output_policy: InvocationOutputPolicy::Capture,
            diagnostics_policy: InvocationDiagnosticsPolicy::Snapshot,
        }
    }

    pub fn with_output_policy(mut self, output_policy: InvocationOutputPolicy) -> Self {
        self.output_policy = output_policy;
        self
    }

    pub fn with_diagnostics_policy(
        mut self,
        diagnostics_policy: InvocationDiagnosticsPolicy,
    ) -> Self {
        self.diagnostics_policy = diagnostics_policy;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationOutputLedger {
    pub invocation_id: String,
    pub records: Vec<OutputRecord>,
    pub record_count: usize,
    pub late_output_policy: LateOutputPolicy,
    pub late_output_count: usize,
    pub drain_failures: Vec<String>,
    pub diagnostics_snapshot: ProviderRuntimeSnapshot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationProfileLedger {
    pub schema: String,
    pub invocation_id: String,
    pub spans: Vec<InvocationProfileSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationProfileSpan {
    pub schema: String,
    pub phase: String,
    pub elapsed_ms: u64,
    pub started_after_ms: u64,
    pub completed_after_ms: u64,
    pub operation: Option<String>,
    pub counters: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedInvocation {
    pub invocation_id: String,
    pub receipt: SettledProviderReceipt,
    pub output: InvocationOutputLedger,
    pub diagnostics: ProviderRuntimeSnapshot,
    pub profile: InvocationProfileLedger,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendPoisonDiagnostic {
    pub code: String,
    pub message: String,
    pub state_before_poison: BackendState,
    pub invocation_id: Option<String>,
    pub output: Vec<OutputRecord>,
    pub diagnostics: ProviderRuntimeSnapshot,
    pub profile: Option<InvocationProfileLedger>,
}

pub struct BunProviderBackend<R: BunEmbeddingRuntime> {
    host: BunHost<R>,
    state: BackendState,
    startup_output: Vec<OutputRecord>,
    poison: Option<BackendPoisonDiagnostic>,
}

pub struct ProviderInvocationLease<'a, R: BunEmbeddingRuntime> {
    backend: Option<&'a mut BunProviderBackend<R>>,
    descriptor: ProviderInvocationDescriptor,
    profile: Option<InvocationProfileBuilder>,
    consumed: bool,
}

pub struct SettledInvocationOutcome<'a, R: BunEmbeddingRuntime> {
    backend: Option<&'a mut BunProviderBackend<R>>,
    descriptor: ProviderInvocationDescriptor,
    receipt: SettledProviderReceipt,
    output_records: Vec<OutputRecord>,
    diagnostics: ProviderRuntimeSnapshot,
    profile: Option<InvocationProfileBuilder>,
    consumed: bool,
}

struct InvocationProfileBuilder {
    invocation_id: String,
    invocation_started_at: Instant,
    spans: Vec<InvocationProfileSpan>,
}

impl<R: BunEmbeddingRuntime> BunProviderBackend<R> {
    pub fn open(config: BunRuntimeConfig) -> LibbunResult<Self> {
        let mut host = BunHost::<R>::initialize(config)?;
        let startup_output = host.drain_captured_output();
        Ok(Self {
            host,
            state: BackendState::Ready,
            startup_output,
            poison: None,
        })
    }

    pub fn begin_invocation(
        &mut self,
        descriptor: ProviderInvocationDescriptor,
    ) -> LibbunResult<ProviderInvocationLease<'_, R>> {
        let mut profile = InvocationProfileBuilder::new(descriptor.invocation_id.clone());
        let begin_started = Instant::now();
        validate_descriptor(&descriptor)?;
        match self.state {
            BackendState::Ready => {}
            BackendState::Active => {
                return Err(LibbunError::backend_state(
                    "retained_backend_begin_invocation_while_active_forbidden",
                    format!(
                        "provider invocation '{}' cannot begin because the retained backend is already Active; finish or poison the existing invocation lease first",
                        descriptor.invocation_id
                    ),
                ));
            }
            BackendState::Poisoned => {
                return Err(self.poisoned_error(
                    "retained_backend_begin_invocation_after_poison_forbidden",
                    &descriptor.invocation_id,
                ));
            }
            BackendState::Shutdown => {
                return Err(LibbunError::backend_state(
                    "retained_backend_begin_invocation_after_shutdown_forbidden",
                    format!(
                        "provider invocation '{}' cannot begin because the retained backend is Shutdown",
                        descriptor.invocation_id
                    ),
                ));
            }
        }

        let late_output = self.host.drain_captured_output();
        if !late_output.is_empty() {
            return Err(self.poison(
                "retained_backend_late_output_before_invocation_forbidden",
                format!(
                    "provider invocation '{}' observed {} output record(s) outside an active invocation; backend reuse is forbidden because output ownership is no longer provable",
                    descriptor.invocation_id,
                    late_output.len()
                ),
                Some(descriptor.invocation_id.clone()),
                late_output,
            ));
        }

        self.state = BackendState::Active;
        profile.record_duration(
            "backend_begin_invocation",
            begin_started,
            None,
            json!({
                "backendState": format!("{:?}", self.state),
                "outputPolicy": descriptor.output_policy,
                "diagnosticsPolicy": descriptor.diagnostics_policy,
            }),
        );
        Ok(ProviderInvocationLease {
            backend: Some(self),
            descriptor,
            profile: Some(profile),
            consumed: false,
        })
    }

    pub fn state(&self) -> BackendState {
        self.state
    }

    pub fn startup_output(&self) -> &[OutputRecord] {
        &self.startup_output
    }

    pub fn poison_diagnostic(&self) -> Option<&BackendPoisonDiagnostic> {
        self.poison.as_ref()
    }

    pub fn diagnostics_snapshot(&self) -> ProviderRuntimeSnapshot {
        self.host.diagnostics_handle().snapshot()
    }

    pub fn shutdown(&mut self) -> LibbunResult<()> {
        if self.state == BackendState::Shutdown {
            return Ok(());
        }
        let result = self.host.shutdown();
        match result {
            Ok(()) => {
                self.state = BackendState::Shutdown;
                Ok(())
            }
            Err(error) => Err(self.poison(
                "retained_backend_shutdown_failed_poisoned",
                format!(
                    "retained backend shutdown failed and left backend state uncertain: {error}"
                ),
                None,
                Vec::new(),
            )),
        }
    }

    fn finish_invocation(
        &mut self,
        descriptor: &ProviderInvocationDescriptor,
        output_records: &[OutputRecord],
    ) -> LibbunResult<ProviderRuntimeSnapshot> {
        if self.state != BackendState::Active {
            return Err(self.poison(
                "retained_backend_finish_without_active_invocation_forbidden",
                format!(
                    "provider invocation '{}' tried to finish while backend state was {:?}",
                    descriptor.invocation_id, self.state
                ),
                Some(descriptor.invocation_id.clone()),
                Vec::new(),
            ));
        }
        let late_output = self.host.drain_captured_output();
        if !late_output.is_empty() {
            return Err(self.poison(
                "retained_backend_late_output_during_finish_forbidden",
                format!(
                    "provider invocation '{}' finished with {} unowned output record(s) still retained by the backend; invocation output must be carried by the finished ledger",
                    descriptor.invocation_id,
                    late_output.len()
                ),
                Some(descriptor.invocation_id.clone()),
                late_output,
            ));
        }
        let diagnostics = self.diagnostics_snapshot();
        if descriptor.output_policy == InvocationOutputPolicy::Drop && !output_records.is_empty() {
            // Dropping is an explicit projection policy, not permission to leave
            // backend-owned records behind. The records were already drained.
        }
        self.state = BackendState::Ready;
        Ok(diagnostics)
    }

    fn poisoned_error(&self, code: &'static str, invocation_id: &str) -> LibbunError {
        let message = match self.poison.as_ref() {
            Some(poison) => format!(
                "provider invocation '{invocation_id}' cannot use poisoned retained backend; original poison `{}`: {}",
                poison.code, poison.message
            ),
            None => format!(
                "provider invocation '{invocation_id}' cannot use poisoned retained backend; poison diagnostic is missing"
            ),
        };
        LibbunError::backend_state(code, message)
    }

    fn poison(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        invocation_id: Option<String>,
        output: Vec<OutputRecord>,
    ) -> LibbunError {
        let code = code.into();
        let message = message.into();
        let state_before_poison = self.state;
        self.state = BackendState::Poisoned;
        self.poison = Some(BackendPoisonDiagnostic {
            code: code.clone(),
            message: message.clone(),
            state_before_poison,
            invocation_id,
            output,
            diagnostics: self.diagnostics_snapshot(),
            profile: None,
        });
        LibbunError::backend_state(code, message)
    }

    fn poison_with_profile(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        invocation_id: Option<String>,
        output: Vec<OutputRecord>,
        profile: InvocationProfileLedger,
    ) -> LibbunError {
        let error = self.poison(code, message, invocation_id, output);
        if let Some(poison) = self.poison.as_mut() {
            poison.profile = Some(profile);
        }
        error
    }
}

impl<'a, R: BunEmbeddingRuntime> ProviderInvocationLease<'a, R> {
    pub fn settle_provider(
        mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
    ) -> LibbunResult<SettledInvocationOutcome<'a, R>> {
        let backend = self
            .backend
            .take()
            .expect("provider invocation lease backend present until consumed");
        let mut profile = self
            .profile
            .take()
            .expect("provider invocation profile present until consumed");
        self.consumed = true;
        let settle_started = Instant::now();
        let receipt = match backend
            .host
            .call_provider_until_settled_for_invocation_ledger(request, options)
        {
            Ok(receipt) => receipt,
            Err(error) => {
                let output = backend.host.drain_captured_output();
                let diagnostics = backend.diagnostics_snapshot();
                profile.extend_provider_diagnostics(&diagnostics, None);
                profile.record_duration(
                    "backend_poison",
                    Instant::now(),
                    None,
                    json!({
                        "code": "retained_backend_provider_call_without_terminal_receipt_poisoned",
                        "outputRecordCount": output.len(),
                    }),
                );
                return Err(backend.poison_with_profile(
                    "retained_backend_provider_call_without_terminal_receipt_poisoned",
                    format!(
                        "provider invocation '{}' failed before libbun produced a terminal receipt; backend state is uncertain and cannot be reused: {error}",
                        self.descriptor.invocation_id
                    ),
                    Some(self.descriptor.invocation_id.clone()),
                    output,
                    profile.finish(),
                ));
            }
        };
        profile.record_duration(
            "provider_call_settlement",
            settle_started,
            receipt_operation(&receipt),
            json!({
                "status": receipt_status(&receipt),
                "outputRecordCount": receipt.output().len(),
            }),
        );
        let output_records = receipt.output().to_vec();
        let diagnostics = backend.diagnostics_snapshot();
        profile.extend_provider_diagnostics(&diagnostics, receipt_call_id(&receipt));
        let retained_output = backend.host.drain_captured_output();
        if !retained_output.is_empty() {
            profile.record_duration(
                "backend_poison",
                Instant::now(),
                receipt_operation(&receipt),
                json!({
                    "code": "retained_backend_post_settlement_host_output_poisoned",
                    "outputRecordCount": retained_output.len(),
                }),
            );
            return Err(backend.poison_with_profile(
                "retained_backend_post_settlement_host_output_poisoned",
                format!(
                    "provider invocation '{}' produced {} output record(s) after the invocation ledger had settled; backend state is uncertain and cannot be reused",
                    self.descriptor.invocation_id,
                    retained_output.len()
                ),
                Some(self.descriptor.invocation_id.clone()),
                retained_output,
                profile.finish(),
            ));
        }
        Ok(SettledInvocationOutcome {
            backend: Some(backend),
            descriptor: self.descriptor.clone(),
            receipt,
            output_records,
            diagnostics,
            profile: Some(profile),
            consumed: false,
        })
    }
}

impl<R: BunEmbeddingRuntime> Drop for ProviderInvocationLease<'_, R> {
    fn drop(&mut self) {
        if self.consumed {
            return;
        }
        if let Some(backend) = self.backend.as_deref_mut() {
            let output = backend.host.drain_captured_output();
            let profile = self.profile.take().map(|mut profile| {
                profile.record_duration(
                    "backend_poison",
                    Instant::now(),
                    None,
                    json!({
                        "code": "retained_backend_invocation_lease_dropped_without_settlement_poisoned",
                        "outputRecordCount": output.len(),
                    }),
                );
                profile.finish()
            });
            let code = "retained_backend_invocation_lease_dropped_without_settlement_poisoned";
            let message = format!(
                "provider invocation '{}' lease was dropped without settlement; retained backend cannot prove provider state or output ownership",
                self.descriptor.invocation_id
            );
            let _ = match profile {
                Some(profile) => backend.poison_with_profile(
                    code,
                    message,
                    Some(self.descriptor.invocation_id.clone()),
                    output,
                    profile,
                ),
                None => backend.poison(
                    code,
                    message,
                    Some(self.descriptor.invocation_id.clone()),
                    output,
                ),
            };
        }
    }
}

impl<'a, R: BunEmbeddingRuntime> SettledInvocationOutcome<'a, R> {
    pub fn finish(mut self) -> LibbunResult<FinishedInvocation> {
        let backend = self
            .backend
            .take()
            .expect("settled invocation outcome backend present until finished");
        let mut profile = self
            .profile
            .take()
            .expect("settled invocation profile present until finished");
        let backend_finish_started = Instant::now();
        let finish_diagnostics =
            match backend.finish_invocation(&self.descriptor, self.output_records.as_slice()) {
                Ok(diagnostics) => diagnostics,
                Err(error) => {
                    profile.record_duration(
                        "backend_poison",
                        Instant::now(),
                        None,
                        json!({
                            "code": "retained_backend_finish_invocation_failed_poisoned",
                            "outputRecordCount": self.output_records.len(),
                        }),
                    );
                    if let Some(poison) = backend.poison.as_mut() {
                        poison.profile = Some(profile.finish());
                    }
                    self.consumed = true;
                    return Err(error);
                }
            };
        profile.record_duration(
            "backend_finish_invocation",
            backend_finish_started,
            None,
            json!({
                "backendState": format!("{:?}", backend.state()),
            }),
        );
        let ledger_finish_started = Instant::now();
        let records = if self.descriptor.output_policy == InvocationOutputPolicy::Capture {
            self.output_records.clone()
        } else {
            Vec::new()
        };
        let ledger = InvocationOutputLedger {
            invocation_id: self.descriptor.invocation_id.clone(),
            record_count: self.output_records.len(),
            records,
            late_output_policy: LateOutputPolicy::Poison,
            late_output_count: 0,
            drain_failures: Vec::new(),
            diagnostics_snapshot: finish_diagnostics.clone(),
        };
        profile.record_duration(
            "output_ledger_finish",
            ledger_finish_started,
            None,
            json!({
                "recordCount": ledger.record_count,
                "projectedRecordCount": ledger.records.len(),
                "lateOutputCount": ledger.late_output_count,
                "drainFailureCount": ledger.drain_failures.len(),
            }),
        );
        let profile = profile.finish();
        self.consumed = true;
        Ok(FinishedInvocation {
            invocation_id: self.descriptor.invocation_id.clone(),
            receipt: self.receipt.clone(),
            output: ledger,
            diagnostics: self.diagnostics.clone(),
            profile,
        })
    }
}

impl<R: BunEmbeddingRuntime> Drop for SettledInvocationOutcome<'_, R> {
    fn drop(&mut self) {
        if self.consumed {
            return;
        }
        if let Some(backend) = self.backend.as_deref_mut() {
            let output = backend.host.drain_captured_output();
            let profile = self.profile.take().map(|mut profile| {
                profile.record_duration(
                    "backend_poison",
                    Instant::now(),
                    None,
                    json!({
                        "code": "retained_backend_settled_outcome_dropped_without_finish_poisoned",
                        "outputRecordCount": output.len(),
                    }),
                );
                profile.finish()
            });
            let code = "retained_backend_settled_outcome_dropped_without_finish_poisoned";
            let message = format!(
                "provider invocation '{}' settled but its outcome was dropped without finish(); retained backend cannot prove final output ledger ownership",
                self.descriptor.invocation_id
            );
            let _ = match profile {
                Some(profile) => backend.poison_with_profile(
                    code,
                    message,
                    Some(self.descriptor.invocation_id.clone()),
                    output,
                    profile,
                ),
                None => backend.poison(
                    code,
                    message,
                    Some(self.descriptor.invocation_id.clone()),
                    output,
                ),
            };
        }
    }
}

fn validate_descriptor(descriptor: &ProviderInvocationDescriptor) -> LibbunResult<()> {
    if descriptor.invocation_id.trim().is_empty() {
        return Err(LibbunError::backend_state(
            "retained_backend_invocation_id_empty_forbidden",
            "provider invocation descriptor must carry a non-empty invocation id",
        ));
    }
    Ok(())
}

impl InvocationProfileBuilder {
    fn new(invocation_id: String) -> Self {
        Self {
            invocation_id,
            invocation_started_at: Instant::now(),
            spans: Vec::new(),
        }
    }

    fn record_duration(
        &mut self,
        phase: impl Into<String>,
        started_at: Instant,
        operation: Option<String>,
        counters: Value,
    ) {
        let started_after_ms = started_at
            .saturating_duration_since(self.invocation_started_at)
            .as_millis() as u64;
        let completed_after_ms = self.invocation_started_at.elapsed().as_millis() as u64;
        self.spans.push(InvocationProfileSpan {
            schema: "libbun.invocation_profile.span.v1".to_owned(),
            phase: phase.into(),
            elapsed_ms: started_at.elapsed().as_millis() as u64,
            started_after_ms,
            completed_after_ms,
            operation,
            counters,
        });
    }

    fn record_diagnostic_span(
        &mut self,
        phase: impl Into<String>,
        enter: &ProviderDiagnosticEvent,
        exit: &ProviderDiagnosticEvent,
    ) {
        let started_after_ms = enter.elapsed_ms;
        let completed_after_ms = exit.elapsed_ms;
        self.spans.push(InvocationProfileSpan {
            schema: "libbun.invocation_profile.span.v1".to_owned(),
            phase: phase.into(),
            elapsed_ms: completed_after_ms.saturating_sub(started_after_ms),
            started_after_ms,
            completed_after_ms,
            operation: Some(provider_operation_name(exit.operation).to_owned()),
            counters: json!({
                "diagnosticPhase": provider_diagnostic_phase_name(exit.phase),
                "diagnosticOperation": provider_operation_name(exit.operation),
                "pendingAsyncTaskCount": exit.pending_async_task_count,
                "capturedOutputRecordCount": exit.captured_output_record_count,
                "runtimeInstanceId": exit.runtime_instance_id,
                "libbunVersion": exit.libbun_version,
                "libbunAbiVersion": exit.libbun_abi_version,
                "bunRevision": exit.bun_revision,
            }),
        });
    }

    fn extend_provider_diagnostics(
        &mut self,
        diagnostics: &ProviderRuntimeSnapshot,
        call_id: Option<&str>,
    ) {
        let mut enters: BTreeMap<u64, &ProviderDiagnosticEvent> = BTreeMap::new();
        for event in diagnostics.recent_events.iter() {
            if let Some(call_id) = call_id
                && event.call_id.0 != call_id
            {
                continue;
            }
            match event.kind {
                ProviderDiagnosticEventKind::PhaseEnter if event.span_id != 0 => {
                    enters.insert(event.span_id, event);
                }
                ProviderDiagnosticEventKind::PhaseExit if event.span_id != 0 => {
                    if let Some(enter) = enters.get(&event.span_id) {
                        self.record_diagnostic_span(
                            invocation_profile_phase_name(event.phase, event.operation),
                            enter,
                            event,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn finish(self) -> InvocationProfileLedger {
        InvocationProfileLedger {
            schema: "libbun.invocation_profile.ledger.v1".to_owned(),
            invocation_id: self.invocation_id,
            spans: self.spans,
        }
    }
}

fn receipt_status(receipt: &SettledProviderReceipt) -> &'static str {
    match receipt {
        SettledProviderReceipt::Ready { .. } => "ready",
        SettledProviderReceipt::Failed(_) => "failed",
    }
}

fn receipt_operation(receipt: &SettledProviderReceipt) -> Option<String> {
    match receipt {
        SettledProviderReceipt::Ready { settlement, .. } => {
            Some(provider_operation_name(settlement.operation).to_owned())
        }
        SettledProviderReceipt::Failed(failure) => {
            Some(provider_operation_name(failure.operation).to_owned())
        }
    }
}

fn receipt_call_id(receipt: &SettledProviderReceipt) -> Option<&str> {
    match receipt {
        SettledProviderReceipt::Ready { settlement, .. } => settlement
            .call_id
            .as_ref()
            .map(|call_id| call_id.0.as_str()),
        SettledProviderReceipt::Failed(failure) => {
            failure.call_id.as_ref().map(|call_id| call_id.0.as_str())
        }
    }
}

fn invocation_profile_phase_name(
    phase: ProviderDiagnosticPhase,
    operation: ProviderExecutionOperation,
) -> &'static str {
    match (phase, operation) {
        (ProviderDiagnosticPhase::ModuleLoad, _) => "provider_module_load",
        (ProviderDiagnosticPhase::CallExport, ProviderExecutionOperation::ProviderExportLookup) => {
            "provider_export_lookup"
        }
        (ProviderDiagnosticPhase::CallExport, _) => "provider_call_dispatch",
        (ProviderDiagnosticPhase::ResolveAsync, _) => "provider_settlement",
        (ProviderDiagnosticPhase::PumpEventLoop, _) => "provider_event_loop_pump",
        (ProviderDiagnosticPhase::DrainOutput, _) => "output_drain",
        (ProviderDiagnosticPhase::Shutdown, _) => "backend_shutdown",
        (ProviderDiagnosticPhase::DeadlineElapsed, _) => "provider_deadline_elapsed",
        (ProviderDiagnosticPhase::Complete, _) => "provider_complete",
        (ProviderDiagnosticPhase::RuntimeInitialize, _) => "runtime_initialize",
    }
}

fn provider_diagnostic_phase_name(phase: ProviderDiagnosticPhase) -> &'static str {
    match phase {
        ProviderDiagnosticPhase::RuntimeInitialize => "runtime_initialize",
        ProviderDiagnosticPhase::ModuleLoad => "module_load",
        ProviderDiagnosticPhase::CallExport => "call_export",
        ProviderDiagnosticPhase::ResolveAsync => "resolve_async",
        ProviderDiagnosticPhase::PumpEventLoop => "pump_event_loop",
        ProviderDiagnosticPhase::DrainOutput => "drain_output",
        ProviderDiagnosticPhase::Shutdown => "shutdown",
        ProviderDiagnosticPhase::DeadlineElapsed => "deadline_elapsed",
        ProviderDiagnosticPhase::Complete => "complete",
    }
}

fn provider_operation_name(operation: ProviderExecutionOperation) -> &'static str {
    match operation {
        ProviderExecutionOperation::RuntimeInitialize => "runtime_initialize",
        ProviderExecutionOperation::AdapterModuleLoad => "adapter_module_load",
        ProviderExecutionOperation::ProviderModuleImport => "provider_module_import",
        ProviderExecutionOperation::ProviderExportLookup => "provider_export_lookup",
        ProviderExecutionOperation::ProviderFactoryValidate => "provider_factory_validate",
        ProviderExecutionOperation::ProviderFactoryInvoke => "provider_factory_invoke",
        ProviderExecutionOperation::ProviderCallableValidate => "provider_callable_validate",
        ProviderExecutionOperation::ProviderCallableInvoke => "provider_callable_invoke",
        ProviderExecutionOperation::ProviderPromiseSettle => "provider_promise_settle",
        ProviderExecutionOperation::ProviderDeadlineElapsed => "provider_deadline_elapsed",
    }
}
