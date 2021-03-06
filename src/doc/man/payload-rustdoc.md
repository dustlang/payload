# payload-rustdoc(1)
{{*set actionverb="Document"}}

## NAME

payload-rustdoc - Build a package's documentation, using specified custom flags

## SYNOPSIS

`payload rustdoc` [_options_] [`--` _args_]

## DESCRIPTION

The specified target for the current package (or package specified by `-p` if
provided) will be documented with the specified _args_ being passed to the
final rustdoc invocation. Dependencies will not be documented as part of this
command. Note that rustdoc will still unconditionally receive arguments such
as `-L`, `--extern`, and `--crate-type`, and the specified _args_ will simply
be added to the rustdoc invocation.

See <https://doc.dustlang.com/rustdoc/index.html> for documentation on rustdoc
flags.

{{> description-one-target }}
To pass flags to all rustdoc processes spawned by Payload, use the
`RUSTDOCFLAGS` [environment variable](../reference/environment-variables.html)
or the `build.rustdocflags` [config value](../reference/config.html).

## OPTIONS

### Documentation Options

{{#options}}

{{#option "`--open`" }}
Open the docs in a browser after building them. This will use your default
browser unless you define another one in the `BROWSER` environment variable.
{{/option}}

{{/options}}

{{> section-options-package }}

### Target Selection

When no target selection options are given, `payload rustdoc` will document all
binary and library targets of the selected package. The binary will be skipped
if its name is the same as the lib target. Binaries are skipped if they have
`required-features` that are missing.

{{> options-targets }}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-release }}

{{/options}}

### Output Options

{{#options}}
{{> options-target-dir }}
{{/options}}

### Display Options

{{#options}}
{{> options-display }}

{{> options-message-format }}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}

{{> options-locked }}
{{/options}}

{{> section-options-common }}

### Miscellaneous Options

{{#options}}
{{> options-jobs }}
{{/options}}

{{> section-profiles }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Build documentation with custom CSS included from a given file:

       payload rustdoc --lib -- --extend-css extra.css

## SEE ALSO
{{man "payload" 1}}, {{man "payload-doc" 1}}, {{man "rustdoc" 1}}
