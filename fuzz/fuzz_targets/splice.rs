// AST-splicing target: start from a REAL Luau program (the embedded conformance
// suite) and apply a fuzzer-byte-driven sequence of statement-level mutations to
// it (see `ulua_fuzz::generate_spliced`). The leading bytes pick the seed; the
// tail bytes are the mutation program, so AFL mutating the tail explores nearby
// mutations of the same real script — reaching language-feature combinations a
// from-scratch grammar never produces. Type-check, compile, and (if it compiles)
// run the result under a step limit; must never crash.

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate_spliced(data);

  // Type-check on THIS thread so the reusable thread-local Checker stays
  // amortized (a thread-local is per-thread; running it on a fresh spawned
  // thread would rebuild the frontend every input). The checker recurses on AST
  // depth — shallow for spliced programs — so it doesn't need the big stack.
  // 复用 `ulua_fuzz::check_reuse`（同五靶共用的前端摊销样板）。
  let _ = ulua_fuzz::check_reuse(&src);

  // Run the (spliced REAL) program on a large native stack: it can recurse ~20k
  // deep (e.g. spliced `pcall.luau`), and ulua recurses natively per Lua call,
  // so the default stack would abort — a false positive, not a bug. See
  // `ulua_fuzz::run_on_big_stack`. The VM state is created INSIDE the thread
  // (Lua/Rc are !Send); only the owned source crosses the boundary.
  ulua_fuzz::run_on_big_stack(move || ulua_fuzz::run_program(&src));
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
