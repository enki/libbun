use std::path::Path;
use std::path::PathBuf;

use sha2::Digest;

use crate::LibbunError;
use crate::LibbunResult;

pub const LIBBUN_PLUGIN_PATH_ENV: &str = "LIBBUN_PLUGIN_PATH";
pub const LIBBUN_HOME_ENV: &str = "LIBBUN_HOME";
pub const RELEASE_REPOSITORY: &str = "enki/libbun";
pub const RELEASE_TAG: &str = concat!("v", env!("CARGO_PKG_VERSION"));
pub const CHECKSUMS_ASSET_SUFFIX: &str = "checksums.txt";
pub const NOTICE_ASSET_SUFFIX: &str = "NOTICE.txt";
pub const SOURCE_ASSET_SUFFIX: &str = "SOURCE.txt";
pub const LICENSES_ASSET_SUFFIX: &str = "licenses.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativePluginAsset {
    pub repository: &'static str,
    pub release_tag: &'static str,
    pub target: &'static str,
    pub asset_name: String,
    pub plugin_filename: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativePluginSource {
    Environment,
    BuildTime,
    Cache,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedNativePlugin {
    pub path: PathBuf,
    pub source: NativePluginSource,
    pub asset: NativePluginAsset,
}

#[derive(Debug, Clone)]
pub struct NativePluginResolver {
    plugin_path: Option<PathBuf>,
    cache_root: Option<PathBuf>,
    use_build_time_plugin: bool,
}

impl Default for NativePluginResolver {
    fn default() -> Self {
        Self {
            plugin_path: None,
            cache_root: None,
            use_build_time_plugin: true,
        }
    }
}

impl NativePluginAsset {
    pub fn current() -> LibbunResult<Self> {
        let target = current_target_triple().ok_or_else(|| {
            LibbunError::initialize(format!(
                "libbun native plugin release assets are not available for host target `{}`",
                compile_time_target()
            ))
        })?;
        Ok(Self::for_target(target))
    }

    pub fn for_target(target: &'static str) -> Self {
        Self {
            repository: RELEASE_REPOSITORY,
            release_tag: RELEASE_TAG,
            target,
            asset_name: format!("libbun-plugin-native-{RELEASE_TAG}-{target}.tar.zst"),
            plugin_filename: plugin_filename_for_target(target),
        }
    }

    pub fn archive_url(&self) -> String {
        self.release_asset_url(&self.asset_name)
    }

    pub fn checksums_asset_name(&self) -> String {
        release_asset_name(CHECKSUMS_ASSET_SUFFIX)
    }

    pub fn notice_asset_name(&self) -> String {
        release_asset_name(NOTICE_ASSET_SUFFIX)
    }

    pub fn source_asset_name(&self) -> String {
        release_asset_name(SOURCE_ASSET_SUFFIX)
    }

    pub fn licenses_asset_name(&self) -> String {
        release_asset_name(LICENSES_ASSET_SUFFIX)
    }

    pub fn checksums_url(&self) -> String {
        self.release_asset_url(&self.checksums_asset_name())
    }

    pub fn notice_url(&self) -> String {
        self.release_asset_url(&self.notice_asset_name())
    }

    pub fn source_url(&self) -> String {
        self.release_asset_url(&self.source_asset_name())
    }

    pub fn licenses_url(&self) -> String {
        self.release_asset_url(&self.licenses_asset_name())
    }

    pub fn cache_dir(&self, cache_root: impl AsRef<Path>) -> PathBuf {
        cache_root.as_ref().join(self.release_tag).join(self.target)
    }

    pub fn cached_plugin_path(&self, cache_root: impl AsRef<Path>) -> PathBuf {
        self.cache_dir(cache_root).join(self.plugin_filename)
    }

    fn release_asset_url(&self, asset_name: &str) -> String {
        format!(
            "https://github.com/{}/releases/download/{}/{}",
            self.repository, self.release_tag, asset_name
        )
    }
}

impl NativePluginResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env() -> Self {
        Self {
            plugin_path: std::env::var_os(LIBBUN_PLUGIN_PATH_ENV).map(PathBuf::from),
            cache_root: std::env::var_os(LIBBUN_HOME_ENV).map(PathBuf::from),
            use_build_time_plugin: true,
        }
    }

    pub fn with_plugin_path(mut self, plugin_path: impl Into<PathBuf>) -> Self {
        self.plugin_path = Some(plugin_path.into());
        self
    }

    pub fn with_cache_root(mut self, cache_root: impl Into<PathBuf>) -> Self {
        self.cache_root = Some(cache_root.into());
        self
    }

    pub fn with_build_time_plugin(mut self, enabled: bool) -> Self {
        self.use_build_time_plugin = enabled;
        self
    }

    pub fn resolve(&self) -> LibbunResult<ResolvedNativePlugin> {
        let asset = NativePluginAsset::current()?;
        if let Some(plugin_path) = &self.plugin_path {
            if plugin_path.is_file() {
                return Ok(ResolvedNativePlugin {
                    path: plugin_path.clone(),
                    source: NativePluginSource::Environment,
                    asset,
                });
            }
            return Err(LibbunError::initialize(format!(
                "{LIBBUN_PLUGIN_PATH_ENV} points to `{}`, but no plugin file exists there",
                plugin_path.display()
            )));
        }

        if self.use_build_time_plugin
            && let Some(build_plugin_path) = build_time_plugin_path()
        {
            if build_plugin_path.is_file() {
                return Ok(ResolvedNativePlugin {
                    path: build_plugin_path,
                    source: NativePluginSource::BuildTime,
                    asset,
                });
            }
        }

        let cache_root = self
            .cache_root
            .clone()
            .or_else(default_cache_root)
            .ok_or_else(|| missing_plugin_error(&asset, None))?;
        let cached_plugin_path = asset.cached_plugin_path(&cache_root);
        if cached_plugin_path.is_file() {
            return Ok(ResolvedNativePlugin {
                path: cached_plugin_path,
                source: NativePluginSource::Cache,
                asset,
            });
        }

        Err(missing_plugin_error(&asset, Some(&cache_root)))
    }
}

