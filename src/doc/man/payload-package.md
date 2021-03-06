# payload-package(1)
{{*set actionverb="Package"}}

## NAME

payload-package - Assemble the local package into a distributable tarball

## SYNOPSIS

`payload package` [_options_]

## DESCRIPTION

This command will create a distributable, compressed `.crate` file with the
source code of the package in the current directory. The resulting file will
be stored in the `target/package` directory. This performs the following
steps:

1. Load and check the current workspace, performing some basic checks.
    - Path dependencies are not allowed unless they have a version key. Payload
      will ignore the path key for dependencies in published packages.
      `dev-dependencies` do not have this restriction.
2. Create the compressed `.crate` file.
    - The original `Payload.toml` file is rewritten and normalized.
    - `[patch]`, `[replace]`, and `[workspace]` sections are removed from the
      manifest.
    - `Payload.lock` is automatically included if the package contains an
      executable binary or example target. {{man "payload-install" 1}} will use the
      packaged lock file if the `--locked` flag is used.
    - A `.payload_vcs_info.json` file is included that contains information
      about the current VCS checkout hash if available (not included with
      `--allow-dirty`).
3. Extract the `.crate` file and build it to verify it can build.
    - This will rebuild your package from scratch to ensure that it can be
      built from a pristine state. The `--no-verify` flag can be used to skip
      this step.
4. Check that build scripts did not modify any source files.

The list of files included can be controlled with the `include` and `exclude`
fields in the manifest.

See [the reference](../reference/publishing.html) for more details about
packaging and publishing.

## OPTIONS

### Package Options

{{#options}}

{{#option "`-l`" "`--list`" }}
Print files included in a package without making one.
{{/option}}

{{#option "`--no-verify`" }}
Don't verify the contents by building them.
{{/option}}

{{#option "`--no-metadata`" }}
Ignore warnings about a lack of human-usable metadata (such as the description
or the license).
{{/option}}

{{#option "`--allow-dirty`" }}
Allow working directories with uncommitted VCS changes to be packaged.
{{/option}}

{{/options}}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-target-dir }}

{{/options}}

{{> section-features }}

### Manifest Options

{{#options}}

{{> options-manifest-path }}

{{> options-locked }}

{{/options}}

### Miscellaneous Options

{{#options}}
{{> options-jobs }}
{{/options}}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create a compressed `.crate` file of the current package:

       payload package

## SEE ALSO
{{man "payload" 1}}, {{man "payload-publish" 1}}
