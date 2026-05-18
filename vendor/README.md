# Vendored Bun

`vendor/bun` contains a tracked source snapshot of upstream Bun. It is vendored
with `git archive`, so it intentionally excludes upstream `.git` metadata and
local build artifacts.

Some Bun build-time source dependencies are fetched by Bun's build scripts into
its own `vendor/` directory rather than tracked in the Bun repository. `libbun`
vendors the dependencies needed for Rust crate consumption as part of the same
snapshot. Today that includes `vendor/bun/vendor/lolhtml`, pinned by Bun's
`scripts/build/deps/lolhtml.ts`.

The pinned upstream commit is recorded in `../BUN_SOURCE_COMMIT`.

To update the snapshot:

```sh
scripts/update-vendored-bun.sh main
```

Set `BUN_UPSTREAM_REPO` to use a fork:

```sh
BUN_UPSTREAM_REPO=https://github.com/oven-sh/bun.git scripts/update-vendored-bun.sh <ref>
```