pub fn current_native_plugin_asset() -> LibbunResult<NativePluginAsset> {
    NativePluginAsset::current()
}

pub fn resolve_native_plugin() -> LibbunResult<ResolvedNativePlugin> {
    NativePluginResolver::from_env().resolve()
}

pub fn build_time_plugin_path() -> Option<PathBuf> {
    option_env!("LIBBUN_BUILD_PLUGIN_PATH").map(PathBuf::from)
}

pub fn build_time_plugin_dir() -> Option<PathBuf> {
    option_env!("LIBBUN_BUILD_PLUGIN_DIR").map(PathBuf::from)
}

pub fn default_cache_root() -> Option<PathBuf> {
    if let Some(cache_home) = std::env::var_os("XDG_CACHE_HOME") {
        return Some(PathBuf::from(cache_home).join("libbun"));
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache").join("libbun"))
}

pub fn current_target_triple() -> Option<&'static str> {
    if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
        Some("aarch64-apple-darwin")
    } else if cfg!(all(
        target_arch = "x86_64",
        target_os = "linux",
        target_env = "gnu"
    )) {
        Some("x86_64-unknown-linux-gnu")
    } else if cfg!(all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu"
    )) {
        Some("aarch64-unknown-linux-gnu")
    } else {
        None
    }
}

pub fn compile_time_target() -> String {
    format!(
        "{}-{}-{}",
        std::env::consts::ARCH,
        std::env::consts::OS,
        std::env::consts::FAMILY
    )
}

pub fn sha256_file(path: impl AsRef<Path>) -> LibbunResult<String> {
    let bytes = std::fs::read(path.as_ref()).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to read `{}` for checksum verification: {err}",
            path.as_ref().display()
        ))
    })?;
    let digest = sha2::Sha256::digest(bytes);
    Ok(hex_lower(&digest))
}

pub fn expected_checksum(checksums: &str, asset_name: &str) -> LibbunResult<String> {
    for line in checksums.lines() {
        let Some((checksum, name)) = line.split_once("  ") else {
            continue;
        };
        if name == asset_name {
            return Ok(checksum.to_string());
        }
    }
    Err(LibbunError::initialize(format!(
        "checksum file does not contain `{asset_name}`"
    )))
}

pub fn verify_file_checksum(
    checksums: &str,
    asset_name: &str,
    path: impl AsRef<Path>,
) -> LibbunResult<()> {
    let expected = expected_checksum(checksums, asset_name)?;
    let actual = sha256_file(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(LibbunError::initialize(format!(
            "checksum mismatch for `{asset_name}`: expected {expected}, got {actual}"
        )))
    }
}

