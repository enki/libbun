#![deny(unsafe_op_in_unsafe_fn)]

use libloading::Library;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use swarm_capability_model::CapabilitySdkError;
use swarm_native_provider_authority::{
    NativeProviderArtifactBytesAdmission, NativeProviderFunctionTableError,
    NativeProviderHostLoadAdmissionSet, NativeProviderHostLoadRequest,
    NativeProviderLoadedHostAdmission, NativeProviderLoaderInstalledHostAdmission,
    SWARM_NATIVE_PROVIDER_ENTRYPOINT_V1, SwarmNativeProviderEntrypointV1, SwarmNativeProviderV1,
    SwarmNativeProviderV1LoaderOwnerOpsV1,
};
use swarm_provider_host_set::{ProviderHostExecutionSession, ProviderHostSet};
use swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NativeProviderLoaderError {
    #[error(transparent)]
    CapabilitySdk(#[from] CapabilitySdkError),
    #[error("native provider artifact read failed for {path}: {source}")]
    ArtifactRead {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("native provider artifact load failed for {path}: {source}")]
    ArtifactLoad {
        path: String,
        #[source]
        source: libloading::Error,
    },
    #[error("native provider entrypoint load failed for {path}: {source}")]
    EntrypointLoad {
        path: String,
        #[source]
        source: libloading::Error,
    },
    #[error("native provider artifact reported unsupported ABI version {observed}")]
    UnsupportedAbiVersion { observed: u32 },
    #[error("native provider artifact did not expose {function_name}")]
    MissingFunction { function_name: &'static str },
    #[error("native provider artifact returned an invalid buffer for {function_name}")]
    InvalidBuffer { function_name: &'static str },
    #[error("native provider artifact descriptor JSON was invalid: {source}")]
    InvalidDescriptorJson {
        #[source]
        source: serde_json::Error,
    },
    #[error("native provider artifact descriptor rejected: {reason}")]
    InvalidDescriptorShape { reason: String },
}

pub type NativeProviderLoaderResult<T> = Result<T, NativeProviderLoaderError>;

impl From<NativeProviderFunctionTableError> for NativeProviderLoaderError {
    fn from(value: NativeProviderFunctionTableError) -> Self {
        match value {
            NativeProviderFunctionTableError::CapabilitySdk(source) => {
                NativeProviderLoaderError::CapabilitySdk(source)
            }
            NativeProviderFunctionTableError::UnsupportedAbiVersion { observed } => {
                NativeProviderLoaderError::UnsupportedAbiVersion { observed }
            }
            NativeProviderFunctionTableError::MissingFunction { function_name } => {
                NativeProviderLoaderError::MissingFunction { function_name }
            }
            NativeProviderFunctionTableError::InvalidBuffer { function_name } => {
                NativeProviderLoaderError::InvalidBuffer { function_name }
            }
            NativeProviderFunctionTableError::InvalidDescriptorJson { source } => {
                NativeProviderLoaderError::InvalidDescriptorJson { source }
            }
            NativeProviderFunctionTableError::InvalidDescriptorShape { reason } => {
                NativeProviderLoaderError::InvalidDescriptorShape { reason }
            }
        }
    }
}

struct LoadedNativeProviderArtifact {
    _library: Library,
    _stable_backing: StableAdmittedNativeProviderArtifactBacking,
    pending_sdk_host: Option<NativeProviderLoadedHostAdmission>,
    sdk_host: Option<NativeProviderLoaderInstalledHostAdmission>,
    provider: SwarmNativeProviderV1,
}

struct StableAdmittedNativeProviderArtifactBacking {
    _file: File,
    load_path: PathBuf,
    cleanup_path: Option<PathBuf>,
}

impl StableAdmittedNativeProviderArtifactBacking {
    fn stage(artifact_bytes: &[u8]) -> io::Result<Self> {
        let mut last_collision = None;
        for _ in 0..64 {
            let path = std::env::temp_dir().join(format!(
                ".swarm-native-provider-{:032x}",
                rand::random::<u128>()
            ));
            match open_private_stable_backing(&path) {
                Ok(mut file) => {
                    if let Err(error) = file
                        .write_all(artifact_bytes)
                        .and_then(|()| file.sync_all())
                    {
                        let _ = fs::remove_file(&path);
                        return Err(error);
                    }
                    return finish_private_stable_backing(file, path);
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    last_collision = Some(error);
                }
                Err(error) => return Err(error),
            }
        }
        Err(last_collision.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                "could not allocate unique stable native-provider backing",
            )
        }))
    }
}

