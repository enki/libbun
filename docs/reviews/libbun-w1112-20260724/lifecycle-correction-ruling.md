# Lifecycle correction-2 ruling

Verdict: BUNDLE REVISE

Verdict commit: 7a5bfc1cf71299681a9edfb8d4f5a8a7501494e1. That commit intentionally has tree 6da13ed79ca5df4554b7c0bf3c89cde7d9dcea0d, identical to its parent ec6a7f249120a833aeaa4e0211fe0f41d17e0565; the exact commit object is preserved beside this record.

The determining omission is missing exact vendored JSC lifecycle authority and missing caller/fixture topology. Correction 2 must bind the Rust VirtualMachine and JSGlobalObject sources, the VM termination/reset wrapper, the C++ termination/reset/deinit implementations, the empty JSC__VM__deinit body, initialization and event-loop drain paths, and the full process/Drop/shutdown caller report. It must also bind the external Swarm producer, host, process, shutdown, and hostile fixture graph at adjacent source SHA 95323ff17cb29928e31467f651ef03bae2099c14.

The preserved lifecycle law is typed custody: private DriveCustody and InvocationReadyProof govern same-worker ready settlement; RetirementProof alone authorizes restart after exact death and complete containment/output drain; private RetirementQuarantine<Purpose> transfers by value into DurableReaper before any bounded public fault; one affine QuarantineCompletionClaim<Purpose> owns private recovery; RetiredDisposal consumes recovered custody; shutdown consumes the backend. Cooperative interrupt may clear a JSC termination request and reuse the epoch only after the exact invocation, persistent output pumps, microtasks, diagnostics, and finalization barrier are proven drained. The attached C++ source proves JSC__VM__deinit is empty, so it cannot serve as retirement proof or OS/process reaping.

Drop remains nonblocking and silent: it may only publish intact preallocated custody to the durable queue. It may not wait, join, abort, allocate, spawn, mint a terminal, expose a quarantine identity, or return a backend husk. The corrected prompt must request typed custody across cancellation, deadline, unwind, reaper panic/retry, claim abandonment, shutdown conversion, and exact restart, with hostile default-parallel tests and a commit-grade replacement.
