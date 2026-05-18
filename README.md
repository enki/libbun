# libbun

Rust facade for hosting JavaScript and TypeScript providers through a non-CLI
Bun embedding boundary.

This repository owns the stable facade, conformance tests, and a vendored Bun
source snapshot. It does not call Bun CLI `main`, `Cli::start`, or
process-global command dispatch.

Current Bun source target:

```text
9ecb985ad0f06fa12cbd8eede2404589992527d5
```

## Status

The initial crate defines the embedding ABI, provider-host receipts, structural
value carriers, explicit event-loop pumping, output capture, deterministic
shutdown, and Rust-substrate rejection.

The native adapter binds this facade to Bun/JSC internals and has a real linked
integration flow for source module load, synchronous export calls, async export
parking/resolution, structured provider errors, event-loop pumping, and
shutdown. It captures Bun stdout/stderr into `OutputRecord`s. Full host-owned
log sink semantics and prepared bundle loading are tracked in ADR-2035 and
ADR-2034.

## Vendored Bun

Bun source is tracked at `vendor/bun`. The snapshot is created from upstream
Git history with `git archive`, so it excludes nested `.git` metadata and local
build artifacts. Bun build-time source dependencies needed by the Rust crates,
including `lolhtml`, are vendored under `vendor/bun/vendor`.

Update to a new upstream ref:

```sh
scripts/update-vendored-bun.sh <ref>
```

Verify the vendored snapshot:

```sh
scripts/verify-vendored-bun.sh
```

Prepare Bun's generated Rust inputs and check the reusable Rust runtime crates:

```sh
scripts/check-vendored-bun-rust.sh
```

That script runs Bun configure/codegen inside `vendor/bun`, rewrites generated
artifact identity to the pinned `BUN_SOURCE_COMMIT`, checks `bun_jsc` plus
`bun_runtime`, and type-checks the `native/` adapter with Bun's pinned nightly
toolchain.

## Native Adapter

`native/` contains the nightly-only adapter that implements `BunEmbeddingRuntime`
over vendored Bun/JSC crates. It is kept out of the default crate so downstream
users can depend on the stable facade without pulling Bun's build toolchain into
their normal Rust build.

Run native adapter integration tests against Bun's C++/JSC objects:

```sh
scripts/prepare-native-bun-link.sh
LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 test --manifest-path native/Cargo.toml
```

The native link manifest intentionally records Bun's C/C++ object archive and
prebuilt WebKit/JSC static libraries, but not Bun's Rust staticlib. The adapter
depends on the vendored Rust crates directly so Rust global state is not linked
twice into the test host.
