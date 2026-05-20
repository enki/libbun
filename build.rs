use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let commit = fs::read_to_string("BUN_SOURCE_COMMIT")
        .expect("BUN_SOURCE_COMMIT must exist")
        .trim()
        .to_string();
    println!("cargo:rustc-env=LIBBUN_BUN_SOURCE_COMMIT={commit}");
    println!("cargo:rerun-if-env-changed=LIBBUN_DOWNLOAD_PLUGIN");
    println!("cargo:rerun-if-env-changed=LIBBUN_PLUGIN_ARCHIVE");
    println!("cargo:rerun-if-env-changed=LIBBUN_PLUGIN_BUNDLE_DIR");
    println!("cargo:rerun-if-env-changed=LIBBUN_PLUGIN_PATH");
    println!("cargo:rustc-check-cfg=cfg(libbun_download_plugin)");

    if env::var_os("CARGO_FEATURE_DOWNLOAD_PLUGIN").is_some() {
        configure_downloaded_plugin();
    }
}

fn configure_downloaded_plugin() {
    let target = env::var("TARGET").expect("TARGET is set by cargo");
    let release_tag = format!(
        "v{}",
        env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is set by cargo")
    );
    let plugin_name = plugin_filename(&target).unwrap_or_else(|| {
        panic!("libbun download-plugin does not support Cargo target `{target}`")
    });

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo"));
    let install_dir = out_dir
        .join("libbun-plugin-native")
        .join(&release_tag)
        .join(&target);
    let plugin_path = install_dir.join(plugin_name);

    if let Some(path) = env::var_os("LIBBUN_PLUGIN_PATH") {
        let path = PathBuf::from(path);
        if path.is_file() {
            emit_plugin(&path, path.parent());
            return;
        }
        panic!(
            "LIBBUN_PLUGIN_PATH points to `{}`, but no plugin file exists there",
            path.display()
        );
    }

    if let Some(bundle_dir) = env::var_os("LIBBUN_PLUGIN_BUNDLE_DIR") {
        let bundle_dir = PathBuf::from(bundle_dir);
        let path = bundle_dir.join(plugin_name);
        if path.is_file() {
            emit_plugin(&path, Some(&bundle_dir));
            return;
        }
        panic!(
            "LIBBUN_PLUGIN_BUNDLE_DIR `{}` does not contain `{plugin_name}`",
            bundle_dir.display()
        );
    }

    if plugin_path.is_file() {
        emit_plugin(&plugin_path, Some(&install_dir));
        return;
    }

    if env::var("LIBBUN_DOWNLOAD_PLUGIN").ok().as_deref() == Some("0") {
        panic!(
            "libbun download-plugin is enabled, but LIBBUN_DOWNLOAD_PLUGIN=0 and no valid LIBBUN_PLUGIN_PATH or LIBBUN_PLUGIN_BUNDLE_DIR override was provided"
        );
    }

    let expected_checksum = checksum_for(&release_tag, &target).unwrap_or_else(|| {
        panic!(
            "libbun download-plugin has no committed checksum for {release_tag} {target}; publish native plugin assets and update src/plugin_checksums_table.in before publishing this crate"
        )
    });

    fs::create_dir_all(&install_dir).expect("create libbun plugin install dir");
    let archive_path = env::var_os("LIBBUN_PLUGIN_ARCHIVE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let archive_path = out_dir.join(asset_name(&release_tag, &target));
            download(&asset_url(&release_tag, &target), &archive_path);
            archive_path
        });
    verify_sha256(&archive_path, expected_checksum);
    extract_tar_zstd(&archive_path, &install_dir);
    install_release_metadata(&release_tag, &install_dir);
    if !plugin_path.is_file() {
        panic!(
            "libbun plugin archive `{}` did not extract `{}` into `{}`",
            archive_path.display(),
            plugin_name,
            install_dir.display()
        );
    }
    emit_plugin(&plugin_path, Some(&install_dir));
}

fn emit_plugin(plugin_path: &Path, bundle_dir: Option<&Path>) {
    println!("cargo:rustc-cfg=libbun_download_plugin");
    println!(
        "cargo:rustc-env=LIBBUN_BUILD_PLUGIN_PATH={}",
        plugin_path.display()
    );
    if let Some(bundle_dir) = bundle_dir {
        println!(
            "cargo:rustc-env=LIBBUN_BUILD_PLUGIN_DIR={}",
            bundle_dir.display()
        );
    }
    println!(
        "cargo:rustc-env=LIBBUN_BUILD_PLUGIN_TARGET={}",
        env::var("TARGET").unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=LIBBUN_BUILD_PLUGIN_VERSION=v{}",
        env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is set by cargo")
    );
}

