extern crate alloc;

mod non_strict_type_checker_binop_is_checked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:561:non_strict_type_checker_binop_is_checked`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_binop_is_checked

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_binop_is_checked() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::{
      functions::require_checked_function_call_error::require_checked_function_call_error,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
        local foo = 4 + abs("foo")
    "#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_checked_function_call_error(&result, 0, "abs", "number", "string");
  }
}

mod non_strict_type_checker_buffer_is_not_unknown {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_buffer_is_not_unknown() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
local function wrap(b: buffer, i: number, v: number)
    buffer.writeu32(b, i * 4, v)
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_exprgroup_is_checked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:546:non_strict_type_checker_exprgroup_is_checked`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_exprgroup_is_checked

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_exprgroup_is_checked() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::{
      functions::require_checked_function_call_error::require_checked_function_call_error,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
        local foo = (abs("foo"))
    "#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_checked_function_call_error(&result, 0, "abs", "number", "string");
  }
}

mod non_strict_type_checker_fn_expr_produces_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:408:non_strict_type_checker_fn_expr_produces_error`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_fn_expr_produces_error

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_fn_expr_produces_error() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 5
local y = function() lower(x) end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 27), "lower");
  }
}

mod non_strict_type_checker_function_def_basic_errors {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_basic_errors() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
function f(x : string)
    abs(x)
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 8), "abs");
  }
}

mod non_strict_type_checker_function_def_basic_no_errors {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_basic_no_errors() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
      r#"
function f(x)
    abs(x)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_function_def_failure {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:360:non_strict_type_checker_function_def_failure`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_failure

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_failure() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
function f(x)
    abs(lower(x))
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 8), "abs");
  }
}

mod non_strict_type_checker_function_def_if_assignment_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:447:non_strict_type_checker_function_def_if_assignment_errors`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_if_assignment_errors

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_if_assignment_errors() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
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
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(7, 10), "lower");
  }
}

mod non_strict_type_checker_function_def_if_assignment_no_errors {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_if_assignment_no_errors() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
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
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_function_def_if_no_else {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_if_no_else() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
      r#"
function f(x)
    if cond() then
        abs(x)
    end
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_function_def_if_warns_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:420:non_strict_type_checker_function_def_if_warns_never`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_if_warns_never

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_if_warns_never() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
function f(x: never)
    if cond() then
        abs(x)
    else
        lower(x)
    end
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_function_def_sequencing_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:371:non_strict_type_checker_function_def_sequencing_errors`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_sequencing_errors

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_sequencing_errors() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_function_definition_error_at::require_non_strict_function_definition_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
function f(x)
    abs(x)
    lower(x)
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_function_definition_error_at(&result, Position::new(1, 11), "x");
  }
}

mod non_strict_type_checker_function_def_sequencing_errors_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:384:non_strict_type_checker_function_def_sequencing_errors_2`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_sequencing_errors_2

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_sequencing_errors_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::check_result::CheckResult,
    };
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local t = {function(x)
    abs(x)
    lower(x)
end}
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "the argument 'x' is used in a way that will error at runtime",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod non_strict_type_checker_function_def_unrelated_checked_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:326:non_strict_type_checker_function_def_unrelated_checked_calls`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_function_def_unrelated_checked_calls

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_function_def_unrelated_checked_calls() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
function h(x, y)
    abs(x)
    lower(y)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_generic_type_instantiation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:463:non_strict_type_checker_generic_type_instantiation`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_generic_type_instantiation

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_generic_type_instantiation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::check_result::CheckResult,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
        function array<T>(): {T}
            return {}
        end

        local foo = array<<number>>()
        local bar = array<<string>>()
    "#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{number}",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
    assert_eq!(
      "{string}",
      to_string_type_id(fixture.base.require_type_string(&String::from("bar")))
    );
  }
}

mod non_strict_type_checker_generic_type_packs_in_non_strict {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_generic_type_packs_in_non_strict() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
      r#"
--!nonstrict
local test: <T...>(T...) -> () -- TypeError: Unknown type 'T'
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_if_then_else_does_not_warn_with_never_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:210:non_strict_type_checker_if_then_else_does_not_warn_with_never_local`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_does_not_warn_with_never_local

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_does_not_warn_with_never_local() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : never
if cond() then
    abs(x)
else
    lower(x)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_if_then_else_doesnt_warn_else_branch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:240:non_strict_type_checker_if_then_else_doesnt_warn_else_branch`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_doesnt_warn_else_branch

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_doesnt_warn_else_branch() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : string = "hi"
if cond() then
    abs(x)
else
    lower(x)
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
  }
}

