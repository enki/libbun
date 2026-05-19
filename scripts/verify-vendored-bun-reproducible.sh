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

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 2
  fi
}

require diff
require git
require perl
require patch

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

generated="$tmp_dir/vendor/bun"
current="$tmp_dir/current"
mkdir -p "$generated"

git clone --filter=blob:none --no-checkout "$repo_url" "$tmp_dir/bun-src" >&2
git -C "$tmp_dir/bun-src" fetch --depth=1 origin "$commit" >&2
git -C "$tmp_dir/bun-src" archive --format=tar "$commit" | tar -x -C "$generated"

LIBBUN_VENDOR_DIR="$generated" "$repo_root/scripts/apply-vendored-bun-patches.sh" >&2
LIBBUN_VENDOR_DIR="$generated" "$repo_root/scripts/vendor-bun-deps.sh" >&2

mkdir -p "$current"
git -C "$repo_root" ls-files -z vendor/bun |
  git -C "$repo_root" checkout-index -z --stdin --prefix="$current/"

if ! git -C "$repo_root" diff --quiet -- vendor/bun; then
  git -C "$repo_root" diff --binary -- vendor/bun | (cd "$current" && patch -p1) >&2
fi

diff_out="$tmp_dir/diff.txt"
if ! git diff --no-index --quiet "$generated" "$current/vendor/bun"; then
  git diff --no-index --name-status "$generated" "$current/vendor/bun" > "$diff_out" || true
  echo "vendored Bun source is not identical after applying libbun scripts" >&2
  sed -n '1,200p' "$diff_out" >&2
  exit 1
fi

echo "vendored Bun is reproducible from $repo_url at $commit plus libbun scripts"