fn plugin_filename(target: &str) -> Option<&'static str> {
    match target {
        "aarch64-apple-darwin" => Some("liblibbun_plugin_native.dylib"),
        "x86_64-unknown-linux-gnu" | "aarch64-unknown-linux-gnu" => {
            Some("liblibbun_plugin_native.so")
        }
        _ => None,
    }
}

fn asset_name(release_tag: &str, target: &str) -> String {
    format!("libbun-plugin-native-{release_tag}-{target}.tar.zst")
}

fn asset_url(release_tag: &str, target: &str) -> String {
    format!(
        "https://github.com/enki/libbun/releases/download/{release_tag}/{}",
        asset_name(release_tag, target)
    )
}

fn release_asset_url(release_tag: &str, asset_name: &str) -> String {
    format!("https://github.com/enki/libbun/releases/download/{release_tag}/{asset_name}")
}

fn release_asset_name(release_tag: &str, suffix: &str) -> String {
    format!("libbun-plugin-native-{release_tag}-{suffix}")
}

fn checksum_for(release_tag: &str, target: &str) -> Option<&'static str> {
    const CHECKSUMS: &[(&str, &str, &str)] = include!("src/plugin_checksums_table.in");
    CHECKSUMS
        .iter()
        .find(|(version, candidate, _)| *version == release_tag && *candidate == target)
        .map(|(_, _, checksum)| *checksum)
}

fn download(url: &str, path: &Path) {
    let status = Command::new("curl")
        .args([
            "--fail",
            "--location",
            "--show-error",
            "--silent",
            "--output",
        ])
        .arg(path)
        .arg(url)
        .status()
        .expect("failed to run curl for libbun plugin download");
    if !status.success() {
        panic!("failed to download libbun plugin asset from {url}");
    }
}

fn verify_sha256(path: &Path, expected: &str) {
    let output = if cfg!(target_os = "macos") {
        Command::new("shasum")
            .arg("-a")
            .arg("256")
            .arg(path)
            .output()
    } else {
        Command::new("sha256sum").arg(path).output()
    }
    .expect("failed to run sha256 tool for libbun plugin archive");
    if !output.status.success() {
        panic!(
            "failed to compute sha256 for libbun plugin archive `{}`",
            path.display()
        );
    }
    let stdout = String::from_utf8(output.stdout).expect("sha256 output is utf-8");
    let actual = stdout
        .split_whitespace()
        .next()
        .expect("sha256 output contains digest");
    if actual != expected {
        panic!(
            "checksum mismatch for libbun plugin archive `{}`: expected {expected}, got {actual}",
            path.display()
        );
    }
}

fn extract_tar_zstd(archive: &Path, destination: &Path) {
    let status = Command::new("tar")
        .arg("--zstd")
        .arg("-xf")
        .arg(archive)
        .arg("-C")
        .arg(destination)
        .status()
        .expect("failed to run tar for libbun plugin extraction");
    if !status.success() {
        panic!(
            "failed to extract libbun plugin archive `{}` into `{}`",
            archive.display(),
            destination.display()
        );
    }
}

fn install_release_metadata(release_tag: &str, install_dir: &Path) {
    let checksums_name = release_asset_name(release_tag, "checksums.txt");
    let checksums_path = install_dir.join("checksums.txt");
    download(
        &release_asset_url(release_tag, &checksums_name),
        &checksums_path,
    );
    let checksums = fs::read_to_string(&checksums_path).expect("read libbun release checksums");
    for (suffix, installed_name) in [
        ("NOTICE.txt", "NOTICE.txt"),
        ("SOURCE.txt", "SOURCE.txt"),
        ("licenses.json", "licenses.json"),
    ] {
        let asset_name = release_asset_name(release_tag, suffix);
        let path = install_dir.join(installed_name);
        download(&release_asset_url(release_tag, &asset_name), &path);
        let expected = checksum_from_file(&checksums, &asset_name).unwrap_or_else(|| {
            panic!("libbun release checksum file does not contain `{asset_name}`")
        });
        verify_sha256(&path, expected);
    }
}

fn checksum_from_file<'a>(checksums: &'a str, asset_name: &str) -> Option<&'a str> {
    checksums.lines().find_map(|line| {
        let (checksum, name) = line.split_once("  ")?;
        (name == asset_name).then_some(checksum)
    })
}
