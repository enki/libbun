# libbun Worker Containment, Retirement, And Quarantine Contract

Status: frozen implementation contract

Date: 2026-07-23

## Owner And Custody

One admitted worker drive is owned by private `DriveCustody`:

```rust
struct DriveCustody {
    backend: BackendContinuation,
    containment: ExactContainment,
    child: Child,
    request: IoTask<RequestCompletion>,
    terminal: IoTask<TerminalCompletion>,
    diagnostics: IoTask<DiagnosticCompletion>,
    output: OutputPumps,
    provisional: Option<ProvisionalTerminal>,
    observed_exit: Option<ExitStatus>,
}
```

Every OS handle, pipe, receiver, and `JoinHandle` remains in this object until
the individual obligation is successfully discharged. A fault never triggers
an unconditional `take` or drop of another unfinished handle.

The only consuming results are:

```rust
enum RetirementDisposition {
    Proved(RetirementProof),
    Quarantined(RetirementQuarantine),
}
```

`RetirementProof` contains the sealed facts needed to mint a terminal.
`RetirementQuarantine` contains the entire remaining `DriveCustody`, its typed
faults, trigger, generation, and quarantine identity.

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
through a sealed refusal or fault terminal. It cannot lose selected work.

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

## Bounded Retirement

Foreground retirement has a fixed budget independent of the drive deadline.
All operations inside it are nonblocking or deadline-bounded:

- `Child::try_wait`, never unconditional `Child::wait`;
- nonblocking containment state polling;
- `Receiver::try_recv`, treating disconnect as a typed completion fault;
- pipe/event readiness with bounded poll time;
- `JoinHandle::is_finished`, with `join` only after true; and
- bounded backoff no later than the retirement deadline.

The poll loop retains the first and subsequent typed faults without discarding
custody. A `Child` is removed only after successful reap observation. A join
handle is removed only after it is known finished and joined. EOF, channel
closure, and output barrier state are recorded separately.

Quiescence requires all of:

- exact containment empty;
- leader reaped with observed exit;
- request writer complete and request pipe closed;
- one complete terminal candidate or typed terminal protocol fault;
- terminal and diagnostic EOF;
- output pumps stopped after their final barriers and EOF;
- result channels completed or typed disconnected;
- all supervisor/pump threads joined; and
- no remaining child, containment, pipe, receiver, or join custody.

If the foreground budget expires, the loop transfers intact custody to
quarantine. It does not block longer, detach a thread, abort, or mint the
provisional terminal.

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
- deletes the item only after every custody obligation is discharged.

A quarantine terminal is a typed mechanical fault. It is not cancellation,
deadline, cargo, successful shutdown, or backend Ready proof.

## Output Pumps

Outer worker stdout, stderr, and protocol pumps start immediately after spawn
and before selected work is sent. Native Bun stdout, stderr, and log pumps start
before their write descriptors are installed into Bun.

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
outside the unwind closure. The result is retirement or quarantine plus a typed
fault.

Cancellation and deadline are private triggers. They request cooperative
interrupt first when the retained protocol can prove Ready afterward, then
force exact containment termination when required. Neither becomes public
evidence until `RetirementProof` exists.

Drop performs only an infallible ownership transfer into the already durable
queue. It does not wait, join, close a live containment prematurely, call user
code, or call `process::abort`.

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
- owner unwind after admission;
- Drop of prepared, admitted, Ready, Poisoned, and ShuttingDown owners;
- reaper thread spawn, wake, panic, retry, and process teardown behavior; and
- proof that no live descendant, zombie, pipe, or detached thread remains after
  a successful terminal.
