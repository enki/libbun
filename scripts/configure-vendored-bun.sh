#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="$repo_root/vendor/bun"
commit="$(tr -d '[:space:]' < "$repo_root/BUN_SOURCE_COMMIT")"
build_options="$bun_dir/build/debug/codegen/build_options.rs"

(cd "$bun_dir" && bun run build --configure-only "$@")
"$repo_root/scripts/apply-vendored-bun-patches.sh" >&2
"$repo_root/scripts/vendor-bun-deps.sh" >&2
(cd "$bun_dir" && ninja -C build/debug codegen)
"$repo_root/scripts/verify-vendored-bun.sh" >&2

if [[ ! -f "$build_options" ]]; then
  echo "Bun configure did not create $build_options" >&2
  exit 1
fi

tmp="$(mktemp)"
sed -E "s/^pub const SHA: &str = \"[0-9a-f]+\";$/pub const SHA: \&str = \"$commit\";/" "$build_options" > "$tmp"
mv "$tmp" "$build_options"

echo "Configured vendored Bun codegen with pinned Bun SHA $commit"
