# W1-11/W1-12 Retained Prepared-Export RED Evidence

Dual-Path Evidence:
- Phase: RED
- Source SHA: `1d5a7852bf114971fb17b8233a3cc05c38ad8945`
- Missing semantic law: retained `BunProviderBackend` must move by value into one affine `PreparedExport`; `PreparedExport::drive(self, DriveControl)` must publish exactly one typed mechanical terminal only after the drive is quiescent or terminally shut down, with interrupt and consuming shutdown custody.
- Rust fixture/test paths: `tests/prepared_export_lifecycle.rs`.
- Parallel Nextest command; exit code; result/diagnostic: `cargo nextest run --locked --test prepared_export_lifecycle retained_owner_surface_is_affine_and_mechanically_closed`; exit `101`; intended `E0432` for absent `BackendShutdownTerminal`, `DriveControl`, `DriveInterrupt`, `MechanicalTerminal`, `PreparedExport`, and `ShutdownControl`, plus `E0107` proving `BunProviderBackend` is still a generic borrowed owner.
- Real downstream fixture path: Swarm `tests/conformance/ss/provider/libbun_provider_value_json_v1_quiescence.test.ss`, preserved with executed RED evidence at Swarm commit `09b9bcd4b91c97f2876c1768e45b03700e72c59b` and pre-implementation source SHA `d854a8ce2aa98f4c0d9852c81bb98f198da0d886`.
- Lane-built downstream binary/result: preserved evidence records `/home/ubuntu/bridge-ops/cargo-slots/swarm-fe618e3e/slot-239/debug/swarm`, SHA-256 `f97fd20958070af8d283c2e0135d9779fcba18f8091e4b05f0734ab24dba2355`; real `swarm test` reached provider delivery and then failed when in-process Bun aborted the authenticated worker with signal 6.
- Same-SHA proof (libbun RED): Lane identity, focused Rust compilation, and this absent-surface diagnostic all name `1d5a7852bf114971fb17b8233a3cc05c38ad8945`. The separately preserved downstream `.ss` RED names its exact Swarm source and Lane-built binary because libbun and Swarm are distinct repositories.