#[cfg(feature = "plugin-installer")]
pub fn install_native_plugin() -> LibbunResult<ResolvedNativePlugin> {
    NativePluginInstall::from_env().install()
}

#[cfg(feature = "plugin-installer")]
#[derive(Debug, Clone)]
pub struct NativePluginInstall {
    cache_root: Option<PathBuf>,
    overwrite: bool,
}

#[cfg(feature = "plugin-installer")]
impl NativePluginInstall {
    pub fn new() -> Self {
        Self {
            cache_root: None,
            overwrite: false,
        }
    }

    pub fn from_env() -> Self {
        Self {
            cache_root: std::env::var_os(LIBBUN_HOME_ENV).map(PathBuf::from),
            overwrite: false,
        }
    }

    pub fn with_cache_root(mut self, cache_root: impl Into<PathBuf>) -> Self {
        self.cache_root = Some(cache_root.into());
        self
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn install(&self) -> LibbunResult<ResolvedNativePlugin> {
        let asset = NativePluginAsset::current()?;
        let cache_root = self
            .cache_root
            .clone()
            .or_else(default_cache_root)
            .ok_or_else(|| missing_plugin_error(&asset, None))?;
        let install_dir = asset.cache_dir(&cache_root);
        let plugin_path = asset.cached_plugin_path(&cache_root);
        if plugin_path.is_file() && !self.overwrite {
            return Ok(ResolvedNativePlugin {
                path: plugin_path,
                source: NativePluginSource::Cache,
                asset,
            });
        }

        std::fs::create_dir_all(&cache_root).map_err(|err| {
            LibbunError::initialize(format!(
                "failed to create libbun cache `{}`: {err}",
                cache_root.display()
            ))
        })?;
        let stage_dir = cache_root.join(format!(
            ".{}.{}.install",
            asset.release_tag,
            std::process::id()
        ));
        if stage_dir.exists() {
            std::fs::remove_dir_all(&stage_dir).map_err(|err| {
                LibbunError::initialize(format!(
                    "failed to remove stale libbun install stage `{}`: {err}",
                    stage_dir.display()
                ))
            })?;
        }
        std::fs::create_dir_all(&stage_dir).map_err(|err| {
            LibbunError::initialize(format!(
                "failed to create libbun install stage `{}`: {err}",
                stage_dir.display()
            ))
        })?;

        let result = install_asset_into_stage(&asset, &stage_dir)
            .and_then(|_| replace_install_dir(&stage_dir, &install_dir, self.overwrite))
            .map(|_| ResolvedNativePlugin {
                path: asset.cached_plugin_path(&cache_root),
                source: NativePluginSource::Cache,
                asset: asset.clone(),
            });

        if result.is_err() {
            let _ = std::fs::remove_dir_all(&stage_dir);
        }
        result
    }
}

#[cfg(feature = "plugin-installer")]
impl Default for NativePluginInstall {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "plugin-installer")]
fn install_asset_into_stage(asset: &NativePluginAsset, stage_dir: &Path) -> LibbunResult<()> {
    let checksums_name = asset.checksums_asset_name();
    let checksums_path = stage_dir.join(&checksums_name);
    download_to(&asset.checksums_url(), &checksums_path)?;
    let checksums = std::fs::read_to_string(&checksums_path).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to read downloaded checksum file `{}`: {err}",
            checksums_path.display()
        ))
    })?;
    std::fs::copy(&checksums_path, stage_dir.join("checksums.txt")).map_err(|err| {
        LibbunError::initialize(format!("failed to install checksum metadata: {err}"))
    })?;

    let archive_path = stage_dir.join(&asset.asset_name);
    download_to(&asset.archive_url(), &archive_path)?;
    verify_file_checksum(&checksums, &asset.asset_name, &archive_path)?;
    unpack_tar_zstd(&archive_path, stage_dir)?;
    if !stage_dir.join(asset.plugin_filename).is_file() {
        return Err(LibbunError::initialize(format!(
            "plugin archive `{}` did not contain `{}`",
            asset.asset_name, asset.plugin_filename
        )));
    }
    std::fs::remove_file(&archive_path).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to remove downloaded archive `{}` after extraction: {err}",
            archive_path.display()
        ))
    })?;

    download_verified_release_text(
        &checksums,
        &asset.notice_asset_name(),
        &asset.notice_url(),
        &stage_dir.join("NOTICE.txt"),
    )?;
    download_verified_release_text(
        &checksums,
        &asset.source_asset_name(),
        &asset.source_url(),
        &stage_dir.join("SOURCE.txt"),
    )?;
    download_verified_release_text(
        &checksums,
        &asset.licenses_asset_name(),
        &asset.licenses_url(),
        &stage_dir.join("licenses.json"),
    )?;
    Ok(())
}

