use alloc::string::ToString;

use ulua_repl_cli::functions::get_completions::get_completions;

use crate::{
  records::{completion::Completion, repl_fixture::ReplFixture},
  type_aliases::completion_set::CompletionSet,
};

impl ReplFixture {
  /// cpp `ReplFixture::getCompletionSet`：经 `Repl.h` 导出的 `getCompletions`
  /// 收集补全项，并校验其不改变 Lua 栈顶。
  pub fn get_completion_set(&mut self, input_prefix: &str) -> CompletionSet {
    let mut result = CompletionSet::default();

    let top = self.state_mut().get_top();

    let mut callback = |completion: &str, display: &str| {
      result.insert(Completion {
        completion: completion.to_string(),
        display: display.to_string(),
      });
    };

    // `get_completions` 已按 review.md §2 收编为 `&mut LuaState` 借用形的安全 fn：
    // 句柄经 `state_mut` 在夹具层唯一物化点交出（fixture 持有的活跃主线程状态），
    // `input_prefix` 为合法 UTF-8 前缀。
    get_completions(self.state_mut(), input_prefix, &mut callback);

    debug_assert!(top == self.state_mut().get_top());

    result
  }
}

// ---------- 台账：`.to_string()` alloc 面让位登记（r7-treq3，2026-09-28） ----------
//
// :22/:23 两枚全让：回调形参 `completion: &str`/`display: &str` 借自
// `get_completions` 每轮实参，不出闭包即失效；`Completion` 双字段按值
// `String`（records/completion.rs:7-8）且 `CompletionSet = BTreeSet<Completion>`
// 按值存储，直传 `&str` 即 E0308（签名通读定型，引 tstr16 按值字段让位先例
// 不复测）。真物化下限，永让；本文件体零改动。解锁=无（存储语义需 owned）。
