use alloc::vec::Vec;

use rustyline::completion::Pair;
use ulua_cli_lib::functions::is_method_or_function_char::is_method_or_function_char;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::ic_get_completions::ic_get_completions;

// Faithful rustyline analog of Repl.cpp's `completeRepl`.
//
// In C++ this was:
//     ic_complete_word(cenv, editBuffer, icGetCompletions, isMethodOrFunctionChar);
// `ic_complete_word` walks backwards from the cursor over characters for which
// the `is_word_char` predicate (`isMethodOrFunctionChar`) returns true to find
// the start of the word being completed, hands that word to the completer
// (`icGetCompletions`), and reports completions relative to that word start.
//
// rustyline's `Completer::complete(line, pos, ctx)` gives us the full line and
// cursor position and expects `(start, candidates)`, so we reproduce isocline's
// word-boundary scan here and return the matches gathered by `ic_get_completions`.
// DELIBERATE DEVIATION（review.md §9.3）：rustyline 补全回调，把状态句柄透传给
// ic_get_completions（ulua-vm c-API 引用形安全面）；词边界回退扫描为纯字节安全逻辑，
// 无裸指针算术。按 §2 收形：句柄以借用 `&mut LuaState` 传入，本入口为安全 fn，唯一的
// 裸指针物化点留在持有该句柄的 `ReplHelper`（rustyline 回调只能以 `&self` 触发，故其
// 长期持有的存活句柄在本 crate 内只能以裸指针形态登记，见 run_repl_impl 的注）。
//
// 调用序契约（由 ReplHelper 成立）：`l` 为活跃状态机。
pub(crate) fn complete_repl(l: &mut LuaState, line: &str, pos: usize) -> (usize, Vec<Pair>) {
  // rustyline 传的是字节偏移，正常必在 line 内且落在字符边界上；越界时按末尾
  // 裁剪，避免补全回调 panic 打断交互（回调用在 readline 内部）。
  let bytes = line.as_bytes();
  let pos = pos.min(line.len());

  // Walk backwards from the cursor over method/function characters
  // (alphanumeric, '.', ':', '_') to locate the start of the word, exactly
  // as isocline's word-boundary detection (`isMethodOrFunctionChar`) does.
  let mut start = pos;
  while start > 0 && is_method_or_function_char(bytes[start - 1]) {
    start -= 1;
  }

  let edit_buffer = &line[start..pos];
  let completions = ic_get_completions(l, edit_buffer);

  (start, completions)
}
