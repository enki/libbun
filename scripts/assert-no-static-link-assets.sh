#!/usr/bin/env bash
set -euo pipefail

context="${1:-}"
shift || true

usage() {
  cat >&2 <<'USAGE'
usage: scripts/assert-no-static-link-assets.sh <context> <archive-or-file>...

Fails if a distributable artifact contains static-linkable binary assets. The
native plugin may be tested with static native inputs, but release archives must
not publish object files, static libraries, Rust rlibs, or bitcode objects.
USAGE
}

if [[ "$context" == "-h" || "$context" == "--help" ]]; then
  usage
  exit 0
fi

if [[ -z "$context" || $# -eq 0 ]]; then
  usage
  exit 2
fi

is_static_link_asset() {
  case "$1" in
    *.a|*.o|*.rlib|*.lib|*.lo|*.bc)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

blocked=()
for artifact in "$@"; do
  if [[ ! -f "$artifact" ]]; then
    echo "artifact not found: $artifact" >&2
    exit 2
  fi

  case "$artifact" in
    *.tar.zst)
      while IFS= read -r member; do
        member="${member#./}"
        if is_static_link_asset "$member"; then
          blocked+=("$artifact:$member")
        fi
      done < <(zstd -dc "$artifact" | tar -tf -)
      ;;
    *)
      name="$(basename "$artifact")"
      if is_static_link_asset "$name"; then
        blocked+=("$artifact")
      fi
      ;;
  esac
done

if [[ ${#blocked[@]} -gt 0 ]]; then
  cat >&2 <<EOF
${context} contains static-linkable assets.

Static native inputs are allowed for local tests, but release artifacts must not
publish object files, static libraries, Rust rlibs, or bitcode objects.

Blocked assets:
EOF
  printf '  %s\n' "${blocked[@]}" >&2
  exit 1
fi

echo "${context} contains no static-linkable assets"
