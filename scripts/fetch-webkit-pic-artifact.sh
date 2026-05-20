#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$repo_root/vendor/bun/build/release"}"
case "$build_dir" in
  /*) ;;
  *) build_dir="$repo_root/$build_dir" ;;
esac

repo="${LIBBUN_WEBKIT_PIC_REPO:-enki/WebKit}"
tag="${LIBBUN_WEBKIT_PIC_TAG:-libbun-webkit-pic-release-5488984d-20260520}"
target="${LIBBUN_WEBKIT_PIC_TARGET:-}"
base_manifest="${LIBBUN_NATIVE_BASE_LINK_MANIFEST:-"$build_dir/libbun_native_link_manifest.txt"}"
out_manifest="${LIBBUN_WEBKIT_PIC_LINK_MANIFEST:-"$build_dir/libbun_native_link_manifest.pic.txt"}"
metadata_out="${LIBBUN_WEBKIT_PIC_METADATA:-"$build_dir/libbun_webkit_pic_artifact.json"}"

usage() {
  cat >&2 <<'USAGE'
usage: scripts/fetch-webkit-pic-artifact.sh [--target <triple>] [--manifest <path>] [--out <path>]

Downloads a pinned public PIC WebKit artifact release, verifies its checksum,
extracts it, and rewrites a libbun native link manifest so WebKit/JSC/WTF static
library entries point at the PIC artifacts.

Environment:
  LIBBUN_WEBKIT_PIC_REPO=<owner/repo>       default: enki/WebKit
  LIBBUN_WEBKIT_PIC_TAG=<release-tag>      default: libbun-webkit-pic-release-5488984d-20260520
  LIBBUN_WEBKIT_PIC_TARGET=<triple>        default: host Linux target
  LIBBUN_NATIVE_BASE_LINK_MANIFEST=<path>  default: <build-dir>/libbun_native_link_manifest.txt
  LIBBUN_WEBKIT_PIC_LINK_MANIFEST=<path>   default: <build-dir>/libbun_native_link_manifest.pic.txt
  LIBBUN_WEBKIT_PIC_METADATA=<path>        default: <build-dir>/libbun_webkit_pic_artifact.json

Requires curl, gzip, tar, and shasum or sha256sum.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      target="${2:-}"
      shift 2
      ;;
    --manifest)
      base_manifest="${2:-}"
      shift 2
      ;;
    --out)
      out_manifest="${2:-}"
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

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 2
  fi
}

require curl
require gzip
require tar

sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    echo "missing required command: shasum or sha256sum" >&2
    exit 2
  fi
}

if [[ -z "$target" ]]; then
  case "$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" in
    x86_64-linux) target="x86_64-unknown-linux-gnu" ;;
    aarch64-linux|arm64-linux) target="aarch64-unknown-linux-gnu" ;;
    *)
      echo "cannot infer Linux WebKit PIC target; pass --target" >&2
      exit 2
      ;;
  esac
fi

case "$target" in
  x86_64-unknown-linux-gnu)
    asset="bun-webkit-linux-amd64-pic-release.tar.gz"
    ;;
  aarch64-unknown-linux-gnu)
    asset="bun-webkit-linux-arm64-pic-release.tar.gz"
    ;;
  *)
    echo "unsupported WebKit PIC target: $target" >&2
    exit 2
    ;;
esac

case "$asset" in
  *debug*|*Debug*)
    echo "debug WebKit PIC assets are forbidden for libbun production plugin builds: $asset" >&2
    exit 1
    ;;
esac

if [[ ! -f "$base_manifest" ]]; then
  echo "native link manifest not found: $base_manifest" >&2
  exit 2
fi

work_dir="${LIBBUN_WEBKIT_PIC_DIR:-"$build_dir/webkit-pic/$tag/$target"}"
download_dir="$work_dir/download"
extract_dir="$work_dir/extract"
rm -rf "$download_dir" "$extract_dir"
mkdir -p "$download_dir" "$extract_dir" "$(dirname "$out_manifest")" "$(dirname "$metadata_out")"

release_url="https://github.com/$repo/releases/download/$tag"
curl -fL --retry 3 --retry-delay 2 --output "$download_dir/$asset" "$release_url/$asset"
curl -fL --retry 3 --retry-delay 2 --output "$download_dir/checksums.txt" "$release_url/checksums.txt"
curl -fL --retry 3 --retry-delay 2 --output "$download_dir/metadata.json" "$release_url/metadata.json"

checksums="$download_dir/checksums.txt"
archive="$download_dir/$asset"
metadata="$download_dir/metadata.json"
if [[ ! -f "$checksums" || ! -f "$archive" || ! -f "$metadata" ]]; then
  echo "downloaded WebKit PIC release is missing required files" >&2
  exit 2
fi

if grep -i "debug" "$metadata" >/dev/null; then
  echo "WebKit PIC metadata contains debug build markers; release PIC inputs are required: $metadata" >&2
  exit 1
fi

expected="$(awk -v asset="$asset" '$2 == asset { print $1 }' "$checksums")"
if [[ -z "$expected" ]]; then
  echo "checksum file does not contain $asset" >&2
  exit 2
fi
actual="$(sha256 "$archive")"
if [[ "$actual" != "$expected" ]]; then
  echo "checksum mismatch for $asset" >&2
  echo "expected $expected" >&2
  echo "got      $actual" >&2
  exit 1
fi

gzip -dc -f "$archive" | tar -C "$extract_dir" -xf -
webkit_dir="$extract_dir/bun-webkit"
if [[ ! -d "$webkit_dir/lib" ]]; then
  echo "WebKit PIC archive did not extract to bun-webkit/lib" >&2
  exit 2
fi

replacement_count=0
while IFS= read -r line; do
  case "$line" in
    static=*)
      path="${line#static=}"
      base="$(basename "$path")"
      replacement="$webkit_dir/lib/$base"
      if [[ -f "$replacement" ]]; then
        printf 'static=%s\n' "$replacement"
        replacement_count=$((replacement_count + 1))
      else
        printf '%s\n' "$line"
      fi
      ;;
    *)
      printf '%s\n' "$line"
      ;;
  esac
done < "$base_manifest" > "$out_manifest"

if [[ "$replacement_count" -eq 0 ]]; then
  echo "no WebKit static libraries were replaced in $out_manifest" >&2
  exit 1
fi

python3 - "$metadata" "$metadata_out" "$repo" "$tag" "$target" "$asset" "$actual" "$webkit_dir" "$out_manifest" <<'PY'
import json
import pathlib
import sys

source, out, repo, tag, target, asset, checksum, webkit_dir, manifest = sys.argv[1:10]
data = json.loads(pathlib.Path(source).read_text())
data["selectedArtifact"] = {
    "repository": repo,
    "releaseTag": tag,
    "target": target,
    "asset": asset,
    "sha256": checksum,
    "extractedWebKitDirectory": webkit_dir,
    "rewrittenLinkManifest": manifest,
}
pathlib.Path(out).write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
PY

echo "WebKit PIC artifact verified: $repo $tag $asset"
echo "rewritten native link manifest: $out_manifest"
echo "WebKit PIC metadata: $metadata_out"
