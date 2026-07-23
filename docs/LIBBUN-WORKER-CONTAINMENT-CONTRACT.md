# libbun Worker Invocation Readiness, Retirement, And Quarantine Contract

Status: frozen implementation contract

Date: 2026-07-23

## Owner And Custody

Persistent worker custody and one admitted invocation are distinct private
ownership layers:

```rust
struct RetainedWorkerCustody {
    containment: ExactContainment,
    child: Child,
    protocol: PersistentProtocol,
    diagnostics: PersistentDiagnostics,
    output: PersistentOutputPumps,
    supervisor: WorkerSupervisor,
}

struct DriveCustody {
    backend: BackendContinuation,
    worker: RetainedWorkerCustody,
    reservation: InvocationReservation,
    request: IoTask<RequestCompletion>,
    terminal: IoTask<TerminalCompletion>,
    invocation_output: InvocationOutputLedger,
    provisional: Option<ProvisionalTerminal>,
    cancel: Option<CancelObservation>,
}
```

Every OS handle, pipe, receiver, and `JoinHandle` remains in these ownership
layers until the individual obligation is successfully discharged. A fault
never triggers an unconditional `take` or drop of another unfinished handle.

The only consuming finalization results are:

```rust
enum DriveDisposition<Brand> {
    Ready(InvocationReadyProof<Brand>),
    Retired(RetirementProof),
    Quarantined(RetirementQuarantine),
}
```

`InvocationReadyProof` consumes the exact invocation custody and returns its
same still-live `RetainedWorkerCustody` sealed inside a Ready backend
continuation. `RetirementProof` consumes the worker custody only after death
and complete teardown and returns a `Restartable` continuation with no worker.
`RetirementQuarantine` contains the entire remaining active custody—whether
`DriveCustody`, `RetainedCustody`, abandoned prepared custody, or shutdown
custody—plus its typed faults, trigger, generation, and quarantine identity.

No public terminal chooses among these outcomes. Private finalization selects
the only outcome proved by current custody. The proof values have no public
fields, parts, clone, serde, or raw observation path.

## Exact Admission

The worker receives no selected package or invocation until all of these are
true:

- containment exists and is non-escapable for the supported platform;
- the exact child/leader is owned;
- inherited handles are allowlisted and all other handles are closed;
- request, terminal, diagnostic, and output pumps own their endpoints;
- completion channels and join handles are installed; and
- the worker/bootstrap has acknowledged the containment generation.

Admission failure before this point returns the same backend/package/invocation
through a sealed refusal or fault terminal. A retryable refusal additionally
consumes `OfferReadyProof`, which proves that no reservation was created and
the same worker/session epoch remains Ready. It cannot lose selected work.

The bounded offer carries only a private admission envelope. Reservation
allocates the exact worker/session slot but does not transmit the selected
package or invocation. Only consuming `PreparedExport::drive` dispatches those
sealed inputs. `cancel_before_dispatch` closes the unused reservation and must
obtain `InvocationReadyProof` before returning the same worker to Ready.

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

## Bounded Invocation Finalization And Retirement

Invocation finalization and worker retirement have separate fixed budgets,
both independent of the drive deadline. All operations inside them are
nonblocking or deadline-bounded.

Invocation finalization may produce `InvocationReadyProof` only after all of:

- the exact reservation is closed and cannot be replayed;
- request delivery and the invocation terminal sequence are complete;
- fulfilled/rejected cargo is one complete nonempty authored frame, or the
  exact cooperative cancellation is acknowledged;
- the worker reports no pending promise, module, or invocation authority;
- the invocation output flush and sequence barrier are complete;
- the sealed output ledger is complete and no output entered IdleCapture after
  the barrier;
- no cancellation, interrupt, or invocation teardown work remains;
- all per-invocation tasks are complete and joined; and
- the same worker is alive, reachable, contained, and reports the same session
  epoch in Ready state.

Persistent worker protocol, diagnostic, output-pump, child, and containment
custody remain live inside the returned Ready backend. They are not closed or
joined to prove invocation readiness. Failure or ambiguity in any readiness
condition forces worker retirement; it cannot be converted into Ready.

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
- no remaining child, containment, pipe, receiver, or join custody.

If the foreground budget expires, the loop transfers intact custody to
quarantine. It does not block longer, detach a thread, abort, or mint the
provisional terminal, `InvocationReadyProof`, or `RetirementProof`.

## Durable Reaper

The process-wide reaper queue is initialized before any Drop submission can
lose work. Queue custody is independent of whether a reaper thread is currently
running.

```rust
enum QuarantineItem {
    Drive(DriveCustody),
    Retained(RetainedCustody),
    AbandonedPrepared(AbandonedPreparedCustody),
    Shutdown(ShutdownCustody),
}
```

Submission consumes the item and returns a sealed `QuarantineId`. Failure to
spawn or wake the reaper leaves the item in the queue. Every later public owner
operation retries reaper activation.

The reaper:

- takes one item without holding the queue lock while polling it;
- keeps ownership outside `catch_unwind` and requeues after a panic;
- uses bounded attempts and backoff;
- repeats containment termination, reap, EOF, channel, and join polling;
- records bounded terminal diagnostics by quarantine identity; and
- deletes active-worker custody only after producing `RetirementProof`, and
  deletes already-retired custody only after verifying its sealed proof.

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

## Panic, Cancellation, And Drop

Supervisor unwind is caught only around operations whose custody remains owned
outside the unwind closure. It forces retirement. The result is a typed unwind
fault after `RetirementProof`, or quarantine plus a typed fault; unwind never
produces `InvocationReadyProof`.

Cancellation and deadline are private triggers. They request cooperative
interrupt first. Exact cooperative acknowledgement may produce cancellation
only after `InvocationReadyProof`. Failed, ambiguous, or forced cancellation
and every deadline force exact containment termination and become public only
after `RetirementProof`. The former returns the same live Ready worker; the
latter return only a `Restartable` continuation with no live worker.

Drop performs only an infallible ownership transfer into the already durable
queue. It does not wait, join, close a live containment prematurely, call user
code, call `process::abort`, or fabricate either proof. The reaper retires any
live worker before deleting its custody.

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
- fulfilled and rejected cargo followed by `InvocationReadyProof`, same-epoch
  worker reuse, and a second invocation;
- pre-dispatch reservation cancellation and cooperative in-drive cancellation
  followed by `InvocationReadyProof` and same-worker reuse;
- forced cancellation and deadline followed by `RetirementProof`, no surviving
  worker, and replacement-worker restart at a new epoch;
- proof that worker exit can never mint fulfilled/rejected cargo or a Ready
  continuation;
- owner unwind after admission;
- Drop of prepared, admitted, Ready, Poisoned, and ShuttingDown owners;
- reaper thread spawn, wake, panic, retry, and process teardown behavior;
- proof that an invocation-ready terminal retains exactly the same contained
  worker and no per-invocation authority or late output; and
- proof that no live descendant, zombie, pipe, or detached thread remains after
  `RetirementProof` or successful shutdown.
