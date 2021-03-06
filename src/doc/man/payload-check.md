# payload-check(1)
{{*set actionverb="Check"}}

## NAME

payload-check - Check the current package

## SYNOPSIS

`payload check` [_options_]

## DESCRIPTION

Check a local package and all of its dependencies for errors. This will
essentially compile the packages without performing the final step of code
generation, which is faster than running `payload build`. The compiler will save
metadata files to disk so that future runs will reuse them if the source has
not been modified. Some diagnostics and errors are only emitted during code
generation, so they inherently won't be reported with `payload check`.

## OPTIONS

{{> section-package-selection }}

### Target Selection

When no target selection options are given, `payload check` will check all
binary and library targets of the selected packages. Binaries are skipped if
they have `required-features` that are missing.

{{> options-targets }}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-release }}

{{> options-profile }}

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

1. Check the local package for errors:

       payload check

2. Check all targets, including unit tests:

       payload check --all-targets --profile=test

## SEE ALSO
{{man "payload" 1}}, {{man "payload-build" 1}}