mod non_strict_type_checker_if_then_else_expr_doesnt_warn_else_branch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:301:non_strict_type_checker_if_then_else_expr_doesnt_warn_else_branch`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_expr_doesnt_warn_else_branch

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_expr_doesnt_warn_else_branch() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : string = "hi"
local y = if cond() then abs(x) else lower(x)
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 29), "abs");
  }
}

mod non_strict_type_checker_if_then_else_expr_should_not_warn_for_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:291:non_strict_type_checker_if_then_else_expr_should_not_warn_for_never`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_expr_should_not_warn_for_never

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_expr_should_not_warn_for_never() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : never
local y = if cond() then abs(x) else lower(x)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_if_then_else_expr_should_warn {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:280:non_strict_type_checker_if_then_else_expr_should_warn`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_expr_should_warn

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_expr_should_warn() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 42
local y = if cond() then abs(x) else lower(x)
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 43), "lower");
  }
}

mod non_strict_type_checker_if_then_else_warns_nil_branches {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:224:non_strict_type_checker_if_then_else_warns_nil_branches`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_else_warns_nil_branches

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_else_warns_nil_branches() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x
if cond() then
    abs(x)
else
    lower(x)
end
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
    require_non_strict_checked_error_at(&result, Position::new(5, 10), "lower");
  }
}

mod non_strict_type_checker_if_then_no_else {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_no_else() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : string
if cond() then
    abs(x)
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(3, 8), "abs");
  }
}

mod non_strict_type_checker_if_then_no_else_err_in_cond {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:268:non_strict_type_checker_if_then_no_else_err_in_cond`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_if_then_no_else_err_in_cond

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_if_then_no_else_err_in_cond() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x : string = ""
if abs(x) then
    lower(x)
end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 7), "abs");
  }
}

mod non_strict_type_checker_incomplete_function_annotation {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_incomplete_function_annotation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: () ->
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod non_strict_type_checker_incorrect_arg_count {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_incorrect_arg_count() {
    use alloc::string::String;

    use ulua_analysis::records::{
      check_result::CheckResult, checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
foo.bar(1,2,3)
abs(3, "hi");
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[0])
      .expect("expected CheckedFunctionIncorrectArgs");
    let r2 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[1])
      .expect("expected CheckedFunctionIncorrectArgs");

    assert_eq!("abs", r1.function_name());
    assert_eq!("foo.bar", r2.function_name());
  }
}

mod non_strict_type_checker_interesting_checked_functions {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_interesting_checked_functions() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
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
    ));

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 12), "onlyNums");
    require_non_strict_checked_error_at(&result, Position::new(5, 10), "mixedArgs");
    require_non_strict_checked_error_at(&result, Position::new(6, 15), "mixedArgs");
    require_non_strict_checked_error_at(&result, Position::new(10, 12), "optionalArg");
  }
}

mod non_strict_type_checker_local_fn_produces_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:397:non_strict_type_checker_local_fn_produces_error`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_local_fn_produces_error

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_local_fn_produces_error() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 5
local function y() lower(x) end
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 25), "lower");
  }
}

mod non_strict_type_checker_local_only_one_warning {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:496:non_strict_type_checker_local_only_one_warning`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_local_only_one_warning

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_local_only_one_warning() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 5
lower(x)
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 6), "lower");
  }
}

mod non_strict_type_checker_nested_function_calls_constant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:197:non_strict_type_checker_nested_function_calls_constant`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_nested_function_calls_constant

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_nested_function_calls_constant() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x
abs(lower(x))
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 4), "abs");
    require_non_strict_checked_error_at(&result, Position::new(2, 10), "lower");
  }
}

mod non_strict_type_checker_new_non_strict_should_suppress_dynamic_require_errors {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_new_non_strict_should_suppress_dynamic_require_errors() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_require::UnknownRequire;
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff_debug_luau_force_old_solver =
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    // Nonstrict mode should suppress dynamic require errors
    let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
function passThrough(module)
    require(module)
end
    "#,
      ),
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
      &String::from(
        r#"
function passThrough(module)
    require(module)
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result_strict.errors.len(), "{:?}", result_strict.errors);
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
  }
}

mod non_strict_type_checker_new_non_strict_should_suppress_unknown_require_errors {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_new_non_strict_should_suppress_unknown_require_errors() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_require::UnknownRequire;
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    // Nonstrict mode: should suppress unknown require errors
    let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
      ),
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
      &String::from(
        r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
      ),
      None,
    );

    assert_eq!(2, result_strict.errors.len(), "{:?}", result_strict.errors);
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[1]).is_some());
  }
}

