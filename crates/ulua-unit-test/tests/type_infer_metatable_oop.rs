//! Port of `cpp/tests/TypeInfer.metatableOOP.test.cpp` 的增量用例（与
//! `type_infer_oop.rs` 合计覆盖该 cpp 文件的其余 TEST_CASE）。
//!
//! 跳过（未移植）7 例：`subclass_property_access`、`setmetatable_uses_expected_type_for_fresh_table_arguments`、
//! `setmetatable_uses_expected_type_in_call_and_return_contexts`、`setmetatable_expected_type_rejects_invalid_fresh_table_values`、
//! `setmetatable_expected_type_is_pushed_into_nested_lambdas`、`setmetatable_overrides_1/2` ——
//! 依赖 Rust 端未同步的 `FFlag::LuauSetmetatableOverrides` / `FFlag::LuauBidirectionalInferenceSetMetatable`
//! 行为（Rust 报 TypeMismatch 于整个 setmetatable 调用 span，而非期望类型驱动的精确诊断）。
//!
//! tw-10 实测复核（7 例逐一探针，维持挂账）：
//! * 前两例 uses_expected_type_* 与 subclass_property_access：Rust 误报 TypeMismatch/UnknownProperty
//!   （如 fresh_table_arguments 在整调用 span {4,21}-{4,66} 报 1 错，cpp 0 错）——期望类型
//!   双向推导未随旗标同步进产品端。
//! * rejects_invalid_fresh_table_values：错误数与类型（1×TypeMismatch）一致，但 span 为
//!   整调用 {4,21}-{4,52}，cpp 为精确诊断 {4,47}-{4,49}（即 `42` 字面量）。
//! * is_pushed_into_nested_lambdas：0 错一致，但 `findExpectedTypeAtPosition({5,23})` 在
//!   Rust 返回 None，期望类型未传播进嵌套 lambda 体内。
//! * overrides_1/2：重复 setmetatable 的 metatable 历史合并（`setmetatable<root, mt2>` 型
//!   可打印别名）未实现，Rust 对 propB 也报 UnknownProperty（cpp 仅 propA 报 1 错）。
// 集成测试共享样板：fixture 前奏宏 fx_check!/bs_check! 由 common 上提，#[macro_export] 后按裸名调用。
mod common;

// Source: `tests/TypeInfer.metatableOOP.test.cpp:933-949`
#[test]
fn type_infer_metatable_oop_setmetatable_expected_type_does_not_widen_aliased_tables() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_unit_test::{
    DOES_NOT_PASS_OLD_SOLVER_GUARD, functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture,
  };

  DOES_NOT_PASS_OLD_SOLVER_GUARD!();

  let (_fixture, result) = bs_check!(
    r#"
        type DateTime = { date: number }
        type A = setmetatable<{}, { test: DateTime? }>

        local mt = { test = nil }
        local x: A = setmetatable({}, mt)
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<TypeMismatch>(&result.errors[0]).is_some());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp:991-1006`
//
// cpp 断言 `LuauBidirectionalInferenceSetMetatable = false` 时回退为
// TypeMismatch；Rust 端无该旗标（行为即旗标关闭语义），断言一致故保留。
#[test]
fn type_infer_metatable_oop_setmetatable_expected_type_is_unchanged_when_flag_is_disabled() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_unit_test::{
    DOES_NOT_PASS_OLD_SOLVER_GUARD, functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture,
  };

  DOES_NOT_PASS_OLD_SOLVER_GUARD!();

  let (_fixture, result) = bs_check!(
    r#"
        type DateTime = { date: number }
        type A = setmetatable<{}, { test: DateTime? }>

        local x: A = setmetatable({}, { test = nil })
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<TypeMismatch>(&result.errors[0]).is_some());
}
