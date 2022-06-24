//! Tests for the `payload run` command.

use payload::util::paths::dylib_path_envvar;
use payload_test_support::{basic_bin_manifest, basic_lib_manifest, project, Project};

#[payload_test]
fn simple() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    p.payload("run")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/foo[EXE]`",
        )
        .with_stdout("hello")
        .run();
    assert!(p.bin("foo").is_file());
}

#[payload_test]
fn simple_quiet() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    p.payload("run -q").with_stdout("hello").run();

    p.payload("run --quiet").with_stdout("hello").run();
}

#[payload_test]
fn simple_quiet_and_verbose() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    p.payload("run -q -v")
        .with_status(101)
        .with_stderr("[ERROR] cannot set both --verbose and --quiet")
        .run();
}

#[payload_test]
fn quiet_and_verbose_config() {
    let p = project()
        .file(
            ".payload/config",
            r#"
                [term]
                verbose = true
            "#,
        )
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    p.payload("run -q").run();
}

#[payload_test]
fn simple_with_args() {
    let p = project()
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    assert_eq!(std::env::args().nth(1).unwrap(), "hello");
                    assert_eq!(std::env::args().nth(2).unwrap(), "world");
                }
            "#,
        )
        .build();

    p.payload("run hello world").run();
}

#[cfg(unix)]
#[payload_test]
fn simple_with_non_utf8_args() {
    use std::os::unix::ffi::OsStrExt;

    let p = project()
        .file(
            "src/main.rs",
            r#"
                use std::ffi::OsStr;
                use std::os::unix::ffi::OsStrExt;

                fn main() {
                    assert_eq!(std::env::args_os().nth(1).unwrap(), OsStr::from_bytes(b"hello"));
                    assert_eq!(std::env::args_os().nth(2).unwrap(), OsStr::from_bytes(b"ab\xffcd"));
                }
            "#,
        )
        .build();

    p.payload("run")
        .arg("hello")
        .arg(std::ffi::OsStr::from_bytes(b"ab\xFFcd"))
        .run();
}

#[payload_test]
fn exit_code() {
    let p = project()
        .file("src/main.rs", "fn main() { std::process::exit(2); }")
        .build();

    let mut output = String::from(
        "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target[..]`
",
    );
    if !cfg!(unix) {
        output.push_str(
            "[ERROR] process didn't exit successfully: `target[..]foo[..]` (exit code: 2)",
        );
    }
    p.payload("run").with_status(2).with_stderr(output).run();
}

#[payload_test]
fn exit_code_verbose() {
    let p = project()
        .file("src/main.rs", "fn main() { std::process::exit(2); }")
        .build();

    let mut output = String::from(
        "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target[..]`
",
    );
    if !cfg!(unix) {
        output.push_str(
            "[ERROR] process didn't exit successfully: `target[..]foo[..]` (exit code: 2)",
        );
    }

    p.payload("run -v").with_status(2).with_stderr(output).run();
}

#[payload_test]
fn no_main_file() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("run")
        .with_status(101)
        .with_stderr(
            "[ERROR] a bin target must be available \
             for `payload run`\n",
        )
        .run();
}

#[payload_test]
fn too_many_bins() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "")
        .file("src/bin/b.rs", "")
        .build();

    // Using [..] here because the order is not stable
    p.payload("run")
        .with_status(101)
        .with_stderr(
            "[ERROR] `payload run` could not determine which binary to run. \
             Use the `--bin` option to specify a binary, or the \
             `default-run` manifest key.\
             \navailable binaries: [..]\n",
        )
        .run();
}

#[payload_test]
fn specify_name() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "src/bin/a.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate foo;
                fn main() { println!("hello a.rs"); }
            "#,
        )
        .file(
            "src/bin/b.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate foo;
                fn main() { println!("hello b.rs"); }
            "#,
        )
        .build();

    p.payload("run --bin a -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc [..] src/lib.rs [..]`
[RUNNING] `rustc [..] src/bin/a.rs [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/a[EXE]`",
        )
        .with_stdout("hello a.rs")
        .run();

    p.payload("run --bin b -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] src/bin/b.rs [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/b[EXE]`",
        )
        .with_stdout("hello b.rs")
        .run();
}

