#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun_dir="$repo_root/vendor/bun"
profile="${LIBBUN_NATIVE_BUN_PROFILE:-release}"
build_dir="${LIBBUN_NATIVE_BUN_BUILD_DIR:-"$bun_dir/build/$profile"}"
exe_target="${LIBBUN_NATIVE_BUN_EXE_TARGET:-}"
if [[ "$profile" != "release" ]]; then
  echo "libbun native plugin links must be prepared from Bun's release profile; got LIBBUN_NATIVE_BUN_PROFILE=$profile" >&2
  exit 1
fi
case "$build_dir" in
  /*) ;;
  *) build_dir="$repo_root/$build_dir" ;;
esac
archive="$build_dir/libbun_native_objects.a"
objects_file="$build_dir/libbun_native_objects.txt"
static_libs_file="$build_dir/libbun_native_static_libs.txt"
build_inputs_file="$build_dir/libbun_native_build_inputs.txt"
manifest="$build_dir/libbun_native_link_manifest.txt"

if [[ "$(uname -s)" == "Linux" ]]; then
  export LIBBUN_NATIVE_PLUGIN_PIC="${LIBBUN_NATIVE_PLUGIN_PIC:-1}"
fi

if [[ -z "$exe_target" ]]; then
  exe_target="bun-profile"
fi

if [[ "$exe_target" != "bun-profile" ]]; then
  echo "libbun native plugin links must use Bun's release bun-profile target; got LIBBUN_NATIVE_BUN_EXE_TARGET=$exe_target" >&2
  exit 1
fi

"$repo_root/scripts/configure-vendored-bun.sh" "--profile=$profile" >&2

ninja -C "$build_dir" -t query "$exe_target" |
  awk '
    /^  input:/ { in_input = 1; next }
    /^  outputs:/ { in_input = 0; next }
    in_input && $1 ~ /\.o$/ { print $1 }
  ' > "$objects_file"

ninja -C "$build_dir" -t query "$exe_target" |
  awk '
    /^  input:/ { in_input = 1; next }
    /^  outputs:/ { in_input = 0; next }
    in_input && $1 ~ /\.a$/ && $1 !~ /rust-target/ { print $1 }
  ' > "$static_libs_file"

if [[ ! -s "$objects_file" ]]; then
  echo "no Bun native object files found from ninja query" >&2
  exit 1
fi

cat "$objects_file" "$static_libs_file" > "$build_inputs_file"
build_inputs=()
while IFS= read -r input; do
  build_inputs+=("$input")
done < "$build_inputs_file"
if [[ ${#build_inputs[@]} -eq 0 ]]; then
  echo "no Bun native link inputs found from ninja query" >&2
  exit 1
fi

ninja -C "$build_dir" -j"${LIBBUN_NATIVE_BUILD_JOBS:-8}" "${build_inputs[@]}" >&2

case "$(uname -s)" in
  Darwin)
    (cd "$build_dir" && xcrun libtool -static -o "$archive" $(cat "$objects_file")) >&2
    ;;
  Linux)
    (cd "$build_dir" && rm -f "$archive" && ar crs "$archive" $(cat "$objects_file")) >&2
    ;;
  *)
    echo "unsupported native plugin build OS: $(uname -s)" >&2
    exit 1
    ;;
esac

{
  printf 'archive=%s\n' "$archive"
  cat "$static_libs_file" |
    while IFS= read -r static_lib; do
      case "$static_lib" in
        /*) printf 'static=%s\n' "$static_lib" ;;
        *) printf 'static=%s\n' "$build_dir/$static_lib" ;;
      esac
    done
} > "$manifest"

if grep -F "build/debug" "$manifest" >/dev/null ||
  grep -F "bun-debug" "$manifest" >/dev/null ||
  grep -F -- "-debug/" "$manifest" >/dev/null; then
  echo "native Bun link manifest contains debug build inputs: $manifest" >&2
  exit 1
fi

if [[ "${LIBBUN_NATIVE_CLEAN_AFTER_MANIFEST:-0}" == "1" ]]; then
  rm -rf \
    "$build_dir/$exe_target" \
    "$build_dir/obj" \
    "$build_dir/rust-target"
fi

echo "Prepared native Bun link manifest for $exe_target at $manifest"
