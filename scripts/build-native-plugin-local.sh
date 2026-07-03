#!/usr/bin/env bash
# build-native-plugin-local.sh — one command, zero arguments, fresh clone to a
# verified libbun native plugin on macOS and Linux.
#
# This is the CI in-process lane (.github/workflows/release-native-plugin.yml)
# extracted into a script a developer can run directly. The in-process lane is
# the ONLY shipping lane; the helper-process lane is quarantined legacy and is
# intentionally not reachable from here.
#
# Phases:
#   1. toolchain preflight   — every missing tool prints its exact fix command
#   2. native Bun link-prep  — configure vendored Bun + assemble the link manifest
#   3. PIC WebKit fetch       — Linux only, automatic; rewrites the manifest to PIC
#   4. build native plugin    — the exact CI cargo invocation for this OS
#   5. verification trio       — relocation inspect (Linux) + both smoke tests,
#                               using the CORRECT `--test <target>` invocation
#                               (CI's target-less form matches ZERO tests locally
#                               and passes vacuously — this script does not).
#
# Idempotent: re-running reuses the vendored-Bun link manifest and the PIC
# artifact when they are already present (pass --force to regenerate them), and
# the final plugin is rebuilt by cargo only when its inputs changed, so a
# no-op re-run is cheap and never produces a stale artifact.
#
# Divergence from CI recorded explicitly:
#   - CI pins LIBBUN_NATIVE_BUILD_JOBS=8 and sets LIBBUN_NATIVE_CLEAN_AFTER_MANIFEST=1
#     to bound disk on the runner. Locally we default jobs to the CPU count and
#     KEEP the intermediate Bun objects so re-runs are fast. Set
#     LIBBUN_NATIVE_CLEAN_AFTER_MANIFEST=1 to match CI's disk behavior.
#   - CI's smoke invocation omits `--test <target>`; measured locally that matches
#     zero tests and passes vacuously. This script always scopes to the target.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

force=0
for arg in "$@"; do
  case "$arg" in
    --force) force=1 ;;
    -h|--help)
      sed -n '2,40p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "unknown argument: $arg (only --force is accepted)" >&2
      exit 2
      ;;
  esac
done

log()  { printf '\n== %s ==\n' "$*"; }
info() { printf '   %s\n' "$*"; }
fail() { printf '   FAIL: %s\n' "$*" >&2; }

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)
    case "$arch" in
      x86_64)          target="x86_64-unknown-linux-gnu" ;;
      aarch64|arm64)   target="aarch64-unknown-linux-gnu" ;;
      *) echo "unsupported Linux arch: $arch" >&2; exit 2 ;;
    esac
    default_jobs="$(nproc)"
    ;;
  Darwin)
    case "$arch" in
      arm64)   target="aarch64-apple-darwin" ;;
      x86_64)  target="x86_64-apple-darwin" ;;
      *) echo "unsupported macOS arch: $arch" >&2; exit 2 ;;
    esac
    default_jobs="$(sysctl -n hw.ncpu)"
    ;;
  *)
    echo "unsupported OS: $os (this script builds only on Linux and macOS)" >&2
    exit 2
    ;;
esac

export LIBBUN_NATIVE_BUILD_JOBS="${LIBBUN_NATIVE_BUILD_JOBS:-$default_jobs}"
build_dir="$repo_root/vendor/bun/build/release"
base_manifest="$build_dir/libbun_native_link_manifest.txt"
pic_manifest="$build_dir/libbun_native_link_manifest.pic.txt"
plugin_name="liblibbun_plugin_native.so"
[[ "$os" == "Darwin" ]] && plugin_name="liblibbun_plugin_native.dylib"

# The pinned nightly the vendored Rust crates require (bare E0554 on stable).
NIGHTLY="nightly-2026-05-06"

# WebKit PIC artifact pin — identical to the CI workflow env.
export LIBBUN_WEBKIT_PIC_REPO="${LIBBUN_WEBKIT_PIC_REPO:-enki/WebKit}"
export LIBBUN_WEBKIT_PIC_TAG="${LIBBUN_WEBKIT_PIC_TAG:-libbun-webkit-pic-release-5488984d-20260520}"

# ---------------------------------------------------------------------------
# Phase 1 — toolchain preflight. Accumulate every failure, then exit once with
# all of them, each naming its exact fix command.
# ---------------------------------------------------------------------------
log "Phase 1/5: toolchain preflight ($os/$arch, target $target)"
preflight_ok=1

