# Config Generator

A derive macro for generating boilerplate code for a configuration struct in rust.

## Installation and usage

To install this crate, add it as a dependency:

`cargo add config-generator`

Additionally, the [serde](https://crates.io/crates/serde) and [toml](https://crates.io/crates/toml) crates are required
to support loading from toml.

`cargo add serde toml`

To use, define a struct containing configuration variable that will be loaded by the generated code. `env_key` annotations define an environment variable key that will be used for loading the variable from the environment.

```rust
use config_generator::ConfigGenerator;

#[derive(ConfigGenerator, Default)]
struct Config {
  #[env_key = "NAME_ENV_KEY"]
  pub name: String,
  #[env_key = "THREAD_COUNT"]
  pub thread_count: u32,
  #[env_key = "ALLOWED_ORIGINS"]
  pub allowed_origins: Vec<String>,
  #[env_key = "ASSETS_PATH"]
  pub assets_path: Option<String>,
}
```

The resulting implementations will allow a config to be loaded from a toml file, the environment, or both, with loaded variables overwriting previously set ones. Note that you will need to implement a way to get some starting state for your config (probably either defining a `new` method or deriving `Default` as above).

```rust
// Loads a default config first from a toml file,
// then overlays environment variable values.
let config = Config::default()
  .with_toml("path/to/config.toml")
  .with_environment();
```

### Features

By default, the `load_toml` feature is enabled, supplying a `with_toml` function implementation on the input struct, and consequently requiring the [serde](https://crates.io/crates/serde) and [toml](https://crates.io/crates/toml) crates. This feature can be disabled, and these crates removed, if the user so chooses.

## Acknowledgements

This macro is inspired by a use-case for the [optional_struct](https://crates.io/crates/optional_struct) macro, which also doubles as a very useful resource for those wishing to learn about writing procedural macros in rust.
