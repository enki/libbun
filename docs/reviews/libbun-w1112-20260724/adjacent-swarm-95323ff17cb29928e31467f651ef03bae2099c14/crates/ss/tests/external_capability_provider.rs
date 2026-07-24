#![forbid(unsafe_code)]

use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn ss_runs_swarm_io_through_package_resolved_rust_provider_owner() {
    let fixture = write_fixture(
        "swarm-io-rust-package-provider",
        r#"import capability { io } from "@swarm/io";

return try await io.print({ value: "hello from package resolved swarm io" });"#,
    );
    write_io_application_package(
        fixture
            .source_path
            .parent()
            .expect("fixture source should have parent"),
    );
    let ss_binary = PathBuf::from(env!("CARGO_BIN_EXE_swarm"));
    let cache_root = fixture_cache_root(&fixture, "prepared-runtime-cache");
    let output = Command::new(&ss_binary)
        .arg("run")
        .arg("--package-root")
        .arg(workspace_root())
        .arg(&fixture.source_path)
        .env("SWARM_CACHE_DIR", cache_root)
        .output()
        .expect("ss binary should run");

    assert!(
        output.status.success(),
        "ss should run package-resolved Rust provider owner\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "hello from package resolved swarm io\n"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).is_empty(),
        "ss should not emit debug output by default"
    );
}

#[test]
fn ss_runs_swarm_io_read_line_through_source_entrypoint_cli_host() {
    let fixture = write_fixture(
        "swarm-io-read-line-cli-host",
        r#"import capability { io } from "@swarm/io";

const first = try await io.readLine({ prompt: "> " });
if (first.eof || first.line != "hello") {
  return try await io.error({ value: "unexpected first read" });
}
const second = try await io.readLine({ prompt: "next> " });
if (!second.eof || second.line != null) {
  return try await io.error({ value: "unexpected eof read" });
}
return try await io.print({ value: "ok" });"#,
    );
    write_io_application_package(
        fixture
            .source_path
            .parent()
            .expect("fixture source should have parent"),
    );
    let ss_binary = PathBuf::from(env!("CARGO_BIN_EXE_swarm"));
    let cache_root = fixture_cache_root(&fixture, "prepared-runtime-cache");
    let mut child = Command::new(&ss_binary)
        .arg("run")
        .arg("--package-root")
        .arg(workspace_root())
        .arg(&fixture.source_path)
        .env("SWARM_CACHE_DIR", cache_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("ss binary should spawn");
    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(b"hello\n")
        .expect("stdin line should be written");
    drop(child.stdin.take());
    let output = child
        .wait_with_output()
        .expect("ss binary should finish after stdin eof");

    assert!(
        output.status.success(),
        "ss should run @swarm/io readLine through source-entrypoint CLI host\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "> next> ok\n");
    assert!(
        String::from_utf8_lossy(&output.stderr).is_empty(),
        "ss should not emit errors for CLI-host readLine"
    );
}

#[test]
fn ss_reuses_one_libbun_runtime_for_multiple_capability_imports() {
    let fixture = write_fixture(
        "multi-provider-shared-libbun-runtime",
        r#"import capability { counter } from "@swarm-fixture/counter";

try await counter.first({});
try await counter.second({});
return true;"#,
    );
    write_counter_provider_package(
        fixture
            .source_path
            .parent()
            .expect("fixture source should have parent"),
    );
    let ss_binary = PathBuf::from(env!("CARGO_BIN_EXE_swarm"));
    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
    let cache_root = fixture_cache_root(&fixture, "prepared-runtime-cache");
    let output = Command::new(&ss_binary)
        .arg("run")
        .arg("--package-root")
        .arg(workspace_root())
        .arg(&fixture.source_path)
        .env("SWARM_CACHE_DIR", cache_root)
        .output()
        .expect("ss binary should run");

    assert!(
        output.status.success(),
        "ss should run both package-resolved libbun providers in one invocation\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("\"status\": \"completed\""),
        "the second capability call must accept the module-level state produced by the first call"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).is_empty(),
        "ss should not emit debug output by default"
    );
}

