# ADR-2033: libbun Embedding Facade, Sibling Repository, and Non-CLI Bun Hosting

Status: In Progress
Date: 2026-05-18

`libbun` is a sibling repository that owns a hostable Bun embedding facade and a
vendored upstream Bun source snapshot. The facade must not call Bun CLI `main`,
`Cli::start`, or process-global command dispatch. Bun provider failures must
return structured errors rather than terminating the host process.

The initial implementation in this repository defines:

- a versioned Rust embedding ABI;
- structural value and provider result carriers;
- explicit module load, export call, event-loop pump, async-handle, output sink,
  and shutdown boundaries;
- provider-host receipts carrying contract identity and `libbun` artifact
  fingerprint;
- Rust-substrate export rejection before provider execution;
- conformance tests for provider success, async resolution, structured provider
  error, deterministic shutdown, captured output, and no process-exit-shaped
  control flow.

The current Bun source target is recorded in `BUN_SOURCE_COMMIT`.
The source snapshot lives in `vendor/bun`, and `scripts/update-vendored-bun.sh`
recreates it from an upstream Git ref without requiring a sibling `../bun`
checkout. `scripts/verify-vendored-bun.sh` checks that the source pin, metadata,
and expected Bun source layout are present.
`scripts/configure-vendored-bun.sh` runs Bun configure/codegen from the vendored
tree, restores Bun-managed Rust source dependencies such as `lolhtml`, and
rewrites generated Bun artifact identity to `BUN_SOURCE_COMMIT` so `libbun`
receipts do not accidentally report the parent `libbun` Git commit.
`scripts/check-vendored-bun-rust.sh` verifies the reusable `bun_jsc` and
`bun_runtime` crates against that prepared vendored tree.

Next work is to bind the facade to reusable non-CLI Bun/JSC internals from the
vendored Bun source without linking the CLI-shaped `bun_bin` staticlib as the
embedding boundary.
