# W1-11/W1-12 Exact-Contained Worker Boundary RED Evidence

Dual-Path Evidence:
- Phase: RED
- Source SHA: `301d3b3787835f7634c86f5a71b59dc696d38746`
- Missing semantic law: public `BunProviderBackend::open(config)` must privately
  resolve and admit the sibling `libbun-runtime-native` worker through exact
  Linux user/PID-namespace containment, while the backend retains complete
  protocol, interrupt, forced-retirement, reap, restart, shutdown, and Drop
  custody. Callers must not select an in-process runtime implementation.
- Rust fixture/test paths: `tests/prepared_export_lifecycle.rs`.
- Parallel Nextest command; exit code; result/diagnostic: `cargo nextest run
  --locked --test prepared_export_lifecycle
  retained_owner_surface_is_affine_and_mechanically_closed`; exit `101`;
  intended `E0283` at `BunProviderBackend::open(config)` proves that the public
  constructor still requires a generic `BunEmbeddingRuntime` selected by its
  caller.
- Real downstream fixture path: Swarm
  `tests/conformance/ss/provider/libbun_provider_value_json_v1_quiescence.test.ss`,
  preserved with executed RED evidence at Swarm commit
  `09b9bcd4b91c97f2876c1768e45b03700e72c59b` and pre-implementation source SHA
  `d854a8ce2aa98f4c0d9852c81bb98f198da0d886`.
- Lane-built downstream binary/result: preserved evidence records
  `/home/ubuntu/bridge-ops/cargo-slots/swarm-fe618e3e/slot-239/debug/swarm`,
  SHA-256
  `f97fd20958070af8d283c2e0135d9779fcba18f8091e4b05f0734ab24dba2355`;
  real `swarm test` reached provider delivery and then failed when in-process
  Bun aborted the authenticated worker with signal 6.
- Same-SHA proof (libbun RED): Lane identity, focused Rust compilation, and
  the absent exact-contained constructor diagnostic all name
  `301d3b3787835f7634c86f5a71b59dc696d38746`. The separately preserved
  downstream `.ss` RED names its exact Swarm source and Lane-built binary
  because libbun and Swarm are distinct repositories; the direct Swarm
  selected-input producer remains a separate prerequisite.
