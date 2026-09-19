use ulua::{Error, compile, eval};

#[test]
fn compile_returns_nonempty_bytecode() {
  let bytecode = compile("return 1 + 1").expect("compile should succeed");
  assert!(!bytecode.is_empty(), "bytecode must be non-empty");
}

#[test]
fn eval_runs_passing_assertion() {
  eval("assert(1 + 1 == 2)").expect("eval should succeed");
}

#[test]
fn eval_rejects_yield_without_reading_yielded_values() {
  for source in ["coroutine.yield()", "coroutine.yield('not an error')"] {
    let err = eval(source).expect_err("yield should be an error");
    let Error::RuntimeError(message) = &err else {
      panic!("expected RuntimeError, got {err:?}");
    };
    // Repl.cpp：yield 文案 + 无条件追加的 stack backtrace。
    assert!(
      message.starts_with("thread yielded unexpectedly"),
      "{message}"
    );
    assert!(message.contains("stack backtrace:"), "{message}");
  }
}

#[test]
fn eval_preserves_error_bytes_and_non_string_has_no_sentinel() {
  // 错误串里的嵌入 NUL 按长度保留（对 C++ strlen 截断的已知超集偏差）。
  let err = eval("error('a\\0b', 0)").expect_err("embedded-NUL error should surface");
  let Error::RuntimeError(message) = &err else {
    panic!("expected RuntimeError, got {err:?}");
  };
  assert!(message.contains("a\0b"), "{message:?}");

  // 非字符串错误与 Repl.cpp 一致：文本为空但仍有回溯，无哨兵串。
  let err = eval("error({}, 0)").expect_err("table error should surface");
  let Error::RuntimeError(message) = &err else {
    panic!("expected RuntimeError, got {err:?}");
  };
  assert!(
    message.starts_with("\nstack backtrace:"),
    "non-string error should carry only the backtrace: {message:?}"
  );

  eval("return 1, 'ok'").unwrap();
}

#[test]
fn eval_reports_compile_errors_without_running_source() {
  let source = "local x =";
  let eval_err = eval(source).expect_err("compile error should fail eval");
  let compile_err = compile(source).expect_err("compile error should be Err");
  assert!(matches!(eval_err, Error::SyntaxError { .. }));
  assert_eq!(eval_err.to_string(), compile_err.to_string());
}

#[test]
fn eval_reports_runtime_error() {
  let err = eval("error('boom')").expect_err("eval should fail");
  assert!(
    err.to_string().contains("boom"),
    "error message should mention boom: {err}"
  );
}

/// cpp `setupState`（Repl.cpp:230）在 openlibs 后调用 `luaL_sandbox`：库表与
/// 真 `_G` 上锁，运行中的脚本改不动标准库。
#[test]
fn eval_runs_in_a_sandboxed_state() {
  eval("assert(not pcall(function() math.bogus_field = 1 end), 'library table must be read-only')")
    .expect("sandbox check should pass");
}

/// `luaL_sandboxthread`（runRepl:566）给宿主 state 一张 __index 指向真 `_G` 的
/// 可写代理全局表：脚本能写新全局，但写入只活在代理表里，不污染已上锁的真 `_G`。
#[test]
fn eval_thread_writes_go_to_the_private_env() {
  eval("x = 1; assert(x == 1)").expect("thread-local global write should work");
  eval("assert(_G.x == nil)").expect("write must not leak into the real _G");
}
