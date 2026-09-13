extern crate alloc;

mod type_infer_operators_add_type_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1564:type_infer_operators_add_type_function_works`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_add_type_function_works

  #[cfg(test)]
  #[test]
  fn type_infer_operators_add_type_function_works() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function add(x, y)
            return x + y
        end

        local a = add(1, 2)
        local b = add("foo", "bar")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "add<string, string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Operator '+' could not be applied to operands of types string and string; there is no corresponding overload for __add",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_operators_and_adds_boolean_no_superfluous_union {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:96:type_infer_operators_and_adds_boolean_no_superfluous_union`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_and_adds_boolean_no_superfluous_union

  #[cfg(test)]
  #[test]
  fn type_infer_operators_and_adds_boolean_no_superfluous_union() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "a" and true
        local x:boolean = s
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_operators_and_binexps_dont_unify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:848:type_infer_operators_and_binexps_dont_unify`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_operators_and_binexps_dont_unify

  #[cfg(test)]
  #[test]
  fn type_infer_operators_and_binexps_dont_unify() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = {}
        while true and t[1] do
            print(t[1].test)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_and_does_not_always_add_boolean {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:86:type_infer_operators_and_does_not_always_add_boolean`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_and_does_not_always_add_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_operators_and_does_not_always_add_boolean() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "a" and 10
        local x:boolean|number = s
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_operators_and_or_ternary {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:106:type_infer_operators_and_or_ternary`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_and_or_ternary

  #[cfg(test)]
  #[test]
  fn type_infer_operators_and_or_ternary() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = (1/2) > 0.5 and "a" or 10
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_operators_call_and_or_of_functions {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:555:type_infer_operators_call_and_or_of_functions`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_call_and_or_of_functions

  #[cfg(test)]
  #[test]
  fn type_infer_operators_call_and_or_of_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function f() return 1; end
function g() return 2; end
local x = false
(x and f or g)()
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_call_or_of_functions {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:544:type_infer_operators_call_or_of_functions`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_call_or_of_functions

  #[cfg(test)]
  #[test]
  fn type_infer_operators_call_or_of_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function f() return 1; end
function g() return 2; end
(f or g)()
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:342:type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local M = {}
        function M.new()
            return setmetatable({}, M)
        end
        function M.__lt(left, right) return true end

        local a = M.new()
        local b = {}
        local c = a < b -- line 10
        local d = b < a -- line 11
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      Location {
        begin: Position {
          line: 10,
          column: 18
        },
        end: Position {
          line: 10,
          column: 23
        }
      },
      result.errors[0].location
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 11,
          column: 18
        },
        end: Position {
          line: 11,
          column: 23
        }
      },
      result.errors[1].location
    );
  }
}

mod type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:292:type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable() {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_compare_unrelated_types::CannotCompareUnrelatedTypes, generic_error::GenericError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        local b = {}
        local c = a < b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      type_error_data_ref::<CannotCompareUnrelatedTypes>(&result.errors[0])
        .expect("expected CannotCompareUnrelatedTypes");
    } else {
      let r#gen =
        type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
      assert_eq!(
        "Type a cannot be compared with < because it has no metatable",
        r#gen.message()
      );
    }
  }
}

mod type_infer_operators_cannot_indirectly_compare_types_that_do_not_offer_overloaded_ordering_operators {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:314:type_infer_operators_cannot_indirectly_compare_types_that_do_not_offer_overloaded_ordering_operators`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_cannot_indirectly_compare_types_that_do_not_offer_overloaded_ordering_operators

  #[cfg(test)]
  #[test]
  fn type_infer_operators_cannot_indirectly_compare_types_that_do_not_offer_overloaded_ordering_operators()
   {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_compare_unrelated_types::CannotCompareUnrelatedTypes, generic_error::GenericError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local M = {}
        function M.new()
            return setmetatable({}, M)
        end
        type M = typeof(M.new())

        local a = M.new()
        local b = M.new()
        local c = a < b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      type_error_data_ref::<CannotCompareUnrelatedTypes>(&result.errors[0])
        .expect("expected CannotCompareUnrelatedTypes");
    } else {
      let r#gen =
        type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
      assert_eq!("Table M does not offer metamethod __lt", r#gen.message());
    }
  }
}

mod type_infer_operators_cli_38355_recursive_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:918:type_infer_operators_cli_38355_recursive_union`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_cli_38355_recursive_union

  #[cfg(test)]
  #[test]
  fn type_infer_operators_cli_38355_recursive_union() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local _
        _ += _ and _ or _ and _ or _ and _
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type used in + operation; consider adding a type annotation to '_'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_operators_compare_numbers {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:270:type_infer_operators_compare_numbers`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_compare_numbers

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compare_numbers() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 441
        local b = 0
        local c = a < b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_compare_singleton_string_to_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1604:type_infer_operators_compare_singleton_string_to_string`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_compare_singleton_string_to_string

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compare_singleton_string_to_string() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function test(a: string, b: string)
            if a == "Pet" and b == "Pet" then
                return true
            elseif a ~= b then
                return a < b
            else
                return false
            end
        end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_compare_strings {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:281:type_infer_operators_compare_strings`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_compare_strings

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compare_strings() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = '441'
        local b = '0'
        local c = a < b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_compound_assign_basic {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:405:type_infer_operators_compound_assign_basic`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_basic

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_basic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = 10
        s += 20
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_operators_compound_assign_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:445:type_infer_operators_compound_assign_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_metatable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type V2B = { x: number, y: number }
        local v2b: V2B = { x = 0, y = 0 }
        local VMT = {}

        VMT.__add = function(a: V2, b: V2): V2
            return setmetatable({ x = a.x + b.x, y = a.y + b.y }, VMT)
        end

        type V2 = typeof(setmetatable(v2b, VMT))

        local v1: V2 = setmetatable({ x = 1, y = 2 }, VMT)
        local v2: V2 = setmetatable({ x = 3, y = 4 }, VMT)
        v1 += v2
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_compound_assign_metatable_with_changing_return_type {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:467:type_infer_operators_compound_assign_metatable_with_changing_return_type`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_metatable_with_changing_return_type

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_metatable_with_changing_return_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type T = { x: number }
        local MT = {}

        function MT:__add(other): number
            return 112
        end

        local t = setmetatable({x = 2}, MT)
        local u = t + 3
        t += 3
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("t", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_operators_compound_assign_mismatch_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:522:type_infer_operators_compound_assign_mismatch_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_mismatch_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_mismatch_metatable() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type V2B = { x: number, y: number }
        local v2b: V2B = { x = 0, y = 0 }
        local VMT = {}
        type V2 = typeof(setmetatable(v2b, VMT))

        function VMT.__mod(a: V2, b: V2): number
            return a.x * b.x + a.y * b.y
        end

        local v1: V2 = setmetatable({ x = 1, y = 2 }, VMT)
        local v2: V2 = setmetatable({ x = 3, y = 4 }, VMT)
        v1 %= v2
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'V2', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_operators_compound_assign_mismatch_op {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:415:type_infer_operators_compound_assign_mismatch_op`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_mismatch_op

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_mismatch_op() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = 10
        s += true
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 13
        },
        end: Position {
          line: 2,
          column: 17
        }
      },
      result.errors[0].location
    );
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    let number_type = fixture.get_builtins().number_type;
    let boolean_type = fixture.get_builtins().boolean_type;
    assert_eq!(number_type, tm.wanted_type);
    assert_eq!(boolean_type, tm.given_type);
  }
}

mod type_infer_operators_compound_assign_mismatch_result {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:425:type_infer_operators_compound_assign_mismatch_result`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_mismatch_result

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_mismatch_result() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = 'hello'
        s += 10
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 8 },
        end: Position { line: 2, column: 9 }
      },
      result.errors[0].location
    );
    let number_type = fixture.get_builtins().number_type;
    let string_type = fixture.get_builtins().string_type;
    let tm0 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(number_type, tm0.wanted_type);
    assert_eq!(string_type, tm0.given_type);

    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        Location {
          begin: Position { line: 2, column: 8 },
          end: Position {
            line: 2,
            column: 15
          }
        },
        result.errors[1].location
      );
      let tm1 =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!(string_type, tm1.wanted_type);
      assert_eq!(number_type, tm1.given_type);
    }
  }
}

