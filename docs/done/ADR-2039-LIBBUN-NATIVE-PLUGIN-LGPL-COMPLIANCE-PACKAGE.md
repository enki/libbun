# ADR-2039: libbun Native Plugin LGPL Compliance Package

Status: Done
Date: 2026-05-18

ADR-2038 makes the Bun/JSC/WebKit-carrying implementation available to
downstream hosts only as a replaceable dynamic plugin. That architecture keeps
ordinary downstream host binaries out of the static Bun/JSC/WebKit link.

The remaining responsibility belongs to `libbun`: when `libbun` publishes or
otherwise distributes a native plugin binary, that binary is the artifact that
carries the LGPL-relevant native dependency set. `libbun` must therefore publish
a complete compliance package for the plugin instead of pushing discovery work
onto every downstream host application.

This ADR is an engineering policy, not legal advice. It is based on the LGPL
2.1 shared-library mechanism, the older LGPL 2.0 relinking language, and Bun's
own license note that Bun statically links JavaScriptCore/WebKit.

## Decision

Every distributed `libbun` native plugin binary must be accompanied by a
same-version compliance package.

The standard publication mechanism is a GitHub Release. Each release that
contains a native plugin binary must attach the matching compliance artifacts as
release assets from the same release page. A moving branch link such as
`main`, or a generic link to the repository, is not enough.

GitHub Actions is the required release factory for official native plugin
binaries. Maintainer laptops may be used for local testing, but an official
binary release must be built, verified, packaged, checksummed, and attached by a
tag-triggered or manually approved GitHub Actions workflow. This keeps the
binary, source archive, notices, inventory, and checksums reproducible from the
repository's declared release inputs instead of depending on private local
state.

The package must include, or point from the same GitHub Release/download
location to:

- the exact `libbun` source used to build the plugin;
- the exact vendored Bun source snapshot and upstream commit;
- all local patches applied to Bun, WebKit, JavaScriptCore, or related native
  dependency sources;
- the scripts, manifests, and build instructions used to compile and install
  the plugin;
- the complete license texts and copyright notices for Bun, WebKit,
  JavaScriptCore, and all other redistributed native dependencies;
- a generated third-party license inventory for the plugin artifact;
- replacement instructions showing how a user can build a modified compatible
  plugin and point a host at that replacement;
- ABI compatibility information for the plugin version, including the
  `libbun` plugin ABI version and Bun source revision.

The preferred distribution shape is:

```text
GitHub Release vX.Y.Z
  libbun facade crate/source tag
  libbun-plugin-native-vX.Y.Z-<platform>.tar.zst
  libbun-plugin-native-vX.Y.Z-source.tar.zst
  libbun-plugin-native-vX.Y.Z-NOTICE.txt
  libbun-plugin-native-vX.Y.Z-licenses.json
  libbun-plugin-native-vX.Y.Z-checksums.txt
  libbun-plugin-native-vX.Y.Z-SOURCE.txt
```

If a release publishes the plugin binary from one place, the corresponding
source/compliance bundle must be reachable from that same place. Do not rely on
an informal written-offer-only process when direct source publication is
practical.

The source archive must be generated from immutable release inputs. It may
include pinned external source URLs for very large upstream components only when
the URL identifies the exact corresponding source by immutable commit, tag, or
content-addressed archive and the notice file tells downstream distributors
exactly how to retrieve it. Prefer attaching all source that `libbun` modifies
or vendors directly to the release.

The GitHub Actions workflow must be the documented path that downstream
consumers can inspect or re-run from a fork if they want to build their own
compatible plugin. Directly building `./plugin` from a checkout is a development
workflow, not the recommended upstream consumption path.

## Downstream Contract

A downstream host that only dynamically loads a replaceable `libbun` plugin and
does not redistribute the plugin binary should not need to provide host
application object files, host source, or host relinking materials because of
`libbun`'s native implementation.

A downstream distributor that redistributes the `libbun` native plugin binary
must still preserve and pass along the plugin compliance package. That
obligation is about the plugin and its bundled native dependency set, not about
opening the downstream host application.

The downstream redistribution path should be mechanical:

- include the unmodified `libbun` plugin binary or point users to the exact
  upstream GitHub Release asset;
- include `SOURCE.txt` or equivalent text naming the exact `libbun` release,
  source archive, notice file, license inventory, and checksums;
- include the `libbun` NOTICE/license materials in the product's third-party
  notices;
- keep the plugin replaceable by path, environment variable, configuration, or
  another documented user-controlled mechanism;
