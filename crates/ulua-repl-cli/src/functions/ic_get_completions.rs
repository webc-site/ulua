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
/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
pub(crate) unsafe fn ic_get_completions(l: *mut LuaState, edit_buffer: &str) -> Vec<Pair> {
  let mut completions: Vec<Pair> = Vec::new();

  // Safety: l 有效（透传自 complete_repl 的存活状态）；completions 是栈上局部 Vec，回调 &mut 借用仅在本帧同步调用期内存，VM 遍历全局表期间不会重入或逃逸持有。
  unsafe {
    get_completions(l, edit_buffer, &mut |completion: &str, display: &str| {
      completions.push(Pair {
        display: display.into(),
        replacement: completion.into(),
      });
    });
  }

  completions
}
