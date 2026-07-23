#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 <version> <release-worker-binary> <output-directory>" >&2
  exit 2
fi

version="$1"
worker="$2"
output="$3"
case "$worker" in
  /*) ;;
  *) worker="$(pwd)/$worker" ;;
esac

sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}
case "$output" in
  /*) ;;
  *) output="$(pwd)/$output" ;;
esac

if [[ ! -x "$worker" ]]; then
  echo "prepared-export worker is missing or not executable: $worker" >&2
  exit 1
fi
if [[ "$worker" != */release/libbun-runtime-native && "$worker" != */release/libbun-runtime-native.exe ]]; then
  echo "release bundles require the release-profile libbun-runtime-native binary: $worker" >&2
  exit 1
fi

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) target="x86_64-unknown-linux-gnu" ;;
  Linux-aarch64) target="aarch64-unknown-linux-gnu" ;;
  Darwin-arm64) target="aarch64-apple-darwin" ;;
  *)
    echo "unsupported prepared-export worker release target: $(uname -s)-$(uname -m)" >&2
    exit 1
    ;;
esac

stage="$(mktemp -d)"
trap 'rm -rf -- "$stage"' EXIT
mkdir -p "$output" "$stage/bin"
cp "$worker" "$stage/bin/$(basename "$worker")"
worker_sha="$(sha256 "$worker")"
python3 - "$stage/manifest.json" "$version" "$target" "$worker_sha" <<'PY'
import json
import pathlib
import sys

path, version, target, worker_sha = sys.argv[1:]
pathlib.Path(path).write_text(json.dumps({
    "format": "libbun.preparedExportWorker",
    "formatVersion": 1,
    "wireVersion": 1,
    "version": version,
    "target": target,
    "worker": {
        "filename": "libbun-runtime-native" + (".exe" if "windows" in target else ""),
        "sha256": worker_sha,
    },
    "execution": "fresh-process-only",
    "fallback": None,
}, indent=2) + "\n")
PY

archive="$output/libbun-prepared-export-worker-${version}-${target}.tar.zst"
tar -C "$stage" -cf - bin manifest.json | zstd -q -19 -T0 -o "$archive"
printf '%s  %s\n' "$(sha256 "$archive")" "$(basename "$archive")" > "$archive.sha256"
echo "created $archive"
