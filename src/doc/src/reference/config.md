## Configuration

This document explains how Payload’s configuration system works, as well as
available keys or configuration. For configuration of a package through its
manifest, see the [manifest format](manifest.md).

### Hierarchical structure

Payload allows local configuration for a particular package as well as global
configuration. It looks for configuration files in the current directory and
all parent directories. If, for example, Payload were invoked in
`/projects/foo/bar/baz`, then the following configuration files would be
probed for and unified in this order:

* `/projects/foo/bar/baz/.payload/config.toml`
* `/projects/foo/bar/.payload/config.toml`
* `/projects/foo/.payload/config.toml`
* `/projects/.payload/config.toml`
* `/.payload/config.toml`
* `$PAYLOAD_HOME/config.toml` which defaults to:
    * Windows: `%USERPROFILE%\.payload\config.toml`
    * Unix: `$HOME/.payload/config.toml`

With this structure, you can specify configuration per-package, and even
possibly check it into version control. You can also specify personal defaults
with a configuration file in your home directory.

If a key is specified in multiple config files, the values will get merged
together. Numbers, strings, and booleans will use the value in the deeper
config directory taking precedence over ancestor directories, where the
home directory is the lowest priority. Arrays will be joined together.

> **Note:** Payload also reads config files without the `.toml` extension, such as
> `.payload/config`. Support for the `.toml` extension was added in version 1.39
> and is the preferred form. If both files exist, Payload will use the file
> without the extension.

### Configuration format

Configuration files are written in the [TOML format][toml] (like the
manifest), with simple key-value pairs inside of sections (tables). The
following is a quick overview of all settings, with detailed descriptions
found below.

```toml
paths = ["/path/to/override"] # path dependency overrides

[alias]     # command aliases
b = "build"
c = "check"
t = "test"
r = "run"
rr = "run --release"
space_example = ["run", "--release", "--", "\"command list\""]

[build]
jobs = 1                      # number of parallel jobs, defaults to # of CPUs
rustc = "rustc"               # the rust compiler tool
rustc-wrapper = "…"           # run this wrapper instead of `rustc`
rustc-workspace-wrapper = "…" # run this wrapper instead of `rustc` for workspace members
rustdoc = "rustdoc"           # the doc generator tool
target = "triple"             # build for the target triple (ignored by `payload install`)
target-dir = "target"         # path of where to place all generated artifacts
rustflags = ["…", "…"]        # custom flags to pass to all compiler invocations
rustdocflags = ["…", "…"]     # custom flags to pass to rustdoc
incremental = true            # whether or not to enable incremental compilation
dep-info-basedir = "…"        # path for the base directory for targets in depfiles
pipelining = true             # rustc pipelining

[payload-new]
name = "Your Name"        # name to use in `authors` field
email = "you@example.com" # email address to use in `authors` field
vcs = "none"              # VCS to use ('git', 'hg', 'pijul', 'fossil', 'none')

[http]
debug = false               # HTTP debugging
proxy = "host:port"         # HTTP proxy in libcurl format
ssl-version = "tlsv1.3"     # TLS version to use
ssl-version.max = "tlsv1.3" # maximum TLS version
ssl-version.min = "tlsv1.1" # minimum TLS version
timeout = 30                # timeout for each HTTP request, in seconds
low-speed-limit = 10        # network timeout threshold (bytes/sec)
cainfo = "cert.pem"         # path to Certificate Authority (CA) bundle
check-revoke = true         # check for SSL certificate revocation
multiplexing = true         # HTTP/2 multiplexing
user-agent = "…"            # the user-agent header

[install]
root = "/some/path"         # `payload install` destination directory

[net]
retry = 2                   # network retries
git-fetch-with-cli = true   # use the `git` executable for git operations
offline = false             # do not access the network

[profile.<name>]         # Modify profile settings via config.
opt-level = 0            # Optimization level.
debug = true             # Include debug info.
debug-assertions = true  # Enables debug assertions.
overflow-checks = true   # Enables runtime integer overflow checks.
lto = false              # Sets link-time optimization.
panic = 'unwind'         # The panic strategy.
incremental = true       # Incremental compilation.
codegen-units = 16       # Number of code generation units.
rpath = false            # Sets the rpath linking option.
[profile.<name>.build-override]  # Overrides build-script settings.
# Same keys for a normal profile.
[profile.<name>.package.<name>]  # Override profile for a package.
# Same keys for a normal profile (minus `panic`, `lto`, and `rpath`).

[registries.<name>]  # registries other than crates.io
index = "…"          # URL of the registry index
token = "…"          # authentication token for the registry

[registry]
default = "…"        # name of the default registry
token = "…"          # authentication token for crates.io

[source.<name>]      # source definition and replacement
replace-with = "…"   # replace this source with the given named source
directory = "…"      # path to a directory source
registry = "…"       # URL to a registry source
local-registry = "…" # path to a local registry source
git = "…"            # URL of a git repository source
branch = "…"         # branch name for the git repository
tag = "…"            # tag name for the git repository
rev = "…"            # revision for the git repository

[target.<triple>]
linker = "…"            # linker to use
runner = "…"            # wrapper to run executables
rustflags = ["…", "…"]  # custom flags for `rustc`

[target.<cfg>]
runner = "…"            # wrapper to run executables
rustflags = ["…", "…"]  # custom flags for `rustc`

[target.<triple>.<links>] # `links` build script override
rustc-link-lib = ["foo"]
rustc-link-search = ["/path/to/foo"]
rustc-flags = ["-L", "/some/path"]
rustc-cfg = ['key="value"']
rustc-env = {key = "value"}
rustc-cdylib-link-arg = ["…"]
metadata_key1 = "value"
metadata_key2 = "value"

[term]
verbose = false        # whether payload provides verbose output
color = 'auto'         # whether payload colorizes output
progress.when = 'auto' # whether payload shows progress bar
progress.width = 80    # width of progress bar
```

