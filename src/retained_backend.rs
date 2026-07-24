use std::cell::UnsafeCell;
use std::fmt;
use std::io::{self, Read};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
#[cfg(test)]
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender, TryRecvError};
use std::sync::{Arc, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::helper_protocol::{
    HelperHello, HelperRequest, HelperRequestPayload, HelperResponse, HelperResponsePayload,
    LIBBUN_HELPER_PROTOCOL_VERSION, read_frame, write_frame,
};
use crate::plugin_abi::LIBBUN_PLUGIN_ABI_VERSION;
#[cfg(test)]
use crate::{BunEmbeddingRuntime, BunHost};
use crate::{
    BunRuntimeConfig, LibbunError, LibbunResult, OutputRecord, ProviderCallResult,
    ProviderContractIdentity, ProviderDomainClass, ProviderRequest, ProviderSettleOptions,
    SettledProviderReceipt, StructuralValue,
};

#[cfg(test)]
static NEXT_SELECTION_BRAND: AtomicU64 = AtomicU64::new(1);
static DURABLE_REAPER: OnceLock<Result<Arc<DurableReaper>, String>> = OnceLock::new();
#[cfg(test)]
thread_local! {
    static FAIL_REAPER_NODE_ALLOCATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static PANIC_CALLER_AFTER_DISPATCH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
#[cfg(test)]
static DURABLE_PUBLISHED: AtomicU64 = AtomicU64::new(0);
#[cfg(test)]
static DURABLE_COMPLETED: AtomicU64 = AtomicU64::new(0);

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
enum WorkerFactory {
    Contained {
        config: BunRuntimeConfig,
        helper: PathBuf,
        bubblewrap: PathBuf,
    },
    #[cfg(test)]
    InProcess {
        config: BunRuntimeConfig,
        spawn: fn(BunRuntimeConfig) -> LibbunResult<WorkerCustody>,
    },
}

struct WorkerCustody {
    commands: Option<Sender<WorkerCommand>>,
    join: Option<JoinHandle<()>>,
    factory: Option<WorkerFactory>,
    retirement_requested: Arc<AtomicBool>,
    reaper_node: Option<Box<DurableReaperNode>>,
}

struct RetirementCustody {
    commands: Option<Sender<WorkerCommand>>,
    join: Option<JoinHandle<()>>,
    factory: Option<WorkerFactory>,
    retirement_requested: Arc<AtomicBool>,
    first_fault: Option<MechanicalFault>,
    shutdown_requested: bool,
}

struct DurableReaperNode {
    next: AtomicPtr<DurableReaperNode>,
    custody: UnsafeCell<Option<RetirementCustody>>,
    queue: Arc<DurableReaper>,
    #[cfg(test)]
    publication_counted: bool,
}

struct DurableReaper {
    head: AtomicPtr<DurableReaperNode>,
    worker: OnceLock<thread::Thread>,
}

unsafe impl Send for DurableReaperNode {}
unsafe impl Sync for DurableReaperNode {}

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

impl WorkerFactory {
    fn spawn(&self) -> LibbunResult<WorkerCustody> {
        match self {
            Self::Contained {
                config,
                helper,
                bubblewrap,
            } => spawn_contained_worker_with_paths(
                config.clone(),
                helper.clone(),
                bubblewrap.clone(),
            ),
            #[cfg(test)]
            Self::InProcess { config, spawn } => spawn(config.clone()),
        }
    }

    fn has_forced_retirement(&self) -> bool {
        matches!(self, Self::Contained { .. })
    }
}

impl WorkerCustody {
    fn new(
        commands: Sender<WorkerCommand>,
        join: JoinHandle<()>,
        factory: WorkerFactory,
        retirement_requested: Arc<AtomicBool>,
        reaper_node: Box<DurableReaperNode>,
    ) -> Self {
        Self {
            commands: Some(commands),
            join: Some(join),
            factory: Some(factory),
            retirement_requested,
            reaper_node: Some(reaper_node),
        }
    }

    fn commands(&self) -> &Sender<WorkerCommand> {
        self.commands
            .as_ref()
            .expect("live worker retains its private command sender")
    }

    fn factory(&self) -> &WorkerFactory {
        self.factory
            .as_ref()
            .expect("live worker retains its private restart factory")
    }

    fn publish_retirement(mut self) {
        let Some(node) = self.reaper_node.take() else {
            return;
        };
        let custody = RetirementCustody {
            commands: self.commands.take(),
            join: self.join.take(),
            factory: self.factory.take(),
            retirement_requested: Arc::clone(&self.retirement_requested),
            first_fault: None,
            shutdown_requested: false,
        };
        node.publish(custody);
    }

    fn disarm_completed(&mut self) {
        self.commands.take();
        self.factory.take();
        self.reaper_node.take();
    }
}

impl Drop for WorkerCustody {
    fn drop(&mut self) {
        let Some(node) = self.reaper_node.take() else {
            return;
        };
        let custody = RetirementCustody {
            commands: self.commands.take(),
            join: self.join.take(),
            factory: self.factory.take(),
            retirement_requested: Arc::clone(&self.retirement_requested),
            first_fault: None,
            shutdown_requested: false,
        };
        node.publish(custody);
    }
}

impl RetirementCustody {
    fn poll(&mut self) -> bool {
        self.retirement_requested.store(true, Ordering::Release);
        if !self.shutdown_requested {
            if let Some(commands) = self.commands.take() {
                let _ = commands.send(WorkerCommand::Shutdown { response: None });
            }
            self.shutdown_requested = true;
        }
        let Some(join) = self.join.as_ref() else {
            self.factory.take();
            return true;
        };
        if !join.is_finished() {
            return false;
        }
        let join = self.join.take().expect("finished join remains in custody");
        if join.join().is_err() && self.first_fault.is_none() {
            self.first_fault = Some(MechanicalFault::new(
                MechanicalFaultKind::SupervisorUnwind,
                "retained_backend_reaper_worker_unwound",
                "retained runtime worker unwound while held by retirement custody",
            ));
        }
        self.factory.take();
        true
    }
}

impl DurableReaperNode {
    #[allow(unused_mut)]
    fn publish(mut self: Box<Self>, custody: RetirementCustody) {
        unsafe {
            *self.custody.get() = Some(custody);
        }
        let queue = Arc::clone(&self.queue);
        self.next.store(std::ptr::null_mut(), Ordering::Relaxed);
        #[cfg(test)]
        if !self.publication_counted {
            DURABLE_PUBLISHED.fetch_add(1, Ordering::Relaxed);
            self.publication_counted = true;
        }
        let node = Box::into_raw(self);
        let mut head = queue.head.load(Ordering::Acquire);
        loop {
            unsafe {
                (*node).next.store(head, Ordering::Relaxed);
            }
            match queue
                .head
                .compare_exchange_weak(head, node, Ordering::Release, Ordering::Acquire)
            {
                Ok(_) => break,
                Err(current) => head = current,
            }
        }
        if let Some(worker) = queue.worker.get() {
            worker.unpark();
        }
    }
}

impl DurableReaper {
    fn drain_snapshot(&self) {
        let mut node = self.head.swap(std::ptr::null_mut(), Ordering::AcqRel);
        while !node.is_null() {
            let owned = unsafe { Box::from_raw(node) };
            node = owned.next.load(Ordering::Relaxed);
            let complete = unsafe {
                (*owned.custody.get())
                    .as_mut()
                    .map(RetirementCustody::poll)
                    .unwrap_or(true)
            };
            if complete {
                unsafe {
                    (*owned.custody.get()).take();
                }
                #[cfg(test)]
                DURABLE_COMPLETED.fetch_add(1, Ordering::Relaxed);
            } else {
                unsafe {
                    let custody = (*owned.custody.get())
                        .take()
                        .expect("pending durable node retains retirement custody");
                    owned.publish(custody);
                }
            }
        }
    }
}

fn preallocate_reaper_node() -> LibbunResult<Box<DurableReaperNode>> {
    #[cfg(test)]
    if FAIL_REAPER_NODE_ALLOCATION.with(|failure| failure.replace(false)) {
        return Err(LibbunError::initialize(
            "retained worker durable reaper node allocation was refused before admission",
        ));
    }
    let queue = ensure_durable_reaper().map_err(|message| {
        LibbunError::initialize(format!(
            "retained prepared-export durable reaper initialization failed: {message}"
        ))
    })?;
    Ok(Box::new(DurableReaperNode {
        next: AtomicPtr::new(std::ptr::null_mut()),
        custody: UnsafeCell::new(None),
        queue,
        #[cfg(test)]
        publication_counted: false,
    }))
}

impl BunProviderBackend {
    pub fn open(config: BunRuntimeConfig) -> LibbunResult<Self> {
        ensure_durable_reaper().map_err(|message| {
            LibbunError::initialize(format!(
                "retained prepared-export durable reaper initialization failed: {message}"
            ))
        })?;
        let worker = spawn_contained_worker(config)?;
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
                factory
                    .spawn()
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
    if worker.commands().send(command).is_err() {
        return terminal_after_worker_disconnect(worker);
    }
    #[cfg(test)]
    PANIC_CALLER_AFTER_DISPATCH.with(|panic_after_dispatch| {
        assert!(
            !panic_after_dispatch.replace(false),
            "injected caller-side drive unwind after dispatch publication"
        );
    });

    let has_forced_retirement = worker.factory().has_forced_retirement();
    let response = match receive.recv_timeout(control.remaining()) {
        Ok(response) => response,
        Err(RecvTimeoutError::Timeout) => {
            control.interrupt.request();
            if !has_forced_retirement {
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
            match receive.recv_timeout(control.retirement_timeout()) {
                Ok(response) => response,
                Err(RecvTimeoutError::Disconnected) => {
                    return terminal_after_worker_disconnect(worker);
                }
                Err(RecvTimeoutError::Timeout) => {
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
            }
        }
        Err(RecvTimeoutError::Disconnected) => {
            return terminal_after_worker_disconnect(worker);
        }
    };

    let selected = control.selection();
    if !response.unowned_output.is_empty() {
        let factory = worker.factory().clone();
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

    if !matches!(selected, ControlSelection::Continue)
        && let Err(error) = &response.result
        && !is_typed_interrupt(error)
    {
        let factory = worker.factory().clone();
        let fault = MechanicalFault::new(
            MechanicalFaultKind::WorkerTermination,
            "retained_prepared_export_forced_retirement_failed",
            error.to_string(),
        );
        let retirement = retire_worker(worker, control.retirement_timeout());
        return fault_after_retirement(fault, factory, retirement);
    }

    match selected {
        ControlSelection::Deadline => {
            let factory = worker.factory().clone();
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
            let factory = worker.factory().clone();
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
                        let factory = worker.factory().clone();
                        let retirement = retire_worker(worker, control.retirement_timeout());
                        fault_after_retirement(fault, factory, retirement)
                    }
                }
            }
            Ok(SettledProviderReceipt::Failed(failure))
                if failure.operation
                    == crate::ProviderExecutionOperation::ProviderDeadlineElapsed =>
            {
                let factory = worker.factory().clone();
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
                let factory = worker.factory().clone();
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
                let factory = worker.factory().clone();
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
                let factory = worker.factory().clone();
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
        RetirementOutcome::Fault(retirement_fault) => {
            fault = retirement_fault;
            Some(Continuation::Restartable(factory))
        }
        RetirementOutcome::Adopted(retirement_fault) => {
            fault = retirement_fault;
            None
        }
    };
    MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
        fault,
        continuation,
    })
}

fn terminal_after_worker_disconnect(worker: WorkerCustody) -> MechanicalTerminal {
    let factory = worker.factory().clone();
    let retirement = finish_worker_join(worker, Duration::from_secs(1));
    let (fault, continuation) = match retirement {
        RetirementOutcome::Complete => (
            MechanicalFault::new(
                MechanicalFaultKind::WorkerTermination,
                "retained_prepared_export_worker_disconnected",
                "the retained runtime worker disconnected before publishing a terminal",
            ),
            Some(Continuation::Restartable(factory)),
        ),
        RetirementOutcome::Fault(fault) => (fault, Some(Continuation::Restartable(factory))),
        RetirementOutcome::Adopted(fault) => (fault, None),
    };
    MechanicalTerminal::MechanicalFault(MechanicalFaultTerminal {
        fault,
        continuation,
    })
}

fn retire_worker(worker: WorkerCustody, timeout: Duration) -> RetirementOutcome {
    let (response, receive) = mpsc::sync_channel(1);
    if worker
        .commands()
        .send(WorkerCommand::Shutdown {
            response: Some(response),
        })
        .is_err()
    {
        return finish_worker_join(worker, timeout);
    }
    match receive.recv_timeout(timeout) {
        Ok(Ok(())) => finish_worker_join(worker, timeout),
        Ok(Err(error)) => {
            let join_outcome = finish_worker_join(worker, timeout);
            match join_outcome {
                RetirementOutcome::Complete => RetirementOutcome::Fault(MechanicalFault::new(
                    MechanicalFaultKind::Shutdown,
                    "retained_backend_runtime_shutdown_failed",
                    error.to_string(),
                )),
                other => other,
            }
        }
        Err(RecvTimeoutError::Disconnected) => finish_worker_join(worker, timeout),
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

fn finish_worker_join(mut worker: WorkerCustody, timeout: Duration) -> RetirementOutcome {
    worker.commands.take();
    let Some(join) = worker.join.take() else {
        worker.disarm_completed();
        return RetirementOutcome::Fault(MechanicalFault::new(
            MechanicalFaultKind::WorkerTermination,
            "retained_backend_worker_join_missing",
            "retained runtime shutdown completed without its affine join custody",
        ));
    };
    worker.join = Some(join);
    let deadline = Instant::now().checked_add(timeout);
    loop {
        if worker.join.as_ref().is_some_and(JoinHandle::is_finished) {
            let join = worker
                .join
                .take()
                .expect("finished join remains in custody");
            let outcome = if join.join().is_ok() {
                RetirementOutcome::Complete
            } else {
                RetirementOutcome::Fault(MechanicalFault::new(
                    MechanicalFaultKind::SupervisorUnwind,
                    "retained_backend_worker_unwound_during_shutdown",
                    "retained runtime worker unwound while finalizing shutdown",
                ))
            };
            worker.disarm_completed();
            return outcome;
        }
        if deadline.is_none_or(|deadline| Instant::now() >= deadline) {
            adopt_for_disposal(worker);
            return RetirementOutcome::Adopted(MechanicalFault::new(
                MechanicalFaultKind::WorkerTermination,
                "retained_backend_join_adopted_after_timeout",
                "retained runtime worker join remained unfinished and was adopted",
            ));
        }
        thread::yield_now();
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

fn ensure_durable_reaper() -> Result<Arc<DurableReaper>, String> {
    DURABLE_REAPER
        .get_or_init(|| {
            let reaper = Arc::new(DurableReaper {
                head: AtomicPtr::new(std::ptr::null_mut()),
                worker: OnceLock::new(),
            });
            let owner = Arc::clone(&reaper);
            let join = thread::Builder::new()
                .name("libbun-retained-durable-reaper".to_owned())
                .spawn(move || durable_reaper_loop(owner))
                .map_err(|error| error.to_string())?;
            reaper
                .worker
                .set(join.thread().clone())
                .map_err(|_| "durable reaper thread identity was already installed".to_owned())?;
            drop(join);
            Ok(reaper)
        })
        .as_ref()
        .map(Arc::clone)
        .map_err(Clone::clone)
}

fn durable_reaper_loop(reaper: Arc<DurableReaper>) {
    loop {
        reaper.drain_snapshot();
        thread::park_timeout(Duration::from_millis(2));
    }
}

fn adopt_for_disposal(worker: WorkerCustody) {
    worker.publish_retirement();
}

fn spawn_contained_worker(mut config: BunRuntimeConfig) -> LibbunResult<WorkerCustody> {
    let (helper, bubblewrap) = resolve_contained_worker_paths()?;
    config.working_directory = config.working_directory.canonicalize().map_err(|error| {
        LibbunError::initialize(format!(
            "retained worker working directory `{}` cannot be resolved exactly: {error}",
            config.working_directory.display()
        ))
    })?;
    spawn_contained_worker_with_paths(config, helper, bubblewrap)
}

#[cfg(target_os = "linux")]
fn resolve_contained_worker_paths() -> LibbunResult<(PathBuf, PathBuf)> {
    use std::os::unix::fs::PermissionsExt;

    fn exact_executable(path: &Path, label: &str) -> LibbunResult<PathBuf> {
        let exact = path.canonicalize().map_err(|error| {
            LibbunError::initialize(format!(
                "{label} `{}` cannot be resolved exactly: {error}",
                path.display()
            ))
        })?;
        let metadata = exact.metadata().map_err(|error| {
            LibbunError::initialize(format!(
                "{label} `{}` metadata cannot be read: {error}",
                exact.display()
            ))
        })?;
        if !metadata.is_file() || metadata.permissions().mode() & 0o111 == 0 {
            return Err(LibbunError::initialize(format!(
                "{label} `{}` is not an executable file",
                exact.display()
            )));
        }
        Ok(exact)
    }

    let current = std::env::current_exe().map_err(|error| {
        LibbunError::initialize(format!(
            "retained worker host executable cannot be resolved: {error}"
        ))
    })?;
    let directory = current.parent().ok_or_else(|| {
        LibbunError::initialize("retained worker host executable has no parent directory")
    })?;
    let mut helper_candidates = vec![directory.join("libbun-runtime-native")];
    if directory.file_name().and_then(|name| name.to_str()) == Some("deps")
        && let Some(parent) = directory.parent()
    {
        helper_candidates.push(parent.join("libbun-runtime-native"));
    }
    let helper = helper_candidates
        .iter()
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| {
            LibbunError::initialize(format!(
                "exact sibling libbun-runtime-native is absent beside `{}`",
                current.display()
            ))
        })?;
    let helper = exact_executable(helper, "retained runtime worker")?;

    let bubblewrap = [Path::new("/usr/bin/bwrap"), Path::new("/bin/bwrap")]
        .into_iter()
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| {
            LibbunError::initialize(
                "exact retained worker containment requires /usr/bin/bwrap or /bin/bwrap",
            )
        })?;
    let bubblewrap = exact_executable(bubblewrap, "Bubblewrap containment runtime")?;
    Ok((helper, bubblewrap))
}

#[cfg(not(target_os = "linux"))]
fn resolve_contained_worker_paths() -> LibbunResult<(PathBuf, PathBuf)> {
    Err(LibbunError::initialize(
        "exact retained worker containment is currently available only on Linux",
    ))
}

fn spawn_contained_worker_with_paths(
    config: BunRuntimeConfig,
    helper: PathBuf,
    bubblewrap: PathBuf,
) -> LibbunResult<WorkerCustody> {
    let reaper_node = preallocate_reaper_node()?;
    let factory = WorkerFactory::Contained {
        config: config.clone(),
        helper: helper.clone(),
        bubblewrap: bubblewrap.clone(),
    };
    let retirement_requested = Arc::new(AtomicBool::new(false));
    let worker_retirement = Arc::clone(&retirement_requested);
    let (commands, receiver) = mpsc::channel();
    let (initialized, initialization) = mpsc::sync_channel(1);
    let join = thread::Builder::new()
        .name("libbun-contained-runtime-owner".to_owned())
        .spawn(move || {
            contained_worker_loop(
                config,
                helper,
                bubblewrap,
                receiver,
                initialized,
                worker_retirement,
            )
        })
        .map_err(|error| {
            LibbunError::initialize(format!(
                "contained runtime owner thread spawn failed: {error}"
            ))
        })?;
    let worker = WorkerCustody::new(commands, join, factory, retirement_requested, reaper_node);
    match initialization.recv_timeout(Duration::from_secs(6)) {
        Ok(Ok(())) => Ok(worker),
        Ok(Err(error)) => {
            adopt_for_disposal(worker);
            Err(error)
        }
        Err(RecvTimeoutError::Timeout) => {
            adopt_for_disposal(worker);
            Err(LibbunError::initialize(
                "contained runtime owner admission exceeded its foreground deadline",
            ))
        }
        Err(RecvTimeoutError::Disconnected) => {
            adopt_for_disposal(worker);
            Err(LibbunError::initialize(
                "contained runtime owner disconnected during initialization",
            ))
        }
    }
}

struct ContainedProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    responses: Receiver<Result<HelperResponse, String>>,
    response_reader: Option<JoinHandle<()>>,
    stderr_reader: Option<JoinHandle<io::Result<Vec<u8>>>>,
    next_id: u64,
    finished: bool,
    exit_status: Option<ExitStatus>,
    retirement_fault: Option<LibbunError>,
}

impl ContainedProcess {
    fn start(config: &BunRuntimeConfig, helper: &Path, bubblewrap: &Path) -> LibbunResult<Self> {
        let mut child = Command::new(bubblewrap)
            .arg("--die-with-parent")
            .arg("--unshare-user")
            .arg("--uid")
            .arg("0")
            .arg("--gid")
            .arg("0")
            .arg("--unshare-pid")
            .arg("--new-session")
            .arg("--proc")
            .arg("/proc")
            .arg("--dev")
            .arg("/dev")
            .arg("--ro-bind")
            .arg("/")
            .arg("/")
            .arg("--chdir")
            .arg(&config.working_directory)
            .arg("--")
            .arg(helper)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| {
                LibbunError::initialize(format!(
                    "exact-contained retained worker admission failed: {error}"
                ))
            })?;

        let Some(stdin) = child.stdin.take() else {
            let admission =
                LibbunError::initialize("contained retained worker stdin custody is missing");
            return Err(retire_partial_admission(&mut child, None)
                .err()
                .unwrap_or(admission));
        };
        let Some(stdout) = child.stdout.take() else {
            drop(stdin);
            let admission =
                LibbunError::initialize("contained retained worker stdout custody is missing");
            return Err(retire_partial_admission(&mut child, None)
                .err()
                .unwrap_or(admission));
        };
        let Some(stderr) = child.stderr.take() else {
            drop(stdin);
            drop(stdout);
            let admission =
                LibbunError::initialize("contained retained worker stderr custody is missing");
            return Err(retire_partial_admission(&mut child, None)
                .err()
                .unwrap_or(admission));
        };
        let (response_sender, responses) = mpsc::channel();
        let response_reader = match thread::Builder::new()
            .name("libbun-contained-response-reader".to_owned())
            .spawn(move || read_helper_responses(stdout, response_sender))
        {
            Ok(reader) => reader,
            Err(error) => {
                drop(stdin);
                drop(stderr);
                let admission = LibbunError::initialize(format!(
                    "contained retained worker response reader spawn failed: {error}"
                ));
                return Err(retire_partial_admission(&mut child, None)
                    .err()
                    .unwrap_or(admission));
            }
        };
        let stderr_reader = match thread::Builder::new()
            .name("libbun-contained-stderr-reader".to_owned())
            .spawn(move || drain_helper_stderr(stderr))
        {
            Ok(reader) => reader,
            Err(error) => {
                drop(stdin);
                let admission = LibbunError::initialize(format!(
                    "contained retained worker stderr reader spawn failed: {error}"
                ));
                return Err(retire_partial_admission(&mut child, Some(response_reader))
                    .err()
                    .unwrap_or(admission));
            }
        };
        let mut process = Self {
            child,
            stdin: Some(stdin),
            responses,
            response_reader: Some(response_reader),
            stderr_reader: Some(stderr_reader),
            next_id: 1,
            finished: false,
            exit_status: None,
            retirement_fault: None,
        };
        let admission = catch_unwind(AssertUnwindSafe(|| process.initialize(config)));
        match admission {
            Ok(Ok(())) => {}
            Ok(Err(admission)) => {
                return Err(process.force_terminate().err().unwrap_or(admission));
            }
            Err(_) => {
                let admission = LibbunError::initialize(
                    "contained retained worker admission supervisor unwound",
                );
                return Err(process.force_terminate().err().unwrap_or(admission));
            }
        }
        Ok(process)
    }

    fn initialize(&mut self, config: &BunRuntimeConfig) -> LibbunResult<()> {
        let hello = self.transact(
            HelperRequestPayload::Hello(HelperHello::current(std::env::consts::ARCH)),
            Duration::from_secs(5),
        )?;
        let HelperResponsePayload::Hello(hello) = hello else {
            return Err(LibbunError::initialize(
                "contained retained worker returned a non-hello handshake payload",
            ));
        };
        let expected = HelperHello::current(std::env::consts::ARCH);
        if hello.plugin_abi_version != LIBBUN_PLUGIN_ABI_VERSION
            || hello.helper_protocol_version != LIBBUN_HELPER_PROTOCOL_VERSION
            || hello.target != expected.target
            || hello.libbun_version != expected.libbun_version
            || hello.bun_revision != expected.bun_revision
        {
            return Err(LibbunError::initialize(
                "contained retained worker handshake does not match the owning libbun build",
            ));
        }
        match self.transact(
            HelperRequestPayload::Create {
                config: config.clone(),
            },
            Duration::from_secs(5),
        )? {
            HelperResponsePayload::Unit => {}
            _ => {
                return Err(LibbunError::initialize(
                    "contained retained worker returned a non-unit create payload",
                ));
            }
        }
        match self.transact(HelperRequestPayload::DrainOutput, Duration::from_secs(5))? {
            HelperResponsePayload::Output(output) if output.is_empty() => Ok(()),
            HelperResponsePayload::Output(output) => Err(LibbunError::initialize(format!(
                "contained retained worker produced {} unowned output record(s) during admission",
                output.len()
            ))),
            _ => Err(LibbunError::initialize(
                "contained retained worker returned a non-output admission drain payload",
            )),
        }
    }

    fn drive(
        &mut self,
        request: ProviderRequest,
        options: ProviderSettleOptions,
        interrupt: &AtomicBool,
        retirement_requested: &AtomicBool,
    ) -> WorkerDriveResponse {
        let call_id =
            match self.send(HelperRequestPayload::CallProviderUntilSettled { request, options }) {
                Ok(id) => id,
                Err(error) => {
                    return WorkerDriveResponse {
                        result: Err(error),
                        unowned_output: Vec::new(),
                    };
                }
            };
        let call = match self.receive_interruptible(call_id, interrupt, retirement_requested) {
            Ok(response) => response,
            Err(error) => {
                return WorkerDriveResponse {
                    result: Err(error),
                    unowned_output: Vec::new(),
                };
            }
        };
        let result = match call.result {
            Ok(HelperResponsePayload::SettledProvider(receipt)) => Ok(receipt),
            Ok(_) => Err(LibbunError::backend_state(
                "retained_worker_protocol_payload_mismatch",
                "contained retained worker returned a non-settled provider payload",
            )),
            Err(message) => Err(LibbunError::backend_state(
                "retained_worker_helper_drive_rejected",
                message,
            )),
        };

        let drain_id = match self.send(HelperRequestPayload::DrainOutput) {
            Ok(id) => id,
            Err(error) => {
                return WorkerDriveResponse {
                    result: Err(error),
                    unowned_output: Vec::new(),
                };
            }
        };
        let unowned_output =
            match self.receive_interruptible(drain_id, interrupt, retirement_requested) {
                Ok(HelperResponse {
                    result: Ok(HelperResponsePayload::Output(output)),
                    ..
                }) => output,
                Ok(HelperResponse { result: Ok(_), .. }) => {
                    return WorkerDriveResponse {
                        result: Err(LibbunError::backend_state(
                            "retained_worker_protocol_payload_mismatch",
                            "contained retained worker returned a non-output drain payload",
                        )),
                        unowned_output: Vec::new(),
                    };
                }
                Ok(HelperResponse {
                    result: Err(message),
                    ..
                }) => {
                    return WorkerDriveResponse {
                        result: Err(LibbunError::backend_state(
                            "retained_worker_output_drain_failed",
                            message,
                        )),
                        unowned_output: Vec::new(),
                    };
                }
                Err(error) => {
                    return WorkerDriveResponse {
                        result: Err(error),
                        unowned_output: Vec::new(),
                    };
                }
            };
        WorkerDriveResponse {
            result,
            unowned_output,
        }
    }

    fn graceful_shutdown(&mut self) -> LibbunResult<()> {
        let exit = match self.transact(HelperRequestPayload::Exit, Duration::from_secs(1)) {
            Ok(exit) => exit,
            Err(error) => {
                self.force_terminate()?;
                return Err(error);
            }
        };
        let status = self.finish_process(Duration::from_secs(1))?;
        match exit {
            HelperResponsePayload::Unit if status.success() => Ok(()),
            HelperResponsePayload::Unit => Err(LibbunError::shutdown(format!(
                "contained retained worker exited with status {status}"
            ))),
            _ => Err(LibbunError::shutdown(
                "contained retained worker returned a non-unit exit payload",
            )),
        }
    }

    fn transact(
        &mut self,
        payload: HelperRequestPayload,
        timeout: Duration,
    ) -> LibbunResult<HelperResponsePayload> {
        let id = self.send(payload)?;
        let response = self.receive(id, timeout)?;
        response.result.map_err(|message| {
            LibbunError::backend_state("retained_worker_helper_rejected", message)
        })
    }

    fn send(&mut self, payload: HelperRequestPayload) -> LibbunResult<u64> {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).ok_or_else(|| {
            LibbunError::backend_state(
                "retained_worker_protocol_sequence_exhausted",
                "contained retained worker protocol sequence exhausted u64",
            )
        })?;
        let stdin = self.stdin.as_mut().ok_or_else(|| {
            LibbunError::backend_state(
                "retained_worker_stdin_closed",
                "contained retained worker stdin is already closed",
            )
        })?;
        write_frame(stdin, &HelperRequest { id, payload }).map_err(|error| {
            LibbunError::backend_state("retained_worker_protocol_write_failed", error.to_string())
        })?;
        Ok(id)
    }

    fn receive(&mut self, id: u64, timeout: Duration) -> LibbunResult<HelperResponse> {
        match self.responses.recv_timeout(timeout) {
            Ok(Ok(response)) if response.id == id => Ok(response),
            Ok(Ok(response)) => Err(LibbunError::backend_state(
                "retained_worker_protocol_correspondence_failed",
                format!(
                    "contained retained worker response {} did not match request {id}",
                    response.id
                ),
            )),
            Ok(Err(message)) => Err(LibbunError::backend_state(
                "retained_worker_protocol_read_failed",
                message,
            )),
            Err(RecvTimeoutError::Timeout) => Err(LibbunError::backend_state(
                "retained_worker_protocol_response_timeout",
                format!("contained retained worker did not answer request {id} in time"),
            )),
            Err(RecvTimeoutError::Disconnected) => Err(LibbunError::backend_state(
                "retained_worker_protocol_reader_disconnected",
                "contained retained worker response reader disconnected",
            )),
        }
    }

    fn receive_interruptible(
        &mut self,
        id: u64,
        interrupt: &AtomicBool,
        retirement_requested: &AtomicBool,
    ) -> LibbunResult<HelperResponse> {
        loop {
            if interrupt.load(Ordering::Acquire) || retirement_requested.load(Ordering::Acquire) {
                match self.responses.try_recv() {
                    Ok(Ok(response)) if response.id == id => return Ok(response),
                    Ok(Ok(response)) => {
                        return Err(LibbunError::backend_state(
                            "retained_worker_protocol_correspondence_failed",
                            format!(
                                "contained retained worker response {} did not match request {id}",
                                response.id
                            ),
                        ));
                    }
                    Ok(Err(message)) => {
                        return Err(LibbunError::backend_state(
                            "retained_worker_protocol_read_failed",
                            message,
                        ));
                    }
                    Err(TryRecvError::Disconnected) => {
                        return Err(LibbunError::backend_state(
                            "retained_worker_protocol_reader_disconnected",
                            "contained retained worker response reader disconnected",
                        ));
                    }
                    Err(TryRecvError::Empty) => {
                        self.force_terminate()?;
                        return Err(LibbunError::backend_state(
                            "retained_prepared_export_interrupt_observed",
                            "contained retained worker was forcibly retired after interruption",
                        ));
                    }
                }
            }
            match self.responses.recv_timeout(Duration::from_millis(2)) {
                Ok(Ok(response)) if response.id == id => return Ok(response),
                Ok(Ok(response)) => {
                    return Err(LibbunError::backend_state(
                        "retained_worker_protocol_correspondence_failed",
                        format!(
                            "contained retained worker response {} did not match request {id}",
                            response.id
                        ),
                    ));
                }
                Ok(Err(message)) => {
                    return Err(LibbunError::backend_state(
                        "retained_worker_protocol_read_failed",
                        message,
                    ));
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(LibbunError::backend_state(
                        "retained_worker_protocol_reader_disconnected",
                        "contained retained worker response reader disconnected",
                    ));
                }
            }
        }
    }

    fn force_terminate(&mut self) -> LibbunResult<()> {
        self.stdin.take();
        while !self.poll_retirement_once() {
            if !self.finished
                && let Err(error) = self.child.kill()
                && self.retirement_fault.is_none()
            {
                self.retirement_fault = Some(LibbunError::shutdown(format!(
                    "contained retained worker namespace leader kill failed: {error}"
                )));
            }
            thread::park_timeout(Duration::from_millis(2));
        }
        match self.retirement_fault.take() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn finish_process(&mut self, timeout: Duration) -> LibbunResult<ExitStatus> {
        self.stdin.take();
        let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
            LibbunError::shutdown("contained retained worker exit deadline overflow")
        })?;
        loop {
            if self.poll_retirement_once() {
                if let Some(error) = self.retirement_fault.take() {
                    return Err(error);
                }
                return self.exit_status.take().ok_or_else(|| {
                    LibbunError::shutdown(
                        "contained retained worker retired without an exit status",
                    )
                });
            }
            if Instant::now() >= deadline {
                if self.retirement_fault.is_none() {
                    self.retirement_fault = Some(LibbunError::shutdown(
                        "contained retained worker acknowledged exit but did not terminate before its deadline",
                    ));
                }
                self.force_terminate()?;
            }
            thread::park_timeout(Duration::from_millis(2));
        }
    }

    fn poll_retirement_once(&mut self) -> bool {
        if !self.finished {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.finished = true;
                    self.exit_status = Some(status);
                }
                Ok(None) => return false,
                Err(error) => {
                    if self.retirement_fault.is_none() {
                        self.retirement_fault = Some(LibbunError::shutdown(format!(
                            "contained retained worker namespace leader status failed: {error}"
                        )));
                    }
                    return false;
                }
            }
        }
        if self
            .response_reader
            .as_ref()
            .is_some_and(|reader| !reader.is_finished())
            || self
                .stderr_reader
                .as_ref()
                .is_some_and(|reader| !reader.is_finished())
        {
            return false;
        }
        if let Some(reader) = self.response_reader.take()
            && reader.join().is_err()
            && self.retirement_fault.is_none()
        {
            self.retirement_fault = Some(LibbunError::shutdown(
                "contained retained worker response reader unwound",
            ));
        }
        if let Some(reader) = self.stderr_reader.take() {
            match reader.join() {
                Err(_) if self.retirement_fault.is_none() => {
                    self.retirement_fault = Some(LibbunError::shutdown(
                        "contained retained worker stderr reader unwound",
                    ));
                }
                Ok(Err(error)) if self.retirement_fault.is_none() => {
                    self.retirement_fault = Some(LibbunError::shutdown(format!(
                        "contained retained worker stderr drain failed: {error}"
                    )));
                }
                _ => {}
            }
        }
        true
    }
}

