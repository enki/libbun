#!/usr/bin/env bash
set -euo pipefail

manifest="${1:-}"
context="${2:-native plugin distribution}"

usage() {
  cat >&2 <<'USAGE'
usage: scripts/assert-distributable-native-link.sh <native-link-manifest> [context]

Fails if a native link manifest contains archive= or static= inputs. Static
native Bun/JSC/WebKit inputs are allowed for local tests only; they must never
enter distributed plugin packages, release assets, or checksum publication.
USAGE
}

if [[ "$manifest" == "-h" || "$manifest" == "--help" ]]; then
  usage
  exit 0
fi

if [[ -z "$manifest" ]]; then
  usage
  exit 2
fi

if [[ ! -f "$manifest" ]]; then
  echo "native link manifest not found: $manifest" >&2
  exit 2
fi

blocked=()
while IFS= read -r line; do
  case "$line" in
    archive=*|static=*)
      blocked+=("$line")
      ;;
  esac
done < "$manifest"

if [[ ${#blocked[@]} -gt 0 ]]; then
  cat >&2 <<EOF
${context} is not distributable: native link manifest contains static-link inputs.

Static native Bun/JSC/WebKit inputs are permitted for local tests, but they must
not be packaged into GitHub Release assets, checksum tables, or crates.io-facing
release artifacts.

Manifest: ${manifest}
Blocked inputs:
EOF
  printf '  %s\n' "${blocked[@]}" >&2
  exit 1
fi

echo "${context} native link manifest is distributable: ${manifest}"
