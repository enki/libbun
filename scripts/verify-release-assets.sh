#!/usr/bin/env bash
set -euo pipefail

version=""
asset_dir=""
repo="${LIBBUN_RELEASE_REPO:-enki/libbun}"
targets=()
tmpdir=""

cleanup() {
  if [[ -n "$tmpdir" ]]; then
    rm -rf "$tmpdir"
  fi
}
trap cleanup EXIT

usage() {
  cat >&2 <<'USAGE'
usage: scripts/verify-release-assets.sh --version <vX.Y.Z> [--dir <asset-dir> | --repo <owner/repo>] [--target <triple>...]

Verifies that a libbun native plugin release has the binary and compliance
assets expected for currently supported plugin targets. With --dir, checks
local files. Without --dir, reads GitHub Release assets through gh.

Default targets:
  aarch64-apple-darwin
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      version="${2:-}"
      shift 2
      ;;
    --dir)
      asset_dir="${2:-}"
      shift 2
      ;;
    --repo)
      repo="${2:-}"
      shift 2
      ;;
    --target)
      targets+=("${2:-}")
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$version" ]]; then
  usage
  exit 2
fi

case "$version" in
  v*) release_version="$version" ;;
  *) release_version="v$version" ;;
esac

if [[ ${#targets[@]} -eq 0 ]]; then
  targets=(aarch64-apple-darwin x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu)
fi

assets=()
if [[ -n "$asset_dir" ]]; then
  if [[ ! -d "$asset_dir" ]]; then
    echo "asset directory not found: $asset_dir" >&2
    exit 2
  fi
  while IFS= read -r asset; do
    assets+=("$(basename "$asset")")
  done < <(find "$asset_dir" -maxdepth 1 -type f -print | sort)
else
  if ! command -v gh >/dev/null 2>&1; then
    echo "missing required command for GitHub release verification: gh" >&2
    exit 2
  fi
  tmpdir="$(mktemp -d)"
  asset_dir="$tmpdir/assets"
  mkdir -p "$asset_dir"
  gh release download "$release_version" --repo "$repo" --dir "$asset_dir" --clobber
  while IFS= read -r asset; do
    assets+=("$(basename "$asset")")
  done < <(find "$asset_dir" -maxdepth 1 -type f -print | sort)
fi

has_asset() {
  local expected="$1"
  local asset
  for asset in "${assets[@]}"; do
    if [[ "$asset" == "$expected" ]]; then
      return 0
    fi
  done
  return 1
}

missing=()
for target in "${targets[@]}"; do
  expected="libbun-plugin-native-${release_version}-${target}.tar.zst"
  if ! has_asset "$expected"; then
    missing+=("$expected")
  fi
done

for suffix in source.tar.zst NOTICE.txt licenses.json SOURCE.txt checksums.txt; do
  expected="libbun-plugin-native-${release_version}-${suffix}"
  if ! has_asset "$expected"; then
    missing+=("$expected")
  fi
done

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "missing required release assets for ${release_version}:" >&2
  printf '  %s\n' "${missing[@]}" >&2
  exit 1
fi

if [[ -n "$asset_dir" ]]; then
  checksums="$asset_dir/libbun-plugin-native-${release_version}-checksums.txt"
  for asset in "${assets[@]}"; do
    if [[ "$asset" == "libbun-plugin-native-${release_version}-checksums.txt" ]]; then
      continue
    fi
    if ! grep -F "  $asset" "$checksums" >/dev/null; then
      echo "checksum file does not list $asset" >&2
      exit 1
    fi
  done

  if ! command -v zstd >/dev/null 2>&1; then
    echo "missing required command for release tarball verification: zstd" >&2
    exit 2
  fi
  if ! command -v python3 >/dev/null 2>&1; then
    echo "missing required command for bundle metadata verification: python3" >&2
    exit 2
  fi

  for target in "${targets[@]}"; do
    archive="$asset_dir/libbun-plugin-native-${release_version}-${target}.tar.zst"
    extract_dir="$(mktemp -d)"
    zstd -dc "$archive" | tar -C "$extract_dir" -xf -
    bundle="$extract_dir/libbun-native-bundle.json"
    if [[ ! -f "$bundle" ]]; then
      echo "$archive does not contain libbun-native-bundle.json" >&2
      exit 1
    fi
    python3 - "$bundle" "$target" <<'PY'
import json
import pathlib
import sys

bundle = pathlib.Path(sys.argv[1])
expected_target = sys.argv[2]
root = bundle.parent
data = json.loads(bundle.read_text())

target = data.get("target")
if target != expected_target:
    raise SystemExit(f"bundle target {target!r} did not match {expected_target!r}")

plugin = data.get("plugin") or {}
plugin_name = plugin.get("filename")
if not plugin_name or not (root / plugin_name).is_file():
    raise SystemExit(f"bundle plugin file is missing: {plugin_name!r}")

runtime_mode = data.get("runtimeMode")
helper = data.get("helper")
if runtime_mode == "in-process":
    if helper is not None:
        raise SystemExit("in-process bundle unexpectedly declares a helper")
elif runtime_mode == "helper-process":
    helper_name = (helper or {}).get("filename")
    if not helper_name or not (root / helper_name).is_file():
        raise SystemExit(f"helper-process bundle helper file is missing: {helper_name!r}")
else:
    raise SystemExit(f"unsupported runtimeMode: {runtime_mode!r}")
PY
    rm -rf "$extract_dir"
  done
fi

echo "release assets verified for ${release_version}"
