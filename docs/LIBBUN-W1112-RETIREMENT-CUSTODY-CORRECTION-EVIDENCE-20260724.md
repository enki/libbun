# W1-11/W1-12 Retirement Custody Correction Evidence

- Review ruling: `ee63b1c427b928bce62865bb75a6c143236d6e04`
- Preserved implementation/evidence:
  `2383773bea5af06c6aead3f55bbe549d7161e78e` and
  `c477240da154ff2cefb6643a47445317756ac246`
- Current-main base: `2022bafe5fbc37c52293831db3f87aff55b1a399`
- Correction implementation: `20bfc9bab56aaece1efcd84240480d1288c48f98`
- Exact tested candidate: `bd3999d8e9240e0bc5eb9b83b4c11fd2b84b8528`

## Corrected custody law

Every live `WorkerCustody` now receives a `DurableReaperNode` before its owner
thread or contained process can be admitted. Worker, command, restart-factory,
and worker-join custody move into one private `RetirementCustody`. The node is
published through an intrusive atomic queue, so Drop and unwind publication do
not allocate, wait, join, format an error, or return custody through a failed
channel send.

The contained owner thread retains the exact namespace leader, stdin,
response/diagnostic channels, and pump joins. A shared retirement request
reaches an in-flight contained drive. Process retirement uses `Child::try_wait`
only. Pump and worker handles are taken only after `JoinHandle::is_finished`.
The first kill/status/pump fault is retained while all unfinished obligations
remain owned. Clean completion may mint the private exact-path restart;
foreground timeout publishes the whole unfinished worker hierarchy before the
typed adopted fault is constructed.

Partial admission follows the same polling rule. Caller-side drive unwind is
caught only after `WorkerCustody::drop` has synchronously published its already
allocated node. `ContainedProcess` has no cleanup Drop implementation.

## Hostile focused proof

- Command: `cargo nextest run --locked --lib retained_backend::tests`
- Result: exit `0`; 16/16 passed; run
  `359d7f39-b976-4e92-98b5-566bd26a0def`.
- Covered hostile cases: reaper-node allocation refusal before admission;
  missing reaper wake with later exact-node drain; caller drive unwind after
  command publication; nonblocking Drop publication; runtime unwind; repeated
  contained cancellation; deadline kill/reap/restart; Exit-unresponsive
  consuming shutdown adoption; shutdown failure; output-quiescence dominance;
  dropped prepared work; and dropped ready continuation.

## Surface and broad proof

- External affine surface: `cargo nextest run --locked --test
  prepared_export_lifecycle
  retained_owner_surface_is_affine_and_mechanically_closed`; exit `0`; 1/1;
  run `a96302a4-b84d-4520-894d-267d8551cc38`.
- Broad default-parallel suite: `ulimit -n 65536; cargo nextest run --locked`;
  exit `0`; 45/45 across seven binaries, including both sibling privacy
  compile-fail proofs; run `c4fe43f1-48dc-44b6-9141-dd85bf65b3db`.
- `cargo check --locked`: exit `0`.
- `cargo check --locked --features dynamic-loading`: exit `0`.
- `cargo fmt --all -- --check`: exit `0`.
- `git diff --check`: exit `0`.

## Tripwires

- `rg -n '\.wait\(' src/retained_backend.rs`: no matches.
- Every production `.join()` in `src/retained_backend.rs` is dominated by the
  same handle's `is_finished()` proof.
- No `Drop` implementation exists for `ContainedProcess` or
  `RetirementCustody`.
- Active retained source adds no public worker path, PID, protocol frame,
  callback, raw selected-input projection, clone, or serde authority surface.
- Swarm selected-input minting and W1-13 settlement remain unchanged and
  outside this correction.