- do not strip, wrap, sign, or install the plugin in a way that prevents a user
  from replacing it with an interface-compatible modified build.

Downstream consumers should be told to depend on the published Rust facade and
use the official GitHub Release assets for native plugin binaries. They should
not be expected to vendor the `libbun` repository and run `./plugin` builds
unless they deliberately choose to become the distributor of their own plugin
build and its compliance package.

Downstream terms must not prevent users from replacing the plugin with an
interface-compatible modified build. If downstream product terms include broad
no-reverse-engineering or no-modification language, they need a carve-out for
debugging modified LGPL-covered library/plugin replacements.

## Required Release Checks

Before publishing a native plugin binary, `libbun` release automation must
verify:

- the plugin is a dynamic library artifact, not a static archive intended to be
  folded into a host executable;
- the public Rust facade does not depend on `libbun-native` or otherwise link
  Bun/JSC/WebKit objects into downstream host binaries;
- the plugin ABI version and Bun source revision are exported or discoverable;
- the GitHub Release tag, plugin binary, source archive, notice file, license
  inventory, and checksums all identify the same source commit and version;
- the release artifacts were produced by the official GitHub Actions release
  workflow, not by an undocumented local build;
- a replacement plugin can be built from the published source bundle;
- a host can load the replacement plugin through the documented runtime path;
- license texts and notices are present in the release bundle;
- the third-party inventory includes Bun, WebKit, JavaScriptCore, and native
  libraries named by Bun's link manifest.

The release workflow must fail closed: if any binary platform cannot produce the
matching source archive, notice file, license inventory, checksum file, dynamic
loader test result, or replacement-build verification, that platform's binary
must not be attached to the release.

## Non-Goals

This ADR does not require downstream host applications to be open source.

This ADR does not require downstream host applications to distribute object
files merely because they use the `libbun` Rust facade or runtime-load a
replaceable plugin.

This ADR does not make the native plugin obligation-free. The plugin binary is
the artifact that carries Bun/JSC/WebKit native code, so the plugin release must
carry the corresponding source, notices, and replacement story.

## Consequences

`libbun` release engineering becomes responsible for a real compliance artifact,
not just a source repository.

Downstream hosts get a simpler rule: use the stable facade plus dynamic loader,
keep the plugin replaceable, and pass through the official plugin compliance
bundle if they redistribute the plugin.

Downstream documentation can link to the exact `libbun` GitHub Release instead
of describing Bun/WebKit/JSC compliance from scratch, as long as it also ships
or references the exact release's source, notice, license inventory, and
checksum artifacts.

The official release page becomes the contract upstream consumers rely on:
`cargo add libbun` for the facade, plus versioned GitHub Release assets for the
native plugin and compliance materials.

The plugin must remain buildable from published source. If the build depends on
private machines, unpublished SDK changes, local cache state, or missing
generated files, the release is not compliant enough to publish as a binary.

## Implementation

This ADR is implemented by:

- `.github/workflows/release-native-plugin.yml`, the official GitHub Actions
  release workflow for native plugin binaries;
- `scripts/preflight-native-plugin-release.sh`, which mirrors the workflow
  locally and verifies a checkout can build, test, load, and package the plugin;
- `scripts/create-native-plugin-release.sh`, which creates and pushes the
  annotated release tag that triggers the GitHub Actions release workflow;
- `scripts/package-native-plugin-release.sh`, which packages the plugin binary,
  matching source archive, notice file, license inventory, source pointers, and
  checksums;
- the `plugin/` `cdylib` crate, which is the only Bun/JSC/WebKit-carrying
  downstream artifact shape;
- the `dynamic-loading` facade feature and dynamic plugin integration test,
  which verify the downstream host path uses runtime loading;
- the `internal-adapter` native feature gate, which prevents accidental direct
  downstream use of the native adapter.

The workflow builds and tests the facade, prepares the Bun native link inputs,
builds the plugin, runs a dynamic-loader smoke test through
`LIBBUN_PLUGIN_PATH`, verifies that the plugin can be rebuilt from the release
inputs, then attaches the generated compliance artifacts to the GitHub Release.

## Source References

- GNU Lesser General Public License v2.1:
  <https://www.gnu.org/licenses/lgpl-2.1.html>
- GNU Library General Public License v2.0:
  <https://www.gnu.org/licenses/old-licenses/lgpl-2.0.en.html>
- GNU GPL FAQ, LGPL static and dynamic linking discussion:
  <https://www.gnu.org/licenses/gpl-faq.html#LGPLStaticVsDynamic>
- Bun license notes in this repository:
  `vendor/bun/LICENSE.md`