mod non_strict_type_checker_new_non_strict_skips_warnings_on_unreduced_typefunctions {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_new_non_strict_skips_warnings_on_unreduced_typefunctions() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
      r#"
function foo(x)
    local y = x + 1
    return abs(y)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:831:non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::{check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError},
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
getAllTheArgsWrong(3, true, "what")
"#,
    ));

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
}

mod non_strict_type_checker_non_strict_shouldnt_warn_on_require_module {
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:639:non_strict_type_checker_non_strict_shouldnt_warn_on_require_module`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record SourceCode (Analysis/include/Luau/FileResolver.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrictModule (tests/NonStrictTypeChecker.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_non_strict_shouldnt_warn_on_require_module

  #[cfg(test)]
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
      .insert(String::from("Modules/A"), Type::Module);

    fixture.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
--!nonstrict
local E = require(script.Parent.A)
"#,
      ),
    );

    let result = fixture.check_non_strict_module(&String::from("Modules/B"));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_non_testable_type_throws_ice {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_non_testable_type_throws_ice() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _force_old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.check_non_strict(&String::from(
        r#"os.time({year = 0, month = 0, day = 0, min = 0, isdst = nil})
"#,
      ));
    }));

    let payload = result.expect_err("expected InternalCompilerError");
    assert!(
      payload.is::<InternalCompilerError>(),
      "expected InternalCompilerError panic payload"
    );
  }
}

mod non_strict_type_checker_nonstrict_check_block_recursion_limit {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_nonstrict_check_block_recursion_limit() {
    use ulua_common::{DFInt, FFlag, FInt};
    use ulua_unit_test::{
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let limit: usize = 250;

    let _sff = ScopedFastFlag::new(&FFlag::LuauAddRecursionCounterToNonStrictTypeChecker, true);

    let _luau_non_strict_type_checker_recursion_limit = ScopedFastInt::new(
      &FInt::LuauNonStrictTypeCheckerRecursionLimit,
      limit as i32 - 100,
    );
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
      &DFInt::LuauConstraintGeneratorRecursionLimit,
      limit as i32 + 500,
    );
    let _luau_check_recursion_limit =
      ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 + 500);

    let mut fixture = NonStrictTypeCheckerFixture::default();
    let code = "do ".repeat(limit) + "local a = 1" + &" end".repeat(limit);
    let result = fixture.check_non_strict(&code);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_nonstrict_check_expr_recursion_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:883:non_strict_type_checker_nonstrict_check_expr_recursion_limit`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_nonstrict_check_expr_recursion_limit

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_nonstrict_check_expr_recursion_limit() {
    use alloc::string::String;

    use ulua_common::{DFInt, FFlag, FInt};
    use ulua_unit_test::{
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let limit: usize = 250;

    let _sff = ScopedFastFlag::new(&FFlag::LuauAddRecursionCounterToNonStrictTypeChecker, true);

    let _luau_non_strict_type_checker_recursion_limit = ScopedFastInt::new(
      &FInt::LuauNonStrictTypeCheckerRecursionLimit,
      limit as i32 - 100,
    );
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
      &DFInt::LuauConstraintGeneratorRecursionLimit,
      limit as i32 + 500,
    );
    let _luau_check_recursion_limit =
      ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 + 500);