mod type_infer_operators_compound_assign_result_must_be_compatible_with_var {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:494:type_infer_operators_compound_assign_result_must_be_compatible_with_var`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_compound_assign_result_must_be_compatible_with_var

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_assign_result_must_be_compatible_with_var() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function __add(left, right)
            return 123
        end

        local mt = {
            __add = __add,
        }

        local x = setmetatable({}, mt)
        local v: number

        v += x -- okay: number + x -> number
        x += v -- not okay: x </: number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 13,
          column: 8
        },
        end: Position {
          line: 13,
          column: 14
        }
      },
      result.errors[0].location
    );

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("x", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_operators_compound_operator_on_upvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1641:type_infer_operators_compound_operator_on_upvalue`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::bytes (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_operators_compound_operator_on_upvalue

  #[cfg(test)]
  #[test]
  fn type_infer_operators_compound_operator_on_upvalue() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local byteCursor: number = 0

        local function advance(bytes: number)
            byteCursor += bytes
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_concat_op_on_free_lhs_and_string_rhs {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:780:type_infer_operators_concat_op_on_free_lhs_and_string_rhs`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_concat_op_on_free_lhs_and_string_rhs

  #[cfg(test)]
  #[test]
  fn type_infer_operators_concat_op_on_free_lhs_and_string_rhs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::cannot_infer_binary_operation::CannotInferBinaryOperation,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x)
            return x .. "y"
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a>(a) -> concat<a, string>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0])
        .expect("expected CannotInferBinaryOperation");
    }
  }
}

mod type_infer_operators_concat_op_on_string_lhs_and_free_rhs {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:800:type_infer_operators_concat_op_on_string_lhs_and_free_rhs`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_concat_op_on_string_lhs_and_free_rhs

  #[cfg(test)]
  #[test]
  fn type_infer_operators_concat_op_on_string_lhs_and_free_rhs() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x)
            return "foo" .. x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "<a>(a) -> concat<string, a>"
    } else {
      "(string) -> string"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:706:type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops

  #[cfg(test)]
  #[test]
  fn type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops()
  {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        generic_error::GenericError, type_mismatch::TypeMismatch,
        uninhabited_type_function::UninhabitedTypeFunction,
      },
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local a = "1.24" + 123 -- not allowed

        local foo = {
            value = 10
        }

        local b = foo + 1 -- not allowed

        local bar = {
            value = 1
        }

        local mt = {}

        setmetatable(bar, mt)

        mt.__add = function(a: typeof(bar), b: number): number
            return a.value + b
        end

        local c = bar + 1 -- allowed

        local d = bar + foo -- not allowed
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
        .expect("expected UninhabitedTypeFunction");
      assert_eq!(
        Location {
          begin: Position {
            line: 2,
            column: 18
          },
          end: Position {
            line: 2,
            column: 30
          }
        },
        result.errors[0].location
      );

      type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[1])
        .expect("expected UninhabitedTypeFunction");
      assert_eq!(
        Location {
          begin: Position {
            line: 8,
            column: 18
          },
          end: Position {
            line: 8,
            column: 25
          }
        },
        result.errors[1].location
      );

