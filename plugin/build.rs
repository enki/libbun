use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let linux_in_process = env::var_os("CARGO_FEATURE_LINUX_IN_PROCESS").is_some();
    if target_os == "linux" && !linux_in_process {
        println!("cargo:warning=building Linux libbun-plugin-native in helper-process mode");
        return;
    }

    if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
        println!(
            "cargo:warning=building libbun-plugin-native requires LIBBUN_NATIVE_LINK_BUN=1 for a linkable cdylib"
        );
        return;
    }

    let manifest = env::var_os("LIBBUN_NATIVE_LINK_MANIFEST")
        .map(PathBuf::from)
        .unwrap_or_else(default_manifest_path);
    let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
        panic!(
            "failed to read native Bun link manifest at {}: {err}. Run scripts/prepare-native-bun-link.sh first.",
            manifest.display()
        )
    });

    for line in contents.lines() {
        let Some((kind, path)) = line.split_once('=') else {
            continue;
        };
        if kind == "archive" || kind == "static" {
            if target_os == "macos" {
                println!("cargo:rustc-link-arg=-Wl,-force_load,{path}");
            } else {
                println!("cargo:rustc-link-arg=-Wl,--whole-archive");
                println!("cargo:rustc-link-arg={path}");
                println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
            }
        }
    }

    if target_os == "macos" {
        println!("cargo:rustc-link-arg=-fsanitize=null");
        println!("cargo:rustc-link-arg=-Wl,-ld_new");
        println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
        println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
        if let Some(ubsan) = find_compiler_rt("libclang_rt.ubsan_osx_dynamic.dylib") {
            let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
            println!("cargo:rustc-link-arg={}", ubsan.display());
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
        }
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=icucore");
        println!("cargo:rustc-link-lib=resolv");
    } else if target_os == "linux" {
        if let Some(ubsan) = find_toolchain_library("libubsan.so") {
            let ubsan_dir = ubsan.parent().expect("ubsan library has parent");
            println!("cargo:rustc-link-search=native={}", ubsan_dir.display());
            println!("cargo:rustc-link-lib=ubsan");
        }
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=m");
    }
}

fn find_compiler_rt(library: &str) -> Option<PathBuf> {
    let output = Command::new("clang")
        .arg(format!("-print-file-name={library}"))
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8(output.stdout).ok()?;
    let path = PathBuf::from(path.trim());
    path.exists().then_some(path)
}

fn find_toolchain_library(library: &str) -> Option<PathBuf> {
    for compiler in ["cc", "gcc", "clang"] {
        let Some(output) = Command::new(compiler)
            .arg(format!("-print-file-name={library}"))
            .output()
            .ok()
        else {
            continue;
        };
        if !output.status.success() {
            continue;
        }
        let path = String::from_utf8(output.stdout).ok()?;
        let path = PathBuf::from(path.trim());
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn default_manifest_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().expect("plugin crate has repo parent");
    env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                repo_root.join(path)
            }
        })
        .unwrap_or_else(|| repo_root.join("vendor/bun/build/debug"))
        .join("libbun_native_link_manifest.txt")
}
