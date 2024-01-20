//! Test the environment-loading capabilities of the macro.

use std::env;

use config_generator::ConfigGenerator;

#[test]
fn test_parse_config() {
  #[derive(ConfigGenerator, Default)]
  struct TestParseConfig {
    #[env_key = "X_KEY"]
    pub x: u32,
  }

  env::set_var("X_KEY", "10");
  let test_struct = TestParseConfig::default().with_env();

  assert_eq!(test_struct.x, 10);
}

#[test]
fn test_vec_config() {
  #[derive(ConfigGenerator, Default)]
  struct TestVecConfig {
    #[env_key = "VEC_KEY"]
    pub list: Vec<String>,
  }

  env::set_var("VEC_KEY", "val1, val2, val3");
  let test_struct = TestVecConfig::default().with_env();

  assert_eq!(
    test_struct.list,
    vec!["val1".to_string(), "val2".to_string(), "val3".to_string()]
  )
}

#[test]
fn test_option_config() {
  #[derive(ConfigGenerator, Default)]
  struct TestOptionConfig {
    #[env_key = "OPTIONAL_KEY"]
    pub optional: Option<String>,
  }

  env::set_var("OPTIONAL_KEY", "a_value");
  let test_struct = TestOptionConfig::default().with_env();

  assert!(test_struct.optional.is_some());
  assert_eq!(test_struct.optional, Some("a_value".to_string()));
}
