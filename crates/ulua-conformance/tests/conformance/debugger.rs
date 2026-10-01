// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
// 调试器、断点、中断与 coverage 钩子用例
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// 本文件保留的裸指针均为 C-API 哨兵形态而非测试数据：`run_conformance` 的 ud、
// `lua_resume` 的 from 参数传 `null_mut()` 表示「无 host 上下文/无父线程」，
// 与 cpp 原样一致，不可安全化。


#[test]
fn conformance_coverage() {
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::functions::{
    conformance_coverage_setup::conformance_coverage_setup,
    default_compile_options::default_compile_options, run_conformance::run_conformance,
  };

  // cpp `Conformance.test.cpp:3317-3319`：`copts = defaultOptions();` 后固定
  // `optimizationLevel = 1`（关闭内联，命中数才是可预期的），并打开表达式级覆盖率。
  let copts = CompileOptions {
    optimization_level: 1,
    coverage_level: 2,
    ..default_compile_options()
  };

  // Safety: 参数均为 'static 字面量与本机函数指针，ud 为故意的 null 哨兵。
  run_conformance(
    "coverage.luau",
    Some(conformance_coverage_setup),
    None,
    None,
    Some(&copts),
    false,
    None,
  );
}

#[test]
fn conformance_debug() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("debug.luau");
}

#[test]
fn conformance_debug_api() {
  use crate::common::functions::{new_state::new_state, safe_api::{getinfo, state_mut, zero_debug}};

  let global_state = new_state();
  let l = global_state.as_ptr();

  state_mut(l).push_number(10.0);

  // `zero_debug` 交出全零 LuaDebug（repr(C) 各字段全零位模式皆合法），
  // getinfo 按 what 串负责填写。
  let mut ar = zero_debug();
  assert_eq!(getinfo(l, -1, b"f\0", &mut ar), 0);
  assert_eq!(getinfo(l, -10, b"f\0", &mut ar), 0);
}

#[test]
fn conformance_debugger() {
  use core::sync::atomic::Ordering;

  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::{
    functions::{
      conformance_debugger_setup::conformance_debugger_setup,
      conformance_debugger_yield::conformance_debugger_yield,
      default_compile_options::default_compile_options, run_conformance::run_conformance,
    },
    records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
  };

  for singlestep in [false, true] {
    CONFORMANCE_DEBUGGER_STATE.reset(singlestep);

    let copts = CompileOptions {
      debug_level: 2,
      ..default_compile_options()
    };

    run_conformance(
      "debugger.luau",
      Some(conformance_debugger_setup),
      Some(conformance_debugger_yield),
      None,
      Some(&copts),
      true,
      None,
    );
    // cpp `Conformance.test.cpp:2331` 的 `CHECK(breakhits == 26)`：13 个生效断点
    // ×2 命中。本 fixture 裁掉了 cpp `tests/conformance/debugger.luau:102-149` 的
    // 两段 `coroutine.finally`（cofinallytest / cowrapfinallytest，连带
    // breakpoint(105/106/111/131/132) 共 5 个断点），剩 8 个生效断点 ×2 = 16。
    // 根因：Rust VM 未实现 `coroutine.finally`——上游 `cpp/VM/src/lcorolib.cpp:488`
    // 在 CO_FUNCS 注册 `{"finally", cofinally}`，全程受 FFlag
    // `DebugLuauCoroutineFinally` 门控，本端口该 flag 与 finally 实现均未移植
    // （`ulua-vm/src/functions/luaopen_coroutine.rs` 的 CO_FUNCS 无 finally 项）。
    assert_eq!(
      CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst),
      16
    );

    if singlestep {
      assert!(CONFORMANCE_DEBUGGER_STATE.stephits.load(Ordering::SeqCst) > 100);
    }
  }
}

