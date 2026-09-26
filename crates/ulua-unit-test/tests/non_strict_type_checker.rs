extern crate alloc;

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_binop_is_checked() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::{
    functions::require_checked_function_call_error::require_checked_function_call_error,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
        local foo = 4 + abs("foo")
    "#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_checked_function_call_error(&result, 0, "abs", "number", "string");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_buffer_is_not_unknown() {
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
local function wrap(b: buffer, i: number, v: number)
    buffer.writeu32(b, i * 4, v)
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_exprgroup_is_checked() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::{
    functions::require_checked_function_call_error::require_checked_function_call_error,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
        local foo = (abs("foo"))
    "#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_checked_function_call_error(&result, 0, "abs", "number", "string");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_fn_expr_produces_error() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 5
local y = function() lower(x) end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 27), "lower");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_function_def_basic_errors() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function f(x : string)
    abs(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 8), "abs");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_function_def_basic_no_errors() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
function f(x)
    abs(x)
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_failure() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function f(x)
    abs(lower(x))
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 8), "abs");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_if_assignment_errors() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function f(x)
    if cond() then
        x = 5
    else
        x = nil
    end
    lower(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(7, 10), "lower");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_function_def_if_assignment_no_errors() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
function f(x : string | number)
    if cond() then
        x = 5
    else
        x = "hi"
    end
    abs(x)
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_function_def_if_no_else() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
function f(x)
    if cond() then
        abs(x)
    end
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_if_warns_never() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function f(x: never)
    if cond() then
        abs(x)
    else
        lower(x)
    end
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_sequencing_errors() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_function_definition_error_at::require_non_strict_function_definition_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function f(x)
    abs(x)
    lower(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_function_definition_error_at(&result, Position::new(1, 11), "x");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_sequencing_errors_2() {
  use ulua_analysis::{
    functions::to_string_error::to_string_type_error, records::check_result::CheckResult,
  };
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local t = {function(x)
    abs(x)
    lower(x)
end}
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "the argument 'x' is used in a way that will error at runtime",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_function_def_unrelated_checked_calls() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
function h(x, y)
    abs(x)
    lower(y)
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_generic_type_instantiation() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::check_result::CheckResult,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _semantics = ScopedFastFlag::new(&fflag::LuauExplicitTypeInstantiationSupport, true);
  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
        function array<T>(): {T}
            return {}
        end

        local foo = array<<number>>()
        local bar = array<<string>>()
    "#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "{number}",
    to_string_type_id(fixture.base.require_type_string("foo"))
  );
  assert_eq!(
    "{string}",
    to_string_type_id(fixture.base.require_type_string("bar"))
  );
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_generic_type_packs_in_non_strict() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
--!nonstrict
local test: <T...>(T...) -> () -- TypeError: Unknown type 'T'
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_does_not_warn_with_never_local() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : never
if cond() then
    abs(x)
else
    lower(x)
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_doesnt_warn_else_branch() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : string = "hi"
if cond() then
    abs(x)
else
    lower(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_expr_doesnt_warn_else_branch() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : string = "hi"
local y = if cond() then abs(x) else lower(x)
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 29), "abs");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_expr_should_not_warn_for_never() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : never
local y = if cond() then abs(x) else lower(x)
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_expr_should_warn() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 42
local y = if cond() then abs(x) else lower(x)
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 43), "lower");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_else_warns_nil_branches() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x
if cond() then
    abs(x)
else
    lower(x)
end
"#,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
  require_non_strict_checked_error_at(&result, Position::new(5, 10), "lower");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_if_then_no_else() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : string
if cond() then
    abs(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_if_then_no_else_err_in_cond() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x : string = ""
if abs(x) then
    lower(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 7), "abs");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_incomplete_function_annotation() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: () ->
    "#,
    None,
  );

  assert!(!result.errors.is_empty());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_incorrect_arg_count() {
  use ulua_analysis::records::{
    check_result::CheckResult, checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
foo.bar(1,2,3)
abs(3, "hi");
"#,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[0])
    .expect("expected CheckedFunctionIncorrectArgs");
  let r2 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[1])
    .expect("expected CheckedFunctionIncorrectArgs");

  assert_eq!("abs", r1.function_name());
  assert_eq!("foo.bar", r2.function_name());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_interesting_checked_functions() {
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
onlyNums(1,1,1)
onlyNums(1, "a")

mixedArgs("a", 1, 2)
mixedArgs(1, 1, 1)
mixedArgs("a", true)

optionalArg(nil)
optionalArg("a")
optionalArg(3)
"#,
  );

  assert_eq!(4, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 12), "onlyNums");
  require_non_strict_checked_error_at(&result, Position::new(5, 10), "mixedArgs");
  require_non_strict_checked_error_at(&result, Position::new(6, 15), "mixedArgs");
  require_non_strict_checked_error_at(&result, Position::new(10, 12), "optionalArg");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_local_fn_produces_error() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 5