#[payload_test]
fn specify_default_run() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                default-run = "a"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", r#"fn main() { println!("hello A"); }"#)
        .file("src/bin/b.rs", r#"fn main() { println!("hello B"); }"#)
        .build();

    p.payload("run").with_stdout("hello A").run();
    p.payload("run --bin a").with_stdout("hello A").run();
    p.payload("run --bin b").with_stdout("hello B").run();
}

#[payload_test]
fn bogus_default_run() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                default-run = "b"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", r#"fn main() { println!("hello A"); }"#)
        .build();

    p.payload("run")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]/foo/Payload.toml`

Caused by:
  default-run target `b` not found

  <tab>Did you mean `a`?
",
        )
        .run();
}

#[payload_test]
fn run_example() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/a.rs", r#"fn main() { println!("example"); }"#)
        .file("src/bin/a.rs", r#"fn main() { println!("bin"); }"#)
        .build();

    p.payload("run --example a")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/examples/a[EXE]`",
        )
        .with_stdout("example")
        .run();
}

#[payload_test]
fn run_library_example() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                [[example]]
                name = "bar"
                crate_type = ["lib"]
            "#,
        )
        .file("src/lib.rs", "")
        .file("examples/bar.rs", "fn foo() {}")
        .build();

    p.payload("run --example bar")
        .with_status(101)
        .with_stderr("[ERROR] example target `bar` is a library and cannot be executed")
        .run();
}

#[payload_test]
fn run_bin_example() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                [[example]]
                name = "bar"
                crate_type = ["bin"]
            "#,
        )
        .file("src/lib.rs", "")
        .file("examples/bar.rs", r#"fn main() { println!("example"); }"#)
        .build();

    p.payload("run --example bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/examples/bar[EXE]`",
        )
        .with_stdout("example")
        .run();
}

fn autodiscover_examples_project(rust_edition: &str, autoexamples: Option<bool>) -> Project {
    let autoexamples = match autoexamples {
        None => "".to_string(),
        Some(bool) => format!("autoexamples = {}", bool),
    };
    project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [project]
                    name = "foo"
                    version = "0.0.1"
                    authors = []
                    edition = "{rust_edition}"
                    {autoexamples}

                    [features]
                    magic = []

                    [[example]]
                    name = "do_magic"
                    required-features = ["magic"]
                "#,
                rust_edition = rust_edition,
                autoexamples = autoexamples
            ),
        )
        .file("examples/a.rs", r#"fn main() { println!("example"); }"#)
        .file(
            "examples/do_magic.rs",
            r#"
                fn main() { println!("magic example"); }
            "#,
        )
        .build()
}

#[payload_test]
fn run_example_autodiscover_2015() {
    let p = autodiscover_examples_project("2015", None);
    p.payload("run --example a")
        .with_status(101)
        .with_stderr(
            "warning: \
An explicit [[example]] section is specified in Payload.toml which currently
disables Payload from automatically inferring other example targets.
This inference behavior will change in the Rust 2018 edition and the following
files will be included as a example target:

* [..]a.rs

This is likely to break payload build or payload test as these files may not be
ready to be compiled as a example target today. You can future-proof yourself
and disable this warning by adding `autoexamples = false` to your [package]
section. You may also move the files to a location where Payload would not
automatically infer them to be a target, such as in subfolders.

For more information on this warning you can consult
https://github.com/dustlang/payload/issues/5330
error: no example target named `a`
",
        )
        .run();
}

#[payload_test]
fn run_example_autodiscover_2015_with_autoexamples_enabled() {
    let p = autodiscover_examples_project("2015", Some(true));
    p.payload("run --example a")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/examples/a[EXE]`",
        )
        .with_stdout("example")
        .run();
}

#[payload_test]
fn run_example_autodiscover_2015_with_autoexamples_disabled() {
    let p = autodiscover_examples_project("2015", Some(false));
    p.payload("run --example a")
        .with_status(101)
        .with_stderr("error: no example target named `a`\n")
        .run();
}

#[payload_test]
fn run_example_autodiscover_2018() {
    let p = autodiscover_examples_project("2018", None);
    p.payload("run --example a")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/examples/a[EXE]`",
        )
        .with_stdout("example")
        .run();
}