      type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[2])
        .expect("expected UninhabitedTypeFunction");
      assert_eq!(
        Location {
          begin: Position {
            line: 24,
            column: 18
          },
          end: Position {
            line: 24,
            column: 27
          }
        },
        result.errors[2].location
      );
    } else {
      let tm0 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("number", to_string_type_id(tm0.wanted_type));
      assert_eq!("string", to_string_type_id(tm0.given_type));

      let gen1 =
        type_error_data_ref::<GenericError>(&result.errors[1]).expect("expected GenericError");
      assert_eq!(
        "Binary operator '+' not supported by types 'foo' and 'number'",
        gen1.message()
      );

      let tm2 =
        type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
      assert_eq!("number", to_string_type_id(tm2.wanted_type));
      assert_eq!(
        fixture.base.require_type_string(&String::from("foo")),
        tm2.given_type
      );
    }
  }
}

mod type_infer_operators_dont_strip_nil_from_rhs_or_operator {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1000:type_infer_operators_dont_strip_nil_from_rhs_or_operator`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_dont_strip_nil_from_rhs_or_operator

  #[cfg(test)]
  #[test]
  fn type_infer_operators_dont_strip_nil_from_rhs_or_operator() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local a: number? = nil
local b: number = 1 or a
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
    assert_eq!("number?", to_string_type_id(tm.given_type));
  }
}

mod type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1247:type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds

  #[cfg(test)]
  #[test]
  fn type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mm = {}
        type Foo = typeof(setmetatable({}, mm))
        local x: Foo
        local y: Foo?

        local v1 = x == y
        local v2 = y == x
        local v3 = x ~= y
        local v4 = y ~= x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result2 = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mm1 = {
            x = "foo",
        }

        local mm2 = {
            y = "bar",
        }

        type Foo = typeof(setmetatable({}, mm1))
        type Bar = typeof(setmetatable({}, mm2))

        local x1: Foo
        local x2: Foo?
        local y1: Bar
        local y2: Bar?

        local v1 = x1 == y1
        local v2 = x2 == y2
    "#,
      ),
      None,
    );

    assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);
    assert_eq!(
      "Types Foo and Bar cannot be compared with == because they do not have the same metatable",
      to_string_type_error(&result2.errors[0])
    );
  }
}

mod type_infer_operators_error_on_invalid_operand_types_to_relational_operators {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:863:type_infer_operators_error_on_invalid_operand_types_to_relational_operators`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_error_on_invalid_operand_types_to_relational_operators

  #[cfg(test)]
  #[test]
  fn type_infer_operators_error_on_invalid_operand_types_to_relational_operators() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: boolean = true
        local b: boolean = false
        local foo = a < b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Types 'boolean' and 'boolean' cannot be compared with relational operator <",
        ge.message()
      );
    } else {
      assert_eq!(
        "Type 'boolean' cannot be compared with relational operator <",
        ge.message()
      );
    }
  }
}

