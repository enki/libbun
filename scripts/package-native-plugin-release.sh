#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="${1:-${GITHUB_REF_NAME:-}}"
plugin_binary="${2:-}"
out_dir="${3:-"$repo_root/dist/native-plugin"}"
helper_binary="${LIBBUN_NATIVE_HELPER_BINARY:-${4:-}}"

if [[ -z "$version" ]]; then
  echo "usage: $0 <version> <plugin-binary> [out-dir]" >&2
  exit 2
fi

if [[ -z "$plugin_binary" || ! -f "$plugin_binary" ]]; then
  echo "plugin binary not found: ${plugin_binary:-<empty>}" >&2
  exit 2
fi

case "$version" in
  v*) release_version="$version" ;;
  *) release_version="v$version" ;;
esac

mkdir -p "$out_dir"

platform="$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$platform" in
  arm64-darwin) platform="aarch64-apple-darwin" ;;
  x86_64-darwin) platform="x86_64-apple-darwin" ;;
  aarch64-linux) platform="aarch64-unknown-linux-gnu" ;;
  x86_64-linux) platform="x86_64-unknown-linux-gnu" ;;
esac

binary_asset="$out_dir/libbun-plugin-native-${release_version}-${platform}.tar.zst"
source_asset="$out_dir/libbun-plugin-native-${release_version}-source.tar.zst"
notice_asset="$out_dir/libbun-plugin-native-${release_version}-NOTICE.txt"
inventory_asset="$out_dir/libbun-plugin-native-${release_version}-licenses.json"
source_txt_asset="$out_dir/libbun-plugin-native-${release_version}-SOURCE.txt"
checksums_asset="$out_dir/libbun-plugin-native-${release_version}-checksums.txt"

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 2
  fi
}

require git
require tar
require zstd
require python3

sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  else
    sha256sum "$1" | awk '{print $1}'
  fi
}