#[cfg(feature = "plugin-installer")]
fn download_verified_release_text(
    checksums: &str,
    asset_name: &str,
    url: &str,
    installed_path: &Path,
) -> LibbunResult<()> {
    let download_path = installed_path.with_file_name(asset_name);
    download_to(url, &download_path)?;
    verify_file_checksum(checksums, asset_name, &download_path)?;
    std::fs::rename(&download_path, installed_path).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to install release metadata `{}`: {err}",
            installed_path.display()
        ))
    })
}

#[cfg(feature = "plugin-installer")]
fn download_to(url: &str, path: &Path) -> LibbunResult<()> {
    let response = ureq::get(url)
        .call()
        .map_err(|err| LibbunError::initialize(format!("failed to download `{url}`: {err}")))?;
    let mut output = std::fs::File::create(path).map_err(|err| {
        LibbunError::initialize(format!("failed to create `{}`: {err}", path.display()))
    })?;
    std::io::copy(&mut response.into_reader(), &mut output).map_err(|err| {
        LibbunError::initialize(format!("failed to write `{}`: {err}", path.display()))
    })?;
    Ok(())
}

#[cfg(feature = "plugin-installer")]
fn unpack_tar_zstd(archive_path: &Path, destination: &Path) -> LibbunResult<()> {
    let archive = std::fs::File::open(archive_path).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to open `{}` for extraction: {err}",
            archive_path.display()
        ))
    })?;
    let decoder = zstd::stream::read::Decoder::new(archive).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to initialize zstd decoder for `{}`: {err}",
            archive_path.display()
        ))
    })?;
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(destination).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to extract `{}` into `{}`: {err}",
            archive_path.display(),
            destination.display()
        ))
    })
}

#[cfg(feature = "plugin-installer")]
fn replace_install_dir(stage_dir: &Path, install_dir: &Path, overwrite: bool) -> LibbunResult<()> {
    if install_dir.exists() {
        if overwrite {
            std::fs::remove_dir_all(install_dir).map_err(|err| {
                LibbunError::initialize(format!(
                    "failed to replace existing libbun cache `{}`: {err}",
                    install_dir.display()
                ))
            })?;
        } else {
            return Err(LibbunError::initialize(format!(
                "libbun plugin cache `{}` already exists",
                install_dir.display()
            )));
        }
    }
    if let Some(parent) = install_dir.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            LibbunError::initialize(format!(
                "failed to create libbun cache parent `{}`: {err}",
                parent.display()
            ))
        })?;
    }
    std::fs::rename(stage_dir, install_dir).map_err(|err| {
        LibbunError::initialize(format!(
            "failed to install libbun plugin cache `{}`: {err}",
            install_dir.display()
        ))
    })
}

fn release_asset_name(suffix: &str) -> String {
    format!("libbun-plugin-native-{RELEASE_TAG}-{suffix}")
}

fn plugin_filename_for_target(target: &str) -> &'static str {
    if target.ends_with("apple-darwin") {
        "liblibbun_plugin_native.dylib"
    } else {
        "liblibbun_plugin_native.so"
    }
}

fn missing_plugin_error(asset: &NativePluginAsset, cache_root: Option<&Path>) -> LibbunError {
    let cache_message = cache_root
        .map(|root| {
            format!(
                "expected cached plugin at `{}`",
                asset.cached_plugin_path(root).display()
            )
        })
        .unwrap_or_else(|| "no cache root is available from LIBBUN_HOME or HOME".to_string());
    LibbunError::initialize(format!(
        "libbun native plugin is not installed for {} {}: {}; set {LIBBUN_PLUGIN_PATH_ENV}, install release asset `{}` from {}, or run a host/libbun plugin installer",
        asset.release_tag,
        asset.target,
        cache_message,
        asset.asset_name,
        asset.archive_url()
    ))
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