mod type_infer_operators_error_on_invalid_operand_types_to_relational_operators_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:887:type_infer_operators_error_on_invalid_operand_types_to_relational_operators_2`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_error_on_invalid_operand_types_to_relational_operators_2

  #[cfg(test)]
  #[test]
  fn type_infer_operators_error_on_invalid_operand_types_to_relational_operators_2() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number | string = ""
        local b: number | string = 1
        local foo = a < b
    "#,
      ),
      None,
    );

    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      return;
    }

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
      "Types 'number | string' and 'number | string' cannot be compared with relational operator <",
      ge.message()
    );
  }
}

mod type_infer_operators_expected_types_through_binary_and {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1288:type_infer_operators_expected_types_through_binary_and`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_expected_types_through_binary_and

  #[cfg(test)]
  #[test]
  fn type_infer_operators_expected_types_through_binary_and() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: "a" | "b" | boolean = math.random() > 0.5 and "a"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_expected_types_through_binary_or {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1297:type_infer_operators_expected_types_through_binary_or`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_expected_types_through_binary_or

  #[cfg(test)]
  #[test]
  fn type_infer_operators_expected_types_through_binary_or() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: "a" | "b" | boolean = math.random() > 0.5 or "b"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_floor_division_binary_op {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:178:type_infer_operators_floor_division_binary_op`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_floor_division_binary_op

  #[cfg(test)]
  #[test]
  fn type_infer_operators_floor_division_binary_op() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 4 // 8
        local b = -4 // 9
        local c = 9
        c //= -6.5
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_operators_in_nonstrict_mode_strip_nil_from_intersections_when_considering_relational_operators {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:390:type_infer_operators_in_nonstrict_mode_strip_nil_from_intersections_when_considering_relational_operators`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_in_nonstrict_mode_strip_nil_from_intersections_when_considering_relational_operators

  #[cfg(test)]
  #[test]
  fn type_infer_operators_in_nonstrict_mode_strip_nil_from_intersections_when_considering_relational_operators()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function maybe_a_number(): number?
            return 50
        end

        local a = maybe_a_number() < maybe_a_number()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_infer_any_in_all_modes_when_lhs_is_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1075:type_infer_operators_infer_any_in_all_modes_when_lhs_is_unknown`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_any_in_all_modes_when_lhs_is_unknown

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_any_in_all_modes_when_lhs_is_unknown() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x + y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> add<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in + operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local function f(x, y)
            return x + y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_infer_type_for_generic_concat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1227:type_infer_operators_infer_type_for_generic_concat`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_concat

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_concat() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x .. y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> concat<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in .. operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_division {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1147:type_infer_operators_infer_type_for_generic_division`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method AssemblyBuilderX64::div (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_division

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_division() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x / y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> div<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in / operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_exponentiation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1187:type_infer_operators_infer_type_for_generic_exponentiation`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_exponentiation

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_exponentiation() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x ^ y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> pow<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in ^ operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_floor_division {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1167:type_infer_operators_infer_type_for_generic_floor_division`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method AssemblyBuilderX64::idiv (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_floor_division

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_floor_division() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x // y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> idiv<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in // operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_modulo {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1207:type_infer_operators_infer_type_for_generic_modulo`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_modulo

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_modulo() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x % y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> mod<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in % operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_multiplication {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1127:type_infer_operators_infer_type_for_generic_multiplication`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_multiplication

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_multiplication() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x * y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> mul<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in * operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_infer_type_for_generic_subtraction {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1107:type_infer_operators_infer_type_for_generic_subtraction`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_infer_type_for_generic_subtraction

  #[cfg(test)]
  #[test]
  fn type_infer_operators_infer_type_for_generic_subtraction() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Strict,
      &String::from(
        r#"
        local function f(x, y)
            return x - y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a, b>(a, b) -> sub<a, b>",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown type used in - operation; consider adding a type annotation to 'x'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_luau_polyfill_array_startswith {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1539:type_infer_operators_luau_polyfill_array_startswith`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function startsWith (Common/src/StringUtils.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - translates_to -> rust_item type_infer_operators_luau_polyfill_array_startswith

  #[cfg(test)]
  #[test]
  fn type_infer_operators_luau_polyfill_array_startswith() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function startsWith(value: string, substring: string, position: number?): boolean
	-- Luau FIXME: we have to use a tmp variable, as Luau doesn't understand the logic below narrow position to `number`
	local position_
	if position == nil or position < 1 then
		position_ = 1
	else
		position_ = position
	end

	return value:find(substring, position_, true) == position_
end

return startsWith

    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_luau_polyfill_is_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1465:type_infer_operators_luau_polyfill_is_array`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_luau_polyfill_is_array

  #[cfg(test)]
  #[test]
  fn type_infer_operators_luau_polyfill_is_array() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
return function(value: any): boolean
    if typeof(value) ~= "table" then
        return false
    end
    if next(value) == nil then
        -- an empty table is an empty array
        return true
    end

    local length = #value

    if length == 0 then
        return false
    end

    local count = 0
    local sum = 0
    for key in pairs(value) do
        if typeof(key) ~= "number" then
            return false
        end
        if key % 1 ~= 0 or key < 1 then
            return false
        end
        count += 1
        sum += key
    end

    return sum == (count * (count + 1) / 2)
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_luau_polyfill_is_array_simplified {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1447:type_infer_operators_luau_polyfill_is_array_simplified`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_luau_polyfill_is_array_simplified

  #[cfg(test)]
  #[test]
  fn type_infer_operators_luau_polyfill_is_array_simplified() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
     --!strict
     return function(value: any) : boolean
        if typeof(value) ~= "number" then
           return false
        end
        if value % 1 ~= 0 or value < 1 then
           return false
        end
        return true
     end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_luau_polyfill_string_slice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1507:type_infer_operators_luau_polyfill_string_slice`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_operators_luau_polyfill_string_slice

  #[cfg(test)]
  #[test]
  fn type_infer_operators_luau_polyfill_string_slice() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function slice(str: string, startIndexStr: string | number, lastIndexStr: (string | number)?): string
	local strLen, invalidBytePosition = utf8.len(str)
	assert(strLen ~= nil, ("string `%s` has an invalid byte at position %s"):format(str, tostring(invalidBytePosition)))
    local startIndex = tonumber(startIndexStr)


	-- if no last index length set, go to str length + 1
	local lastIndex = strLen + 1

	assert(typeof(lastIndex) == "number", "lastIndexStr should convert to number")

	if lastIndex > strLen then
		lastIndex = strLen + 1
	end

	local startIndexByte = utf8.offset(str, startIndex)

	return string.sub(str, startIndexByte, startIndexByte)
end

return slice


    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_metatable_operator_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1654:type_infer_operators_metatable_operator_follow`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_operators_metatable_operator_follow

  #[cfg(test)]
  #[test]
  fn type_infer_operators_metatable_operator_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local t1 = {}
local t2 = {}
local mt = {}

mt.__eq = function(a, b)
    return false
end

setmetatable(t1, mt)
setmetatable(t2, mt)

if t1 == t2 then

end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_mm_comparisons_must_return_a_boolean {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1332:type_infer_operators_mm_comparisons_must_return_a_boolean`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_mm_comparisons_must_return_a_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_operators_mm_comparisons_must_return_a_boolean() {
    // Upstream body is disabled under #if 0.
  }
}

mod type_infer_operators_negated_integer_literal_is_a_constant {
  #[cfg(test)]
  #[test]
  fn type_infer_operators_negated_integer_literal_is_a_constant() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff1 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _sff2 = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local a = -4194626i
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "integer",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_operators_no_infinite_expansion_of_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1623:type_infer_operators_no_infinite_expansion_of_free_type`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_operators_no_infinite_expansion_of_free_type

  #[cfg(test)]
  #[test]
  fn type_infer_operators_no_infinite_expansion_of_free_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tooltip = {}

        function tooltip:Show()
            local playerGui = self.Player:FindFirstChild("PlayerGui")
            for _,c in ipairs(playerGui:GetChildren()) do
                if c:IsA("ScreenGui") and c.DisplayOrder > self.Gui.DisplayOrder then
                end
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_operators_normalize_strings_comparison {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1587:type_infer_operators_normalize_strings_comparison`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_operators_normalize_strings_comparison

  #[cfg(test)]
  #[test]
  fn type_infer_operators_normalize_strings_comparison() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function sortKeysForPrinting(a: any, b)
	local typeofA = type(a)
	local typeofB = type(b)
	-- strings and numbers are sorted numerically/alphabetically
	if typeofA == typeofB and (typeofA == "number" or typeofA == "string") then
		return a < b
	end
	-- sort the rest by type name
	return typeofA < typeofB
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_operator_eq_completely_incompatible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1050:type_infer_operators_operator_eq_completely_incompatible`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_operator_eq_completely_incompatible

  #[cfg(test)]
  #[test]
  fn type_infer_operators_operator_eq_completely_incompatible() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string | number = "hi"
        local b: {x: string}? = {x = "bye"}

        local r1 = a == b
        local r2 = b == a
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_operator_eq_operands_are_not_subtypes_of_each_other_but_has_overlap {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1037:type_infer_operators_operator_eq_operands_are_not_subtypes_of_each_other_but_has_overlap`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_operators_operator_eq_operands_are_not_subtypes_of_each_other_but_has_overlap

  #[cfg(test)]
  #[test]
  fn type_infer_operators_operator_eq_operands_are_not_subtypes_of_each_other_but_has_overlap() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: string | number, b: boolean | number)
            return a == b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_operator_eq_verifies_types_do_intersect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1016:type_infer_operators_operator_eq_verifies_types_do_intersect`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_operator_eq_verifies_types_do_intersect

  #[cfg(test)]
  #[test]
  fn type_infer_operators_operator_eq_verifies_types_do_intersect() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = { [number]: T }
        type Fiber = { id: number }
        type null = {}

        local fiberStack: Array<Fiber | null> = {}
        local index = 0

        local function f(fiber: Fiber)
            local a = fiber ~= fiberStack[index]
            local b = fiberStack[index] ~= fiber
        end

        return f
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_or_joins_types {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:26:type_infer_operators_or_joins_types`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_or_joins_types

  #[cfg(test)]
  #[test]
  fn type_infer_operators_or_joins_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "a" or 10
        local x:string|number = s
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(string & ~(false?)) | number",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
    } else {
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
    }
  }
}

mod type_infer_operators_or_joins_types_with_no_extras {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:47:type_infer_operators_or_joins_types_with_no_extras`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_or_joins_types_with_no_extras

  #[cfg(test)]
  #[test]
  fn type_infer_operators_or_joins_types_with_no_extras() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "a" or 10
        local x:number|string = s
        local y = x or "s"
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(string & ~(false?)) | number",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
      );
    } else {
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
      );
    }
  }
}

mod type_infer_operators_or_joins_types_with_no_superfluous_union {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:69:type_infer_operators_or_joins_types_with_no_superfluous_union`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_or_joins_types_with_no_superfluous_union

  #[cfg(test)]
  #[test]
  fn type_infer_operators_or_joins_types_with_no_superfluous_union() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "a" or "b"
        local x:string = s
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(string & ~(false?)) | string",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
    } else {
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("s")))
      );
    }
  }
}

mod type_infer_operators_overload_concat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1676:type_infer_operators_overload_concat`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function writestring (VM/src/lbaselib.cpp)
  //!   - translates_to -> rust_item type_infer_operators_overload_concat

  #[cfg(test)]
  #[test]
  fn type_infer_operators_overload_concat() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauConcatDoesntAlwaysReturnString, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type classData = {
            b:buffer;
            len:number;
        }
        local metatable = {
            __concat = function(self:cls,str:string):cls
                buffer.writestring(self.b,self.len,str)
                self.len+=#str
                return self
            end;
        }

        export type cls = typeof(setmetatable({}::classData, metatable))

        --returns a long string
        local new = function():cls
            return setmetatable({
                b = buffer.create(100_000::number);
                len = 0;
            }::classData,metatable)::cls
        end
        local class = new()

        class ..= "Hello"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_primitive_arith_no_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:115:type_infer_operators_primitive_arith_no_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_operators_primitive_arith_no_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_primitive_arith_no_metatable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function add(a: number, b: string)
            return a + (tonumber(b) :: number), tostring(a) .. b
        end
        local n, s = add(2,"3")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let function_type = fixture.base.require_type_string(&String::from("add"));
    let function_type =
      get_type_id::<FunctionType>(follow_type_id(function_type)).expect("expected FunctionType");
    let ret_type = first(function_type.ret_types(), false).expect("expected first return type");
    assert_eq!("number", to_string_type_id(ret_type));
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("n")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_operators_primitive_arith_no_metatable_with_follows {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:135:type_infer_operators_primitive_arith_no_metatable_with_follows`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_primitive_arith_no_metatable_with_follows

  #[cfg(test)]
  #[test]
  fn type_infer_operators_primitive_arith_no_metatable_with_follows() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local PI=3.1415926535897931
        local SOLAR_MASS=4*PI * PI
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let number_type = fixture.get_builtins().number_type;
    let solar_mass_type = follow_type_id(fixture.require_type_string(&String::from("SOLAR_MASS")));
    assert_eq!(number_type, solar_mass_type);
  }
}

mod type_infer_operators_primitive_arith_possible_metatable {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:146:type_infer_operators_primitive_arith_possible_metatable`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_primitive_arith_possible_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_operators_primitive_arith_possible_metatable() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function add(a: number, b: any)
            return a + b
        end
        local t = add(1,2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:365:type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not

  #[cfg(test)]
  #[test]
  fn type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not()
   {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local M = {}
        function M.new()
            return setmetatable({}, M)
        end
        function M.__lt(left, right) return true end
        type M = typeof(M.new())

        local a = M.new()
        local b = {}
        local c = a < b -- line 10
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
      "Types M and b cannot be compared with < because they do not have the same metatable",
      err.message()
    );
  }
}

mod type_infer_operators_reducing_and {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1428:type_infer_operators_reducing_and`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_operators_reducing_and

  #[cfg(test)]
  #[test]
  fn type_infer_operators_reducing_and() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo = { name: string?, flag: boolean? }
local arr: {Foo} = {}

local function foo(arg: {name: string}?)
    local name = if arg and arg.name then arg.name else nil

    table.insert(arr, {
        name = name or "",
        flag = name ~= nil and name ~= "",
    })
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_refine_and_or {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1063:type_infer_operators_refine_and_or`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_refine_and_or

  #[cfg(test)]
  #[test]
  fn type_infer_operators_refine_and_or() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: number?}? = {x = nil}
        local u = t and t.x or 5
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("u")))
    );
  }
}

mod type_infer_operators_reworked_and {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1368:type_infer_operators_reworked_and`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_reworked_and

  #[cfg(test)]
  #[test]
  fn type_infer_operators_reworked_and() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: number? = 5
local b: boolean = (a or 1) > 10
local c -- free

local x = a and 1
local y = 'a' and 1
local z = b and 1
local w = c and 1
    "#,
      ),
      None,
    );

    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "false | number",
        to_string_type_id(fixture.base.require_type_string(&String::from("z")))
      );
      assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("w")))
      );
    } else {
      assert_eq!(
        "boolean | number",
        to_string_type_id(fixture.base.require_type_string(&String::from("z")))
      );
      assert_eq!(
        "(boolean | number)?",
        to_string_type_id(fixture.base.require_type_string(&String::from("w")))
      );
    }
  }
}

mod type_infer_operators_reworked_or {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1394:type_infer_operators_reworked_or`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_reworked_or

  #[cfg(test)]
  #[test]
  fn type_infer_operators_reworked_or() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: number | false = 5
local b: number? = 6
local c: boolean = true
local d: true = true
local e: false = false
local f: nil = false

local a1 = a or 'a'
local b1 = b or 4
local c1 = c or 'c'
local d1 = d or 'd'
local e1 = e or 'e'
local f1 = f or 'f'
    "#,
      ),
      None,
    );

    assert_eq!(
      "number | string",
      to_string_type_id(fixture.base.require_type_string(&String::from("a1")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b1")))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string | true",
        to_string_type_id(fixture.base.require_type_string(&String::from("c1")))
      );
      assert_eq!(
        "string | true",
        to_string_type_id(fixture.base.require_type_string(&String::from("d1")))
      );
    } else {
      assert_eq!(
        "boolean | string",
        to_string_type_id(fixture.base.require_type_string(&String::from("c1")))
      );
      assert_eq!(
        "boolean | string",
        to_string_type_id(fixture.base.require_type_string(&String::from("d1")))
      );
    }
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("e1")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("f1")))
    );
  }
}

