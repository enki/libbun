#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="${1:-}"

usage() {
  cat >&2 <<'USAGE'
usage: scripts/create-native-plugin-release.sh <version>

Creates and pushes an annotated v* tag for the current commit. Pushing the tag
triggers .github/workflows/release-native-plugin.yml, which builds the native
plugin and attaches the compliance assets to the GitHub Release.

Run scripts/preflight-native-plugin-release.sh <version> first when preparing a
real release.

Environment:
  LIBBUN_RELEASE_REMOTE=<name>     remote to push to, default: origin
  LIBBUN_RELEASE_SKIP_BRANCH_PUSH=1
                                   push only the tag, not the current branch
USAGE
}

if [[ "$version" == "-h" || "$version" == "--help" ]]; then
  usage
  exit 0
fi

if [[ -z "$version" ]]; then
  usage
  exit 2
fi

case "$version" in
  v*) release_version="$version" ;;
  *) release_version="v$version" ;;
esac

remote="${LIBBUN_RELEASE_REMOTE:-origin}"

cd "$repo_root"

crate_version="$(python3 - <<'PY'
import pathlib
import tomllib

manifest = tomllib.loads(pathlib.Path("Cargo.toml").read_text())
print(manifest["package"]["version"])
PY
)"
if [[ "v${crate_version}" != "$release_version" ]]; then
  echo "Cargo.toml version is ${crate_version}, release tag is ${release_version}; refusing divergent release" >&2
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "working tree is dirty; commit or stash changes before creating ${release_version}" >&2
  git status --short >&2
  exit 1
fi

if ! git remote get-url "$remote" >/dev/null 2>&1; then
  echo "git remote not found: $remote" >&2
  exit 2
fi

if git rev-parse -q --verify "refs/tags/${release_version}" >/dev/null; then
  echo "tag already exists locally: ${release_version}" >&2
  exit 1
fi

if git ls-remote --exit-code --tags "$remote" "refs/tags/${release_version}" >/dev/null 2>&1; then
  echo "tag already exists on ${remote}: ${release_version}" >&2
  exit 1
fi

current_branch="$(git branch --show-current)"
if [[ -z "$current_branch" ]]; then
  echo "not on a branch; refusing to create release tag from detached HEAD" >&2
  exit 1
fi

echo "creating annotated tag ${release_version} at $(git rev-parse --short HEAD)"
git tag -a "$release_version" -m "Release ${release_version}"

if [[ "${LIBBUN_RELEASE_SKIP_BRANCH_PUSH:-0}" != "1" ]]; then
  echo "pushing branch ${current_branch} to ${remote}"
  git push "$remote" "$current_branch"
fi

echo "pushing tag ${release_version} to ${remote}"
git push "$remote" "$release_version"

cat <<EOF
release tag pushed: ${release_version}

GitHub Actions should now run:
  .github/workflows/release-native-plugin.yml

Check the release workflow and resulting GitHub Release before announcing the
native plugin asset.
EOF
