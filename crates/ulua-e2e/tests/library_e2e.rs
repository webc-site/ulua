//! Library-level end-to-end tests driving the `ulua` umbrella API directly:
//! `compile`/`eval` round-trips, value/error returns, sequential evals on fresh
//! states, a hand-driven compile → load → run via `ulua::vm`, and a check that
//! error strings are sane (the VM surfaces Lua errors as `Err`, not panics).

use core::{ffi::c_char, ptr::null_mut};

use ulua::{common::set_all_flags, compile, eval};
#[test]
fn compile_returns_nonempty_bytecode() {
  let bc = compile("return 2 + 2").expect("compile ok");
  assert!(!bc.is_empty());
  // Valid bytecode does not begin with the \0 error marker.
  assert_ne!(bc.first(), Some(&0u8), "bytecode must not be an error blob");
}

#[test]
fn compile_reports_syntax_error_as_err() {
  let err = compile("local = = =").expect_err("syntax error should be Err");
  assert!(!err.is_empty(), "error message should be non-empty");
}

#[test]
fn eval_runs_passing_assertion() {
  eval("assert(1 + 1 == 2)").expect("eval ok");
}

#[test]
fn eval_reports_runtime_error_message() {
  let err = eval("error('boom-from-lib')").expect_err("runtime error should be Err");
  assert!(
    err.contains("boom-from-lib"),
    "error should mention boom: {err}"
  );
}

#[test]
fn eval_reports_assertion_failure() {
  let err = eval("assert(false, 'nope')").expect_err("failed assert should be Err");
  assert!(
    err.contains("nope"),
    "error should carry the assert message: {err}"
  );
}

#[test]
fn eval_reports_nil_index_error() {
  let err = eval("local t = nil; return t.x").expect_err("indexing nil should be Err");
  assert!(
    !err.is_empty(),
    "nil-index error should be non-empty: {err}"
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
  // Mirror what `eval` does internally, but drive the raw `ulua::vm` API to
  // confirm compiled bytecode loads and runs on a freshly-built state.
  use ulua::vm::functions::{
    lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_newthread::lua_newthread, lua_resume::lua_resume, luau_load::luau_load,
  };

  let bytecode = compile("assert(3 + 4 == 7)").expect("compile ok");
  set_all_flags(true);

  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate returned null");
    lua_l_openlibs(l);
    let t = lua_newthread(l);
    assert!(!t.is_null(), "lua_newthread returned null");

    let rc = luau_load(
      t,
      c"=libtest".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    );
    assert_eq!(rc, 0, "luau_load should succeed on valid bytecode");

    let status = lua_resume(t, null_mut(), 0);
    assert_eq!(status, 0, "script should run to completion (status 0)");

    lua_close(l);
  }
}

#[test]
fn eval_does_not_panic_on_diverse_errors() {
  // A battery of error-shaped programs must all come back as Err strings, with
  // no Rust panic escaping (the e2e crate would abort the test process).
  let cases = [
    "error()",
    "error({code = 1})",         // non-string error object
    "(nil)()",                   // call a nil value
    "return 1 + {}",             // arithmetic on a table
    "local t = {}; return #t.x", // length of nil field
    "string.rep('x', -1)",       // odd-but-defined stdlib call
  ];
  for src in cases {
    let result = eval(src);
    // We don't assert Ok/Err per case (some are valid) — only that the call
    // returned a Result rather than unwinding the test harness.
    let _ = result;
  }
}

#[test]
fn compile_handles_unicode_and_long_strings() {
  compile("local s = 'héllo wörld 🦀'; return #s").expect("unicode source compiles");
  let long = format!("return '{}'", "a".repeat(10_000));
  compile(&long).expect("long string literal compiles");
}
