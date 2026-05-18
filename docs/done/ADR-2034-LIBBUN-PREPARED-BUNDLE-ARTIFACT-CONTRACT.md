# ADR-2034: libbun Prepared Bundle Artifact Contract

Status: Done
Date: 2026-05-18

`BunModuleSpec::PreparedBundle` names an embedding capability, but accepting
arbitrary bytes would make `libbun` appear reusable while leaking unstable Bun
CLI artifact assumptions to downstream hosts.

`libbun` defines `PreparedBundleV1` as a versioned, host-independent source
bundle envelope. The native adapter accepts only that envelope; it does not call
Bun CLI `main`, `Cli::start`, `bun build`, or process-global command dispatch.

The v1 artifact contract contains:

- `format = "libbun.preparedBundle"` and `formatVersion = 1`;
- `bundleId`;
- Bun source revision metadata;
- `libbun` ABI version metadata;
- an entry module path;
- a map of relative POSIX-style module paths to source text;
- a deterministic SHA-256 fingerprint over the serialized artifact.

Validation rejects:

- unknown format identifiers or format versions;
- empty bundle ids, Bun revisions, module maps, module paths, or module source;
- absolute paths, parent-directory paths, current-directory segments, empty path
  segments, and backslash paths;
- requested bundle ids that do not match artifact bundle ids;
- Bun revision or `libbun` ABI metadata that does not match the current runtime.

The native adapter materializes validated modules under its runtime tempdir and
imports the entry module through Bun's module loader. This preserves the
embedding boundary while giving hosts a reusable prepared-artifact handoff and
cache key.

Evidence:

- `PreparedBundleV1::source_bundle`, `to_bytes`, `from_bytes`,
  `fingerprint`, and `validate_for_current_runtime` define the facade-side
  producer/consumer contract.
- `native/src/lib.rs` handles `BunModuleSpec::PreparedBundle` by validating and
  materializing the artifact before calling `JSModuleLoader::import_ptr`.
- `tests/conformance.rs` covers artifact round-trip, fingerprint, compatibility
  validation, and path rejection.
- `native/tests/native_prepared_bundle.rs` loads a prepared bundle through the
  native adapter without CLI dispatch.
