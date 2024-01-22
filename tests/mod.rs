//! This module contains integration tests for the macro.

mod env_tests;
#[cfg(feature = "load_toml")]
mod toml_tests;

#[test]
fn test_should_compile() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/should_fail/simple.rs");
}