need_cmd() {
  # need_cmd <command> <fix-message>
  if command -v "$1" >/dev/null 2>&1; then
    info "ok: $1 -> $(command -v "$1")"
  else
    fail "missing '$1'"
    printf '         fix: %s\n' "$2" >&2
    preflight_ok=0
  fi
}

# bun (drives the vendored-Bun codegen configure).
need_cmd bun 'install Bun: curl -fsSL https://bun.sh/install | bash   (then restart your shell)'

# rustup + toolchains.
if command -v rustup >/dev/null 2>&1; then
  info "ok: rustup -> $(command -v rustup)"
  if rustup toolchain list 2>/dev/null | grep -q "^$NIGHTLY"; then
    info "ok: rust toolchain $NIGHTLY"
  else
    fail "missing rust toolchain $NIGHTLY"
    printf '         fix: rustup toolchain install %s\n' "$NIGHTLY" >&2
    preflight_ok=0
  fi
  if ! rustup toolchain list 2>/dev/null | grep -q "^stable"; then
    fail "missing rust toolchain stable"
    printf '         fix: rustup toolchain install stable\n' >&2
    preflight_ok=0
  fi
else
  fail "missing 'rustup'"
  printf "         fix: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n" >&2
  preflight_ok=0
fi

if [[ "$os" == "Linux" ]]; then
  llvm_bin="/usr/lib/llvm-21/bin"
  # Put the versioned LLVM bindir on PATH if apt.llvm.org installed it there.
  if [[ -d "$llvm_bin" ]]; then
    case ":$PATH:" in *":$llvm_bin:"*) ;; *) export PATH="$llvm_bin:$PATH" ;; esac
  fi
  llvm_fix='install clang/lld 21 from apt.llvm.org:
              wget -O /tmp/llvm.sh https://apt.llvm.org/llvm.sh && chmod +x /tmp/llvm.sh && sudo /tmp/llvm.sh 21 all
              (clang-21/lld-21 are NOT in the Ubuntu noble repos)'
  for tool in clang-21 clang++-21 lld-21; do need_cmd "$tool" "$llvm_fix"; done
  need_cmd ninja      'sudo apt-get update && sudo apt-get install -y ninja-build'
  need_cmd cmake      'sudo apt-get update && sudo apt-get install -y cmake'
  need_cmd zstd       'sudo apt-get update && sudo apt-get install -y zstd'
  need_cmd ar         'sudo apt-get update && sudo apt-get install -y binutils'
  need_cmd readelf    'sudo apt-get update && sudo apt-get install -y binutils'
  need_cmd curl       'sudo apt-get update && sudo apt-get install -y curl'
else
  brew_llvm='brew install llvm@21 ninja cmake zstd   (then add "$(brew --prefix llvm@21)/bin" to PATH)'
  if command -v brew >/dev/null 2>&1; then
    llvm_prefix="$(brew --prefix llvm@21 2>/dev/null || true)"
    if [[ -n "$llvm_prefix" && -d "$llvm_prefix/bin" ]]; then
      case ":$PATH:" in *":$llvm_prefix/bin:"*) ;; *) export PATH="$llvm_prefix/bin:$PATH" ;; esac
    fi
  fi
  need_cmd clang   "$brew_llvm"
  need_cmd ninja   'brew install ninja'
  need_cmd cmake   'brew install cmake'
  need_cmd zstd    'brew install zstd'
  need_cmd xcrun   'install the Xcode command line tools: xcode-select --install'
fi

if [[ "$preflight_ok" != "1" ]]; then
  fail "toolchain preflight failed; install the tools named above and re-run."
  exit 1
fi
info "toolchain preflight passed"

# ---------------------------------------------------------------------------
# Phase 2 — native Bun link-prep. prepare-native-bun-link.sh runs the vendored
# Bun configure (bun bd --configure-only + ninja codegen) internally, builds the
# native objects, and writes $base_manifest.
# ---------------------------------------------------------------------------
log "Phase 2/5: native Bun link-prep (jobs=$LIBBUN_NATIVE_BUILD_JOBS)"
if [[ "$force" == "0" && -s "$base_manifest" ]]; then
  info "reusing existing native link manifest: $base_manifest"
  info "(pass --force to regenerate the vendored-Bun link inputs)"
else
  scripts/prepare-native-bun-link.sh
fi
[[ -s "$base_manifest" ]] || { fail "native link manifest not produced: $base_manifest"; exit 1; }

