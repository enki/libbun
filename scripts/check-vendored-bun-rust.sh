#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$repo_root/scripts/configure-vendored-bun.sh"

cargo +nightly-2026-05-06 check --manifest-path "$repo_root/vendor/bun/Cargo.toml" -p bun_jsc
cargo +nightly-2026-05-06 check --manifest-path "$repo_root/vendor/bun/Cargo.toml" -p bun_runtime
cargo +nightly-2026-05-06 check --manifest-path "$repo_root/native/Cargo.toml" --features internal-adapter
cargo +nightly-2026-05-06 check --manifest-path "$repo_root/plugin/Cargo.toml"
