use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
#[cfg(test)]
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender};
use std::sync::{Arc, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::{
    BunEmbeddingRuntime, BunHost, BunRuntimeConfig, LibbunError, LibbunResult, OutputRecord,
    ProviderCallResult, ProviderContractIdentity, ProviderDomainClass, ProviderRequest,
    ProviderSettleOptions, SettledProviderReceipt, StructuralValue,
};

#[cfg(test)]
static NEXT_SELECTION_BRAND: AtomicU64 = AtomicU64::new(1);
static DURABLE_REAPER: OnceLock<Result<Sender<WorkerCustody>, String>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoredSettlementKind {
    Fulfilled,
    Rejected,
}

pub struct AuthoredSettlementCargo {
    kind: AuthoredSettlementKind,
    bytes: Vec<u8>,
    output: Vec<OutputRecord>,
}

impl AuthoredSettlementCargo {
    pub fn kind(&self) -> AuthoredSettlementKind {
        self.kind
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn output(&self) -> &[OutputRecord] {
        &self.output
    }
}

impl fmt::Debug for AuthoredSettlementCargo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoredSettlementCargo")
            .field("kind", &self.kind)
            .field("byte_len", &self.bytes.len())
            .field("output_record_count", &self.output.len())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MechanicalFaultKind {
    Admission,
    DeadlineConstruction,
    Dispatch,
    ProviderPreparation,
    OutputQuiescence,
    WorkerTermination,
    Shutdown,
    SupervisorUnwind,
}

pub struct MechanicalFault {
    kind: MechanicalFaultKind,
    code: &'static str,
    message: String,
}

impl MechanicalFault {
    fn new(kind: MechanicalFaultKind, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> MechanicalFaultKind {
        self.kind
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Debug for MechanicalFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MechanicalFault")
            .field("kind", &self.kind)
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}

#[derive(Clone)]
pub struct DriveInterrupt {
    requested: Arc<AtomicBool>,
}

impl DriveInterrupt {
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

impl fmt::Debug for DriveInterrupt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DriveInterrupt")
            .field("requested", &self.is_requested())
            .finish()
    }
}

pub struct DriveControl {
    timeout: Duration,
    deadline: Instant,
    interrupt: DriveInterrupt,
}

impl DriveControl {
    pub fn deadline_after(timeout: Duration) -> Result<Self, MechanicalFault> {
        let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
            MechanicalFault::new(
                MechanicalFaultKind::DeadlineConstruction,
                "retained_prepared_export_deadline_overflow",
                "the prepared-export deadline cannot be represented by the monotonic clock",
            )
        })?;
        Ok(Self {
            timeout,
            deadline,
            interrupt: DriveInterrupt {
                requested: Arc::new(AtomicBool::new(false)),
            },
        })
    }

    pub fn interrupt(&self) -> DriveInterrupt {
        self.interrupt.clone()
    }

    fn remaining(&self) -> Duration {
        self.deadline.saturating_duration_since(Instant::now())
    }

    fn selection(&self) -> ControlSelection {
        if Instant::now() >= self.deadline {
            ControlSelection::Deadline
        } else if self.interrupt.is_requested() {
            ControlSelection::Cancelled
        } else {
            ControlSelection::Continue
        }
    }

    fn retirement_timeout(&self) -> Duration {
        Duration::from_secs(1)
    }
}

impl fmt::Debug for DriveControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DriveControl")
            .field("timeout", &self.timeout)
            .field("interrupt_requested", &self.interrupt.is_requested())
            .finish()
    }
}

pub struct ShutdownControl {
    timeout: Duration,
}

impl ShutdownControl {
    pub fn deadline_after(timeout: Duration) -> Result<Self, MechanicalFault> {
        Instant::now().checked_add(timeout).ok_or_else(|| {
            MechanicalFault::new(
                MechanicalFaultKind::DeadlineConstruction,
                "retained_backend_shutdown_deadline_overflow",
                "the retained-backend shutdown deadline cannot be represented by the monotonic clock",
            )
        })?;
        Ok(Self { timeout })
    }
}

impl fmt::Debug for ShutdownControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ShutdownControl")
            .field("timeout", &self.timeout)
            .finish()
    }
}

pub struct SelectedProviderPackage {
    brand: u64,
    contract: ProviderContractIdentity,
    domain: ProviderDomainClass,
    module: crate::BunModuleSpec,
    export: String,
}

pub struct ProviderInvocation {
    brand: u64,
    input: StructuralValue,
    options: ProviderSettleOptions,
}

pub struct BunProviderBackend {
    worker: Option<WorkerCustody>,
}

pub struct PreparedExport {
    worker: Option<WorkerCustody>,
    request: Option<ProviderRequest>,
    options: Option<ProviderSettleOptions>,
}

pub enum MechanicalTerminal {
    Cargo(CargoTerminal),
    Cancelled(CancelledTerminal),
    DeadlineElapsed(DeadlineElapsedTerminal),
    MechanicalFault(MechanicalFaultTerminal),
}

pub struct CargoTerminal {
    cargo: AuthoredSettlementCargo,
    continuation: Option<Continuation>,
}

pub struct CancelledTerminal {
    continuation: Option<Continuation>,
}

pub struct DeadlineElapsedTerminal {
    continuation: Option<Continuation>,
}

