use alloc::string::ToString;

use ulua_repl_cli::functions::get_completions::get_completions;
use ulua_vm::functions::lua_gettop::lua_gettop;

use crate::{
  records::{completion::Completion, repl_fixture::ReplFixture},
  type_aliases::completion_set::CompletionSet,
};

impl ReplFixture {
  /// cpp `ReplFixture::getCompletionSet`：经 `Repl.h` 导出的 `getCompletions`
  /// 收集补全项，并校验其不改变 Lua 栈顶。
  pub fn get_completion_set(&mut self, input_prefix: &str) -> CompletionSet {
    let mut result = CompletionSet::default();

    // SAFETY: `self.l` 为 fixture 持有的活跃主线程状态。
    let top = unsafe { lua_gettop(self.l) };

    let mut callback = |completion: &str, display: &str| {
      result.insert(Completion {
        completion: completion.to_string(),
        display: display.to_string(),
      });
    };

    // SAFETY: 同上；`input_prefix` 为合法 UTF-8 前缀。
    unsafe { get_completions(self.l, input_prefix, &mut callback) };

    debug_assert!(top == unsafe { lua_gettop(self.l) });

    result
  }
}
