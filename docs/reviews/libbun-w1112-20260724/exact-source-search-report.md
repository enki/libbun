# libbun W1-11/W1-12 exact-source search report (correction 2)

Libbun product SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

Libbun product tree: cb964de8ab8162449fbe95959bf34d231570aa5c

Adjacent swarm SHA: 95323ff17cb29928e31467f651ef03bae2099c14

Adjacent swarm tree: 43b47bbd49a6053d270b3e15cc141cb1b1bb86da

Every section records its literal Git command, pattern, pathspecs, semantic meaning, expected exit, observed exit, and output. Exit 1 is accepted only for the explicitly labeled required-definition absence search.

## Required owner and lifecycle definitions (expected negative)

Meaning: Proves the poisoned candidate has no positive retained-backend/proof implementation.

Expected result: Exit 1 means every named required definition is absent from the complete implementation pathset.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'BunProviderBackend|SelectedProviderPackage|ProviderInvocation|OfferCustody|OfferReadyProof|ReservedCustody|ReservationReleaseProof|DriveCustody|InvocationReadyProof|RetirementProof|DurableReaper|RetirementQuarantine|QuarantineObservation|QuarantineCompletionClaim|RetiredDisposal' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Pattern: BunProviderBackend|SelectedProviderPackage|ProviderInvocation|OfferCustody|OfferReadyProof|ReservedCustody|ReservationReleaseProof|DriveCustody|InvocationReadyProof|RetirementProof|DurableReaper|RetirementQuarantine|QuarantineObservation|QuarantineCompletionClaim|RetiredDisposal

Pathspecs: src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Exit: 1

Output:

<no matches>

## Native/wire public and RAW bridge shapes

Meaning: Finds current public protocol/native entry points and forbidden raw/parts/callback proof shapes.

Expected result: Exit 0 with every current public bridge available for migration/deletion review.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'pub (struct|enum|fn)|DriveRequest|drive_prepared_export|internal-adapter|install_prepared_export|from_parts|into_parts|selector|descriptor|Clone|Serialize|Deserialize|callback|receipt' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Pattern: pub (struct|enum|fn)|DriveRequest|drive_prepared_export|internal-adapter|install_prepared_export|from_parts|into_parts|selector|descriptor|Clone|Serialize|Deserialize|callback|receipt

