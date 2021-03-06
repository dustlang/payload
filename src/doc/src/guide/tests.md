## Tests

Payload can run your tests with the `payload test` command. Payload looks for tests
to run in two places: in each of your `src` files and any tests in `tests/`.
Tests in your `src` files should be unit tests, and tests in `tests/` should be
integration-style tests. As such, you’ll need to import your crates into
the files in `tests`.

Here's an example of running `payload test` in our [package][def-package], which
currently has no tests:

```console
$ payload test
   Compiling rand v0.1.0 (https://github.com/dustlang-nursery/rand.git#9f35b8e)
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running target/test/hello_world-9c2b65bbb79eabce

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

If our package had tests, we would see more output with the correct number of
tests.

You can also run a specific test by passing a filter:

```console
$ payload test foo
```

This will run any test with `foo` in its name.

`payload test` runs additional checks as well.  It will compile any
examples you’ve included and will also test the examples in your
documentation. Please see the [testing guide][testing] in the Rust
documentation for more details.

[def-package]:  ../appendix/glossary.md#package  '"package" (glossary entry)'
[testing]: ../../book/ch11-00-testing.html
