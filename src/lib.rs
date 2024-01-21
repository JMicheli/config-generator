//! The [ConfigGenerator] derive macro is intended to simplify the process of writing
//! configuration loading boilerplate. It currently provides two main features:
//! TOML loading, and loading of values from environment variables.
//!
//! The macro is used by applying it to a struct via `#[derive(ConfigGenerator)`.
//!
//! ```
//! use config_generator::ConfigGenerator;
//!
//! #[derive(ConfigGenerator, Default)]
//! struct Config {
//!   #[env_key = "NAME_ENV_KEY"]
//!   pub name: String,
//!   #[env_key = "THREAD_COUNT"]
//!   pub thread_count: u32,
//!   #[env_key = "ALLOWED_ORIGINS"]
//!   pub allowed_origins: Vec<String>,
//!   #[env_key = "ASSETS_PATH"]
//!   pub assets_path: Option<String>,
//! }
//! ```
//!
//! This generates a private struct called `Optional<InputStruct>`. Thus, in the example above,
//! `OptionalConfig` would be generated. The generated optional struct is identical to the input
//! struct except that each of its fields is `Option<T>`. This field is used internally to track
//! loaded data and apply it to the configuration struct when the user calls the `with_toml` or
//! `with_environment` functions to load values.
//!
//! For example, the following basic example loads values from an input toml file.
//! ```
//! use config_generator::ConfigGenerator;
//!
//! #[derive(ConfigGenerator, Default)]
//! struct Config {
//!   pub name: String,
//! }
//!
//! fn main() {
//!   let config = Config::default().with_toml(&"path/to/config.toml");
//! }
//! ```
//!
//! Similarly, the following example would load values from the environment, specifically
//! from the `NAME_KEY` environment variable annotated above the `name` field.
//! ```
//! use config_generator::ConfigGenerator;
//!
//! #[derive(ConfigGenerator, Default)]
//! struct Config {
//!   #[env_key = "NAME_KEY"]
//!   pub name: String,
//! }
//!
//! fn main() {
//!   let config = Config::default().with_environment();
//! }
//! ```
//!
//! These functions can also be composed together. When composed, a latter-executed function
//! will superimpose loaded values on the prior config.
//! ```
//! use config_generator::ConfigGenerator;
//!
//! #[derive(ConfigGenerator, Default)]
//! struct Config {
//!   #[env_key = "NAME_KEY"]
//!   pub name: String,
//! }
//!
//! fn main() {
//!   let config = Config::default()
//!     .with_toml(&"path/to/config.toml")
//!     .with_environment();
//! }
//! ```
//!
//! Superimposing values follows certain rules:
//! 1. If the original struct had an `Option<T>` field, the value for that field will _only_ be
//! replaced if the new value is `Some`. A subseqently-loaded `None` value will _not_ be applied.
//! 2. If the original struct had a `Vec<T>` field, subsequently-loaded values will be _combined_
//! with prior values, such that the `Vec` has each new value pushed to the end.
//! 3. For all other types, if a value is loaded, then that value will replace any previously-set value.

pub use config_gen_macro_impl::ConfigGenerator;
