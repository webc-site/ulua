//! 对应 cpp `CLI/include/Luau/Repl.h` 导出的 `getCompletions`：`Repl.test.cpp`
//! 的 `getCompletionSet` 就是通过它拿到补全项，因此本函数是 crate 的公开入口，
//! 内部的 `complete_indexer` 在 cpp 里是 `static`。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::complete_indexer::complete_indexer;

// DELIBERATE DEVIATION（review.md §9.3）：cpp `Repl.h` 对外导出的补全入口，`Repl.test.cpp`
// 经 `self.l()` 取句柄调用；本 port 按 §2 把该句柄收编为借用 `&mut LuaState`，存活/独占
// 前提由类型承载，调用方（ulua-cli-test 夹具的 `state_mut`、crate 内 rustyline 补全回调）
// 各自在唯一的裸指针出处物化一次，本入口因此是安全 fn。内部按 §3 收形：透传给
// complete_indexer（ulua-vm c-API 引用形安全面），下游全程只见 `&mut LuaState`。
//
// 调用序契约（由调用方成立）：`l` 为活跃状态机。
pub fn get_completions(
  l: &mut LuaState,
  edit_buffer: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  complete_indexer(l, edit_buffer, add_completion_callback);
}
