//! Port of `cpp/tests/TypeInfer.metatableOOP.test.cpp` 的增量用例（与
//! `type_infer_oop.rs` 合计覆盖该 cpp 文件的其余 TEST_CASE）。
//!
//! 跳过（未移植）7 例：`subclass_property_access`、`setmetatable_uses_expected_type_for_fresh_table_arguments`、
//! `setmetatable_uses_expected_type_in_call_and_return_contexts`、`setmetatable_expected_type_rejects_invalid_fresh_table_values`、
//! `setmetatable_expected_type_is_pushed_into_nested_lambdas`、`setmetatable_overrides_1/2` ——
//! 依赖 Rust 端未同步的 `FFlag::LuauSetmetatableOverrides` / `FFlag::LuauBidirectionalInferenceSetMetatable`
//! 行为（Rust 报 TypeMismatch 于整个 setmetatable 调用 span，而非期望类型驱动的精确诊断）。

// Source: `tests/TypeInfer.metatableOOP.test.cpp:933-949`
#[test]
fn type_infer_metatable_oop_setmetatable_expected_type_does_not_widen_aliased_tables() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_unit_test::{
    DOES_NOT_PASS_OLD_SOLVER_GUARD, functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture,
  };

  DOES_NOT_PASS_OLD_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type DateTime = { date: number }
        type A = setmetatable<{}, { test: DateTime? }>

        local mt = { test = nil }
        local x: A = setmetatable({}, mt)
    "#,
    None,
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

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type DateTime = { date: number }
        type A = setmetatable<{}, { test: DateTime? }>

        local x: A = setmetatable({}, { test = nil })
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<TypeMismatch>(&result.errors[0]).is_some());
}