fn retire_partial_admission(
    child: &mut Child,
    mut response_reader: Option<JoinHandle<()>>,
) -> LibbunResult<()> {
    let mut first_fault = None;
    loop {
        if let Err(error) = child.kill()
            && first_fault.is_none()
        {
            first_fault = Some(LibbunError::shutdown(format!(
                "partially admitted retained worker kill failed: {error}"
            )));
        }
        match child.try_wait() {
            Ok(Some(_)) => {
                if response_reader
                    .as_ref()
                    .is_some_and(|reader| !reader.is_finished())
                {
                    thread::park_timeout(Duration::from_millis(2));
                    continue;
                }
                if let Some(reader) = response_reader.take()
                    && reader.join().is_err()
                    && first_fault.is_none()
                {
                    first_fault = Some(LibbunError::shutdown(
                        "partially admitted retained worker response reader unwound",
                    ));
                }
                return match first_fault {
                    Some(error) => Err(error),
                    None => Ok(()),
                };
            }
            Ok(None) => {}
            Err(error) if first_fault.is_none() => {
                first_fault = Some(LibbunError::shutdown(format!(
                    "partially admitted retained worker status failed: {error}"
                )));
            }
            Err(_) => {}
        }
        thread::park_timeout(Duration::from_millis(2));
    }
}