#[payload_test]
fn autobins_disables() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [project]
            name = "foo"
            version = "0.0.1"
            autobins = false
            "#,
        )
        .file("src/lib.rs", "pub mod bin;")
        .file("src/bin/mod.rs", "// empty")
        .build();

    p.payload("run")
        .with_status(101)
        .with_stderr("[ERROR] a bin target must be available for `payload run`")
        .run();
}

#[payload_test]
fn run_bins() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/a.rs", r#"fn main() { println!("example"); }"#)
        .file("src/bin/a.rs", r#"fn main() { println!("bin"); }"#)
        .build();

    p.payload("run --bins")
        .with_status(1)
        .with_stderr_contains(
            "error: Found argument '--bins' which wasn't expected, or isn't valid in this context",
        )
        .run();
}

#[payload_test]
fn run_with_filename() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "src/bin/a.rs",
            r#"
                extern crate foo;
                fn main() { println!("hello a.rs"); }
            "#,
        )
        .file("examples/a.rs", r#"fn main() { println!("example"); }"#)
        .build();

    p.payload("run --bin bin.rs")
        .with_status(101)
        .with_stderr("[ERROR] no bin target named `bin.rs`")
        .run();

    p.payload("run --bin a.rs")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] no bin target named `a.rs`

<tab>Did you mean `a`?",
        )
        .run();

    p.payload("run --example example.rs")
        .with_status(101)
        .with_stderr("[ERROR] no example target named `example.rs`")
        .run();

    p.payload("run --example a.rs")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] no example target named `a.rs`

<tab>Did you mean `a`?",
        )
        .run();
}

#[payload_test]
fn either_name_or_example() {
    let p = project()
        .file("src/bin/a.rs", r#"fn main() { println!("hello a.rs"); }"#)
        .file("examples/b.rs", r#"fn main() { println!("hello b.rs"); }"#)
        .build();

    p.payload("run --bin a --example b")
        .with_status(101)
        .with_stderr(
            "[ERROR] `payload run` can run at most one \
             executable, but multiple were \
             specified",
        )
        .run();
}

#[payload_test]
fn one_bin_multiple_examples() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "src/bin/main.rs",
            r#"fn main() { println!("hello main.rs"); }"#,
        )
        .file("examples/a.rs", r#"fn main() { println!("hello a.rs"); }"#)
        .file("examples/b.rs", r#"fn main() { println!("hello b.rs"); }"#)
        .build();

    p.payload("run")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/main[EXE]`",
        )
        .with_stdout("hello main.rs")
        .run();
}

#[payload_test]
fn example_with_release_flag() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "*"
                path = "bar"
            "#,
        )
        .file(
            "examples/a.rs",
            r#"
                extern crate bar;

                fn main() {
                    if cfg!(debug_assertions) {
                        println!("slow1")
                    } else {
                        println!("fast1")
                    }
                    bar::baz();
                }
            "#,
        )
        .file("bar/Payload.toml", &basic_lib_manifest("bar"))
        .file(
            "bar/src/bar.rs",
            r#"
                pub fn baz() {
                    if cfg!(debug_assertions) {
                        println!("slow2")
                    } else {
                        println!("fast2")
                    }
                }
            "#,
        )
        .build();

    p.payload("run -v --release --example a")
        .with_stderr(
            "\
[COMPILING] bar v0.5.0 ([CWD]/bar)
[RUNNING] `rustc --crate-name bar bar/src/bar.rs [..]--crate-type lib \
        --emit=[..]link \
        -C opt-level=3[..]\
        -C metadata=[..] \
        --out-dir [CWD]/target/release/deps \
        -L dependency=[CWD]/target/release/deps`
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name a examples/a.rs [..]--crate-type bin \
        --emit=[..]link \
        -C opt-level=3[..]\
        -C metadata=[..] \
        --out-dir [CWD]/target/release/examples \
        -L dependency=[CWD]/target/release/deps \
         --extern bar=[CWD]/target/release/deps/libbar-[..].rlib`
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `target/release/examples/a[EXE]`
",
        )
        .with_stdout(
            "\
fast1
fast2",
        )
        .run();

    p.payload("run -v --example a")
        .with_stderr(
            "\
[COMPILING] bar v0.5.0 ([CWD]/bar)
[RUNNING] `rustc --crate-name bar bar/src/bar.rs [..]--crate-type lib \
        --emit=[..]link[..]\
        -C debuginfo=2 \
        -C metadata=[..] \
        --out-dir [CWD]/target/debug/deps \
        -L dependency=[CWD]/target/debug/deps`
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name a examples/a.rs [..]--crate-type bin \
        --emit=[..]link[..]\
        -C debuginfo=2 \
        -C metadata=[..] \
        --out-dir [CWD]/target/debug/examples \
        -L dependency=[CWD]/target/debug/deps \
         --extern bar=[CWD]/target/debug/deps/libbar-[..].rlib`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/examples/a[EXE]`
",
        )
        .with_stdout(
            "\
slow1
slow2",
        )
        .run();
}

#[payload_test]
fn run_dylib_dep() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
            "#,
        )
        .file(
            "src/main.rs",
            r#"extern crate bar; fn main() { bar::bar(); }"#,
        )
        .file(
            "bar/Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [lib]
                name = "bar"
                crate-type = ["dylib"]
            "#,
        )
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.payload("run hello world").run();
}

