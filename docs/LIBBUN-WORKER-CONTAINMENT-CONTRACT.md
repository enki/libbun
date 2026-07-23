# libbun Worker Invocation Readiness, Retirement, And Quarantine Contract

Status: frozen implementation contract

Date: 2026-07-23

## Owner And Custody

Persistent worker, undispatched reservation, dispatched invocation, retirement,
and quarantine are distinct private ownership layers.

```rust
struct RetainedWorkerCustody {
    containment: ExactContainment,
    child: Child,
    protocol: PersistentProtocol,
    diagnostics: PersistentDiagnostics,
    output: PersistentOutputPumps,
    supervisor: WorkerSupervisor,
    drop_node: PreallocatedQueueNode,
}

struct ReservedCustody<Brand> {
    backend: BackendContinuation,
    worker: RetainedWorkerCustody,
    reservation: InvocationReservation<Brand>,
    selected: SelectedPair<Brand>,
    dispatch_permit: DispatchPermit<Brand>,
}

struct DriveCustody<Brand> {
    backend: BackendContinuation,
    worker: RetainedWorkerCustody,
    reservation: InvocationReservation<Brand>,
    request: IoTask<RequestCompletion>,
    terminal: IoTask<TerminalCompletion>,
    invocation_output: InvocationOutputLedger,
    provisional: Option<ProvisionalTerminal<Brand>>,
    cancel: Option<CancelObservation>,
    per_invocation_joins: InvocationJoins,
}

struct RetirementCustody {
    backend: BackendContinuation,
    worker: RetainedWorkerCustody,
    outstanding_invocation: Option<ErasedInvocationCustody>,
    trigger: RetirementTrigger,
    faults: RetirementFaultJournal,
}
```

Request, terminal, output-generation, cancellation, and per-invocation join
authority are created only when dispatch consumes the dispatch permit. They do
not exist in `ReservedCustody`.

Every OS handle, pipe, receiver, channel, pump, task, and `JoinHandle` remains
owned until its exact obligation is successfully discharged. A fault never
causes an unconditional `take`, detach, or drop of another unfinished
obligation.

Reservation release may produce only `ReservationReleaseProof`,
`RetirementProof`, or `RetirementQuarantine<Purpose>`. Dispatched finalization
may produce only `InvocationReadyProof`, `RetirementProof`, or
`RetirementQuarantine<Purpose>`.

`RetirementQuarantine<Purpose>` privately owns all unresolved custody until
`DurableReaper::adopt` consumes it exactly once. A public quarantine fault never
owns unresolved custody.

## Exact Admission

The worker receives no selected package or invocation until all of these are
true:

- containment exists and is non-escapable for the supported platform;
- the exact child/leader is owned;
- inherited handles are allowlisted and all other handles are closed;
- request, terminal, diagnostic, and output pumps own their endpoints;
- completion channels and join handles are installed; and
- the worker/bootstrap has acknowledged the containment generation.

Admission failure before reservation returns a sealed refusal or fault
terminal. A retryable refusal consumes `OfferReadyProof`, proving that no
reservation was created and the same worker and epoch remain Ready.

The bounded offer carries only a private admission envelope. Reservation
allocates the exact worker/session slot but transmits no selected package or
invocation. `cancel_before_dispatch` closes the exact unused reservation and
must obtain `ReservationReleaseProof` before returning the same worker to
Ready. It never obtains or substitutes `InvocationReadyProof`. Consuming
`PreparedExport::drive` dispatches the selected inputs and permanently disables
`ReservationReleaseProof`.

## Platform Containment

Process groups are not containment and are forbidden as a fallback.

### Linux

The trusted worker bootstrap establishes a PID namespace before reading
selected work. The engine is PID 1 for that namespace or is supervised by its
trusted namespace PID 1. Killing namespace PID 1 kills all namespace members,
including `setsid`, double-fork, and nested-session descendants. The bootstrap
reaps namespace work and reports retirement before the host reaps the
bootstrap.

If the required namespace/user-namespace facility is unavailable, admission
returns `ContainmentUnavailable`. It does not fall back to a process group.

### macOS

The trusted worker installs an admitted sandbox before reading selected work or
initializing Bun. The sandbox denies process creation by selected code. A
parent-death watchdog and the directly owned worker child provide retirement.
Sandbox failure is an admission fault. There is no permissive process-group
mode.

### Windows

The host creates a non-breakaway Job Object with kill-on-close and an I/O
completion port. It creates the worker suspended with an explicit inherited
handle list, assigns it to the job, installs all custody, and only then resumes
the initial thread. Retirement requires leader completion plus
`ACTIVE_PROCESS_ZERO` before closing the job handle.

If atomic job admission is impossible under the host's job policy, admission
fails. `BREAKAWAY_OK`, `SILENT_BREAKAWAY_OK`, and spawn-before-assignment are
forbidden.