fn read_helper_responses(
    mut stdout: ChildStdout,
    responses: Sender<Result<HelperResponse, String>>,
) {
    loop {
        match read_frame(&mut stdout) {
            Ok(Some(response)) => {
                if responses.send(Ok(response)).is_err() {
                    break;
                }
            }
            Ok(None) => {
                let _ = responses.send(Err(
                    "contained retained worker closed stdout before the protocol ended".to_owned(),
                ));
                break;
            }
            Err(error) => {
                let _ = responses.send(Err(error.to_string()));
                break;
            }
        }
    }
}

fn drain_helper_stderr(mut stderr: ChildStderr) -> io::Result<Vec<u8>> {
    const MAX_CAPTURED_STDERR: usize = 64 * 1024;

    let mut captured = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let read = stderr.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        let retained = MAX_CAPTURED_STDERR.saturating_sub(captured.len()).min(read);
        captured.extend_from_slice(&chunk[..retained]);
    }
    Ok(captured)
}

fn contained_worker_loop(
    config: BunRuntimeConfig,
    helper: PathBuf,
    bubblewrap: PathBuf,
    receiver: Receiver<WorkerCommand>,
    initialized: SyncSender<LibbunResult<()>>,
    retirement_requested: Arc<AtomicBool>,
) {
    let mut process = match ContainedProcess::start(&config, &helper, &bubblewrap) {
        Ok(process) => process,
        Err(error) => {
            let _ = initialized.send(Err(error));
            return;
        }
    };
    if initialized.send(Ok(())).is_err() {
        let _ = process.force_terminate();
        return;
    }
    let _ = catch_unwind(AssertUnwindSafe(|| {
        while let Ok(command) = receiver.recv() {
            match command {
                WorkerCommand::Drive {
                    request,
                    options,
                    interrupt,
                    response,
                } => {
                    let drive = process.drive(
                        request,
                        options,
                        interrupt.as_ref(),
                        retirement_requested.as_ref(),
                    );
                    let terminated =
                        matches!(&drive.result, Err(error) if is_typed_interrupt(error));
                    let _ = response.send(drive);
                    if terminated {
                        break;
                    }
                }
                WorkerCommand::Shutdown { response } => {
                    let result = process.graceful_shutdown();
                    if let Some(response) = response {
                        let _ = response.send(result);
                    }
                    break;
                }
            }
        }
    }));
    if !process.finished {
        let _ = process.force_terminate();
    }
}

