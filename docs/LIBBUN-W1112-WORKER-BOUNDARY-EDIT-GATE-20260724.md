# W1-11/W1-12 Exact-Contained Worker Boundary Edit Gate

Status: active positive owner tranche

Source basis: `fb170c0469da16fb6979654719525c3c6b7e8a8b`

## Edit Gate

- Bucket: exact-contained retained worker boundary.
- First source edit: change public `BunProviderBackend::open(config)` from a
  generic in-process runtime constructor into private sibling-worker
  resolution plus exact Linux PID-namespace admission. Retain generic runtime
  construction only inside owner tests.
- Owner boundary: `BunProviderBackend` owns worker resolution, containment,
  child/process custody, protocol correspondence, persistent pipes, output
  drain, forced retirement, reap, EOF, join, restart, shutdown, and Drop.
- Selected input: the existing opaque co-branded `SelectedProviderPackage` and
  `ProviderInvocation`; this tranche does not add any mint or RAW projection.
- Output product: unchanged closed `MechanicalTerminal` algebra. Authored cargo
  remains opaque mechanical cargo; Swarm settlement is out of scope.
- Containment: on Linux the worker is admitted only through Bubblewrap user and
  PID namespaces with parent-death coupling. Missing or refused namespace
  admission is a typed admission fault before selected work can be sent.
- Interrupt/deadline: cooperative selection is observed first; a retained
  process that cannot cooperatively answer is killed through its namespace
  leader, then reaped and joined before `Cancelled` or `DeadlineElapsed` is
  published. Incomplete proof produces a dominating typed fault.
- Drop/shutdown: the existing durable reaper receives the complete process
  supervisor custody. Drop publishes no terminal or restart authority.
- Forbidden shapes: public worker path, environment/path override as authority,
  process id getter, protocol frame getter, process-group fallback, callback
  proof, borrowed shutdown, raw selected-input mint, or W1-13 settlement.

## Direct Producer Boundary

The Swarm-selected input producer remains a separate direct producer. Its
required API is a consuming operation on
`DurableExternalProviderInvocationAuthority` that returns one opaque libbun
admission product without exposing contract/path/export/input parts to a
sibling caller. No such producer is implemented in this libbun tranche.
