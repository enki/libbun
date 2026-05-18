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

The native adapter that binds this facade to Bun/JSC internals is the next
implementation layer.

## Vendored Bun

Bun source is tracked at `vendor/bun`. The snapshot is created from upstream
Git history with `git archive`, so it excludes nested `.git` metadata and local
build artifacts.

Update to a new upstream ref:

```sh
scripts/update-vendored-bun.sh <ref>
```

Verify the vendored snapshot:

```sh
scripts/verify-vendored-bun.sh
```
