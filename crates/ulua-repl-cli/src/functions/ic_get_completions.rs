use alloc::vec::Vec;
use core::cell::RefCell;

use rustyline::completion::Pair;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::get_completions::get_completions;

// Faithful rustyline analog of Repl.cpp's `icGetCompletions`.
//
// In C++ this bridged isocline's completion environment to the REPL's
// `getCompletions`, calling `ic_add_completion_ex(cenv, completion, display, …)`
// for each match. With rustyline the "completion environment" becomes a
// `Vec<Pair>`: the `replacement` is the full completed word (what isocline used
// as the inserted completion text) and the `display` is the matched key (the
// human-readable alternative that isocline displayed when listing matches).
//
// `getCompletions`'s callback is `const AddCompletionCallback&` (an immutable
// `Fn` in Rust, mirroring C++'s `std::function` passed by const reference), so
// the accumulator uses interior mutability to collect the matches.
pub unsafe fn ic_get_completions(l: *mut lua_State, edit_buffer: &str) -> Vec<Pair> {
  unsafe {
    let completions: RefCell<Vec<Pair>> = RefCell::new(Vec::new());

    get_completions(l, edit_buffer, &|completion: &str, display: &str| {
      completions.borrow_mut().push(Pair {
        display: display.into(),
        replacement: completion.into(),
      });
    });

    completions.into_inner()
  }
}
