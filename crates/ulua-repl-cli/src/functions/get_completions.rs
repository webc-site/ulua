//! 对应 cpp `CLI/include/Luau/Repl.h` 导出的 `getCompletions`：`Repl.test.cpp`
//! 的 `getCompletionSet` 就是通过它拿到补全项，因此本函数是 crate 的公开入口，
//! 内部的 `complete_indexer` 在 cpp 里是 `static`。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::{complete_indexer::complete_indexer, state_ref::state};

// DELIBERATE DEVIATION（review.md §9.3）：cpp `Repl.h` 对外导出的补全入口，`Repl.test.cpp`
// 经 `self.l()`（裸 `*mut LuaState`）调用，故本入口保留裸指针形参（跨 crate 句柄边界，
// cli-test 在夹具层收口）。内部按 §3 收形：入口经 `state` 门面一次物化为借用后透传给
// complete_indexer（ulua-vm c-API 边界），下游全程只见 `&mut LuaState`。
//
// 调用序契约（由 REPL 单线程补全路径传入并逐层透传）：`l` 必须是有效、活跃的
// `LuaState` 指针。
pub fn get_completions(
  l: *mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // 唯一物化点：`state` 门面契约（l 非空、活跃、单线程无并存可变别名）；其后借用透传。
  complete_indexer(state(l), edit_buffer, add_completion_callback);
}
