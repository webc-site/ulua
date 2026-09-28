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
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`：`ic_get_completions` 会解引用它遍历全局表；调用须在
/// 该状态有效且无并发的 REPL 单线程路径上进行。
pub(crate) unsafe fn complete_repl(l: *mut LuaState, line: &str, pos: usize) -> (usize, Vec<Pair>) {
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
  // Safety: l 是 ReplHelper 持有的主线程状态，rustyline 在同一 REPL 线程的补全回调里同步调用（状态自 run_repl_impl 起全程有效、无并发），满足 ic_get_completions 的存活前提。
  let completions = unsafe { ic_get_completions(l, edit_buffer) };

  (start, completions)
}
