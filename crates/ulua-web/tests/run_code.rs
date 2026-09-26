//! `run_code`（cpp `CLI/src/Web.cpp:71-140`）的集成测试。
//!
//! `run_code` 是 `executeScript` / wasm `run` 共用的跑码单元，故这里锁两件事：
//! 返回文本的具体内容（对齐 cpp 的装配规则）与「每次调用后父栈回到调用前的
//! `lua_gettop`」——后者是 cpp 靠 `lua_pop(L, 1)` 手工维持的不变量
//! （`Web.cpp:109`、`Web.cpp:137`）。

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_l_sandbox::lua_l_sandbox,
  },
  records::{lua_state::LuaState, lua_state_guard::LuaStateGuard},
};
use ulua_web::functions::run_code::run_code;

/// 建一个已装配（openlibs + sandbox）的沙箱状态，`Drop` 时 `lua_close`。
///
/// 对应 cpp `Web.cpp:64-69` 的 `static void setupState`——它在 cpp 是文件内静态
/// 函数，故此处直接调 VM 的两步，而不为此造一个 crate 公共 API。
/// `luaL_newstate` 在内存耗尽时返回 null，而 openlibs/sandbox 的前提是
/// 「valid, non-null」，故先判空（cpp 把可能为 null 的状态直接交给 setupState，
/// 属其宿主隐含前提，不移植）。
fn sandboxed_state() -> LuaStateGuard {
  let guard = LuaStateGuard(lua_l_newstate());
  assert!(!guard.0.is_null(), "lua_l_newstate 失败（内存耗尽）");
  // Safety: 上一行已保证 guard.0 是刚创建、未别处持有的有效状态。
  unsafe {
    lua_l_openlibs(guard.0);
    lua_l_sandbox(guard.0);
  }
  guard
}

/// 各类脚本（编译错误、运行错误、意外 yield、成功）的返回文本与父栈恢复。
///
/// 期望文本按 cpp 出处逐级锁定（原为 `contains(短子串)` 弱断言）：
/// - 加载失败：`runCode` 直接返回 `luau_load` 压栈的错误消息（cpp
///   Web.cpp:83-90）；lvmload.cpp:291-298 对版本 0 错误 blob 组装
///   `"%s%.*s"`＝chunkid（`"=stdin"` 经 luaO_chunkid 化简为 `stdin`）＋
///   blob 正文（`":%d: %s"` 格式，Compiler.cpp:5821），故全文以
///   `stdin:1: ` 开头；正文出处见 Parser.test.cpp:917 同格式用例、
///   Parser.cpp:788（break）。
/// - 运行失败：cpp Web.cpp:120-135 组装「错误消息或 yield 文案 →
///   `"\nstack backtrace:\n"` → lua_debugtrace」，两段逐字锁全。
#[test]
fn repeated_runs_restore_the_parent_stack() {
  let state = sandboxed_state();
  let l: *mut LuaState = state.0;

  for (source, expected) in [
    (
      "local x =",
      "stdin:1: Expected identifier when parsing expression, got <eof>",
    ),
    ("break", "stdin:1: break statement must be inside a loop"),
    (
      "error('boom')",
      "stdin:1: boom\nstack backtrace:\n[C] function error\nstdin:1\n",
    ),
    (
      "coroutine.yield()",
      "thread yielded unexpectedly\nstack backtrace:\n[C] function yield\nstdin:1\n",
    ),
    ("return", ""),
    ("return 1, 2", ""),
    ("print('hi')", ""),
  ] {
    // Safety: `l` 由 `sandboxed_state` 建立且本用例独占；gettop 只读栈高。
    let top = unsafe { lua_gettop(l) };
    // Safety: 同上，`run_code` 的前置（openlibs + sandbox）由 fixture 成立。
    assert_eq!(unsafe { run_code(l, source) }, expected, "{source}");
    // Safety: 同上。
    assert_eq!(unsafe { lua_gettop(l) }, top, "{source} 结束后父栈须复原");
  }
}

/// 运行期错误：`消息 + "\nstack backtrace:\n" + 回溯` 两段全文锁定
/// （`Web.cpp:129-135`：`lua_tostring(T, -1)` 与 `lua_debugtrace(T)`）。
#[test]
fn runtime_error_has_message_and_backtrace() {
  let state = sandboxed_state();
  // Safety: fixture 保证状态有效且本用例独占。
  let result = unsafe { run_code(state.0, "error('boom')") };
  // cpp Web.cpp:128-135：错误消息体在前、`\nstack backtrace:\n` 紧随、其后为
  // lua_debugtrace；消息体自带 luaB_error/luaL_where 的 `stdin:1: ` 位置前缀。
  // 原两个独立 `contains` 挡不住「消息与回溯顺序颠倒/夹带装饰」，这里两段全锁。
  let (message, backtrace) = result
    .split_once("\nstack backtrace:\n")
    .unwrap_or(("<缺栈回溯段>", ""));
  assert_eq!(message, "stdin:1: boom", "got: {result}");
  assert_eq!(backtrace, "[C] function error\nstdin:1\n", "got: {result}");
  // Safety: 同上；线程出栈后父栈应为空。
  assert_eq!(unsafe { lua_gettop(state.0) }, 0, "线程出栈后父栈应清空");
}

/// 返回值走 `Web.cpp:99-107` 的 print 分支而非错误文本：返回空串且父栈平衡。
#[test]
fn return_values_are_printed_not_reported() {
  let state = sandboxed_state();
  // Safety: fixture 保证状态有效且本用例独占。
  assert_eq!(unsafe { run_code(state.0, "return 1, 2") }, "");
  // Safety: 同上。
  assert_eq!(unsafe { lua_gettop(state.0) }, 0);
}
