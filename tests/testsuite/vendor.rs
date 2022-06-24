//! Tests for the `payload vendor` command.
//!
//! Note that every test here uses `--respect-source-config` so that the
//! "fake" crates.io is used. Otherwise `vendor` would download the crates.io
//! index from the network.

use std::fs;

use payload_test_support::git;
use payload_test_support::registry::{self, Package};
use payload_test_support::{basic_lib_manifest, paths, project, Project};

#[payload_test]
fn vendor_simple() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                log = "0.3.5"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("log", "0.3.5").publish();

    p.payload("vendor --respect-source-config").run();
    let lock = p.read_file("vendor/log/Payload.toml");
    assert!(lock.contains("version = \"0.3.5\""));

    add_vendor_config(&p);
    p.payload("build").run();
}

fn add_vendor_config(p: &Project) {
    p.change_file(
        ".payload/config",
        r#"
            [source.crates-io]
            replace-with = 'vendor'

            [source.vendor]
            directory = 'vendor'
        "#,
    );
}

#[payload_test]
fn two_versions() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bitflags = "0.8.0"
                bar = { path = "bar" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "bar/Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"

                [dependencies]
                bitflags = "0.7.0"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    Package::new("bitflags", "0.7.0").publish();
    Package::new("bitflags", "0.8.0").publish();

    p.payload("vendor --respect-source-config").run();

    let lock = p.read_file("vendor/bitflags/Payload.toml");
    assert!(lock.contains("version = \"0.8.0\""));
    let lock = p.read_file("vendor/bitflags-0.7.0/Payload.toml");
    assert!(lock.contains("version = \"0.7.0\""));

    add_vendor_config(&p);
    p.payload("build").run();
}

#[payload_test]
fn two_explicit_versions() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bitflags = "0.8.0"
                bar = { path = "bar" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "bar/Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"

                [dependencies]
                bitflags = "0.7.0"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    Package::new("bitflags", "0.7.0").publish();
    Package::new("bitflags", "0.8.0").publish();

    p.payload("vendor --respect-source-config --versioned-dirs")
        .run();

    let lock = p.read_file("vendor/bitflags-0.8.0/Payload.toml");
    assert!(lock.contains("version = \"0.8.0\""));
    let lock = p.read_file("vendor/bitflags-0.7.0/Payload.toml");
    assert!(lock.contains("version = \"0.7.0\""));

    add_vendor_config(&p);
    p.payload("build").run();
}

#[payload_test]
fn help() {
    let p = project().build();
    p.payload("vendor -h").run();
}

#[payload_test]
fn update_versions() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bitflags = "0.7.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("bitflags", "0.7.0").publish();
    Package::new("bitflags", "0.8.0").publish();

    p.payload("vendor --respect-source-config").run();

    let lock = p.read_file("vendor/bitflags/Payload.toml");
    assert!(lock.contains("version = \"0.7.0\""));

    p.change_file(
        "Payload.toml",
        r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bitflags = "0.8.0"
        "#,
    );
    p.payload("vendor --respect-source-config").run();

    let lock = p.read_file("vendor/bitflags/Payload.toml");
    assert!(lock.contains("version = \"0.8.0\""));
}

#[payload_test]
fn two_lockfiles() {
    let p = project()
        .no_manifest()
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bitflags = "=0.7.0"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .file(
            "bar/Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"

                [dependencies]
                bitflags = "=0.8.0"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    Package::new("bitflags", "0.7.0").publish();
    Package::new("bitflags", "0.8.0").publish();

    p.payload("vendor --respect-source-config -s bar/Payload.toml --manifest-path foo/Payload.toml")
        .run();

    let lock = p.read_file("vendor/bitflags/Payload.toml");
    assert!(lock.contains("version = \"0.8.0\""));
    let lock = p.read_file("vendor/bitflags-0.7.0/Payload.toml");
    assert!(lock.contains("version = \"0.7.0\""));

    add_vendor_config(&p);
    p.payload("build").cwd("foo").run();
    p.payload("build").cwd("bar").run();
}

#[payload_test]
fn delete_old_crates() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bitflags = "=0.7.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("bitflags", "0.7.0").publish();
    Package::new("log", "0.3.5").publish();

    p.payload("vendor --respect-source-config").run();
    p.read_file("vendor/bitflags/Payload.toml");

    p.change_file(
        "Payload.toml",
        r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            log = "=0.3.5"
        "#,
    );

    p.payload("vendor --respect-source-config").run();
    let lock = p.read_file("vendor/log/Payload.toml");
    assert!(lock.contains("version = \"0.3.5\""));
    assert!(!p.root().join("vendor/bitflags/Payload.toml").exists());
}

