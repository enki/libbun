use std::ffi::OsString;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Child;
use std::process::ChildStderr;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use libbun_prepared_export_wire::MAX_CANDIDATE_BYTES;
use libbun_prepared_export_wire::MAX_REQUEST_BYTES;
use libbun_prepared_export_wire::TERMINAL_HEADER_LEN;
use libbun_prepared_export_wire::TERMINAL_MAGIC;
use libbun_prepared_export_wire::VERSION;
use libbun_prepared_export_wire::WorkerFaultKind;
const MAX_DIAGNOSTIC_BYTES: usize = 4096;

/// One invocation-bound export prepared for exactly one mechanical drive.
///
/// The fields and minting operation are private. Driving consumes the value so
/// a prepared export cannot be replayed or shared between invocations.
pub struct PreparedExport {
    worker: WorkerLaunch,
    request: Vec<u8>,
    #[cfg(test)]
    panic_after_admission: bool,
}

enum WorkerLaunch {
    #[cfg(test)]
    Exact {
        program: PathBuf,
        arguments: Vec<OsString>,
    },
}

impl PreparedExport {
    /// Performs the complete mechanical drive and consumes this prepared
    /// export. Every return path has retired the fresh worker boundary.
    pub fn drive(self, control: DriveControl) -> MechanicalTerminal {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.drive_guarded(control)))
        {
            Ok(terminal) => terminal,
            Err(_) => MechanicalTerminal::MechanicalFault(MechanicalFault::new(
                MechanicalFaultKind::SupervisorUnwind,
                "prepared-export supervisor unwound after retiring its worker boundary",
            )),
        }
    }

    fn drive_guarded(self, control: DriveControl) -> MechanicalTerminal {
        #[cfg(test)]
        let panic_after_admission = self.panic_after_admission;
        if control.cancellation.is_selected() {
            return MechanicalTerminal::Cancelled(Cancelled::mint());
        }
        if control.deadline_is_elapsed() {
            return MechanicalTerminal::DeadlineElapsed(DeadlineElapsed::mint());
        }
        if self.request.len() > MAX_REQUEST_BYTES {
            return MechanicalTerminal::MechanicalFault(MechanicalFault::new(
                MechanicalFaultKind::RequestWrite,
                "prepared-export request exceeds the bounded internal wire limit",
            ));
        }

        let (worker_program, worker_arguments) = match self.worker.resolve() {
            Ok(worker) => worker,
            Err(error) => {
                return MechanicalTerminal::MechanicalFault(MechanicalFault::new(
                    MechanicalFaultKind::WorkerAdmission,
                    format!("fresh prepared-export worker resolution failed: {error}"),
                ));
            }
        };
        let mut command = Command::new(worker_program);
        command
            .args(worker_arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_retirement_boundary(&mut command);

        let child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                return MechanicalTerminal::MechanicalFault(MechanicalFault::new(
                    MechanicalFaultKind::WorkerAdmission,
                    format!("fresh prepared-export worker spawn failed: {error}"),
                ));
            }
        };

        let mut guard = match DriveGuard::admit(child, self.request) {
            Ok(guard) => guard,
            Err(fault) => return MechanicalTerminal::MechanicalFault(fault),
        };
        #[cfg(test)]
        if panic_after_admission {
            panic!("injected prepared-export supervisor unwind after worker admission");
        }
        let selected = guard.select_terminal(&control);
        match guard.retire() {
            Ok(()) => selected.into_terminal(),
            Err(fault) => MechanicalTerminal::MechanicalFault(fault),
        }
    }

    #[cfg(test)]
    fn from_test_worker(
        worker_program: PathBuf,
        worker_arguments: Vec<OsString>,
        opaque_request: Vec<u8>,
    ) -> Self {
        Self {
            worker: WorkerLaunch::Exact {
                program: worker_program,
                arguments: worker_arguments,
            },
            request: opaque_request,
            panic_after_admission: false,
        }
    }
}

impl WorkerLaunch {
    fn resolve(self) -> io::Result<(PathBuf, Vec<OsString>)> {
        match self {
            #[cfg(test)]
            Self::Exact { program, arguments } => Ok((program, arguments)),
        }
    }
}

/// Mechanical cancellation observation shared with a drive supervisor.
///
/// This signal carries no worker handle and cannot mint a terminal result.
#[derive(Clone)]
pub struct DriveCancellation {
    selected: Arc<AtomicBool>,
}

/// Mechanical deadline and cancellation admitted for one drive.
pub struct DriveControl {
    deadline: Option<Instant>,
    cancellation: DriveCancellation,
}

impl DriveControl {
    pub fn unbounded() -> Self {
        Self {
            deadline: None,
            cancellation: DriveCancellation {
                selected: Arc::new(AtomicBool::new(false)),
            },
        }
    }