local function y() lower(x) end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 25), "lower");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_local_only_one_warning() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 5
lower(x)
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 6), "lower");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_nested_function_calls_constant() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x
abs(lower(x))
"#,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 4), "abs");
  require_non_strict_checked_error_at(&result, Position::new(2, 10), "lower");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_new_non_strict_should_suppress_dynamic_require_errors() {
  use ulua_analysis::records::unknown_require::UnknownRequire;
  use ulua_ast::enums::mode::Mode;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff_debug_luau_force_old_solver =
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  // Nonstrict mode should suppress dynamic require errors
  let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
function passThrough(module)
    require(module)
end
    "#,
    None,
  );

  assert_eq!(
    0,
    result_nonstrict.errors.len(),
    "{:?}",
    result_nonstrict.errors
  );

  // Strict mode should still warn about dynamic requires
  let result_strict = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Strict,
    r#"
function passThrough(module)
    require(module)
end
    "#,
    None,
  );

  assert_eq!(1, result_strict.errors.len(), "{:?}", result_strict.errors);
  assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_new_non_strict_should_suppress_unknown_require_errors() {
  use ulua_analysis::records::unknown_require::UnknownRequire;
  use ulua_ast::enums::mode::Mode;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  // Nonstrict mode: should suppress unknown require errors
  let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
    None,
  );

  assert_eq!(
    0,
    result_nonstrict.errors.len(),
    "{:?}",
    result_nonstrict.errors
  );

  // Strict mode: should report unknown require errors
  let result_strict = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Strict,
    r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
    None,
  );

  assert_eq!(2, result_strict.errors.len(), "{:?}", result_strict.errors);
  assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
  assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[1]).is_some());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_new_non_strict_skips_warnings_on_unreduced_typefunctions() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
function foo(x)
    local y = x + 1
    return abs(y)
end
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed() {
  use ulua_analysis::{
    functions::to_string_error::to_string_type_error,
    records::{check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError},
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
getAllTheArgsWrong(3, true, "what")
"#,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[0]).is_some());
  assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[1]).is_some());
  assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[2]).is_some());
  assert_eq!(
    "the function 'getAllTheArgsWrong' expects to get a string as its 1st argument, but is being given a number",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "the function 'getAllTheArgsWrong' expects to get a number as its 2nd argument, but is being given a boolean",
    to_string_type_error(&result.errors[1])
  );
  assert_eq!(
    "the function 'getAllTheArgsWrong' expects to get a boolean as its 3rd argument, but is being given a string",
    to_string_type_error(&result.errors[2])
  );
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_non_strict_shouldnt_warn_on_require_module() {
  use alloc::string::String;

  use ulua_analysis::enums::type_file_resolver::Type;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  fixture.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
--!strict
type t = {x : number}
local e : t = {x = 3}
return e
"#,
    ),
  );
  fixture
    .base
    .file_resolver
    .source_types
    .insert(String::from("Modules/A").into(), Type::Module);

  fixture.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
