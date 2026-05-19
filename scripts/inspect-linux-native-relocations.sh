#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="${LIBBUN_NATIVE_LINK_MANIFEST:-"$repo_root/vendor/bun/build/debug/libbun_native_link_manifest.txt"}"

usage() {
  cat >&2 <<'USAGE'
usage: scripts/inspect-linux-native-relocations.sh [manifest]

Inspects the native Bun link manifest for ELF archive/object relocations that
cannot be linked into a Linux shared object.

Environment:
  LIBBUN_NATIVE_LINK_MANIFEST=<path>   override the default manifest path
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ $# -gt 1 ]]; then
  usage
  exit 2
fi

if [[ $# -eq 1 ]]; then
  manifest="$1"
fi

case "$(uname -s)" in
  Linux) ;;
  *)
    echo "linux relocation inspection requires a Linux host" >&2
    exit 2
    ;;
esac

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 2
  fi
}

require ar
require readelf

if [[ ! -f "$manifest" ]]; then
  echo "native link manifest not found: $manifest" >&2
  exit 2
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

findings="$tmpdir/findings.txt"
: > "$findings"

record_relocations() {
  local label="$1"
  local object="$2"

  if ! readelf -h "$object" >/dev/null 2>&1; then
    return 0
  fi

  readelf -rW "$object" 2>/dev/null |
    awk -v label="$label" '
      /R_X86_64_TPOFF32|R_X86_64_TPOFF64|R_AARCH64_TLSLE_/ {
        print label ": " $0
      }
    ' >> "$findings"
}

inspect_archive() {
  local archive="$1"
  local member
  local index=0

  while IFS= read -r member; do
    [[ -n "$member" ]] || continue
    index=$((index + 1))
    object="$tmpdir/member-${index}.o"
    if ar p "$archive" "$member" > "$object" 2>/dev/null; then
      record_relocations "$archive($member)" "$object"
    fi
  done < <(ar t "$archive")
}

inspect_path() {
  local path="$1"

  if [[ ! -f "$path" ]]; then
    echo "native link input not found: $path" >&2
    exit 2
  fi

  case "$path" in
    *.a) inspect_archive "$path" ;;
    *.o) record_relocations "$path" "$path" ;;
  esac
}

while IFS= read -r line; do
  [[ -n "$line" ]] || continue
  case "$line" in
    archive=*|static=*) inspect_path "${line#*=}" ;;
  esac
done < "$manifest"

if [[ -s "$findings" ]]; then
  echo "shared-object-hostile Linux TLS relocations found:" >&2
  sed 's/^/  /' "$findings" >&2
  exit 1
fi

echo "linux native link inputs have no known shared-object-hostile TLS relocations"
