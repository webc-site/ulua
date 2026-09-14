use alloc::vec::Vec;

use rustyline::completion::Pair;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::{
  ic_get_completions::ic_get_completions, is_method_or_function_char::is_method_or_function_char,
};

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
pub unsafe fn complete_repl(l: *mut lua_State, line: &str, pos: usize) -> (usize, Vec<Pair>) {
  unsafe {
    let bytes = line.as_bytes();

    // Walk backwards from the cursor over method/function characters
    // (alphanumeric, '.', ':', '_') to locate the start of the word, exactly
    // as isocline's word-boundary detection (`isMethodOrFunctionChar`) does.
    let mut start = pos;
    while start > 0 {
      if is_method_or_function_char(bytes[start - 1]) {
        start -= 1;
      } else {
        break;
      }
    }

    let edit_buffer = &line[start..pos];
    let completions = ic_get_completions(l, edit_buffer);

    (start, completions)
  }
}
