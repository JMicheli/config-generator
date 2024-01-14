use config_gen::ConfigGenerator;

#[derive(ConfigGenerator)]
struct TesterStruct {
  #[env_key = "NAME_KEY"]
  name: Option<String>,
  #[env_key = "X_KEY"]
  x: u32,
  #[env_key = "Y_KEY"]
  y: u32,
  #[env_key = "TEST_VEC_KEY"]
  test_vec: Vec<f32>,
  #[env_key = "TEST_STRING_VEC_KEY"]
  test_string_vec: Vec<String>,
}

fn main() {
  println!("Hello, world!");
}