pub struct MechanicalFaultTerminal {
    fault: MechanicalFault,
    continuation: Option<Continuation>,
}

pub enum BackendShutdownTerminal {
    Complete,
    MechanicalFault(MechanicalFault),
}

enum Continuation {
    Ready(Option<WorkerCustody>),
    Restartable(WorkerFactory),
}

#[derive(Clone)]
struct WorkerFactory {
    config: BunRuntimeConfig,
    spawn: fn(BunRuntimeConfig) -> LibbunResult<WorkerCustody>,
}

struct WorkerCustody {
    commands: Sender<WorkerCommand>,
    join: Option<JoinHandle<()>>,
    factory: WorkerFactory,
}

enum WorkerCommand {
    Drive {
        request: ProviderRequest,
        options: ProviderSettleOptions,
        interrupt: Arc<AtomicBool>,
        response: SyncSender<WorkerDriveResponse>,
    },
    Shutdown {
        response: Option<SyncSender<LibbunResult<()>>>,
    },
}

struct WorkerDriveResponse {
    result: LibbunResult<SettledProviderReceipt>,
    unowned_output: Vec<OutputRecord>,
}

enum ControlSelection {
    Continue,
    Cancelled,
    Deadline,
}

enum RetirementOutcome {
    Complete,
    Fault(MechanicalFault),
    Adopted(MechanicalFault),
}

impl BunProviderBackend {
    pub fn open<R>(config: BunRuntimeConfig) -> LibbunResult<Self>
    where
        R: BunEmbeddingRuntime + 'static,
    {
        ensure_durable_reaper().map_err(|message| {
            LibbunError::initialize(format!(
                "retained prepared-export durable reaper initialization failed: {message}"
            ))
        })?;
        let worker = spawn_worker::<R>(config)?;
        Ok(Self {
            worker: Some(worker),
        })
    }

    pub fn prepare(
        mut self,
        package: SelectedProviderPackage,
        invocation: ProviderInvocation,
    ) -> Result<PreparedExport, MechanicalTerminal> {
        if package.brand != invocation.brand {
            let continuation = self
                .worker
                .take()
                .map(|worker| Continuation::Ready(Some(worker)));
            return Err(MechanicalTerminal::MechanicalFault(
                MechanicalFaultTerminal {
                    fault: MechanicalFault::new(
                        MechanicalFaultKind::Admission,
                        "retained_prepared_export_selection_brand_mismatch",
                        "the selected provider package and invocation were not minted by the same selection",
                    ),
                    continuation,
                },
            ));
        }
        let request = ProviderRequest {
            contract: package.contract,
            domain: package.domain,
            module: package.module,
            export: package.export,
            input: invocation.input,
        };
        Ok(PreparedExport {
            worker: self.worker.take(),
            request: Some(request),
            options: Some(invocation.options),
        })
    }

    pub fn shutdown(mut self, control: ShutdownControl) -> BackendShutdownTerminal {
        let Some(worker) = self.worker.take() else {
            return BackendShutdownTerminal::Complete;
        };
        shutdown_worker(worker, control)
    }
}

impl Drop for BunProviderBackend {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            adopt_for_disposal(worker);
        }
    }
}

impl PreparedExport {
    pub fn drive(mut self, control: DriveControl) -> MechanicalTerminal {
        let worker = self
            .worker
            .take()
            .expect("prepared export retains worker custody until consumed");
        let request = self
            .request
            .take()
            .expect("prepared export retains selected request until consumed");
        let options = self
            .options
            .take()
            .expect("prepared export retains settle options until consumed");

        match control.selection() {
            ControlSelection::Deadline => {
                return MechanicalTerminal::DeadlineElapsed(DeadlineElapsedTerminal {
                    continuation: Some(Continuation::Ready(Some(worker))),
                });
            }
            ControlSelection::Cancelled => {
                return MechanicalTerminal::Cancelled(CancelledTerminal {
                    continuation: Some(Continuation::Ready(Some(worker))),
                });
            }
            ControlSelection::Continue => {}
        }

        let result = catch_unwind(AssertUnwindSafe(|| {
            drive_worker(worker, request, options, control)
        }));
        match result {
            Ok(terminal) => terminal,
            Err(_) => MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
                fault: MechanicalFault::new(
                    MechanicalFaultKind::SupervisorUnwind,
                    "retained_prepared_export_supervisor_unwind",
                    "the prepared-export supervisor unwound while it owned the drive",
                ),
                continuation: None,
            }),
        }
    }

    pub fn shutdown(mut self, control: ShutdownControl) -> BackendShutdownTerminal {
        self.request.take();
        self.options.take();
        let Some(worker) = self.worker.take() else {
            return BackendShutdownTerminal::Complete;
        };
        shutdown_worker(worker, control)
    }
}

impl Drop for PreparedExport {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            adopt_for_disposal(worker);
        }
    }
}

impl MechanicalTerminal {
    pub fn cargo(&self) -> Option<&AuthoredSettlementCargo> {
        match self {
            Self::Cargo(terminal) => Some(&terminal.cargo),
            _ => None,
        }
    }

    pub fn fault(&self) -> Option<&MechanicalFault> {
        match self {
            Self::MechanicalFault(terminal) => Some(&terminal.fault),
            _ => None,
        }
    }

