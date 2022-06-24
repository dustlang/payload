//! Tests for the metabuild feature (declarative build scripts).

use payload_test_support::{
    basic_lib_manifest, basic_manifest, is_coarse_mtime, project, registry::Package, rustc_host,
    Project,
};

use std::str;

#[payload_test]
fn metabuild_gated() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = ["mb"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr_contains(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  feature `metabuild` is required

  consider adding `payload-features = [\"metabuild\"]` to the manifest
",
        )
        .run();
}

fn basic_project() -> Project {
    project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = ["mb", "mb-other"]

                [build-dependencies]
                mb = {path="mb"}
                mb-other = {path="mb-other"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .file(
            "mb-other/Payload.toml",
            r#"
                [package]
                name = "mb-other"
                version = "0.0.1"
            "#,
        )
        .file(
            "mb-other/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb-other"); }"#,
        )
        .build()
}

#[payload_test]
fn metabuild_basic() {
    let p = basic_project();
    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[foo 0.0.1] Hello mb")
        .with_stdout_contains("[foo 0.0.1] Hello mb-other")
        .run();
}

#[payload_test]
fn metabuild_error_both() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", r#"fn main() {}"#)
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr_contains(
            "\
error: failed to parse manifest at [..]

Caused by:
  cannot specify both `metabuild` and `build`
",
        )
        .run();
}

#[payload_test]
fn metabuild_missing_dep() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = "mb"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr_contains(
            "\
error: failed to parse manifest at [..]

Caused by:
  metabuild package `mb` must be specified in `build-dependencies`",
        )
        .run();
}

#[payload_test]
fn metabuild_optional_dep() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb", optional=true}
            "#,
        )
        .file("src/lib.rs", "")
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_does_not_contain("[foo 0.0.1] Hello mb")
        .run();

    p.payload("build -vv --features mb")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[foo 0.0.1] Hello mb")
        .run();
}

#[payload_test]
fn metabuild_lib_name() {
    // Test when setting `name` on [lib].
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb"}
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "mb/Payload.toml",
            r#"
                [package]
                name = "mb"
                version = "0.0.1"
                [lib]
                name = "other"
            "#,
        )
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[foo 0.0.1] Hello mb")
        .run();
}

#[payload_test]
fn metabuild_fresh() {
    if is_coarse_mtime() {
        // This test doesn't work on coarse mtimes very well. Because the
        // metabuild script is created at build time, its mtime is almost
        // always equal to the mtime of the output. The second call to `build`
        // will then think it needs to be rebuilt when it should be fresh.
        return;
    }

    // Check that rebuild is fresh.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[foo 0.0.1] Hello mb")
        .run();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_does_not_contain("[foo 0.0.1] Hello mb")
        .with_stderr(
            "\
[FRESH] mb [..]
[FRESH] foo [..]
[FINISHED] dev [..]
",
        )
        .run();
}

#[payload_test]
fn metabuild_links() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                links = "cat"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"
                pub fn metabuild() {
                    assert_eq!(std::env::var("PAYLOAD_MANIFEST_LINKS"),
                        Ok("cat".to_string()));
                    println!("Hello mb");
                }
            "#,
        )
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[foo 0.0.1] Hello mb")
        .run();
}

#[payload_test]
fn metabuild_override() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "foo"
                version = "0.0.1"
                links = "cat"
                metabuild = "mb"

                [build-dependencies]
                mb = {path="mb"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("mb/Payload.toml", &basic_lib_manifest("mb"))
        .file(
            "mb/src/lib.rs",
            r#"pub fn metabuild() { panic!("should not run"); }"#,
        )
        .file(
            ".payload/config",
            &format!(
                r#"
                    [target.{}.cat]
                    rustc-link-lib = ["a"]
                "#,
                rustc_host()
            ),
        )
        .build();

    p.payload("build -vv").masquerade_as_nightly_payload().run();
}