mod type_infer_operators_some_primitive_binary_ops {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:159:type_infer_operators_some_primitive_binary_ops`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_some_primitive_binary_ops

  #[cfg(test)]
  #[test]
  fn type_infer_operators_some_primitive_binary_ops() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 4 + 8
        local b = a + 9
        local s = 'hotdogs'
        local t = s .. s
        local c = b - a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_operators_strict_binary_op_where_lhs_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:816:type_infer_operators_strict_binary_op_where_lhs_unknown`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_strict_binary_op_where_lhs_unknown

  #[cfg(test)]
  #[test]
  fn type_infer_operators_strict_binary_op_where_lhs_unknown() {
    use alloc::{format, string::String};

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let ops = ["+", "-", "*", "/", "%", "^", ".."];
    let mut src = String::from("function foo(a, b)\n");

    for op in ops {
      src.push_str(&format!("local _ = a {} b\n", op));
    }

    src.push_str("end");

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(&src, None);

    assert_eq!(ops.len(), result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Operator '+' could not be applied to operands of types unknown and unknown; there is no corresponding overload for __add",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Operator '-' could not be applied to operands of types unknown and unknown; there is no corresponding overload for __sub",
        to_string_type_error(&result.errors[1])
      );
    } else {
      assert_eq!(
        "Unknown type used in + operation; consider adding a type annotation to 'a'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_strip_nil_from_lhs_or_operator {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:978:type_infer_operators_strip_nil_from_lhs_or_operator`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_strip_nil_from_lhs_or_operator

  #[cfg(test)]
  #[test]
  fn type_infer_operators_strip_nil_from_lhs_or_operator() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local a: number? = nil
local b: number = a or 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_strip_nil_from_lhs_or_operator_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:989:type_infer_operators_strip_nil_from_lhs_or_operator_2`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_strip_nil_from_lhs_or_operator_2

  #[cfg(test)]
  #[test]
  fn type_infer_operators_strip_nil_from_lhs_or_operator_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
local a: number? = nil
local b: number = a or 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:194:type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection

  #[cfg(test)]
  #[test]
  fn type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local Vec3 = {}
        Vec3.__index = Vec3
        function Vec3.new()
            return setmetatable({x=0, y=0, z=0}, Vec3)
        end

        export type Vec3 = typeof(Vec3.new())

        local thefun: any = function(self, o) return self end

        local multiply: ((Vec3, Vec3) -> Vec3) & ((Vec3, number) -> Vec3) = thefun

        Vec3.__mul = multiply

        local a = Vec3.new()
        local b = Vec3.new()
        local c = a * b
        local d = a * 2
        local e = a * 'cabbage'
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "mul<Vec3, string>"
    } else {
      "Vec3"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("e")))
    );
  }
}

mod type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:232:type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs

  #[cfg(test)]
  #[test]
  fn type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local Vec3 = {}
        Vec3.__index = Vec3
        function Vec3.new()
            return setmetatable({x=0, y=0, z=0}, Vec3)
        end

        export type Vec3 = typeof(Vec3.new())

        local thefun: any = function(self, o) return self end

        local multiply: ((Vec3, Vec3) -> Vec3) & ((Vec3, number) -> Vec3) = thefun

        Vec3.__mul = multiply

        local a = Vec3.new()
        local b = Vec3.new()
        local c = b * a
        local d = 2 * a
        local e = 'cabbage' * a
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "mul<string, Vec3>"
    } else {
      "Vec3"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("e")))
    );
  }
}

mod type_infer_operators_typecheck_unary_len_error {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:663:type_infer_operators_typecheck_unary_len_error`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_operators_typecheck_unary_len_error

  #[cfg(test)]
  #[test]
  fn type_infer_operators_typecheck_unary_len_error() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local mt = {}

        mt.__len = function(val): string
            return "test"
        end

        local foo = setmetatable({
            value = 10,
        }, mt)

        local a = #foo
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));
    assert_eq!("string", to_string_type_id(tm.given_type));
  }
}

