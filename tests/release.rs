use libbun::release::{
    BundledNativePluginResolver, LIBBUN_PLUGIN_PATH_ENV, NativePluginResolver, NativePluginSource,
    RELEASE_TAG, current_native_plugin_asset, expected_checksum, verify_file_checksum,
};

#[cfg(feature = "plugin-installer")]
use libbun::release::NativePluginInstall;

#[test]
fn current_asset_names_match_release_contract() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    assert_eq!(asset.release_tag, RELEASE_TAG);
    assert_eq!(asset.repository, "enki/libbun");
    assert_eq!(
        asset.asset_name,
        format!(
            "libbun-plugin-native-{}-{}.tar.zst",
            RELEASE_TAG, asset.target
        )
    );
    assert!(asset.archive_url().ends_with(&asset.asset_name));
    assert_eq!(
        asset.checksums_asset_name(),
        format!("libbun-plugin-native-{}-checksums.txt", RELEASE_TAG)
    );
    if asset.target.ends_with("apple-darwin") {
        assert_eq!(asset.plugin_filename, "liblibbun_plugin_native.dylib");
    } else {
        assert_eq!(asset.plugin_filename, "liblibbun_plugin_native.so");
    }
}

#[test]
fn resolver_prefers_explicit_plugin_path() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");
    let plugin_path = tempdir.path().join(asset.plugin_filename);
    std::fs::write(&plugin_path, b"not a real dynamic library").expect("write plugin placeholder");
    let cached_dir = asset.cache_dir(tempdir.path().join("cache"));
    std::fs::create_dir_all(&cached_dir).expect("create cache dir");
    std::fs::write(cached_dir.join(asset.plugin_filename), b"cached").expect("write cache");

    let resolved = NativePluginResolver::new()
        .with_plugin_path(&plugin_path)
        .with_cache_root(tempdir.path().join("cache"))
        .resolve()
        .expect("explicit plugin path resolves");

    assert_eq!(resolved.path, plugin_path);
    assert_eq!(resolved.source, NativePluginSource::Environment);
}

#[test]
fn resolver_uses_standard_release_cache() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");
    let cached_plugin = asset.cached_plugin_path(tempdir.path());
    std::fs::create_dir_all(cached_plugin.parent().expect("plugin parent")).expect("cache dir");
    std::fs::write(&cached_plugin, b"not a real dynamic library").expect("write plugin");

    let resolved = NativePluginResolver::new()
        .with_cache_root(tempdir.path())
        .with_build_time_plugin(false)
        .resolve()
        .expect("cached plugin resolves");

    assert_eq!(resolved.path, cached_plugin);
    assert_eq!(resolved.source, NativePluginSource::Cache);
}

#[test]
fn bundled_resolver_uses_plugin_next_to_host_binary() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");
    let host_binary = tempdir.path().join("ss");
    let plugin_path = tempdir.path().join(asset.plugin_filename);
    std::fs::write(&host_binary, b"host").expect("write host placeholder");
    std::fs::write(&plugin_path, b"not a real dynamic library").expect("write plugin placeholder");

    let resolved = BundledNativePluginResolver::new()
        .with_host_binary_path(&host_binary)
        .resolve()
        .expect("bundled plugin resolves");

    assert_eq!(resolved.path, plugin_path);
    assert_eq!(resolved.source, NativePluginSource::Bundled);
}

#[test]
fn bundled_resolver_does_not_consult_cache_root() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");
    let cache_root = tempdir.path().join("cache");
    let cached_plugin = asset.cached_plugin_path(&cache_root);
    std::fs::create_dir_all(cached_plugin.parent().expect("plugin parent")).expect("cache dir");
    std::fs::write(&cached_plugin, b"cached plugin").expect("write cached plugin");
    let host_binary = tempdir.path().join("bin").join("ss");
    std::fs::create_dir_all(host_binary.parent().expect("host binary parent"))
        .expect("host bin dir");
    std::fs::write(&host_binary, b"host").expect("write host placeholder");

    let error = BundledNativePluginResolver::new()
        .with_host_binary_path(&host_binary)
        .resolve()
        .expect_err("bundled resolver must not use cache");

    let message = error.to_string();
    assert!(message.contains("bundled libbun plugin"), "{message}");
    assert!(
        message.contains(host_binary.parent().unwrap().to_str().unwrap()),
        "{message}"
    );
}

#[cfg(libbun_download_plugin)]
#[test]
fn resolver_uses_build_time_plugin_after_explicit_path() {
    let build_path =
        libbun::release::build_time_plugin_path().expect("download-plugin emitted plugin path");

    let resolved = NativePluginResolver::new()
        .resolve()
        .expect("build-time plugin resolves");

    assert_eq!(resolved.path, build_path);
    assert_eq!(resolved.source, NativePluginSource::BuildTime);
}

#[test]
fn resolver_missing_plugin_error_is_actionable() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");
    let error = NativePluginResolver::new()
        .with_cache_root(tempdir.path())
        .with_build_time_plugin(false)
        .resolve()
        .expect_err("missing plugin should fail");
    let message = error.to_string();

    assert!(message.contains(LIBBUN_PLUGIN_PATH_ENV), "{message}");
    assert!(message.contains(&asset.asset_name), "{message}");
    assert!(message.contains(&asset.archive_url()), "{message}");
    assert!(message.contains(&asset.target), "{message}");
}

#[test]
fn checksum_helpers_parse_and_verify_release_format() {
    let tempdir = tempfile::tempdir().expect("tempdir");
    let asset_path = tempdir.path().join("asset.txt");
    std::fs::write(&asset_path, b"hello").expect("write asset");
    let checksums = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  asset.txt\n";

    assert_eq!(
        expected_checksum(checksums, "asset.txt").expect("checksum entry"),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    verify_file_checksum(checksums, "asset.txt", &asset_path).expect("checksum verifies");
}

#[cfg(feature = "plugin-installer")]
#[test]
#[ignore = "downloads and extracts the current GitHub release asset"]
fn installer_downloads_current_release_asset() {
    let asset = current_native_plugin_asset().expect("current target is supported");
    let tempdir = tempfile::tempdir().expect("tempdir");

    let resolved = NativePluginInstall::new()
        .with_cache_root(tempdir.path())
        .install()
        .expect("release asset installs");

    assert_eq!(resolved.source, NativePluginSource::Cache);
    assert_eq!(resolved.path, asset.cached_plugin_path(tempdir.path()));
    assert!(resolved.path.is_file());
    assert!(
        asset
            .cache_dir(tempdir.path())
            .join("checksums.txt")
            .is_file()
    );
    assert!(asset.cache_dir(tempdir.path()).join("SOURCE.txt").is_file());
    assert!(asset.cache_dir(tempdir.path()).join("NOTICE.txt").is_file());
    assert!(
        asset
            .cache_dir(tempdir.path())
            .join("licenses.json")
            .is_file()
    );
}