    pub fn prepare_next(
        mut self,
        package: SelectedProviderPackage,
        invocation: ProviderInvocation,
    ) -> Result<PreparedExport, Self> {
        let Some(continuation) = self.take_continuation() else {
            return Err(self);
        };
        match continuation.into_backend() {
            Ok(backend) => backend.prepare(package, invocation),
            Err(fault) => Err(MechanicalTerminal::MechanicalFault(
                MechanicalFaultTerminal {
                    fault,
                    continuation: None,
                },
            )),
        }
    }

    pub fn shutdown(mut self, control: ShutdownControl) -> BackendShutdownTerminal {
        match self.take_continuation() {
            Some(continuation) => continuation.shutdown(control),
            None => BackendShutdownTerminal::Complete,
        }
    }

    fn take_continuation(&mut self) -> Option<Continuation> {
        match self {
            Self::Cargo(terminal) => terminal.continuation.take(),
            Self::Cancelled(terminal) => terminal.continuation.take(),
            Self::DeadlineElapsed(terminal) => terminal.continuation.take(),
            Self::MechanicalFault(terminal) => terminal.continuation.take(),
        }
    }
}

impl Continuation {
    fn into_backend(mut self) -> Result<BunProviderBackend, MechanicalFault> {
        match &mut self {
            Self::Ready(worker) => {
                let worker = worker.take().ok_or_else(|| {
                    MechanicalFault::new(
                        MechanicalFaultKind::WorkerTermination,
                        "retained_backend_ready_continuation_empty",
                        "the retained backend ready continuation had already been consumed",
                    )
                })?;
                Ok(BunProviderBackend {
                    worker: Some(worker),
                })
            }
            Self::Restartable(factory) => {
                let factory = factory.clone();
                (factory.spawn)(factory.config.clone())
                    .map(|worker| BunProviderBackend {
                        worker: Some(worker),
                    })
                    .map_err(|error| {
                        MechanicalFault::new(
                            MechanicalFaultKind::Admission,
                            "retained_backend_restart_failed",
                            error.to_string(),
                        )
                    })
            }
        }
    }

    fn shutdown(mut self, control: ShutdownControl) -> BackendShutdownTerminal {
        match &mut self {
            Self::Ready(worker) => match worker.take() {
                Some(worker) => shutdown_worker(worker, control),
                None => BackendShutdownTerminal::MechanicalFault(MechanicalFault::new(
                    MechanicalFaultKind::WorkerTermination,
                    "retained_backend_ready_continuation_empty",
                    "the retained backend ready continuation had already been consumed",
                )),
            },
            Self::Restartable(_) => BackendShutdownTerminal::Complete,
        }
    }
}

impl Drop for Continuation {
    fn drop(&mut self) {
        if let Self::Ready(worker) = self
            && let Some(worker) = worker.take()
        {
            adopt_for_disposal(worker);
        }
    }
}

impl fmt::Debug for MechanicalTerminal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cargo(terminal) => formatter
                .debug_tuple("Cargo")
                .field(&terminal.cargo)
                .finish(),
            Self::Cancelled(_) => formatter.write_str("Cancelled"),
            Self::DeadlineElapsed(_) => formatter.write_str("DeadlineElapsed"),
            Self::MechanicalFault(terminal) => formatter
                .debug_tuple("MechanicalFault")
                .field(&terminal.fault)
                .finish(),
        }
    }
}

impl fmt::Debug for BackendShutdownTerminal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => formatter.write_str("Complete"),
            Self::MechanicalFault(fault) => formatter
                .debug_tuple("MechanicalFault")
                .field(fault)
                .finish(),
        }
    }
}

