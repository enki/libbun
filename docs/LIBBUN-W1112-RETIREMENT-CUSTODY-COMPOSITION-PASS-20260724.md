# W1-11/W1-12 Retirement Custody Composition Review

- Verdict: `COMPOSITION PASS`
- Reviewed base: `2022bafe5fbc37c52293831db3f87aff55b1a399`
- Corrected source/evidence tip: `9c1b8f20b72562258cc5e0330a70b1b04cfe2dc9`
- Prior rejecting ruling: `ee63b1c427b928bce62865bb75a6c143236d6e04`
- Composed equivalent ruling patch: `6c81192c`
- Lane owner: `libbun-contained-worker-retirement-composition-review-20260724`
- Worktree: `/home/ubuntu/bridge-ops/dev-worktrees/libbun-contained-worker-retirement-composition-review-20260724`
- Cargo target: `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-16`

The prior ruling and its composed equivalent have the same stable patch ID,
`05ff8fbf2579e5b597067527a016e813db631cd8`. The correction closes the rejected
retirement defects without changing unrelated libbun behavior.

## Lifecycle ruling

Every admitted `WorkerCustody` owns a `DurableReaperNode` allocated before the
worker thread or contained process is admitted. Normal completion joins only a
finished worker. Cancellation, deadline, shutdown, caller unwind, and owner
Drop either complete retirement or synchronously move commands, restart
factory, retirement request, and join custody into the already allocated node
before a terminal fault is constructed.

`DurableReaperNode::publish` installs custody before a release publication to
the intrusive queue. Publication does not allocate and cannot return custody
through a failed channel send. A missing wake leaves the exact node reachable
from the queue for a later drain. The reaper takes a join handle only after
`JoinHandle::is_finished`; incomplete retirement is republished with the same
custody. Repeated publication is prevented by taking the worker's sole node.

The contained-process owner retains the namespace leader, stdin, response and
diagnostic readers, and their joins. Retirement uses `Child::try_wait`; reader
joins are taken only after both readers report finished. Cancellation and
deadline force retirement before a restart continuation is minted. Unresolved
foreground retirement is durably adopted and returns no continuation.
Consuming shutdown likewise completes or adopts before returning.

There is no unconditional `Child::wait()`, no cleanup `Drop` for
`ContainedProcess` or `RetirementCustody`, no detached `WorkerCustody`, and no
terminal custody publication that requires allocation, wait, join, formatting,
or fallible delivery.

## Executed evidence

- Focused retirement SCC: `ulimit -n 65536; cargo nextest run --locked --lib
  retained_backend::tests`; run `ec73086b-e18a-4507-a781-efc2f21afcc4`;
  16/16 passed.
- External affine surface: `cargo nextest run --locked --test
  prepared_export_lifecycle retained_owner_surface_is_affine_and_mechanically_closed`;
  run `b5ac52ad-451c-4fa6-bb9e-1087029afe89`; 1/1 passed.
- Broad default-parallel suite: `ulimit -n 65536; cargo nextest run --locked`;
  run `d45cfaf7-c681-441d-af4d-052699f2cdde`; 45/45 passed across seven
  binaries.
- Default owning check: `cargo check --locked`; passed.
- Dynamic-loading owning check: `cargo check --locked --features
  dynamic-loading`; passed.
- Formatter: `cargo fmt --all -- --check`; passed.
- Base-range whitespace gate: `git diff --check
  2022bafe5fbc37c52293831db3f87aff55b1a399..HEAD`; passed.

The focused run includes allocation refusal before live admission, missing-wake
publication and later exact-node drain, caller unwind after dispatch, runtime
unwind, nonblocking Drop, dropped prepared and continuation owners, repeated
contained cancellation, deadline reap and exact-path restart, normal shutdown,
shutdown failure, and unresponsive consuming-shutdown adoption.

## Static tripwires

- `rg -n '\.wait\s*\(' src/retained_backend.rs`: no matches.
- `rg -n 'impl\s+Drop\s+for\s+(ContainedProcess|RetirementCustody|DurableReaperNode)'
  src/retained_backend.rs`: no matches.
- Every production worker/pump `.join()` is dominated by that handle's
  `is_finished()` proof.
- All `WorkerCustody` occurrences are private and remain inside the affine
  backend/prepared/terminal/factory lifecycle.

No deployment was performed.
