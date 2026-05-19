#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo_url="${BUN_UPSTREAM_REPO:-https://github.com/oven-sh/bun.git}"
commit="$(tr -d '[:space:]' < "$repo_root/BUN_SOURCE_COMMIT")"
vendor_dir="$repo_root/vendor/bun"

if [[ ! "$commit" =~ ^[0-9a-f]{40}$ ]]; then
  echo "BUN_SOURCE_COMMIT must contain a 40-character git SHA" >&2
  exit 1
fi

if [[ ! -d "$vendor_dir" ]]; then
  echo "missing vendored Bun directory: $vendor_dir" >&2
  exit 1
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

git clone --filter=blob:none --no-checkout "$repo_url" "$tmp_dir/bun-src" >&2
git -C "$tmp_dir/bun-src" fetch --depth=1 origin "$commit" >&2

paths_file="$tmp_dir/source-paths"
git -C "$tmp_dir/bun-src" ls-tree -rz --name-only FETCH_HEAD |
  while IFS= read -r -d '' rel; do
    path="$vendor_dir/$rel"
    if [[ -e "$path" || -L "$path" ]]; then
      printf 'vendor/bun/%s\0' "$rel"
    fi
  done > "$paths_file"

xargs -0 git -C "$repo_root" add -f -- < "$paths_file"

git -C "$repo_root" add -f -- \
  BUN_SOURCE_COMMIT \
  vendor/bun.LIBBUN_VENDOR.json \
  vendor/bun/vendor/lolhtml

echo "staged reproducible vendored Bun source files from $commit"