fn drive_worker(
    worker: WorkerCustody,
    request: ProviderRequest,
    mut options: ProviderSettleOptions,
    control: DriveControl,
) -> MechanicalTerminal {
    options.deadline.deadline_ms = control
        .remaining()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    let (response, receive) = mpsc::sync_channel(1);
    let command = WorkerCommand::Drive {
        request,
        options,
        interrupt: Arc::clone(&control.interrupt.requested),
        response,
    };
    if worker.commands.send(command).is_err() {
        return terminal_after_worker_disconnect(worker);
    }

    let response = match receive.recv_timeout(control.remaining()) {
        Ok(response) => response,
        Err(RecvTimeoutError::Timeout) => {
            control.interrupt.request();
            adopt_for_disposal(worker);
            return MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
                fault: MechanicalFault::new(
                    MechanicalFaultKind::WorkerTermination,
                    "retained_prepared_export_deadline_retirement_adopted",
                    "the drive exceeded its foreground deadline; unresolved runtime custody was adopted before this terminal was published",
                ),
                continuation: None,
            });
        }
        Err(RecvTimeoutError::Disconnected) => {
            return terminal_after_worker_disconnect(worker);
        }
    };

    let selected = control.selection();
    if !response.unowned_output.is_empty() {
        let factory = worker.factory.clone();
        let retirement = retire_worker(worker, control.retirement_timeout());
        return fault_after_retirement(
            MechanicalFault::new(
                MechanicalFaultKind::OutputQuiescence,
                "retained_prepared_export_unowned_output_after_terminal",
                format!(
                    "{} output record(s) remained outside the terminal cargo ledger",
                    response.unowned_output.len()
                ),
            ),
            factory,
            retirement,
        );
    }

    match selected {
        ControlSelection::Deadline => {
            let factory = worker.factory.clone();
            match retire_worker(worker, control.retirement_timeout()) {
                RetirementOutcome::Complete => {
                    MechanicalTerminal::DeadlineElapsed(DeadlineElapsedTerminal {
                        continuation: Some(Continuation::Restartable(factory)),
                    })
                }
                retirement => fault_after_retirement(
                    MechanicalFault::new(
                        MechanicalFaultKind::WorkerTermination,
                        "retained_prepared_export_deadline_retirement_failed",
                        "deadline was selected but terminal worker retirement did not complete cleanly",
                    ),
                    factory,
                    retirement,
                ),
            }
        }
        ControlSelection::Cancelled => {
            let factory = worker.factory.clone();
            match retire_worker(worker, control.retirement_timeout()) {
                RetirementOutcome::Complete => MechanicalTerminal::Cancelled(CancelledTerminal {
                    continuation: Some(Continuation::Restartable(factory)),
                }),
                retirement => fault_after_retirement(
                    MechanicalFault::new(
                        MechanicalFaultKind::WorkerTermination,
                        "retained_prepared_export_cancel_retirement_failed",
                        "cancellation was selected but terminal worker retirement did not complete cleanly",
                    ),
                    factory,
                    retirement,
                ),
            }
        }
        ControlSelection::Continue => match response.result {
            Ok(SettledProviderReceipt::Ready { result, output, .. }) => {
                match authored_cargo(result, output) {
                    Ok(cargo) => MechanicalTerminal::Cargo(CargoTerminal {
                        cargo,
                        continuation: Some(Continuation::Ready(Some(worker))),
                    }),
                    Err(fault) => {
                        let factory = worker.factory.clone();
                        let retirement = retire_worker(worker, control.retirement_timeout());
                        fault_after_retirement(fault, factory, retirement)
                    }
                }
            }
            Ok(SettledProviderReceipt::Failed(failure))
                if failure.operation
                    == crate::ProviderExecutionOperation::ProviderDeadlineElapsed =>
            {
                let factory = worker.factory.clone();
                match retire_worker(worker, control.retirement_timeout()) {
                    RetirementOutcome::Complete => {
                        MechanicalTerminal::DeadlineElapsed(DeadlineElapsedTerminal {
                            continuation: Some(Continuation::Restartable(factory)),
                        })
                    }
                    retirement => fault_after_retirement(
                        MechanicalFault::new(
                            MechanicalFaultKind::WorkerTermination,
                            "retained_prepared_export_runtime_deadline_retirement_failed",
                            "the runtime reported deadline but retirement did not complete cleanly",
                        ),
                        factory,
                        retirement,
                    ),
                }
            }
            Ok(SettledProviderReceipt::Failed(failure)) => {
                let factory = worker.factory.clone();
                let fault = MechanicalFault::new(
                    MechanicalFaultKind::ProviderPreparation,
                    "retained_prepared_export_provider_preparation_failed",
                    failure.js_error_message.unwrap_or_else(|| {
                        "provider preparation failed without a JavaScript diagnostic".to_owned()
                    }),
                );
                let retirement = retire_worker(worker, control.retirement_timeout());
                fault_after_retirement(fault, factory, retirement)
            }
            Err(error) if is_typed_interrupt(&error) => {
                let factory = worker.factory.clone();
                match retire_worker(worker, control.retirement_timeout()) {
                    RetirementOutcome::Complete => {
                        MechanicalTerminal::Cancelled(CancelledTerminal {
                            continuation: Some(Continuation::Restartable(factory)),
                        })
                    }
                    retirement => fault_after_retirement(
                        MechanicalFault::new(
                            MechanicalFaultKind::WorkerTermination,
                            "retained_prepared_export_interrupt_retirement_failed",
                            "the runtime observed cancellation but retirement did not complete cleanly",
                        ),
                        factory,
                        retirement,
                    ),
                }
            }
            Err(error) => {
                let factory = worker.factory.clone();
                let fault = MechanicalFault::new(
                    MechanicalFaultKind::Dispatch,
                    "retained_prepared_export_dispatch_failed",
                    error.to_string(),
                );
                let retirement = retire_worker(worker, control.retirement_timeout());
                fault_after_retirement(fault, factory, retirement)
            }
        },
    }
}

fn authored_cargo(
    result: ProviderCallResult,
    output: Vec<OutputRecord>,
) -> Result<AuthoredSettlementCargo, MechanicalFault> {
    #[derive(Serialize)]
    #[serde(rename_all = "snake_case", tag = "kind", content = "cargo")]
    enum Envelope {
        Fulfilled(StructuralValue),
        Rejected(crate::ProviderError),
    }

    let (kind, envelope) = match result {
        ProviderCallResult::Ok(value) => (
            AuthoredSettlementKind::Fulfilled,
            Envelope::Fulfilled(value),
        ),
        ProviderCallResult::Err(error) => {
            (AuthoredSettlementKind::Rejected, Envelope::Rejected(error))
        }
    };
    let bytes = serde_json::to_vec(&envelope).map_err(|error| {
        MechanicalFault::new(
            MechanicalFaultKind::ProviderPreparation,
            "retained_prepared_export_authored_cargo_encode_failed",
            error.to_string(),
        )
    })?;
    if bytes.is_empty() {
        return Err(MechanicalFault::new(
            MechanicalFaultKind::ProviderPreparation,
            "retained_prepared_export_authored_cargo_empty",
            "authored settlement cargo encoded to an empty byte sequence",
        ));
    }
    Ok(AuthoredSettlementCargo {
        kind,
        bytes,
        output,
    })
}

