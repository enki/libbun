# ADR-2045: Linux Release Matrix and Helper Bundle

Status: Done
Date: 2026-05-18

## Context

ADR-2044 permits Linux native support through a helper-backed runtime bundle:
the downstream host dynamically loads `liblibbun_plugin_native.so`, and that
plugin launches a companion `libbun-runtime-native` executable when Linux
cannot safely link all Bun/WebKit inputs into a single `.so`.

The release workflow currently publishes only macOS arm64. Linux x86_64 and
Linux arm64 were removed because the in-process plugin link was not viable with
the upstream prebuilt WebKit archives. Linux support should return only when
the complete helper-backed release bundle is built, tested, packaged, and
verified.

## Decision

The mature Linux release matrix is:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
```

Each Linux release asset must contain:

```text
liblibbun_plugin_native.so
libbun-runtime-native
libbun-native-bundle.json
```

`libbun-native-bundle.json` records:

- target triple;
- plugin ABI version;
- helper protocol version;
- libbun release version;
- libbun git commit;
- Bun source commit;
- plugin filename and SHA-256;
- helper filename and SHA-256;
- source archive filename;
- notice and license inventory filenames.

The release workflow must not attach Linux assets until the helper-backed
plugin passes dynamic-loading tests on that same target.

The release naming and downstream setup must not encode helper IPC as a
permanent platform contract. The Linux tarball may contain a helper executable
now, but the stable consumer-facing object remains the plugin path:

```text
LIBBUN_PLUGIN_PATH=/path/to/liblibbun_plugin_native.so
```

If Linux later switches to an in-process plugin, the target tarball names,
facade crate usage, and `LIBBUN_PLUGIN_PATH` setup should remain unchanged.

## Required CI Shape

The GitHub Actions release workflow must build and test:

- `aarch64-apple-darwin` in-process plugin;
- `x86_64-unknown-linux-gnu` helper-backed plugin bundle;
- `aarch64-unknown-linux-gnu` helper-backed plugin bundle.

Linux jobs must run these gates before packaging:

- facade tests;
- dynamic-loading facade check;
- native helper build;
- plugin build;
- plugin/helper protocol smoke test;
- `LIBBUN_PLUGIN_PATH` dynamic provider flow;
- helper replacement-path smoke test;
- release package generation;
- release asset verifier.

The CI design must leave room for an in-process Linux implementation. Linux
jobs should select the runtime mode explicitly, for example:

```text
helper-process
in-process
```

Only `helper-process` is required for the first working Linux release. A future
PIC retry can add an `in-process` experimental lane without changing the
release asset names until that lane is ready to become the default.

The release verifier must require Linux assets by default only after both Linux
targets pass in CI.

## Packaging Requirements

Linux binary asset names remain target-specific:

```text
libbun-plugin-native-vX.Y.Z-x86_64-unknown-linux-gnu.tar.zst
libbun-plugin-native-vX.Y.Z-aarch64-unknown-linux-gnu.tar.zst
```

Each tarball contains the Linux plugin `.so`, helper executable, and bundle
metadata. The shared compliance assets remain attached once per release:

```text
libbun-plugin-native-vX.Y.Z-source.tar.zst
libbun-plugin-native-vX.Y.Z-NOTICE.txt
libbun-plugin-native-vX.Y.Z-licenses.json
libbun-plugin-native-vX.Y.Z-SOURCE.txt
libbun-plugin-native-vX.Y.Z-checksums.txt
```

The checksum file must include every platform tarball and every shared
compliance asset.

## Documentation Requirements

README must document:

- the shared consumer contract:
  `consumer app -> LIBBUN_PLUGIN_PATH -> native plugin`;
- the macOS implementation:
  `consumer app -> dynamically loaded .dylib -> in-process Bun/JSC/WebKit`;
- the Linux implementation:
  `consumer app -> dynamically loaded .so -> helper process -> Bun/JSC/WebKit`;
- Linux x86_64 download and setup;
- Linux arm64 download and setup;
- that the Linux tarball contains both a plugin and helper executable;
- how `LIBBUN_PLUGIN_PATH` points at the plugin;
- how to override the helper path for replacement builds;
- that downstream hosts still dynamically load the plugin and do not link
  Bun/JSC/WebKit.
- that Linux may switch to an in-process plugin later if suitable WebKit/JSC/WTF
  artifacts become available, without changing the downstream loading contract.
- that helper IPC is an implementation detail of the current Linux bundle, not
  a permanent downstream API.

Release instructions must describe how to cut a release, inspect CI, verify
assets, and avoid announcing Linux support before both Linux jobs are green.

## Non-Goals

This ADR does not require a single-file Linux `.so`.

This ADR does not require cross-compiling Linux targets from macOS.

This ADR does not change the Cargo facade publication plan.

This ADR does not add Windows support.

## Consequences

Linux users get predictable GitHub Release assets for the two common Linux
server architectures.

The Linux artifact is a bundle, not a lone shared library. That is a deliberate
tradeoff: it preserves downstream dynamic loading while avoiding the fragile
requirement that every upstream WebKit/JSC/WTF static archive be suitable for
in-process shared-object linking.

The artifact naming intentionally stays compatible with a future in-process
Linux plugin. The bundle contents can shrink later if the helper executable is
no longer needed, but consumers should continue to configure only the plugin
path.

## Acceptance Criteria

This ADR can move to `docs/done/` when:

- `.github/workflows/release-native-plugin.yml` includes Linux x86_64 and
  Linux arm64 helper-backed lanes;
- both Linux lanes build `liblibbun_plugin_native.so`;
- both Linux lanes build `libbun-runtime-native`;
- both Linux lanes pass `LIBBUN_PLUGIN_PATH` dynamic loading tests;
- both Linux lanes pass helper replacement-path tests;
- CI names or environment variables select the Linux runtime mode so a future
  in-process PIC lane can be added without changing target names or downstream
  docs;
- `scripts/package-native-plugin-release.sh` packages plugin, helper, and
  bundle metadata for Linux;
- `scripts/verify-release-assets.sh` requires both Linux target tarballs by
  default;
- README documents Linux x86_64 and Linux arm64 usage, the macOS/Linux runtime
  implementation difference, and the shared dynamic-loading consumer contract;
- release compliance assets include source/build instructions sufficient to
  rebuild and replace the helper-backed Linux runtime bundle.
