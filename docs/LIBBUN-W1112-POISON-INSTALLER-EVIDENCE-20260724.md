# libbun W1-11/W1-12 Raw Installer Poison Evidence

Date: 2026-07-24

Classification: `POISON`

Implementation base: `5b14f8d0599e40630f788cc3863d0b3d96116199`

## Completed Cut

- Deleted `install_prepared_export(Vec<u8>, String, Vec<u8>)` from
  `src/prepared_export.rs`.
- Deleted its facade reexport from `src/lib.rs`.
- Deleted the now-unreachable production `WorkerLaunch::Bundled` mint and
  sibling worker-path resolver. Test-only exact-worker custody remains solely
  to preserve the existing negative mechanical evidence.
- Added an isolated external dependent crate whose import and fully qualified
  call fixtures must fail because `libbun` no longer exports the symbol.
- Added a successful adjacent-control fixture proving `PreparedExport`,
  `DriveControl`, and `MechanicalTerminal` remain ordinary public types.

No replacement behavior, alias, fixed error, fallback, empty/default cargo,
selector, parts product, or public wire/native aperture was added.

## Stale Producer Record

Repository-wide searches in libbun and `/home/ubuntu/swarm` found no direct
source caller of `install_prepared_export`. The first stale producer boundary
is therefore the downstream caller family described by the frozen edit gate:
any caller that separately supplies prepared artifact bytes, selected export
text, and opaque invocation bytes. That family must not be patched back through
another raw constructor. The next positive owner tranche must migrate it to
producer-minted branded `SelectedProviderPackage` plus branded
`ProviderInvocation`, consumed by the opaque by-value `BunProviderBackend`.

## Evidence

- `cargo check --locked`: pass.
- `cargo nextest run --test public_api_boundary` under default parallelism:
  one external boundary test passed.
- `cargo nextest run --workspace` under default parallelism: 15/15 tests
  passed, including all 14 retained candidate tests and the external boundary
  test.
- The external adjacent-control bin compiled successfully with exit 0.
- The external import bin failed with exit 101 and `E0432`, naming
  `install_prepared_export` as absent from the crate root.
- The external fully qualified call bin failed with exit 101 and `E0425`,
  naming `install_prepared_export` as absent from `libbun`.

The frozen highest owner remains `BunProviderBackend`, and the Fifteen-Step
Hard-Cut Order in `docs/README.md` remains unchanged.