#[cfg(test)]
fn spawn_in_process_worker<R>(config: BunRuntimeConfig) -> LibbunResult<WorkerCustody>
where
    R: BunEmbeddingRuntime + 'static,
{
    let reaper_node = preallocate_reaper_node()?;
    let factory = WorkerFactory::InProcess {
        config: config.clone(),
        spawn: spawn_in_process_worker::<R>,
    };
    let retirement_requested = Arc::new(AtomicBool::new(false));
    let worker_retirement = Arc::clone(&retirement_requested);
    let (commands, receiver) = mpsc::channel();
    let (initialized, initialization) = mpsc::sync_channel(1);
    let join = thread::Builder::new()
        .name("libbun-retained-runtime-owner".to_owned())
        .spawn(move || retained_worker_loop::<R>(config, receiver, initialized, worker_retirement))
        .map_err(|error| {
            LibbunError::initialize(format!(
                "retained runtime owner thread spawn failed: {error}"
            ))
        })?;
    let worker = WorkerCustody::new(commands, join, factory, retirement_requested, reaper_node);
    match initialization.recv() {
        Ok(Ok(())) => Ok(worker),
        Ok(Err(error)) => {
            adopt_for_disposal(worker);
            Err(error)
        }
        Err(error) => {
            adopt_for_disposal(worker);
            Err(LibbunError::initialize(format!(
                "retained runtime owner disconnected during initialization: {error}"
            )))
        }
    }
}

