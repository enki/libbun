#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
commit_file="$repo_root/BUN_SOURCE_COMMIT"
metadata_file="$repo_root/vendor/bun.LIBBUN_VENDOR.json"
vendor_dir="$repo_root/vendor/bun"

commit="$(tr -d '[:space:]' < "$commit_file")"

if [[ ! "$commit" =~ ^[0-9a-f]{40}$ ]]; then
  echo "BUN_SOURCE_COMMIT must contain a 40-character git SHA" >&2
  exit 1
fi

if [[ ! -d "$vendor_dir" ]]; then
  echo "missing vendored Bun directory: vendor/bun" >&2
  exit 1
fi

for required in Cargo.toml package.json src/bun_bin/Cargo.toml src/jsc/Cargo.toml src/runtime/Cargo.toml vendor/lolhtml/c-api/Cargo.toml; do
  if [[ ! -f "$vendor_dir/$required" ]]; then
    echo "vendored Bun is missing $required" >&2
    exit 1
  fi
done

if [[ -d "$vendor_dir/.git" ]]; then
  echo "vendor/bun must be an archive snapshot, not a nested git checkout" >&2
  exit 1
fi

if ! grep -q "\"commit\": \"$commit\"" "$metadata_file"; then
  echo "vendor metadata commit does not match BUN_SOURCE_COMMIT" >&2
  exit 1
fi

lolhtml_commit="$(sed -nE 's/^const LOLHTML_COMMIT = "([0-9a-f]{40})";$/\1/p' "$vendor_dir/scripts/build/deps/lolhtml.ts")"
if [[ -z "$lolhtml_commit" ]]; then
  echo "could not find LOLHTML_COMMIT in vendored Bun source" >&2
  exit 1
fi

if [[ "$(tr -d '[:space:]' < "$vendor_dir/vendor/lolhtml/.ref")" != "$lolhtml_commit" ]]; then
  echo "vendored lolhtml .ref does not match Bun's pinned LOLHTML_COMMIT" >&2
  exit 1
fi

echo "Vendored Bun snapshot verified: $commit"