#[payload_test]
fn metabuild_workspace() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["member1", "member2"]
            "#,
        )
        .file(
            "member1/Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "member1"
                version = "0.0.1"
                metabuild = ["mb1", "mb2"]

                [build-dependencies]
                mb1 = {path="../../mb1"}
                mb2 = {path="../../mb2"}
            "#,
        )
        .file("member1/src/lib.rs", "")
        .file(
            "member2/Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "member2"
                version = "0.0.1"
                metabuild = ["mb1"]

                [build-dependencies]
                mb1 = {path="../../mb1"}
            "#,
        )
        .file("member2/src/lib.rs", "")
        .build();

    project()
        .at("mb1")
        .file("Payload.toml", &basic_lib_manifest("mb1"))
        .file(
            "src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb1 {}", std::env::var("PAYLOAD_MANIFEST_DIR").unwrap()); }"#,
        )
        .build();

    project()
        .at("mb2")
        .file("Payload.toml", &basic_lib_manifest("mb2"))
        .file(
            "src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb2 {}", std::env::var("PAYLOAD_MANIFEST_DIR").unwrap()); }"#,
        )
        .build();

    p.payload("build -vv --workspace")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[member1 0.0.1] Hello mb1 [..]member1")
        .with_stdout_contains("[member1 0.0.1] Hello mb2 [..]member1")
        .with_stdout_contains("[member2 0.0.1] Hello mb1 [..]member2")
        .with_stdout_does_not_contain("[member2 0.0.1] Hello mb2 [..]member2")
        .run();
}

#[payload_test]
fn metabuild_metadata() {
    // The metabuild Target is filtered out of the `metadata` results.
    let p = basic_project();

    let output = p
        .payload("metadata --format-version=1")
        .masquerade_as_nightly_payload()
        .exec_with_output()
        .expect("payload metadata failed");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let meta: serde_json::Value = serde_json::from_str(stdout).expect("failed to parse json");
    let mb_info: Vec<&str> = meta["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"].as_str().unwrap() == "foo")
        .unwrap()["metabuild"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap())
        .collect();
    assert_eq!(mb_info, ["mb", "mb-other"]);
}

#[payload_test]
fn metabuild_build_plan() {
    let p = basic_project();

    p.payload("build --build-plan -Zunstable-options")
        .masquerade_as_nightly_payload()
        .with_json(
            r#"
            {
                "invocations": [
                    {
                        "package_name": "mb",
                        "package_version": "0.5.0",
                        "target_kind": ["lib"],
                        "compile_mode": "build",
                        "kind": null,
                        "deps": [],
                        "outputs": [
                            "[..]/target/debug/deps/libmb-[..].rlib",
                            "[..]/target/debug/deps/libmb-[..].rmeta"
                        ],
                        "links": {},
                        "program": "rustc",
                        "args": "{...}",
                        "env": "{...}",
                        "cwd": "[..]"
                    },
                    {
                        "package_name": "mb-other",
                        "package_version": "0.0.1",
                        "target_kind": ["lib"],
                        "compile_mode": "build",
                        "kind": null,
                        "deps": [],
                        "outputs": [
                            "[..]/target/debug/deps/libmb_other-[..].rlib",
                            "[..]/target/debug/deps/libmb_other-[..].rmeta"
                        ],
                        "links": {},
                        "program": "rustc",
                        "args": "{...}",
                        "env": "{...}",
                        "cwd": "[..]"
                    },
                    {
                        "package_name": "foo",
                        "package_version": "0.0.1",
                        "target_kind": ["custom-build"],
                        "compile_mode": "build",
                        "kind": null,
                        "deps": [0, 1],
                        "outputs": "{...}",
                        "links": "{...}",
                        "program": "rustc",
                        "args": "{...}",
                        "env": "{...}",
                        "cwd": "[..]"
                    },
                    {
                        "package_name": "foo",
                        "package_version": "0.0.1",
                        "target_kind": ["custom-build"],
                        "compile_mode": "run-custom-build",
                        "kind": null,
                        "deps": [2],
                        "outputs": [],
                        "links": {},
                        "program": "[..]/foo/target/debug/build/foo-[..]/metabuild-foo",
                        "args": [],
                        "env": "{...}",
                        "cwd": "[..]"
                    },
                    {
                        "package_name": "foo",
                        "package_version": "0.0.1",
                        "target_kind": ["lib"],
                        "compile_mode": "build",
                        "kind": null,
                        "deps": [3],
                        "outputs": [
                            "[..]/foo/target/debug/deps/libfoo-[..].rlib",
                            "[..]/foo/target/debug/deps/libfoo-[..].rmeta"
                        ],
                        "links": "{...}",
                        "program": "rustc",
                        "args": "{...}",
                        "env": "{...}",
                        "cwd": "[..]"
                    }
                ],
                "inputs": [
                    "[..]/foo/Payload.toml",
                    "[..]/foo/mb/Payload.toml",
                    "[..]/foo/mb-other/Payload.toml"
                ]
            }
            "#,
        )
        .run();

    assert_eq!(p.glob("target/.metabuild/metabuild-foo-*.rs").count(), 1);
}

