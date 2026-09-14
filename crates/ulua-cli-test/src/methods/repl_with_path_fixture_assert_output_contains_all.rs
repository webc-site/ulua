use crate::records::repl_with_path_fixture::ReplWithPathFixture;

impl ReplWithPathFixture {
  /// 断言捕获输出包含 `list` 中所有片段（对齐 cpp 的 assertOutputContainsAll）
  pub fn assert_output_contains_all(&mut self, list: &[&str]) {
    let captured_output = self.get_captured_output();
    for elem in list {
      assert!(
        captured_output.contains(elem),
        "Captured output: {}",
        captured_output
      );
    }
  }
}
