# ADR-2049: Bundled Plugin and File-Free Runtime

Status: Open
Date: 2026-05-20

ADR-2038 remains the native consumption law: downstream hosts load Bun/JSC/WebKit
through a replaceable dynamic plugin and never statically link the native
adapter. ADR-2047 made an opt-in `download-plugin` Cargo build path available,
but that path is not the final product topology for hosts that ship a native
binary.

The product boundary should be simpler:

```text
shipped host binary
  -> bundled verified libbun plugin next to the binary
  -> dynamic load by host-owned relative path

development host binary
  -> development plugin next to the development binary or target artifact
  -> dynamic load by host-owned relative path
```

The runtime boundary should also be file-free for generated provider adapters.
Current evidence such as source-root `libbun-native-*` directories containing
`module-1.mjs`, `stdout.capture`, `stderr.capture`, or `log.capture` is not an
acceptable embedding contract. Those files are implementation leakage from the
native adapter into the host workspace.

## Decision

`libbun` will support bundled plugin resolution and file-free native runtime
execution.

### Runtime Concurrency

The embedded Bun runtime is a process-global native subsystem for libbun's
current adapter shape. `libbun` must not claim that multiple in-process Bun VMs
can execute concurrently unless that is proven against Bun/JSC/WebKit state,
global output wiring, environment overlays, event-loop state, and plugin
teardown.

Until that proof exists, a process may hold at most one live libbun runtime
lease. Creating a second native or dynamic runtime while the first is active
must fail immediately with a clear initialization error. It must not block on a
mutex, spin, silently share the VM, or create a second in-process VM.

Hosts that need multiple capabilities should load them into the same runtime and
dispatch by module/export through that runtime. Parallelism belongs above or
outside the single runtime lease unless a future adapter proves isolated VM
instances.

Any native adapter test binary that creates real Bun runtimes must run those
tests serially. Parallel runtime tests in one process are invalid unless the
adapter first proves multiple isolated Bun runtimes.

### Plugin Resolution

Hosts that ship a binary must bundle the verified native plugin beside that
binary, or in a deterministic directory relative to it. Hosts that run from a
development target directory must place or symlink the development plugin
relative to that development binary.

`libbun` must expose APIs that let a host load a plugin from an explicit path or
from a host-provided relative search root. A user/admin override such as
`LIBBUN_PLUGIN_PATH` may remain for replacement and diagnostics, but persistent
runtime plugin caches are not the product contract.

`download-plugin` may remain a convenience for local crate development and
experiments, but product hosts must not depend on user cache state or
crate-managed release caches at runtime. Any cache-oriented resolver must be
separate from the host-bundled resolver and must not be the default shape for
shipped binaries.

### Native Link Inputs

Native plugin and native adapter tests must link Bun from the release profile
only. Debug-profile Bun objects are structurally invalid for libbun plugin
artifacts because Bun debug builds load bundled JavaScript builtins from
absolute build-directory paths such as `build/debug/js`. A relocatable plugin
must use release inputs where those builtins are embedded in the linked binary.

`scripts/prepare-native-bun-link.sh`, `native/build.rs`, and `plugin/build.rs`
must reject debug-profile inputs. They must not silently consume
`vendor/bun/build/debug/libbun_native_link_manifest.txt`, `bun-debug`,
`build/debug`, or WebKit/JSC static libraries from debug cache directories. A
stale manifest from another machine must fail because its link inputs do not
exist in the current checkout.

Linux release builds use the PIC in-process plugin path now that durable PIC
WebKit inputs exist for the supported Linux targets. The helper-process Linux
transport is quarantined as a legacy diagnostic path only. It must not be the
default for local preflight, release packaging, published bundle metadata, or
downstream documentation. Building or packaging it requires an explicitly named
legacy opt-in and must not be reachable by accidental Linux defaults.

The Linux dynamic plugin is also a strict C ABI boundary. It must export only
the `libbun_plugin_*` symbols that define the dynamic facade ABI. Bun, JSC,
WebKit, Zig, Rust, allocator, and helper implementation symbols are private
implementation details of the plugin and must not be present in the dynamic
symbol table as defined exports. A plugin that exports native runtime internals
is structurally invalid even if it links, because ELF symbol preemption and
loader-visible global state can change native runtime behavior across the
`cdylib` boundary.

The PIC WebKit inputs themselves must be release-grade artifacts. Debug PIC
WebKit archives are not acceptable production inputs, even when relocation
inspection passes, because debug WebKit/JSC can carry assertion, sanitizer, and
runtime assumptions that do not match the libbun release plugin lane. The
libbun fetch, preflight, and release workflows must reject any WebKit PIC asset
or metadata marked as debug.

### Runtime Files

`BunModuleSpec::Source` represents host-provided in-memory source. It must be
loaded through Bun's virtual module/data-source path or equivalent in-memory
module loader. It must not be materialized as `module-*.mjs`.

