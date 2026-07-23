# libbun

`libbun` is the one-shot mechanical boundary for driving an already-selected
Bun prepared export in a fresh worker process.

```rust
use std::time::Duration;
use libbun::{DriveControl, MechanicalTerminal, install_prepared_export};

let prepared = install_prepared_export(
    prepared_artifact_bytes,
    selected_export,
    opaque_invocation_bytes,
);
let terminal = prepared.drive(DriveControl::with_deadline_after(
    Duration::from_secs(30),
));

match terminal {
    MechanicalTerminal::Cargo(cargo) => consume(cargo.into_bytes()),
    MechanicalTerminal::Cancelled(_) => cancelled(),
    MechanicalTerminal::DeadlineElapsed(_) => deadline_elapsed(),
    MechanicalTerminal::MechanicalFault(fault) => mechanical_fault(fault.kind()),
}
```

`PreparedExport` is affine: it has private fields, cannot be cloned or
serialized, and `drive` consumes it. Each drive resolves the bundled sibling
`libbun-runtime-native` worker, spawns it in a private retirement boundary,
and returns only after normal exit or kill, reap, pipe closure, and supervisor
thread join. Cargo remains provisional until that retirement completes.

Libbun does not own provider contracts, TSON admission, authored result
interpretation, semantic settlement, reusable runtimes, module or promise
handles, event-loop controls, callbacks, plugin paths, or execution fallback.
The worker protocol is an unpublished implementation package.

The native worker requires Bun's pinned nightly and generated vendored inputs:

```sh
scripts/configure-vendored-bun.sh
cargo +nightly-2026-05-06 build --release --manifest-path runtime/Cargo.toml
```

Release bundles place `libbun-runtime-native` beside the host executable. A
missing or incompatible worker is a typed mechanical fault; execution never
falls back in process.
