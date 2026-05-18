#!/usr/bin/env bash
set -euo pipefail

repo_url="${BUN_UPSTREAM_REPO:-https://github.com/oven-sh/bun.git}"
ref="${1:-main}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
vendor_dir="$repo_root/vendor/bun"
commit_file="$repo_root/BUN_SOURCE_COMMIT"
metadata_file="$repo_root/vendor/bun.LIBBUN_VENDOR.json"

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

git clone --filter=blob:none --no-checkout "$repo_url" "$tmp_dir/bun" >&2
git -C "$tmp_dir/bun" fetch --depth=1 origin "$ref" >&2
commit="$(git -C "$tmp_dir/bun" rev-parse FETCH_HEAD)"

rm -rf "$vendor_dir"
mkdir -p "$vendor_dir"
git -C "$tmp_dir/bun" archive --format=tar "$commit" | tar -x -C "$vendor_dir"

printf '%s\n' "$commit" > "$commit_file"
mkdir -p "$(dirname "$metadata_file")"
cat > "$metadata_file" <<JSON
{
  "upstream": "$repo_url",
  "ref": "$ref",
  "commit": "$commit",
  "vendoredPath": "vendor/bun",
  "updatedAt": "$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
}
JSON

echo "Vendored Bun $commit from $repo_url ($ref)"