#[payload_test]
fn release_works() {
    let p = project()
        .file(
            "src/main.rs",
            r#"
                fn main() { if cfg!(debug_assertions) { panic!() } }
            "#,
        )
        .build();

    p.payload("run --release")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `target/release/foo[EXE]`
",
        )
        .run();
    assert!(p.release_bin("foo").is_file());
}

#[payload_test]
fn run_bin_different_name() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [[bin]]
                name = "bar"
            "#,
        )
        .file("src/bar.rs", "fn main() {}")
        .build();

    p.payload("run").run();
}

#[payload_test]
fn dashes_are_forwarded() {
    let p = project()
        .file(
            "src/bin/bar.rs",
            r#"
                fn main() {
                    let s: Vec<String> = std::env::args().collect();
                    assert_eq!(s[1], "--");
                    assert_eq!(s[2], "a");
                    assert_eq!(s[3], "--");
                    assert_eq!(s[4], "b");
                }
            "#,
        )
        .build();

    p.payload("run -- -- a -- b").run();
}

#[payload_test]
fn run_from_executable_folder() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    let cwd = p.root().join("target").join("debug");
    p.payload("build").run();

    p.payload("run")
        .cwd(cwd)
        .with_stderr(
            "[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]\n\
             [RUNNING] `./foo[EXE]`",
        )
        .with_stdout("hello")
        .run();
}

