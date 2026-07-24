POISON CUT PASS

# libbun W1-11/W1-12 raw installer poison composition review

Date: 2026-07-24

## Reviewed identities

- Candidate commit: `6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb`
- Candidate tree: `cb964de8ab8162449fbe95959bf34d231570aa5c`
- Exact parent: `5b14f8d0599e40630f788cc3863d0b3d96116199`
- Candidate subject: `poison raw prepared-export installer`
- Frozen ruling commit: `b046f85a3dd41ac86cabed2de6391876ea77c0f4`
- Frozen ruling path:
  `docs/LIBBUN-W1112-FINAL-COMPOSITION-REVIEW-20260724.md`
- Frozen ruling blob: `a95e33e499a6da7ef529490c2e77546642ba1304`
- Frozen ruling content SHA-256:
  `3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6`
- Review Lane owner: `libbun-w1112-poison-installer-review-20260724`
- Review worktree:
  `/home/ubuntu/bridge-ops/dev-worktrees/libbun-w1112-poison-installer-review-20260724`
- Review `CARGO_TARGET_DIR`:
  `/home/ubuntu/bridge-ops/cargo-slots/libbun-5bf6685e/slot-8`

The Lane resolved to the exact worktree and Cargo target above, was detached at
the exact candidate, and was clean before review. The candidate has the exact
parent recorded above.

## Verdict

The candidate exactly executes the first `POISON` source edit ordered by the
frozen ruling:

1. `install_prepared_export(Vec<u8>, String, Vec<u8>)` is deleted from
   `src/prepared_export.rs`.
2. Its crate-root reexport is deleted from `src/lib.rs`.
3. An isolated external sibling crate proves both import and fully qualified
   call denial for the intended Rust privacy/API-absence reasons.
4. No replacement constructor, alias, fallback, fixed error, empty/default
   cargo, selector, parts product, or callback was added.

This verdict approves only the required poison cut. It does not treat the
remaining fresh-process implementation as the W1-11/W1-12 positive owner move.
The next tranche remains the ruling's opaque, by-value `BunProviderBackend`
with branded `SelectedProviderPackage` and `ProviderInvocation`, followed in
the frozen fifteen-step order.

## Complete diff and stale-aperture audit

The candidate changes nine files: one evidence document, two production files,
the isolated fixture manifest/lock and three fixture bins, and one integration
test. Production changes are deletion-only except for the mechanically rewritten
root reexport list:

- `src/lib.rs`: removes only the raw installer reexport.
- `src/prepared_export.rs`: removes the raw installer, its now-unused
  `DriveRequest` import, `WorkerLaunch::Bundled`, its bundled executable resolver
  arm, and `worker_asset_name`.

At the exact parent, repository-wide references prove:

- the sole `WorkerLaunch::Bundled` mint was inside
  `install_prepared_export`;
- the sole `Self::Bundled` consumer was the corresponding `resolve` arm;
- `worker_asset_name` was called only by that arm; and
- the facade reexport and the installer definition were the only production
  references to `install_prepared_export`.

The remaining `WorkerLaunch::Exact` and `PreparedExport::from_test_worker` are
both `#[cfg(test)]` and preserve the existing mechanical negative tests. No
production constructor for `PreparedExport` remains. Therefore deletion of
`WorkerLaunch::Bundled` and the sibling worker-path resolver removes only the
unreachable implementation tail of the poisoned raw aperture; it does not
delete a separate live owner transition.

No direct `install_prepared_export` caller exists in candidate implementation
source or in `/home/ubuntu/swarm` implementation source. Candidate occurrences
are limited to documentation, the two intentional external compile-fail bins,
and the integration test's diagnostic assertion.

## External sibling and Cargo boundary

`tests/fixtures/public_api_boundary/Cargo.toml` is a standalone workspace and
declares only:

```toml
[dependencies]
libbun = { path = "../../.." }
```

Locked, offline Cargo metadata resolved that dependency to the exact review
worktree. Direct fixture checks produced:

| Fixture bin | Exit | Result |
| --- | ---: | --- |
| `adjacent-public-controls` | 0 | `PreparedExport`, `DriveControl`, and `MechanicalTerminal` remain importable. |
| `import-raw-installer` | 101 | `error[E0432]: unresolved import libbun::install_prepared_export`; no symbol exists in the crate root. |
| `call-raw-installer` | 101 | `error[E0425]: cannot find function install_prepared_export in crate libbun`. |

The failures name the deleted symbol and occur after the external dependency
and adjacent public controls compile. They are not dependency, lock, network,
fixture, syntax, or unrelated type errors.

## Implementation-only searches

Searches over `src`, `native`, `runtime`, `wire`, and `scripts`, with evidence
and negative fixtures excluded where applicable, found no candidate match for:

```text
install_prepared_export
WorkerLaunch::Bundled
Self::Bundled
worker_asset_name
```

Facade implementation searches found no installer/new/from/prepare/admit/select
alias involving `PreparedExport`, and no public function replaces the deleted
installer. A zero-context production diff addition search found no new behavior:
the only added production line is the existing reexport list without
`install_prepared_export`.

## Repeated acceptance gates

Every command below ran through the exact claimed Lane. Nextest used default
parallelism and a same-shell soft file-descriptor limit of 65536.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo check --locked --workspace` | pass |
| `cargo nextest run --locked --test public_api_boundary` | pass, 1/1 |
| `cargo nextest run --locked --workspace` | pass, 15/15 across two binaries |
| Locked/offline external `adjacent-public-controls` check | pass, exit 0 |
| Locked/offline external `import-raw-installer` check | intended failure, exit 101, E0432 |
| Locked/offline external `call-raw-installer` check | intended failure, exit 101, E0425 |
| `git diff --check 5b14f8d0..6066a5b8` | pass |
| Root and external fixture lock diff checks | pass, unchanged |
| Candidate implementation-only old-shape searches | clean |
| Lane census after acceptance commands | `processes: []` |

The full suite retained all fourteen preexisting mechanical tests and added the
external boundary test. The candidate worktree remained source-clean after all
commands.

## Disposition

`POISON CUT PASS`. The candidate is composition-eligible as the first poison
commit required by the frozen W1-11/W1-12 replacement order. It must remain a
hard deletion: later compiler fallout migrates producers to the branded,
backend-owned positive transition and must not restore this raw constructor or
an equivalent facade.

No product source was changed during this review. Only this durable verdict
artifact is added by the review commit.
