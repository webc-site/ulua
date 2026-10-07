// table / 排序 / 变参 / 函数调用 fixture 用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_calls() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("calls.luau");
}

/// 本仓新增（cpp/tests/conformance 无对应源）：JIT call_fallback 慢路径定向用例。
/// 断言以 ./cpp oracle 行为为准（`cpp/CodeGen/src/CodeGenUtils.cpp:301`
/// `callFallback`），已在 cpp 侧解释器与原生两种模式下逐条核对；fixture 内全部
/// 经值中转的调用在字节码里都是普通 CALL，LUAU_CODEGEN=1 下原生代码每条 CALL
/// 都经 call_fallback 建帧，保证慢路径被真实执行。
#[test]
fn conformance_calls_fallback() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("calls_fallback.luau");
}

#[test]
fn conformance_sort() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("sort.luau");
}

#[test]
fn conformance_tables() {
  use crate::common::functions::{
    conformance_tables_setup::conformance_tables_setup, run_conformance::run_fixture_setup,
  };

  run_fixture_setup("tables.luau", conformance_tables_setup);
}

#[test]
fn conformance_var_arg() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("vararg.luau");
}
