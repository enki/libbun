# Process, Drop, shutdown caller, and external fixture report

Libbun source: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb (cb964de8ab8162449fbe95959bf34d231570aa5c)

Adjacent swarm source: 95323ff17cb29928e31467f651ef03bae2099c14 (43b47bbd49a6053d270b3e15cc141cb1b1bb86da)

## Libbun process, thread, Drop, cancellation, and shutdown callers

Meaning: Enumerates every candidate process/thread/destructor/cancellation/shutdown caller.

Expected result: Exit 0; no omitted libbun process or Drop family.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'Command::new|\.spawn\(|Child|try_wait|wait_with_output|\.wait\(|\.kill\(|JoinHandle|\.join\(|impl Drop|process::abort|catch_unwind|cancel|deadline|retire|shutdown' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src/prepared_export.rs native/src/lib.rs runtime/src/main.rs wire/src/lib.rs tests scripts

Pattern: Command::new|\.spawn\(|Child|try_wait|wait_with_output|\.wait\(|\.kill\(|JoinHandle|\.join\(|impl Drop|process::abort|catch_unwind|cancel|deadline|retire|shutdown

Pathspecs: src/prepared_export.rs native/src/lib.rs runtime/src/main.rs wire/src/lib.rs tests scripts

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:330:        // `shutdown`, which consumes all public operations through the facade.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:523:        let bundle_dir = tempdir.path().join(format!("{module_id}.bundle"));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:529:            let path = bundle_dir.join(module_path);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:544:        let entry_module = bundle_dir.join(bundle.entry_module);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:897:            .join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:6:use std::process::Child;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:7:use std::process::ChildStderr;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:16:use std::thread::JoinHandle;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:49:    /// export. Every return path has retired the fresh worker boundary.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:51:        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.drive_guarded(control)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:64:        if control.cancellation.is_selected() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:67:        if control.deadline_is_elapsed() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:86:        let mut command = Command::new(worker_program);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:92:        configure_retirement_boundary(&mut command);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:94:        let child = match command.spawn() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:113:        match guard.retire() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:145:/// Mechanical cancellation observation shared with a drive supervisor.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:153:/// Mechanical deadline and cancellation admitted for one drive.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:155:    deadline: Option<Instant>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:156:    cancellation: DriveCancellation,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:162:            deadline: None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:163:            cancellation: DriveCancellation {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:169:    pub fn with_deadline_after(duration: Duration) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:171:            deadline: Instant::now().checked_add(duration),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:172:            cancellation: DriveCancellation {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:178:    pub fn cancellable() -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:179:        let cancellation = DriveCancellation {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:184:                deadline: None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:185:                cancellation: cancellation.clone(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:187:            cancellation,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:191:    pub fn cancellable_with_deadline_after(duration: Duration) -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:192:        let cancellation = DriveCancellation {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:197:                deadline: Instant::now().checked_add(duration),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:198:                cancellation: cancellation.clone(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:200:            cancellation,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:206:    pub fn cancel(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:216:    fn deadline_is_elapsed(&self) -> bool {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:217:        self.deadline
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:218:            .is_some_and(|deadline| Instant::now() >= deadline)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:358:    child: Option<Child>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:360:    writer: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:361:    reader: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:362:    stderr: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:366:    retired: bool,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:370:    fn admit(child: Child, request: Vec<u8>) -> Result<Self, MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:375:                let _ = child.kill();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:376:                let _ = child.wait();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:379:                    format!("fresh worker retirement-boundary admission failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:392:            retired: false,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:420:            .spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:421:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:449:            .spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:450:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:473:            .spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:474:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:530:                    .try_wait()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:569:            if control.cancellation.is_selected() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:572:            if control.deadline_is_elapsed() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:580:                        format!("fresh worker descendant retirement failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:601:    fn retire(&mut self) -> Result<(), MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:602:        if self.retired {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:616:            && let Err(error) = child.wait()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:631:            if handle.is_some_and(|handle| handle.join().is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:635:                        format!("worker {name} thread panicked during retirement"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:641:        self.retired = cleanup_fault.is_none();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:646:impl Drop for DriveGuard {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:648:        if self.retired {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:651:        if self.retire().is_err() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:653:            // retirement would violate the mechanical boundary.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:654:            std::process::abort();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:745:fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:807:    fn for_child(child: &Child) -> io::Result<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:894:impl Drop for ProcessBoundary {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:904:fn configure_retirement_boundary(command: &mut Command) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:917:fn configure_retirement_boundary(_command: &mut Command) {}
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1053:    fn cargo_followed_by_a_hung_worker_is_discarded_at_deadline() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1057:            prepared.drive(DriveControl::with_deadline_after(Duration::from_millis(75))),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1063:    fn cancellation_before_spawn_does_not_require_a_worker_asset() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1069:        let (control, cancellation) = DriveControl::cancellable();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1070:        cancellation.cancel();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1079:    fn never_settling_worker_is_cancelled_and_retired() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1081:        let (control, cancellation) = DriveControl::cancellable();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1082:        let cancelling = thread::spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1084:            cancellation.cancel();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1091:        cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1103:                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1115:            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1123:    fn inherited_protocol_descriptor_descendant_is_retired_before_cargo_returns() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1124:        let bytes = b"retired descendant".to_vec();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1130:            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1132:            panic!("successful leader plus retired descendant should return cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1141:                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1158:                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1166:                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1178:                .drive(DriveControl::with_deadline_after(Duration::from_secs(3)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1188:            .drive(DriveControl::with_deadline_after(Duration::from_secs(5)));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1196:    fn supervisor_unwind_after_admission_retires_before_fault_return() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1207:    fn cancellation_deadline_race_selects_exactly_one_post_retirement_terminal() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1210:            let (control, cancellation) =
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1211:                DriveControl::cancellable_with_deadline_after(Duration::from_millis(30));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1212:            let cancelling = thread::spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1214:                cancellation.cancel();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1217:            cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:7:    let manifest = repository.join("tests/fixtures/public_api_boundary/Cargo.toml");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:10:        .unwrap_or_else(|| repository.join("target"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:11:        .join("external-public-api-boundary");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:13:    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))

## Vendored VM termination, reset, drain, and deinit callers

Meaning: Binds the JSC cooperative interrupt/reset, drain, teardown, and empty C++ deinit facts.

Expected result: Exit 0; all named VM/global/C++ lifecycle operations are visible.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'request_termination|clear_termination|notify_need_termination|has_termination_request|drain_microtasks|JSC__VM__deinit|pub fn destroy|pub fn deinit|terminate_all_workers|shutdown' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- vendor/bun/src/jsc/VirtualMachine.rs vendor/bun/src/jsc/VM.rs vendor/bun/src/jsc/JSGlobalObject.rs vendor/bun/src/jsc/virtual_machine_exports.rs vendor/bun/src/jsc/bindings/bindings.cpp vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp vendor/bun/src/jsc/VirtualMachine.zig

Pattern: request_termination|clear_termination|notify_need_termination|has_termination_request|drain_microtasks|JSC__VM__deinit|pub fn destroy|pub fn deinit|terminate_all_workers|shutdown

Pathspecs: vendor/bun/src/jsc/VirtualMachine.rs vendor/bun/src/jsc/VM.rs vendor/bun/src/jsc/JSGlobalObject.rs vendor/bun/src/jsc/virtual_machine_exports.rs vendor/bun/src/jsc/bindings/bindings.cpp vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp vendor/bun/src/jsc/VirtualMachine.zig

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/JSGlobalObject.rs:213:    pub fn request_termination(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/JSGlobalObject.rs:218:    pub fn clear_termination_exception(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/JSGlobalObject.rs:619:        self.vm().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:19:    safe fn JSC__VM__deinit(vm: &VM, global_object: &JSGlobalObject);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:74:    pub fn deinit(&self, global_object: &JSGlobalObject) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:75:        JSC__VM__deinit(self, global_object)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:160:    pub fn notify_need_termination(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:187:    pub fn has_termination_request(&self) -> bool {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:191:    pub fn clear_has_termination_request(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VM.rs:208:    pub fn drain_microtasks(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1003:    /// Per-callback hot path: `drain_microtasks_with_global` calls
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1182:    pub fn drain_microtasks(&mut self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1183:        let _ = self.event_loop_mut().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1468:        // shutdown begins. Grab the config and null it out to make this
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1478:        // profile but before shutdown.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1528:            // live worker and wait for each to reach shutdown() first.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1531:                // until each unparks at shutdown().
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1532:                (hooks.terminate_all_workers_and_wait)(10_000);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1622:    /// idle — folding it into `auto_tick` would change shutdown semantics.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:1737:    pub terminate_all_workers_and_wait: fn(timeout_ms: u64),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:2402:        let _ = self.event_loop_mut().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:2793:    pub fn deinit(this: *mut IPCInstance) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:3187:            bun_core::debug_warn!("unhandledRejection during shutdown.");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:3200:            let _ = this.event_loop_mut().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:3278:                if self.event_loop_mut().drain_microtasks().is_err() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:4302:    pub fn destroy(&mut self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:4515:        let _ = self.event_loop_mut().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.rs:4569:        let _ = self.event_loop_mut().drain_microtasks();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:597:        Output.debugWarn("unhandledRejection during shutdown.", .{});
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:924:    // Write CPU profile if profiling was enabled - do this FIRST before any shutdown begins
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:933:    // Write heap profile if profiling was enabled - do this after CPU profile but before shutdown
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:972:        // shutdown() (past all resolver access) first. Node.js does the
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:2109:pub fn deinit(this: *VirtualMachine) void {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/VirtualMachine.zig:3931:    pub fn deinit(this: *IPCInstance) void {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/bindings/bindings.cpp:4940:void JSC__VM__deinit(JSC::VM* arg1, JSC::JSGlobalObject* globalObject)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/bun/src/jsc/virtual_machine_exports.rs:35:pub fn drain_microtasks() {

## Adjacent consumer, transport, process, Drop, and shutdown callers

Meaning: Traces every attached external consumer/process/shutdown edge and current raw reconstruction.

Expected result: Exit 0; all attached direct callers and compatibility edges are visible.

Command: git -C /home/ubuntu/swarm grep -n -E 'invoke_manifest_resolved_call|begin_execution_session|shutdown|impl Drop|Command::new|\.spawn\(|wait_with_output|libbun|ProviderRequest|into_call_input_and_output_settlement|into_contract_and_module' 95323ff17cb29928e31467f651ef03bae2099c14 -- crates/ss-runtime-external-capability-provider-owner/src/lib.rs crates/swarm-provider-host-set/src/external_transport.rs crates/swarm-provider-host-set/src/provider_host_set.rs crates/ss/src/product.rs crates/ss/tests/external_capability_provider.rs

Pattern: invoke_manifest_resolved_call|begin_execution_session|shutdown|impl Drop|Command::new|\.spawn\(|wait_with_output|libbun|ProviderRequest|into_call_input_and_output_settlement|into_contract_and_module

Pathspecs: crates/ss-runtime-external-capability-provider-owner/src/lib.rs crates/swarm-provider-host-set/src/external_transport.rs crates/swarm-provider-host-set/src/provider_host_set.rs crates/ss/src/product.rs crates/ss/tests/external_capability_provider.rs

Exit: 0

Output:

95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:30:    Libbun(#[from] libbun::LibbunError),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:38:    libbun::BunProviderBackend<libbun::dynamic::DynamicBunRuntime>;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:79:        let mut config = libbun::BunRuntimeConfig::new("libbun", working_directory);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:80:        config.stdout = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:81:        config.stderr = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:82:        config.log = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:90:    pub fn shutdown(&mut self) -> SsExternalCapabilityProviderResult<()> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:91:        self.backend.shutdown()?;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:96:pub fn install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:103:    let host = ExternalTransportCapabilityProviderHost::libbun_for_ss_external_capability_provider_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:122:    fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:128:            .into_call_input_and_output_settlement_for_durable_external_provider_owner_v1();
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:130:            call_authority.into_contract_and_module_for_durable_external_provider_owner_v1();
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:158:            "ss-libbun-external-provider-{}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:161:        let provider_adapter_source = libbun_provider_adapter_source_for_selected_route(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:167:        let request = libbun::ProviderRequest {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:168:            contract: libbun::ProviderContractIdentity {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:173:            domain: libbun::ProviderDomainClass::JavaScriptExternalTransport,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:174:            module: libbun::BunModuleSpec::Source {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:179:            input: libbun::StructuralValue(input),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:181:        let descriptor = libbun::ProviderInvocationDescriptor::new(invocation_id.clone())
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:182:            .with_output_policy(libbun::InvocationOutputPolicy::Drop);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:189:                    libbun::ProviderSettleOptions::new(libbun::ProviderDeadline::after(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:198:            libbun::SettledProviderReceipt::Ready {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:199:                result: libbun::ProviderCallResult::Ok(output),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:202:            libbun::SettledProviderReceipt::Ready {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:203:                result: libbun::ProviderCallResult::Err(error),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:213:            libbun::SettledProviderReceipt::Failed(failure) => {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:232:    fn shutdown_for_provider_host_set_owner_v1(&mut self) -> Result<(), CapabilitySdkError> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:233:        self.shutdown().map_err(external_provider_fault)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:239:/// `libbun` invokes one module export with one structural input. Swarm provider
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:244:fn libbun_provider_adapter_source_for_selected_route(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:333:        "retained libbun external provider failed: {}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:344:        let source = libbun_provider_adapter_source_for_selected_route(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:61:            libbun_external_capability_provider_enabled,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:62:        } => run_providers_operation(libbun_external_capability_provider_enabled),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:143:    ss_runtime_external_capability_provider_owner::install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:247:fn run_providers_operation(libbun_external_capability_provider_enabled: bool) -> SsResult<Value> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:248:    ss_runtime_provider_listing_owner::providers_with_libbun(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:249:        libbun_external_capability_provider_enabled,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:705:impl Drop for SubstrateDiagnosticJson {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:851:                .spawn(action)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:858:        let output = Command::new(std::env::current_exe().expect("test executable must resolve"))
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:27:    let output = Command::new(&ss_binary)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:77:    let mut child = Command::new(&ss_binary)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:86:        .spawn()
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:96:        .wait_with_output()
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:114:fn ss_reuses_one_libbun_runtime_for_multiple_capability_imports() {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:116:        "multi-provider-shared-libbun-runtime",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:130:    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:132:    let output = Command::new(&ss_binary)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:143:        "ss should run both package-resolved libbun providers in one invocation\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:159:fn ss_test_pool_child_conserves_package_roots_for_test_and_libbun_providers() {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:168:  try await io.print({ value: "pool child libbun provider" });
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:173:  "pool child conserves package roots for test and libbun providers",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:183:    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:184:    let output = Command::new(&ss_binary)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:428:fn bundle_development_libbun_plugin_next_to_ss_binary(ss_binary: &Path) {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:429:    let asset = libbun::release::current_native_plugin_asset()
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:430:        .expect("libbun plugin asset metadata should exist for this host");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:439:        .join("libbun/plugin/target/release")
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:443:        "release-profile libbun plugin must be built before running this test; expected {}. Build with LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --release --manifest-path ../libbun/plugin/Cargo.toml",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:448:            "failed to bundle development libbun plugin from {} to {}: {error}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:20:    fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:25:    fn shutdown_for_provider_host_set_owner_v1(&mut self) -> CapabilitySdkResult<()>;
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:69:    pub fn libbun_for_ss_external_capability_provider_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:72:        Self::new("libbun", "libbun", factory)
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:91:    pub(crate) fn begin_execution_session_for_provider_host_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:116:    pub(crate) fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:126:            .invoke_manifest_resolved_call_for_provider_host_set_owner_v1(invocation)
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:129:    pub(crate) fn shutdown_for_provider_host_set_owner_v1(&mut self) -> CapabilitySdkResult<()> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:131:            provider.shutdown_for_provider_host_set_owner_v1()?;
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:11:    HostAdmittedTypedProviderRequest, ProviderHostContext, ProviderHostResourceReleaseFaultV1,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:164:/// The `LoadedNativePark` payload is a SEALED [`HostAdmittedTypedProviderRequest`]
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:176:    LoadedNativePark(HostAdmittedTypedProviderRequest),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:182:        request: HostAdmittedTypedProviderRequest,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:199:    ) -> Option<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:272:                .map(ExternalTransportCapabilityProviderHost::begin_execution_session_for_provider_host_owner_v1),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:510:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:524:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:567:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:576:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:634:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:675:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:681:        request: HostAdmittedTypedProviderRequest,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:690:        request: HostAdmittedTypedProviderRequest,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:894:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:993:    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:1000:        request: HostAdmittedTypedProviderRequest,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:1027:                .invoke_manifest_resolved_call_for_provider_host_set_owner_v1(invocation),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:1032:impl Drop for ProviderHostExecutionSession {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:1035:            let _ = session.shutdown_for_provider_host_set_owner_v1();

## Adjacent external fixture graph

Meaning: Binds the current real-binary retained-runtime and external-result fixture graph.

Expected result: Exit 0; every attached external fixture/test definition is visible.

Command: git -C /home/ubuntu/swarm grep -n -E '#\[test\]|libbun|external.provider|counter|multiple.capability|pool.child|cancellation|deadline|shutdown|provider result' 95323ff17cb29928e31467f651ef03bae2099c14 -- crates/ss/tests/external_capability_provider.rs tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss

Pattern: #\[test\]|libbun|external.provider|counter|multiple.capability|pool.child|cancellation|deadline|shutdown|provider result

Pathspecs: crates/ss/tests/external_capability_provider.rs tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss

Exit: 0

Output:

95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:11:#[test]
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:53:#[test]
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:113:#[test]
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:114:fn ss_reuses_one_libbun_runtime_for_multiple_capability_imports() {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:116:        "multi-provider-shared-libbun-runtime",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:117:        r#"import capability { counter } from "@swarm-fixture/counter";
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:119:try await counter.first({});
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:120:try await counter.second({});
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:123:    write_counter_provider_package(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:130:    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:143:        "ss should run both package-resolved libbun providers in one invocation\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:158:#[test]
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:159:fn ss_test_pool_child_conserves_package_roots_for_test_and_libbun_providers() {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:161:        "pool-child-multi-provider-package-roots",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:168:  try await io.print({ value: "pool child libbun provider" });
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:173:  "pool child conserves package roots for test and libbun providers",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:183:    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:200:        .expect("ss test should run through a pool child");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:206:        "pool child must re-admit the parent CLI package roots for both providers\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:277:  "name": "@swarm-fixture/counter-app",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:294:fn write_counter_provider_package(root: &Path) {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:298:  "name": "@swarm-fixture/counter-app",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:305:    "@swarm-fixture/counter": "0.0.1"
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:310:    .expect("counter app package.json should be written");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:311:    let package_root = root.join("node_modules/@swarm-fixture/counter");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:313:    fs::create_dir_all(&src_root).expect("counter provider package source root");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:317:  "name": "@swarm-fixture/counter",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:322:      "swarm": "./src/counter.contract.ts"
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:334:    .expect("counter provider package.json should be written");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:336:        src_root.join("counter.contract.ts"),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:346:export declare const counter: Counter;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:349:    .expect("counter contract should be written");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:353:import type { Counter, CounterOutcome } from "./counter.contract.ts";
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:364:        domain: "swarm.fixture.counter",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:365:        name: "counter_state_not_shared",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:386:    .expect("counter provider module should be written");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:428:fn bundle_development_libbun_plugin_next_to_ss_binary(ss_binary: &Path) {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:429:    let asset = libbun::release::current_native_plugin_asset()
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:430:        .expect("libbun plugin asset metadata should exist for this host");
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:439:        .join("libbun/plugin/target/release")
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:443:        "release-profile libbun plugin must be built before running this test; expected {}. Build with LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --release --manifest-path ../libbun/plugin/Cargo.toml",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:448:            "failed to bundle development libbun plugin from {} to {}: {error}",
95323ff17cb29928e31467f651ef03bae2099c14:tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss:17:  "external provider JSON text is NFC at ingress",
95323ff17cb29928e31467f651ef03bae2099c14:tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss:21:  "imported helper returns an external provider result payload",
