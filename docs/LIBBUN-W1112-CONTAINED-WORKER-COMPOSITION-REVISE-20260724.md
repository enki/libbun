# W1-11/W1-12 Contained Worker Composition Review

- Review date: 2026-07-24
- Current-main base: `2022bafe5fbc37c52293831db3f87aff55b1a399`
- Preserved candidate tip: `c477240da154ff2cefb6643a47445317756ac246`
- Implementation under review: `2383773bea5af06c6aead3f55bbe549d7161e78e`
- Review Lane: `libbun-contained-worker-composition-review-20260724`
- Verdict: `REVISE`

## Preserved stack composed

The complete preserved sequence was cherry-picked in order from
`origin/review/libbun-prepared-export-contained-worker-checkpoint-20260724`:

1. `1d5a7852bf114971fb17b8233a3cc05c38ad8945`
2. `15316977adc03ca85f58a275ab081bf0ff4d9c95`
3. `7bdba3f099293bcdb1619d2e1ef83d23b7c94a6a`
4. `fb170c0469da16fb6979654719525c3c6b7e8a8b`
5. `301d3b3787835f7634c86f5a71b59dc696d38746`
6. `1698a76c250e653826e35668391709039bd63e8e`
7. `2383773bea5af06c6aead3f55bbe549d7161e78e`
8. `c477240da154ff2cefb6643a47445317756ac246`

The resulting source tree is byte-identical to the preserved candidate tip
before this review evidence and the formatter-only trailing-blank-line repair.

## Approved portions

The public retained surface is affine and mechanically closed:

- `BunProviderBackend`, `PreparedExport`, and terminal continuations are
  consuming, non-cloneable owners.
- `SelectedProviderPackage` and `ProviderInvocation` have private fields, no
  public constructors, no parts projections, no serde, and no production raw
  mint. The only mint is the `#[cfg(test)]` owner fixture.
- The old borrowed invocation lease/descriptor/outcome family is deleted.
- No public worker path, process id, factory, protocol frame, callback proof,
  or selected-input getter was added to the retained surface.
- Authored fulfilled/rejected bytes remain mechanical cargo; no Swarm input
  producer or W1-13 semantic settlement was added.
- Public `open` resolves a private sibling worker and private Bubblewrap path;
  containment is an internal mechanical admission step.
- The common-path tests prove deadline-over-interrupt selection, contained
  kill/reap before cancel/deadline publication, exact-path restart, output
  quiescence fault dominance, consuming shutdown, and ready continuation reuse.

## Blocking custody defects

### 1. Fallible retirement uses forbidden blocking waits

`src/retained_backend.rs` contains unconditional `Child::wait()` at candidate
lines 1200, 1208, 1217, 1232, 1246, and 1555. Five are partial-admission
cleanup paths; the sixth is the forced cancellation/deadline retirement path.
The checked-in positive Edit Gate explicitly names `Child::wait` as a tripwire.

The forced path kills the namespace leader and then blocks without a bounded
poll owner. It subsequently joins both pumps unconditionally. A retained pipe,
wait fault, or unfinished reader can therefore block the worker owner or its
single durable-reaper thread indefinitely instead of preserving each
unfinished obligation in typed retirement custody.

### 2. `ContainedProcess::drop` performs fallible blocking cleanup and erases it

`ContainedProcess::drop` calls `force_terminate()` and discards its result.
That Drop can kill, wait, and join. If kill, wait, pipe EOF, or join fails, the
error is erased and Rust drops the remaining `Child`/join fields. The caller
can then observe a fault, disconnect, or restartable continuation without an
exact retirement proof or a durable adoption that still owns every unresolved
obligation.

### 3. Caller-side unwind and reaper disconnect do not transfer complete custody

`PreparedExport::drive` moves `WorkerCustody` into the `catch_unwind` closure.
`WorkerCustody` has no Drop/adoption guard. A panic in the caller-side drive
supervisor drops its command sender and `JoinHandle`; the owner thread is
detached rather than moved into the durable reaper before the public unwind
terminal is constructed. The existing unwind test covers a panic in the
runtime-owner thread followed by channel disconnect, not this caller-side
guard.

Likewise, `adopt_for_disposal` maps `Sender::send(worker)` failure to a string.
The returned `SendError<WorkerCustody>` is then dropped, which detaches the
join handle. The process-local `OnceLock<Result<Sender<_>, _>>` cannot replace
a reaper whose receiver has failed. These paths do not retain complete
supervisor custody.

The three defects are one owner problem. Replacing only `wait()` with a polling
loop would still leave error-erasing Drop and detached unwind custody.

## Clean replacement owner

Create one private `RetirementCustody` owner and one preallocated
`DurableReaperNode` before a live worker can be admitted.

`RetirementCustody` owns, without public getters or parts:

- the exact Bubblewrap namespace leader `Child`;
- stdin, response, and diagnostic pipes;
- response/diagnostic receivers;
- all pump and worker `JoinHandle`s;
- the private exact-path `WorkerFactory` continuation;
- the first and subsequent typed retirement faults; and
- its preallocated durable queue node.

