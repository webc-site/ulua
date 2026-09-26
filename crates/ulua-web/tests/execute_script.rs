//! `execute_script`（cpp `CLI/src/Web.cpp:184-208` 的 `executeScript`）的集成测试。
//!
//! 期望文本对齐 cpp oracle 的错误装配（`Web.cpp:114-138`）；`None` 对应
//! `Web.cpp:207` 的 `result.empty() ? nullptr : result.c_str()`。

use ulua_web::functions::execute_script::execute_script;

/// 成功脚本（含 `print` 输出与返回值打印）无错误文本 → `None`。
#[test]
fn successful_runs_return_none() {
  for source in ["local x = 1", "print('hi')", "return 1, 2", ""] {
    assert_eq!(execute_script(source), None, "{source:?} 应无错误文本");
  }
}

/// 运行期错误：`消息 + "\nstack backtrace:\n" + 回溯` 两段全文锁定
/// （`Web.cpp:129-135`：`lua_tostring(T, -1)` 与 `lua_debugtrace(T)`）。
#[test]
fn runtime_error_locks_message_and_backtrace() {
  let result = execute_script("error('boom')").expect("运行期错误应有文本");
  let (message, backtrace) = result
    .split_once("\nstack backtrace:\n")
    .unwrap_or(("<缺栈回溯段>", ""));
  // 消息自带的 `stdin:1: ` 由 VM 的位置前缀给出（`Web.cpp:129`）；
  // `Web.cpp:117-123` 的 getinfo 前缀在 level 0 无活跃帧时恒为空串。
  assert_eq!(message, "stdin:1: boom", "got: {result}");
  assert_eq!(backtrace, "[C] function error\nstdin:1\n", "got: {result}");
}

/// 意外 yield 走 `LUA_YIELD` 分支：固定文案，不取栈顶消息（`Web.cpp:125-127`）。
#[test]
fn unexpected_yield_locks_message() {
  assert_eq!(
    execute_script("coroutine.yield()").expect("意外 yield 应有文本"),
    "thread yielded unexpectedly\nstack backtrace:\n[C] function yield\nstdin:1\n"
  );
}

/// 编译/加载失败：直接回栈上原始消息，不带 web 层前缀（`Web.cpp:78-87`）。
#[test]
fn load_errors_lock_message_and_line() {
  for (source, expected) in [
    (
      "local x =",
      "stdin:1: Expected identifier when parsing expression, got <eof>",
    ),
    ("break", "stdin:1: break statement must be inside a loop"),
  ] {
    assert_eq!(
      execute_script(source).expect("编译错误应有文本"),
      expected,
      "{source:?}"
    );
  }
}

/// 运行期错误的行号取自实际出错行（锁定行号不被偏移）。
#[test]
fn runtime_error_reports_the_offending_line() {
  let result = execute_script("local t = {}\nt.a.b = 1").expect("运行期错误应有文本");
  let (message, backtrace) = result
    .split_once("\nstack backtrace:\n")
    .expect("运行期错误应带栈回溯段");
  assert_eq!(message, "stdin:2: attempt to index nil with 'b'");
  assert_eq!(backtrace, "stdin:2\n");
}

/// 逐调用无残留：上一次的错误文本不得影响下一次调用（cpp 靠函数内
/// `static std::string` 的整体赋值覆盖，`Web.cpp:205`）。
#[test]
fn stateless_between_calls() {
  assert!(execute_script("error('first')").is_some());
  assert_eq!(execute_script("local x = 1"), None, "残留前一次结果");
}
