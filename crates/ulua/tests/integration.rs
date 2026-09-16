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
fn eval_reports_runtime_error() {
  let err = eval("error('boom')").expect_err("eval should fail");
  assert!(
    err.contains("boom"),
    "error message should mention boom: {err}"
  );
}