Its foreground poll uses only `Child::try_wait`, nonblocking channel polls, and
`JoinHandle::is_finished`; it takes a child or join only after successful reap
or finished join. A deadline with unfinished custody moves the whole owner into
the preallocated node. `DurableReaper::adopt` publishes that node before a
terminal is constructed and cannot fail by returning the custody to Drop.

`WorkerCustody`, `PreparedExport`, terminal continuation, and the drive
supervisor each contain an armed private disposal guard. Normal transitions
disarm it only after moving all fields into Ready, Restartable, or
`RetirementCustody`. Unwind and Drop can then perform one infallible,
nonblocking publication of the already-prepared node. No Drop kills, waits,
joins, allocates, formats an error, or fabricates a terminal.

The public algebra remains unchanged for this bounded checkpoint. Clean
retirement may produce a private Restartable continuation. Unresolved
retirement produces the existing typed mechanical fault with no continuation
only after durable adoption. Bubblewrap remains a mechanical containment
mechanism and never becomes a selector, proof supplied by the caller, or public
authority product.

## Exact first source edits

1. Immediately before `WorkerCustody`, add private `RetirementCustody`,
   `DurableReaperNode`, and an armed `WorkerDisposalGuard`. Construct the node
   before spawning the worker thread/process and store it in every live owner.
2. Replace `DURABLE_REAPER: OnceLock<Result<Sender<WorkerCustody>, String>>`
   with a queue owner whose node publication is infallible after admission.
   Reaper thread start/wake failure must leave the published node in queue
   ownership for retry; it must not return `WorkerCustody` through
   `SendError`.
3. Change `drive_worker` to operate through the armed guard. On
   `catch_unwind`, consume the guard into `RetirementCustody`, publish/adopt it,
   and only then construct `SupervisorUnwind` with no continuation.
4. Delete all six `Child::wait()` calls and `ContainedProcess::drop` cleanup.
   Partial admission and forced termination both move the same process/pump
   fields into `RetirementCustody`; only its polling owner may mark fields
   complete.
5. Replace unconditional worker/pump joins with `is_finished` followed by
   `join`. Keep unfinished handles in custody. A join panic is a typed fault but
   does not authorize dropping another unfinished obligation.
6. Mint `Continuation::Restartable` only after leader reap, namespace
   emptiness/pipe EOF, channel closure, and every join are proved. Preserve the
   private exact helper/Bubblewrap paths only inside that continuation.

## Required hostile correction tests

Add private fault-injection traits around child status/termination and thread
completion; do not add public test constructors, getters, or authority parts.

- `caller_drive_unwind_adopts_complete_worker_before_terminal`: panic after
  dispatch command publication while the helper is blocked; prove the durable
  node owns child and join custody before the unwind terminal exists.
- `transient_try_wait_fault_retains_child_until_reap`: inject a status fault,
  then successful reap; prove the first fault dominates and no field was
  dropped early.
- `permanent_try_wait_fault_publishes_retirement_custody`: foreground deadline
  adopts the exact child/pipes/joins and returns no continuation.
- `descendant_held_pipe_prevents_restart_until_eof_and_join`: keep stdout or
  stderr open after leader death; prove no Restartable/Cancelled/Deadline
  terminal is published until EOF and joins complete, or adoption dominates.
- `pump_join_unfinished_is_never_taken`: an unfinished reader remains in
  custody across foreground timeout and reaper retry.
- `partial_admission_failure_adopts_spawned_child`: inject each missing-pipe and
  reader-thread-spawn failure and prove the spawned child is reaped or durably
  owned before `open` returns its typed fault.
- `reaper_receiver_failure_retains_published_node`: fail wake/thread start and
  prove a later reaper generation drains the same node exactly once.
- `drop_is_nonblocking_and_publishes_once`: Drop returns without kill/wait/join
  and the queue owns exactly one complete item.
- Repeat contained cancel, deadline, restart, consuming shutdown, and the 12
  existing owner tests under default parallelism.

## Executed candidate evidence

The following candidate gates passed and remain useful regression evidence,
but they do not cover the rejected fault shapes:

- focused retained owner: 12/12, Nextest run
  `bc802000-c846-4642-8c9e-8aed39453211`;
- external affine surface: 1/1, run
  `455ed0c9-7bc1-4a08-a299-51d211a7da19`;
- external privacy compile-fail: 2/2, run
  `02b16044-1e05-468b-bc3d-ad7a5cde7b68`;
- broad default-parallel suite: 41/41, run
  `9d22dd9f-38d9-4881-b554-f94b49ab763a`;
- `cargo check --locked`: passed;
- `cargo check --locked --features dynamic-loading`: passed;
- `cargo fmt --all -- --check`: passed.

The preservation stack also contained one trailing blank line rejected by
base-range `git diff --check`; this review removes only that mechanical blank
line. No semantic source correction or live deployment was performed.