Other platforms are unsupported until they provide an equally exact primitive
and hostile proof.

## Bounded Release, Invocation Finalization, And Retirement

Reservation release, dispatched-invocation finalization, and worker retirement
have separate fixed budgets. All operations are nonblocking or
deadline-bounded.

Reservation release may produce `ReservationReleaseProof` only after all of:

- the exact reservation is closed and unreplayable;
- the dispatch permit remained unconsumed;
- no selected-package or invocation byte was enqueued or transmitted;
- no invocation request, terminal, promise, module, cancellation, output
  generation, or per-invocation join authority exists;
- capture never left IdleCapture and idle capture is empty;
- no release or reservation-teardown work remains; and
- the same worker is alive, reachable, contained, and Ready at the same epoch.

Dispatched invocation finalization may produce `InvocationReadyProof` only
after all of:

- the exact reservation is closed and unreplayable;
- request delivery and the invocation terminal sequence are complete;
- fulfilled/rejected cargo is one complete nonempty authored frame, or exact
  cooperative cancellation is acknowledged;
- no pending promise, module, or invocation authority remains;
- output flush and the sequence barrier are complete;
- the output ledger is sealed and no output entered IdleCapture after the
  barrier;
- no cancellation, interrupt, or invocation teardown work remains;
- every per-invocation task is complete and joined; and
- the same worker is alive, reachable, contained, and Ready at the same epoch.

Failure or ambiguity in either proof's predicates forces retirement. Neither
proof can substitute for the other.

Retirement uses:

- `Child::try_wait`, never unconditional `Child::wait`;
- nonblocking containment state polling;
- `Receiver::try_recv`, treating disconnect as a typed completion fault;
- pipe/event readiness with bounded poll time;
- `JoinHandle::is_finished`, with `join` only after true; and
- bounded backoff no later than the retirement deadline.

The finalization and retirement loops retain the first and subsequent typed
faults without discarding custody. A `Child` is removed only after successful
reap observation. A join handle is removed only after it is known finished and
joined. Invocation barriers, worker EOF, channel closure, and pump completion
are recorded separately.

Retirement quiescence requires all of:

- exact containment empty;
- leader reaped with observed exit;
- request and terminal writers closed;
- worker protocol and diagnostic EOF;
- persistent output pumps stopped after their final barriers and EOF;
- all persistent result channels closed;
- all invocation, supervisor, protocol, diagnostic, and pump threads joined;
  and
- no remaining child, containment, pipe, receiver, channel, pump, task, or join
  custody.

If the foreground retirement budget expires or an obligation remains
unresolved, the foreground moves all remaining custody into private
`RetirementQuarantine<Purpose>`. It does not mint a provisional terminal,
Ready-family proof, or `RetirementProof`.

## Durable Reaper And Completion Claims

The process-wide durable queue is initialized before any live custody can reach
a Drop or quarantine path. Every custody owner carries a preallocated queue
node. Queue custody is independent of reaper-thread liveness.

`DurableReaper::adopt`:

1. consumes `RetirementQuarantine<Purpose>` by value exactly once;
2. installs the intact item in its preallocated node;
3. publishes the node into durable queue ownership;
4. only after publication produces bounded observation and at most one private
   claim for an observed recovery or shutdown path;
5. produces no observation or claim for silent Drop adoption; and
6. performs best-effort wake or spawn only after publication.

There is no public or public-view quarantine id, UUID, number, path, process id,
epoch, index, node address, lookup key, registry, raw receipt, or selector.
Internal queue identity remains private and unobservable.

A private `QuarantineCompletionClaim<Purpose>` contains one generative entry
capability. It is affine, purpose-typed, non-cloneable, non-serializable, and
has no getter or parts projection. A concrete public terminal may privately
contain one such claim but publicly exposes only `QuarantineObservation`.
That observation has private fields, no clone, serde, or parts projection, and
cannot be supplied to any queue, poll, claim, restart, or shutdown operation.

The queue entry states are:

```rust
enum QueueEntryState {
    Pending(QueueOwnedItem, ClaimMode),
    Reaping(ReaperOwnedLease, ClaimMode),
    CompletedRecoverable(RestartableCustody, CompletionJournal),
    CompletedShutdown(CompletionJournal),
    Closed(CompletionJournal),
}
```

Pending `poll(self)` moves the same claim into the returned pending terminal.
After exact `RetirementProof`, a live recovery claim permits one atomic claim of
one `RestartableCustody`. A shutdown claim can produce only fault-complete
shutdown.

Dropping a recovery claim before completion marks recovery abandoned; after
`RetirementProof` the queue disposes without spawning. Dropping after completion
causes the queue to consume the stored `RestartableCustody`. Converting a
recovery claim to shutdown before completion changes its purpose atomically;
converting after completion atomically consumes the stored
`RestartableCustody`. No race can produce both a recovered terminal and a
queue-owned continuation.

