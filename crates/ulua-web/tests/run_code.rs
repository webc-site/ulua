//! `run_code`（cpp `CLI/src/Web.cpp:71-140`）的集成测试。

use ulua_vm::functions::{
  lua_close::lua_close, lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
};
use ulua_web::functions::{run_code::run_code, setup_state::setup_state};

/// 各类脚本（编译错误、运行错误、yield、成功）返回码正确且不污染父栈。
#[test]
fn repeated_runs_restore_the_parent_stack() {
  // SAFETY: l 是刚创建的合法 lua_State。
  unsafe {
    let l = lua_l_newstate();
    setup_state(l);
    for (source, message) in [
      ("local x =", ":1:"),
      ("break", "break statement must be inside a loop"),
      ("error('boom')", "boom"),
      ("coroutine.yield()", "thread yielded unexpectedly"),
      ("return", ""),
    ] {
      let top = lua_gettop(l);
      let result = run_code(l, source);
      if message.is_empty() {
        assert!(result.is_empty(), "{result}");
      } else {
        assert!(result.contains(message), "{source}: {result}");
      }
      assert_eq!(lua_gettop(l), top, "{source}");
    }
    lua_close(l);
  }
}

/// 运行期错误：返回错误信息 + 栈回溯。
#[test]
fn runtime_error_has_message_and_backtrace() {
  // SAFETY: l 是刚创建的合法 lua_State。
  unsafe {
    let l = lua_l_newstate();
    setup_state(l);
    let result = run_code(l, "error('boom')");
    assert!(result.contains("boom"), "got: {result}");
    assert!(result.contains("stack backtrace:"), "got: {result}");
    assert_eq!(lua_gettop(l), 0, "线程出栈后父栈应清空");
    lua_close(l);
  }
}

/// 返回值走 `print` 分支而非错误文本：`run_code` 返回空串且父栈平衡。
#[test]
fn return_values_are_printed_not_reported() {
  // SAFETY: l 是刚创建的合法 lua_State。
  unsafe {
    let l = lua_l_newstate();
    setup_state(l);
    let result = run_code(l, "return 1, 2");
    assert_eq!(result, "");
    assert_eq!(lua_gettop(l), 0);
    lua_close(l);
  }
}