#[payload_test]
fn ignore_files() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                url = "1.4.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("url", "1.4.1")
        .file("src/lib.rs", "")
        .file("foo.orig", "")
        .file(".gitignore", "")
        .file(".gitattributes", "")
        .file("foo.rej", "")
        .publish();

    p.payload("vendor --respect-source-config").run();
    let csum = p.read_file("vendor/url/.payload-checksum.json");
    assert!(!csum.contains("foo.orig"));
    assert!(!csum.contains(".gitignore"));
    assert!(!csum.contains(".gitattributes"));
    assert!(!csum.contains(".payload-ok"));
    assert!(!csum.contains("foo.rej"));
}

#[payload_test]
fn included_files_only() {
    let git = git::new("a", |p| {
        p.file("Payload.toml", &basic_lib_manifest("a"))
            .file("src/lib.rs", "")
            .file(".gitignore", "a")
            .file("a/b.md", "")
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies]
                    a = {{ git = '{}' }}
                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("vendor --respect-source-config").run();
    let csum = p.read_file("vendor/a/.payload-checksum.json");
    assert!(!csum.contains("a/b.md"));
}

#[payload_test]
fn dependent_crates_in_crates() {
    let git = git::new("a", |p| {
        p.file(
            "Payload.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"

                [dependencies]
                b = { path = 'b' }
            "#,
        )
        .file("src/lib.rs", "")
        .file("b/Payload.toml", &basic_lib_manifest("b"))
        .file("b/src/lib.rs", "")
    });
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies]
                    a = {{ git = '{}' }}
                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("vendor --respect-source-config").run();
    p.read_file("vendor/a/.payload-checksum.json");
    p.read_file("vendor/b/.payload-checksum.json");
}