#[cfg(test)]
fn retained_worker_loop<R>(
    config: BunRuntimeConfig,
    receiver: Receiver<WorkerCommand>,
    initialized: SyncSender<LibbunResult<()>>,
    _retirement_requested: Arc<AtomicBool>,
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
    #[cfg(target_os = "linux")]
    use std::os::unix::fs::PermissionsExt;
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
        let worker =
            spawn_in_process_worker::<OwnerTestRuntime>(BunRuntimeConfig::new(host_id, "/tmp"))
                .expect("owner test backend opens");
        BunProviderBackend {
            worker: Some(worker),
        }
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

    #[test]
    fn reaper_node_allocation_failure_precedes_live_worker_admission() {
        FAIL_REAPER_NODE_ALLOCATION.with(|failure| failure.set(true));
        let result = spawn_in_process_worker::<OwnerTestRuntime>(BunRuntimeConfig::new(
            "allocation-refused",
            "/tmp",
        ));
        assert!(matches!(result, Err(LibbunError::Initialize { .. })));
    }

    #[test]
    fn caller_drive_unwind_publishes_worker_custody_before_terminal() {
        let published_before = DURABLE_PUBLISHED.load(Ordering::Acquire);
        PANIC_CALLER_AFTER_DISPATCH.with(|panic_after_dispatch| panic_after_dispatch.set(true));
        let terminal = drive(backend("caller-unwind"), "blocking", Duration::from_secs(1));
        assert_eq!(
            terminal.fault().expect("caller unwind is typed").kind(),
            MechanicalFaultKind::SupervisorUnwind
        );
        assert!(DURABLE_PUBLISHED.load(Ordering::Acquire) > published_before);
    }

    #[test]
    fn drop_is_nonblocking_and_publishes_preallocated_node() {
        let backend = backend("nonblocking-drop");
        let published_before = DURABLE_PUBLISHED.load(Ordering::Acquire);
        let started = Instant::now();
        drop(backend);
        assert!(started.elapsed() < Duration::from_millis(50));
        assert!(DURABLE_PUBLISHED.load(Ordering::Acquire) > published_before);
    }

    #[test]
    fn reaper_wake_failure_retains_published_node_until_later_drain() {
        let queue = Arc::new(DurableReaper {
            head: AtomicPtr::new(std::ptr::null_mut()),
            worker: OnceLock::new(),
        });
        let node = Box::new(DurableReaperNode {
            next: AtomicPtr::new(std::ptr::null_mut()),
            custody: UnsafeCell::new(None),
            queue: Arc::clone(&queue),
            publication_counted: false,
        });
        let (commands, receiver) = mpsc::channel();
        let join = thread::spawn(|| {});
        while !join.is_finished() {
            thread::yield_now();
        }
        let worker = WorkerCustody::new(
            commands,
            join,
            WorkerFactory::InProcess {
                config: BunRuntimeConfig::new("wake-failure", "/tmp"),
                spawn: spawn_in_process_worker::<OwnerTestRuntime>,
            },
            Arc::new(AtomicBool::new(false)),
            node,
        );
        drop(worker);
        assert!(!queue.head.load(Ordering::Acquire).is_null());
        queue.drain_snapshot();
        assert!(queue.head.load(Ordering::Acquire).is_null());
        drop(receiver);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn contained_process_interrupt_and_deadline_reap_and_restart_the_exact_helper() {
        let bubblewrap = [Path::new("/usr/bin/bwrap"), Path::new("/bin/bwrap")]
            .into_iter()
            .find(|candidate| candidate.is_file())
            .expect("contained process proof requires Bubblewrap")
            .canonicalize()
            .expect("Bubblewrap path resolves exactly");
        let fixture = tempfile::tempdir().expect("fixture directory is created");
        let helper = fixture.path().join("libbun-runtime-native");
        std::fs::write(
            &helper,
            r#"#!/usr/bin/python3
import json
import struct
import sys
import time

reader = sys.stdin.buffer
writer = sys.stdout.buffer
block_exit = False

def read_frame():
    header = reader.read(4)
    if not header:
        return None
    length = struct.unpack(">I", header)[0]
    return json.loads(reader.read(length))

def write_frame(value):
    encoded = json.dumps(value, separators=(",", ":")).encode()
    writer.write(struct.pack(">I", len(encoded)))
    writer.write(encoded)
    writer.flush()

while True:
    request = read_frame()
    if request is None:
        break
    payload = request["payload"]
    kind = payload["type"]
    if kind == "hello":
        response = {"type": "hello", "payload": payload["payload"]}
    elif kind == "create":
        block_exit = payload["payload"]["config"]["hostId"] == "shutdown-block"
        response = {"type": "unit"}
    elif kind == "exit":
        if block_exit:
            time.sleep(60)
        response = {"type": "unit"}
    elif kind == "drainOutput":
        response = {"type": "output", "payload": []}
    elif kind == "callProviderUntilSettled":
        time.sleep(60)
        response = {"type": "unit"}
    else:
        response = {"type": "unit"}
    write_frame({"id": request["id"], "result": {"Ok": response}})
    if kind == "exit":
        break
"#,
        )
        .expect("fixture helper is written");
        let mut permissions = helper
            .metadata()
            .expect("fixture helper metadata is readable")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&helper, permissions).expect("fixture helper is executable");
        let helper = helper.canonicalize().expect("helper path resolves exactly");
        let config = BunRuntimeConfig::new("contained-owner-test", fixture.path());
        let worker = spawn_contained_worker_with_paths(config, helper.clone(), bubblewrap.clone())
            .expect("exact-contained fixture worker opens");
        let backend = BunProviderBackend {
            worker: Some(worker),
        };
        let (package, invocation) = selected("ok", json!(null));
        let prepared = backend
            .prepare(package, invocation)
            .expect("matching selected input prepares");
        let control = DriveControl::deadline_after(Duration::from_secs(2))
            .expect("drive deadline is representable");
        let interrupt = control.interrupt();
        let requester = thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            interrupt.request();
        });
        let cancelled = prepared.drive(control);
        requester.join().expect("interrupt requester joins");
        assert!(matches!(cancelled, MechanicalTerminal::Cancelled(_)));

        let (package, invocation) = selected("ok", json!(null));
        let restarted = cancelled
            .prepare_next(package, invocation)
            .expect("forced retirement preserves one exact-path restart");
        let deadline = restarted.drive(
            DriveControl::deadline_after(Duration::from_millis(20))
                .expect("deadline is representable"),
        );
        assert!(matches!(deadline, MechanicalTerminal::DeadlineElapsed(_)));

        let (package, invocation) = selected("ok", json!(null));
        let restarted = deadline
            .prepare_next(package, invocation)
            .expect("deadline retirement preserves one exact-path restart");
        let control = DriveControl::deadline_after(Duration::from_secs(2))
            .expect("second cancellation deadline is representable");
        let interrupt = control.interrupt();
        let requester = thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            interrupt.request();
        });
        let cancelled = restarted.drive(control);
        requester.join().expect("second interrupt requester joins");
        assert!(matches!(cancelled, MechanicalTerminal::Cancelled(_)));

        let (package, invocation) = selected("ok", json!(null));
        let restarted = cancelled
            .prepare_next(package, invocation)
            .expect("repeated cancellation preserves exact-path restart");
        assert!(matches!(
            restarted.shutdown(
                ShutdownControl::deadline_after(Duration::from_secs(2))
                    .expect("shutdown deadline is representable")
            ),
            BackendShutdownTerminal::Complete
        ));

        let config = BunRuntimeConfig::new("shutdown-block", fixture.path());
        let worker = spawn_contained_worker_with_paths(config, helper, bubblewrap)
            .expect("shutdown-hostile fixture worker opens");
        let backend = BunProviderBackend {
            worker: Some(worker),
        };
        let started = Instant::now();
        assert!(matches!(
            backend.shutdown(
                ShutdownControl::deadline_after(Duration::from_millis(20))
                    .expect("hostile shutdown deadline is representable")
            ),
            BackendShutdownTerminal::MechanicalFault(_)
        ));
        assert!(started.elapsed() < Duration::from_millis(100));
    }
}
