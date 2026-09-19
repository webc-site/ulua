//! 对应 cpp `CLI/include/Luau/Repl.h` 导出的 `getCompletions`：`Repl.test.cpp`
//! 的 `getCompletionSet` 就是通过它拿到补全项，因此本函数是 crate 的公开入口，
//! 内部的 `complete_indexer` 在 cpp 里是 `static`。

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::complete_indexer::complete_indexer;

/// # Safety
///
/// `l` 必须是有效、活跃的 `lua_State` 指针。
pub unsafe fn get_completions(
  l: *mut lua_State,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  unsafe {
    complete_indexer(l, edit_buffer, add_completion_callback);
  }
}
