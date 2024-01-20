//! Should fail because "env" isn't one of the attributes we define.

use config_generator::ConfigGenerator;

#[derive(ConfigGenerator, Default)]
struct TestStruct {
  #[env = "NAME_ENV_KEY"]
  pub name: String,
  pub description: Option<String>,
  pub x: u32,
  pub y: u32,
}

fn main() {
  let test_struct = TestStruct::default().with_env();
}
