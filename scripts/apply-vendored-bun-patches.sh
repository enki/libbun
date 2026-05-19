#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="${LIBBUN_VENDOR_DIR:-"$repo_root/vendor/bun"}"
externs="$bun_dir/src/windows_sys/externs.rs"
pic_patch="$repo_root/patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch"

apply_patch_file_once() {
  local patch_file="$1"
  local marker="$2"

  if [[ ! -f "$patch_file" ]]; then
    echo "missing vendored Bun patch: $patch_file" >&2
    exit 1
  fi

  if rg -q "$marker" "$bun_dir"; then
    echo "Vendored Bun patch already applied: $(basename "$patch_file")"
    return
  fi

  (cd "$bun_dir" && patch -p1 < "$patch_file") >&2
}

apply_patch_file_once "$pic_patch" "LIBBUN_NATIVE_PLUGIN_PIC"

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