impl Drop for StableAdmittedNativeProviderArtifactBacking {
    fn drop(&mut self) {
        if let Some(path) = self.cleanup_path.take() {
            let _ = fs::remove_file(path);
        }
    }
}

#[cfg(unix)]
fn open_private_stable_backing(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(windows)]
fn open_private_stable_backing(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .share_mode(FILE_SHARE_READ)
        .open(path)
}

#[cfg(unix)]
fn finish_private_stable_backing(
    file: File,
    path: PathBuf,
) -> io::Result<StableAdmittedNativeProviderArtifactBacking> {
    use std::os::fd::AsRawFd;

    fs::remove_file(&path)?;
    #[cfg(target_os = "linux")]
    let load_path = PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()));
    #[cfg(not(target_os = "linux"))]
    let load_path = PathBuf::from(format!("/dev/fd/{}", file.as_raw_fd()));
    Ok(StableAdmittedNativeProviderArtifactBacking {
        _file: file,
        load_path,
        cleanup_path: None,
    })
}

#[cfg(windows)]
fn finish_private_stable_backing(
    file: File,
    path: PathBuf,
) -> io::Result<StableAdmittedNativeProviderArtifactBacking> {
    Ok(StableAdmittedNativeProviderArtifactBacking {
        _file: file,
        load_path: path.clone(),
        cleanup_path: Some(path),
    })
}

struct PendingNativeProviderFunctionTableDropGuard {
    provider: Option<SwarmNativeProviderV1>,
}

impl PendingNativeProviderFunctionTableDropGuard {
    fn new(provider: SwarmNativeProviderV1) -> Self {
        Self {
            provider: Some(provider),
        }
    }

    fn provider(&self) -> &SwarmNativeProviderV1 {
        self.provider
            .as_ref()
            .expect("pending native provider function table must remain guarded")
    }

    fn into_provider(mut self) -> SwarmNativeProviderV1 {
        self.provider
            .take()
            .expect("pending native provider function table must disarm once")
    }
}

impl Drop for PendingNativeProviderFunctionTableDropGuard {
    fn drop(&mut self) {
        if let Some(mut provider) = self.provider.take() {
            provider.drop_for_durable_native_provider_loader_owner_v1();
        }
    }
}

impl LoadedNativeProviderArtifact {
    fn load_from_host_load_request(
        request: NativeProviderHostLoadRequest,
    ) -> NativeProviderLoaderResult<Self> {
        let artifact_path =
            request.admitted_artifact_path_for_durable_native_provider_loader_owner_v1()?;
        let artifact_bytes = read_artifact_bytes(&artifact_path)?;
        let request = request
            .admit_artifact_bytes_for_durable_native_provider_loader_owner_v1(&artifact_bytes)?;
        let stable_backing = StableAdmittedNativeProviderArtifactBacking::stage(&artifact_bytes)
            .map_err(|source| NativeProviderLoaderError::ArtifactRead {
                path: artifact_path.display().to_string(),
                source,
            })?;
        Self::load_from_stable_backing(artifact_path, stable_backing, request)
    }

