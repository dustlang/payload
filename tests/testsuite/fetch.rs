//! Tests for the `payload fetch` command.

use payload_test_support::registry::Package;
use payload_test_support::rustc_host;
use payload_test_support::{basic_manifest, cross_compile, project};

#[payload_test]
fn no_deps() {
    let p = project()
        .file("src/main.rs", "mod a; fn main() {}")
        .file("src/a.rs", "")
        .build();

    p.payload("fetch").with_stdout("").run();
}

#[payload_test]
fn fetch_all_platform_dependencies_when_no_target_is_given() {
    if cross_compile::disabled() {
        return;
    }

    Package::new("d1", "1.2.3")
        .file("Payload.toml", &basic_manifest("d1", "1.2.3"))
        .file("src/lib.rs", "")
        .publish();

    Package::new("d2", "0.1.2")
        .file("Payload.toml", &basic_manifest("d2", "0.1.2"))
        .file("src/lib.rs", "")
        .publish();

    let target = cross_compile::alternate();
    let host = rustc_host();
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.0.1"
                    authors = []

                    [target.{host}.dependencies]
                    d1 = "1.2.3"

                    [target.{target}.dependencies]
                    d2 = "0.1.2"
                "#,
                host = host,
                target = target
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("fetch")
        .with_stderr_contains("[DOWNLOADED] d1 v1.2.3 [..]")
        .with_stderr_contains("[DOWNLOADED] d2 v0.1.2 [..]")
        .run();
}

#[payload_test]
fn fetch_platform_specific_dependencies() {
    if cross_compile::disabled() {
        return;
    }

    Package::new("d1", "1.2.3")
        .file("Payload.toml", &basic_manifest("d1", "1.2.3"))
        .file("src/lib.rs", "")
        .publish();

    Package::new("d2", "0.1.2")
        .file("Payload.toml", &basic_manifest("d2", "0.1.2"))
        .file("src/lib.rs", "")
        .publish();

    let target = cross_compile::alternate();
    let host = rustc_host();
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.0.1"
                    authors = []

                    [target.{host}.dependencies]
                    d1 = "1.2.3"

                    [target.{target}.dependencies]
                    d2 = "0.1.2"
                "#,
                host = host,
                target = target
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("fetch --target")
        .arg(&host)
        .with_stderr_contains("[DOWNLOADED] d1 v1.2.3 [..]")
        .with_stderr_does_not_contain("[DOWNLOADED] d2 v0.1.2 [..]")
        .run();

    p.payload("fetch --target")
        .arg(&target)
        .with_stderr_contains("[DOWNLOADED] d2 v0.1.2[..]")
        .with_stderr_does_not_contain("[DOWNLOADED] d1 v1.2.3 [..]")
        .run();
}

#[payload_test]
fn fetch_warning() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "1.0.0"
            misspelled = "wut"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.payload("fetch")
        .with_stderr("[WARNING] unused manifest key: package.misspelled")
        .run();
}
