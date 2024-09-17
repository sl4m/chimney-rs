[![Build](https://github.com/sl4m/chimney-rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/sl4m/chimney-rs/actions/workflows/build.yml)

An implementation of the [Santa sync protocol](https://santa.dev/development/sync-protocol.html).

## Getting Started

### Configuring chimney

Pass in a TOML based configuration file path as a flag or environment variable named `CHIMNEY_CONFIG`.

| Key                              | Required | Description |
| -------------------------------- | -------- | ----------- |
| bind_address                     | true     | chimney will bind to IP address and TCP port. |
| client_config_path               | true     | Path to the client configurations. Must contain a `global.toml`. More info under Client Configurations. |
| event_log_path                   | false    | Path to the event log file. Enable if you want to record events. Uses Bunyan logging. |
| log_level                        | false    | Log level. Defaults to info. |
| log_path                         | true     | Path to the log file. |
| tls_config.cert_file             | false    | Path to the TLS cert file. |
| tls_config.key_file              | false    | Path to the TLS private key file. Must be in PKCS#8 format. |

A couple of things to note:

1. If you plan to terminate SSL elsewhere, you do not need to provide a `tls_config` section.
1. All paths must be absolute paths. Tilde expansion is not supported at this time.

#### chimney config example

```toml
bind_address = "127.0.0.1:0"
client_config_path = "/path/to/client/configs"
event_log_path = "/path/to/event.log"
log_level = "info"
log_path = "/path/to/chimney.log"

[tls_config]
cert_file = "/path/to/server.crt"
key_file = "/path/to/server.key"
```

### Client configurations

Client configuration files are also TOML based. They are read by chimney using the provided path (`client_config_path`) and cached on server start. `global.toml` must exist in the path. Similar to moroz, chimney uses `global.toml` to form `preflight` and `ruledownload` responses unless a machine specific configuration is provided. All machine specific configuration files are named after their machine id (e.g., hardware UUID - 3AC82A0D-3779-7B99-A598-C02FED123A04.toml).

| Key                              | Required | Type    | Description |
| -------------------------------- | -------- | ------- | ----------- |
| enable_bundles                   | false    | boolean | Enable bundle scanning. Defaults to false. |
| enable_transitive_rules          | false    | boolean | Whether or not to enable transitive allowlisting. Defaults to false. |
| batch_size                       | false    | number  | Number of events to upload at a time. |
| full_sync_interval               | false    | number  | Number of seconds between full syncs. Defaults to 600 seconds. |
| client_mode                      | true     | string  | Operating mode to set for the client. Either `MONITOR` or `LOCKDOWN`. |
| allowed_path_regex               | false    | string  | Regular expression to allow a binary to execute from a path. |
| blocked_path_regex               | false    | string  | Regular expression to block a binary from executing by path. |
| block_usb_mount                  | false    | boolean | Block USB mass storage devices. Defaults to false. |
| remount_usb_mode                 | false    | string  | Force USB mass storage devices to be remounted with the given permissions. |
| sync_type                        | false    | string  | The type of sync the client should perform. Either `NORMAL`, `CLEAN`, or `CLEAN_ALL`. Defaults to `NORMAL`. |
| override_file_access_action      | true     | string  | Override file access config policy action. Either `DISABLE`, `AUDIT_ONLY`, or `NONE`. |
| rules.n.rule_type                | true*    | string  | Only required if defining a rule. Identifies the type of rule. Either `BINARY`, `CERTIFICATE`, `SIGNINGID`, `TEAMID`, or `CDHASH`. |
| rules.n.policy                   | true*    | string  | Only required if defining a rule. Identifies the action to perform in response to the rule matching. Either `ALLOWLIST`, `ALLOWLIST_COMPILER`, `BLOCKLIST`, `REMOVE`, or `SILENT_BLOCKLIST`. |
| rules.n.identifier               | true*    | string  | Only required if defining a rule. The attribute of the binary the rule should match on e.g., the signing ID, team ID, or CDHash of a binary or SHA256 has value. |
| rules.n.custom_msg               | false    | string  | A custom message to display when the rule matches. |
| rules.n.custom_url               | false    | string  | A custom URL to use for the open button when the rule matches. |
| rules.n.creation_time            | false    | float   | Time the rule was created. |
| rules.n.file_bundle_binary_count | false    | number  | The number of binaries in a bundle. |
| rules.n.file_bundle_hash         | false    | string  | The SHA256 of all binaries in a bundle. |

#### Client configuration example

```toml
enable_bundles = false
enable_transitive_rules = true
batch_size = 100
full_sync_interval = 600
client_mode = "MONITOR"
# allowed_path_regex = "^(?:/Users)/.*"
# blocked_path_regex = "^(?:/Users)/.*"
block_usb_mount = false
# remount_usb_mode = "noexec"
sync_type = "CLEAN"
override_file_access_action = "AUDIT_ONLY"

[[rules]]
rule_type = "BINARY"
policy = "BLOCKLIST"
identifier = "2dc104631939b4bdf5d6bccab76e166e37fe5e1605340cf68dab919df58b8eda"
custom_msg = "blocklist firefox"

[[rules]]
rule_type = "TEAMID"
policy = "ALLOWLIST"
identifier = "EQHXZ8M8AV"
custom_msg = "allow google team id"
```
