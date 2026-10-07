// Port of Luau's `kFuzzVM` path: compile arbitrary source and, if it compiles,
// run it on the VM. Execution is bounded by an interrupt step-limit so a
// generated infinite loop can't hang the fuzzer. The VM must never panic/crash
// — only return `Ok`/`Err`.

use std::str::from_utf8;

fn exercise_input(data: &[u8]) {
  let Ok(src) = from_utf8(data) else {
    return;
  };
  // VM 状态必须在大栈线程内创建（Lua/Rc 均 !Send），只把 owned 源码跨界传
  // 入——Lua-to-Lua 调用走原生递归，默认栈会被合法深递归打穿成假阳性 abort。
  let src = src.to_string();
  ulua_fuzz::run_on_big_stack(move || ulua_fuzz::run_program(&src));
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
