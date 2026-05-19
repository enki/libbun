#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE' >&2
usage: scripts/update-plugin-checksums.sh --version <vX.Y.Z> [--dir <asset-dir> | --repo <owner/repo>]

Updates src/plugin_checksums_table.in from native plugin release assets.
USAGE
}

version=""
asset_dir=""
repo="enki/libbun"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      version="${2:?missing version}"
      shift 2
      ;;
    --dir)
      asset_dir="${2:?missing asset dir}"
      shift 2
      ;;
    --repo)
      repo="${2:?missing repo}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
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

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
targets=(
  aarch64-apple-darwin
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
)

tmpdir=""
if [[ -z "$asset_dir" ]]; then
  command -v gh >/dev/null || {
    echo "gh is required when --dir is omitted" >&2
    exit 1
  }
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT
  gh release download "$release_version" \
    --repo "$repo" \
    --pattern "libbun-plugin-native-${release_version}-checksums.txt" \
    --dir "$tmpdir"
  asset_dir="$tmpdir"
fi

checksums="$asset_dir/libbun-plugin-native-${release_version}-checksums.txt"
if [[ ! -f "$checksums" ]]; then
  echo "missing checksum file: $checksums" >&2
  exit 1
fi

python3 - "$release_version" "$checksums" "$repo_root/src/plugin_checksums_table.in" "${targets[@]}" <<'PY'
import pathlib
import sys

release_version = sys.argv[1]
checksums_path = pathlib.Path(sys.argv[2])
out_path = pathlib.Path(sys.argv[3])
targets = sys.argv[4:]

checksums = {}
for line in checksums_path.read_text().splitlines():
    digest, sep, name = line.partition("  ")
    if sep:
        checksums[name] = digest

rows = []
for target in targets:
    name = f"libbun-plugin-native-{release_version}-{target}.tar.zst"
    try:
        digest = checksums[name]
    except KeyError:
        raise SystemExit(f"checksum file does not contain {name}")
    rows.append((release_version, target, digest))

lines = ["&[\n"]
for version, target, digest in rows:
    lines.append(f'    ("{version}", "{target}", "{digest}"),\n')
lines.append("]\n")
out_path.write_text("".join(lines))
PY

echo "updated src/plugin_checksums_table.in for ${release_version}"
