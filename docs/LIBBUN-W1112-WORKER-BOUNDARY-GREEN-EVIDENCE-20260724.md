# W1-11/W1-12 Exact-Contained Worker Boundary GREEN Evidence

This is the libbun mechanical worker-boundary checkpoint. The separate Swarm
selected-input producer and real `.ss` GREEN remain pending; W1-13 settlement
is not implemented here.

Dual-Path Evidence:
- Phase: GREEN (libbun worker boundary); downstream `.ss` GREEN pending.
- Source SHA: `2383773bea5af06c6aead3f55bbe549d7161e78e`.
- Implemented semantic law: public `BunProviderBackend::open(config)` privately
  resolves the exact sibling `libbun-runtime-native` and admits it only through
  Bubblewrap user/PID namespaces with parent-death coupling. The affine backend
  owns protocol correspondence, persistent stdin/stdout/stderr, output drain,
  interrupt/deadline forced retirement, wait/reap/join, exact-path restart,
  consuming shutdown, and Drop adoption. The generic in-process owner exists
  only in owner unit tests.
- Rust fixture/test paths: `src/retained_backend.rs`, `src/lib.rs`,
  `tests/prepared_export_lifecycle.rs`, `tests/public_api_boundary.rs`, and
  `tests/fixtures/prepared_export_privacy/`.
- Focused parallel Nextest command; exit code; result: `cargo nextest run
  --locked --lib retained_backend::tests`; exit `0`; 12/12 passed. The hostile
  contained-process proof uses framed persistent IPC through real Bubblewrap,
  blocks the helper during selected work, and proves both cancellation and
  deadline kill/reap before typed terminal publication plus exact-path restart
  and consuming shutdown.
- External surface command; exit code; result: `cargo nextest run --locked
  --test prepared_export_lifecycle
  retained_owner_surface_is_affine_and_mechanically_closed`; exit `0`; 1/1
  passed. `BunProviderBackend::open(config)` is non-generic at the sibling
  boundary.
- Broad parallel Nextest command; exit code; result: `ulimit -n 65536; cargo
  nextest run --locked`; exit `0`; 41/41 passed across seven binaries,
  including both privacy compile-fail tests.
- Containment runtime: canonical `/usr/bin/bwrap`, SHA-256
  `52231e1caf55bcbc667b269f49c63599a6f7db4767ae6a039580d0ff853db712`,
  invoked with `--die-with-parent`, user namespace, PID namespace, fresh
  `/proc`, fresh `/dev`, read-only root bind, exact working directory, and no
  process-group fallback.
- Owning checks: `cargo fmt --all -- --check`, `git diff --check`, `cargo check
  --locked`, and `cargo check --locked --features dynamic-loading`; all exited
  `0` at the source SHA.
- Old/public-family search: active `src/` and the external lifecycle surface
  are clean for the deleted lease/descriptor/profile family and
  `BunProviderBackend::open::<...>`. `src/retained_backend.rs` adds no public
  worker path, PID, protocol frame, selector getter, callback, or environment
  override.
- Native consumer check: `cargo check --locked --manifest-path
  native/Cargo.toml` compiled the changed root `libbun` crate and then stopped
  only at the pre-existing vendored Bun code-generation wall:
  `build_options.rs`, `generated_classes.rs`, and `cpp.rs` are absent. No
  diagnostic was attributable to the changed worker graph.
- Runtime consumer check: `cargo check --locked --manifest-path
  runtime/Cargo.toml` stops before compilation because the committed runtime
  lockfile already requires an update. No lockfile or source was changed.
- All-features check: `cargo check --locked --all-features` stops only at the
  existing `download-plugin` gate because v0.2.3 has no committed x86_64 Linux
  plugin checksum.
- `.ss` fixture path: Swarm
  `tests/conformance/ss/provider/libbun_provider_value_json_v1_quiescence.test.ss`
  remains the preserved RED downstream path. Swarm must first add the consuming
  owner operation on `DurableExternalProviderInvocationAuthority` that mints
  one opaque libbun admission product, pin this libbun candidate, and build the
  real worker package. This checkpoint does not expose RAW selected-input
  parts and does not claim `.ss` GREEN.
- Post-SHA proof: the exact SHA printed and asserted before the focused owner,
  external surface, broad privacy/default, formatter, default check, dynamic
  check, native consumer, search, and clean-status gates was
  `2383773bea5af06c6aead3f55bbe549d7161e78e`.
