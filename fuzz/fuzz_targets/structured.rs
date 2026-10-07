// Port of Luau's `fuzz/proto.cpp`: structured generation. Rather than feeding
// raw bytes at the parser, the bytes drive a grammar that emits syntactically
// valid Luau (see `ulua_fuzz::generate`, which builds an `Unstructured` over the
// bytes), so the input reaches deep into the compiler + VM while AFL's coverage
// feedback steers generation. Compile and (if it compiles) run under an interrupt
// step-limit; must never crash.

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate(data);

  // Also type-check it (valid programs exercise the analysis layer). 复用线程本地
  // Checker（见 `ulua_fuzz::check_reuse`），在本线程逐输入摊销——跨线程会每输入
  // 重建前端。
  let _ = ulua_fuzz::check_reuse(&src);

  // VM 状态在大栈线程内创建（Lua/Rc 均 !Send），只把 owned 源码跨界传入；
  // 语法合法程序可深递归（`f() f()` 自嵌套），默认栈会假阳性 abort。见
  // `ulua_fuzz::run_on_big_stack` 与 `ulua_fuzz::run_program`。
  ulua_fuzz::run_on_big_stack(move || ulua_fuzz::run_program(&src));
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