    pub fn with_deadline_after(duration: Duration) -> Self {
        Self {
            deadline: Instant::now().checked_add(duration),
            cancellation: DriveCancellation {
                selected: Arc::new(AtomicBool::new(false)),
            },
        }
    }

    pub fn cancellable() -> (Self, DriveCancellation) {
        let cancellation = DriveCancellation {
            selected: Arc::new(AtomicBool::new(false)),
        };
        (
            Self {
                deadline: None,
                cancellation: cancellation.clone(),
            },
            cancellation,
        )
    }

    pub fn cancellable_with_deadline_after(duration: Duration) -> (Self, DriveCancellation) {
        let cancellation = DriveCancellation {
            selected: Arc::new(AtomicBool::new(false)),
        };
        (
            Self {
                deadline: Instant::now().checked_add(duration),
                cancellation: cancellation.clone(),
            },
            cancellation,
        )
    }
}

impl DriveCancellation {
    pub fn cancel(&self) {
        self.selected.store(true, Ordering::Release);
    }

    fn is_selected(&self) -> bool {
        self.selected.load(Ordering::Acquire)
    }
}

impl DriveControl {
    fn deadline_is_elapsed(&self) -> bool {
        self.deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
    }
}

/// The sole public outcome of a quiescent prepared-export drive.
#[derive(Debug)]
pub enum MechanicalTerminal {
    Cargo(Cargo),
    Cancelled(Cancelled),
    DeadlineElapsed(DeadlineElapsed),
    MechanicalFault(MechanicalFault),
}

/// Bounded opaque worker cargo. Libbun does not interpret these bytes.
#[derive(Debug)]
pub struct Cargo {
    bytes: Vec<u8>,
    _evidence: TerminalEvidence,
}

impl Cargo {
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Debug)]
pub struct Cancelled {
    _evidence: TerminalEvidence,
}

#[derive(Debug)]
pub struct DeadlineElapsed {
    _evidence: TerminalEvidence,
}

#[derive(Debug)]
pub struct MechanicalFault {
    kind: MechanicalFaultKind,
    diagnostic: Box<str>,
    _evidence: TerminalEvidence,
}

impl MechanicalFault {
    pub fn kind(&self) -> MechanicalFaultKind {
        self.kind
    }

    pub fn diagnostic(&self) -> &str {
        &self.diagnostic
    }

    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
        let diagnostic = bounded_diagnostic(diagnostic.into()).into_boxed_str();
        Self {
            kind,
            diagnostic,
            _evidence: TerminalEvidence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MechanicalFaultKind {
    WorkerAdmission,
    RequestWrite,
    Preparation,
    InputLowering,
    JavaScriptRejection,
    CargoExtraction,
    WorkerProtocol,
    WorkerTermination,
    Pipe,
    Wait,
    Retirement,
    ThreadJoin,
    Correspondence,
    SupervisorUnwind,
}

#[derive(Debug)]
struct TerminalEvidence;

impl Cancelled {
    fn mint() -> Self {
        Self {
            _evidence: TerminalEvidence,
        }
    }
}

impl DeadlineElapsed {
    fn mint() -> Self {
        Self {
            _evidence: TerminalEvidence,
        }
    }
}

enum SelectedTerminal {
    Cargo(Vec<u8>),
    Cancelled,
    DeadlineElapsed,
    MechanicalFault(FaultSeed),
}

impl SelectedTerminal {
    fn into_terminal(self) -> MechanicalTerminal {
        match self {
            Self::Cargo(bytes) => MechanicalTerminal::Cargo(Cargo {
                bytes,
                _evidence: TerminalEvidence,
            }),
            Self::Cancelled => MechanicalTerminal::Cancelled(Cancelled::mint()),
            Self::DeadlineElapsed => MechanicalTerminal::DeadlineElapsed(DeadlineElapsed::mint()),
            Self::MechanicalFault(fault) => MechanicalTerminal::MechanicalFault(fault.into_fault()),
        }
    }
}

struct FaultSeed {
    kind: MechanicalFaultKind,
    diagnostic: String,
}

impl FaultSeed {
    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
        Self {
            kind,
            diagnostic: diagnostic.into(),
        }
    }

    fn into_fault(self) -> MechanicalFault {
        MechanicalFault::new(self.kind, self.diagnostic)
    }
}

struct DriveGuard {
    child: Option<Child>,
    process_boundary: ProcessBoundary,
    writer: Option<JoinHandle<()>>,
    reader: Option<JoinHandle<()>>,
    stderr: Option<JoinHandle<()>>,
    writer_result: Option<mpsc::Receiver<Result<(), FaultSeed>>>,
    reader_result: Option<mpsc::Receiver<Result<Vec<u8>, FaultSeed>>>,
    stderr_result: Option<mpsc::Receiver<Result<(), FaultSeed>>>,
    retired: bool,
}

impl DriveGuard {
    fn admit(child: Child, request: Vec<u8>) -> Result<Self, MechanicalFault> {
        let mut child = child;
        let process_boundary = match ProcessBoundary::for_child(&child) {
            Ok(boundary) => boundary,
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(MechanicalFault::new(
                    MechanicalFaultKind::WorkerAdmission,
                    format!("fresh worker retirement-boundary admission failed: {error}"),
                ));
            }
        };
        let mut guard = Self {
            child: Some(child),
            process_boundary,
            writer: None,
            reader: None,
            stderr: None,
            writer_result: None,
            reader_result: None,
            stderr_result: None,
            retired: false,
        };
        let child = guard
            .child
            .as_mut()
            .expect("new admission guard owns child");
        let mut stdin = child.stdin.take().ok_or_else(|| {
            MechanicalFault::new(
                MechanicalFaultKind::Pipe,
                "fresh worker did not provide its private request pipe",
            )
        })?;
        let mut stdout = child.stdout.take().ok_or_else(|| {
            MechanicalFault::new(
                MechanicalFaultKind::Pipe,
                "fresh worker did not provide its private terminal pipe",
            )
        })?;
        let mut stderr = child.stderr.take().ok_or_else(|| {
            MechanicalFault::new(
                MechanicalFaultKind::Pipe,
                "fresh worker did not provide its bounded diagnostic pipe",
            )
        })?;

