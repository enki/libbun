#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="${LIBBUN_VENDOR_DIR:-"$repo_root/vendor/bun"}"
lolhtml_repo="${LOLHTML_UPSTREAM_REPO:-https://github.com/cloudflare/lol-html.git}"
lolhtml_dir="$bun_dir/vendor/lolhtml"

lolhtml_commit="$(sed -nE 's/^const LOLHTML_COMMIT = "([0-9a-f]{40})";$/\1/p' "$bun_dir/scripts/build/deps/lolhtml.ts")"
if [[ -z "$lolhtml_commit" ]]; then
  echo "could not find LOLHTML_COMMIT in vendored Bun source" >&2
  exit 1
fi

if [[ -f "$lolhtml_dir/.ref" ]] &&
   [[ "$(tr -d '[:space:]' < "$lolhtml_dir/.ref")" == "$lolhtml_commit" ]] &&
   [[ -f "$lolhtml_dir/c-api/Cargo.toml" ]]; then
  echo "Vendored lolhtml already present at $lolhtml_commit"
  exit 0
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

rm -rf "$lolhtml_dir"
mkdir -p "$lolhtml_dir"

git clone --filter=blob:none --no-checkout "$lolhtml_repo" "$tmp_dir/lolhtml" >&2
git -C "$tmp_dir/lolhtml" fetch --depth=1 origin "$lolhtml_commit" >&2
git -C "$tmp_dir/lolhtml" archive --format=tar "$lolhtml_commit" | tar -x -C "$lolhtml_dir"
(cd "$lolhtml_dir" && patch -p1 < "$bun_dir/patches/lolhtml/0001-rlib-only.patch") >&2
printf '%s\n' "$lolhtml_commit" > "$lolhtml_dir/.ref"

echo "Vendored lolhtml $lolhtml_commit from $lolhtml_repo"