    fn load_from_stable_backing(
        admitted_artifact_path: PathBuf,
        stable_backing: StableAdmittedNativeProviderArtifactBacking,
        sdk_host: NativeProviderArtifactBytesAdmission,
    ) -> NativeProviderLoaderResult<Self> {
        let path_label = admitted_artifact_path.display().to_string();
        let library = unsafe {
            // SAFETY: `stable_backing` contains exactly the bytes admitted above and is
            // retained after `Library` in `LoadedNativeProviderArtifact`, so neither the
            // loaded bytes nor the function pointers can be replaced or outlive their file.
            Library::new(&stable_backing.load_path).map_err(|source| {
                NativeProviderLoaderError::ArtifactLoad {
                    path: path_label.clone(),
                    source,
                }
            })?
        };
        let entrypoint = unsafe {
            // SAFETY: The symbol name is the ADR-2042 C ABI entrypoint. The provider is
            // accepted only after its descriptor is read and validated against the load plan.
            library
                .get::<SwarmNativeProviderEntrypointV1>(SWARM_NATIVE_PROVIDER_ENTRYPOINT_V1)
                .map_err(|source| NativeProviderLoaderError::EntrypointLoad {
                    path: path_label.clone(),
                    source,
                })?
        };
        let provider = unsafe {
            // SAFETY: The loaded symbol has the exact `extern "C"` entrypoint type required
            // above. Its returned function table is validated before use.
            entrypoint()
        };
        let provider = PendingNativeProviderFunctionTableDropGuard::new(provider);
        let sdk_host = sdk_host
            .into_loaded_host_admission_from_function_table_for_durable_native_provider_loader_owner_v1(
                provider.provider(),
            )?;
        Ok(Self {
            _library: library,
            _stable_backing: stable_backing,
            pending_sdk_host: Some(sdk_host),
            sdk_host: None,
            provider: provider.into_provider(),
        })
    }

    fn sdk_host(&self) -> &NativeProviderLoaderInstalledHostAdmission {
        self.sdk_host
            .as_ref()
            .expect("loaded native provider artifact must be installed before use")
    }

    fn take_pending_sdk_host(&mut self) -> NativeProviderLoadedHostAdmission {
        self.pending_sdk_host
            .take()
            .expect("loaded native provider artifact install admission must be consumed once")
    }

    fn install_sdk_host(&mut self, sdk_host: NativeProviderLoaderInstalledHostAdmission) {
        assert!(
            self.sdk_host.replace(sdk_host).is_none(),
            "loaded native provider artifact must be installed once"
        );
    }
}

impl Drop for LoadedNativeProviderArtifact {
    fn drop(&mut self) {
        self.provider
            .drop_for_durable_native_provider_loader_owner_v1();
    }
}

pub struct LoadedNativeProviderArtifactSet {
    provider_hosts: ProviderHostSet,
    artifacts: Vec<LoadedNativeProviderArtifact>,
}

impl std::fmt::Debug for LoadedNativeProviderArtifactSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LoadedNativeProviderArtifactSet")
            .field("provider_hosts", &self.provider_hosts.observations())
            .field("artifact_count", &self.artifacts.len())
            .finish()
    }
}

impl LoadedNativeProviderArtifactSet {
    pub fn load_from_host_load_admissions(
        provider_hosts: &ProviderHostSet,
        admissions: NativeProviderHostLoadAdmissionSet,
    ) -> NativeProviderLoaderResult<Self> {
        let admitted_loads =
            admissions.into_admitted_load_requests_for_durable_native_provider_loader_owner_v1()?;
        let mut artifacts = Vec::with_capacity(admitted_loads.len());
        for request in admitted_loads {
            let loaded = LoadedNativeProviderArtifact::load_from_host_load_request(request)?;
            artifacts.push(loaded);
        }
        let pending_hosts = artifacts
            .iter_mut()
            .map(LoadedNativeProviderArtifact::take_pending_sdk_host)
            .collect::<Vec<_>>();
        let installed_roles = pending_hosts
            .into_iter()
            .map(
                NativeProviderLoadedHostAdmission::into_installed_host_roles_for_durable_native_provider_loader_owner_v1,
            )
            .collect::<Vec<_>>();
        let (loader_hosts, installed_hosts): (Vec<_>, Vec<_>) = installed_roles.into_iter().unzip();
        let provider_hosts = provider_hosts
            .admit_loaded_native_provider_hosts_for_durable_native_provider_loader_owner_v1(
                installed_hosts,
            )?;
        for (artifact, loader_host) in artifacts.iter_mut().zip(loader_hosts) {
            artifact.install_sdk_host(loader_host);
        }
        Ok(Self {
            provider_hosts,
            artifacts,
        })
    }

    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    pub fn begin_provider_execution_session_v1(&self) -> ProviderHostExecutionSession {
        self.provider_hosts.begin_provider_execution_session_v1()
    }