fn fault_after_retirement(
    mut fault: MechanicalFault,
    factory: WorkerFactory,
    retirement: RetirementOutcome,
) -> MechanicalTerminal {
    let continuation = match retirement {
        RetirementOutcome::Complete => Some(Continuation::Restartable(factory)),
        RetirementOutcome::Fault(retirement_fault)
        | RetirementOutcome::Adopted(retirement_fault) => {
            fault = retirement_fault;
            None
        }
    };
    MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
        fault,
        continuation,
    })
}

fn terminal_after_worker_disconnect(mut worker: WorkerCustody) -> MechanicalTerminal {
    let factory = worker.factory.clone();
    let join = worker.join.take();
    drop(worker.commands);
    let fault = match join.and_then(|join| join.join().err()) {
        Some(_) => MechanicalFault::new(
            MechanicalFaultKind::SupervisorUnwind,
            "retained_prepared_export_worker_unwound",
            "the retained runtime worker unwound while it owned the invocation",
        ),
        None => MechanicalFault::new(
            MechanicalFaultKind::WorkerTermination,
            "retained_prepared_export_worker_disconnected",
            "the retained runtime worker disconnected before publishing a terminal",
        ),
    };
    MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
        fault,
        continuation: Some(Continuation::Restartable(factory)),
    })
}

fn retire_worker(worker: WorkerCustody, timeout: Duration) -> RetirementOutcome {
    let (response, receive) = mpsc::sync_channel(1);
    if worker
        .commands
        .send(WorkerCommand::Shutdown {
            response: Some(response),
        })
        .is_err()
    {
        return join_disconnected_worker(worker);
    }
    match receive.recv_timeout(timeout) {
        Ok(Ok(())) => join_shutdown_worker(worker),
        Ok(Err(error)) => {
            let join_outcome = join_shutdown_worker(worker);
            match join_outcome {
                RetirementOutcome::Complete => RetirementOutcome::Fault(MechanicalFault::new(
                    MechanicalFaultKind::Shutdown,
                    "retained_backend_runtime_shutdown_failed",
                    error.to_string(),
                )),
                other => other,
            }
        }
        Err(RecvTimeoutError::Disconnected) => join_disconnected_worker(worker),
        Err(RecvTimeoutError::Timeout) => {
            adopt_for_disposal(worker);
            RetirementOutcome::Adopted(MechanicalFault::new(
                MechanicalFaultKind::WorkerTermination,
                "retained_backend_retirement_adopted_after_timeout",
                "retained runtime retirement exceeded its foreground deadline and was adopted by the durable reaper",
            ))
        }
    }
}

fn join_shutdown_worker(mut worker: WorkerCustody) -> RetirementOutcome {
    let Some(join) = worker.join.take() else {
        return RetirementOutcome::Fault(MechanicalFault::new(
            MechanicalFaultKind::WorkerTermination,
            "retained_backend_worker_join_missing",
            "retained runtime shutdown completed without its affine join custody",
        ));
    };
    drop(worker.commands);
    match join.join() {
        Ok(()) => RetirementOutcome::Complete,
        Err(_) => RetirementOutcome::Fault(MechanicalFault::new(
            MechanicalFaultKind::SupervisorUnwind,
            "retained_backend_worker_unwound_during_shutdown",
            "retained runtime worker unwound while finalizing shutdown",
        )),
    }
}

fn join_disconnected_worker(mut worker: WorkerCustody) -> RetirementOutcome {
    let Some(join) = worker.join.take() else {
        return RetirementOutcome::Fault(MechanicalFault::new(
            MechanicalFaultKind::WorkerTermination,
            "retained_backend_disconnected_worker_join_missing",
            "disconnected retained runtime worker had no join custody",
        ));
    };
    drop(worker.commands);
    match join.join() {
        Ok(()) => RetirementOutcome::Complete,
        Err(_) => RetirementOutcome::Fault(MechanicalFault::new(
            MechanicalFaultKind::SupervisorUnwind,
            "retained_backend_disconnected_worker_unwound",
            "disconnected retained runtime worker ended by unwind",
        )),
    }
}

fn shutdown_worker(worker: WorkerCustody, control: ShutdownControl) -> BackendShutdownTerminal {
    match retire_worker(worker, control.timeout) {
        RetirementOutcome::Complete => BackendShutdownTerminal::Complete,
        RetirementOutcome::Fault(fault) | RetirementOutcome::Adopted(fault) => {
            BackendShutdownTerminal::MechanicalFault(fault)
        }
    }
}

fn is_typed_interrupt(error: &LibbunError) -> bool {
    matches!(
        error,
        LibbunError::BackendState { code, .. }
            if code == "retained_prepared_export_interrupt_observed"
    )
}

fn ensure_durable_reaper() -> Result<Sender<WorkerCustody>, String> {
    DURABLE_REAPER
        .get_or_init(|| {
            let (sender, receiver) = mpsc::channel();
            thread::Builder::new()
                .name("libbun-retained-durable-reaper".to_owned())
                .spawn(move || durable_reaper_loop(receiver))
                .map(|_| sender)
                .map_err(|error| error.to_string())
        })
        .as_ref()
        .map(Sender::clone)
        .map_err(Clone::clone)
}