#[test]
fn ss_test_pool_child_conserves_package_roots_for_test_and_libbun_providers() {
    let fixture = write_fixture_source(
        "pool-child-multi-provider-package-roots",
        "main.test.ss",
        r#"import type { TestExecutionContext } from "@swarm/test";
import capability { test, expect } from "@swarm/test";
import capability { io } from "@swarm/io";

const body = function(context: TestExecutionContext): null {
  try await io.print({ value: "pool child libbun provider" });
  return try await expect.equal({ actual: 2 + 3, expected: 5 });
};

return try await test(
  "pool child conserves package roots for test and libbun providers",
  body,
);"#,
    );
    let fixture_root = fixture
        .source_path
        .parent()
        .expect("fixture source should have parent");
    write_pool_worker_multi_provider_application_package(fixture_root);
    let ss_binary = PathBuf::from(env!("CARGO_BIN_EXE_swarm"));
    bundle_development_libbun_plugin_next_to_ss_binary(&ss_binary);
    let output = Command::new(&ss_binary)
        .arg("test")
        .arg(&fixture.source_path)
        .arg("--parallel")
        .arg("--reporter")
        .arg("json")
        .arg("--package-root")
        .arg(fixture_root)
        .arg("--package-root")
        .arg(workspace_root())
        .current_dir(fixture_root)
        .env(
            "SWARM_CACHE_DIR",
            fixture_cache_root(&fixture, "prepared-runtime-cache"),
        )
        .output()
        .expect("ss test should run through a pool child");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "pool child must re-admit the parent CLI package roots for both providers\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        stdout,
        stderr,
    );
    assert!(
        stdout.contains("\"passed\": 1") || stdout.contains("1 pass across 1 file"),
        "the exact multi-provider test case must settle passed\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.is_empty(),
        "ss test must keep stderr clean: {stderr}"
    );
}

struct Fixture {
    source_path: PathBuf,
}

fn write_fixture(name: &str, source: &str) -> Fixture {
    write_fixture_source(name, "main.ss", source)
}

fn write_fixture_source(name: &str, source_name: &str, source: &str) -> Fixture {
    let root = std::env::temp_dir().join(format!(
        "swarm-ss-external-capability-provider-{name}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&root).expect("fixture root should be created");
    let source_path = root.join(source_name);
    fs::write(&source_path, source).expect("fixture source should be written");
    Fixture { source_path }
}

fn fixture_cache_root(fixture: &Fixture, name: &str) -> PathBuf {
    fixture
        .source_path
        .parent()
        .expect("fixture source should have parent")
        .join(name)
}

fn write_io_application_package(root: &Path) {
    fs::write(
        root.join("package.json"),
        r#"{
  "name": "@swarm-fixture/io-app",
  "private": true,
  "type": "module",
  "exports": {
    ".": {
      "swarm": "./main.ss"
    }
  },
  "dependencies": {
    "@swarm/io": "workspace:*"
  }
}
"#,
    )
    .expect("io app package.json should be written");
}

fn write_pool_worker_multi_provider_application_package(root: &Path) {
    fs::write(
        root.join("package.json"),
        r#"{
  "name": "@swarm-fixture/counter-app",
  "private": true,
  "type": "module",
  "exports": {
    ".": "./main.test.ss",
    "./main.test.ss": "./main.test.ss"
  },
  "dependencies": {
    "@swarm/io": "workspace:*",
    "@swarm/test": "workspace:*"
  }
}
"#,
    )
    .expect("pool-worker multi-provider app package.json should be written");
}

