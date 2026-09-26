// Port of Luau's `fuzz/proto.cpp`: structured generation. Rather than feeding
// raw bytes at the parser, the bytes drive a grammar that emits syntactically
// valid Luau (see `ulua_fuzz::generate`, which builds an `Unstructured` over the
// bytes), so the input reaches deep into the compiler + VM while AFL's coverage
// feedback steers generation. Compile and (if it compiles) run under an interrupt
// step-limit; must never crash.

use std::cell::RefCell;

#[cfg(not(feature = "afl-runtime"))]
include!("standalone.rs");

use ulua_rt::Lua;

thread_local! {
    // The type-check here used the one-shot `ulua_rt::check`, which rebuilds the
    // frontend and re-checks the whole @luau definition file every input — that
    // alone capped the target near ~400 exec/s (observed ~77/s combined with the
    // compile+run below). Reusing a Checker registers the builtins once, so the
    // per-input analysis cost drops to just parsing+checking the program. A faster
    // target explores more inputs per wall-clock second — i.e. finds more bugs.
    static CHECKER: RefCell<ulua_rt::Checker> = RefCell::new(ulua_rt::Checker::new());
}

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate(data);

  // Also type-check it (valid programs exercise the analysis layer). 在当前
  // 线程做：thread-local CHECKER 逐输入摊销，跨线程会每输入重建前端。
  CHECKER.with(|c| {
    let _ = c.borrow_mut().check(&src);
  });

  // VM 状态在大栈线程内创建（Lua/Rc 均 !Send），只把 owned 源码跨界传入；
  // 语法合法程序可深递归（`f() f()` 自嵌套），默认栈会假阳性 abort。见
  // `ulua_fuzz::run_on_big_stack`。
  ulua_fuzz::run_on_big_stack(move || {
    let lua = Lua::new();
    ulua_fuzz::install_step_limit(&lua);
    if let Ok(f) = lua.load(&src).set_name("fuzz").into_function() {
      let _ = f.call::<()>(());
    }
  });
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
