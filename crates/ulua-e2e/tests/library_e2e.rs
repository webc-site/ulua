//! Library-level end-to-end tests driving the `ulua` umbrella API directly:
//! `compile`/`eval` round-trips, value/error returns, sequential evals on fresh
//! states, a compile → load → run via the safe `ulua::rt` session API, and a
//! check that error strings are sane (the VM surfaces Lua errors as `Err`, not
//! panics).

use ulua::{Error, compile, eval, rt::Lua};
#[test]
fn compile_returns_nonempty_bytecode() {
  let bc = compile("return 2 + 2").expect("compile ok");
  assert!(!bc.is_empty());
  // 首字节必须是版本号而非 0/其他：cpp Bytecode.h:523 `LBC_VERSION_TARGET = 9`，
  // 经 `set_luau_bool_flags` 开启 `LuauCompileFastpcall` 时为 14（同 cli_integration.rs:278）；
  // 进程内并发执行的 `eval` 用例会触发全局旗标初始化，故允许 9 与 14。
  let version = *bc.first().expect("bytecode has version byte");
  assert!(
    version == 9 || version == 14,
    "bytecode must start with a valid Luau target version (9 or 14), got {version}"
  );
}

#[test]
fn compile_reports_syntax_error_as_err() {
  let err = compile("local = = =").expect_err("syntax error should be Err");
  let Error::SyntaxError { message, .. } = err else {
    panic!("expected SyntaxError, got {err:?}");
  };
  // 全等锁定：cpp Compiler.cpp:5821 错误 blob 正文 `":%d: %s"` 前缀 +
  // Parser.cpp:5428 正文（context=variable name，同 tests/Parser.test.cpp:917
  // error_on_unicode 格式；'=' 为 Lexer "'%c'" 形态）。
  // 原 `!message.is_empty()` 挡不住任意非空文案。
  assert_eq!(
    message,
    ":1: Expected identifier when parsing variable name, got '='"
  );
}

#[test]
fn eval_runs_passing_assertion() {
  eval("assert(1 + 1 == 2)").expect("eval ok");
}

#[test]
fn eval_reports_runtime_error_message() {
  let err = eval("error('boom-from-lib')").expect_err("runtime error should be Err");
  let Error::RuntimeError(message) = &err else {
    panic!("expected RuntimeError, got {err:?}");
  };
  assert!(
    message.contains("boom-from-lib"),
    "error should mention boom: {message}"
  );
  // 与 Repl.cpp runCode 一致：错误文本后追加 lua_debugtrace 回溯。
  assert!(
    message.contains("stack backtrace:"),
    "error should carry a stack backtrace: {message}"
  );
}

#[test]
fn eval_reports_assertion_failure() {
  // cpp lbaselib.cpp:261 luaB_assert 用 `luaL_error(L, "%s", optstring)`：
  // 带消息的失败断言以消息原文（含 luaL_where 前缀）浮出，默认文案
  // "assertion failed!" 被整体替换——只盯 contains("nope") 挡不住
  // "assertion failed: nope" 这类混排实现。
  let err = eval("assert(false, 'nope')").expect_err("failed assert should be Err");
  let Error::RuntimeError(message) = &err else {
    panic!("expected RuntimeError, got {err:?}");
  };
  assert!(
    message.lines().next().is_some_and(|l| l.ends_with("nope")),
    "assert message should end with the supplied text verbatim: {message}"
  );
  assert!(
    !message.contains("assertion failed"),
    "custom message must replace, not decorate, the default: {message}"
  );
}

#[test]
fn eval_reports_nil_index_error() {
  // 与 CLI 路径（edge_cases::runtime_error_mid_execution_is_clean）同一 oracle：
  // cpp luaG_typeerror 的 "attempt to index nil with 'x'"，只测非空挡不住错报形态
  let err = eval("local t = nil; return t.x").expect_err("indexing nil should be Err");
  let Error::RuntimeError(message) = &err else {
    panic!("expected RuntimeError, got {err:?}");
  };
  assert!(
    message.contains("attempt to index nil"),
    "nil-index error should name the offense: {message}"
  );
}

#[test]
fn multiple_sequential_evals_each_fresh_state() {
  // Each eval opens a brand-new lua_State; state must not leak across calls.
  eval("x = 1; assert(x == 1)").expect("first eval ok");
  // A fresh state means `x` is no longer defined here (it's nil); reading a
  // global nil is fine, but asserting it equals 1 must now fail.
  eval("assert(x == nil)").expect("second eval sees a fresh global table");
  eval("assert(2 * 21 == 42)").expect("third eval ok");
}

#[test]
fn compile_then_load_and_run_via_vm() {
  // Mirror what `eval` does internally, but drive the safe `ulua::rt` session
  // API (`Lua` handle + `Chunk::exec`) to confirm compiled bytecode loads and
  // runs on a freshly-built state — no hand-rolled newstate/openlibs/newthread/
  // resume dance or raw pointers at the test level.

  let bytecode = compile("assert(3 + 4 == 7)").expect("compile ok");
  // 不调 set_luau_bool_flags：其文档契约为「Call before threads start」，
  // 本用例无需旗标配置——chunk 加载/执行不依赖 Luau* 旗标取值。

  let lua = Lua::new();
  let status = lua.load_bytecode(&bytecode).set_name("=libtest").exec();
  assert!(
    status.is_ok(),
    "script should run to completion: {status:?}"
  );
}

#[test]
fn compile_handles_unicode_and_long_strings() {
  // 字节长度同 edge_cases 的 unicode 用例（18 字节）
  eval("local s = 'héllo wörld 🦀'; assert(#s == 18)").expect("unicode source runs");
  let long = format!("return '{}'", "a".repeat(10_000));
  let bc = compile(&long).expect("long string literal compiles");
  // 字面量原文进常量池：产物必然长于字面量本身，短了就是没嵌进去
  assert!(
    bc.len() > 10_000,
    "bytecode must embed the 10k literal, got {} bytes",
    bc.len()
  );
}