fn durable_reaper_loop(receiver: Receiver<WorkerCustody>) {
    for mut worker in receiver {
        let _ = worker
            .commands
            .send(WorkerCommand::Shutdown { response: None });
        drop(worker.commands);
        if let Some(join) = worker.join.take() {
            let _ = join.join();
        }
    }
}

fn adopt_for_disposal(worker: WorkerCustody) {
    match ensure_durable_reaper().and_then(|sender| {
        sender
            .send(worker)
            .map_err(|error| format!("durable reaper queue disconnected: {error}"))
    }) {
        Ok(()) => {}
        Err(_) => {
            // A disconnected durable reaper drops the command sender. The
            // runtime worker then observes channel closure, shuts down inside
            // its owning thread, and its detached JoinHandle has no authority
            // surface back to the caller.
        }
    }
}

fn spawn_worker<R>(config: BunRuntimeConfig) -> LibbunResult<WorkerCustody>
where
    R: BunEmbeddingRuntime + 'static,
{
    let factory = WorkerFactory {
        config: config.clone(),
        spawn: spawn_worker::<R>,
    };
    let (commands, receiver) = mpsc::channel();
    let (initialized, initialization) = mpsc::sync_channel(1);
    let join = thread::Builder::new()
        .name("libbun-retained-runtime-owner".to_owned())
        .spawn(move || retained_worker_loop::<R>(config, receiver, initialized))
        .map_err(|error| {
            LibbunError::initialize(format!(
                "retained runtime owner thread spawn failed: {error}"
            ))
        })?;
    match initialization.recv() {
        Ok(Ok(())) => Ok(WorkerCustody {
            commands,
            join: Some(join),
            factory,
        }),
        Ok(Err(error)) => {
            let _ = join.join();
            Err(error)
        }
        Err(error) => {
            let _ = join.join();
            Err(LibbunError::initialize(format!(
                "retained runtime owner disconnected during initialization: {error}"
            )))
        }
    }
}

fn retained_worker_loop<R>(
    config: BunRuntimeConfig,
    receiver: Receiver<WorkerCommand>,
    initialized: SyncSender<LibbunResult<()>>,
) where
    R: BunEmbeddingRuntime + 'static,
{
    let mut host = match BunHost::<R>::initialize(config) {
        Ok(host) => host,
        Err(error) => {
            let _ = initialized.send(Err(error));
            return;
        }
    };
    host.drain_captured_output();
    if initialized.send(Ok(())).is_err() {
        return;
    }
    while let Ok(command) = receiver.recv() {
        match command {
            WorkerCommand::Drive {
                request,
                options,
                interrupt,
                response,
            } => {
                let result = host.call_provider_until_settled_for_prepared_export(
                    request,
                    options,
                    interrupt.as_ref(),
                );
                let unowned_output = host.drain_captured_output();
                let _ = response.send(WorkerDriveResponse {
                    result,
                    unowned_output,
                });
            }
            WorkerCommand::Shutdown { response } => {
                let result = host.shutdown();
                if let Some(response) = response {
                    let _ = response.send(result);
                }
                break;
            }
        }
    }
}

