# Vendored Bun Patches

Every libbun change to `vendor/bun` must be reproducible after re-vendoring a
new upstream Bun snapshot.

Prefer a numbered patch file in this directory, then apply it from
`scripts/apply-vendored-bun-patches.sh`. Deterministic rewrites in that script
are allowed for generated or frequently reshuffled upstream files, but they
must be idempotent and must verify the expected postcondition.

`scripts/update-vendored-bun.sh` archives the upstream snapshot, runs
`scripts/apply-vendored-bun-patches.sh`, vendors required extra dependencies,
and then writes the new vendor metadata.

`scripts/verify-vendored-bun-reproducible.sh` rebuilds a temporary vendor tree
from `BUN_SOURCE_COMMIT`, applies the same scripts, and compares the generated
tree to the tracked `vendor/bun` files.

Use `scripts/stage-vendored-bun-source.sh` after re-vendoring. It force-stages
only the pinned upstream Bun source files and explicitly scripted extra
dependency source, avoiding local build products while keeping ignored upstream
source files from being accidentally omitted.
