//! rustyline 版的 cpp `static void icGetCompletions(void* cenv, ...)`。

use alloc::vec::Vec;

use rustyline::completion::Pair;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::get_completions::get_completions;

// cpp 用 isocline 的补全环境（cenv）承接结果；rustyline 的对应物是 `Vec<Pair>`：
// `replacement` 是插入的完整补全文本（cpp 的 completion），`display` 是列出的
// 候选名（cpp 的 display）。
//
// 回调在 Rust 侧是 `&mut impl FnMut`，收集器只是栈上的 Vec，无需 cpp
// `std::function` 捕获语义对应的内部可变性。
// DELIBERATE DEVIATION（review.md §9.3）：cpp `icGetCompletions` 的 rustyline 版，
// 把回调收集器换成栈上 `Vec<Pair>`；`get_completions`（ulua-vm c-API 引用形安全面）
// 在其内部遍历全局表。按 §2 收形：句柄以借用 `&mut LuaState` 透传，回调借用仅本帧
// 同步存活、无逃逸，本入口为安全 fn。
//
// 调用序契约（透传自 complete_repl）：`l` 为活跃状态机。
pub(crate) fn ic_get_completions(l: &mut LuaState, edit_buffer: &str) -> Vec<Pair> {
  let mut completions: Vec<Pair> = Vec::new();

  // completions 是栈上局部 Vec，回调 &mut 借用仅在本帧同步调用期内存，
  // VM 遍历全局表期间不会重入或逃逸持有。
  get_completions(l, edit_buffer, &mut |completion: &str, display: &str| {
    completions.push(Pair {
      display: display.into(),
      replacement: completion.into(),
    });
  });

  completions
}
