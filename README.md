# libbun

`libbun` is the Rust ownership boundary for retained Bun provider backends and
worker-only prepared-export execution.

## Status

The implementation at
`f1c450b042e4aa2c0c7abe05f9e95c86b8c1e697` is rejected and is not
release-eligible. It removed the required retained backend and did not prove
bounded worker retirement, exact containment, Rust privacy, or an executable
worker release.

The frozen replacement contract is indexed in [docs/README.md](docs/README.md):

- [retained backend and prepared-export lifecycle](docs/LIBBUN-LIFECYCLE-CONTRACT.md);
- [worker invocation readiness, retirement, and quarantine](docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md);
- [worker build, package, and release](docs/LIBBUN-WORKER-RELEASE-CONTRACT.md).

## Frozen Product Boundary

`BunProviderBackend` is the highest owner. It consumes producer-minted,
generatively branded selected-package and invocation products. An admitted
one-shot invocation becomes an affine `PreparedExport`; private `DriveCustody`
owns all invocation and worker custody until bounded `InvocationReadyProof`,
`RetirementProof`, or intact `RetirementQuarantine`.

`InvocationReadyProof` proves one invocation settled and drained while the same
worker remains alive and reusable. It is required for fulfilled/rejected cargo
and cooperative cancellation. `RetirementProof` exclusively proves worker
death and complete containment teardown; it is required for forced
cancellation, deadline, unwind, and active-worker shutdown and cannot return the
same live session. `RetirementQuarantine` retains unresolved retirement custody.
JavaScript fulfillment and rejection are closed authored cargo. Undefined,
unserializable, missing-export, worker, protocol, containment, output, and
retirement failures are distinct typed faults.

The worker is a linked binary-only product. There is no public raw constructor,
parts projection, callback proof, in-process native entry point, plugin,
dynamic loader, compatibility fallback, unsafe `Send`/`Sync`, process-group
containment fallback, blocking/aborting Drop, or unlinked release mode.

Do not implement against the rejected public API shown by the current source.
Implementation starts with the poison and owner-move order recorded in the
[decision and handoff index](docs/README.md#fifteen-step-hard-cut-order).