bun_commit="$(tr -d '[:space:]' < "$repo_root/BUN_SOURCE_COMMIT")"
git_commit="$(git -C "$repo_root" rev-parse HEAD)"
plugin_checksum="$(sha256 "$plugin_binary")"
helper_checksum=""
build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/debug"}"
case "$build_dir" in
  /*) ;;
  *) build_dir="$repo_root/$build_dir" ;;
esac
manifest="${LIBBUN_NATIVE_LINK_MANIFEST:-"$build_dir/libbun_native_link_manifest.txt"}"

if [[ ! -f "$manifest" ]]; then
  echo "native link manifest not found: $manifest" >&2
  exit 2
fi

runtime_mode="in-process"
case "$platform" in
  *-linux-gnu)
    runtime_mode="helper-process"
    if [[ -z "$helper_binary" || ! -f "$helper_binary" ]]; then
      echo "Linux native plugin packages require LIBBUN_NATIVE_HELPER_BINARY or a fourth helper-binary argument" >&2
      exit 2
    fi
    helper_checksum="$(sha256 "$helper_binary")"
    ;;
esac

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

binary_stage="$tmpdir/binary"
source_stage="$tmpdir/source/libbun-${release_version}"
mkdir -p "$binary_stage" "$source_stage"

cp "$plugin_binary" "$binary_stage/"
if [[ -n "$helper_binary" ]]; then
  cp "$helper_binary" "$binary_stage/"
fi
python3 - "$binary_stage/libbun-native-bundle.json" "$platform" "$runtime_mode" "$release_version" "$git_commit" "$bun_commit" "$(basename "$plugin_binary")" "$plugin_checksum" "${helper_binary:+$(basename "$helper_binary")}" "$helper_checksum" <<'PY'
import json
import pathlib
import sys

(
    out,
    target,
    runtime_mode,
    release_version,
    git_commit,
    bun_commit,
    plugin_name,
    plugin_checksum,
    helper_name,
    helper_checksum,
) = sys.argv[1:11]

bundle = {
    "target": target,
    "runtimeMode": runtime_mode,
    "pluginAbiVersion": 1,
    "helperProtocolVersion": 1 if runtime_mode == "helper-process" else None,
    "libbunReleaseVersion": release_version,
    "libbunGitCommit": git_commit,
    "bunSourceCommit": bun_commit,
    "plugin": {
        "filename": plugin_name,
        "sha256": plugin_checksum,
    },
    "helper": None,
    "sourceArchive": f"libbun-plugin-native-{release_version}-source.tar.zst",
    "noticeFile": f"libbun-plugin-native-{release_version}-NOTICE.txt",
    "licenseInventory": f"libbun-plugin-native-{release_version}-licenses.json",
    "sourceInstructions": f"libbun-plugin-native-{release_version}-SOURCE.txt",
    "checksums": f"libbun-plugin-native-{release_version}-checksums.txt",
}

if helper_name:
    bundle["helper"] = {
        "filename": helper_name,
        "sha256": helper_checksum,
        "overrideEnvironment": "LIBBUN_RUNTIME_NATIVE_PATH",
    }

pathlib.Path(out).write_text(json.dumps(bundle, indent=2, sort_keys=True) + "\n")
PY
tar -C "$binary_stage" -cf - . | zstd -19 -q -o "$binary_asset"

git -C "$repo_root" archive --format=tar --prefix="libbun-${release_version}/" HEAD |
  tar -C "$tmpdir/source" -xf -
mkdir -p "$source_stage/release"
cp "$manifest" "$source_stage/release/libbun-native-link-manifest.txt"
tar -C "$tmpdir/source" -cf - "libbun-${release_version}" | zstd -19 -q -o "$source_asset"

cat > "$notice_asset" <<NOTICE
libbun native plugin ${release_version}

This binary is produced by the libbun GitHub Actions release workflow from:

  libbun commit: ${git_commit}
  Bun source commit: ${bun_commit}
  native link manifest: release/libbun-native-link-manifest.txt

The matching source archive, license inventory, SOURCE.txt, and checksums are
published as companion assets on the same GitHub Release as the plugin binary.

The native plugin includes Bun native runtime integration. Bun's license notice
states that Bun itself is MIT-licensed and that its JavaScriptCore/WebKit
runtime path carries LGPL requirements. See vendor/bun/LICENSE.md in the source
archive for the upstream notice text and dependency list.

Users may replace this plugin with an interface-compatible modified build by
building the source archive and configuring the host application to load that
replacement plugin path.

On Linux, the plugin asset is a native runtime bundle. The host still loads the
plugin dynamically through LIBBUN_PLUGIN_PATH; the plugin then starts the
replaceable libbun-runtime-native helper described by libbun-native-bundle.json.
NOTICE

python3 - "$repo_root" "$manifest" "$inventory_asset" "$release_version" "$git_commit" "$bun_commit" "$plugin_binary" "$plugin_checksum" "$platform" "$runtime_mode" "$helper_binary" "$helper_checksum" <<'PY'
import json
import pathlib
import sys

repo = pathlib.Path(sys.argv[1])
manifest = pathlib.Path(sys.argv[2])
out = pathlib.Path(sys.argv[3])
release_version, git_commit, bun_commit, plugin_binary, checksum, platform, runtime_mode, helper_binary, helper_checksum = sys.argv[4:13]

manifest_entries = []
for line in manifest.read_text().splitlines():
    if not line.strip() or "=" not in line:
        continue
    kind, path = line.split("=", 1)
    manifest_entries.append({"kind": kind, "path": path})

inventory = {
    "name": "libbun native plugin",
    "version": release_version,
    "gitCommit": git_commit,
    "bunSourceCommit": bun_commit,
    "pluginAbiVersion": 1,
    "helperProtocolVersion": 1 if runtime_mode == "helper-process" else None,
    "platform": platform,
    "runtimeMode": runtime_mode,
    "pluginBinary": pathlib.Path(plugin_binary).name,
    "pluginSha256": checksum,
    "helperBinary": pathlib.Path(helper_binary).name if helper_binary else None,
    "helperSha256": helper_checksum or None,
    "bundleMetadata": "libbun-native-bundle.json",
    "sourceArchive": f"libbun-plugin-native-{release_version}-source.tar.zst",
    "noticeFile": f"libbun-plugin-native-{release_version}-NOTICE.txt",
    "sourceInstructions": f"libbun-plugin-native-{release_version}-SOURCE.txt",
    "checksums": f"libbun-plugin-native-{release_version}-checksums.txt",
    "nativeLinkManifest": manifest_entries,
    "licenseInputs": [
        {
            "name": "libbun",
            "license": "Apache-2.0",
            "source": "source archive root",
        },
        {
            "name": "Bun",
            "license": "MIT with separately documented native dependency notices",
            "source": "vendor/bun in the source archive",
            "licenseFile": "vendor/bun/LICENSE.md",
        },
        {
            "name": "JavaScriptCore/WebKit",
            "license": "LGPL family as documented by Bun/WebKit source notices",
            "source": "vendor/bun/LICENSE.md links the patched WebKit source used by Bun",
        },
    ],
    "redistributionNotes": {
        "pluginBinary": "<same GitHub Release plugin asset URL>",
        "correspondingSource": "<same GitHub Release source archive URL>",
        "noticesAndLicenses": "<same GitHub Release NOTICE and licenses.json URLs>",
        "checksums": "<same GitHub Release checksums URL>",
        "replacement": "Configure the host application to load an interface-compatible replacement plugin.",
        "linuxHelperReplacement": "On Linux, replace the helper next to the plugin or set LIBBUN_RUNTIME_NATIVE_PATH.",
    },
}

out.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
PY

expected_inventory="$repo_root/release/native-license-inventory.expected.json"
if [[ -f "$expected_inventory" ]] && ! diff -u "$expected_inventory" "$inventory_asset"; then
  echo "generated native license inventory differs from $expected_inventory" >&2
  exit 1
fi

cat > "$source_txt_asset" <<SOURCE
libbun native plugin ${release_version} source instructions

Binary platform: ${platform}
Runtime mode: ${runtime_mode}
libbun commit: ${git_commit}
Bun source commit: ${bun_commit}
Plugin SHA-256: ${plugin_checksum}
${helper_checksum:+Helper SHA-256: ${helper_checksum}}

The corresponding source for this plugin binary is:

  libbun-plugin-native-${release_version}-source.tar.zst

That archive is attached to the same GitHub Release as the plugin binary. It
contains the libbun source tree at the release commit and the native link
manifest used by the plugin build at:

  release/libbun-native-link-manifest.txt

Build outline:

  scripts/prepare-native-bun-link.sh
  cargo +nightly-2026-05-06 build --manifest-path plugin/Cargo.toml

For macOS in-process plugin builds, set LIBBUN_NATIVE_LINK_BUN=1 when building
plugin/Cargo.toml.

For Linux helper-backed bundle builds, build the plugin without
LIBBUN_NATIVE_LINK_BUN and build the helper executable with:

  LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --manifest-path runtime/Cargo.toml

Host applications must keep the plugin replaceable. Users can build a modified
compatible plugin from the corresponding source and configure the host to load
that replacement path. On Linux, users can also replace the helper executable
next to the plugin or set LIBBUN_RUNTIME_NATIVE_PATH.
SOURCE

(
  cd "$out_dir"
  for asset in \
    "$(basename "$binary_asset")" \
    "$(basename "$source_asset")" \
    "$(basename "$notice_asset")" \
    "$(basename "$inventory_asset")" \
    "$(basename "$source_txt_asset")"; do
    printf '%s  %s\n' "$(sha256 "$asset")" "$asset"
  done
) > "$checksums_asset"

echo "created native plugin release assets in $out_dir"