mod type_infer_operators_typecheck_unary_minus {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:567:type_infer_operators_typecheck_unary_minus`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_operators_typecheck_unary_minus

  #[cfg(test)]
  #[test]
  fn type_infer_operators_typecheck_unary_minus() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        generic_error::GenericError, type_mismatch::TypeMismatch,
        uninhabited_type_function::UninhabitedTypeFunction,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local foo
        local mt = {}

        mt.__unm = function(val): string
            return tostring(val.value) .. "test"
        end

        foo = setmetatable({
            value = 10
        }, mt)

        local a = -foo

        local b = 1+-1

        local bar = {
            value = 10
        }
        local c = -bar -- disallowed
    "#,
      ),
      None,
    );

    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      let utf = type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
        .expect("expected UninhabitedTypeFunction");
      assert_eq!("unm<bar>", to_string_type_id(utf.ty()));

      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("bar", to_string_type_id(tm.given_type));
      assert_eq!("number", to_string_type_id(tm.wanted_type));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      let r#gen =
        type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
      assert_eq!(
        "Unary operator '-' not supported by type 'bar'",
        r#gen.message()
      );
    }
  }
}

mod type_infer_operators_typecheck_unary_minus_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:618:type_infer_operators_typecheck_unary_minus_error`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_typecheck_unary_minus_error

