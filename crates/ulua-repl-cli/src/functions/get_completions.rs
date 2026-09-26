//! 对应 cpp `CLI/include/Luau/Repl.h` 导出的 `getCompletions`：`Repl.test.cpp`
//! 的 `getCompletionSet` 就是通过它拿到补全项，因此本函数是 crate 的公开入口，
//! 内部的 `complete_indexer` 在 cpp 里是 `static`。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::complete_indexer::complete_indexer;

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
pub unsafe fn get_completions(
  l: *mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // Safety: l 由 REPL 单线程补全路径（ic_get_completions/complete_repl）传入且此刻有效，complete_indexer 的 /// # Safety 前提（有效状态+单线程）由同一调用链保持。
  unsafe {
    complete_indexer(l, edit_buffer, add_completion_callback);
  }
}
