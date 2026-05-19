#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="${1:-}"

usage() {
  cat >&2 <<'USAGE'
usage: scripts/preflight-native-plugin-release.sh <version>

Runs the local checks that mirror the native plugin GitHub Actions release job:

  - facade tests
  - dynamic-loading check
  - native Bun link preparation
  - native plugin build
  - Linux native helper build when running on Linux
  - dynamic plugin loading test
  - native adapter integration tests
  - release asset packaging smoke test

Environment:
  LIBBUN_RELEASE_SKIP_NATIVE_TEST=1   skip native adapter integration tests
  LIBBUN_RELEASE_OUT_DIR=<path>       override generated test asset directory
USAGE
}

if [[ "$version" == "-h" || "$version" == "--help" ]]; then
  usage
  exit 0
fi

if [[ -z "$version" ]]; then
  usage
  exit 2
fi

case "$version" in
  v*) release_version="$version" ;;
  *) release_version="v$version" ;;
esac

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 2
  fi
}

require cargo
require rustup
require bun
require zstd

cd "$repo_root"

native_build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/debug"}"
case "$native_build_dir" in
  /*) ;;
  *) native_build_dir="$repo_root/$native_build_dir" ;;
esac
export BUN_CODEGEN_DIR="${BUN_CODEGEN_DIR:-"$native_build_dir/codegen"}"

echo "==> preflight ${release_version}: facade tests"
cargo test

echo "==> preflight ${release_version}: dynamic-loading check"
cargo check --features dynamic-loading

echo "==> preflight ${release_version}: prepare native Bun link inputs"
scripts/prepare-native-bun-link.sh

case "$(uname -s)" in
  Linux)
    plugin_name="liblibbun_plugin_native.so"
    helper_name="libbun-runtime-native"
    runtime_mode="helper-process"
    ;;
  Darwin)
    plugin_name="liblibbun_plugin_native.dylib"
    helper_name=""
    runtime_mode="in-process"
    ;;
  *)
    echo "unsupported native plugin preflight OS: $(uname -s)" >&2
    exit 2
    ;;
esac

echo "==> preflight ${release_version}: build native plugin"
if [[ "$runtime_mode" == "helper-process" ]]; then
  cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
else
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
fi

plugin_path="$(find plugin/target/debug -maxdepth 1 -name "$plugin_name" -print -quit)"
if [[ -z "$plugin_path" || ! -f "$plugin_path" ]]; then
  echo "native plugin binary was not produced under plugin/target/debug: $plugin_name" >&2
  exit 1
fi

helper_path=""
if [[ "$runtime_mode" == "helper-process" ]]; then
  echo "==> preflight ${release_version}: build Linux native helper"
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path runtime/Cargo.toml
  helper_path="$(find runtime/target/debug -maxdepth 1 -name "$helper_name" -print -quit)"
  if [[ -z "$helper_path" || ! -f "$helper_path" ]]; then
    echo "native helper binary was not produced under runtime/target/debug: $helper_name" >&2
    exit 1
  fi
fi

echo "==> preflight ${release_version}: dynamic plugin loading test"
smoke_log="$(mktemp)"
LIBBUN_PLUGIN_PATH="$plugin_path" \
  LIBBUN_RUNTIME_NATIVE_PATH="$helper_path" \
  cargo test --features dynamic-loading dynamic_plugin_provider_flow -- --exact --nocapture 2>&1 | tee "$smoke_log"
if grep -q "mimalloc: error" "$smoke_log"; then
  echo "dynamic plugin smoke emitted a mimalloc diagnostic" >&2
  exit 1
fi

if [[ "${LIBBUN_RELEASE_SKIP_NATIVE_TEST:-0}" != "1" && "$runtime_mode" == "in-process" ]]; then
  echo "==> preflight ${release_version}: native adapter integration tests"
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 test --manifest-path native/Cargo.toml --features internal-adapter
else
  echo "==> preflight ${release_version}: skipping native adapter integration tests"
fi

out_dir="${LIBBUN_RELEASE_OUT_DIR:-"$repo_root/dist/native-plugin-preflight/${release_version}"}"
rm -rf "$out_dir"

echo "==> preflight ${release_version}: package release assets"
LIBBUN_NATIVE_HELPER_BINARY="$helper_path" scripts/package-native-plugin-release.sh "$release_version" "$plugin_path" "$out_dir"

echo "==> preflight ${release_version}: verify generated inventory"
python3 -m json.tool "$out_dir/libbun-plugin-native-${release_version}-licenses.json" >/dev/null

echo "==> preflight ${release_version}: release asset smoke test"
zstd -dc "$out_dir/libbun-plugin-native-${release_version}-source.tar.zst" | tar -tf - >/dev/null
binary_asset="$(find "$out_dir" -maxdepth 1 -type f -name "libbun-plugin-native-${release_version}-*.tar.zst" ! -name "*-source.tar.zst" -print -quit)"
if [[ -z "$binary_asset" ]]; then
  echo "native plugin binary archive was not generated" >&2
  exit 1
fi
zstd -dc "$binary_asset" | tar -tf - >/dev/null

echo "preflight ${release_version} passed"
echo "assets: $out_dir"
