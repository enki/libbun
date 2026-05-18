#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="$repo_root/vendor/bun"
build_dir="$bun_dir/build/debug"
archive="$build_dir/libbun_native_objects.a"
objects_file="$build_dir/libbun_native_objects.txt"
manifest="$build_dir/libbun_native_link_manifest.txt"

"$repo_root/scripts/configure-vendored-bun.sh" --profile=debug-no-asan >&2
ninja -C "$build_dir" bun -j"${LIBBUN_NATIVE_BUILD_JOBS:-8}" >&2

ninja -C "$build_dir" -t query bun-debug |
  awk '
    /^  input:/ { in_input = 1; next }
    /^  outputs:/ { in_input = 0; next }
    in_input && $1 ~ /\.o$/ { print $1 }
  ' > "$objects_file"

if [[ ! -s "$objects_file" ]]; then
  echo "no Bun native object files found from ninja query" >&2
  exit 1
fi

(cd "$build_dir" && xcrun libtool -static -o "$archive" $(cat "$objects_file")) >&2

{
  printf 'archive=%s\n' "$archive"
  ninja -C "$build_dir" -t query bun-debug |
    awk '
      /^  input:/ { in_input = 1; next }
      /^  outputs:/ { in_input = 0; next }
      in_input && $1 ~ /\.a$/ && $1 !~ /rust-target/ { print $1 }
    ' |
    while IFS= read -r static_lib; do
      case "$static_lib" in
        /*) printf 'static=%s\n' "$static_lib" ;;
        *) printf 'static=%s\n' "$build_dir/$static_lib" ;;
      esac
    done
} > "$manifest"

echo "Prepared native Bun link manifest at $manifest"
