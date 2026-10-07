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

// ---------- 台账：`.to_string()` alloc 面让位登记（r7-treq3，2026-09-28） ----------
//
// :16 一枚让：`expected_display.to_string()` 物化的是查询探针
// `Completion { .. }`（瞬时值，:19 `contains(&probe)` 后即弃），
// `Completion.display` 按值 `String`（records/completion.rs:8），
// `BTreeSet::contains` 只收 `&Completion`，无借用探针口，直传 `&str` 即
// E0308（定型引按值口让位先例不复测）。永让。解锁=借用形探针类型
// （泛型/Cow 的 `BTreeSet` 异构查询），属记录面改造，本票不抢跑。
