# Vendored Bun

`vendor/bun` contains a tracked source snapshot of upstream Bun. It is vendored
with `git archive`, so it intentionally excludes upstream `.git` metadata and
local build artifacts.

The pinned upstream commit is recorded in `../BUN_SOURCE_COMMIT`.

To update the snapshot:

```sh
scripts/update-vendored-bun.sh main
```

Set `BUN_UPSTREAM_REPO` to use a fork:

```sh
BUN_UPSTREAM_REPO=https://github.com/oven-sh/bun.git scripts/update-vendored-bun.sh <ref>
```
