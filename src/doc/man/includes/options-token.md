{{#option "`--token` _token_" }}
API token to use when authenticating. This overrides the token stored in
the credentials file (which is created by {{man "payload-login" 1}}).

[Payload config](../reference/config.html) environment variables can be
used to override the tokens stored in the credentials file. The token for
crates.io may be specified with the `PAYLOAD_REGISTRY_TOKEN` environment
variable. Tokens for other registries may be specified with environment
variables of the form `PAYLOAD_REGISTRIES_NAME_TOKEN` where `NAME` is the name
of the registry in all capital letters.
{{/option}}