The reaper:

- owns each in-flight item outside `catch_unwind`;
- requeues the exact item and unprocessed remainder after panic;
- retains items after wake, spawn, retry, or bounded-attempt failure;
- repeats containment termination, reap, EOF, channel, pipe, pump, and join
  polling;
- records only bounded private diagnostics; and
- deletes or transforms active-worker custody only after exact
  `RetirementProof`.

A quarantine terminal is a typed mechanical fault. It is not cancellation,
deadline, cargo, successful shutdown, or backend Ready proof.

## Output Pumps

Outer worker stdout, stderr, and protocol pumps start immediately after backend
startup and before selected work is offered. Native Bun stdout, stderr, and log
pumps start before their write descriptors are installed into Bun.

Each pump:

- drains continuously;
- retains at most the configured byte limit;
- records total bytes and overflow;
- after overflow, discards additional bytes while continuing to drain;
- supports a sequence barrier after Bun flush;
- distinguishes idle output from invocation output; and
- drains to EOF during retirement.

The forbidden shape is flush-then-read on a blocking pipe. Bun-facing write
descriptors must not be able to deadlock before a reader runs.

For a retained session, each invocation owns a capture generation:

```text
IdleCapture
  -> InvocationCapture(sequence)
  -> Bun flush
  -> pump barrier
  -> sealed invocation ledger
  -> IdleCapture
```

Output appearing in IdleCapture poisons the retained session before reuse.
Overflow is a typed fault and never stops the drain.

## Panic, Cancellation, Fault Dominance, And Drop

Supervisor unwind is caught only while custody remains owned outside the unwind
closure. It forces retirement and dominates every provisional cargo,
rejection, release, cancellation, deadline, Ready, or shutdown-success
candidate.

Pre-dispatch release becomes public only after `ReservationReleaseProof`.
Cooperative in-drive cancellation becomes public only after
`InvocationReadyProof`. Failed or ambiguous cooperative cancellation, forced
cancellation, and every deadline force retirement. Deadline wins a cancellation
tie observed at the same private poll point.

A finalization or retirement fault dominates the provisional trigger. If
foreground retirement later obtains `RetirementProof`, the result remains a
typed retired fault. If foreground retirement cannot obtain proof,
`DurableReaper::adopt` publishes the item and the concrete quarantine fault
dominates cargo, rejection, cancellation, deadline, unwind continuation, and
shutdown success. Reaper completion never retroactively fabricates the
displaced terminal.

Drop performs only silent adoption using the custody's preallocated node.
Publication allocates nothing, waits for nothing, joins nothing, calls no user
code, fabricates no proof or terminal, and precedes best-effort wake or spawn.
Drop of an already-adopted terminal abandons only its private completion claim.

## Hostile Proof

Required tests include:

- Linux `setsid`, double-fork, nested-session, and inherited-pipe descendants;
- macOS denied `fork`, `posix_spawn`, and descendant descriptor retention;
- Windows `CREATE_BREAKAWAY_FROM_JOB` and spawn-before-assignment races;
- request, terminal, stderr, Bun stdout/stderr/log, and output-barrier
  saturation;
- output overflow while continuing to drain;
- `try_wait` transient and permanent faults with child custody retained;
- containment termination transient faults;
- channel disconnect without a result;
- reader/writer/output thread panic and unfinished join;
- cargo candidate followed by hang, deadline, descendant, or retirement fault;
- cancellation/deadline ties and later retirement fault dominance;
- refused offer retry with `OfferReadyProof` and no reservation transfer;
- pre-dispatch release followed by `ReservationReleaseProof`, same-worker
  same-epoch reuse, and proof that no selected or request byte was dispatched;
- proof that `ReservationReleaseProof` cannot be minted after dispatch;
- fulfilled/rejected cargo and cooperative in-drive cancellation followed by
  `InvocationReadyProof`, same-worker same-epoch reuse, and a second invocation;
- proof that `InvocationReadyProof` cannot be minted without dispatch;
- forced cancellation and deadline followed by `RetirementProof`, no surviving
  worker, and restart at a new epoch;
- adoption publication before public fault construction;
- public quarantine faults containing observation and at most one private claim
  but no OS custody or public identity;
- pending poll preserving exactly one claim;
- completion-versus-claim atomic race and single recovery;
- claim Drop before and after completion;
- shutdown conversion before and after recoverable completion;
- shutdown-origin quarantine with no backend recovery;
- silent Drop adoption with no observation or claim;
- reaper spawn, wake, retry, and panic failure with exact item retention;
- abandonment and completed-custody disposal without spawning;
- proof that worker exit can never mint cargo or a Ready-family proof; and
- proof that no live descendant, zombie, pipe, receiver, channel, pump, or
  detached thread remains after `RetirementProof` or successful shutdown.
