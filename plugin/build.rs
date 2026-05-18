use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_BUN");
    println!("cargo:rerun-if-env-changed=LIBBUN_NATIVE_LINK_MANIFEST");

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
        match kind {
            "archive" | "static" => println!("cargo:rustc-link-arg=-Wl,-force_load,{path}"),
            _ => {}
        }
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "macos" {
        println!("cargo:rustc-link-arg=-fsanitize=null");
        println!("cargo:rustc-link-arg=-Wl,-ld_new");
        println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
        println!("cargo:rustc-link-arg=-mmacosx-version-min=26");
        let ubsan = PathBuf::from(
            "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/21/lib/darwin/libclang_rt.ubsan_osx_dynamic.dylib",
        );
        if ubsan.exists() {
            let ubsan_dir = ubsan.parent().expect("ubsan dylib has parent");
            println!("cargo:rustc-link-arg={}", ubsan.display());
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ubsan_dir.display());
        }
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=icucore");
        println!("cargo:rustc-link-lib=resolv");
    }
}

fn default_manifest_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    manifest_dir
        .parent()
        .expect("plugin crate has repo parent")
        .join("vendor/bun/build/debug/libbun_native_link_manifest.txt")
}
