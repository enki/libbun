# libbun

`libbun` is the Rust ownership boundary for retained Bun provider backends and
worker-only prepared-export execution.

## Status

The implementation at
`d5d007e09b89eb3d7d23ba8380eb723b7bd6948d` is rejected and is not
release-eligible. Its source remains a fresh-worker, raw-constructor
implementation and its governing documents still require the proof and
quarantine corrections below. The earlier
`f1c450b042e4aa2c0c7abe05f9e95c86b8c1e697` tree remains historical negative
evidence only.

After the governing documents contain the frozen correction, source editing is
eligible for locked Step 1 only: delete the raw installer and its root
re-export. No positive lifecycle or release work is eligible before that poison
cut and its compile-fail tripwires land.

The frozen replacement contract is indexed in [docs/README.md](docs/README.md):

- [retained backend and prepared-export lifecycle](docs/LIBBUN-LIFECYCLE-CONTRACT.md);
- [worker invocation readiness, retirement, and quarantine](docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md);
- [worker build, package, and release](docs/LIBBUN-WORKER-RELEASE-CONTRACT.md).

## Frozen Product Boundary

`BunProviderBackend` is the highest owner. It consumes producer-minted,
generatively branded selected-package and invocation products. Private
`OfferCustody` owns the bounded offer; private `ReservedCustody` owns an
accepted but undispatched reservation; private `DriveCustody` begins only when
the dispatch permit and selected inputs are consumed.

`OfferReadyProof` governs refusal and unchanged retry.
`ReservationReleaseProof` proves that one exact reservation was closed before
dispatch, with no selected package or invocation transmitted, and permits
same-worker same-epoch reuse. `InvocationReadyProof` proves that one dispatched
invocation settled and drained while the same worker remains alive and Ready.
`RetirementProof` exclusively proves worker death and complete containment,
pipe, channel, pump, receiver, and join discharge.

Private `RetirementQuarantine<Purpose>` owns all unresolved retirement custody
until `DurableReaper::adopt` consumes it by value exactly once and publishes its
preallocated node into durable queue ownership. A concrete public quarantine
fault exposes only a bounded non-authoritative `QuarantineObservation`. When
backend recovery is meaningful, the fault privately seals exactly one opaque
affine `QuarantineCompletionClaim<Purpose>`. There is no public quarantine
identifier, selector, registry, receipt, backend husk, or custody handle.

JavaScript fulfillment and rejection are closed authored cargo. Undefined,
unserializable, missing-export, worker, protocol, containment, output, and
retirement failures are distinct typed faults.

The worker is a linked binary-only product. There is no public raw constructor,
parts projection, callback proof, quarantine id or lookup key, raw completion
receipt, in-process native entry point, plugin, dynamic loader, compatibility
fallback, unsafe `Send`/`Sync`, process-group containment fallback,
blocking/aborting Drop, or unlinked release mode.
