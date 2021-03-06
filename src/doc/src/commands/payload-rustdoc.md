# payload-rustdoc(1)


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

This command requires that only one target is being compiled when additional
arguments are provided. If more than one target is available for the current
package the filters of `--lib`, `--bin`, etc, must be used to select which
target is compiled.

To pass flags to all rustdoc processes spawned by Payload, use the
`RUSTDOCFLAGS` [environment variable](../reference/environment-variables.html)
or the `build.rustdocflags` [config value](../reference/config.html).

## OPTIONS

### Documentation Options

<dl>

<dt class="option-term" id="option-payload-rustdoc---open"><a class="option-anchor" href="#option-payload-rustdoc---open"></a><code>--open</code></dt>
<dd class="option-desc">Open the docs in a browser after building them. This will use your default
browser unless you define another one in the <code>BROWSER</code> environment variable.</dd>


</dl>

### Package Selection

By default, the package in the current working directory is selected. The `-p`
flag can be used to choose a different package in a workspace.

<dl>

<dt class="option-term" id="option-payload-rustdoc--p"><a class="option-anchor" href="#option-payload-rustdoc--p"></a><code>-p</code> <em>spec</em></dt>
<dt class="option-term" id="option-payload-rustdoc---package"><a class="option-anchor" href="#option-payload-rustdoc---package"></a><code>--package</code> <em>spec</em></dt>
<dd class="option-desc">The package to document. See <a href="payload-pkgid.html">payload-pkgid(1)</a> for the SPEC
format.</dd>


</dl>


### Target Selection

When no target selection options are given, `payload rustdoc` will document all
binary and library targets of the selected package. The binary will be skipped
if its name is the same as the lib target. Binaries are skipped if they have
`required-features` that are missing.

Passing target selection flags will document only the specified
targets. 

Note that `--bin`, `--example`, `--test` and `--bench` flags also 
support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your 
shell accidentally expanding glob patterns before Payload handles them, you must 
use single quotes or double quotes around each glob pattern.

<dl>

<dt class="option-term" id="option-payload-rustdoc---lib"><a class="option-anchor" href="#option-payload-rustdoc---lib"></a><code>--lib</code></dt>
<dd class="option-desc">Document the package's library.</dd>


<dt class="option-term" id="option-payload-rustdoc---bin"><a class="option-anchor" href="#option-payload-rustdoc---bin"></a><code>--bin</code> <em>name</em>...</dt>
<dd class="option-desc">Document the specified binary. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-rustdoc---bins"><a class="option-anchor" href="#option-payload-rustdoc---bins"></a><code>--bins</code></dt>
<dd class="option-desc">Document all binary targets.</dd>



<dt class="option-term" id="option-payload-rustdoc---example"><a class="option-anchor" href="#option-payload-rustdoc---example"></a><code>--example</code> <em>name</em>...</dt>
<dd class="option-desc">Document the specified example. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-rustdoc---examples"><a class="option-anchor" href="#option-payload-rustdoc---examples"></a><code>--examples</code></dt>
<dd class="option-desc">Document all example targets.</dd>


<dt class="option-term" id="option-payload-rustdoc---test"><a class="option-anchor" href="#option-payload-rustdoc---test"></a><code>--test</code> <em>name</em>...</dt>
<dd class="option-desc">Document the specified integration test. This flag may be specified
multiple times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-rustdoc---tests"><a class="option-anchor" href="#option-payload-rustdoc---tests"></a><code>--tests</code></dt>
<dd class="option-desc">Document all targets in test mode that have the <code>test = true</code> manifest
flag set. By default this includes the library and binaries built as
unittests, and integration tests. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
unittest, and once as a dependency for binaries, integration tests, etc.).
Targets may be enabled or disabled by setting the <code>test</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-payload-rustdoc---bench"><a class="option-anchor" href="#option-payload-rustdoc---bench"></a><code>--bench</code> <em>name</em>...</dt>
<dd class="option-desc">Document the specified benchmark. This flag may be specified multiple
times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-rustdoc---benches"><a class="option-anchor" href="#option-payload-rustdoc---benches"></a><code>--benches</code></dt>
<dd class="option-desc">Document all targets in benchmark mode that have the <code>bench = true</code>
manifest flag set. By default this includes the library and binaries built
as benchmarks, and bench targets. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
benchmark, and once as a dependency for binaries, benchmarks, etc.).
Targets may be enabled or disabled by setting the <code>bench</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-payload-rustdoc---all-targets"><a class="option-anchor" href="#option-payload-rustdoc---all-targets"></a><code>--all-targets</code></dt>
<dd class="option-desc">Document all targets. This is equivalent to specifying <code>--lib --bins --tests --benches --examples</code>.</dd>


</dl>


### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-payload-rustdoc---features"><a class="option-anchor" href="#option-payload-rustdoc---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-rustdoc---all-features"><a class="option-anchor" href="#option-payload-rustdoc---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-rustdoc---no-default-features"><a class="option-anchor" href="#option-payload-rustdoc---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-payload-rustdoc---target"><a class="option-anchor" href="#option-payload-rustdoc---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Document for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-rustdoc---release"><a class="option-anchor" href="#option-payload-rustdoc---release"></a><code>--release</code></dt>
<dd class="option-desc">Document optimized artifacts with the <code>release</code> profile. See the
<a href="#profiles">PROFILES</a> section for details on how this affects profile
selection.</dd>



</dl>

### Output Options

<dl>
<dt class="option-term" id="option-payload-rustdoc---target-dir"><a class="option-anchor" href="#option-payload-rustdoc---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-rustdoc--v"><a class="option-anchor" href="#option-payload-rustdoc--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-rustdoc---verbose"><a class="option-anchor" href="#option-payload-rustdoc---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-rustdoc--q"><a class="option-anchor" href="#option-payload-rustdoc--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-rustdoc---quiet"><a class="option-anchor" href="#option-payload-rustdoc---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-rustdoc---color"><a class="option-anchor" href="#option-payload-rustdoc---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



<dt class="option-term" id="option-payload-rustdoc---message-format"><a class="option-anchor" href="#option-payload-rustdoc---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
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
<dt class="option-term" id="option-payload-rustdoc---manifest-path"><a class="option-anchor" href="#option-payload-rustdoc---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-rustdoc---frozen"><a class="option-anchor" href="#option-payload-rustdoc---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-rustdoc---locked"><a class="option-anchor" href="#option-payload-rustdoc---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-rustdoc---offline"><a class="option-anchor" href="#option-payload-rustdoc---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-payload-rustdoc-+toolchain"><a class="option-anchor" href="#option-payload-rustdoc-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-rustdoc--h"><a class="option-anchor" href="#option-payload-rustdoc--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-rustdoc---help"><a class="option-anchor" href="#option-payload-rustdoc---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-rustdoc--Z"><a class="option-anchor" href="#option-payload-rustdoc--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


### Miscellaneous Options

<dl>
<dt class="option-term" id="option-payload-rustdoc--j"><a class="option-anchor" href="#option-payload-rustdoc--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-rustdoc---jobs"><a class="option-anchor" href="#option-payload-rustdoc---jobs"></a><code>--jobs</code> <em>N</em></dt>
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

1. Build documentation with custom CSS included from a given file:

       payload rustdoc --lib -- --extend-css extra.css

## SEE ALSO
[payload(1)](payload.html), [payload-doc(1)](payload-doc.html), [rustdoc(1)](https://doc.dustlang.com/rustdoc/index.html)
