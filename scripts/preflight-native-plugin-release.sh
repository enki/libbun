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
  - Linux PIC in-process plugin build when running on Linux
  - dynamic plugin loading test
  - native adapter integration tests
  - release asset packaging smoke test

Environment:
  LIBBUN_RELEASE_SKIP_NATIVE_TEST=1   skip native adapter integration tests
  LIBBUN_RELEASE_OUT_DIR=<path>       override generated test asset directory
  LIBBUN_NATIVE_RUNTIME_MODE=<mode>   override runtime mode; default is in-process
  LIBBUN_ENABLE_LEGACY_LINUX_HELPER=1 allow quarantined helper-process diagnostics
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

cargo_debug_artifact_dir() {
  if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
    case "$CARGO_TARGET_DIR" in
      /*) printf '%s/debug\n' "$CARGO_TARGET_DIR" ;;
      *) printf '%s/%s/debug\n' "$repo_root" "$CARGO_TARGET_DIR" ;;
    esac
    return
  fi

  printf '%s\n' "$1"
}

validate_linux_plugin_exports() {
  local plugin_path="$1"

  if [[ "$(uname -s)" != "Linux" ]]; then
    return
  fi

  require nm

  local unexpected=0
  while read -r _address _kind symbol _rest; do
    if [[ -z "${symbol:-}" ]]; then
      continue
    fi
    symbol="${symbol%%@@*}"
    symbol="${symbol%%@*}"
    case "$symbol" in
      libbun_plugin_abi_version|\
      libbun_plugin_buffer_free|\
      libbun_plugin_runtime_create|\
      libbun_plugin_runtime_destroy|\
      libbun_plugin_runtime_load_module|\
      libbun_plugin_runtime_call_export|\
      libbun_plugin_runtime_pump_event_loop|\
      libbun_plugin_runtime_resolve_async|\
      libbun_plugin_runtime_drain_output|\
      libbun_plugin_runtime_shutdown)
        ;;
      *)
        echo "Linux native plugin exports non-ABI symbol: $symbol" >&2
        unexpected=1
        ;;
    esac
  done < <(nm -D --defined-only "$plugin_path")

  if [[ "$unexpected" != "0" ]]; then
    echo "Linux native plugin must export only the libbun plugin C ABI" >&2
    exit 1
  fi
}

crate_version="$(python3 - <<'PY'
import pathlib
import tomllib

manifest = tomllib.loads(pathlib.Path("Cargo.toml").read_text())
print(manifest["package"]["version"])
PY
)"
if [[ "v${crate_version}" != "$release_version" ]]; then
  echo "Cargo.toml version is ${crate_version}, release tag is ${release_version}; refusing divergent preflight" >&2
  exit 1
fi

native_build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/release"}"
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
	    runtime_mode="${LIBBUN_NATIVE_RUNTIME_MODE:-in-process}"
    ;;
  Darwin)
    plugin_name="liblibbun_plugin_native.dylib"
    helper_name=""
    runtime_mode="${LIBBUN_NATIVE_RUNTIME_MODE:-in-process}"
    ;;
  *)
    echo "unsupported native plugin preflight OS: $(uname -s)" >&2
    exit 2
    ;;
esac

case "$runtime_mode" in
	  in-process|helper-process) ;;
  *)
    echo "unsupported native plugin preflight runtime mode: $runtime_mode" >&2
    exit 2
    ;;
esac

if [[ "$(uname -s)" == "Linux" && "$runtime_mode" == "helper-process" && "${LIBBUN_ENABLE_LEGACY_LINUX_HELPER:-0}" != "1" ]]; then
  echo "Linux helper-process preflight is quarantined; set LIBBUN_ENABLE_LEGACY_LINUX_HELPER=1 only for legacy diagnostics" >&2
  exit 2
fi

if [[ "$(uname -s)" == "Linux" && "$runtime_mode" == "in-process" ]]; then
  echo "==> preflight ${release_version}: fetch Linux PIC WebKit inputs"
  base_manifest="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/release"}/libbun_native_link_manifest.txt"
  pic_manifest="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/release"}/libbun_native_link_manifest.pic.txt"
  scripts/fetch-webkit-pic-artifact.sh --manifest "$base_manifest" --out "$pic_manifest"
  export LIBBUN_NATIVE_LINK_MANIFEST="$pic_manifest"
  scripts/inspect-linux-native-relocations.sh "$LIBBUN_NATIVE_LINK_MANIFEST"
fi

echo "==> preflight ${release_version}: build native plugin"
if [[ "$runtime_mode" == "helper-process" ]]; then
  cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml --features legacy-linux-helper-process
elif [[ "$(uname -s)" == "Linux" ]]; then
  LIBBUN_NATIVE_LINK_BUN=1 RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml --features linux-in-process
else
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml
fi

plugin_debug_dir="$(cargo_debug_artifact_dir "$repo_root/plugin/target/debug")"
plugin_path="$(find "$plugin_debug_dir" -maxdepth 1 -name "$plugin_name" -print -quit)"
if [[ -z "$plugin_path" || ! -f "$plugin_path" ]]; then
  echo "native plugin binary was not produced under $plugin_debug_dir: $plugin_name" >&2
  exit 1
fi
validate_linux_plugin_exports "$plugin_path"

helper_path=""
if [[ "$runtime_mode" == "helper-process" ]]; then
  echo "==> preflight ${release_version}: build Linux native helper"
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path runtime/Cargo.toml
  runtime_debug_dir="$(cargo_debug_artifact_dir "$repo_root/runtime/target/debug")"
  helper_path="$(find "$runtime_debug_dir" -maxdepth 1 -name "$helper_name" -print -quit)"
  if [[ -z "$helper_path" || ! -f "$helper_path" ]]; then
    echo "native helper binary was not produced under $runtime_debug_dir: $helper_name" >&2
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

echo "==> preflight ${release_version}: dynamic plugin conformance test"
conformance_log="$(mktemp)"
LIBBUN_PLUGIN_PATH="$plugin_path" \
  LIBBUN_RUNTIME_NATIVE_PATH="$helper_path" \
  cargo test --features dynamic-loading dynamic_plugin_facade_conformance -- --exact --nocapture 2>&1 | tee "$conformance_log"
if grep -q "mimalloc: error" "$conformance_log"; then
  echo "dynamic plugin conformance emitted a mimalloc diagnostic" >&2
  exit 1
fi

if [[ "${LIBBUN_RELEASE_SKIP_NATIVE_TEST:-0}" != "1" && "$runtime_mode" == "in-process" ]]; then
  echo "==> preflight ${release_version}: native adapter integration tests"
  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 test --manifest-path native/Cargo.toml --features internal-adapter -- --test-threads=1
else
  echo "==> preflight ${release_version}: skipping native adapter integration tests"
fi

out_dir="${LIBBUN_RELEASE_OUT_DIR:-"$repo_root/dist/native-plugin-preflight/${release_version}"}"
rm -rf "$out_dir"

echo "==> preflight ${release_version}: package release assets"
LIBBUN_NATIVE_RUNTIME_MODE="$runtime_mode" LIBBUN_NATIVE_HELPER_BINARY="$helper_path" scripts/package-native-plugin-release.sh "$release_version" "$plugin_path" "$out_dir"

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
