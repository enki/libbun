# libbun W1-11/W1-12 exact-source search report

This report was generated read-only from candidate 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb. It is negative and topology evidence for the review bundles; it is not an implementation verdict.

## Reproduction output

Candidate SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb
Candidate tree: cb964de8ab8162449fbe95959bf34d231570aa5c

[required implementation symbols; expected absent]
exit=1

[current forbidden/rejected shapes]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3560: "windows_aarch64_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3563: "windows_i686_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3566: "windows_x86_64_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3571:name = "windows_aarch64_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3589:name = "windows_i686_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.lock:3607:name = "windows_x86_64_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:14:internal-adapter = []
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:43:        println!("cargo:rustc-link-arg=-fsanitize=null");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:86:                repo_root.join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:89:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:90:        .join("libbun_native_link_manifest.txt")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/build.rs:110:                .join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:7:#[cfg(not(feature = "internal-adapter"))]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:33:use libbun_prepared_export_wire::DriveRequest;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:98:    fn null() -> Self {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:244:pub fn drive_prepared_export(request: DriveRequest) -> Result<Vec<u8>, NativeDriveFailure> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:285:                    WorkerFaultKind::JavaScriptRejection,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:330:        // `shutdown`, which consumes all public operations through the facade.
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:355:        if value.is_undefined() || value.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:356:            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:367:            return Ok(ProviderCallResult::Ok(StructuralValue::null()));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:386:    fn js_error_to_string(&self, value: JSValue, fallback: &str) -> String {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:431:                .unwrap_or_else(|| fallback.to_string());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:436:            .unwrap_or_else(|| fallback.to_string())
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:500:    fn drain_output(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:501:        bun_core::Output::flush();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:502:        self.stdout.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:503:        self.stderr.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:504:        self.log.drain()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:523:        let bundle_dir = tempdir.path().join(format!("{module_id}.bundle"));
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:529:            let path = bundle_dir.join(module_path);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:544:        let entry_module = bundle_dir.join(bundle.entry_module);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:565:    fn drain(&mut self) -> LibbunResult<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:751:            NonNull::new(vm).ok_or_else(|| LibbunError::initialize("Bun VM init returned null"))?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:787:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:823:                self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:833:            self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:838:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:847:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:878:        self.drain_output()?;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:897:            .join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:925:    if len == 0 || data.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:951:    let bytes = if msg.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/src/lib.rs:961:    if vm.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3554: "windows_aarch64_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3557: "windows_i686_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3560: "windows_x86_64_gnullvm",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3565:name = "windows_aarch64_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3583:name = "windows_i686_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.lock:3601:name = "windows_x86_64_gnullvm"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:14:libbun-native = { path = "../native", features = ["internal-adapter"] }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:51:        println!("cargo:rustc-link-arg=-fsanitize=null");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:109:                repo_root.join(path)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:112:        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/build.rs:113:        .join("libbun_native_link_manifest.txt")
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/src/main.rs:25:    match libbun_native::drive_prepared_export(request) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:10:cargo +nightly-2026-05-06 check --manifest-path "$repo_root/native/Cargo.toml" --features internal-adapter
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:18:  if command -v sha256sum >/dev/null 2>&1; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:69:    "execution": "fresh-process-only",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:70:    "fallback": None,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:93:if grep -F "build/debug" "$manifest" >/dev/null ||
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:94:  grep -F "bun-debug" "$manifest" >/dev/null ||
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:95:  grep -F -- "-debug/" "$manifest" >/dev/null; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun-reproducible.sh:20:  if ! command -v "$1" >/dev/null 2>&1; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:287:    JavaScriptRejection,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:375:                let _ = child.kill();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:376:                let _ = child.wait();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:475:                    drain_bounded_stderr(&mut stderr)
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:616:            && let Err(error) = child.wait()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:631:            if handle.is_some_and(|handle| handle.join().is_err()) {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:646:impl Drop for DriveGuard {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:654:            std::process::abort();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:728:        value if value == WorkerFaultKind::JavaScriptRejection as u8 => {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:729:            MechanicalFaultKind::JavaScriptRejection
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:745:fn drain_bounded_stderr(stderr: &mut ChildStderr) -> Result<(), FaultSeed> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:825:            let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:826:            if job.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:859:            let result = unsafe { libc::kill(-self.process_group, libc::SIGKILL) };
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:872:            if self.job.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:878:            self.job = std::ptr::null_mut();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:894:impl Drop for ProcessBoundary {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:896:        if !self.job.is_null() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:898:            self.job = std::ptr::null_mut();
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:908:            if libc::setpgid(0, 0) == -1 {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:968:    sys.stdout.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:993:    sys.stderr.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:999:sys.stdout.buffer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1091:        cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1170:        assert_eq!(rejection.kind(), MechanicalFaultKind::JavaScriptRejection);
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1174:    fn large_worker_stderr_is_drained_without_blocking_cargo() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1180:            panic!("bounded diagnostic drain should not block cargo");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:1217:            cancelling.join().expect("cancellation thread joins");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs:2:    let _ = libbun::install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:1:use libbun::install_prepared_export;
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:4:    let _ = install_prepared_export(Vec::new(), String::new(), Vec::new());
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:7:    let manifest = repository.join("tests/fixtures/public_api_boundary/Cargo.toml");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:10:        .unwrap_or_else(|| repository.join("target"))
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:11:        .join("external-public-api-boundary");
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:49:            stderr.contains("install_prepared_export") && stderr.contains(intended_diagnostic),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:12:pub struct DriveRequest {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:23:    JavaScriptRejection = 3,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:28:pub fn encode_drive_material(request: DriveRequest) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:55:    writer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:58:pub fn read_drive_request(reader: &mut impl Read) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:109:    writer.flush()
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:112:fn decode_drive_material(material: &[u8]) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:120:    Ok(DriveRequest {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:159:        let material = encode_drive_material(DriveRequest {
exit=0

[test definitions]
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
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:50:    pub fn drive(self, control: DriveControl) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:61:    fn drive_guarded(self, control: DriveControl) -> MechanicalTerminal {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:120:    fn from_test_worker(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:src/prepared_export.rs:137:    fn resolve(self) -> io::Result<(PathBuf, Vec<OsString>)> {
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
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs:5:fn adjacent_public_controls_remain_available(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs:12:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs:1:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs:3:fn main() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:5:fn check_fixture(bin: &str) -> Output {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:29:#[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:tests/public_api_boundary.rs:30:fn raw_installer_is_absent_from_external_import_and_call_surfaces() {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:28:pub fn encode_drive_material(request: DriveRequest) -> Vec<u8> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:44:pub fn write_drive_request(writer: &mut impl Write, material: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:58:pub fn read_drive_request(reader: &mut impl Read) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:85:pub fn write_cargo(writer: &mut impl Write, cargo: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:89:pub fn write_fault(
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:97:fn write_candidate(writer: &mut impl Write, kind: u8, payload: &[u8]) -> io::Result<()> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:112:fn decode_drive_material(material: &[u8]) -> io::Result<DriveRequest> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:127:fn take_field(material: &[u8], cursor: &mut usize) -> io::Result<Vec<u8>> {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:149:fn invalid(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:157:    #[test]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/src/lib.rs:158:    fn request_round_trips_opaque_fields() {
exit=0

[crate and workflow topology]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:.github/workflows/ci.yml:20:      - name: Test private one-drive wire
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:.github/workflows/ci.yml:21:        run: cargo test --manifest-path wire/Cargo.toml
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:.github/workflows/ci.yml:26:      - name: Check one-shot native worker
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:.github/workflows/ci.yml:27:        run: cargo +nightly-2026-05-06 check --manifest-path runtime/Cargo.toml
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:2:name = "libbun"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:8:repository = "https://github.com/enki/libbun"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:9:homepage = "https://github.com/enki/libbun"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:19:    "/wire/**",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:Cargo.toml:27:libbun-prepared-export-wire = { path = "wire" }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:2:name = "libbun-native"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:14:internal-adapter = []
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:21:bun_runtime = { path = "../vendor/bun/src/runtime" }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:native/Cargo.toml:22:libbun-prepared-export-wire = { path = "../wire" }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:2:name = "libbun-runtime-native"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:10:name = "libbun-runtime-native"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:14:libbun-native = { path = "../native", features = ["internal-adapter"] }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:runtime/Cargo.toml:15:libbun-prepared-export-wire = { path = "../wire" }
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:7:pic_patch="$repo_root/patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:8:call_frame_patch="$repo_root/patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/apply-vendored-bun-patches.sh:45:echo "Applied libbun vendored Bun patches"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:9:cargo +nightly-2026-05-06 check --manifest-path "$repo_root/vendor/bun/Cargo.toml" -p bun_runtime
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:10:cargo +nightly-2026-05-06 check --manifest-path "$repo_root/native/Cargo.toml" --features internal-adapter
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/check-vendored-bun-rust.sh:11:cargo +nightly-2026-05-06 check --manifest-path "$repo_root/runtime/Cargo.toml"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/configure-vendored-bun.sh:7:build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$bun_dir/build/release"}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:5:  echo "usage: $0 <version> <release-worker-binary> <output-directory>" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:33:if [[ "$worker" != */release/libbun-runtime-native && "$worker" != */release/libbun-runtime-native.exe ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:34:  echo "release bundles require the release-profile libbun-runtime-native binary: $worker" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:43:    echo "unsupported prepared-export worker release target: $(uname -s)-$(uname -m)" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:60:    "format": "libbun.preparedExportWorker",
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:62:    "wireVersion": 1,
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:66:        "filename": "libbun-runtime-native" + (".exe" if "windows" in target else ""),
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/package-prepared-export-worker-release.sh:74:archive="$output/libbun-prepared-export-worker-${version}-${target}.tar.zst"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:6:profile="${LIBBUN_NATIVE_BUN_PROFILE:-release}"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:9:if [[ "$profile" != "release" ]]; then
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:10:  echo "libbun native worker links must be prepared from Bun's release profile; got LIBBUN_NATIVE_BUN_PROFILE=$profile" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:17:archive="$build_dir/libbun_native_objects.a"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:18:objects_file="$build_dir/libbun_native_objects.txt"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:19:static_libs_file="$build_dir/libbun_native_static_libs.txt"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:20:build_inputs_file="$build_dir/libbun_native_build_inputs.txt"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:21:manifest="$build_dir/libbun_native_link_manifest.txt"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:32:  echo "libbun native worker links must use Bun's release bun-profile target; got LIBBUN_NATIVE_BUN_EXE_TARGET=$exe_target" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:53:  echo "no Bun native object files found from ninja query" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:63:  echo "no Bun native link inputs found from ninja query" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:77:    echo "unsupported native worker build OS: $(uname -s)" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:96:  echo "native Bun link manifest contains debug build inputs: $manifest" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/prepare-native-bun-link.sh:107:echo "Prepared native Bun link manifest for $exe_target at $manifest"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun-reproducible.sh:59:  echo "vendored Bun source is not identical after applying libbun scripts" >&2
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun-reproducible.sh:64:echo "vendored Bun is reproducible from $repo_url at $commit plus libbun scripts"
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:scripts/verify-vendored-bun.sh:21:for required in Cargo.toml package.json src/bun_bin/Cargo.toml src/jsc/Cargo.toml src/runtime/Cargo.toml vendor/lolhtml/c-api/Cargo.toml; do
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/Cargo.toml:1:[package]
6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb:wire/Cargo.toml:2:name = "libbun-prepared-export-wire"
exit=0

[tracked lock and workflow files]
.github/workflows/ci.yml
Cargo.lock
native/Cargo.lock
runtime/Cargo.lock
tests/fixtures/public_api_boundary/Cargo.lock
vendor/bun/Cargo.lock
vendor/bun/bench/ffi/src/Cargo.lock
vendor/bun/packages/bun-native-plugin-rs/Cargo.lock
vendor/bun/scripts/verify-baseline-static/Cargo.lock
vendor/bun/vendor/lolhtml/c-api/Cargo.lock
vendor/bun/vendor/lolhtml/js-api/Cargo.lock
