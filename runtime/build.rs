use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_BUN_BUILD_DIR");

    if env::var("LIBBUN_NATIVE_LINK_BUN").as_deref() != Ok("1") {
        println!(
            "cargo:warning=building libbun-runtime-native without native Bun link inputs; set LIBBUN_NATIVE_LINK_BUN=1 for a runnable helper"
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

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-fuse-ld=lld");
    }

    for line in contents.lines() {
        let Some((kind, path)) = line.split_once('=') else {
            continue;
        };
        if kind == "archive" || kind == "static" {
            reject_debug_native_link_input(&manifest, path);
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
        println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1200000");
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
        link_compiler_rt_archive("libclang_rt.ubsan_standalone-aarch64.a");
        link_compiler_rt_archive("libclang_rt.ubsan_standalone-x86_64.a");
        link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-aarch64.a");
        link_compiler_rt_archive("libclang_rt.ubsan_standalone_cxx-x86_64.a");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=m");
    }
}

fn link_compiler_rt_archive(library: &str) {
    let Some(path) = find_compiler_rt(library) else {
        return;
    };
    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-arg={}", path.display());
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
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

fn default_manifest_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir
        .parent()
        .expect("runtime crate has repo parent");
    env::var_os("LIBBUN_NATIVE_BUN_BUILD_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                repo_root.join(path)
            }
        })
        .unwrap_or_else(|| repo_root.join("vendor/bun/build/release"))
        .join("libbun_native_link_manifest.txt")
}

fn reject_debug_native_link_input(manifest: &Path, path: &str) {
    if path.contains("/build/debug/")
        || path.contains("\\build\\debug\\")
        || path.contains("/bun-debug")
        || path.contains("\\bun-debug")
        || path.contains("-debug/")
        || path.contains("-debug\\")
    {
        panic!(
            "native Bun link manifest {} contains debug build input {}. Regenerate it from Bun's release profile with scripts/prepare-native-bun-link.sh.",
            manifest.display(),
            path
        );
    }
}