### Environment variables

Payload can also be configured through environment variables in addition to the
TOML configuration files. For each configuration key of the form `foo.bar` the
environment variable `PAYLOAD_FOO_BAR` can also be used to define the value.
Keys are converted to uppercase, dots and dashes are converted to underscores.
For example the `target.x86_64-unknown-linux-gnu.runner` key can also be
defined by the `PAYLOAD_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER` environment
variable.

Environment variables will take precedence over TOML configuration files.
Currently only integer, boolean, string and some array values are supported to
be defined by environment variables. Descriptions below indicate which keys
support environment variables.

In addition to the system above, Payload recognizes a few other specific
[environment variables][env].

### Config-relative paths

Paths in config files may be absolute, relative, or a bare name without any
path separators. Paths for executables without a path separator will use the
`PATH` environment variable to search for the executable. Paths for
non-executables will be relative to where the config value is defined. For
config files, that is relative to the parent directory of the `.payload`
directory where the value was defined. For environment variables it is
relative to the current working directory.

```toml
# Relative path examples.

[target.x86_64-unknown-linux-gnu]
runner = "foo"  # Searches `PATH` for `foo`.

[source.vendored-sources]
# Directory is relative to the parent where `.payload/config.toml` is located.
# For example, `/my/project/.payload/config.toml` would result in `/my/project/vendor`.
directory = "vendor"
```

### Credentials

Configuration values with sensitive information are stored in the
`$PAYLOAD_HOME/credentials.toml` file. This file is automatically created and updated
by [`payload login`]. It follows the same format as Payload config files.

```toml
[registry]
token = "…"   # Access token for crates.io

[registries.<name>]
token = "…"   # Access token for the named registry
```

Tokens are used by some Payload commands such as [`payload publish`] for
authenticating with remote registries. Care should be taken to protect the
tokens and to keep them secret.