Generated provider adapters are `BunModuleSpec::Source` modules. They are not
package files and must not appear in the application source root, build tree, or
temporary runtime directory.

Stdout, stderr, and internal log capture must use memory buffers or
host-provided file descriptors. The native adapter may use OS pipes or another
file-descriptor transport because Bun's output machinery speaks descriptors,
but it must not create capture files.

Prepared bundles may continue to be a separate artifact contract, but if they
are used for generated adapters they must follow the same file-free rule. Any
future prepared-bundle materialization must be explicitly justified as a bundle
format requirement, not reused for provider adapters.

### Module Diagnostics

Module-load failures are public embedding-boundary failures. They must identify
the owner operation and the module specifier involved. If Bun/JSC throws or
rejects during import, libbun must preserve the JavaScript exception message and
stack when available. Returning only `module import threw`, `module load
failed`, or another context-free string is structurally invalid because it
forces downstream hosts to debug by source archaeology.

If Bun/JSC does not expose exception details, the diagnostic must self-accuse by
saying that the exception detail was unavailable and still include the import
specifier, operation, and libbun boundary. Downstream hosts may wrap that error
with their own provider/contract context, but libbun remains responsible for the
Bun-facing module-load facts.

## Required Refactor

1. Add host-bundled plugin resolution APIs that accept a host binary path,
   plugin directory, or explicit plugin path and do not inspect cache
   directories.
2. Keep `DynamicBunRuntime::load(path, config)` as the lowest-level exact path
   API.
3. Stop making `DynamicBunRuntime::initialize` depend on a cache-first resolver
   for product hosts.
4. Make native link manifest generation and consumption release-only, rejecting
   debug-profile inputs and stale manifests before Cargo emits linker flags.
5. Replace `BunModuleSpec::Source` materialization with an in-memory import
   specifier, virtual module registration, or equivalent Bun host module loader.
6. Replace file-backed stdout/stderr/log capture with pipe-backed or
   memory-backed capture.
7. Remove the native adapter's source-root runtime tempdir requirement for
   source-module execution and output capture.
8. Add tests proving no `module-*.mjs`, `stdout.capture`, `stderr.capture`,
   `log.capture`, or `libbun-native-*` runtime artifact is created for source
   modules.
9. Add process-local runtime lease enforcement for both the native adapter and
   dynamic plugin facade so a second live runtime fails immediately.
10. Update downstream documentation so binary products bundle the plugin instead
   of relying on `~/.cache/libbun`, `LIBBUN_HOME`, or build-output download
   caches.
11. Make Linux PIC in-process the default preflight/release/package path and put
    helper-process behind explicit legacy diagnostic opt-ins.
12. Consume only release-grade Linux WebKit PIC artifacts and reject debug PIC
    assets before extraction, manifest rewriting, or linking.
13. Give the Linux plugin an explicit dynamic export boundary that exposes only
    the libbun plugin C ABI and keeps all Bun/JSC/WebKit/native internals local.
14. Add release/preflight gates that reject Linux plugin artifacts exporting any
    non-ABI defined dynamic symbol.
15. Make source/path/prepared-bundle module import failures preserve the import
    specifier and JavaScript exception message/stack, or self-accuse when Bun/JSC
    exposes no exception details.

## Acceptance Criteria

- A host can load a bundled plugin by exact or binary-relative path without any
  cache directory.
- A generated provider adapter passed as `BunModuleSpec::Source` executes
  without creating a module file.
- Captured stdout, stderr, and log output work without capture files.
- Running a source-module provider against a clean working directory leaves no
  `libbun-native-*`, `module-*.mjs`, or `*.capture` artifacts behind.
- The dynamic plugin ABI remains replaceable and version-checked.
- A process cannot create two live libbun runtimes concurrently; the second
  initialization returns a clear structural error instead of blocking.
- Release preflight runs native adapter runtime tests serially so the test
  harness does not violate the intentional single-runtime lease.
- Native link manifests consumed by adapter/plugin builds are release-profile
  manifests and cannot point at debug Bun objects, `bun-debug`, or debug
  WebKit/JSC libraries.
- Linux release/preflight packaging defaults to PIC in-process plugins. The
  helper-process path cannot build or package unless a legacy diagnostic opt-in
  is explicitly set.
- Linux in-process release/preflight lanes consume release WebKit PIC inputs;
  debug PIC inputs fail structurally before linking.
- Linux in-process plugins export only the libbun plugin C ABI. Release and
  preflight packaging reject artifacts that expose Bun/JSC/WebKit/native
  implementation symbols through the dynamic symbol table.
- `download-plugin` is clearly documented as a development/convenience path, not
  the product shipping contract.
- Module-load errors include the failed import specifier and JavaScript
  exception detail when available; context-free `module import threw` diagnostics
  are rejected.
