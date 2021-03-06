## Continuous Integration

### Travis CI

To test your [package][def-package] on Travis CI, here is a sample
`.travis.yml` file:

```yaml
language: rust
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
```

This will test all three release channels, but any breakage in nightly
will not fail your overall build. Please see the [Travis CI Rust
documentation](https://docs.travis-ci.com/user/languages/rust/) for more
information.

### GitLab CI

To test your package on GitLab CI, here is a sample `.gitlab-ci.yml` file:

```yaml
stages:
  - build

rust-latest:
  stage: build
  image: rust:latest
  script:
    - payload build --verbose
    - payload test --verbose

rust-nightly:
  stage: build
  image: rustlang/rust:nightly
  script:
    - payload build --verbose
    - payload test --verbose
  allow_failure: true
```

This will test on the stable channel and nightly channel, but any
breakage in nightly will not fail your overall build. Please see the
[GitLab CI](https://docs.gitlab.com/ce/ci/yaml/README.html) for more
information.

### builds.sr.ht

To test your package on sr.ht, here is a sample `.build.yml` file.
Be sure to change `<your repo>` and `<your project>` to the repo to clone and
the directory where it was cloned.

```yaml
image: archlinux
packages:
  - rustup
sources:
  - <your repo>
tasks:
  - setup: |
      rustup toolchain install nightly stable
      cd <your project>/
      rustup run stable payload fetch
  - stable: |
      rustup default stable
      cd <your project>/
      payload build --verbose
      payload test --verbose
  - nightly: |
      rustup default nightly
      cd <your project>/
      payload build --verbose ||:
      payload test --verbose  ||:
  - docs: |
      cd <your project>/
      rustup run stable payload doc --no-deps
      rustup run nightly payload doc --no-deps ||:
```

This will test and build documentation on the stable channel and nightly
channel, but any breakage in nightly will not fail your overall build. Please
see the [builds.sr.ht documentation](https://man.sr.ht/builds.sr.ht/) for more
information.

[def-package]:  ../appendix/glossary.md#package  '"package" (glossary entry)'