        let (writer_tx, writer_result) = mpsc::sync_channel(1);
        let writer = thread::Builder::new()
            .name("libbun-prepared-export-writer".to_string())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    write_request(&mut stdin, &request).map_err(|error| {
                        FaultSeed::new(
                            MechanicalFaultKind::RequestWrite,
                            format!("private worker request write failed: {error}"),
                        )
                    })
                }))
                .unwrap_or_else(|_| {
                    Err(FaultSeed::new(
                        MechanicalFaultKind::ThreadJoin,
                        "private worker writer thread unwound",
                    ))
                });
                let _ = writer_tx.send(result);
            })
            .map_err(|error| {
                MechanicalFault::new(
                    MechanicalFaultKind::ThreadJoin,
                    format!("private worker writer thread spawn failed: {error}"),
                )
            })?;
        guard.writer = Some(writer);
        guard.writer_result = Some(writer_result);

        let (reader_tx, reader_result) = mpsc::sync_channel(1);
        let reader = thread::Builder::new()
            .name("libbun-prepared-export-reader".to_string())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    read_single_candidate(&mut stdout)
                }))
                .unwrap_or_else(|_| {
                    Err(FaultSeed::new(
                        MechanicalFaultKind::ThreadJoin,
                        "private worker reader thread unwound",
                    ))
                });
                let _ = reader_tx.send(result);
            })
            .map_err(|error| {
                MechanicalFault::new(
                    MechanicalFaultKind::ThreadJoin,
                    format!("private worker reader thread spawn failed: {error}"),
                )
            })?;
        guard.reader = Some(reader);
        guard.reader_result = Some(reader_result);

        let (stderr_tx, stderr_result) = mpsc::sync_channel(1);
        let stderr_thread = thread::Builder::new()
            .name("libbun-prepared-export-stderr".to_string())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    drain_bounded_stderr(&mut stderr)
                }))
                .unwrap_or_else(|_| {
                    Err(FaultSeed::new(
                        MechanicalFaultKind::ThreadJoin,
                        "private worker diagnostic thread unwound",
                    ))
                });
                let _ = stderr_tx.send(result);
            })
            .map_err(|error| {
                MechanicalFault::new(
                    MechanicalFaultKind::ThreadJoin,
                    format!("private worker diagnostic thread spawn failed: {error}"),
                )
            })?;
        guard.stderr = Some(stderr_thread);
        guard.stderr_result = Some(stderr_result);

        Ok(guard)
    }

    fn select_terminal(&mut self, control: &DriveControl) -> SelectedTerminal {
        let mut writer = None;
        let mut reader = None;
        let mut stderr = None;
        let mut exit = None;
        let mut boundary_closed_after_exit = false;
        let mut pipe_fault_observed_at = None;

        loop {
            receive_once(
                self.writer_result
                    .as_ref()
                    .expect("admitted drive owns writer result"),
                &mut writer,
            );
            receive_once(
                self.reader_result
                    .as_ref()
                    .expect("admitted drive owns reader result"),
                &mut reader,
            );
            receive_once(
                self.stderr_result
                    .as_ref()
                    .expect("admitted drive owns diagnostic result"),
                &mut stderr,
            );

            if exit.is_none() {
                match self
                    .child
                    .as_mut()
                    .expect("live drive owns child")
                    .try_wait()
                {
                    Ok(status) => exit = status,
                    Err(error) => {
                        return SelectedTerminal::MechanicalFault(FaultSeed::new(
                            MechanicalFaultKind::Wait,
                            format!("fresh worker wait observation failed: {error}"),
                        ));
                    }
                }
            }

            if let Some(status) = exit
                && !status.success()
            {
                return SelectedTerminal::MechanicalFault(worker_exit_fault(status));
            }

            if writer.as_ref().is_some_and(Result::is_err)
                || reader.as_ref().is_some_and(Result::is_err)
                || stderr.as_ref().is_some_and(Result::is_err)
            {
                let first_observation = pipe_fault_observed_at.get_or_insert_with(Instant::now);
                // A child that exits nonzero commonly closes its pipes just
                // before wait status becomes observable. Give wait a bounded
                // opportunity to preserve the stronger termination class.
                if exit.is_some() || first_observation.elapsed() >= Duration::from_millis(10) {
                    if let Some(Err(fault)) = writer.take_if(|result| result.is_err()) {
                        return SelectedTerminal::MechanicalFault(fault);
                    }
                    if let Some(Err(fault)) = reader.take_if(|result| result.is_err()) {
                        return SelectedTerminal::MechanicalFault(fault);
                    }
                    if let Some(Err(fault)) = stderr.take_if(|result| result.is_err()) {
                        return SelectedTerminal::MechanicalFault(fault);
                    }
                }
            }

            if control.cancellation.is_selected() {
                return SelectedTerminal::Cancelled;
            }
            if control.deadline_is_elapsed() {
                return SelectedTerminal::DeadlineElapsed;
            }

            if exit.is_some() && !boundary_closed_after_exit {
                if let Err(error) = self.process_boundary.terminate_descendants() {
                    return SelectedTerminal::MechanicalFault(FaultSeed::new(
                        MechanicalFaultKind::Retirement,
                        format!("fresh worker descendant retirement failed: {error}"),
                    ));
                }
                boundary_closed_after_exit = true;
            }

            if exit.is_some()
                && writer.as_ref().is_some_and(Result::is_ok)
                && reader.as_ref().is_some_and(Result::is_ok)
                && stderr.as_ref().is_some_and(Result::is_ok)
            {
                let Ok(bytes) = reader.take().expect("reader result checked") else {
                    unreachable!("reader error returned above")
                };
                return SelectedTerminal::Cargo(bytes);
            }

            thread::sleep(Duration::from_millis(1));
        }
    }

    fn retire(&mut self) -> Result<(), MechanicalFault> {
        if self.retired {
            return Ok(());
        }

        let mut cleanup_fault = None;
        if let Err(error) = self.process_boundary.terminate_descendants() {
            cleanup_fault.get_or_insert_with(|| {
                MechanicalFault::new(
                    MechanicalFaultKind::Retirement,
                    format!("worker process-boundary termination failed: {error}"),
                )
            });
        }
        if let Some(child) = self.child.as_mut()
            && let Err(error) = child.wait()
        {
            cleanup_fault.get_or_insert_with(|| {
                MechanicalFault::new(
                    MechanicalFaultKind::Wait,
                    format!("worker reap failed: {error}"),
                )
            });
        }

        for (name, handle) in [
            ("writer", self.writer.take()),
            ("reader", self.reader.take()),
            ("diagnostic", self.stderr.take()),
        ] {
            if handle.is_some_and(|handle| handle.join().is_err()) {
                cleanup_fault.get_or_insert_with(|| {
                    MechanicalFault::new(
                        MechanicalFaultKind::ThreadJoin,
                        format!("worker {name} thread panicked during retirement"),
                    )
                });
            }
        }
        self.child.take();
        self.retired = cleanup_fault.is_none();
        cleanup_fault.map_or(Ok(()), Err)
    }
}