#[test]
fn conformance_interrupt() {
  use core::ffi::c_int;

  use ulua_compiler::records::compile_options::CompileOptions;
  use ulua_vm::{enums::lua_status::LuaStatus, macros::lua_globalsindex::LUA_GLOBALSINDEX};

  use crate::common::{
    functions::{
      conformance_interrupt_interrupt::conformance_interrupt_interrupt,
      default_compile_options::default_compile_options, run_conformance::run_conformance,
      safe_api::{callbacks_mut, l_checklstring, newthread, pop, resume, state_mut},
    },
    records::conformance_interrupt_state::{
      CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS, CONFORMANCE_INTERRUPT_MODE_HANG,
      CONFORMANCE_INTERRUPT_MODE_HANG_PCALL, CONFORMANCE_INTERRUPT_MODE_INFLOOP,
      CONFORMANCE_INTERRUPT_STATE,
    },
  };

  // cpp `Conformance.test.cpp:3637-3638`：`copts = defaultOptions();` 后固定
  // `optimizationLevel = 1` —— 关闭循环展开，中断命中数的预期值才成立（默认的
  // `LUAU_OPTIMIZATION_LEVEL` 可能是 0/2）。
  //
  // cpp `Conformance.test.cpp:3728` 还设
  // `ScopedFastFlag luauFastpcallInterrupt{FFlag::LuauFastpcallInterrupt, true}`：仅给
  // FASTPCALL 路径（VM `lvmexecute.cpp:3708` 与原生 `IrTranslation.cpp:1287` 的
  // INTERRUPT 注入）补中断检查。本端口未移植该旗标与 fastpcall 机制：
  // `ulua-vm/src/functions/luau_execute.rs` 的 LOP_FASTPCALL 分支恒等价于上游
  // `LuauFastpcall = false` 缺省（标记指令直通慢路径 CALL，中断照常检查），
  // 故无需作用域 flag。
  let copts = CompileOptions {
    optimization_level: 1,
    ..default_compile_options()
  };

  let global_state = run_conformance(
    "interrupt.luau",
    None,
    None,
    None,
    Some(&copts),
    false,
    None,
  );
  let l = global_state.as_ptr();

  // `l` 为 `run_conformance` 返回的存活状态；把 interrupt 钩子登记进
  // callbacks 是 cpp `Interrupt` 用例同款的裸函数指针写入。
  callbacks_mut(l).interrupt = Some(conformance_interrupt_interrupt);

  // 用例①（cpp `Interrupt` 的 `test` 线程块）：两段 resume 分别停在 yield 与
  // 终点，核对中断计数落点。`t` 为新建线程，`test` 由 interrupt.luau 注入全局表。
  let t = newthread(l);
  state_mut(t).get_field_str(LUA_GLOBALSINDEX, "test");

  CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS);

  // 两次 resume 在同一存活线程 `t` 上顺序执行（global_state 覆盖本块）；
  // `index()` 读的是中断回调留下的原子快照。
  // FFI: c-API 要求 NULL（resume 的 from 为无父线程哨兵，与 cpp 原样一致）。
  let status = resume(t, None, 0);
  assert_eq!(status, LuaStatus::Yield as c_int);
  assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 4);

  let status = resume(t, None, 0);
  assert_eq!(status, LuaStatus::Ok as c_int);
  assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 22);

  pop(l, 1);

  // 用例②（cpp `Interrupt` 的 `for (test = 1; test <= 10)`）：infloop1..10 每次
  // 中断都命中，resume 以 yield 收场。
  for test in 1..=10 {
    let name = format!("infloop{test}");

    // 同用例①，`t` 为本轮新线程，栈上仅有 infloop{test} 函数。
    let t = newthread(l);
    state_mut(t).get_field_str(LUA_GLOBALSINDEX, &name);

    CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_INFLOOP);

    // yield 状态与中断落点由 interrupt.luau 固定。
    let status = resume(t, None, 0);
    assert_eq!(status, LuaStatus::Yield as c_int);
    assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 11);

    pop(l, 1);
  }

  CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_HANG);

  // 用例③（cpp `Conformance.test.cpp:3729` 的 `for (int test = 1; test <= 7; ++test)`）：
  // hang1..hang7 必须被中断为 ErrRun，错误串含 "timeout"。其中 hang7 是
  // `pcall(l0)` 的指数级递归，专门验证中断错误能穿透海量嵌套的受保护调用。
  for test in 1..=7 {
    let name = format!("hang{test}");

    // 同用例①，`t` 为本轮新线程，栈上仅有 hang{test} 函数。
    let t = newthread(l);
    state_mut(t).get_field_str(LUA_GLOBALSINDEX, &name);

    CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_HANG);

    // resume 的出错状态即 cpp 中断计时器应产生的 ErrRun。
    let status = resume(t, None, 0);
    assert_eq!(status, LuaStatus::ErrRun as c_int);

    // `t` 出错后栈顶为错误字符串，读 -1 的字节切片（非串即抛）；
    // `cstr_text` 借用仍存活的 VM 栈缓冲并立即转 owned；`pop`
    // 回收父栈上的线程引用。
    let error = String::from_utf8_lossy(&l_checklstring(t, -1)).into_owned();
    assert!(
      error.contains("timeout"),
      "expected timeout error, got {error}"
    );

    pop(l, 1);
  }

  // 用例④（cpp `Interrupt` 的 hangpcall 块）：HANG_PCALL 模式每 1000 次中断抛一次
  // timeout 并重新计数，hangpcall 的百轮 `pcall(...)` 每轮都把它吞下，最终 resume 应 Ok。
  // 同用例①，`t` 为新建线程且全程存活；本线程的中断错误均由被测脚本的 pcall 消化。
  let t = newthread(l);
  state_mut(t).get_field_str(LUA_GLOBALSINDEX, "hangpcall");

  CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_HANG_PCALL);
  let status = resume(t, None, 0);
  assert_eq!(status, LuaStatus::Ok as c_int);

  pop(l, 1);
}

