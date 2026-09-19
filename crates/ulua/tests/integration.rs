use ulua::{compile, eval};

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
    assert_eq!(eval(source).unwrap_err(), "thread yielded unexpectedly");
  }
}

#[test]
fn eval_preserves_error_bytes_and_non_string_fallback() {
  assert_eq!(eval("error('a\\0b', 0)").unwrap_err(), "a\0b");
  assert_eq!(eval("error({}, 0)").unwrap_err(), "<non-string error>");
  eval("return 1, 'ok'").unwrap();
}

#[test]
fn eval_reports_compile_errors_without_running_source() {
  let source = "local x =";
  assert_eq!(eval(source), compile(source).map(|_| ()));
}

#[test]
fn eval_reports_runtime_error() {
  let err = eval("error('boom')").expect_err("eval should fail");
  assert!(
    err.contains("boom"),
    "error message should mention boom: {err}"
  );
}
