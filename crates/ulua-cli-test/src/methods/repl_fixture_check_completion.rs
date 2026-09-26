use alloc::string::ToString;

use crate::{records::completion::Completion, type_aliases::completion_set::CompletionSet};

/// 对齐 cpp/tests/Repl.test.cpp 的 `checkCompletion`：期望补全 =
/// `prefix + expected`，display 取 `expected` 中 `(` 之前的部分
pub fn repl_fixture_check_completion(
  completions: &CompletionSet,
  prefix: &str,
  expected: &str,
) -> bool {
  let expected_display = &expected[..expected.find('(').unwrap_or(expected.len())];

  let expected_completion = Completion {
    completion: format!("{prefix}{expected}"),
    display: expected_display.to_string(),
  };

  completions.contains(&expected_completion)
}