    pub fn validate_compiled_swarm_binary_manifest_v1(
        &self,
        manifest: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> NativeProviderLoaderResult<()> {
        let _ = self;
        match manifest {}
    }

    pub fn admit_compiled_swarm_binary_manifest_for_libswarm_runtime_owner_v1(
        &self,
        manifest: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> NativeProviderLoaderResult<swarm_provider_host_set::AdmittedCompiledSwarmBinaryManifest>
    {
        let _ = self;
        match manifest {}
    }

    pub fn admit_compiled_swarm_binary_manifest_from_libswarm_runtime_owner_inputs_v1(
        &self,
        manifest_inputs: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> NativeProviderLoaderResult<swarm_provider_host_set::AdmittedCompiledSwarmBinaryManifest>
    {
        let _ = self;
        match manifest_inputs {}
    }

    pub(crate) fn provider_host_observation_projection_json_values(
        &self,
    ) -> Vec<serde_json::Value> {
        self.provider_hosts
            .observations()
            .into_iter()
            .map(|observation| observation.diagnostic_value())
            .collect()
    }

    pub(crate) fn artifact_projection_json_values(&self) -> Vec<serde_json::Value> {
        self.artifacts
            .iter()
            .map(|artifact| {
                artifact
                    .sdk_host()
                    .loaded_host_projection_json_for_durable_native_provider_loader_owner_v1()
            })
            .collect()
    }

    pub fn artifact_projection_json_values_for_libswarm_runner_artifact_owner_v1(
        &self,
    ) -> Vec<serde_json::Value> {
        self.artifact_projection_json_values()
    }

    pub fn to_runner_loaded_hosts_json_value(
        &self,
        schema: &'static str,
        runner: &'static str,
        platform: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "schema": schema,
            "runner": runner,
            "platform": platform,
            "providerHosts": self.provider_host_observation_projection_json_values(),
            "artifacts": self.artifact_projection_json_values(),
        })
    }
}

fn read_artifact_bytes(artifact_path: &Path) -> NativeProviderLoaderResult<Vec<u8>> {
    let path = artifact_path.display().to_string();
    fs::read(artifact_path)
        .map_err(|source| NativeProviderLoaderError::ArtifactRead { path, source })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn stable_backing_retains_exact_admitted_bytes() {
        let admitted = b"exact admitted native provider bytes";
        let backing = StableAdmittedNativeProviderArtifactBacking::stage(admitted)
            .expect("stage exact admitted bytes");
        assert_eq!(
            fs::read(&backing.load_path).expect("read stable backing through load path"),
            admitted
        );
    }

    #[test]
    fn pending_function_table_guard_drops_provider_once() {
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "C" fn record_drop(_provider_context: *mut c_void) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }

        DROPS.store(0, Ordering::SeqCst);
        let provider = SwarmNativeProviderV1 {
            abi_version: 0,
            provider_context: std::ptr::null_mut(),
            descriptor_json: None,
            drive_json: None,
            free_buffer: None,
            drop_provider: Some(record_drop),
        };
        drop(PendingNativeProviderFunctionTableDropGuard::new(provider));
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }
}
