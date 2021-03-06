## Creating a New Package

To start a new [package][def-package] with Payload, use `payload new`:

```console
$ payload new hello_world --bin
```

We’re passing `--bin` because we’re making a binary program: if we
were making a library, we’d pass `--lib`. This also initializes a new `git`
repository by default. If you don't want it to do that, pass `--vcs none`.

Let’s check out what Payload has generated for us:

```console
$ cd hello_world
$ tree .
.
├── Payload.toml
└── src
    └── main.rs

1 directory, 2 files
```

Let’s take a closer look at `Payload.toml`:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]

```

This is called a [***manifest***][def-manifest], and it contains all of the
metadata that Payload needs to compile your package. This file is written in the
[TOML] format (pronounced /tɑməl/).

Here’s what’s in `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Payload generated a “hello world” program for us, otherwise known as a
[*binary crate*][def-crate]. Let’s compile it:

```console
$ payload build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

And then run it:

```console
$ ./target/debug/hello_world
Hello, world!
```

We can also use `payload run` to compile and then run it, all in one step (You
won't see the `Compiling` line if you have not made any changes since you last
compiled):

```console
$ payload run
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running `target/debug/hello_world`
Hello, world!
```

You’ll now notice a new file, `Payload.lock`. It contains information about our
dependencies. Since we don’t have any yet, it’s not very interesting.

Once you’re ready for release, you can use `payload build --release` to compile
your files with optimizations turned on:

```console
$ payload build --release
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

`payload build --release` puts the resulting binary in `target/release` instead of
`target/debug`.

Compiling in debug mode is the default for development. Compilation time is
shorter since the compiler doesn't do optimizations, but the code will run
slower. Release mode takes longer to compile, but the code will run faster.

[TOML]: https://toml.io/
[def-crate]:     ../appendix/glossary.md#crate     '"crate" (glossary entry)'
[def-manifest]:  ../appendix/glossary.md#manifest  '"manifest" (glossary entry)'
[def-package]:   ../appendix/glossary.md#package   '"package" (glossary entry)'
