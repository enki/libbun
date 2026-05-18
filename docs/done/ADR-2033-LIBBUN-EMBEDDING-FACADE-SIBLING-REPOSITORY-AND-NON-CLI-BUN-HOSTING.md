# ADR-2033: libbun Embedding Facade, Sibling Repository, and Non-CLI Bun Hosting

Status: Done
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

The native linked integration tests currently cover source modules and prepared
`libbun` bundles loaded through Bun's module loader, synchronous export calls,
async export parking and resolution, event-loop pumping, structured provider
errors, and shutdown against the real Bun C++ / JSC object set. The native
adapter also initializes Bun stdout/stderr to host-owned capture files before VM
creation and drains those files into `OutputRecord`s.

Follow-up ADRs:

- ADR-2034 defines the implemented prepared bundle artifact contract for
  embedding-only source bundle artifacts.
- ADR-2035 defines the implemented host-owned output delivery contract.
- ADR-2036 tracks a future dedicated native Bun internal log stream separate
  from provider stderr.
- ADR-2037 tracks future host environment overlay support without mutating
  process-global environment state; the unused config field has been removed
  from the active facade.

Completion boundary:

ADR-2036 and ADR-2037 are not blockers for this ADR. They track capabilities
that are deliberately outside the active `libbun` contract: dedicated native Bun
internal log classification and host environment overlays. The active facade
does not promise either capability.

Completion audit:

- Sibling reusable repository: satisfied by this repository's stable `libbun`
  crate, separate nightly-only `native/` crate, and private `enki/libbun`
  remote.
- Vendored Bun source: satisfied by `vendor/bun`, `BUN_SOURCE_COMMIT`,
  `scripts/update-vendored-bun.sh`, and `scripts/verify-vendored-bun.sh`.
- No sibling `../bun` dependency: satisfied by vendored update/check scripts and
  `scripts/verify-vendored-bun.sh`.
- Non-CLI embedding boundary: satisfied by `native/src/lib.rs`, which uses
  `bun_jsc`, `bun_runtime`, `VirtualMachine`, and `JSModuleLoader` directly.
  Repository search finds no active `bun_bin`, `Cli::start`, or Bun CLI `main`
  call in `src/`, `native/`, or tests.
- Stable host facade: satisfied by `src/lib.rs` with `BunEmbeddingRuntime`,
  `BunHost`, `BunRuntimeConfig`, structural values, module specs, async handles,
  output records, provider receipts, and structured errors.
- Rust-substrate rejection: satisfied by `BunHost::call_provider` rejecting
  `ProviderDomainClass::RustSubstrateAuthority` and covered by
  `tests/conformance.rs`.
- Source module embedding: satisfied by `native/tests/native_runtime.rs`.
- Prepared bundle embedding: satisfied by `PreparedBundleV1`,
  ADR-2034, and `native/tests/native_prepared_bundle.rs`.
- Provider success and structural results: satisfied by `tests/conformance.rs`
  and `native/tests/native_runtime.rs`.
- Async parking, event-loop pumping, and async resolution: satisfied by
  `tests/conformance.rs` and `native/tests/native_runtime.rs`.
- Structured provider errors without host process termination: satisfied by
  `tests/conformance.rs` and `native/tests/native_runtime.rs`.
- Host-owned stdout/stderr output capture and drop: satisfied by ADR-2035,
  `native/tests/native_runtime.rs`, and `native/tests/native_output_policy.rs`.
- Host-owned output draining and callback delivery: satisfied by `BunHost`,
  `BunEmbeddingRuntime::drain_captured_output`, and `tests/conformance.rs`.
- Deterministic shutdown: satisfied by `BunHost::shutdown`,
  `NativeBunRuntime::shutdown`, `tests/conformance.rs`, and
  `native/tests/native_runtime.rs`.
- Generic downstream substrate: satisfied by keeping the stable facade in the
  default crate and isolating the nightly Bun/JSC adapter in `native/`.

Verification performed before moving this ADR:

- `cargo test`
- `cargo clippy --tests -- -D warnings`
- `bash -n scripts/*.sh`
- `scripts/verify-vendored-bun.sh`
- `cargo +nightly-2026-05-06 check --manifest-path native/Cargo.toml`
- `LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 test --manifest-path native/Cargo.toml`

Known residual risk:

- The native test link still emits upstream WebKit/JSC linker alignment
  warnings on macOS. The linked tests pass, and the warnings are not introduced
  by the `libbun` facade.
- The native adapter embeds a single Bun VM lifecycle suitable for the current
  host boundary. Broader multi-runtime, worker-thread, WASM transport, dedicated
  internal log, and host environment semantics are tracked by follow-up ADRs.