#[payload_test]
fn metabuild_two_versions() {
    // Two versions of a metabuild dep with the same name.
    let p = project()
        .at("ws")
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["member1", "member2"]
            "#,
        )
        .file(
            "member1/Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "member1"
                version = "0.0.1"
                metabuild = ["mb"]

                [build-dependencies]
                mb = {path="../../mb1"}
            "#,
        )
        .file("member1/src/lib.rs", "")
        .file(
            "member2/Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "member2"
                version = "0.0.1"
                metabuild = ["mb"]

                [build-dependencies]
                mb = {path="../../mb2"}
            "#,
        )
        .file("member2/src/lib.rs", "")
        .build();

    project().at("mb1")
        .file("Payload.toml", r#"
            [package]
            name = "mb"
            version = "0.0.1"
        "#)
        .file(
            "src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb1 {}", std::env::var("PAYLOAD_MANIFEST_DIR").unwrap()); }"#,
        )
        .build();

    project().at("mb2")
        .file("Payload.toml", r#"
            [package]
            name = "mb"
            version = "0.0.2"
        "#)
        .file(
            "src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb2 {}", std::env::var("PAYLOAD_MANIFEST_DIR").unwrap()); }"#,
        )
        .build();

    p.payload("build -vv --workspace")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[member1 0.0.1] Hello mb1 [..]member1")
        .with_stdout_contains("[member2 0.0.1] Hello mb2 [..]member2")
        .run();

    assert_eq!(
        p.glob("target/.metabuild/metabuild-member?-*.rs").count(),
        2
    );
}

#[payload_test]
fn metabuild_external_dependency() {
    Package::new("mb", "1.0.0")
        .file("Payload.toml", &basic_manifest("mb", "1.0.0"))
        .file(
            "src/lib.rs",
            r#"pub fn metabuild() { println!("Hello mb"); }"#,
        )
        .publish();
    Package::new("dep", "1.0.0")
        .file(
            "Payload.toml",
            r#"
                payload-features = ["metabuild"]
                [package]
                name = "dep"
                version = "1.0.0"
                metabuild = ["mb"]

                [build-dependencies]
                mb = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build_dep("mb", "1.0.0")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            [dependencies]
            dep = "1.0"
            "#,
        )
        .file("src/lib.rs", "extern crate dep;")
        .build();

    p.payload("build -vv")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("[dep 1.0.0] Hello mb")
        .run();

    assert_eq!(p.glob("target/.metabuild/metabuild-dep-*.rs").count(), 1);
}

#[payload_test]
fn metabuild_json_artifact() {
    let p = basic_project();
    p.payload("build --message-format=json")
        .masquerade_as_nightly_payload()
        .with_json_contains_unordered(
            r#"
            {
              "executable": null,
              "features": [],
              "filenames": "{...}",
              "fresh": false,
              "package_id": "foo [..]",
              "manifest_path": "[..]",
              "profile": "{...}",
              "reason": "compiler-artifact",
              "target": {
                "crate_types": [
                  "bin"
                ],
                "doc": false,
                "doctest": false,
                "edition": "2018",
                "kind": [
                  "custom-build"
                ],
                "name": "metabuild-foo",
                "src_path": "[..]/foo/target/.metabuild/metabuild-foo-[..].rs",
                "test": false
              }
            }

            {
              "cfgs": [],
              "env": [],
              "linked_libs": [],
              "linked_paths": [],
              "package_id": "foo [..]",
              "out_dir": "[..]",
              "reason": "build-script-executed"
            }
            "#,
        )
        .run();
}

#[payload_test]
fn metabuild_failed_build_json() {
    let p = basic_project();
    // Modify the metabuild dep so that it fails to compile.
    p.change_file("mb/src/lib.rs", "");
    p.payload("build --message-format=json")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_json_contains_unordered(
            r#"
            {
              "message": {
                "children": "{...}",
                "code": "{...}",
                "level": "error",
                "message": "cannot find function `metabuild` in [..] `mb`",
                "rendered": "[..]",
                "spans": "{...}"
              },
              "package_id": "foo [..]",
              "manifest_path": "[..]",
              "reason": "compiler-message",
              "target": {
                "crate_types": [
                  "bin"
                ],
                "doc": false,
                "doctest": false,
                "edition": "2018",
                "kind": [
                  "custom-build"
                ],
                "name": "metabuild-foo",
                "src_path": null,
                "test": false
              }
            }
            "#,
        )
        .run();
}
