// table / 排序 / 变参 / 函数调用 fixture 用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_calls() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("calls.luau");
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