Pathspecs: src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:14:internal-adapter = []
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:7:#[cfg(not(feature = "internal-adapter"))]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:33:use libbun_prepared_export_wire::DriveRequest;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:35:use serde::Deserialize;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:127:#[derive(Deserialize)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:139:#[derive(Deserialize)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:217:pub struct NativeDriveFailure {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:223:    pub fn kind(&self) -> WorkerFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:227:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:244:pub fn drive_prepared_export(request: DriveRequest) -> Result<Vec<u8>, NativeDriveFailure> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:14:libbun-native = { path = "../native", features = ["internal-adapter"] }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:25:    match libbun_native::drive_prepared_export(request) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:10:cargo +nightly-2026-05-06 check --manifest-path "$repo_root/native/Cargo.toml" --features internal-adapter
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/lib.rs:4://! module handle, promise handle, event-loop control, callback, or path-fed
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:32:pub struct PreparedExport {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:50:    pub fn drive(self, control: DriveControl) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:148:#[derive(Clone)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:149:pub struct DriveCancellation {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:154:pub struct DriveControl {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:160:    pub fn unbounded() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:169:    pub fn with_deadline_after(duration: Duration) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:178:    pub fn cancellable() -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:191:    pub fn cancellable_with_deadline_after(duration: Duration) -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:206:    pub fn cancel(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:224:pub enum MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:233:pub struct Cargo {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:239:    pub fn into_bytes(self) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:245:pub struct Cancelled {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:250:pub struct DeadlineElapsed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:255:pub struct MechanicalFault {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:262:    pub fn kind(&self) -> MechanicalFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:266:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:280:#[derive(Debug, Clone, Copy, PartialEq, Eq)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:282:pub enum MechanicalFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:922:    use serde::Serialize;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:923:    use serde::de::DeserializeOwned;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:926:    assert_not_impl_any!(PreparedExport: Clone, Copy, Serialize, DeserializeOwned);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:927:    assert_not_impl_any!(Cargo: Clone, Copy, Serialize, DeserializeOwned);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:928:    assert_not_impl_any!(Cancelled: Clone, Copy, Serialize, DeserializeOwned);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:929:    assert_not_impl_any!(DeadlineElapsed: Clone, Copy, Serialize, DeserializeOwned);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:930:    assert_not_impl_any!(MechanicalFault: Clone, Copy, Serialize, DeserializeOwned);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1123:    fn inherited_protocol_descriptor_descendant_is_retired_before_cargo_returns() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs:2:    let _ = libbun::install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:1:use libbun::install_prepared_export;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:4:    let _ = install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:49:            stderr.contains("install_prepared_export") && stderr.contains(intended_diagnostic),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:12:pub struct DriveRequest {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:18:#[derive(Debug, Clone, Copy, PartialEq, Eq)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:20:pub enum WorkerFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:28:pub fn encode_drive_material(request: DriveRequest) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:44:pub fn write_drive_request(writer: &mut impl Write, material: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:58:pub fn read_drive_request(reader: &mut impl Read) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:85:pub fn write_cargo(writer: &mut impl Write, cargo: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:89:pub fn write_fault(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:112:fn decode_drive_material(material: &[u8]) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:120:    Ok(DriveRequest {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:159:        let material = encode_drive_material(DriveRequest {

## Process containment, raw handle, and join topology

Meaning: Finds all worker process, containment, raw descriptor/handle, channel, and join custody.

Expected result: Exit 0; every match is current candidate topology, including rejected process-group fallback.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'Command::new|Child|try_wait|\.wait\(|\.kill\(|setpgid|SIGKILL|CreateJobObject|AssignProcessToJobObject|namespace|sandbox|RawFd|RawHandle|JoinHandle|Receiver|sync_channel|\.join\(' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Pattern: Command::new|Child|try_wait|\.wait\(|\.kill\(|setpgid|SIGKILL|CreateJobObject|AssignProcessToJobObject|namespace|sandbox|RawFd|RawHandle|JoinHandle|Receiver|sync_channel|\.join\(

Pathspecs: src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:65:    let output = Command::new("clang")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:86:                repo_root.join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:89:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:90:        .join("libbun_native_link_manifest.txt")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:110:                .join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:479:            Unwrapped::Fulfilled(namespace) => Ok(namespace),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:523:        let bundle_dir = tempdir.path().join(format!("{module_id}.bundle"));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:529:            let path = bundle_dir.join(module_path);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:544:        let entry_module = bundle_dir.join(bundle.entry_module);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:587:    use std::os::fd::AsRawFd;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:594:    use std::os::fd::FromRawFd;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:644:    use std::os::windows::io::AsRawHandle;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:783:        let namespace = self.import_module_specifier(&specifier)?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:785:        self.vm().run_with_api_lock(|| namespace.protect());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:786:        self.modules.insert(id.clone(), namespace);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:797:        let namespace = *self
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:801:        let function = namespace
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:813:            match function.call(self.vm().global(), namespace, &[arg]) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:897:            .join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:86:    let output = Command::new("clang")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:109:                repo_root.join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:112:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:113:        .join("libbun_native_link_manifest.txt")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:6:use std::process::Child;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:7:use std::process::ChildStderr;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:16:use std::thread::JoinHandle;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:86:        let mut command = Command::new(worker_program);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:358:    child: Option<Child>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:360:    writer: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:361:    reader: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:362:    stderr: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:363:    writer_result: Option<mpsc::Receiver<Result<(), FaultSeed>>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:364:    reader_result: Option<mpsc::Receiver<Result<Vec<u8>, FaultSeed>>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:365:    stderr_result: Option<mpsc::Receiver<Result<(), FaultSeed>>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:370:    fn admit(child: Child, request: Vec<u8>) -> Result<Self, MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:375:                let _ = child.kill();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:376:                let _ = child.wait();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:417:        let (writer_tx, writer_result) = mpsc::sync_channel(1);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:446:        let (reader_tx, reader_result) = mpsc::sync_channel(1);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:470:        let (stderr_tx, stderr_result) = mpsc::sync_channel(1);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:530:                    .try_wait()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:616:            && let Err(error) = child.wait()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:631:            if handle.is_some_and(|handle| handle.join().is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:659:fn receive_once<T>(receiver: &mpsc::Receiver<T>, slot: &mut Option<T>) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:745:fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:807:    fn for_child(child: &Child) -> io::Result<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:817:            use std::os::windows::io::AsRawHandle;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:818:            use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:819:            use windows_sys::Win32::System::JobObjects::CreateJobObjectW;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:825:            let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:841:                && unsafe { AssignProcessToJobObject(job, child.as_raw_handle() as _) } != 0;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:859:            let result = unsafe { libc::kill(-self.process_group, libc::SIGKILL) };
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:908:            if libc::setpgid(0, 0) == -1 {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1091:        cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1217:            cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:7:    let manifest = repository.join("tests/fixtures/public_api_boundary/Cargo.toml");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:10:        .unwrap_or_else(|| repository.join("target"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:11:        .join("external-public-api-boundary");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:13:    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))

## Output drain, overflow, barrier, EOF, and diagnostic topology

Meaning: Finds every current output path and the absence/presence of persistent bounded pumps and barriers.

Expected result: Exit 0; matches enumerate candidate output custody and tests.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'stdout|stderr|diagnostic|log|OutputCapture|drain|flush|overflow|barrier|EOF|read_single_candidate|write_request|pipe' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Pattern: stdout|stderr|diagnostic|log|OutputCapture|drain|flush|overflow|barrier|EOF|read_single_candidate|write_request|pipe

Pathspecs: src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:72:    let path = String::from_utf8(output.stdout).ok()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:219:    diagnostic: String,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:227:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:228:        &self.diagnostic
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:231:    fn new(kind: WorkerFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:232:        let mut diagnostic = diagnostic.into();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:233:        if diagnostic.len() > 4096 {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:235:            while !diagnostic.is_char_boundary(boundary) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:238:            diagnostic.truncate(boundary);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:240:        Self { kind, diagnostic }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:312:    stdout: OutputCapture,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:313:    stderr: OutputCapture,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:314:    log: OutputCapture,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:322:struct OutputCapture {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:395:                return bounded_js_diagnostic_text(stack);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:419:                    return bounded_js_diagnostic_text(format!("{name}: {message}"));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:422:                    return bounded_js_diagnostic_text(message);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:425:                    return bounded_js_diagnostic_text(name);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:430:                .map(bounded_js_diagnostic_text)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:435:            .map(bounded_js_diagnostic_text)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:500:    fn drain_output(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:501:        bun_core::Output::flush();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:502:        self.stdout.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:503:        self.stderr.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:504:        self.log.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:552:impl OutputCapture {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:554:        let (read_file, write_file) = create_nonblocking_pipe_pair()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:565:    fn drain(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:575:                        "output pipe read failed: {err}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:593:fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:597:    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:599:            "output pipe create failed: {}",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:622:            "output pipe flags read failed: {}",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:628:            "output pipe nonblocking setup failed: {}",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:636:fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:673:fn bounded_js_diagnostic_text(text: impl Into<String>) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:685:    text.push_str("\n[libbun truncated JavaScript diagnostic after 16384 bytes]");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:731:        let stdout = OutputCapture::create()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:732:        let stderr = OutputCapture::create()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:733:        let log = OutputCapture::create()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:734:        bun_core::Output::Source::set_init(stdout.bun_file(), stderr.bun_file());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:738:                .write(bun_core::Output::output_sink().quiet_writer_from_fd(log.bun_file().0));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:765:            stdout,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:766:            stderr,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:767:            log,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:778:        bun_core::scoped_log!(LibbunNative, "loading module {}", id);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:787:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:823:                self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:833:            self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:838:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:847:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:878:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:891:    // which makes canonicalize report a mimalloc invalid-pointer diagnostic.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:93:    let path = String::from_utf8(output.stdout).ok()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:18:                &mut io::stdout().lock(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:27:            libbun_prepared_export_wire::write_cargo(&mut io::stdout().lock(), &cargo)?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:31:                &mut io::stdout().lock(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:33:                failure.diagnostic(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/configure-vendored-bun.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/stage-vendored-bun-source.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/update-vendored-bun.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/vendor-bun-deps.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun-reproducible.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun.sh:2:set -euo pipefail
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:89:            .stdin(Stdio::piped())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:90:            .stdout(Stdio::piped())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:91:            .stderr(Stdio::piped());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:257:    diagnostic: Box<str>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:266:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:267:        &self.diagnostic
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:270:    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:271:        let diagnostic = bounded_diagnostic(diagnostic.into()).into_boxed_str();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:274:            diagnostic,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:341:    diagnostic: String,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:345:    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:348:            diagnostic: diagnostic.into(),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:353:        MechanicalFault::new(self.kind, self.diagnostic)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:362:    stderr: Option<JoinHandle<()>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:365:    stderr_result: Option<mpsc::Receiver<Result<(), FaultSeed>>>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:388:            stderr: None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:391:            stderr_result: None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:401:                "fresh worker did not provide its private request pipe",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:404:        let mut stdout = child.stdout.take().ok_or_else(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:407:                "fresh worker did not provide its private terminal pipe",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:410:        let mut stderr = child.stderr.take().ok_or_else(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:413:                "fresh worker did not provide its bounded diagnostic pipe",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:422:                    write_request(&mut stdin, &request).map_err(|error| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:451:                    read_single_candidate(&mut stdout)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:470:        let (stderr_tx, stderr_result) = mpsc::sync_channel(1);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:471:        let stderr_thread = thread::Builder::new()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:472:            .name("libbun-prepared-export-stderr".to_string())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:475:                    drain_bounded_stderr(&mut stderr)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:480:                        "private worker diagnostic thread unwound",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:483:                let _ = stderr_tx.send(result);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:488:                    format!("private worker diagnostic thread spawn failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:491:        guard.stderr = Some(stderr_thread);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:492:        guard.stderr_result = Some(stderr_result);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:500:        let mut stderr = None;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:503:        let mut pipe_fault_observed_at = None;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:519:                self.stderr_result
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:521:                    .expect("admitted drive owns diagnostic result"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:522:                &mut stderr,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:550:                || stderr.as_ref().is_some_and(Result::is_err)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:552:                let first_observation = pipe_fault_observed_at.get_or_insert_with(Instant::now);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:553:                // A child that exits nonzero commonly closes its pipes just
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:563:                    if let Some(Err(fault)) = stderr.take_if(|result| result.is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:589:                && stderr.as_ref().is_some_and(Result::is_ok)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:629:            ("diagnostic", self.stderr.take()),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:667:fn write_request(writer: &mut impl Write, request: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:671:fn read_single_candidate(reader: &mut impl Read) -> Result<Vec<u8>, FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:717:            format!("worker terminal pipe EOF observation failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:722:fn worker_candidate_fault(kind: u8, diagnostic: Vec<u8>) -> FaultSeed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:742:    FaultSeed::new(kind, String::from_utf8_lossy(&diagnostic).into_owned())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:745:fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:749:        let read = stderr.read(&mut buffer).map_err(|error| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:752:                format!("worker diagnostic pipe read failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:762:                "worker diagnostic output exceeds bounded limit",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:785:fn bounded_diagnostic(mut diagnostic: String) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:786:    if diagnostic.len() <= MAX_DIAGNOSTIC_BYTES {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:787:        return diagnostic;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:790:    while !diagnostic.is_char_boundary(boundary) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:793:    diagnostic.truncate(boundary);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:794:    diagnostic
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:965:    sys.stdout.buffer.write(frame(payload))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:967:    sys.stdout.buffer.write(frame(payload))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:968:    sys.stdout.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:971:    sys.stdout.buffer.write(b'wrong-frame')
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:973:    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x00' + (99).to_bytes(4, 'big') + b'x')
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:975:    sys.stdout.buffer.write(frame(payload) + frame(payload))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:977:    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x00' + (16 * 1024 * 1024 + 1).to_bytes(4, 'big'))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:984:    sys.stdout.buffer.write(frame(payload))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:986:    sys.stdout.buffer.write(frame(str(os.getpid()).encode()))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:988:    sys.stdout.buffer.write(b'LBPT' + (99).to_bytes(2, 'big') + b'\x00' + (0).to_bytes(4, 'big'))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:990:    sys.stdout.buffer.write(b'LBPT' + (1).to_bytes(2, 'big') + b'\x03' + len(payload).to_bytes(4, 'big') + payload)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:991:elif mode == 'large-stderr':
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:992:    sys.stderr.buffer.write(b'x' * (2 * 1024 * 1024))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:993:    sys.stderr.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:994:    sys.stdout.buffer.write(frame(payload))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:999:sys.stdout.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1014:                WorkerBehavior::LargeStderr(payload) => ("large-stderr", payload),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1174:    fn large_worker_stderr_is_drained_without_blocking_cargo() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1175:        let bytes = b"after diagnostic flood".to_vec();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1180:            panic!("bounded diagnostic drain should not block cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:35:        String::from_utf8_lossy(&control.stderr)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:38:    for (bin, intended_diagnostic) in [
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:43:        let stderr = String::from_utf8_lossy(&output.stderr);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:49:            stderr.contains("install_prepared_export") && stderr.contains(intended_diagnostic),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:50:            "{bin} failed for an unintended reason:\n{stderr}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:55:    writer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:92:    diagnostic: &str,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:94:    write_candidate(writer, kind as u8, diagnostic.as_bytes())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:109:    writer.flush()

## Lifecycle, refusal, retry, cancellation, unwind, Drop, and shutdown topology

Meaning: Finds current lifecycle transitions, fault paths, destructors, aborts, and missing proof algebra.

Expected result: Exit 0; all current lifecycle and destructor sites are named.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'release|reservation|ready|retire|reaper|quarantine|cancel|deadline|catch_unwind|panic|impl Drop|process::abort|shutdown|restart|fault' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Pattern: release|reservation|ready|retire|reaper|quarantine|cancel|deadline|catch_unwind|panic|impl Drop|process::abort|shutdown|restart|fault

Pathspecs: src native/src native/build.rs wire/src runtime/src runtime/build.rs scripts .github tests Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:18:        .unwrap_or_else(default_manifest_path);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:20:        panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:27:    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:77:fn default_manifest_path() -> PathBuf {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:89:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:113:            panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:123:        panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:139:        panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:140:            "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:330:        // `shutdown`, which consumes all public operations through the facade.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:722:                "another native Bun runtime is already active in this process",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:747:            ..Default::default()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:950:pub extern "C" fn Bun__panic(msg: *const u8, len: usize) -> ! {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:956:    bun_core::Output::panic(format_args!("{}", String::from_utf8_lossy(bytes)));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:21:        .unwrap_or_else(default_manifest_path);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:23:        panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:29:    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:98:fn default_manifest_path() -> PathBuf {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:112:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:124:        panic!(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:125:            "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:17:            return libbun_prepared_export_wire::write_fault(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:30:            libbun_prepared_export_wire::write_fault(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:8:call_frame_patch="$repo_root/patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:23:    echo "Vendored Bun patch already applied: $(basename "$patch_file")"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/configure-vendored-bun.sh:7:build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$bun_dir/build/release"}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:5:  echo "usage: $0 <version> <release-worker-binary> <output-directory>" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:33:if [[ "$worker" != */release/libbun-runtime-native && "$worker" != */release/libbun-runtime-native.exe ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:34:  echo "release bundles require the release-profile libbun-runtime-native binary: $worker" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:43:    echo "unsupported prepared-export worker release target: $(uname -s)-$(uname -m)" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:6:profile="${LIBBUN_NATIVE_BUN_PROFILE:-release}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:9:if [[ "$profile" != "release" ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:10:  echo "libbun native worker links must be prepared from Bun's release profile; got LIBBUN_NATIVE_BUN_PROFILE=$profile" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:32:  echo "libbun native worker links must use Bun's release bun-profile target; got LIBBUN_NATIVE_BUN_EXE_TARGET=$exe_target" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/vendor-bun-deps.sh:18:  echo "Vendored lolhtml already present at $lolhtml_commit"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/lib.rs:6://! a fresh worker through terminal retirement.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:36:    panic_after_admission: bool,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:49:    /// export. Every return path has retired the fresh worker boundary.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:51:        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.drive_guarded(control)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:63:        let panic_after_admission = self.panic_after_admission;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:64:        if control.cancellation.is_selected() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:67:        if control.deadline_is_elapsed() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:92:        configure_retirement_boundary(&mut command);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:106:            Err(fault) => return MechanicalTerminal::MechanicalFault(fault),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:109:        if panic_after_admission {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:110:            panic!("injected prepared-export supervisor unwind after worker admission");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:113:        match guard.retire() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:115:            Err(fault) => MechanicalTerminal::MechanicalFault(fault),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:131:            panic_after_admission: false,
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
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:334:            Self::MechanicalFault(fault) => MechanicalTerminal::MechanicalFault(fault.into_fault()),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:352:    fn into_fault(self) -> MechanicalFault {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:366:    retired: bool,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:379:                    format!("fresh worker retirement-boundary admission failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:392:            retired: false,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:421:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:450:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:474:                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:503:        let mut pipe_fault_observed_at = None;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:545:                return SelectedTerminal::MechanicalFault(worker_exit_fault(status));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:552:                let first_observation = pipe_fault_observed_at.get_or_insert_with(Instant::now);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:557:                    if let Some(Err(fault)) = writer.take_if(|result| result.is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:558:                        return SelectedTerminal::MechanicalFault(fault);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:560:                    if let Some(Err(fault)) = reader.take_if(|result| result.is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:561:                        return SelectedTerminal::MechanicalFault(fault);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:563:                    if let Some(Err(fault)) = stderr.take_if(|result| result.is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:564:                        return SelectedTerminal::MechanicalFault(fault);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:569:            if control.cancellation.is_selected() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:572:            if control.deadline_is_elapsed() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:580:                        format!("fresh worker descendant retirement failed: {error}"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:601:    fn retire(&mut self) -> Result<(), MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:602:        if self.retired {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:606:        let mut cleanup_fault = None;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:608:            cleanup_fault.get_or_insert_with(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:618:            cleanup_fault.get_or_insert_with(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:632:                cleanup_fault.get_or_insert_with(|| {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:635:                        format!("worker {name} thread panicked during retirement"),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:641:        self.retired = cleanup_fault.is_none();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:642:        cleanup_fault.map_or(Ok(()), Err)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:646:impl Drop for DriveGuard {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:648:        if self.retired {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:651:        if self.retire().is_err() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:653:            // retirement would violate the mechanical boundary.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:654:            std::process::abort();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:710:        Ok(0) => Err(worker_candidate_fault(candidate_kind, bytes)),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:722:fn worker_candidate_fault(kind: u8, diagnostic: Vec<u8>) -> FaultSeed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:768:fn worker_exit_fault(status: ExitStatus) -> FaultSeed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:894:impl Drop for ProcessBoundary {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:904:fn configure_retirement_boundary(command: &mut Command) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:917:fn configure_retirement_boundary(_command: &mut Command) {}
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1046:            panic!("expected cargo terminal");
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
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1095:    fn malformed_truncated_duplicate_and_oversized_frames_are_typed_faults() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1102:            let MechanicalTerminal::MechanicalFault(fault) =
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1103:                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1105:                panic!("invalid worker frame must be a typed fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1107:            assert_eq!(fault.kind(), MechanicalFaultKind::WorkerProtocol);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1112:    fn nonzero_worker_exit_is_a_typed_fault_after_reap() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1114:        let MechanicalTerminal::MechanicalFault(fault) =
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1115:            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1117:            panic!("nonzero worker exit must be a typed fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1119:        assert_eq!(fault.kind(), MechanicalFaultKind::WorkerTermination);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1123:    fn inherited_protocol_descriptor_descendant_is_retired_before_cargo_returns() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1124:        let bytes = b"retired descendant".to_vec();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1130:            prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1132:            panic!("successful leader plus retired descendant should return cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1141:                prepared.drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1143:                panic!("worker pid fixture must return cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1155:    fn wrong_version_and_worker_rejection_remain_distinct_typed_faults() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1158:                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1160:            panic!("wrong wire version must fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1166:                .drive(DriveControl::with_deadline_after(Duration::from_secs(2)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1168:            panic!("worker rejection must fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1178:                .drive(DriveControl::with_deadline_after(Duration::from_secs(3)))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1180:            panic!("bounded diagnostic drain should not block cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1188:            .drive(DriveControl::with_deadline_after(Duration::from_secs(5)));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1189:        let MechanicalTerminal::MechanicalFault(fault) = terminal else {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1190:            panic!("aborted worker must fault, got {terminal:?}");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1192:        assert_eq!(fault.kind(), MechanicalFaultKind::WorkerTermination);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1196:    fn supervisor_unwind_after_admission_retires_before_fault_return() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1198:        prepared.panic_after_admission = true;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1200:        let MechanicalTerminal::MechanicalFault(fault) = terminal else {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1201:            panic!("supervisor unwind must become typed fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1203:        assert_eq!(fault.kind(), MechanicalFaultKind::SupervisorUnwind);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1207:    fn cancellation_deadline_race_selects_exactly_one_post_retirement_terminal() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1210:            let (control, cancellation) =
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1211:                DriveControl::cancellable_with_deadline_after(Duration::from_millis(30));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1212:            let cancelling = thread::spawn(move || {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1214:                cancellation.cancel();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1217:            cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1226:    fn oversized_request_faults_before_worker_spawn() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1232:        let MechanicalTerminal::MechanicalFault(fault) = prepared.drive(DriveControl::unbounded())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1234:            panic!("oversized request must fault");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1236:        assert_eq!(fault.kind(), MechanicalFaultKind::RequestWrite);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:89:pub fn write_fault(

## Package, lock, license, compliance, release, and extracted-smoke topology

Meaning: Finds all current packaging/release modes and compliance inputs without scanning lock payload noise.

Expected result: Exit 0; matches expose current fresh-process/fallback and missing immutable release workflow.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E 'package|archive|release|linked|unlinked|fallback|fresh-process|Cargo.lock|license|notice|compliance|workflow|tag|symbol|extract|smoke' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml scripts .github README.md docs vendor/README.md vendor/bun.LIBBUN_VENDOR.json

Pattern: package|archive|release|linked|unlinked|fallback|fresh-process|Cargo.lock|license|notice|compliance|workflow|tag|symbol|extract|smoke

Pathspecs: Cargo.toml native/Cargo.toml runtime/Cargo.toml wire/Cargo.toml scripts .github README.md docs vendor/README.md vendor/bun.LIBBUN_VENDOR.json

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:6:license = "Apache-2.0"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:14:    "/Cargo.lock",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:10:release-eligible. Its source remains a fresh-worker, raw-constructor
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:18:re-export. No positive lifecycle or release work is eligible before that poison
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:25:- [worker build, package, and release](docs/LIBBUN-WORKER-RELEASE-CONTRACT.md).
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:30:generatively branded selected-package and invocation products. Private
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:37:dispatch, with no selected package or invocation transmitted, and permits
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:59:The worker is a linked binary-only product. There is no public raw constructor,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:62:fallback, unsafe `Send`/`Sync`, process-group containment fallback,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:README.md:63:blocking/aborting Drop, or unlinked release mode.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:11:release proof are specified separately but are part of the same completion
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:35:- Too high: a generic embedding runtime, workflow/session framework, public
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:40:The source/package owner mints both products after it has selected and checked
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:41:the package, export, and invocation. Their brand proves generative
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:87:Admission consumes the backend, package, and invocation:
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:93:        package: SelectedProviderPackage<Brand>,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:103:offer contains no selected package, export, invocation, or dispatch authority.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:110:  preallocated queue node. Reservation transmits no selected package or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:156:  and is now closed and unreplayable before dispatch; no selected package or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:160:  release.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:237:2. Pre-dispatch release requires an exact release acknowledgement and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:275:- package/export correspondence and preparation;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:278:- undefined or unserializable result/cargo extraction;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:296:Pre-dispatch release is settled only by `ReservationReleaseProof`.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:308:incomplete cancellation, or incomplete release forces retirement or adoption.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:355:- construct or clone package, invocation, backend, prepared export, proof, or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-LIFECYCLE-CONTRACT.md:357:- obtain raw package bytes, export names, worker paths, ids, session epochs,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md:18:  call fixtures must fail because `libbun` no longer exports the symbol.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md:22:No replacement behavior, alias, fixed error, fallback, empty/default cargo,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:61:Reservation release may produce only `ReservationReleaseProof`,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:72:The worker receives no selected package or invocation until all of these are
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:87:allocates the exact worker/session slot but transmits no selected package or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:96:Process groups are not containment and are forbidden as a fallback.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:135:Reservation release, dispatched-invocation finalization, and worker retirement
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:139:Reservation release may produce `ReservationReleaseProof` only after all of:
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:143:- no selected-package or invocation byte was enqueued or transmitted;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:147:- no release or reservation-teardown work remains; and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:304:rejection, release, cancellation, deadline, Ready, or shutdown-success
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:307:Pre-dispatch release becomes public only after `ReservationReleaseProof`.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md:348:- pre-dispatch release followed by `ReservationReleaseProof`, same-worker
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:9:The only executable Bun authority is `libbun-runtime-native`, a linked,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:11:loader, helper fallback, in-process feature, or callable Rust drive entry point.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:13:The root facade contains supervisor/model code only. The runtime package has a
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:25:- plugin/dynamic-loading packages, features, checksums, paths, installers,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:26:  caches, aliases, and workflows; and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:35:release-profile Bun link manifest and every referenced archive/static input
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:36:exists. Warning-only unlinked mode is forbidden.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:38:The acceptance build is an actual linked binary:
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:43:  --release \
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:47:`cargo check` is useful but cannot satisfy the worker gate. The linked binary
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:51:The repository uses one current lockfile unless a reviewed release reason
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:53:facade must package without an unpublished path library dependency.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:57:Each target package contains:
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:59:- the exact release-profile `libbun-runtime-native` executable;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:65:- notices, license texts, and dependency inventory; and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:66:- package checksums.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:68:The manifest contains no fallback key, plugin ABI, shared-library runtime mode,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:72:The package test extracts into a new directory, places the worker at the
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:73:documented sibling location, and runs the complete smoke protocol against the
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:74:extracted binary. Testing the build-tree executable is not package proof.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:83:4. locked release Bun link preparation;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:84:5. actual linked worker build;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:85:6. nonzero linked runtime/native engine tests;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:87:   `ReservationReleaseProof` pre-dispatch release, and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:97:11. worker package creation;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:98:12. extracted-package execution;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:99:13. source/notice/license/compliance verification;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:100:14. facade `cargo package` and repeat lock generation with no diff;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:101:15. symbol, dependency, and stale-shape searches; and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:104:A linked test target reporting zero tests fails the gate. CI cannot skip real
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:109:The worker-only release workflow builds from an immutable tag, repeats every CI
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:110:gate, packages each supported target, verifies extracted packages, publishes
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:111:binary and compliance assets, and verifies the release inventory after upload.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:113:No old plugin workflow is restored. No platform enters the release matrix until
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:114:its exact containment and hostile tests pass on the release runner.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:135:fallback
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:161:Binary symbol scans must show no Rust drive entry point, plugin ABI, shared
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:170:- every pre-dispatch release terminal is post-`ReservationReleaseProof`;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:202:- the runtime binary is actually linked and executed;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:203:- the extracted package is executed;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:204:- locks and package metadata are stable;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:205:- compliance assets match the binary inputs;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:210:installer, uses fresh-worker semantics, checks an unlinked worker, lacks the
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/LIBBUN-WORKER-RELEASE-CONTRACT.md:211:retained hostile gates above, and is not release-ready.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:12:and must not be released or treated as a compatibility baseline. The earlier
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:39:3. [Worker build, package, and release](LIBBUN-WORKER-RELEASE-CONTRACT.md)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:40:   fixes the Rust privacy boundary, linked binary, package inventory, CI,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:41:   release, hostile tests, and stale-shape gates.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:44:dynamic-loading, callback, path-fed, raw-runtime, and fallback decisions remain
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:64:the exact reservation is closed and unreplayable, no selected package or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:118:- permits an unlinked, non-runnable worker check to stand in for a worker build;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:120:- lacks a tested retained worker-only release path.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:141:  fallback, `cancel_before_spawn`, cargo from `RetirementProof`,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:148:default cargo, fallback, or compatibility route. Compiler fallout identifies
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:169:6. Define the pre-dispatch release terminal. Successful release uses only
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:170:   `ReservationReleaseProof`; failed or ambiguous release forces retirement or
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:191:    group fallback.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:196:13. Implement retained offer/reserve/release/dispatch/cancel/ready/shutdown,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:201:    operations; build the linked worker package and run hostile, privacy,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:202:    real-worker, extracted-package, compliance, containment, proof-boundary,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:205:    lock/package generation, current-tree evidence capture, and independent
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:docs/README.md:230:fault dominance, containment primitive, or release boundary requires a new
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:5:license = "Apache-2.0"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:5:license = "Apache-2.0"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:8:call_frame_patch="$repo_root/patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/configure-vendored-bun.sh:7:build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$bun_dir/build/release"}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:5:  echo "usage: $0 <version> <release-worker-binary> <output-directory>" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:33:if [[ "$worker" != */release/libbun-runtime-native && "$worker" != */release/libbun-runtime-native.exe ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:34:  echo "release bundles require the release-profile libbun-runtime-native binary: $worker" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:43:    echo "unsupported prepared-export worker release target: $(uname -s)-$(uname -m)" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:48:stage="$(mktemp -d)"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:49:trap 'rm -rf -- "$stage"' EXIT
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:50:mkdir -p "$output" "$stage/bin"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:51:cp "$worker" "$stage/bin/$(basename "$worker")"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:53:python3 - "$stage/manifest.json" "$version" "$target" "$worker_sha" <<'PY'
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:69:    "execution": "fresh-process-only",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:70:    "fallback": None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:74:archive="$output/libbun-prepared-export-worker-${version}-${target}.tar.zst"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:75:tar -C "$stage" -cf - bin manifest.json | zstd -q -19 -T0 -o "$archive"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:76:printf '%s  %s\n' "$(sha256 "$archive")" "$(basename "$archive")" > "$archive.sha256"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:77:echo "created $archive"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:6:profile="${LIBBUN_NATIVE_BUN_PROFILE:-release}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:9:if [[ "$profile" != "release" ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:10:  echo "libbun native worker links must be prepared from Bun's release profile; got LIBBUN_NATIVE_BUN_PROFILE=$profile" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:17:archive="$build_dir/libbun_native_objects.a"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:32:  echo "libbun native worker links must use Bun's release bun-profile target; got LIBBUN_NATIVE_BUN_EXE_TARGET=$exe_target" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:71:    (cd "$build_dir" && xcrun libtool -static -o "$archive" $(cat "$objects_file")) >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:74:    (cd "$build_dir" && rm -f "$archive" && ar crs "$archive" $(cat "$objects_file")) >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:83:  printf 'archive=%s\n' "$archive"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/stage-vendored-bun-source.sh:44:echo "staged reproducible vendored Bun source files from $commit"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/update-vendored-bun.sh:25:git -C "$tmp_dir/bun" archive --format=tar "$commit" | tar -x -C "$vendor_dir"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/update-vendored-bun.sh:51:echo "Run scripts/stage-vendored-bun-source.sh before committing the vendored update."
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/vendor-bun-deps.sh:33:git -C "$tmp_dir/lolhtml" archive --format=tar "$lolhtml_commit" | tar -x -C "$lolhtml_dir"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun-reproducible.sh:43:git -C "$tmp_dir/bun-src" archive --format=tar "$commit" | tar -x -C "$generated"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun.sh:21:for required in Cargo.toml package.json src/bun_bin/Cargo.toml src/jsc/Cargo.toml src/runtime/Cargo.toml vendor/lolhtml/c-api/Cargo.toml; do
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun.sh:29:  echo "vendor/bun must be an archive snapshot, not a nested git checkout" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:vendor/README.md:4:with `git archive`, so it intentionally excludes upstream `.git` metadata and
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/Cargo.toml:1:[package]

## Current test and external privacy fixture definitions

Meaning: Enumerates every current unit/integration/privacy fixture definition to retain, delete, or migrate.

Expected result: Exit 0; all candidate test families and raw-installer tripwires are visible.

Command: git -C /home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-bundle-correction2-20260724 grep -n -E '#\[test\]|#\[cfg\(test\)\]|fn [A-Za-z0-9_]+\(|install_prepared_export|compile-fail|public.API|fixture' 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb -- src native/src wire/src runtime/src tests

Pattern: #\[test\]|#\[cfg\(test\)\]|fn [A-Za-z0-9_]+\(|install_prepared_export|compile-fail|public.API|fixture

Pathspecs: src native/src wire/src runtime/src tests

Exit: 0

Output:

6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:52:    fn initialize(message: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:58:    fn module_load(message: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:64:    fn export_call(message: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:76:    fn one_shot() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:98:    fn null() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:145:    fn from_bytes(bytes: &[u8]) -> LibbunResult<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:153:    fn validate_for_current_runtime(&self, expected_bundle_id: &str) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:171:    fn validate(&self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:199:fn validate_bundle_module_path(path: &str) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:223:    pub fn kind(&self) -> WorkerFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:227:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:231:    fn new(kind: WorkerFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:244:pub fn drive_prepared_export(request: DriveRequest) -> Result<Vec<u8>, NativeDriveFailure> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:328:    fn vm(&self) -> &VirtualMachine {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:334:    fn vm_mut(&mut self) -> &mut VirtualMachine {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:340:    fn evaluate_json(&self, value: &StructuralValue) -> LibbunResult<JSValue> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:354:    fn value_to_result(&self, value: JSValue) -> LibbunResult<ProviderCallResult> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:378:    fn rejected_to_result(&self, value: JSValue) -> ProviderCallResult {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:386:    fn js_error_to_string(&self, value: JSValue, fallback: &str) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:439:    fn import_module_specifier(&mut self, specifier: &str) -> LibbunResult<JSValue> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:456:    fn resolve_module_promise(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:483:    fn promise_result(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:500:    fn drain_output(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:508:    fn materialize_prepared_bundle(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:553:    fn create() -> LibbunResult<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:561:    fn bun_file(&self) -> bun_core::Output::File {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:565:    fn drain(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:586:fn fd_from_file(file: &std::fs::File) -> bun_core::Fd {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:593:fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:618:fn set_nonblocking(fd: libc::c_int) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:636:fn create_nonblocking_pipe_pair() -> LibbunResult<(std::fs::File, std::fs::File)> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:643:fn fd_from_file(file: &std::fs::File) -> bun_core::Fd {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:649:fn js_value_to_string(global: &JSGlobalObject, value: JSValue) -> Option<String> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:663:fn js_value_to_string_lossy(global: &JSGlobalObject, value: JSValue) -> Option<String> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:673:fn bounded_js_diagnostic_text(text: impl Into<String>) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:689:fn apply_environment_overlay(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:709:fn validate_environment_key(key: &str) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:719:    fn initialize(config: BunRuntimeConfig) -> LibbunResult<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:775:    fn load_module(&mut self, spec: BunModuleSpec) -> LibbunResult<BunModuleHandle> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:791:    fn call_export(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:842:    fn pump_event_loop(&mut self, budget: PumpBudget) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:851:    fn resolve_async(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:883:fn native_runtime_guard() -> &'static Mutex<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:888:fn path_to_file_specifier(path: &Path) -> LibbunResult<String> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:910:fn ensure_macos_compat_symbols() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:917:fn ensure_macos_compat_symbols() {}
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:921:pub extern "C" fn libbun_libcxx_hash_memory_compat(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:950:pub extern "C" fn Bun__panic(msg: *const u8, len: usize) -> ! {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:960:pub extern "C" fn Bun__VM__scriptExecutionStatus(vm: *const VirtualMachine) -> i32 {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:6:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:13:fn run_one_drive() -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:35:    #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:40:    #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:50:    pub fn drive(self, control: DriveControl) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:61:    fn drive_guarded(self, control: DriveControl) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:62:        #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:108:        #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:119:    #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:120:    fn from_test_worker(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:137:    fn resolve(self) -> io::Result<(PathBuf, Vec<OsString>)> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:139:            #[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:160:    pub fn unbounded() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:169:    pub fn with_deadline_after(duration: Duration) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:178:    pub fn cancellable() -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:191:    pub fn cancellable_with_deadline_after(duration: Duration) -> (Self, DriveCancellation) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:206:    pub fn cancel(&self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:210:    fn is_selected(&self) -> bool {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:216:    fn deadline_is_elapsed(&self) -> bool {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:239:    pub fn into_bytes(self) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:262:    pub fn kind(&self) -> MechanicalFaultKind {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:266:    pub fn diagnostic(&self) -> &str {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:270:    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:303:    fn mint() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:311:    fn mint() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:326:    fn into_terminal(self) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:345:    fn new(kind: MechanicalFaultKind, diagnostic: impl Into<String>) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:352:    fn into_fault(self) -> MechanicalFault {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:370:    fn admit(child: Child, request: Vec<u8>) -> Result<Self, MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:497:    fn select_terminal(&mut self, control: &DriveControl) -> SelectedTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:601:    fn retire(&mut self) -> Result<(), MechanicalFault> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:647:    fn drop(&mut self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:667:fn write_request(writer: &mut impl Write, request: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:671:fn read_single_candidate(reader: &mut impl Read) -> Result<Vec<u8>, FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:722:fn worker_candidate_fault(kind: u8, diagnostic: Vec<u8>) -> FaultSeed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:745:fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:768:fn worker_exit_fault(status: ExitStatus) -> FaultSeed {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:785:fn bounded_diagnostic(mut diagnostic: String) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:807:    fn for_child(child: &Child) -> io::Result<Self> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:856:    fn terminate_descendants(&mut self) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:895:    fn drop(&mut self) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:904:fn configure_retirement_boundary(command: &mut Command) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:917:fn configure_retirement_boundary(_command: &mut Command) {}
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:919:#[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:950:        fn test_worker(behavior: WorkerBehavior) -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1030:    fn hex(bytes: &[u8]) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1040:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1041:    fn opaque_provider_looking_bytes_cross_the_real_drive_unchanged() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1052:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1053:    fn cargo_followed_by_a_hung_worker_is_discarded_at_deadline() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1062:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1063:    fn cancellation_before_spawn_does_not_require_a_worker_asset() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1078:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1079:    fn never_settling_worker_is_cancelled_and_retired() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1094:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1095:    fn malformed_truncated_duplicate_and_oversized_frames_are_typed_faults() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1111:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1112:    fn nonzero_worker_exit_is_a_typed_fault_after_reap() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1122:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1123:    fn inherited_protocol_descriptor_descendant_is_retired_before_cargo_returns() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1137:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1138:    fn each_drive_uses_a_fresh_worker_process() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1143:                panic!("worker pid fixture must return cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1154:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1155:    fn wrong_version_and_worker_rejection_remain_distinct_typed_faults() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1173:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1174:    fn large_worker_stderr_is_drained_without_blocking_cargo() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1185:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1186:    fn worker_abort_is_reaped_as_signal_termination() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1195:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1196:    fn supervisor_unwind_after_admission_retires_before_fault_return() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1206:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1207:    fn cancellation_deadline_race_selects_exactly_one_post_retirement_terminal() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1225:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1226:    fn oversized_request_faults_before_worker_spawn() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/Cargo.lock:19:name = "libbun-public-api-boundary-fixture"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/Cargo.toml:2:name = "libbun-public-api-boundary-fixture"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs:5:fn adjacent_public_controls_remain_available(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs:12:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs:1:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs:2:    let _ = libbun::install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:1:use libbun::install_prepared_export;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:3:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:4:    let _ = install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:5:fn check_fixture(bin: &str) -> Output {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:7:    let manifest = repository.join("tests/fixtures/public_api_boundary/Cargo.toml");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:26:        .expect("external public-API fixture cargo check must launch")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:29:#[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:30:fn raw_installer_is_absent_from_external_import_and_call_surfaces() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:31:    let control = check_fixture("adjacent-public-controls");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:42:        let output = check_fixture(bin);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:49:            stderr.contains("install_prepared_export") && stderr.contains(intended_diagnostic),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:28:pub fn encode_drive_material(request: DriveRequest) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:44:pub fn write_drive_request(writer: &mut impl Write, material: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:58:pub fn read_drive_request(reader: &mut impl Read) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:85:pub fn write_cargo(writer: &mut impl Write, cargo: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:89:pub fn write_fault(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:97:fn write_candidate(writer: &mut impl Write, kind: u8, payload: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:112:fn decode_drive_material(material: &[u8]) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:127:fn take_field(material: &[u8], cursor: &mut usize) -> io::Result<Vec<u8>> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:149:fn invalid(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:153:#[cfg(test)]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:157:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:158:    fn request_round_trips_opaque_fields() {

## Adjacent exact-call and invocation producers

Meaning: Binds the real exact-route producer and sealed invocation/output-settlement producer.

Expected result: Exit 0; constructors, raw splitters, and settlement operations are all present.

Command: git -C /home/ubuntu/swarm grep -n -E 'ManifestResolvedExternalProviderCallAuthority|ManifestResolvedExternalProviderCallAdmission|DurableExternalProviderInvocationAuthority|select_exact_call|into_contract_and_module|into_call_input_and_output_settlement' 95323ff17cb29928e31467f651ef03bae2099c14 -- crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs

Pattern: ManifestResolvedExternalProviderCallAuthority|ManifestResolvedExternalProviderCallAdmission|DurableExternalProviderInvocationAuthority|select_exact_call|into_contract_and_module|into_call_input_and_output_settlement

Pathspecs: crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs

Exit: 0

Output:

95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:142:        call_authority: ManifestResolvedExternalProviderCallAuthority,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:151:    ManifestResolvedExternal(DurableExternalProviderInvocationAuthority),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:154:// compiler-custody: symbol=DurableExternalProviderInvocationAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:156:pub struct DurableExternalProviderInvocationAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:157:    call_authority: ManifestResolvedExternalProviderCallAuthority,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:1196:        admission: ManifestResolvedExternalProviderCallAdmission,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:1320:                    DurableExternalProviderInvocationAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:1339:impl DurableExternalProviderInvocationAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:1340:    pub fn into_call_input_and_output_settlement_for_durable_external_provider_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs:1343:        ManifestResolvedExternalProviderCallAuthority,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:329:// compiler-custody: symbol=ManifestResolvedExternalProviderCallAuthority disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=Self::select_exact_call_for_provider_host_set_owner_v1; consumer=Self::contract_for_provider_host_set_owner_v1; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:331:pub struct ManifestResolvedExternalProviderCallAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:339:pub struct ManifestResolvedExternalProviderCallAdmission {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:340:    call_authority: ManifestResolvedExternalProviderCallAuthority,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:345:pub enum ManifestResolvedExternalProviderCallAdmissionSelection {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:346:    Admitted(ManifestResolvedExternalProviderCallAdmission),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:351:pub struct ManifestResolvedExternalProviderCallAdmissionSelectionFault {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:353:    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:357:enum ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:919:        ManifestResolvedExternalProviderCallAdmissionSelection,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:920:        ManifestResolvedExternalProviderCallAdmissionSelectionFault,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:923:            Some(routes) => routes.select_exact_call_for_provider_host_set_owner_v1(contract),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:924:            None => Ok(ManifestResolvedExternalProviderCallAdmissionSelection::Unmatched(contract)),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:930:    fn select_exact_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:934:        ManifestResolvedExternalProviderCallAdmissionSelection,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:935:        ManifestResolvedExternalProviderCallAdmissionSelectionFault,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:960:                            ManifestResolvedExternalProviderCallAdmissionSelection::Admitted(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:961:                                ManifestResolvedExternalProviderCallAdmission {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:962:                                    call_authority: ManifestResolvedExternalProviderCallAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:985:                ManifestResolvedExternalProviderCallAdmissionSelectionFault {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:987:                    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ExactContractOperationMismatch {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:998:                ManifestResolvedExternalProviderCallAdmissionSelectionFault {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1000:                    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ContractFingerprintMismatch {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1009:        Ok(ManifestResolvedExternalProviderCallAdmissionSelection::Unmatched(contract))
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1013:impl ManifestResolvedExternalProviderCallAdmission {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1017:        ManifestResolvedExternalProviderCallAuthority,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1024:impl ManifestResolvedExternalProviderCallAdmissionSelectionFault {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1032:impl std::fmt::Display for ManifestResolvedExternalProviderCallAdmissionSelectionFault {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1035:            ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ContractFingerprintMismatch {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1046:            ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ExactContractOperationMismatch {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1058:impl std::error::Error for ManifestResolvedExternalProviderCallAdmissionSelectionFault {}
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1060:impl ManifestResolvedExternalProviderCallAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1067:    pub fn into_contract_and_module_for_durable_external_provider_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1093:impl std::fmt::Debug for ManifestResolvedExternalProviderCallAuthority {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs:1096:            .debug_struct("ManifestResolvedExternalProviderCallAuthority")

## Adjacent sole consumer, transport, process, and shutdown graph

Meaning: Binds the sole libbun consumer, callback trait boundary, raw reconstruction, process callers, shutdown, and Cargo direction.

Expected result: Exit 0; all current cross-repository ownership and compatibility shapes are visible.

Command: git -C /home/ubuntu/swarm grep -n -E 'SsExternalCapabilityProviderHost|invoke_manifest_resolved_call|ProviderRequest|adapter_source|begin_invocation|settle_provider|shutdown|impl Drop|Command::new|wait_with_output|libbun' 95323ff17cb29928e31467f651ef03bae2099c14 -- crates/ss-runtime-external-capability-provider-owner/src/lib.rs crates/swarm-provider-host-set/src/external_transport.rs crates/swarm-provider-host-set/src/provider_host_set.rs crates/ss/src/product.rs crates/ss/tests/external_capability_provider.rs crates/ss-runtime-external-capability-provider-owner/Cargo.toml crates/ss/Cargo.toml Cargo.toml

Pattern: SsExternalCapabilityProviderHost|invoke_manifest_resolved_call|ProviderRequest|adapter_source|begin_invocation|settle_provider|shutdown|impl Drop|Command::new|wait_with_output|libbun

Pathspecs: crates/ss-runtime-external-capability-provider-owner/src/lib.rs crates/swarm-provider-host-set/src/external_transport.rs crates/swarm-provider-host-set/src/provider_host_set.rs crates/ss/src/product.rs crates/ss/tests/external_capability_provider.rs crates/ss-runtime-external-capability-provider-owner/Cargo.toml crates/ss/Cargo.toml Cargo.toml

Exit: 0

Output:

95323ff17cb29928e31467f651ef03bae2099c14:Cargo.toml:268:libbun = { git = "https://github.com/enki/libbun.git", rev = "2f053c35e56a47468391341ba8d89d347711641e" }
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/Cargo.toml:9:libbun = { workspace = true, features = ["dynamic-loading"] }
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:30:    Libbun(#[from] libbun::LibbunError),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:38:    libbun::BunProviderBackend<libbun::dynamic::DynamicBunRuntime>;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:49:pub struct SsExternalCapabilityProviderHost {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:54:impl std::fmt::Debug for SsExternalCapabilityProviderHost {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:57:            .debug_struct("SsExternalCapabilityProviderHost")
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:63:impl SsExternalCapabilityProviderHost {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:79:        let mut config = libbun::BunRuntimeConfig::new("libbun", working_directory);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:80:        config.stdout = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:81:        config.stderr = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:82:        config.log = libbun::SinkPolicy::Drop;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:90:    pub fn shutdown(&mut self) -> SsExternalCapabilityProviderResult<()> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:91:        self.backend.shutdown()?;
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:96:pub fn install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:103:    let host = ExternalTransportCapabilityProviderHost::libbun_for_ss_external_capability_provider_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:115:        SsExternalCapabilityProviderHost::new_with_dropped_process_output(&self.working_directory)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:121:impl DurableExternalCapabilityProvider for SsExternalCapabilityProviderHost {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:122:    fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:158:            "ss-libbun-external-provider-{}",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:161:        let provider_adapter_source = libbun_provider_adapter_source_for_selected_route(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:167:        let request = libbun::ProviderRequest {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:168:            contract: libbun::ProviderContractIdentity {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:173:            domain: libbun::ProviderDomainClass::JavaScriptExternalTransport,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:174:            module: libbun::BunModuleSpec::Source {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:176:                source: provider_adapter_source,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:179:            input: libbun::StructuralValue(input),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:181:        let descriptor = libbun::ProviderInvocationDescriptor::new(invocation_id.clone())
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:182:            .with_output_policy(libbun::InvocationOutputPolicy::Drop);
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:185:            .begin_invocation(descriptor)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss-runtime-external-capability-provider-owner/src/lib.rs:187:                lease.settle_provider(
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
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/Cargo.toml:15:    "native-distribution-libbun",
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/Cargo.toml:17:native-distribution-libbun = []
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/Cargo.toml:36:libbun = { workspace = true, features = ["dynamic-loading"] }
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:61:            libbun_external_capability_provider_enabled,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:62:        } => run_providers_operation(libbun_external_capability_provider_enabled),
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:143:    ss_runtime_external_capability_provider_owner::install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:247:fn run_providers_operation(libbun_external_capability_provider_enabled: bool) -> SsResult<Value> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:248:    ss_runtime_provider_listing_owner::providers_with_libbun(
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:249:        libbun_external_capability_provider_enabled,
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:705:impl Drop for SubstrateDiagnosticJson {
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/src/product.rs:858:        let output = Command::new(std::env::current_exe().expect("test executable must resolve"))
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:27:    let output = Command::new(&ss_binary)
95323ff17cb29928e31467f651ef03bae2099c14:crates/ss/tests/external_capability_provider.rs:77:    let mut child = Command::new(&ss_binary)
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
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:116:    pub(crate) fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:126:            .invoke_manifest_resolved_call_for_provider_host_set_owner_v1(invocation)
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:129:    pub(crate) fn shutdown_for_provider_host_set_owner_v1(&mut self) -> CapabilitySdkResult<()> {
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/external_transport.rs:131:            provider.shutdown_for_provider_host_set_owner_v1()?;
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:11:    HostAdmittedTypedProviderRequest, ProviderHostContext, ProviderHostResourceReleaseFaultV1,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:164:/// The `LoadedNativePark` payload is a SEALED [`HostAdmittedTypedProviderRequest`]
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:176:    LoadedNativePark(HostAdmittedTypedProviderRequest),
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:182:        request: HostAdmittedTypedProviderRequest,
95323ff17cb29928e31467f651ef03bae2099c14:crates/swarm-provider-host-set/src/provider_host_set.rs:199:    ) -> Option<HostAdmittedTypedProviderRequest> {
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
