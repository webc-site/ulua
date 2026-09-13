extern crate alloc;

mod type_infer_const_assign_different_values_to_const_x {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:131:type_infer_const_assign_different_values_to_const_x`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item type_infer_const_assign_different_values_to_const_x

  #[cfg(test)]
  #[test]
  fn type_infer_const_assign_different_values_to_const_x() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::syntax_error::SyntaxError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const x: string? = nil
        local a = x
        x = "hello!"
        local b = x
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "Variable 'x' is constant and may not be reassigned",
      err.message()
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_const_basic_declarations_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:16:type_infer_const_basic_declarations_work`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_const_basic_declarations_work

  #[cfg(test)]
  #[test]
  fn type_infer_const_basic_declarations_work() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const PI = 3.14
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("PI")))
    );
  }
}

mod type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:66:type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::count_mismatch::CountMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getparams(): (number, number)
            return 42, 13
        end

        const X, Y, Z = getparams()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(3, err.actual());
    assert_eq!(2, err.expected());
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("X")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("Y")))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_string(&String::from("Z")))
    );
  }
}

mod type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:91:type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill() {
    use alloc::string::String;

    use ulua_analysis::{functions::get_error::get_type_error, records::syntax_error::SyntaxError};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const X, Y, Z = 42, 13

        return { X, Y, Z }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!("Missing initializer in const declaration", err.message());
  }
}

mod type_infer_const_const_recursive_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:151:type_infer_const_const_recursive_function_works`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_const_const_recursive_function_works

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_recursive_function_works() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const function f(x)
            f(5)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(unknown) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(
        "(number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    }
  }
}

mod type_infer_const_const_shadowing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:191:type_infer_const_const_shadowing`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_const_const_shadowing

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_shadowing() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const X = "huh"
        const X = 3.14

        local y = X
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_const_const_syntax_error_in_annotation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:112:type_infer_const_const_syntax_error_in_annotation`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_const_const_syntax_error_in_annotation

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_syntax_error_in_annotation() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const foo: {
            bar
            baz
        } = {}

        return foo
    "#,
      ),
      None,
    );
  }
}

mod type_infer_const_const_tables_are_still_mutable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:168:type_infer_const_const_tables_are_still_mutable`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_const_const_tables_are_still_mutable

  #[cfg(test)]
  #[test]
  fn type_infer_const_const_tables_are_still_mutable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        const TABLE = {}
        TABLE.foobar = "the fooest of bars!"
        TABLE.TAU = 6.12
        function TABLE.callback(x, y)
            print(math.abs(x), string.len(y))
            return true
        end

        return TABLE
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let table_ty = fixture.base.require_type_string(&String::from("TABLE"));
    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{| TAU: number, callback: (number, string) -> boolean, foobar: string |}",
        to_string_type_id_to_string_options(table_ty, &mut opts)
      );
    } else {
      assert_eq!(
        "{ TAU: number, callback: (number, string) -> boolean, foobar: string }",
        to_string_type_id_to_string_options(table_ty, &mut opts)
      );
    }
  }
}

mod type_infer_const_empty_domain_is_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:44:type_infer_const_empty_domain_is_ok`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_const_empty_domain_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_const_empty_domain_is_ok() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::syntax_error::SyntaxError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const PI

        return PI
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!("Missing initializer in const declaration", err.message());
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_string(&String::from("PI")))
    );
  }
}

mod type_infer_const_reassignments_dont_affect_type_state {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:27:type_infer_const_reassignments_dont_affect_type_state`
  //! Source: `tests/TypeInfer.const.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.const.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.const.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item type_infer_const_reassignments_dont_affect_type_state

  #[cfg(test)]
  #[test]
  fn type_infer_const_reassignments_dont_affect_type_state() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::syntax_error::SyntaxError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        const PI = 3.14
        PI = "apple"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "Variable 'PI' is constant and may not be reassigned",
      err.message()
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("PI")))
    );
  }
}