  #[cfg(test)]
  #[test]
  fn type_infer_operators_typecheck_unary_minus_error() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{type_mismatch::TypeMismatch, uninhabited_type_function::UninhabitedTypeFunction},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local mt = {}

        mt.__unm = function(val: boolean): string
            return "test"
        end

        local foo = setmetatable({
            value = 10
        }, mt)

        local a = -foo
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      assert_eq!(
        "unm<foo>",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
      );

      type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
        .expect("expected UninhabitedTypeFunction");

      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("(foo) -> unm<foo>", to_string_type_id(tm.wanted_type));
      assert_eq!("(boolean) -> string", to_string_type_id(tm.given_type));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
      );

      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("boolean", to_string_type_id(tm.wanted_type));
    }
  }
}

mod type_infer_operators_unary_not_is_boolean {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:694:type_infer_operators_unary_not_is_boolean`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_operators_unary_not_is_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_operators_unary_not_is_boolean() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local b = not "string"
        local c = not (math.random() > 0.5 and "string" or 7)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_operators_unknown_global_compound_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:933:type_infer_operators_unknown_global_compound_assign`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - translates_to -> rust_item type_infer_operators_unknown_global_compound_assign

  #[cfg(test)]
  #[test]
  fn type_infer_operators_unknown_global_compound_assign() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if FFlag::DebugLuauForceOldSolver.get() {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
                --!nonstrict
                a = a + 1
                print(a)
            "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown global 'a'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            --!strict
            a += 1
            print(a)
        "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'a'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );

    if FFlag::DebugLuauForceOldSolver.get() {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
                --!nonstrict
                a += 1
                print(a)
            "#,
        ),
        None,
      );

      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Unknown global 'a'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_operators_unknown_type_in_comparison {
  //! Ported from `tests/TypeInfer.operators.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:768:type_infer_operators_unknown_type_in_comparison`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_operators_unknown_type_in_comparison

  #[cfg(test)]
  #[test]
  fn type_infer_operators_unknown_type_in_comparison() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function merge(lower, greater)
            if lower.y == greater.y then
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_unrelated_extern_types_cannot_be_compared {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1306:type_infer_operators_unrelated_extern_types_cannot_be_compared`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_operators_unrelated_extern_types_cannot_be_compared

  #[cfg(test)]
  #[test]
  fn type_infer_operators_unrelated_extern_types_cannot_be_compared() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = BaseClass.New()
        local b = UnrelatedClass.New()

        local c = a == b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_operators_unrelated_primitives_cannot_be_compared {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1318:type_infer_operators_unrelated_primitives_cannot_be_compared`
  //! Source: `tests/TypeInfer.operators.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.operators.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.operators.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_operators_unrelated_primitives_cannot_be_compared

  #[cfg(test)]
  #[test]
  fn type_infer_operators_unrelated_primitives_cannot_be_compared() {
    use alloc::string::String;

    use ulua_analysis::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = 5 == true
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotCompareUnrelatedTypes>(&result.errors[0])
      .expect("expected CannotCompareUnrelatedTypes");
  }
}
