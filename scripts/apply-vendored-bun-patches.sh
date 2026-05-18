#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="$repo_root/vendor/bun"
externs="$bun_dir/src/windows_sys/externs.rs"

if [[ ! -f "$externs" ]]; then
  echo "missing vendored Bun windows externs at $externs" >&2
  exit 1
fi

perl -0pi -e 's/#\[link\(name = "([^"]+)"\)\]/#[cfg_attr(windows, link(name = "$1"))]/g' "$externs"

if rg '#\[link\(name =' "$externs" >/dev/null; then
  echo "unconditional Windows link attributes remain in $externs" >&2
  exit 1
fi

echo "Applied libbun vendored Bun patches"
