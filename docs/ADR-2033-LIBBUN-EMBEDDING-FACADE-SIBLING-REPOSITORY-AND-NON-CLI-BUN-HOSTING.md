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
`bun_runtime` crates against that prepared vendored tree and type-checks the
nightly-only `native/` adapter.

The native adapter lives in `native/` so the default `libbun` crate remains a
generic stable facade. The adapter implements `BunEmbeddingRuntime` using the
vendored `bun_jsc` and `bun_runtime` crates rather than the CLI-shaped
`bun_bin` staticlib boundary.
`scripts/prepare-native-bun-link.sh` builds the vendored Bun no-ASAN debug
target, archives the native C/C++ object set used by Bun's own link, records the
prebuilt WebKit/JSC static libraries, and lets `native/build.rs` link them for
integration tests when `LIBBUN_NATIVE_LINK_BUN=1` is set. The manifest
intentionally excludes Bun's Rust staticlib; the native adapter links the
vendored Rust crates directly to avoid duplicate Rust runtime and VM state.

The native linked integration test currently covers a source module loaded
through Bun's module loader, synchronous export calls, async export parking and
resolution, event-loop pumping, structured provider errors, and shutdown
against the real Bun C++ / JSC object set. The native adapter also initializes
Bun stdout/stderr to host-owned capture files before VM creation and drains
those files into `OutputRecord`s.

Known follow-up ADRs:

- ADR-2034 defines the prepared bundle artifact contract before
  `BunModuleSpec::PreparedBundle` can be honestly implemented.
- ADR-2035 defines host-owned output and log sink semantics beyond the current
  stdout/stderr capture path.

Remaining work before this ADR can move to `docs/done/`:

- prepared bundle loading semantics, tracked by ADR-2034;
- complete host-owned output/log sink semantics, tracked by ADR-2035;
- a completion audit proving the non-CLI native adapter covers the provider
  success, async, structured error, event-loop pump, output, and shutdown cases
  end to end.
