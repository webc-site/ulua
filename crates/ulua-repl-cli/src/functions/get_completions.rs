//! 对应 cpp `CLI/include/Luau/Repl.h` 导出的 `getCompletions`：`Repl.test.cpp`
//! 的 `getCompletionSet` 就是通过它拿到补全项，因此本函数是 crate 的公开入口，
//! 内部的 `complete_indexer` 在 cpp 里是 `static`。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::complete_indexer::complete_indexer;

// DELIBERATE DEVIATION（review.md §9.3）：cpp `Repl.h` 对外导出的补全入口，直接以
// `*mut LuaState` 驱动 complete_indexer 遍历全局表（ulua-vm c-API 边界）；按 §2
// 收口为「安全入口 + 存活前置条件」形态。
//
// 前置条件（由 REPL 单线程补全路径传入并逐层透传）：`l` 必须是有效、活跃的
// `LuaState` 指针。
pub fn get_completions(
  l: *mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // complete_indexer 的存活前提（有效状态+单线程）由同一调用链透传保持。
  complete_indexer(l, edit_buffer, add_completion_callback);
}
