# payload-run(1)


## NAME

payload-run - Run the current package

## SYNOPSIS

`payload run` [_options_] [`--` _args_]

## DESCRIPTION

Run a binary or example of the local package.

All the arguments following the two dashes (`--`) are passed to the binary to
run. If you're passing arguments to both Payload and the binary, the ones after
`--` go to the binary, the ones before go to Payload.

## OPTIONS

### Package Selection

By default, the package in the current working directory is selected. The `-p`
flag can be used to choose a different package in a workspace.

<dl>

<dt class="option-term" id="option-payload-run--p"><a class="option-anchor" href="#option-payload-run--p"></a><code>-p</code> <em>spec</em></dt>
<dt class="option-term" id="option-payload-run---package"><a class="option-anchor" href="#option-payload-run---package"></a><code>--package</code> <em>spec</em></dt>
<dd class="option-desc">The package to run. See <a href="payload-pkgid.html">payload-pkgid(1)</a> for the SPEC
format.</dd>


</dl>


### Target Selection

When no target selection options are given, `payload run` will run the binary
target. If there are multiple binary targets, you must pass a target flag to
choose one. Or, the `default-run` field may be specified in the `[package]`
section of `Payload.toml` to choose the name of the binary to run by default.

<dl>

<dt class="option-term" id="option-payload-run---bin"><a class="option-anchor" href="#option-payload-run---bin"></a><code>--bin</code> <em>name</em></dt>
<dd class="option-desc">Run the specified binary.</dd>


<dt class="option-term" id="option-payload-run---example"><a class="option-anchor" href="#option-payload-run---example"></a><code>--example</code> <em>name</em></dt>
<dd class="option-desc">Run the specified example.</dd>


</dl>

### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-payload-run---features"><a class="option-anchor" href="#option-payload-run---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-run---all-features"><a class="option-anchor" href="#option-payload-run---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-run---no-default-features"><a class="option-anchor" href="#option-payload-run---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-payload-run---target"><a class="option-anchor" href="#option-payload-run---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Run for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-run---release"><a class="option-anchor" href="#option-payload-run---release"></a><code>--release</code></dt>
<dd class="option-desc">Run optimized artifacts with the <code>release</code> profile. See the
<a href="#profiles">PROFILES</a> section for details on how this affects profile
selection.</dd>



</dl>

### Output Options

<dl>
<dt class="option-term" id="option-payload-run---target-dir"><a class="option-anchor" href="#option-payload-run---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-payload-run--v"><a class="option-anchor" href="#option-payload-run--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-run---verbose"><a class="option-anchor" href="#option-payload-run---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-run--q"><a class="option-anchor" href="#option-payload-run--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-run---quiet"><a class="option-anchor" href="#option-payload-run---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-run---color"><a class="option-anchor" href="#option-payload-run---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



<dt class="option-term" id="option-payload-run---message-format"><a class="option-anchor" href="#option-payload-run---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
<dd class="option-desc">The output format for diagnostic messages. Can be specified multiple times
and consists of comma-separated values. Valid values:</p>
<ul>
<li><code>human</code> (default): Display in a human-readable text format. Conflicts with
<code>short</code> and <code>json</code>.</li>
<li><code>short</code>: Emit shorter, human-readable text messages. Conflicts with <code>human</code>
and <code>json</code>.</li>
<li><code>json</code>: Emit JSON messages to stdout. See
<a href="../reference/external-tools.html#json-messages">the reference</a>
for more details. Conflicts with <code>human</code> and <code>short</code>.</li>
<li><code>json-diagnostic-short</code>: Ensure the <code>rendered</code> field of JSON messages contains
the &quot;short&quot; rendering from rustc. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-diagnostic-rendered-ansi</code>: Ensure the <code>rendered</code> field of JSON messages
contains embedded ANSI color codes for respecting rustc's default color
scheme. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-render-diagnostics</code>: Instruct Payload to not include rustc diagnostics in
in JSON messages printed, but instead Payload itself should render the
JSON diagnostics coming from rustc. Payload's own JSON diagnostics and others
coming from rustc are still emitted. Cannot be used with <code>human</code> or <code>short</code>.</li>
</ul></dd>



</dl>

### Manifest Options

<dl>

<dt class="option-term" id="option-payload-run---manifest-path"><a class="option-anchor" href="#option-payload-run---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-run---frozen"><a class="option-anchor" href="#option-payload-run---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-run---locked"><a class="option-anchor" href="#option-payload-run---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-run---offline"><a class="option-anchor" href="#option-payload-run---offline"></a><code>--offline</code></dt>
<dd class="option-desc">Prevents Payload from accessing the network for any reason. Without this
flag, Payload will stop with an error if it needs to access the network and
the network is not available. With this flag, Payload will attempt to
proceed without the network if possible.</p>
<p>Beware that this may result in different dependency resolution than online
mode. Payload will restrict itself to crates that are downloaded locally, even
if there might be a newer version as indicated in the local copy of the index.
See the <a href="payload-fetch.html">payload-fetch(1)</a> command to download dependencies before going
offline.</p>
<p>May also be specified with the <code>net.offline</code> <a href="../reference/config.html">config value</a>.</dd>



</dl>

### Common Options

<dl>

<dt class="option-term" id="option-payload-run-+toolchain"><a class="option-anchor" href="#option-payload-run-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-run--h"><a class="option-anchor" href="#option-payload-run--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-run---help"><a class="option-anchor" href="#option-payload-run---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-run--Z"><a class="option-anchor" href="#option-payload-run--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


### Miscellaneous Options

<dl>
<dt class="option-term" id="option-payload-run--j"><a class="option-anchor" href="#option-payload-run--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-run---jobs"><a class="option-anchor" href="#option-payload-run---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of CPUs.</dd>


</dl>

## PROFILES

Profiles may be used to configure compiler options such as optimization levels
and debug settings. See [the reference](../reference/profiles.html) for more
details.

Profile selection depends on the target and crate being built. By default the
`dev` or `test` profiles are used. If the `--release` flag is given, then the
`release` or `bench` profiles are used.

Target | Default Profile | `--release` Profile
-------|-----------------|---------------------
lib, bin, example | `dev` | `release`
test, bench, or any target in "test" or "bench" mode | `test` | `bench`

Dependencies use the `dev`/`release` profiles.


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Build the local package and run its main target (assuming only one binary):

       payload run

2. Run an example with extra arguments:

       payload run --example exname -- --exoption exarg1 exarg2

## SEE ALSO
[payload(1)](payload.html), [payload-build(1)](payload-build.html)