--!nonstrict
local E = require(script.Parent.A)
"#,
    ),
  );

  let result = fixture.check_non_strict_module("Modules/B");

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_non_testable_type_throws_ice() {
  use std::panic::{AssertUnwindSafe, catch_unwind};

  use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _force_old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = catch_unwind(AssertUnwindSafe(|| {
    fixture.check_non_strict(
      r#"os.time({year = 0, month = 0, day = 0, min = 0, isdst = nil})
"#,
    );
  }));

  let payload = result.expect_err("expected InternalCompilerError");
  assert!(
    payload.is::<InternalCompilerError>(),
    "expected InternalCompilerError panic payload"
  );
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_nonstrict_check_block_recursion_limit() {
  use ulua_common::{dfint, fflag, fint};
  use ulua_unit_test::{
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let limit: usize = 250;

  let _sff = ScopedFastFlag::new(&fflag::LuauAddRecursionCounterToNonStrictTypeChecker, true);

  let _luau_non_strict_type_checker_recursion_limit = ScopedFastInt::new(
    &fint::LuauNonStrictTypeCheckerRecursionLimit,
    limit as i32 - 100,
  );
  let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
    &dfint::LuauConstraintGeneratorRecursionLimit,
    limit as i32 + 500,
  );
  let _luau_check_recursion_limit =
    ScopedFastInt::new(&fint::LuauCheckRecursionLimit, limit as i32 + 500);

  let mut fixture = NonStrictTypeCheckerFixture::default();
  let code = "do ".repeat(limit) + "local a = 1" + &" end".repeat(limit);
  let result = fixture.check_non_strict(&code);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_nonstrict_check_expr_recursion_limit() {
  use alloc::string::String;

  use ulua_common::{dfint, fflag, fint};
  use ulua_unit_test::{
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let limit: usize = 250;

  let _sff = ScopedFastFlag::new(&fflag::LuauAddRecursionCounterToNonStrictTypeChecker, true);

  let _luau_non_strict_type_checker_recursion_limit = ScopedFastInt::new(
    &fint::LuauNonStrictTypeCheckerRecursionLimit,
    limit as i32 - 100,
  );
  let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
    &dfint::LuauConstraintGeneratorRecursionLimit,
    limit as i32 + 500,
  );
  let _luau_check_recursion_limit =
    ScopedFastInt::new(&fint::LuauCheckRecursionLimit, limit as i32 + 500);

  let mut fixture = NonStrictTypeCheckerFixture::default();
  let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
  let result = fixture.check_non_strict(&code);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_nonstrict_method_calls() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result: CheckResult = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        local test = "test"
        test:lower()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_nonstrict_shouldnt_warn_on_valid_buffer_use() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    fixture::Fixture, non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture {
    base: Fixture::fixture_bool(false),
    definitions: String::new(),
  };

  fixture.base.load_definition(
    r#"
declare buffer: {
    create: @checked (size: number) -> buffer,
    readi8: @checked (b: buffer, offset: number) -> number,
    writef64: @checked (b: buffer, offset: number, value: number) -> (),
}
"#,
    false,
  );

  let result = fixture.check_non_strict(
    r#"
local b = buffer.create(100)
buffer.writef64(b, 0, 5)
buffer.readi8(b, 0)
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_optionals_in_checked_function_can_be_omitted() {
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result = fixture.check_non_strict(
    r#"
optionalArgsAtTheEnd1("a")
optionalArgsAtTheEnd1("a", 3)
optionalArgsAtTheEnd1("a", nil, 3)
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_optionals_in_checked_function_in_middle_cannot_be_omitted() {
  use ulua_analysis::records::{
    check_result::CheckResult, checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
  };
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::{
      require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      type_error_data_ref::type_error_data_ref,
    },
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
optionalArgsAtTheEnd2("a", "a") -- error
optionalArgsAtTheEnd2("a", nil, "b")
optionalArgsAtTheEnd2("a", 3, "b")
optionalArgsAtTheEnd2("a", "b", "c") -- error
"#,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(1, 27), "optionalArgsAtTheEnd2");
  require_non_strict_checked_error_at(&result, Position::new(4, 27), "optionalArgsAtTheEnd2");

  let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[2])
    .expect("expected CheckedFunctionIncorrectArgs");

  assert_eq!(3, r1.expected());
  assert_eq!(2, r1.actual());
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_phi_node_assignment() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = "a" -- x1
if cond() then
    x = 3 -- x2
end
lower(x) -- phi {x1, x2}
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_phi_node_assignment_err() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = nil
if cond() then
    if cond() then
        x = 5
    end
    abs(x)
else
    lower(x)
end
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(8, 10), "lower");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_sequencing_if_checked_call() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x
if cond() then
  x = 5
else
  x = nil
end
lower(x)
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(7, 6), "lower");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_simple_negation_caching_example() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 3
abs(x)
abs(x)
"#,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result: CheckResult = fixture.check_non_strict(
    r#"
local x = 3
contrived(x)
contrived(x)
"#,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(2, 10), "contrived");
  require_non_strict_checked_error_at(&result, Position::new(3, 10), "contrived");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_simple_non_strict_failure() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
abs("hi")
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(1, 4), "abs");
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_tblprop_is_checked() {
  use ulua_analysis::records::check_result::CheckResult;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  };

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
foo.bar("hi")
"#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  require_non_strict_checked_error_at(&result, Position::new(1, 8), "foo.bar");
}

// Source: `tests/NonStrictTypeChecker.test.cpp`
#[test]
fn non_strict_type_checker_typecheck_class_method_bodies() {
  use ulua_analysis::records::{
    check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _force_old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _user_defined_classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy_type_prototyping = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);

  let mut fixture = NonStrictTypeCheckerFixture::default();

  let result: CheckResult = fixture.check_non_strict(
    r#"
        --!nonstrict
        class Student
            public name: number
            function greet(self)
                return `Hello, {lower(self.name)}!`
            end
        end
    "#,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[0]).is_some());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_unknown_globals_in_function_calls() {
  use ulua_analysis::records::{
    check_result::CheckResult,
    unknown_symbol::{Context, UnknownSymbol},
  };
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);

  let result: CheckResult = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        local function foo() : ()
            bar()
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let unknown_symbol =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("bar", unknown_symbol.name());
  assert_eq!(Context::Binding, unknown_symbol.context());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_unknown_globals_in_non_strict_1() {
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();

  let result = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        foo = 5
        local wrong1 = foob

        local x = 12
        local wrong2 = x + foblm
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_unknown_globals_in_one_sided_conditionals() {
  use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        local function foo(cond) : ()
            if cond then
                bar()
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("bar", err.name());
  assert_eq!(Context::Binding, err.context());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_unknown_types_in_non_strict() {
  use ulua_analysis::records::{
    check_result::CheckResult,
    unknown_symbol::{Context, UnknownSymbol},
  };
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);

  let result: CheckResult = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("Foo", err.name());
  assert_eq!(Context::Type, err.context());
}

// Ported from `tests/NonStrictTypeChecker.test.cpp`.
#[test]
fn non_strict_type_checker_unknown_types_in_non_strict_2() {
  use ulua_analysis::records::{
    check_result::CheckResult,
    unknown_symbol::{Context, UnknownSymbol},
  };
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);

  let result: CheckResult = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        --!nonstrict
        local foo = 1 :: Foo
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("Foo", err.name());
  assert_eq!(Context::Type, err.context());
}
