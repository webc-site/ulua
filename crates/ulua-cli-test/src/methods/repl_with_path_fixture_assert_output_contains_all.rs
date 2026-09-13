use alloc::string::String;

use crate::records::repl_with_path_fixture::ReplWithPathFixture;

impl ReplWithPathFixture {
  pub fn assert_output_contains_all(&mut self, list: &[String]) {
    let captured_output = self.get_captured_output();
    for elem in list {
      assert!(
        captured_output.contains(elem.as_str()),
        "Captured output: {}",
        captured_output
      );
    }
  }
}