fn write_counter_provider_package(root: &Path) {
    fs::write(
        root.join("package.json"),
        r#"{
  "name": "@swarm-fixture/counter-app",
  "private": true,
  "type": "module",
  "exports": {
    ".": "./main.ss"
  },
  "dependencies": {
    "@swarm-fixture/counter": "0.0.1"
  }
}
"#,
    )
    .expect("counter app package.json should be written");
    let package_root = root.join("node_modules/@swarm-fixture/counter");
    let src_root = package_root.join("src");
    fs::create_dir_all(&src_root).expect("counter provider package source root");
    fs::write(
        package_root.join("package.json"),
        r#"{
  "name": "@swarm-fixture/counter",
  "private": true,
  "type": "module",
  "exports": {
    ".": {
      "swarm": "./src/counter.contract.ts"
    },
    "./swarm": "./src/swarm.ts",
    "./swarm.ts": "./src/swarm.ts"
  },
  "dependencies": {
    "@swarm/provider": "workspace:*",
    "@swarm/swarmscript-types": "workspace:*"
  }
}
"#,
    )
    .expect("counter provider package.json should be written");
    fs::write(
        src_root.join("counter.contract.ts"),
        r#"import type { ErrorValue, Node, Result } from "@swarm/swarmscript-types/types.contract.ts";

export type CounterOutcome = Result<{ value: number }, ErrorValue>;

export interface Counter {
  first: (args: {}) => Node<CounterOutcome>;
  second: (args: {}) => Node<CounterOutcome>;
}

export declare const counter: Counter;
"#,
    )
    .expect("counter contract should be written");
    fs::write(
        src_root.join("swarm.ts"),
        r#"import { _, defineProvider } from "@swarm/provider";
import type { Counter, CounterOutcome } from "./counter.contract.ts";

let count = 0;

async function next(label: string): Promise<CounterOutcome> {
  count += 1;
  if ((label === "first" && count !== 1) || (label === "second" && count !== 2)) {
    return {
      kind: "err",
      error: {
        kind: "ErrorValue",
        domain: "swarm.fixture.counter",
        name: "counter_state_not_shared",
        message: `${label} observed count ${count}`,
      },
    };
  }
  return { kind: "ok", value: { value: count } };
}

const provider = {
  first: async () => await next("first"),
  second: async () => await next("second"),
} satisfies Counter;

export const { first, second } = defineProvider()
  .with(provider)
  .exports({
    first: _,
    second: _,
  });
"#,
    )
    .expect("counter provider module should be written");
    link_workspace_package(root, "@swarm/provider", "packages/provider");
    link_workspace_package(
        root,
        "@swarm/swarmscript-types",
        "packages/swarmscript-types",
    );
}

fn link_workspace_package(root: &Path, package_name: &str, workspace_relative_path: &str) {
    let mut package_path = root.join("node_modules");
    for segment in package_name.split('/') {
        package_path.push(segment);
    }
    if package_path.exists() {
        return;
    }
    let parent = package_path
        .parent()
        .expect("workspace package link should have parent");
    fs::create_dir_all(parent).expect("workspace package link parent should be created");
    let target = workspace_root().join(workspace_relative_path);
    #[cfg(unix)]
    symlink(&target, &package_path).unwrap_or_else(|error| {
        panic!(
            "failed to symlink workspace package from {} to {}: {error}",
            target.display(),
            package_path.display()
        )
    });
    #[cfg(not(unix))]
    panic!("external capability provider package fixture links require symlink support");
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("ss product crate should live under workspace crates directory")
        .to_path_buf()
}

fn bundle_development_libbun_plugin_next_to_ss_binary(ss_binary: &Path) {
    let asset = libbun::release::current_native_plugin_asset()
        .expect("libbun plugin asset metadata should exist for this host");
    let binary_dir = ss_binary
        .parent()
        .expect("test ss binary should have a parent directory");
    let bundled_plugin = binary_dir.join(asset.plugin_filename);

    let development_plugin = workspace_root()
        .parent()
        .expect("swarm workspace should have sibling repositories parent")
        .join("libbun/plugin/target/release")
        .join(asset.plugin_filename);
    assert!(
        development_plugin.is_file(),
        "release-profile libbun plugin must be built before running this test; expected {}. Build with LIBBUN_NATIVE_LINK_BUN=1 cargo +nightly-2026-05-06 build --release --manifest-path ../libbun/plugin/Cargo.toml",
        development_plugin.display()
    );
    fs::copy(&development_plugin, &bundled_plugin).unwrap_or_else(|error| {
        panic!(
            "failed to bundle development libbun plugin from {} to {}: {error}",
            development_plugin.display(),
            bundled_plugin.display()
        )
    });
}
