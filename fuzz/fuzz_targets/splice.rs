// AST-splicing target: start from a REAL Luau program (the embedded conformance
// suite) and apply a fuzzer-byte-driven sequence of statement-level mutations to
// it (see `ulua_fuzz::generate_spliced`). The leading bytes pick the seed; the
// tail bytes are the mutation program, so AFL mutating the tail explores nearby
// mutations of the same real script — reaching language-feature combinations a
// from-scratch grammar never produces. Type-check, compile, and (if it compiles)
// run the result under a step limit; must never crash.

use std::cell::RefCell;

use ulua_rt::Checker;

#[cfg(not(feature = "afl-runtime"))]
include!("standalone.rs");

use ulua_rt::Lua;

thread_local! {
    // Builtins registered once (the expensive step); see the `typeck` target.
    static CHECKER: RefCell<ulua_rt::Checker> = RefCell::new(Checker::new());
}

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate_spliced(data);

  // Type-check on THIS thread so the reusable thread-local CHECKER stays
  // amortized (a thread-local is per-thread; running it on a fresh spawned
  // thread would rebuild the frontend every input). The checker recurses on AST
  // depth — shallow for spliced programs — so it doesn't need the big stack.
  CHECKER.with(|c| {
    let _ = c.borrow_mut().check(&src);
  });

  // Run the (spliced REAL) program on a large native stack: it can recurse ~20k
  // deep (e.g. spliced `pcall.luau`), and ulua recurses natively per Lua call,
  // so the default stack would abort — a false positive, not a bug. See
  // `ulua_fuzz::run_on_big_stack`. The VM state is created INSIDE the thread
  // (Lua/Rc are !Send); only the owned source crosses the boundary.
  ulua_fuzz::run_on_big_stack(move || run_program(&src));
}

fn run_program(src: &str) {
  let lua = Lua::new();
  ulua_fuzz::install_step_limit(&lua);

  if let Ok(f) = lua.load(src).set_name("fuzz").into_function() {
    let _ = f.call::<()>(());
  }
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
