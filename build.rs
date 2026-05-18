use std::fs;

fn main() {
    let commit = fs::read_to_string("BUN_SOURCE_COMMIT")
        .expect("BUN_SOURCE_COMMIT must exist")
        .trim()
        .to_string();
    println!("cargo:rustc-env=LIBBUN_BUN_SOURCE_COMMIT={commit}");
}
