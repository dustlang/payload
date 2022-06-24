//! Tests for the cache file for the rustc version info.

use payload_test_support::paths::PayloadPathExt;
use payload_test_support::{basic_manifest, project};
use std::env;

#[payload_test]
fn rustc_info_cache() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("hello"); }"#)
        .build();

    let miss = "[..] rustc info cache miss[..]";
    let hit = "[..]rustc info cache hit[..]";
    let update = "[..]updated rustc info cache[..]";

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .with_stderr_contains("[..]failed to read rustc info cache[..]")
        .with_stderr_contains(miss)
        .with_stderr_does_not_contain(hit)
        .with_stderr_contains(update)
        .run();

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .with_stderr_contains("[..]reusing existing rustc info cache[..]")
        .with_stderr_contains(hit)
        .with_stderr_does_not_contain(miss)
        .with_stderr_does_not_contain(update)
        .run();

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .env("PAYLOAD_CACHE_RUSTC_INFO", "0")
        .with_stderr_contains("[..]rustc info cache disabled[..]")
        .with_stderr_does_not_contain(update)
        .run();

    let other_rustc = {
        let p = project()
            .at("compiler")
            .file("Payload.toml", &basic_manifest("compiler", "0.1.0"))
            .file(
                "src/main.rs",
                r#"
                    use std::process::Command;
                    use std::env;

                    fn main() {
                        let mut cmd = Command::new("rustc");
                        for arg in env::args_os().skip(1) {
                            cmd.arg(arg);
                        }
                        std::process::exit(cmd.status().unwrap().code().unwrap());
                    }
                "#,
            )
            .build();
        p.payload("build").run();

        p.root()
            .join("target/debug/compiler")
            .with_extension(env::consts::EXE_EXTENSION)
    };

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .env("RUSTC", other_rustc.display().to_string())
        .with_stderr_contains("[..]different compiler, creating new rustc info cache[..]")
        .with_stderr_contains(miss)
        .with_stderr_does_not_contain(hit)
        .with_stderr_contains(update)
        .run();

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .env("RUSTC", other_rustc.display().to_string())
        .with_stderr_contains("[..]reusing existing rustc info cache[..]")
        .with_stderr_contains(hit)
        .with_stderr_does_not_contain(miss)
        .with_stderr_does_not_contain(update)
        .run();

    other_rustc.move_into_the_future();

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .env("RUSTC", other_rustc.display().to_string())
        .with_stderr_contains("[..]different compiler, creating new rustc info cache[..]")
        .with_stderr_contains(miss)
        .with_stderr_does_not_contain(hit)
        .with_stderr_contains(update)
        .run();

    p.payload("build")
        .env("PAYLOAD_LOG", "payload::util::rustc=debug")
        .env("RUSTC", other_rustc.display().to_string())
        .with_stderr_contains("[..]reusing existing rustc info cache[..]")
        .with_stderr_contains(hit)
        .with_stderr_does_not_contain(miss)
        .with_stderr_does_not_contain(update)
        .run();
}
