#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="$repo_root/vendor/bun"
commit="$(tr -d '[:space:]' < "$repo_root/BUN_SOURCE_COMMIT")"
build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$bun_dir/build/debug"}"
case "$build_dir" in
  /*) ;;
  *) build_dir="$repo_root/$build_dir" ;;
esac
build_options="$build_dir/codegen/build_options.rs"
configure_args=("$@")

"$repo_root/scripts/apply-vendored-bun-patches.sh" >&2

has_build_dir=0
for arg in "${configure_args[@]}"; do
  case "$arg" in
    --build-dir|--build-dir=*)
      has_build_dir=1
      ;;
  esac
done

if [[ "$has_build_dir" == "0" && -n "${LIBBUN_NATIVE_BUN_BUILD_DIR:-}" ]]; then
  configure_args+=("--build-dir=$build_dir")
fi

(cd "$bun_dir" && bun run build --configure-only "${configure_args[@]}")
(cd "$bun_dir" && ninja -C "$build_dir" codegen)
"$repo_root/scripts/vendor-bun-deps.sh" >&2
"$repo_root/scripts/verify-vendored-bun.sh" >&2

if [[ ! -f "$build_options" ]]; then
  echo "Bun configure did not create $build_options" >&2
  exit 1
fi

tmp="$(mktemp)"
sed -E "s/^pub const SHA: &str = \"[0-9a-f]+\";$/pub const SHA: \&str = \"$commit\";/" "$build_options" > "$tmp"
mv "$tmp" "$build_options"

echo "Configured vendored Bun codegen with pinned Bun SHA $commit"