impl Drop for DriveGuard {
    fn drop(&mut self) {
        if self.retired {
            return;
        }
        if self.retire().is_err() {
            // Continuing the host without proof of irreversible worker
            // retirement would violate the mechanical boundary.
            std::process::abort();
        }
    }
}

fn receive_once<T>(receiver: &mpsc::Receiver<T>, slot: &mut Option<T>) {
    if slot.is_none()
        && let Ok(value) = receiver.try_recv()
    {
        *slot = Some(value);
    }
}

fn write_request(writer: &mut impl Write, request: &[u8]) -> io::Result<()> {
    libbun_prepared_export_wire::write_drive_request(writer, request)
}

fn read_single_candidate(reader: &mut impl Read) -> Result<Vec<u8>, FaultSeed> {
    let mut header = [0_u8; TERMINAL_HEADER_LEN];
    reader.read_exact(&mut header).map_err(|error| {
        FaultSeed::new(
            MechanicalFaultKind::WorkerProtocol,
            format!("worker terminal frame is missing or truncated: {error}"),
        )
    })?;
    if header[..4] != TERMINAL_MAGIC {
        return Err(FaultSeed::new(
            MechanicalFaultKind::WorkerProtocol,
            "worker terminal frame has invalid magic",
        ));
    }
    let version = u16::from_be_bytes([header[4], header[5]]);
    if version != VERSION {
        return Err(FaultSeed::new(
            MechanicalFaultKind::Correspondence,
            format!("worker wire version {version} does not match {VERSION}"),
        ));
    }
    let candidate_kind = header[6];
    let length = u32::from_be_bytes([header[7], header[8], header[9], header[10]]) as usize;
    if length > MAX_CANDIDATE_BYTES {
        return Err(FaultSeed::new(
            MechanicalFaultKind::WorkerProtocol,
            format!("worker terminal candidate length {length} exceeds bounded limit"),
        ));
    }
    let mut bytes = vec![0_u8; length];
    reader.read_exact(&mut bytes).map_err(|error| {
        FaultSeed::new(
            MechanicalFaultKind::WorkerProtocol,
            format!("worker terminal candidate payload is truncated: {error}"),
        )
    })?;
    let mut extra = [0_u8; 1];
    match reader.read(&mut extra) {
        Ok(0) if candidate_kind == 0 => Ok(bytes),
        Ok(0) => Err(worker_candidate_fault(candidate_kind, bytes)),
        Ok(_) => Err(FaultSeed::new(
            MechanicalFaultKind::WorkerProtocol,
            "worker emitted duplicate or contradictory terminal data",
        )),
        Err(error) => Err(FaultSeed::new(
            MechanicalFaultKind::Pipe,
            format!("worker terminal pipe EOF observation failed: {error}"),
        )),
    }
}

