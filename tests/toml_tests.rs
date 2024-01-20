//! Test the toml loading capabilities of the macro.

use config_generator::ConfigGenerator;

#[derive(ConfigGenerator, Default)]
pub struct TestParseConfig {
  #[env_key = "X_KEY"]
  pub x: u32,
}

#[derive(ConfigGenerator, Default)]
pub struct TestVecConfig {
  #[env_key = "VEC_KEY"]
  pub list: Vec<String>,
}

#[derive(ConfigGenerator, Default)]
pub struct TestOptionConfig {
  #[env_key = "OPTIONAL_KEY"]
  pub optional: Option<String>,
}

#[test]
fn test_parse_config() {
  let test_struct = TestParseConfig::default().with_toml(&"tests/tomls/simple_config.toml");
  assert_eq!(test_struct.x, 10);
}

#[test]
fn test_vec_config() {
  let test_struct = TestVecConfig::default().with_toml(&"tests/tomls/simple_config.toml");
  assert_eq!(
    test_struct.list,
    vec!["val1".to_string(), "val2".to_string(), "val3".to_string()]
  )
}

#[test]
fn test_optional_config() {
  let test_struct = TestOptionConfig::default().with_toml(&"tests/tomls/simple_config.toml");

  assert!(test_struct.optional.is_some());
  assert_eq!(test_struct.optional, Some("a_value".to_string()));
}