#[cfg(test)]
fn select_request_for_owner_test(
    request: ProviderRequest,
    options: ProviderSettleOptions,
) -> (SelectedProviderPackage, ProviderInvocation) {
    let brand = NEXT_SELECTION_BRAND.fetch_add(1, Ordering::Relaxed);
    (
        SelectedProviderPackage {
            brand,
            contract: request.contract,
            domain: request.domain,
            module: request.module,
            export: request.export,
        },
        ProviderInvocation {
            brand,
            input: request.input,
            options,
        },
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use serde_json::json;

    use super::*;
    use crate::{
        BunAsyncHandle, BunModuleHandle, BunModuleSpec, ExportCallResult, OutputStream,
        ProviderDeadline, ProviderError, PumpBudget, PumpOutcome,
    };

    static SHUTDOWNS: AtomicUsize = AtomicUsize::new(0);

    struct OwnerTestRuntime {
        output: Vec<OutputRecord>,
        late_output: Option<OutputRecord>,
        late_output_delay: usize,
        pending: BTreeMap<String, Option<ProviderCallResult>>,
        fail_shutdown: bool,
    }

    impl BunEmbeddingRuntime for OwnerTestRuntime {
        fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
            Ok(Self {
                output: Vec::new(),
                late_output: None,
                late_output_delay: 0,
                pending: BTreeMap::new(),
                fail_shutdown: config.host_id == "shutdown-fail",
            })
        }

        fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
            match spec {
                BunModuleSpec::Source { module_id, source } => Ok(BunModuleHandle {
                    id: format!("{module_id}:{source}"),
                }),
                _ => Err(LibbunError::module_load(
                    "owner tests require an in-memory source module",
                )),
            }
        }

        fn call_export(
            &mut self,
            module: &BunModuleHandle,
            export: &str,
            input: StructuralValue,
        ) -> LibbunResult<crate::ExportCallResult> {
            if export != "default" {
                return Err(LibbunError::export_call("selected export is not callable"));
            }
            let behavior = module
                .id
                .split_once(':')
                .map(|(_, behavior)| behavior)
                .unwrap_or_default();
            match behavior {
                "ok" => {
                    self.output.push(OutputRecord {
                        stream: OutputStream::Stdout,
                        text: "owner output".to_owned(),
                    });
                    Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
                }
                "reject" => Ok(ExportCallResult::Ready(ProviderCallResult::Err(
                    ProviderError {
                        code: "authored_rejection".to_owned(),
                        message: "authored rejection cargo".to_owned(),
                    },
                ))),
                "never" => {
                    self.pending.insert("pending".to_owned(), None);
                    Ok(ExportCallResult::Pending(BunAsyncHandle {
                        id: "pending".to_owned(),
                    }))
                }
                "blocking" => {
                    thread::sleep(Duration::from_millis(100));
                    Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
                }
                "late" => {
                    self.late_output = Some(OutputRecord {
                        stream: OutputStream::Log,
                        text: "late owner output".to_owned(),
                    });
                    self.late_output_delay = 1;
                    Ok(ExportCallResult::Ready(ProviderCallResult::Ok(input)))
                }
                "panic" => panic!("owner test runtime panic"),
                _ => Err(LibbunError::module_load("unknown owner test behavior")),
            }
        }

        fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<PumpOutcome> {
            Ok(PumpOutcome {
                ticks: budget.max_ticks,
                pending_async_work: self.pending.len(),
            })
        }

        fn resolve_async(
            &mut self,
            handle: &BunAsyncHandle,
        ) -> LibbunResult<Option<ProviderCallResult>> {
            self.pending
                .get_mut(&handle.id)
                .ok_or_else(|| LibbunError::export_call("unknown owner test async handle"))
                .map(Option::take)
        }

        fn captured_output(&self) -> &[OutputRecord] {
            &self.output
        }

        fn drain_captured_output(&mut self) -> Vec<OutputRecord> {
            if self.output.is_empty() && self.late_output_delay > 0 {
                self.late_output_delay -= 1;
                return Vec::new();
            }
            if self.output.is_empty()
                && let Some(late) = self.late_output.take()
            {
                return vec![late];
            }
            std::mem::take(&mut self.output)
        }

        fn shutdown(&mut self) -> LibbunResult<()> {
            SHUTDOWNS.fetch_add(1, Ordering::Relaxed);
            if self.fail_shutdown {
                Err(LibbunError::shutdown("owner test shutdown failure"))
            } else {
                Ok(())
            }
        }
    }

    fn backend(host_id: &str) -> BunProviderBackend {
        BunProviderBackend::open::<OwnerTestRuntime>(BunRuntimeConfig::new(host_id, "/tmp"))
            .expect("owner test backend opens")
    }

    fn selected(
        behavior: &str,
        input: serde_json::Value,
    ) -> (SelectedProviderPackage, ProviderInvocation) {
        select_request_for_owner_test(
            ProviderRequest {
                contract: ProviderContractIdentity {
                    package: "owner-test".to_owned(),
                    capability: "drive".to_owned(),
                    contract_fingerprint: "owner-test-v1".to_owned(),
                },
                domain: ProviderDomainClass::JavaScriptExternalTransport,
                module: BunModuleSpec::Source {
                    module_id: "owner-test".to_owned(),
                    source: behavior.to_owned(),
                },
                export: "default".to_owned(),
                input: StructuralValue(input),
            },
            ProviderSettleOptions::new(ProviderDeadline::from_millis(5_000)),
        )
    }

    fn drive(backend: BunProviderBackend, behavior: &str, timeout: Duration) -> MechanicalTerminal {
        let (package, invocation) = selected(behavior, json!({ "value": 41 }));
        backend
            .prepare(package, invocation)
            .expect("matching selected input prepares")
            .drive(DriveControl::deadline_after(timeout).expect("deadline is representable"))
    }

    #[test]
    fn affine_cargo_drive_retains_one_ready_continuation_for_second_invocation() {
        let first = drive(backend("cargo"), "ok", Duration::from_secs(1));
        let first_cargo = first.cargo().expect("fulfilled terminal owns cargo");
        assert_eq!(first_cargo.kind(), AuthoredSettlementKind::Fulfilled);
        assert!(!first_cargo.bytes().is_empty());
        assert_eq!(first_cargo.output().len(), 1);

        let (package, invocation) = selected("reject", json!(null));
        let second = first
            .prepare_next(package, invocation)
            .expect("ready terminal prepares one next invocation")
            .drive(
                DriveControl::deadline_after(Duration::from_secs(1))
                    .expect("deadline is representable"),
            );
        assert_eq!(
            second.cargo().expect("rejection is authored cargo").kind(),
            AuthoredSettlementKind::Rejected
        );
        assert!(matches!(
            second.shutdown(
                ShutdownControl::deadline_after(Duration::from_secs(1))
                    .expect("shutdown deadline is representable")
            ),
            BackendShutdownTerminal::Complete
        ));
    }

    #[test]
    fn typed_interrupt_retires_pending_drive_before_cancelled_terminal() {
        let (package, invocation) = selected("never", json!(null));
        let prepared = backend("cancel")
            .prepare(package, invocation)
            .expect("matching selected input prepares");
        let control = DriveControl::deadline_after(Duration::from_secs(2))
            .expect("deadline is representable");
        let interrupt = control.interrupt();
        let requester = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            interrupt.request();
        });
        let terminal = prepared.drive(control);
        requester.join().expect("interrupt requester joins");
        assert!(matches!(terminal, MechanicalTerminal::Cancelled(_)));
    }

    #[test]
    fn deadline_retires_pending_drive_and_yields_only_deadline_terminal() {
        let terminal = drive(backend("deadline"), "never", Duration::from_millis(10));
        assert!(matches!(terminal, MechanicalTerminal::DeadlineElapsed(_)));
    }

    #[test]
    fn deadline_dominates_a_simultaneous_predispatch_interrupt() {
        let (package, invocation) = selected("ok", json!(null));
        let prepared = backend("deadline-dominance")
            .prepare(package, invocation)
            .expect("matching selected input prepares");
        let control =
            DriveControl::deadline_after(Duration::ZERO).expect("zero deadline is representable");
        control.interrupt().request();
        assert!(matches!(
            prepared.drive(control),
            MechanicalTerminal::DeadlineElapsed(_)
        ));
    }

    #[test]
    fn blocking_deadline_adopts_unresolved_custody_and_fault_dominates() {
        let terminal = drive(
            backend("blocking-deadline"),
            "blocking",
            Duration::from_millis(10),
        );
        let fault = terminal
            .fault()
            .expect("unresolved deadline retirement is a typed fault");
        assert_eq!(fault.kind(), MechanicalFaultKind::WorkerTermination);
        assert_eq!(
            fault.code(),
            "retained_prepared_export_deadline_retirement_adopted"
        );
        let (package, invocation) = selected("ok", json!(null));
        assert!(terminal.prepare_next(package, invocation).is_err());
    }

    #[test]
    fn predispatch_interrupt_preserves_one_ready_continuation() {
        let (package, invocation) = selected("ok", json!(null));
        let prepared = backend("predispatch-cancel")
            .prepare(package, invocation)
            .expect("matching selected input prepares");
        let control = DriveControl::deadline_after(Duration::from_secs(1))
            .expect("deadline is representable");
        control.interrupt().request();
        let cancelled = prepared.drive(control);
        assert!(matches!(cancelled, MechanicalTerminal::Cancelled(_)));
        let (package, invocation) = selected("ok", json!(null));
        let resumed = cancelled
            .prepare_next(package, invocation)
            .expect("predispatch cancellation retains the ready worker")
            .drive(
                DriveControl::deadline_after(Duration::from_secs(1))
                    .expect("deadline is representable"),
            );
        assert!(matches!(resumed, MechanicalTerminal::Cargo(_)));
    }

    #[test]
    fn late_output_fault_dominates_provisional_authored_cargo() {
        let terminal = drive(backend("late"), "late", Duration::from_secs(1));
        let fault = terminal
            .fault()
            .expect("late output is a typed mechanical fault");
        assert_eq!(fault.kind(), MechanicalFaultKind::OutputQuiescence);
        assert_eq!(
            fault.code(),
            "retained_prepared_export_unowned_output_after_terminal"
        );
    }

    #[test]
    fn runtime_unwind_yields_typed_terminal_and_restartable_continuation() {
        let terminal = drive(backend("panic"), "panic", Duration::from_secs(1));
        assert_eq!(
            terminal.fault().expect("unwind is typed").kind(),
            MechanicalFaultKind::SupervisorUnwind
        );
        let (package, invocation) = selected("ok", json!(null));
        let restarted = terminal
            .prepare_next(package, invocation)
            .expect("unwound worker has one restartable continuation")
            .drive(
                DriveControl::deadline_after(Duration::from_secs(1))
                    .expect("deadline is representable"),
            );
        assert!(matches!(restarted, MechanicalTerminal::Cargo(_)));
    }

    #[test]
    fn consuming_shutdown_reports_typed_failure_without_backend_retry() {
        let terminal = backend("shutdown-fail").shutdown(
            ShutdownControl::deadline_after(Duration::from_secs(1))
                .expect("shutdown deadline is representable"),
        );
        assert!(matches!(
            terminal,
            BackendShutdownTerminal::MechanicalFault(_)
        ));
    }

    #[test]
    fn dropped_undispatched_prepared_export_transfers_shutdown_to_reaper() {
        let before = SHUTDOWNS.load(Ordering::Relaxed);
        let (package, invocation) = selected("ok", json!(null));
        let prepared = backend("drop")
            .prepare(package, invocation)
            .expect("matching selected input prepares");
        drop(prepared);
        let deadline = Instant::now() + Duration::from_secs(2);
        while SHUTDOWNS.load(Ordering::Relaxed) <= before && Instant::now() < deadline {
            thread::yield_now();
        }
        assert!(SHUTDOWNS.load(Ordering::Relaxed) > before);
    }

    #[test]
    fn dropped_ready_terminal_transfers_its_continuation_to_reaper() {
        let before = SHUTDOWNS.load(Ordering::Relaxed);
        drop(drive(
            backend("drop-terminal"),
            "ok",
            Duration::from_secs(1),
        ));
        let deadline = Instant::now() + Duration::from_secs(2);
        while SHUTDOWNS.load(Ordering::Relaxed) <= before && Instant::now() < deadline {
            thread::yield_now();
        }
        assert!(SHUTDOWNS.load(Ordering::Relaxed) > before);
    }
}