#[test]
fn conformance_interrupt_error_inspection() {
  use crate::common::{
    functions::{
      compile_and_load::compile_and_load,
      conformance_interrupt_error_inspection_interrupt::conformance_interrupt_error_inspection_interrupt,
      conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook,
      new_state::new_state, openlibs_and_sandbox::openlibs_and_sandbox,
      safe_api::{callhook, callbacks_mut, getinfo, resume, zero_debug},
    },
    records::conformance_interrupt_error_inspection_state::CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE,
  };

  let source = r#"
function fib(n)
    return n < 2 and 1 or fib(n - 1) + fib(n - 2)
end

fib(5)
"#;

  for target in 0..20 {
    CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE.reset(target);

    let global_state = new_state();
    let l = global_state.as_ptr();

    openlibs_and_sandbox(l);
    compile_and_load(l, source, "=InterruptErrorInspection", None);

    // 登记 interrupt 钩子后在主线程上 resume——钩子在第 `target` 次中断处
    // yield，模拟「错误传播途中做栈内省」的场景；resume 的 from 为无父线程哨兵。
    callbacks_mut(l).interrupt = Some(conformance_interrupt_error_inspection_interrupt);
    resume(l, None, 0);

    // `zero_debug` 交出全零 LuaDebug，getinfo 按 what 串负责填写；
    // callhook 的 ud 传 null 与钩子签名一致。
    let mut ar = zero_debug();
    assert_ne!(0, getinfo(l, 0, b"nsl\0", &mut ar));

    callhook(l, Some(conformance_interrupt_inspection_hook), None);
  }
}

#[test]
fn conformance_interrupt_inspection() {
  use crate::common::functions::{
    conformance_interrupt_inspection_setup::conformance_interrupt_inspection_setup,
    conformance_interrupt_inspection_yield::conformance_interrupt_inspection_yield,
    run_conformance::run_conformance,
  };

  run_conformance(
    "basic.luau",
    Some(conformance_interrupt_inspection_setup),
    Some(conformance_interrupt_inspection_yield),
    None,
    None,
    true,
    None,
  );
}

#[test]
fn conformance_n_debug_get_up_value() {
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::functions::{
    conformance_n_debug_get_up_value_yield::conformance_n_debug_get_up_value_yield,
    default_compile_options::default_compile_options, run_conformance::run_conformance,
  };

  let copts = CompileOptions {
    optimization_level: 0,
    debug_level: 0,
    ..default_compile_options()
  };

  run_conformance(
    "ndebug_upvalues.luau",
    None,
    Some(conformance_n_debug_get_up_value_yield),
    None,
    Some(&copts),
    false,
    None,
  );
}

#[test]
fn conformance_tag_method_error() {
  use core::sync::atomic::Ordering;

  use crate::common::{
    functions::{
      conformance_tag_method_error_setup::conformance_tag_method_error_setup,
      conformance_tag_method_error_yield::conformance_tag_method_error_yield,
      run_conformance::run_conformance,
    },
    records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE,
  };

  for lua_break in [false, true] {
    CONFORMANCE_TAG_METHOD_ERROR_STATE.reset(lua_break);

    run_conformance(
      "tmerror.luau",
      Some(conformance_tag_method_error_setup),
      Some(conformance_tag_method_error_yield),
      None,
      None,
      false,
      None,
    );
    assert_eq!(
      CONFORMANCE_TAG_METHOD_ERROR_STATE
        .index
        .load(Ordering::SeqCst),
      3
    );
  }
}