# ---------------------------------------------------------------------------
# Phase 3 — PIC WebKit fetch (Linux in-process only). Default build-cache WebKit
# archives are non-PIC; their R_X86_64_TPOFF32 relocations fail the -shared link.
# ---------------------------------------------------------------------------
link_manifest="$base_manifest"
if [[ "$os" == "Linux" ]]; then
  log "Phase 3/5: PIC WebKit fetch (Linux, automatic)"
  if [[ "$force" == "0" && -s "$pic_manifest" ]]; then
    info "reusing existing PIC link manifest: $pic_manifest"
    info "(pass --force to re-download and re-verify the PIC WebKit artifact)"
  else
    scripts/fetch-webkit-pic-artifact.sh \
      --target "$target" \
      --manifest "$base_manifest" \
      --out "$pic_manifest"
  fi
  [[ -s "$pic_manifest" ]] || { fail "PIC link manifest not produced: $pic_manifest"; exit 1; }
  link_manifest="$pic_manifest"
else
  log "Phase 3/5: PIC WebKit fetch — skipped (macOS does not need PIC WebKit)"
fi
export LIBBUN_NATIVE_LINK_MANIFEST="$link_manifest"

# ---------------------------------------------------------------------------
# Phase 4 — build the native plugin (exact CI cargo invocation for this OS).
# ---------------------------------------------------------------------------
log "Phase 4/5: build native plugin ($NIGHTLY, release)"
if [[ "$os" == "Linux" ]]; then
  LIBBUN_NATIVE_LINK_BUN=1 \
  CARGO_INCREMENTAL=0 \
  CARGO_PROFILE_DEV_DEBUG=0 \
  RUSTFLAGS="-C link-arg=-fuse-ld=lld" \
    cargo "+$NIGHTLY" build --release --manifest-path plugin/Cargo.toml --features linux-in-process
else
  LIBBUN_NATIVE_LINK_BUN=1 \
  CARGO_INCREMENTAL=0 \
  CARGO_PROFILE_DEV_DEBUG=0 \
    cargo "+$NIGHTLY" build --release --manifest-path plugin/Cargo.toml
fi

plugin_path="$(find plugin/target/release -maxdepth 1 -name "$plugin_name" -print -quit)"
[[ -n "$plugin_path" ]] || { fail "built plugin not found: plugin/target/release/$plugin_name"; exit 1; }
plugin_path="$(cd "$(dirname "$plugin_path")" && pwd)/$(basename "$plugin_path")"
info "built plugin: $plugin_path"

# ---------------------------------------------------------------------------
# Phase 5 — verification trio: relocation inspect (Linux) + both smoke tests,
# scoped to the correct test target so they cannot pass vacuously.
# ---------------------------------------------------------------------------
log "Phase 5/5: verification trio"

if [[ "$os" == "Linux" ]]; then
  info "relocation inspect (PIC manifest)"
  scripts/inspect-linux-native-relocations.sh "$link_manifest"
else
  info "relocation inspect — skipped (Linux-only check)"
fi

run_smoke() {
  # run_smoke <label> <test-target> <test-name> <logfile>
  local label="$1" target_name="$2" test_name="$3" logfile="$4"
  info "smoke: $label ($target_name :: $test_name)"
  set -o pipefail
  env -u LIBBUN_RUNTIME_NATIVE_PATH LIBBUN_PLUGIN_PATH="$plugin_path" \
    cargo test --features dynamic-loading --test "$target_name" "$test_name" -- --exact --nocapture \
    2>&1 | tee "$logfile"
  if grep -q "mimalloc: error" "$logfile"; then
    fail "$label emitted a mimalloc diagnostic"; exit 1
  fi
  if grep -Eq '\[(loop|filesink)\]' "$logfile"; then
    fail "$label leaked internal Bun diagnostics to host stdout/stderr"; exit 1
  fi
  if ! grep -Eq '1 passed;.*0 failed' "$logfile"; then
    fail "$label did not report exactly 1 passing test (CI's target-less form matches zero and passes vacuously)"
    exit 1
  fi
}

smoke_dir="${TMPDIR:-/tmp}"
run_smoke "provider flow"     dynamic_plugin       dynamic_plugin_provider_flow       "$smoke_dir/libbun-dynamic-plugin-smoke.log"
run_smoke "facade conformance" dynamic_conformance dynamic_plugin_facade_conformance "$smoke_dir/libbun-dynamic-plugin-conformance.log"

log "DONE"
info "native plugin verified: $plugin_path"
info "export it for the swarm build (BUILD time):"
info "  export SS_LIBBUN_PLUGIN_NATIVE_PATH=$plugin_path"