fn worker_candidate_fault(kind: u8, diagnostic: Vec<u8>) -> FaultSeed {
    let kind = match kind {
        value if value == WorkerFaultKind::Preparation as u8 => MechanicalFaultKind::Preparation,
        value if value == WorkerFaultKind::InputLowering as u8 => {
            MechanicalFaultKind::InputLowering
        }
        value if value == WorkerFaultKind::JavaScriptRejection as u8 => {
            MechanicalFaultKind::JavaScriptRejection
        }
        value if value == WorkerFaultKind::CargoExtraction as u8 => {
            MechanicalFaultKind::CargoExtraction
        }
        value if value == WorkerFaultKind::Internal as u8 => MechanicalFaultKind::WorkerTermination,
        value => {
            return FaultSeed::new(
                MechanicalFaultKind::WorkerProtocol,
                format!("worker emitted unsupported terminal candidate kind {value}"),
            );
        }
    };
    FaultSeed::new(kind, String::from_utf8_lossy(&diagnostic).into_owned())
}

fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
    let mut total = 0_usize;
    let mut buffer = [0_u8; 8192];
    loop {
        let read = stderr.read(&mut buffer).map_err(|error| {
            FaultSeed::new(
                MechanicalFaultKind::Pipe,
                format!("worker diagnostic pipe read failed: {error}"),
            )
        })?;
        if read == 0 {
            return Ok(());
        }
        total = total.saturating_add(read);
        if total > MAX_CANDIDATE_BYTES {
            return Err(FaultSeed::new(
                MechanicalFaultKind::Pipe,
                "worker diagnostic output exceeds bounded limit",
            ));
        }
    }
}

fn worker_exit_fault(status: ExitStatus) -> FaultSeed {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return FaultSeed::new(
                MechanicalFaultKind::WorkerTermination,
                format!("fresh worker terminated by signal {signal}"),
            );
        }
    }
    FaultSeed::new(
        MechanicalFaultKind::WorkerTermination,
        format!("fresh worker exited unsuccessfully with {status}"),
    )
}

fn bounded_diagnostic(mut diagnostic: String) -> String {
    if diagnostic.len() <= MAX_DIAGNOSTIC_BYTES {
        return diagnostic;
    }
    let mut boundary = MAX_DIAGNOSTIC_BYTES;
    while !diagnostic.is_char_boundary(boundary) {
        boundary -= 1;
    }
    diagnostic.truncate(boundary);
    diagnostic
}

struct ProcessBoundary {
    #[cfg(unix)]
    process_group: libc::pid_t,
    #[cfg(windows)]
    job: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(not(any(unix, windows)))]
    process_id: u32,
}