#[payload_test]
fn vendoring_git_crates() {
    let git = git::new("git", |p| {
        p.file("Payload.toml", &basic_lib_manifest("serde_derive"))
            .file("src/lib.rs", "")
            .file("src/wut.rs", "")
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies.serde]
                    version = "0.5.0"

                    [dependencies.serde_derive]
                    version = "0.5.0"

                    [patch.crates-io]
                    serde_derive = {{ git = '{}' }}
                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();
    Package::new("serde", "0.5.0")
        .dep("serde_derive", "0.5")
        .publish();
    Package::new("serde_derive", "0.5.0").publish();

    p.payload("vendor --respect-source-config").run();
    p.read_file("vendor/serde_derive/src/wut.rs");

    add_vendor_config(&p);
    p.payload("build").run();
}

#[payload_test]
fn git_simple() {
    let git = git::new("git", |p| {
        p.file("Payload.toml", &basic_lib_manifest("a"))
            .file("src/lib.rs", "")
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies]
                    a = {{ git = '{}' }}
                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("vendor --respect-source-config").run();
    let csum = p.read_file("vendor/a/.payload-checksum.json");
    assert!(csum.contains("\"package\":null"));
}

#[payload_test]
fn git_duplicate() {
    let git = git::new("a", |p| {
        p.file(
            "Payload.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"

                [dependencies]
                b = { path = 'b' }
            "#,
        )
        .file("src/lib.rs", "")
        .file("b/Payload.toml", &basic_lib_manifest("b"))
        .file("b/src/lib.rs", "")
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies]
                    a = {{ git = '{}' }}
                    b = '0.5.0'

                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();
    Package::new("b", "0.5.0").publish();

    p.payload("vendor --respect-source-config")
        .with_stderr(
            "\
[UPDATING] [..]
[UPDATING] [..]
[DOWNLOADING] [..]
[DOWNLOADED] [..]
error: failed to sync

Caused by:
  found duplicate version of package `b v0.5.0` vendored from two sources:

  <tab>source 1: [..]
  <tab>source 2: [..]
",
        )
        .with_status(101)
        .run();
}

#[payload_test]
fn depend_on_vendor_dir_not_deleted() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                libc = "0.2.30"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("libc", "0.2.30").publish();

    p.payload("vendor --respect-source-config").run();
    assert!(p.root().join("vendor/libc").is_dir());

    p.change_file(
        "Payload.toml",
        r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            libc = "0.2.30"

            [patch.crates-io]
            libc = { path = 'vendor/libc' }
        "#,
    );

    p.payload("vendor --respect-source-config").run();
    assert!(p.root().join("vendor/libc").is_dir());
}

#[payload_test]
fn ignore_hidden() {
    // Don't delete files starting with `.`
    Package::new("bar", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "1.0.0"
            [dependencies]
            bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.payload("vendor --respect-source-config").run();
    // Add a `.git` directory.
    let repo = git::init(&p.root().join("vendor"));
    git::add(&repo);
    git::commit(&repo);
    assert!(p.root().join("vendor/.git").exists());
    // Vendor again, shouldn't change anything.
    p.payload("vendor --respect-source-config").run();
    // .git should not be removed.
    assert!(p.root().join("vendor/.git").exists());
    // And just for good measure, make sure no files changed.
    let mut opts = git2::StatusOptions::new();
    assert!(repo
        .statuses(Some(&mut opts))
        .unwrap()
        .iter()
        .all(|status| status.status() == git2::Status::CURRENT));
}

#[payload_test]
fn config_instructions_works() {
    // Check that the config instructions work for all dependency kinds.
    registry::alt_init();
    Package::new("dep", "0.1.0").publish();
    Package::new("altdep", "0.1.0").alternative(true).publish();
    let git_project = git::new("gitdep", |project| {
        project
            .file("Payload.toml", &basic_lib_manifest("gitdep"))
            .file("src/lib.rs", "")
    });
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                dep = "0.1"
                altdep = {{version="0.1", registry="alternative"}}
                gitdep = {{git='{}'}}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();
    let output = p
        .payload("vendor --respect-source-config")
        .exec_with_output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    p.change_file(".payload/config", &output);

    p.payload("check -v")
        .with_stderr_contains("[..]foo/vendor/dep/src/lib.rs[..]")
        .with_stderr_contains("[..]foo/vendor/altdep/src/lib.rs[..]")
        .with_stderr_contains("[..]foo/vendor/gitdep/src/lib.rs[..]")
        .run();
}

#[payload_test]
fn git_crlf_preservation() {
    // Check that newlines don't get changed when you vendor
    // (will only fail if your system is setup with core.autocrlf=true on windows)
    let input = "hello \nthere\nmy newline\nfriends";
    let git_project = git::new("git", |p| {
        p.file("Payload.toml", &basic_lib_manifest("a"))
            .file("src/lib.rs", input)
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.1.0"

                    [dependencies]
                    a = {{ git = '{}' }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    fs::write(
        paths::home().join(".gitconfig"),
        r#"
            [core]
            autocrlf = true
        "#,
    )
    .unwrap();

    p.payload("vendor --respect-source-config").run();
    let output = p.read_file("vendor/a/src/lib.rs");
    assert_eq!(input, output);
}

#[payload_test]
#[cfg(unix)]
fn vendor_preserves_permissions() {
    use std::os::unix::fs::MetadataExt;

    Package::new("bar", "1.0.0")
        .file_with_mode("example.sh", 0o755, "#!/bin/sh")
        .file("src/lib.rs", "")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("vendor --respect-source-config").run();

    let metadata = fs::metadata(p.root().join("vendor/bar/src/lib.rs")).unwrap();
    assert_eq!(metadata.mode() & 0o777, 0o644);
    let metadata = fs::metadata(p.root().join("vendor/bar/example.sh")).unwrap();
    assert_eq!(metadata.mode() & 0o777, 0o755);
}
