The "authors" field in the manifest is determined from the environment or
configuration settings. A name is required and is determined from (first match
wins):

- `payload-new.name` Payload config value
- `PAYLOAD_NAME` environment variable
- `GIT_AUTHOR_NAME` environment variable
- `GIT_COMMITTER_NAME` environment variable
- `user.name` git configuration value
- `USER` environment variable
- `USERNAME` environment variable
- `NAME` environment variable

The email address is optional and is determined from:

- `payload-new.email` Payload config value
- `PAYLOAD_EMAIL` environment variable
- `GIT_AUTHOR_EMAIL` environment variable
- `GIT_COMMITTER_EMAIL` environment variable
- `user.email` git configuration value
- `EMAIL` environment variable

See [the reference](../reference/config.html) for more information about
configuration files.