impl ProcessBoundary {
    fn for_child(child: &Child) -> io::Result<Self> {
        #[cfg(unix)]
        {
            Ok(Self {
                process_group: child.id() as libc::pid_t,
            })
        }
        #[cfg(windows)]
        {
            use std::mem::size_of;
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
            use windows_sys::Win32::System::JobObjects::CreateJobObjectW;
            use windows_sys::Win32::System::JobObjects::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            use windows_sys::Win32::System::JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION;
            use windows_sys::Win32::System::JobObjects::JobObjectExtendedLimitInformation;
            use windows_sys::Win32::System::JobObjects::SetInformationJobObject;

            let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
            if job.is_null() {
                return Err(io::Error::last_os_error());
            }
            let mut information: JOBOBJECT_EXTENDED_LIMIT_INFORMATION =
                unsafe { std::mem::zeroed() };
            information.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let configured = unsafe {
                SetInformationJobObject(
                    job,
                    JobObjectExtendedLimitInformation,
                    (&mut information as *mut JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
                    size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                )
            };
            let assigned = configured != 0
                && unsafe { AssignProcessToJobObject(job, child.as_raw_handle() as _) } != 0;
            if !assigned {
                unsafe { windows_sys::Win32::Foundation::CloseHandle(job) };
                return Err(io::Error::last_os_error());
            }
            Ok(Self { job })
        }
        #[cfg(not(any(unix, windows)))]
        {
            Ok(Self {
                process_id: child.id(),
            })
        }
    }

    fn terminate_descendants(&mut self) -> io::Result<()> {
        #[cfg(unix)]
        {
            let result = unsafe { libc::kill(-self.process_group, libc::SIGKILL) };
            if result == 0 {
                return Ok(());
            }
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ESRCH) {
                Ok(())
            } else {
                Err(error)
            }
        }
        #[cfg(windows)]
        {
            if self.job.is_null() {
                return Ok(());
            }
            let terminated =
                unsafe { windows_sys::Win32::System::JobObjects::TerminateJobObject(self.job, 1) };
            let close = unsafe { windows_sys::Win32::Foundation::CloseHandle(self.job) };
            self.job = std::ptr::null_mut();
            if terminated == 0 || close == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _ = self.process_id;
            Ok(())
        }
    }
}

#[cfg(windows)]
impl Drop for ProcessBoundary {
    fn drop(&mut self) {
        if !self.job.is_null() {
            unsafe { windows_sys::Win32::Foundation::CloseHandle(self.job) };
            self.job = std::ptr::null_mut();
        }
    }
}