#[payload_test]
fn run_with_library_paths() {
    let p = project();

    // Only link search directories within the target output directory are
    // propagated through to dylib_path_envvar() (see #3366).
    let mut dir1 = p.target_debug_dir();
    dir1.push("foo\\backslash");

    let mut dir2 = p.target_debug_dir();
    dir2.push("dir=containing=equal=signs");

    let p = p
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file(
            "build.rs",
            &format!(
                r##"
                    fn main() {{
                        println!(r#"payload:rustc-link-search=native={}"#);
                        println!(r#"payload:rustc-link-search={}"#);
                    }}
                "##,
                dir1.display(),
                dir2.display()
            ),
        )
        .file(
            "src/main.rs",
            &format!(
                r##"
                    fn main() {{
                        let search_path = std::env::var_os("{}").unwrap();
                        let paths = std::env::split_paths(&search_path).collect::<Vec<_>>();
                        println!("{{:#?}}", paths);
                        assert!(paths.contains(&r#"{}"#.into()));
                        assert!(paths.contains(&r#"{}"#.into()));
                    }}
                "##,
                dylib_path_envvar(),
                dir1.display(),
                dir2.display()
            ),
        )
        .build();

    p.payload("run").run();
}

#[payload_test]
fn library_paths_sorted_alphabetically() {
    let p = project();

    let mut dir1 = p.target_debug_dir();
    dir1.push("zzzzzzz");

    let mut dir2 = p.target_debug_dir();
    dir2.push("BBBBBBB");

    let mut dir3 = p.target_debug_dir();
    dir3.push("aaaaaaa");

    let p = p
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file(
            "build.rs",
            &format!(
                r##"
                    fn main() {{
                        println!(r#"payload:rustc-link-search=native={}"#);
                        println!(r#"payload:rustc-link-search=native={}"#);
                        println!(r#"payload:rustc-link-search=native={}"#);
                    }}
                "##,
                dir1.display(),
                dir2.display(),
                dir3.display()
            ),
        )
        .file(
            "src/main.rs",
            &format!(
                r##"
                    fn main() {{
                        let search_path = std::env::var_os("{}").unwrap();
                        let paths = std::env::split_paths(&search_path).collect::<Vec<_>>();
                        // ASCII case-sensitive sort
                        assert_eq!("BBBBBBB", paths[0].file_name().unwrap().to_string_lossy());
                        assert_eq!("aaaaaaa", paths[1].file_name().unwrap().to_string_lossy());
                        assert_eq!("zzzzzzz", paths[2].file_name().unwrap().to_string_lossy());
                    }}
                "##,
                dylib_path_envvar()
            ),
        )
        .build();

    p.payload("run").run();
}

#[payload_test]
fn fail_no_extra_verbose() {
    let p = project()
        .file("src/main.rs", "fn main() { std::process::exit(1); }")
        .build();

    p.payload("run -q")
        .with_status(1)
        .with_stdout("")
        .with_stderr("")
        .run();
}

#[payload_test]
fn run_multiple_packages() {
    let p = project()
        .no_manifest()
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [workspace]

                [dependencies]
                d1 = { path = "d1" }
                d2 = { path = "d2" }
                d3 = { path = "../d3" } # outside of the workspace

                [[bin]]
                name = "foo"
            "#,
        )
        .file("foo/src/foo.rs", "fn main() { println!(\"foo\"); }")
        .file("foo/d1/Payload.toml", &basic_bin_manifest("d1"))
        .file("foo/d1/src/lib.rs", "")
        .file("foo/d1/src/main.rs", "fn main() { println!(\"d1\"); }")
        .file("foo/d2/Payload.toml", &basic_bin_manifest("d2"))
        .file("foo/d2/src/main.rs", "fn main() { println!(\"d2\"); }")
        .file("d3/Payload.toml", &basic_bin_manifest("d3"))
        .file("d3/src/main.rs", "fn main() { println!(\"d2\"); }")
        .build();

    let payload = || {
        let mut process_builder = p.payload("run");
        process_builder.cwd("foo");
        process_builder
    };

    payload().arg("-p").arg("d1").with_stdout("d1").run();

    payload()
        .arg("-p")
        .arg("d2")
        .arg("--bin")
        .arg("d2")
        .with_stdout("d2")
        .run();

    payload().with_stdout("foo").run();

    payload().arg("-p").arg("d1").arg("-p").arg("d2")
                    .with_status(1)
                    .with_stderr_contains("error: The argument '--package <SPEC>' was provided more than once, but cannot be used multiple times").run();

    payload()
        .arg("-p")
        .arg("d3")
        .with_status(101)
        .with_stderr_contains("[ERROR] package(s) `d3` not found in workspace [..]")
        .run();

    payload()
        .arg("-p")
        .arg("d*")
        .with_status(101)
        .with_stderr_contains(
            "[ERROR] `payload run` does not support glob pattern `d*` on package selection",
        )
        .run();
}

#[payload_test]
fn explicit_bin_with_args() {
    let p = project()
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    assert_eq!(std::env::args().nth(1).unwrap(), "hello");
                    assert_eq!(std::env::args().nth(2).unwrap(), "world");
                }
            "#,
        )
        .build();

    p.payload("run --bin foo hello world").run();
}

#[payload_test]
fn run_workspace() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["a", "b"]
            "#,
        )
        .file("a/Payload.toml", &basic_bin_manifest("a"))
        .file("a/src/main.rs", r#"fn main() {println!("run-a");}"#)
        .file("b/Payload.toml", &basic_bin_manifest("b"))
        .file("b/src/main.rs", r#"fn main() {println!("run-b");}"#)
        .build();

    p.payload("run")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] `payload run` could not determine which binary to run[..]
available binaries: a, b",
        )
        .run();
    p.payload("run --bin a").with_stdout("run-a").run();
}

#[payload_test]
fn default_run_workspace() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["a", "b"]
            "#,
        )
        .file(
            "a/Payload.toml",
            r#"
                [project]
                name = "a"
                version = "0.0.1"
                default-run = "a"
            "#,
        )
        .file("a/src/main.rs", r#"fn main() {println!("run-a");}"#)
        .file("b/Payload.toml", &basic_bin_manifest("b"))
        .file("b/src/main.rs", r#"fn main() {println!("run-b");}"#)
        .build();

    p.payload("run").with_stdout("run-a").run();
}

#[payload_test]
#[cfg(target_os = "macos")]
fn run_link_system_path_macos() {
    use payload_test_support::paths::{self, PayloadPathExt};
    use std::fs;
    // Check that the default system library path is honored.
    // First, build a shared library that will be accessed from
    // DYLD_FALLBACK_LIBRARY_PATH.
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [project]
            name = "foo"
            version = "0.0.1"
            [lib]
            crate-type = ["cdylib"]
            "#,
        )
        .file(
            "src/lib.rs",
            "#[no_mangle] pub extern fn something_shared() {}",
        )
        .build();
    p.payload("build").run();

    // This is convoluted. Since this test can't modify things in /usr,
    // this needs to dance around to check that things work.
    //
    // The default DYLD_FALLBACK_LIBRARY_PATH is:
    //      $(HOME)/lib:/usr/local/lib:/lib:/usr/lib
    //
    // This will make use of ~/lib in the path, but the default cc link
    // path is /usr/lib:/usr/local/lib. So first need to build in one
    // location, and then move it to ~/lib.
    //
    // 1. Build with rustc-link-search pointing to libfoo so the initial
    //    binary can be linked.
    // 2. Move the library to ~/lib
    // 3. Run `payload run` to make sure it can still find the library in
    //    ~/lib.
    //
    // This should be equivalent to having the library in /usr/local/lib.
    let p2 = project()
        .at("bar")
        .file("Payload.toml", &basic_bin_manifest("bar"))
        .file(
            "src/main.rs",
            r#"
            extern {
                fn something_shared();
            }
            fn main() {
                unsafe { something_shared(); }
            }
            "#,
        )
        .file(
            "build.rs",
            &format!(
                r#"
                fn main() {{
                    println!("payload:rustc-link-lib=foo");
                    println!("payload:rustc-link-search={}");
                }}
                "#,
                p.target_debug_dir().display()
            ),
        )
        .build();
    p2.payload("build").run();
    p2.payload("test").run();

    let libdir = paths::home().join("lib");
    fs::create_dir(&libdir).unwrap();
    fs::rename(
        p.target_debug_dir().join("libfoo.dylib"),
        libdir.join("libfoo.dylib"),
    )
    .unwrap();
    p.root().rm_rf();
    const VAR: &str = "DYLD_FALLBACK_LIBRARY_PATH";
    // Reset DYLD_FALLBACK_LIBRARY_PATH so that we don't inherit anything that
    // was set by the payload that invoked the test.
    p2.payload("run").env_remove(VAR).run();
    p2.payload("test").env_remove(VAR).run();
    // Ensure this still works when DYLD_FALLBACK_LIBRARY_PATH has
    // a value set.
    p2.payload("run").env(VAR, &libdir).run();
    p2.payload("test").env(VAR, &libdir).run();
}