As with most other config values, tokens may be specified with environment
variables. The token for [crates.io] may be specified with the
`PAYLOAD_REGISTRY_TOKEN` environment variable. Tokens for other registries may
be specified with environment variables of the form
`PAYLOAD_REGISTRIES_<name>_TOKEN` where `<name>` is the name of the registry in
all capital letters.

### Configuration keys

This section documents all configuration keys. The description for keys with
variable parts are annotated with angled brackets like `target.<triple>` where
the `<triple>` part can be any target triple like
`target.x86_64-pc-windows-msvc`.

#### `paths`
* Type: array of strings (paths)
* Default: none
* Environment: not supported

An array of paths to local packages which are to be used as overrides for
dependencies. For more information see the [Overriding Dependencies
guide](overriding-dependencies.md#paths-overrides).

#### `[alias]`
* Type: string or array of strings
* Default: see below
* Environment: `PAYLOAD_ALIAS_<name>`

The `[alias]` table defines CLI command aliases. For example, running `payload
b` is an alias for running `payload build`. Each key in the table is the
subcommand, and the value is the actual command to run. The value may be an
array of strings, where the first element is the command and the following are
arguments. It may also be a string, which will be split on spaces into
subcommand and arguments. The following aliases are built-in to Payload:

```toml
[alias]
b = "build"
c = "check"
t = "test"
r = "run"
```

Aliases are not allowed to redefine existing built-in commands.

#### `[build]`

The `[build]` table controls build-time operations and compiler settings.

##### `build.jobs`
* Type: integer
* Default: number of logical CPUs
* Environment: `PAYLOAD_BUILD_JOBS`

Sets the maximum number of compiler processes to run in parallel.

Can be overridden with the `--jobs` CLI option.

##### `build.rustc`
* Type: string (program path)
* Default: "rustc"
* Environment: `PAYLOAD_BUILD_RUSTC` or `RUSTC`

Sets the executable to use for `rustc`.

##### `build.rustc-wrapper`
* Type: string (program path)
* Default: none
* Environment: `PAYLOAD_BUILD_RUSTC_WRAPPER` or `RUSTC_WRAPPER`

Sets a wrapper to execute instead of `rustc`. The first argument passed to the
wrapper is the path to the actual `rustc`.

##### `build.rustc-workspace-wrapper`
* Type: string (program path)
* Default: none
* Environment: `PAYLOAD_BUILD_RUSTC_WORKSPACE_WRAPPER` or `RUSTC_WORKSPACE_WRAPPER`

Sets a wrapper to execute instead of `rustc`, for workspace members only.
The first argument passed to the wrapper is the path to the actual `rustc`.
It affects the filename hash so that artifacts produced by the wrapper are cached separately.

##### `build.rustdoc`
* Type: string (program path)
* Default: "rustdoc"
* Environment: `PAYLOAD_BUILD_RUSTDOC` or `RUSTDOC`

Sets the executable to use for `rustdoc`.

##### `build.target`
* Type: string
* Default: host platform
* Environment: `PAYLOAD_BUILD_TARGET`

The default target platform triple to compile to.

This may also be a relative path to a `.json` target spec file.

Can be overridden with the `--target` CLI option.

##### `build.target-dir`
* Type: string (path)
* Default: "target"
* Environment: `PAYLOAD_BUILD_TARGET_DIR` or `PAYLOAD_TARGET_DIR`

The path to where all compiler output is placed. The default if not specified
is a directory named `target` located at the root of the workspace.

Can be overridden with the `--target-dir` CLI option.

##### `build.rustflags`
* Type: string or array of strings
* Default: none
* Environment: `PAYLOAD_BUILD_RUSTFLAGS` or `RUSTFLAGS`

Extra command-line flags to pass to `rustc`. The value may be a array of
strings or a space-separated string.

There are three mutually exclusive sources of extra flags. They are checked in
order, with the first one being used:

1. `RUSTFLAGS` environment variable.
2. All matching `target.<triple>.rustflags` and `target.<cfg>.rustflags`
   config entries joined together.
3. `build.rustflags` config value.

Additional flags may also be passed with the [`payload rustc`] command.

If the `--target` flag (or [`build.target`](#buildtarget)) is used, then the
flags will only be passed to the compiler for the target. Things being built
for the host, such as build scripts or proc macros, will not receive the args.
Without `--target`, the flags will be passed to all compiler invocations
(including build scripts and proc macros) because dependencies are shared. If
you have args that you do not want to pass to build scripts or proc macros and
are building for the host, pass `--target` with the host triple.

##### `build.rustdocflags`
* Type: string or array of strings
* Default: none
* Environment: `PAYLOAD_BUILD_RUSTDOCFLAGS` or `RUSTDOCFLAGS`

Extra command-line flags to pass to `rustdoc`. The value may be a array of
strings or a space-separated string.

There are two mutually exclusive sources of extra flags. They are checked in
order, with the first one being used:

1. `RUSTDOCFLAGS` environment variable.
2. `build.rustdocflags` config value.

Additional flags may also be passed with the [`payload rustdoc`] command.

##### `build.incremental`
* Type: bool
* Default: from profile
* Environment: `PAYLOAD_BUILD_INCREMENTAL` or `PAYLOAD_INCREMENTAL`

Whether or not to perform [incremental compilation]. The default if not set is
to use the value from the [profile]. Otherwise this overrides the setting of
all profiles.

The `PAYLOAD_INCREMENTAL` environment variable can be set to `1` to force enable
incremental compilation for all profiles, or `0` to disable it. This env var
overrides the config setting.

##### `build.dep-info-basedir`
* Type: string (path)
* Default: none
* Environment: `PAYLOAD_BUILD_DEP_INFO_BASEDIR`

Strips the given path prefix from [dep
info](../guide/build-cache.md#dep-info-files) file paths. This config setting
is intended to convert absolute paths to relative paths for tools that require
relative paths.

The setting itself is a config-relative path. So, for example, a value of
`"."` would strip all paths starting with the parent directory of the `.payload`
directory.

##### `build.pipelining`
* Type: boolean
* Default: true
* Environment: `PAYLOAD_BUILD_PIPELINING`

Controls whether or not build pipelining is used. This allows Payload to
schedule overlapping invocations of `rustc` in parallel when possible.

#### `[payload-new]`

The `[payload-new]` table defines defaults for the [`payload new`] command.

##### `payload-new.name`
* Type: string
* Default: from environment
* Environment: `PAYLOAD_NAME` or `PAYLOAD_PAYLOAD_NEW_NAME`

Defines the name to use in the `authors` field when creating a new
`Payload.toml` file. If not specified in the config, Payload searches the
environment or your `git` configuration as described in the [`payload new`]
documentation.

##### `payload-new.email`
* Type: string
* Default: from environment
* Environment: `PAYLOAD_EMAIL` or `PAYLOAD_PAYLOAD_NEW_EMAIL`

Defines the email address used in the `authors` field when creating a new
`Payload.toml` file. If not specified in the config, Payload searches the
environment or your `git` configuration as described in the [`payload new`]
documentation. The `email` value may be set to an empty string to prevent
Payload from placing an address in the authors field.

##### `payload-new.vcs`
* Type: string
* Default: "git" or "none"
* Environment: `PAYLOAD_PAYLOAD_NEW_VCS`

Specifies the source control system to use for initializing a new repository.
Valid values are `git`, `hg` (for Mercurial), `pijul`, `fossil` or `none` to
disable this behavior. Defaults to `git`, or `none` if already inside a VCS
repository. Can be overridden with the `--vcs` CLI option.

#### `[http]`

The `[http]` table defines settings for HTTP behavior. This includes fetching
crate dependencies and accessing remote git repositories.

##### `http.debug`
* Type: boolean
* Default: false
* Environment: `PAYLOAD_HTTP_DEBUG`

If `true`, enables debugging of HTTP requests. The debug information can be
seen by setting the `PAYLOAD_LOG=payload::ops::registry=debug` environment
variable (or use `trace` for even more information).

Be wary when posting logs from this output in a public location. The output
may include headers with authentication tokens which you don't want to leak!
Be sure to review logs before posting them.

##### `http.proxy`
* Type: string
* Default: none
* Environment: `PAYLOAD_HTTP_PROXY` or `HTTPS_PROXY` or `https_proxy` or `http_proxy`

Sets an HTTP and HTTPS proxy to use. The format is in [libcurl format] as in
`[protocol://]host[:port]`. If not set, Payload will also check the `http.proxy`
setting in your global git configuration. If none of those are set, the
`HTTPS_PROXY` or `https_proxy` environment variables set the proxy for HTTPS
requests, and `http_proxy` sets it for HTTP requests.

##### `http.timeout`
* Type: integer
* Default: 30
* Environment: `PAYLOAD_HTTP_TIMEOUT` or `HTTP_TIMEOUT`

Sets the timeout for each HTTP request, in seconds.

##### `http.cainfo`
* Type: string (path)
* Default: none
* Environment: `PAYLOAD_HTTP_CAINFO`

Path to a Certificate Authority (CA) bundle file, used to verify TLS
certificates. If not specified, Payload attempts to use the system certificates.

##### `http.check-revoke`
* Type: boolean
* Default: true (Windows) false (all others)
* Environment: `PAYLOAD_HTTP_CHECK_REVOKE`

This determines whether or not TLS certificate revocation checks should be
performed. This only works on Windows.

##### `http.ssl-version`
* Type: string or min/max table
* Default: none
* Environment: `PAYLOAD_HTTP_SSL_VERSION`

This sets the minimum TLS version to use. It takes a string, with one of the
possible values of "default", "tlsv1", "tlsv1.0", "tlsv1.1", "tlsv1.2", or
"tlsv1.3".

This may alternatively take a table with two keys, `min` and `max`, which each
take a string value of the same kind that specifies the minimum and maximum
range of TLS versions to use.

The default is a minimum version of "tlsv1.0" and a max of the newest version
supported on your platform, typically "tlsv1.3".

##### `http.low-speed-limit`
* Type: integer
* Default: 10
* Environment: `PAYLOAD_HTTP_LOW_SPEED_LIMIT`

This setting controls timeout behavior for slow connections. If the average
transfer speed in bytes per second is below the given value for
[`http.timeout`](#httptimeout) seconds (default 30 seconds), then the
connection is considered too slow and Payload will abort and retry.

##### `http.multiplexing`
* Type: boolean
* Default: true
* Environment: `PAYLOAD_HTTP_MULTIPLEXING`

When `true`, Payload will attempt to use the HTTP2 protocol with multiplexing.
This allows multiple requests to use the same connection, usually improving
performance when fetching multiple files. If `false`, Payload will use HTTP 1.1
without pipelining.

##### `http.user-agent`
* Type: string
* Default: Payload's version
* Environment: `PAYLOAD_HTTP_USER_AGENT`

Specifies a custom user-agent header to use. The default if not specified is a
string that includes Payload's version.

#### `[install]`

The `[install]` table defines defaults for the [`payload install`] command.

##### `install.root`
* Type: string (path)
* Default: Payload's home directory
* Environment: `PAYLOAD_INSTALL_ROOT`

Sets the path to the root directory for installing executables for [`payload
install`]. Executables go into a `bin` directory underneath the root.

The default if not specified is Payload's home directory (default `.payload` in
your home directory).

Can be overridden with the `--root` command-line option.

#### `[net]`

The `[net]` table controls networking configuration.

##### `net.retry`
* Type: integer
* Default: 2
* Environment: `PAYLOAD_NET_RETRY`

Number of times to retry possibly spurious network errors.

##### `net.git-fetch-with-cli`
* Type: boolean
* Default: false
* Environment: `PAYLOAD_NET_GIT_FETCH_WITH_CLI`

If this is `true`, then Payload will use the `git` executable to fetch registry
indexes and git dependencies. If `false`, then it uses a built-in `git`
library.

Setting this to `true` can be helpful if you have special authentication
requirements that Payload does not support. See [Git
Authentication](../appendix/git-authentication.md) for more information about
setting up git authentication.

##### `net.offline`
* Type: boolean
* Default: false
* Environment: `PAYLOAD_NET_OFFLINE`

If this is `true`, then Payload will avoid accessing the network, and attempt to
proceed with locally cached data. If `false`, Payload will access the network as
needed, and generate an error if it encounters a network error.

Can be overridden with the `--offline` command-line option.

#### `[profile]`

The `[profile]` table can be used to globally change profile settings, and
override settings specified in `Payload.toml`. It has the same syntax and
options as profiles specified in `Payload.toml`. See the [Profiles chapter] for
details about the options.

[Profiles chapter]: profiles.md

##### `[profile.<name>.build-override]`
* Environment: `PAYLOAD_PROFILE_<name>_BUILD_OVERRIDE_<key>`

The build-override table overrides settings for build scripts, proc macros,
and their dependencies. It has the same keys as a normal profile. See the
[overrides section](profiles.md#overrides) for more details.

##### `[profile.<name>.package.<name>]`
* Environment: not supported

The package table overrides settings for specific packages. It has the same
keys as a normal profile, minus the `panic`, `lto`, and `rpath` settings. See
the [overrides section](profiles.md#overrides) for more details.

##### `profile.<name>.codegen-units`
* Type: integer
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_CODEGEN_UNITS`

See [codegen-units](profiles.md#codegen-units).

##### `profile.<name>.debug`
* Type: integer or boolean
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_DEBUG`

See [debug](profiles.md#debug).

##### `profile.<name>.debug-assertions`
* Type: boolean
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_DEBUG_ASSERTIONS`

See [debug-assertions](profiles.md#debug-assertions).

##### `profile.<name>.incremental`
* Type: boolean
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_INCREMENTAL`

See [incremental](profiles.md#incremental).

##### `profile.<name>.lto`
* Type: string or boolean
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_LTO`

See [lto](profiles.md#lto).

##### `profile.<name>.overflow-checks`
* Type: boolean
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_OVERFLOW_CHECKS`

See [overflow-checks](profiles.md#overflow-checks).

##### `profile.<name>.opt-level`
* Type: integer or string
* Default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_OPT_LEVEL`

See [opt-level](profiles.md#opt-level).

##### `profile.<name>.panic`
* Type: string
* default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_PANIC`

See [panic](profiles.md#panic).

##### `profile.<name>.rpath`
* Type: boolean
* default: See profile docs.
* Environment: `PAYLOAD_PROFILE_<name>_RPATH`

See [rpath](profiles.md#rpath).


#### `[registries]`

The `[registries]` table is used for specifying additional [registries]. It
consists of a sub-table for each named registry.

##### `registries.<name>.index`
* Type: string (url)
* Default: none
* Environment: `PAYLOAD_REGISTRIES_<name>_INDEX`

Specifies the URL of the git index for the registry.

##### `registries.<name>.token`
* Type: string
* Default: none
* Environment: `PAYLOAD_REGISTRIES_<name>_TOKEN`

Specifies the authentication token for the given registry. This value should
only appear in the [credentials](#credentials) file. This is used for registry
commands like [`payload publish`] that require authentication.

Can be overridden with the `--token` command-line option.

#### `[registry]`

The `[registry]` table controls the default registry used when one is not
specified.

##### `registry.index`

This value is no longer accepted and should not be used.

##### `registry.default`
* Type: string
* Default: `"crates-io"`
* Environment: `PAYLOAD_REGISTRY_DEFAULT`

The name of the registry (from the [`registries` table](#registries)) to use
by default for registry commands like [`payload publish`].

Can be overridden with the `--registry` command-line option.

##### `registry.token`
* Type: string
* Default: none
* Environment: `PAYLOAD_REGISTRY_TOKEN`

Specifies the authentication token for [crates.io]. This value should only
appear in the [credentials](#credentials) file. This is used for registry
commands like [`payload publish`] that require authentication.

Can be overridden with the `--token` command-line option.

#### `[source]`

The `[source]` table defines the registry sources available. See [Source
Replacement] for more information. It consists of a sub-table for each named
source. A source should only define one kind (directory, registry,
local-registry, or git).

##### `source.<name>.replace-with`
* Type: string
* Default: none
* Environment: not supported

If set, replace this source with the given named source.

##### `source.<name>.directory`
* Type: string (path)
* Default: none
* Environment: not supported

Sets the path to a directory to use as a directory source.

##### `source.<name>.registry`
* Type: string (url)
* Default: none
* Environment: not supported

Sets the URL to use for a registry source.

##### `source.<name>.local-registry`
* Type: string (path)
* Default: none
* Environment: not supported

Sets the path to a directory to use as a local registry source.

##### `source.<name>.git`
* Type: string (url)
* Default: none
* Environment: not supported

Sets the URL to use for a git repository source.

##### `source.<name>.branch`
* Type: string
* Default: none
* Environment: not supported

Sets the branch name to use for a git repository.

If none of `branch`, `tag`, or `rev` is set, defaults to the `master` branch.

##### `source.<name>.tag`
* Type: string
* Default: none
* Environment: not supported

Sets the tag name to use for a git repository.

If none of `branch`, `tag`, or `rev` is set, defaults to the `master` branch.

##### `source.<name>.rev`
* Type: string
* Default: none
* Environment: not supported

Sets the [revision] to use for a git repository.

If none of `branch`, `tag`, or `rev` is set, defaults to the `master` branch.


#### `[target]`

The `[target]` table is used for specifying settings for specific platform
targets. It consists of a sub-table which is either a platform triple or a
[`cfg()` expression]. The given values will be used if the target platform
matches either the `<triple>` value or the `<cfg>` expression.

```toml
[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-gcc"
runner = "my-emulator"
rustflags = ["…", "…"]

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "my-arm-wrapper"
rustflags = ["…", "…"]
```

`cfg` values come from those built-in to the compiler (run `rustc --print=cfg`
to view), values set by [build scripts], and extra `--cfg` flags passed to
`rustc` (such as those defined in `RUSTFLAGS`). Do not try to match on
`debug_assertions` or Payload features like `feature="foo"`.

If using a target spec JSON file, the `<triple>` value is the filename stem.
For example `--target foo/bar.json` would match `[target.bar]`.

##### `target.<triple>.ar`

This option is deprecated and unused.

##### `target.<triple>.linker`
* Type: string (program path)
* Default: none
* Environment: `PAYLOAD_TARGET_<triple>_LINKER`

Specifies the linker which is passed to `rustc` (via [`-C linker`]) when the
`<triple>` is being compiled for. By default, the linker is not overridden.

##### `target.<triple>.runner`
* Type: string or array of strings (program path and args)
* Default: none
* Environment: `PAYLOAD_TARGET_<triple>_RUNNER`

If a runner is provided, executables for the target `<triple>` will be
executed by invoking the specified runner with the actual executable passed as
an argument. This applies to [`payload run`], [`payload test`] and [`payload bench`]
commands. By default, compiled executables are executed directly.

The value may be an array of strings like `['/path/to/program', 'somearg']` or
a space-separated string like `'/path/to/program somearg'`. The arguments will
be passed to the runner with the executable to run as the last argument. If
the runner program does not have path separators, it will search `PATH` for
the runner executable.

##### `target.<cfg>.runner`

This is similar to the [target runner](#targettriplerunner), but using
a [`cfg()` expression]. If both a `<triple>` and `<cfg>` runner match,
the `<triple>` will take precedence. It is an error if more than one
`<cfg>` runner matches the current target.

##### `target.<triple>.rustflags`
* Type: string or array of strings
* Default: none
* Environment: `PAYLOAD_TARGET_<triple>_RUSTFLAGS`

Passes a set of custom flags to the compiler for this `<triple>`. The value
may be a array of strings or a space-separated string.

See [`build.rustflags`](#buildrustflags) for more details on the different
ways to specific extra flags.

##### `target.<cfg>.rustflags`

This is similar to the [target rustflags](#targettriplerustflags), but
using a [`cfg()` expression]. If several `<cfg>` and `<triple>` entries
match the current target, the flags are joined together.

##### `target.<triple>.<links>`

The links sub-table provides a way to [override a build script]. When
specified, the build script for the given `links` library will not be
run, and the given values will be used instead.

```toml
[target.x86_64-unknown-linux-gnu.foo]
rustc-link-lib = ["foo"]
rustc-link-search = ["/path/to/foo"]
rustc-flags = "-L /some/path"
rustc-cfg = ['key="value"']
rustc-env = {key = "value"}
rustc-cdylib-link-arg = ["…"]
metadata_key1 = "value"
metadata_key2 = "value"
```

#### `[term]`

The `[term]` table controls terminal output and interaction.

##### `term.verbose`
* Type: boolean
* Default: false
* Environment: `PAYLOAD_TERM_VERBOSE`

Controls whether or not extra detailed messages are displayed by Payload.

Specifying the `--quiet` flag will override and disable verbose output.
Specifying the `--verbose` flag will override and force verbose output.

##### `term.color`
* Type: string
* Default: "auto"
* Environment: `PAYLOAD_TERM_COLOR`

Controls whether or not colored output is used in the terminal. Possible values:

* `auto` (default): Automatically detect if color support is available on the
  terminal.
* `always`: Always display colors.
* `never`: Never display colors.

Can be overridden with the `--color` command-line option.

##### `term.progress.when`
* Type: string
* Default: "auto"
* Environment: `PAYLOAD_TERM_PROGRESS_WHEN`

Controls whether or not progress bar is shown in the terminal. Possible values:

* `auto` (default): Intelligently guess whether to show progress bar.
* `always`: Always show progress bar.
* `never`: Never show progress bar.

##### `term.progress.width`
* Type: integer
* Default: none
* Environment: `PAYLOAD_TERM_PROGRESS_WIDTH`

Sets the width for progress bar.

[`payload bench`]: ../commands/payload-bench.md
[`payload login`]: ../commands/payload-login.md
[`payload new`]: ../commands/payload-new.md
[`payload publish`]: ../commands/payload-publish.md
[`payload run`]: ../commands/payload-run.md
[`payload rustc`]: ../commands/payload-rustc.md
[`payload test`]: ../commands/payload-test.md
[`payload rustdoc`]: ../commands/payload-rustdoc.md
[`payload install`]: ../commands/payload-install.md
[env]: environment-variables.md
[`cfg()` expression]: ../../reference/conditional-compilation.html
[build scripts]: build-scripts.md
[`-C linker`]: ../../rustc/codegen-options/index.md#linker
[override a build script]: build-scripts.md#overriding-build-scripts
[toml]: https://toml.io/
[incremental compilation]: profiles.md#incremental
[profile]: profiles.md
[libcurl format]: https://ec.haxx.se/usingcurl-proxies.html
[source replacement]: source-replacement.md
[revision]: https://git-scm.com/docs/gitrevisions
[registries]: registries.md
[crates.io]: https://crates.io/