#[cfg(unix)]
fn configure_retirement_boundary(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

#[cfg(not(unix))]
fn configure_retirement_boundary(_command: &mut Command) {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use static_assertions::assert_not_impl_any;

    assert_not_impl_any!(PreparedExport: Clone, Copy, Serialize, DeserializeOwned);
    assert_not_impl_any!(Cargo: Clone, Copy, Serialize, DeserializeOwned);
    assert_not_impl_any!(Cancelled: Clone, Copy, Serialize, DeserializeOwned);
    assert_not_impl_any!(DeadlineElapsed: Clone, Copy, Serialize, DeserializeOwned);
    assert_not_impl_any!(MechanicalFault: Clone, Copy, Serialize, DeserializeOwned);

    enum WorkerBehavior {
        Cargo(Vec<u8>),
        CargoThenHang(Vec<u8>),
        Malformed,
        Truncated,
        Duplicate(Vec<u8>),
        Oversized,
        NeverSettles,
        ExitNonzero,
        CargoWithInheritedDescriptor(Vec<u8>),
        ProcessId,
        WrongVersion,
        TypedRejection,
        LargeStderr(Vec<u8>),
        Abort,
    }

    impl PreparedExport {
        fn test_worker(behavior: WorkerBehavior) -> Self {
            const WORKER: &str = r#"
import os
import subprocess
import sys
import time

mode = sys.argv[1]
payload = bytes.fromhex(sys.argv[2]) if len(sys.argv) > 2 else b''
sys.stdin.buffer.read()

def frame(value):
    return b'LBPT' + (1).to_bytes(2, 'big') + b'\x00' + len(value).to_bytes(4, 'big') + value

if mode == 'cargo':
    sys.stdout.buffer.write(frame(payload))
elif mode == 'cargo-hang':
    sys.stdout.buffer.write(frame(payload))
    sys.stdout.buffer.flush()
    time.sleep(300)
elif mode == 'malformed':
    sys.stdout.buffer.write(b'wrong-frame')
elif mode == 'truncated':
    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x00' + (99).to_bytes(4, 'big') + b'x')
elif mode == 'duplicate':
    sys.stdout.buffer.write(frame(payload) + frame(payload))
elif mode == 'oversized':
    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x00' + (16 * 1024 * 1024 + 1).to_bytes(4, 'big'))
elif mode == 'never':
    time.sleep(300)
elif mode == 'nonzero':
    sys.exit(23)
elif mode == 'inherited-fd':
    subprocess.Popen(['sleep', '300'])
    sys.stdout.buffer.write(frame(payload))
elif mode == 'pid':
    sys.stdout.buffer.write(frame(str(os.getpid()).encode()))
elif mode == 'wrong-version':
    sys.stdout.buffer.write(b'LBPT' + (99).to_bytes(2, 'big') + b'\x00' + (0).to_bytes(4, 'big'))
elif mode == 'typed-rejection':
    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x03' + len(payload).to_bytes(4, 'big') + payload)
elif mode == 'large-stderr':
    sys.stderr.buffer.write(b'x' * (2 * 1024 * 1024))
    sys.stderr.buffer.flush()
    sys.stdout.buffer.write(frame(payload))
elif mode == 'abort':
    os.abort()
else:
    sys.exit(24)
sys.stdout.buffer.flush()
"#;
            let (mode, payload) = match behavior {
                WorkerBehavior::Cargo(payload) => ("cargo", payload),
                WorkerBehavior::CargoThenHang(payload) => ("cargo-hang", payload),
                WorkerBehavior::Malformed => ("malformed", Vec::new()),
                WorkerBehavior::Truncated => ("truncated", Vec::new()),
                WorkerBehavior::Duplicate(payload) => ("duplicate", payload),
                WorkerBehavior::Oversized => ("oversized", Vec::new()),
                WorkerBehavior::NeverSettles => ("never", Vec::new()),
                WorkerBehavior::ExitNonzero => ("nonzero", Vec::new()),
                WorkerBehavior::CargoWithInheritedDescriptor(payload) => ("inherited-fd", payload),
                WorkerBehavior::ProcessId => ("pid", Vec::new()),
                WorkerBehavior::WrongVersion => ("wrong-version", Vec::new()),
                WorkerBehavior::TypedRejection => ("typed-rejection", b"promise rejected".to_vec()),
                WorkerBehavior::LargeStderr(payload) => ("large-stderr", payload),
                WorkerBehavior::Abort => ("abort", Vec::new()),
            };
            Self::from_test_worker(
                PathBuf::from("python3"),
                vec![
                    OsString::from("-c"),
                    OsString::from(WORKER),
                    OsString::from(mode),
                    OsString::from(hex(&payload)),
                ],
                b"one opaque invocation".to_vec(),
            )
        }
    }

    fn hex(bytes: &[u8]) -> String {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            encoded.push(DIGITS[(byte >> 4) as usize] as char);
            encoded.push(DIGITS[(byte & 0xf) as usize] as char);
        }
        encoded
    }

    #[test]
    fn opaque_provider_looking_bytes_cross_the_real_drive_unchanged() {
        let bytes = br#"{\"kind\":\"err\",\"provider\":true}\xff\0not-tson"#.to_vec();
        let prepared = PreparedExport::test_worker(WorkerBehavior::Cargo(bytes.clone()));

        let MechanicalTerminal::Cargo(cargo) = prepared.drive(DriveControl::unbounded()) else {
            panic!("expected cargo terminal");
        };

        assert_eq!(cargo.into_bytes(), bytes);
    }

    #[test]
    fn cargo_followed_by_a_hung_worker_is_discarded_at_deadline() {
        let prepared = PreparedExport::test_worker(WorkerBehavior::CargoThenHang(b"late".to_vec()));

        assert!(matches!(
            prepared.drive(DriveControl::with_deadline_after(Duration::from_millis(75))),
            MechanicalTerminal::DeadlineElapsed(_)
        ));
    }

    #[test]
    fn cancellation_before_spawn_does_not_require_a_worker_asset() {
        let prepared = PreparedExport::from_test_worker(
            PathBuf::from("/definitely/missing/libbun-worker"),
            Vec::new(),
            Vec::new(),
        );
        let (control, cancellation) = DriveControl::cancellable();
        cancellation.cancel();

        assert!(matches!(
            prepared.drive(control),
            MechanicalTerminal::Cancelled(_)
        ));
    }

    #[test]
    fn never_settling_worker_is_cancelled_and_retired() {
        let prepared = PreparedExport::test_worker(WorkerBehavior::NeverSettles);
        let (control, cancellation) = DriveControl::cancellable();
        let cancelling = thread::spawn(move || {
            thread::sleep(Duration::from_millis(40));
            cancellation.cancel();
        });

        assert!(matches!(
            prepared.drive(control),
            MechanicalTerminal::Cancelled(_)
        ));
        cancelling.join().expect("cancellation thread joins");
    }

    #[test]
    fn malformed_truncated_duplicate_and_oversized_frames_are_typed_faults() {
        for prepared in [
            PreparedExport::test_worker(WorkerBehavior::Malformed),
            PreparedExport::test_worker(WorkerBehavior::Truncated),
            PreparedExport::test_worker(WorkerBehavior::Duplicate(b"x".to_vec())),
            PreparedExport::test_worker(WorkerBehavior::Oversized),
        ] {
            let MechanicalTerminal::MechanicalFault(fault) =
                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
            else {
                panic!("invalid worker frame must be a typed fault");
            };
            assert_eq!(fault.kind(), MechanicalFaultKind::WorkerProtocol);
        }
    }

    #[test]
    fn nonzero_worker_exit_is_a_typed_fault_after_reap() {
        let prepared = PreparedExport::test_worker(WorkerBehavior::ExitNonzero);
        let MechanicalTerminal::MechanicalFault(fault) =
            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
        else {
            panic!("nonzero worker exit must be a typed fault");
        };
        assert_eq!(fault.kind(), MechanicalFaultKind::WorkerTermination);
    }

    #[test]
    fn inherited_protocol_descriptor_descendant_is_retired_before_cargo_returns() {
        let bytes = b"retired descendant".to_vec();
        let prepared = PreparedExport::test_worker(WorkerBehavior::CargoWithInheritedDescriptor(
            bytes.clone(),
        ));

        let MechanicalTerminal::Cargo(cargo) =
            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
        else {
            panic!("successful leader plus retired descendant should return cargo");
        };
        assert_eq!(cargo.into_bytes(), bytes);
    }

    #[test]
    fn each_drive_uses_a_fresh_worker_process() {
        let pid = |prepared: PreparedExport| {
            let MechanicalTerminal::Cargo(cargo) =
                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
            else {
                panic!("worker pid fixture must return cargo");
            };
            cargo.into_bytes()
        };

        assert_ne!(
            pid(PreparedExport::test_worker(WorkerBehavior::ProcessId)),
            pid(PreparedExport::test_worker(WorkerBehavior::ProcessId))
        );
    }

    #[test]
    fn wrong_version_and_worker_rejection_remain_distinct_typed_faults() {
        let MechanicalTerminal::MechanicalFault(version) =
            PreparedExport::test_worker(WorkerBehavior::WrongVersion)
                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
        else {
            panic!("wrong wire version must fault");
        };
        assert_eq!(version.kind(), MechanicalFaultKind::Correspondence);

        let MechanicalTerminal::MechanicalFault(rejection) =
            PreparedExport::test_worker(WorkerBehavior::TypedRejection)
                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
        else {
            panic!("worker rejection must fault");
        };
        assert_eq!(rejection.kind(), MechanicalFaultKind::JavaScriptRejection);
    }

    #[test]
    fn large_worker_stderr_is_drained_without_blocking_cargo() {
        let bytes = b"after diagnostic flood".to_vec();
        let MechanicalTerminal::Cargo(cargo) =
            PreparedExport::test_worker(WorkerBehavior::LargeStderr(bytes.clone()))
                .drive(DriveControl::with_deadline_after(Duration::from_secs(3)))
        else {
            panic!("bounded diagnostic drain should not block cargo");
        };
        assert_eq!(cargo.into_bytes(), bytes);
    }

    #[test]
    fn worker_abort_is_reaped_as_signal_termination() {
        let terminal = PreparedExport::test_worker(WorkerBehavior::Abort)
            .drive(DriveControl::with_deadline_after(Duration::from_secs(5)));
        let MechanicalTerminal::MechanicalFault(fault) = terminal else {
            panic!("aborted worker must fault, got {terminal:?}");
        };
        assert_eq!(fault.kind(), MechanicalFaultKind::WorkerTermination);
    }

    #[test]
    fn supervisor_unwind_after_admission_retires_before_fault_return() {
        let mut prepared = PreparedExport::test_worker(WorkerBehavior::NeverSettles);
        prepared.panic_after_admission = true;
        let terminal = prepared.drive(DriveControl::unbounded());
        let MechanicalTerminal::MechanicalFault(fault) = terminal else {
            panic!("supervisor unwind must become typed fault");
        };
        assert_eq!(fault.kind(), MechanicalFaultKind::SupervisorUnwind);
    }

    #[test]
    fn cancellation_deadline_race_selects_exactly_one_post_retirement_terminal() {
        for _ in 0..8 {
            let prepared = PreparedExport::test_worker(WorkerBehavior::NeverSettles);
            let (control, cancellation) =
                DriveControl::cancellable_with_deadline_after(Duration::from_millis(30));
            let cancelling = thread::spawn(move || {
                thread::sleep(Duration::from_millis(30));
                cancellation.cancel();
            });
            let terminal = prepared.drive(control);
            cancelling.join().expect("cancellation thread joins");
            assert!(matches!(
                terminal,
                MechanicalTerminal::Cancelled(_) | MechanicalTerminal::DeadlineElapsed(_)
            ));
        }
    }

    #[test]
    fn oversized_request_faults_before_worker_spawn() {
        let prepared = PreparedExport::from_test_worker(
            PathBuf::from("/definitely/missing/libbun-worker"),
            Vec::new(),
            vec![0; MAX_REQUEST_BYTES + 1],
        );
        let MechanicalTerminal::MechanicalFault(fault) = prepared.drive(DriveControl::unbounded())
        else {
            panic!("oversized request must fault");
        };
        assert_eq!(fault.kind(), MechanicalFaultKind::RequestWrite);
    }
}