    let mut fixture = NonStrictTypeCheckerFixture::default();
    let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
    let result = fixture.check_non_strict(&code);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_nonstrict_method_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:676:non_strict_type_checker_nonstrict_method_calls`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - calls -> method NonStrictTypeCheckerFixture::getFrontend (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> function registerBuiltinGlobals (Analysis/src/BuiltinDefinitions.cpp)
  //!   - calls -> method Fixture::registerTestTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_nonstrict_method_calls

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_nonstrict_method_calls() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result: CheckResult = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local test = "test"
        test:lower()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_nonstrict_shouldnt_warn_on_valid_buffer_use {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
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
      &String::from(
        r#"
declare buffer: {
    create: @checked (size: number) -> buffer,
    readi8: @checked (b: buffer, offset: number) -> number,
    writef64: @checked (b: buffer, offset: number, value: number) -> (),
}
"#,
      ),
      false,
    );

    let result = fixture.check_non_strict(&String::from(
      r#"
local b = buffer.create(100)
buffer.writef64(b, 0, 5)
buffer.readi8(b, 0)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_optionals_in_checked_function_can_be_omitted {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_optionals_in_checked_function_can_be_omitted() {
    use alloc::string::String;

    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
      r#"
optionalArgsAtTheEnd1("a")
optionalArgsAtTheEnd1("a", 3)
optionalArgsAtTheEnd1("a", nil, 3)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_optionals_in_checked_function_in_middle_cannot_be_omitted {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_optionals_in_checked_function_in_middle_cannot_be_omitted() {
    use alloc::string::String;

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

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
optionalArgsAtTheEnd2("a", "a") -- error
optionalArgsAtTheEnd2("a", nil, "b")
optionalArgsAtTheEnd2("a", 3, "b")
optionalArgsAtTheEnd2("a", "b", "c") -- error
"#,
    ));

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(1, 27), "optionalArgsAtTheEnd2");
    require_non_strict_checked_error_at(&result, Position::new(4, 27), "optionalArgsAtTheEnd2");

    let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[2])
      .expect("expected CheckedFunctionIncorrectArgs");

    assert_eq!(3, r1.expected());
    assert_eq!(2, r1.actual());
  }
}

mod non_strict_type_checker_phi_node_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:506:non_strict_type_checker_phi_node_assignment`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item non_strict_type_checker_phi_node_assignment

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_phi_node_assignment() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_unit_test::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = "a" -- x1
if cond() then
    x = 3 -- x2
end
lower(x) -- phi {x1, x2}
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_phi_node_assignment_err {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:519:non_strict_type_checker_phi_node_assignment_err`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_phi_node_assignment_err

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_phi_node_assignment_err() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
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
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(8, 10), "lower");
  }
}

mod non_strict_type_checker_sequencing_if_checked_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:311:non_strict_type_checker_sequencing_if_checked_call`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item non_strict_type_checker_sequencing_if_checked_call

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_sequencing_if_checked_call() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x
if cond() then
  x = 5
else
  x = nil
end
lower(x)
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(7, 6), "lower");
  }
}

mod non_strict_type_checker_simple_negation_caching_example {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_simple_negation_caching_example() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 3
abs(x)
abs(x)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
local x = 3
contrived(x)
contrived(x)
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 10), "contrived");
    require_non_strict_checked_error_at(&result, Position::new(3, 10), "contrived");
  }
}

mod non_strict_type_checker_simple_non_strict_failure {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_simple_non_strict_failure() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
abs("hi")
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(1, 4), "abs");
  }
}

mod non_strict_type_checker_tblprop_is_checked {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_tblprop_is_checked() {
    use alloc::string::String;

    use ulua_analysis::records::check_result::CheckResult;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
    };

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
foo.bar("hi")
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(1, 8), "foo.bar");
  }
}

mod non_strict_type_checker_typecheck_class_method_bodies {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:900:non_strict_type_checker_typecheck_class_method_bodies`
  //! Source: `tests/NonStrictTypeChecker.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
  //!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item non_strict_type_checker_typecheck_class_method_bodies

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_typecheck_class_method_bodies() {
    use alloc::string::String;

    use ulua_analysis::records::{
      check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _force_old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _user_defined_classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy_type_prototyping = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
      r#"
        --!nonstrict
        class Student
            public name: number
            function greet(self)
                return `Hello, {lower(self.name)}!`
            end
        end
    "#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[0]).is_some());
  }
}

mod non_strict_type_checker_unknown_globals_in_function_calls {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_unknown_globals_in_function_calls() {
    use alloc::string::String;

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
      &String::from(
        r#"
        local function foo() : ()
            bar()
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let unknown_symbol =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("bar", unknown_symbol.name());
    assert_eq!(Context::Binding, unknown_symbol.context());
  }
}

mod non_strict_type_checker_unknown_globals_in_non_strict_1 {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_unknown_globals_in_non_strict_1() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        foo = 5
        local wrong1 = foob

        local x = 12
        local wrong2 = x + foblm
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod non_strict_type_checker_unknown_globals_in_one_sided_conditionals {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_unknown_globals_in_one_sided_conditionals() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local function foo(cond) : ()
            if cond then
                bar()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("bar", err.name());
    assert_eq!(Context::Binding, err.context());
  }
}

mod non_strict_type_checker_unknown_types_in_non_strict {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_unknown_types_in_non_strict() {
    use alloc::string::String;

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
      &String::from(
        r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("Foo", err.name());
    assert_eq!(Context::Type, err.context());
  }
}

mod non_strict_type_checker_unknown_types_in_non_strict_2 {
  //! Ported from `tests/NonStrictTypeChecker.test.cpp`.

  #[cfg(test)]
  #[test]
  fn non_strict_type_checker_unknown_types_in_non_strict_2() {
    use alloc::string::String;

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
      &String::from(
        r#"
        --!nonstrict
        local foo = 1 :: Foo
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("Foo", err.name());
    assert_eq!(Context::Type, err.context());
  }
}
