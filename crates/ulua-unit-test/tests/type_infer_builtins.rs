extern crate alloc;

mod type_infer_builtins_aliased_string_format {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:934:type_infer_builtins_aliased_string_format`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_aliased_string_format

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_aliased_string_format() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local fmt = string.format
        local s = fmt("%d", "oops")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_assert_removes_falsy_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1085:type_infer_builtins_assert_removes_falsy_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_assert_removes_falsy_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_assert_removes_falsy_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: (number | boolean)?)
            return assert(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "((boolean | number)?) -> number | true"
    } else {
      "((boolean | number)?) -> boolean | number"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_assert_removes_falsy_types_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1101:type_infer_builtins_assert_removes_falsy_types_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_assert_removes_falsy_types_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_assert_removes_falsy_types2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: (number | boolean)?): number | true
            return assert(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "((boolean | number)?) -> number | true",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_assert_removes_falsy_types_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1113:type_infer_builtins_assert_removes_falsy_types_3`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_builtins_assert_removes_falsy_types_3

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_assert_removes_falsy_types3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: (number | boolean)?)
            assert(x)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "((boolean | number)?) -> number | true"
    } else {
      "((boolean | number)?) -> boolean | number"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_assert_removes_falsy_types_even_from_type_pack_tail_but_only_for_the_first_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1129:type_infer_builtins_assert_removes_falsy_types_even_from_type_pack_tail_but_only_for_the_first_type`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_assert_removes_falsy_types_even_from_type_pack_tail_but_only_for_the_first_type

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_assert_removes_falsy_types_even_from_type_pack_tail_but_only_for_the_first_type()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(...: number?)
            return assert(...)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(...number?) -> (number, ...number?)",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_assert_returns_false_and_string_iff_it_knows_the_first_argument_cannot_be_truthy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1144:type_infer_builtins_assert_returns_false_and_string_iff_it_knows_the_first_argument_cannot_be_truthy`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_assert_returns_false_and_string_iff_it_knows_the_first_argument_cannot_be_truthy

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_assert_returns_false_and_string_iff_it_knows_the_first_argument_cannot_be_truthy()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: nil)
            return assert(x, "hmm")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(nil) -> (never, ...never)",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_bad_select_should_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:729:type_infer_builtins_bad_select_should_not_crash`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_bad_select_should_not_crash

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_bad_select_should_not_crash() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_error::to_string_type_error},
      records::count_mismatch::CountMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        do end
        local _ = function(l0,...)
        end
        local _ = function()
            _(_);
            _ += select(_())
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location {
          begin: Position {
            line: 6,
            column: 17,
          },
          end: Position {
            line: 6,
            column: 23,
          },
        },
        result.errors[0].location
      );
      let err = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
      assert_eq!(1, err.expected());
      assert_eq!(0, err.actual());
    } else if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Argument count mismatch. Function expects at least 1 argument, but none are specified",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Argument count mismatch. Function expects at least 1 argument, but none are specified",
        to_string_type_error(&result.errors[1])
      );
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Argument count mismatch. Function '_' expects at least 1 argument, but none are specified",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Argument count mismatch. Function 'select' expects 1 argument, but none are specified",
        to_string_type_error(&result.errors[1])
      );
    }
  }
}

mod type_infer_builtins_better_string_format_error_when_format_string_is_dynamic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1785:type_infer_builtins_better_string_format_error_when_format_string_is_dynamic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_builtins_better_string_format_error_when_format_string_is_dynamic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_better_string_format_error_when_format_string_is_dynamic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _dynamic_format_errors =
      ScopedFastFlag::new(&FFlag::LuauSilenceDynamicFormatStringErrors, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local fmt: string = "Hello, %s!"
        print(string.format(fmt, "hello"))
        print(string.format(fmt :: any, "hello")) -- no error
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "We cannot statically check the type of `string.format` when called with a format string that is not statically known.\nIf you'd like to use an unchecked `string.format` call, you can cast the format string to `any` using `:: any`.",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_buffer_is_a_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:522:type_infer_builtins_buffer_is_a_type`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_buffer_is_a_type

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_buffer_is_a_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local b = buffer.create(10)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "buffer",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_builtins_builtin_tables_sealed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:201:type_infer_builtins_builtin_tables_sealed`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_builtins_builtin_tables_sealed

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_builtin_tables_sealed() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState, functions::get_type_alt_j::get_type_id,
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local b = bit32
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let bit32 = fixture.base.require_type_string(&String::from("b"));
    assert!(!bit32.is_null());
    let bit32t = get_type_id::<TableType>(bit32).expect("expected bit32 table type");
    assert_eq!(TableState::Sealed, bit32t.state);
  }
}

mod type_infer_builtins_coroutine_resume_anything_goes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:532:type_infer_builtins_coroutine_resume_anything_goes`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function resume (VM/src/ldo.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_coroutine_resume_anything_goes

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_coroutine_resume_anything_goes() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function nifty(x, y)
            print(x, y)
            local z = coroutine.yield(1, 2)
            print(z)
            return 42
        end

        local co = coroutine.create(nifty)
        local x, y = coroutine.resume(co, 1, 2)
        local answer = coroutine.resume(co, 3)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_coroutine_wrap_anything_goes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:552:type_infer_builtins_coroutine_wrap_anything_goes`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_coroutine_wrap_anything_goes

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_coroutine_wrap_anything_goes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function nifty(x, y)
            print(x, y)
            local z = coroutine.yield(1, 2)
            print(z)
            return 42
        end

        local f = coroutine.wrap(nifty)
        local x, y = f(1, 2)
        local answer = f(3)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_debug_info_is_crazy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:920:type_infer_builtins_debug_info_is_crazy`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_debug_info_is_crazy

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_debug_info_is_crazy() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(co: thread, f: () -> ())
            -- debug.info takes thread?, level, options or function, options
            debug.info(1, "n")
            debug.info(co, 1, "n")
            debug.info(f, "n")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_debug_traceback_is_crazy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:902:type_infer_builtins_debug_traceback_is_crazy`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_debug_traceback_is_crazy

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_debug_traceback_is_crazy() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(co: thread)
            -- debug.traceback takes thread?, message?, level? - yes, all optional!
            debug.traceback()
            debug.traceback(nil, 1)
            debug.traceback("msg")
            debug.traceback("msg", 1)
            debug.traceback(co)
            debug.traceback(co, "msg")
            debug.traceback(co, "msg", 1)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_dont_add_definitions_to_persistent_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1058:type_infer_builtins_dont_add_definitions_to_persistent_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_dont_add_definitions_to_persistent_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_dont_add_definitions_to_persistent_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id, records::function_type::FunctionType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f = math.sin
        local function g(x) return math.sin(x) end
        f = g
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.base.require_type_string(&String::from("f"));
    let ftv = get_type_id::<FunctionType>(f_type).expect("expected function");
    assert!(unsafe { (*f_type).persistent });
    assert!(ftv.definition().is_none());

    let g_type = fixture.base.require_type_string(&String::from("g"));
    let gtv = get_type_id::<FunctionType>(g_type).expect("expected function");
    assert!(!unsafe { (*g_type).persistent });
    assert!(gtv.definition().is_some());
  }
}

mod type_infer_builtins_find_capture_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1595:type_infer_builtins_find_capture_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_find_capture_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_find_capture_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local d, e, a, b, c = string.find("This is a string", "(.()(%a+))")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for (name, expected) in [
      ("a", "string?"),
      ("b", "number?"),
      ("c", "string?"),
      ("d", "number?"),
      ("e", "number?"),
    ] {
      assert_eq!(
        expected,
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_find_capture_types_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1610:type_infer_builtins_find_capture_types_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_find_capture_types_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_find_capture_types2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local d, e, a, b, c = string.find("This is a string", "(.()(%a+))", "this should be a number")
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number?", to_string_type_id(mismatch.wanted_type));
    assert_eq!("string", to_string_type_id(mismatch.given_type));
    for (name, expected) in [
      ("a", "string?"),
      ("b", "number?"),
      ("c", "string?"),
      ("d", "number?"),
      ("e", "number?"),
    ] {
      assert_eq!(
        expected,
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_find_capture_types_3_type_infer_builtins_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1630:type_infer_builtins_find_capture_types_3`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_find_capture_types_3

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_find_capture_types3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local d, e, a, b, c = string.find("This is a string", "(.()(%a+))", 1, "this should be a bool")
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("boolean?", to_string_type_id(mismatch.wanted_type));
    assert_eq!("string", to_string_type_id(mismatch.given_type));
    for (name, expected) in [
      ("a", "string?"),
      ("b", "number?"),
      ("c", "string?"),
      ("d", "number?"),
      ("e", "number?"),
    ] {
      assert_eq!(
        expected,
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_find_capture_types_3_type_infer_builtins_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1650:type_infer_builtins_find_capture_types_3`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_find_capture_types_3

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_find_capture_types3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local d, e, a, b = string.find("This is a string", "(.()(%a+))", 1, true)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(2, mismatch.expected());
    assert_eq!(4, mismatch.actual());
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("e")))
    );
  }
}

mod type_infer_builtins_gcinfo {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:483:type_infer_builtins_gcinfo`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_gcinfo

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gcinfo() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n = gcinfo()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("n")))
    );
  }
}

mod type_infer_builtins_getfenv {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:493:type_infer_builtins_getfenv`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_builtins_getfenv

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_getfenv() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from("getfenv(1)"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_global_singleton_types_are_sealed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1411:type_infer_builtins_global_singleton_types_are_sealed`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function split (Common/src/StringUtils.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_global_singleton_types_are_sealed

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_global_singleton_types_are_sealed() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function f(x: string)
    local p = x:split('a')
    p = table.pack(table.unpack(p, 1, #p - 1))
    return p
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_gmatch_capture_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1435:type_infer_builtins_gmatch_capture_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = string.gmatch("This is a string", "(.()(%a+))")()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1448:type_infer_builtins_gmatch_capture_types_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = ("This is a string"):gmatch("(.()(%a+))")()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_balanced_escaped_parens {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1478:type_infer_builtins_gmatch_capture_types_balanced_escaped_parens`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_balanced_escaped_parens

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_balanced_escaped_parens() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c, d = string.gmatch("T(his) is a string", "((.)%b()())")()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(3, mismatch.expected());
    assert_eq!(4, mismatch.actual());
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_default_capture {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1461:type_infer_builtins_gmatch_capture_types_default_capture`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_default_capture

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_default_capture() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c, d = string.gmatch("T(his)() is a string", ".")()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(1, mismatch.expected());
    assert_eq!(4, mismatch.actual());
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1542:type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = string.gmatch("T(his)() is a string", ")")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> (...string)",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1553:type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_invalid_pattern_fallback_to_builtin2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = string.gmatch("T(his)() is a string", "[")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> (...string)",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_leading_end_bracket_is_part_of_set {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1527:type_infer_builtins_gmatch_capture_types_leading_end_bracket_is_part_of_set`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_leading_end_bracket_is_part_of_set

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_leading_end_bracket_is_part_of_set() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- An immediate right-bracket following a left-bracket is included within the set;
        -- thus, '[]]'' is the set containing ']', and '[]' is an invalid set missing an enclosing
        -- right-bracket. We detect an invalid set in this case and fall back to to default gmatch
        -- typing.
        local foo = string.gmatch("T[hi%]s]]]() is a string", "([]s)")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> (...string)",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1497:type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = string.gmatch("T(his)() is a string", "(T[()])()")()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(2, mismatch.expected());
    assert_eq!(3, mismatch.actual());
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_builtins_gmatch_capture_types_set_containing_lbracket {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1515:type_infer_builtins_gmatch_capture_types_set_containing_lbracket`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_set_containing_lbracket

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_capture_types_set_containing_lbracket() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b = string.gmatch("[[[", "()([[])")()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_builtins_gmatch_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:964:type_infer_builtins_gmatch_definition`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_gmatch_definition

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_gmatch_definition() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = ("hey"):gmatch("(.)(.)(.)")()

        for c in ("hey"):gmatch("(.)") do
            print(c:upper())
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_instantiation_works_on_builtins {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1925:type_infer_builtins_instantiation_works_on_builtins`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_instantiation_works_on_builtins

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_instantiation_works_on_builtins() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _instantiation = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = table.create<<string>>(4)
        local bar = table.unpack<<string>>({})
        local baz = table.find<<string>>({}, 1) -- should error
        assert<<string>>("asd", "lol")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_ipairs_iterator_should_infer_types_and_type_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:88:type_infer_builtins_ipairs_iterator_should_infer_types_and_type_check`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_ipairs_iterator_should_infer_types_and_type_check

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_ipairs_iterator_should_infer_types_and_type_check() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Map<K, V> = { [K]: V }
        local array: Map<number, string> = { "foo", "bar", "baz" }

        local it: (Map<number, string>, number) -> (number?, string), t: Map<number, string>, i: number = ipairs(array)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_lua_51_exported_globals_all_exist {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:213:type_infer_builtins_lua_51_exported_globals_all_exist`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> macro setupvalue (VM/src/lobject.h)
  //!   - calls -> function getn (VM/src/ltablib.cpp)
  //!   - calls -> function foreachi (VM/src/ltablib.cpp)
  //!   - calls -> function maxn (VM/src/ltablib.cpp)
  //!   - calls -> function foreach (VM/src/ltablib.cpp)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> function resume (VM/src/ldo.cpp)
  //!   - calls -> function load (Config/src/LuauConfig.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_lua_51_exported_globals_all_exist

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_lua_51_exported_globals_all_exist() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local v__G = _G
        local v_string_sub = string.sub
        local v_string_upper = string.upper
        local v_string_len = string.len
        local v_string_rep = string.rep
        local v_string_find = string.find
        local v_string_match = string.match
        local v_string_char = string.char
        local v_string_gmatch = string.gmatch
        local v_string_reverse = string.reverse
        local v_string_byte = string.byte
        local v_string_format = string.format
        local v_string_gsub = string.gsub
        local v_string_lower = string.lower

        local v_xpcall = xpcall

        --local v_package_loadlib = package.loadlib
        --local v_package_loaders_1_ = package.loaders[1]
        --local v_package_loaders_2_ = package.loaders[2]
        --local v_package_loaders_3_ = package.loaders[3]
        --local v_package_loaders_4_ = package.loaders[4]

        local v_tostring = tostring
        local v_print = print

        --local v_os_exit = os.exit
        --local v_os_setlocale = os.setlocale
        local v_os_date = os.date
        --local v_os_getenv = os.getenv
        local v_os_difftime = os.difftime
        --local v_os_remove = os.remove
        local v_os_time = os.time
        --local v_os_clock = os.clock
        --local v_os_tmpname = os.tmpname
        --local v_os_rename = os.rename
        --local v_os_execute = os.execute

        local v_unpack = unpack
        local v_require = require
        local v_getfenv = getfenv
        local v_setmetatable = setmetatable
        local v_next = next
        local v_assert = assert
        local v_tonumber = tonumber

        --local v_io_lines = io.lines
        --local v_io_write = io.write
        --local v_io_close = io.close
        --local v_io_flush = io.flush
        --local v_io_open = io.open
        --local v_io_output = io.output
        --local v_io_type = io.type
        --local v_io_read = io.read
        --local v_io_stderr = io.stderr
        --local v_io_stdin = io.stdin
        --local v_io_input = io.input
        --local v_io_stdout = io.stdout
        --local v_io_popen = io.popen
        --local v_io_tmpfile = io.tmpfile

        local v_rawequal = rawequal
        --local v_collectgarbage = collectgarbage
        local v_getmetatable = getmetatable
        local v_rawset = rawset

        local v_math_log = math.log
        local v_math_max = math.max
        local v_math_acos = math.acos
        local v_math_huge = math.huge
        local v_math_ldexp = math.ldexp
        local v_math_pi = math.pi
        local v_math_cos = math.cos
        local v_math_tanh = math.tanh
        local v_math_pow = math.pow
        local v_math_deg = math.deg
        local v_math_tan = math.tan
        local v_math_cosh = math.cosh
        local v_math_sinh = math.sinh
        local v_math_random = math.random
        local v_math_randomseed = math.randomseed
        local v_math_frexp = math.frexp
        local v_math_ceil = math.ceil
        local v_math_floor = math.floor
        local v_math_rad = math.rad
        local v_math_abs = math.abs
        local v_math_sqrt = math.sqrt
        local v_math_modf = math.modf
        local v_math_asin = math.asin
        local v_math_min = math.min
        --local v_math_mod = math.mod
        local v_math_fmod = math.fmod
        local v_math_log10 = math.log10
        local v_math_atan2 = math.atan2
        local v_math_exp = math.exp
        local v_math_sin = math.sin
        local v_math_atan = math.atan

        --local v_debug_getupvalue = debug.getupvalue
        --local v_debug_debug = debug.debug
        --local v_debug_sethook = debug.sethook
        --local v_debug_getmetatable = debug.getmetatable
        --local v_debug_gethook = debug.gethook
        --local v_debug_setmetatable = debug.setmetatable
        --local v_debug_setlocal = debug.setlocal
        --local v_debug_traceback = debug.traceback
        --local v_debug_setfenv = debug.setfenv
        --local v_debug_getinfo = debug.getinfo
        --local v_debug_setupvalue = debug.setupvalue
        --local v_debug_getlocal = debug.getlocal
        --local v_debug_getregistry = debug.getregistry
        --local v_debug_getfenv = debug.getfenv

        local v_pcall = pcall

        --local v_table_setn = table.setn
        local v_table_insert = table.insert
        --local v_table_getn = table.getn
        --local v_table_foreachi = table.foreachi
        local v_table_maxn = table.maxn
        --local v_table_foreach = table.foreach
        local v_table_concat = table.concat
        local v_table_sort = table.sort
        local v_table_remove = table.remove

        local v_newproxy = newproxy
        local v_type = type

        local v_coroutine_resume = coroutine.resume
        local v_coroutine_yield = coroutine.yield
        local v_coroutine_status = coroutine.status
        local v_coroutine_wrap = coroutine.wrap
        local v_coroutine_create = coroutine.create
        local v_coroutine_running = coroutine.running

        local v_select = select
        local v_gcinfo = gcinfo
        local v_pairs = pairs
        local v_rawget = rawget
        local v_loadstring = loadstring
        local v_ipairs = ipairs
        local v__VERSION = _VERSION
        --local v_dofile = dofile
        local v_setfenv = setfenv
        --local v_load = load
        local v_error = error
        --local v_loadfile = loadfile
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_match_capture_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1564:type_infer_builtins_match_capture_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_match_capture_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_match_capture_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = string.match("This is a string", "(.()(%a+))")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_builtins_match_capture_types_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1577:type_infer_builtins_match_capture_types_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_match_capture_types_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_match_capture_types2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b, c = string.match("This is a string", "(.()(%a+))", "this should be a number")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number?", to_string_type_id(mismatch.wanted_type));
    assert_eq!("string", to_string_type_id(mismatch.given_type));
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_builtins_math_max_checks_for_numbers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:191:type_infer_builtins_math_max_checks_for_numbers`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_math_max_checks_for_numbers

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_math_max_checks_for_numbers() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n = math.max(1,2,"3")
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_math_max_variatic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:181:type_infer_builtins_math_max_variatic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_math_max_variatic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_math_max_variatic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n = math.max(1,2,3,4,5,6,7,8,9,0)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("n")))
    );
  }
}

mod type_infer_builtins_math_things_are_defined {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:20:type_infer_builtins_math_things_are_defined`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item type_infer_builtins_math_things_are_defined

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_math_things_are_defined() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a00 = math.frexp
        local a01 = math.ldexp
        local a02 = math.fmod
        local a03 = math.modf
        local a04 = math.pow
        local a05 = math.exp
        local a06 = math.floor
        local a07 = math.abs
        local a08 = math.sqrt
        local a09 = math.log
        local a10 = math.log10
        local a11 = math.rad
        local a12 = math.deg
        local a13 = math.sin
        local a14 = math.cos
        local a15 = math.tan
        local a16 = math.sinh
        local a17 = math.cosh
        local a18 = math.tanh
        local a19 = math.atan
        local a20 = math.acos
        local a21 = math.asin
        local a22 = math.atan2
        local a23 = math.ceil
        local a24 = math.min
        local a25 = math.max
        local a26 = math.pi
        local a27 = math.huge
        local a28 = math.nan
        local a29 = math.e
        local a30 = math.phi
        local a31 = math.sqrt2
        local a32 = math.tau
        local a33 = math.randomseed
        local a34 = math.random
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_next_iterator_should_infer_types_and_type_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:63:type_infer_builtins_next_iterator_should_infer_types_and_type_check`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_next_iterator_should_infer_types_and_type_check

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_next_iterator_should_infer_types_and_type_check() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string, b: number = next({ 1 })

        local s = "foo"
        local t = { [s] = 1 }
        local c: string?, d: number = next(t)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_next_with_refined_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1874:type_infer_builtins_next_with_refined_any`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_builtins_next_with_refined_any

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_next_with_refined_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t: any = {"hello", "world"}
        if type(t) == "table" and next(t) then
            local foo, bar = next(t)
            local _ = foo
            local _ = bar
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 5,
        column: 23
      }))
    );
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 23
      }))
    );
  }
}

mod type_infer_builtins_no_persistent_typelevel_change {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1391:type_infer_builtins_no_persistent_typelevel_change`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_no_persistent_typelevel_change

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_no_persistent_typelevel_change() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_mutable_level::get_mutable_level, get_mutable_type::get_mutable_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{function_type::FunctionType, scope::Scope, table_type::TableType},
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let global_scope = fixture.get_frontend().globals.global_scope();
    let math_ty = unsafe {
      fixture.base.require_type_scope_ptr_string(
        global_scope.as_ref() as *const Scope as *mut Scope,
        &String::from("math"),
      )
    };

    let ttv = get_mutable_type_id::<TableType>(math_ty).expect("expected math table type");
    let frexp_ty = ttv
      .props
      .get("frexp")
      .and_then(|prop| prop.read_ty)
      .expect("expected math.frexp read type");
    assert!(get_type_id::<FunctionType>(frexp_ty).is_some());

    let original_level = unsafe {
      let level = get_mutable_level(frexp_ty);
      assert!(!level.is_null());
      *level
    };

    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from("local a = math.frexp"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let current_level = unsafe {
      let level = get_mutable_level(frexp_ty);
      assert!(!level.is_null());
      *level
    };
    assert_eq!(original_level, current_level);
  }
}

mod type_infer_builtins_os_time_takes_optional_date_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:498:type_infer_builtins_os_time_takes_optional_date_table`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item type_infer_builtins_os_time_takes_optional_date_table

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_os_time_takes_optional_date_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local n1 = os.time()
        local n2 = os.time({ year = 2020, month = 4, day = 20 })
        local n3 = os.time({ year = 2020, month = 4, day = 20, hour = 0, min = 0, sec = 0, isdst = true })
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for name in ["n1", "n2", "n3"] {
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_pairs_iterator_should_infer_types_and_type_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:76:type_infer_builtins_pairs_iterator_should_infer_types_and_type_check`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_pairs_iterator_should_infer_types_and_type_check

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_pairs_iterator_should_infer_types_and_type_check() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Map<K, V> = { [K]: V }
        local map: Map<string, number> = { ["foo"] = 1, ["bar"] = 2, ["baz"] = 3 }

        local it: (Map<string, number>, string | nil) -> (string?, number), t: Map<string, number>, i: nil = pairs(map)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_pairs_with_refined_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1894:type_infer_builtins_pairs_with_refined_any`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_builtins_pairs_with_refined_any

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_pairs_with_refined_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t: any = {"hello", "world"}
        if type(t) == "table" and pairs(t) then
	        local foo, bar, lorem = pairs(t)
            local _ = foo
            local _ = bar
            local _ = lorem
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({+ [unknown]: unknown +}, unknown?) -> (unknown?, unknown)",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 5,
        column: 23
      }))
    );
    assert_eq!(
      "{+ [unknown]: unknown +}",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 23
      }))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 23
      }))
    );
  }
}

mod type_infer_builtins_pcall_returns_at_least_two_value_but_function_returns_nothing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:675:type_infer_builtins_pcall_returns_at_least_two_value_but_function_returns_nothing`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_pcall_returns_at_least_two_value_but_function_returns_nothing

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_pcall_returns_at_least_two_value_but_function_returns_nothing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): () end
        local ok, res = pcall(f)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("ok")))
    );
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("res")))
    );
  }
}

mod type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1838:type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        if bit32.bnot then
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_read_refinements_on_persistent_tables_known_property_narrow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1860:type_infer_builtins_read_refinements_on_persistent_tables_known_property_narrow`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_builtins_read_refinements_on_persistent_tables_known_property_narrow

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_read_refinements_on_persistent_tables_known_property_narrow() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local myutf8 = utf8
        if myutf8.charpattern == "lol" then
            local foobar = myutf8.charpattern
            local _ = foobar
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "\"lol\"",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 4,
        column: 23
      }))
    );
  }
}

mod type_infer_builtins_read_refinements_on_persistent_tables_unknown_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1847:type_infer_builtins_read_refinements_on_persistent_tables_unknown_property`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_builtins_read_refinements_on_persistent_tables_unknown_property

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_read_refinements_on_persistent_tables_unknown_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        if bit32.scrambleEggs then
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      get_type_error::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("scrambleEggs", err.key());
    assert_eq!("typeof(bit32)", to_string_type_id(err.table()));
  }
}

mod type_infer_builtins_see_thru_select {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:700:type_infer_builtins_see_thru_select`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_see_thru_select

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_see_thru_select() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a:number, b:boolean = select(2,"hi", 10, true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_see_thru_select_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:709:type_infer_builtins_see_thru_select_count`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_see_thru_select_count

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_see_thru_select_count() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r##"
        local a = select("#","hi", 10, true)
    "##,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_select_on_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:977:type_infer_builtins_select_on_variadic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_select_on_variadic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_on_variadic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (number, ...(boolean | number))
            return 100, true, 1
        end

        local a, b, c = select(f())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for name in ["a", "b", "c"] {
      assert_eq!(
        "any",
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_select_slightly_out_of_range {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:785:type_infer_builtins_select_slightly_out_of_range`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_builtins_select_slightly_out_of_range

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_slightly_out_of_range() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_error::to_string_type_error},
      records::generic_error::GenericError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        select(3, "a", 1)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(get_type_error::<GenericError>(&result.errors[0]).is_some());
    assert_eq!(
      "bad argument #1 to select (index out of range)",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_select_way_out_of_range {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:770:type_infer_builtins_select_way_out_of_range`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_builtins_select_way_out_of_range

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_way_out_of_range() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_error::to_string_type_error},
      records::generic_error::GenericError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        select(5432598430953240958)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(get_type_error::<GenericError>(&result.errors[0]).is_some());
    assert_eq!(
      "bad argument #1 to select (index out of range)",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_select_with_decimal_argument_is_rounded_down {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:719:type_infer_builtins_select_with_decimal_argument_is_rounded_down`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_select_with_decimal_argument_is_rounded_down

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_with_decimal_argument_is_rounded_down() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number, b: boolean = select(2.9, "foo", 1, true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_select_with_variadic_typepack_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:800:type_infer_builtins_select_with_variadic_typepack_tail`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_select_with_variadic_typepack_tail

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_with_variadic_typepack_tail() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function f(...)
            return ...
        end

        local foo, bar, baz, quux = select(1, f("foo", true))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for name in ["foo", "bar", "baz", "quux"] {
      assert_eq!(
        "any",
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_select_with_variadic_typepack_tail_and_string_head {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:819:type_infer_builtins_select_with_variadic_typepack_tail_and_string_head`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_select_with_variadic_typepack_tail_and_string_head

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_select_with_variadic_typepack_tail_and_string_head() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function f(...)
            return ...
        end

        local foo, bar, baz, quux = select(1, "foo", f("bar", true))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for name in ["foo", "bar", "baz", "quux"] {
      assert_eq!(
        "any",
        to_string_type_id(fixture.base.require_type_string(&String::from(name)))
      );
    }
  }
}

mod type_infer_builtins_set_metatable_needs_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1344:type_infer_builtins_set_metatable_needs_arguments`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_set_metatable_needs_arguments

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_set_metatable_needs_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {b=setmetatable}
        a.b()
        a:b()
        a:b({})
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'a.b' expects 2 arguments, but none are specified",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Argument count mismatch. Function 'a.b' expects 2 arguments, but only 1 is specified",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_builtins_setmetatable_on_union_of_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:378:type_infer_builtins_setmetatable_on_union_of_tables`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_setmetatable_on_union_of_tables

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_setmetatable_on_union_of_tables() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {tag: "A", x: number}
        type B = {tag: "B", y: string}

        type T = A | B

        type X = typeof(
            setmetatable({} :: T, {})
        )
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ @metatable {  }, A } | { @metatable {  }, B }"
    } else {
      "{ @metatable {|  |}, A } | { @metatable {|  |}, B }"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_alias(&String::from("X")))
    );
  }
}

mod type_infer_builtins_setmetatable_should_not_mutate_persisted_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:571:type_infer_builtins_setmetatable_should_not_mutate_persisted_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_builtins_setmetatable_should_not_mutate_persisted_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_setmetatable_should_not_mutate_persisted_types() {
    use alloc::string::String;

    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::table_type::TableType};
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local string = string

        setmetatable(string, {})
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let string_type = fixture.base.require_type_string(&String::from("string"));
    assert!(get_type_id::<TableType>(string_type).is_some());
  }
}

mod type_infer_builtins_setmetatable_unpacks_arg_types_correctly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:370:type_infer_builtins_setmetatable_unpacks_arg_types_correctly`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_setmetatable_unpacks_arg_types_correctly

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_setmetatable_unpacks_arg_types_correctly() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        setmetatable({}, setmetatable({}, {}))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_sort {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:121:type_infer_builtins_sort`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_sort

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_sort() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {1, 2, 3};
        table.sort(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_sort_with_bad_predicate {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:143:type_infer_builtins_sort_with_bad_predicate`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_builtins_sort_with_bad_predicate

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_sort_with_bad_predicate() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = {'one', 'two', 'three'}
        local function p(a: number, b: number) return a < b end
        table.sort(t, p)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t'((string, string) -> boolean)?'\nbut got\n\t'(number, number) -> boolean'\ncaused by:\n  None of the union options are compatible. For example:\nExpected this to be\n\t'(string, string) -> boolean'\nbut got\n\t'(number, number) -> boolean'\ncaused by:\n  Argument #1 type is not compatible.\nExpected this to be 'number', but got 'string'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_builtins_sort_with_predicate {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:131:type_infer_builtins_sort_with_predicate`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_sort_with_predicate

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_sort_with_predicate() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = {1, 2, 3}
        local function p(a: number, b: number) return a < b end
        table.sort(t, p)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_find_should_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1668:type_infer_builtins_string_find_should_not_crash`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_find_should_not_crash

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_find_should_not_crash() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function StringSplit(input, separator)
            string.find(input, separator)
            if not separator then
                separator = "%s+"
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_format_arg_count_mismatch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:602:type_infer_builtins_string_format_arg_count_mismatch`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_arg_count_mismatch

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_arg_count_mismatch() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        string.format("%f %d %s")
        string.format("%s", "hi", 42)
        string.format("%s", "hi", 42, ...)
        string.format("%s", "hi", ...)
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(2, result.errors[0].location.begin.line);
    assert_eq!(3, result.errors[1].location.begin.line);
    assert_eq!(4, result.errors[2].location.begin.line);
  }
}

mod type_infer_builtins_string_format_arg_types_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:589:type_infer_builtins_string_format_arg_types_inference`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_arg_types_inference

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_arg_types_inference() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function f(a, b, c)
            return string.format("%f %d %s", a, b, c)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number, number, string) -> string",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_string_format_as_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:842:type_infer_builtins_string_format_as_method`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_as_method

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_as_method() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture
      .check_string_optional_frontend_options(&String::from("local _ = ('%s'):format(5)"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().string_type, mismatch.wanted_type);
    assert_eq!(fixture.get_builtins().number_type, mismatch.given_type);
  }
}

mod type_infer_builtins_string_format_correctly_ordered_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:618:type_infer_builtins_string_format_correctly_ordered_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_correctly_ordered_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_correctly_ordered_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        string.format("%s", 123)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      fixture.base.get_builtins().string_type,
      mismatch.wanted_type
    );
    assert_eq!(fixture.base.get_builtins().number_type, mismatch.given_type);
  }
}

mod type_infer_builtins_string_format_report_all_type_errors_at_correct_positions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:993:type_infer_builtins_string_format_report_all_type_errors_at_correct_positions`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> type_alias TypeErrorData (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_report_all_type_errors_at_correct_positions

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_report_all_type_errors_at_correct_positions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        ("%s%d%s"):format(1, "hello", true)
        string.format("%s%d%s", 1, "hello", true)
    "#,
      ),
      None,
    );

    assert_eq!(6, result.errors.len(), "{:?}", result.errors);

    let expected = [
      (
        Location {
          begin: Position {
            line: 1,
            column: 26,
          },
          end: Position {
            line: 1,
            column: 27,
          },
        },
        "Expected this to be 'string', but got 'number'",
      ),
      (
        Location {
          begin: Position {
            line: 1,
            column: 29,
          },
          end: Position {
            line: 1,
            column: 36,
          },
        },
        "Expected this to be 'number', but got 'string'",
      ),
      (
        Location {
          begin: Position {
            line: 1,
            column: 38,
          },
          end: Position {
            line: 1,
            column: 42,
          },
        },
        "Expected this to be 'string', but got 'boolean'",
      ),
      (
        Location {
          begin: Position {
            line: 2,
            column: 32,
          },
          end: Position {
            line: 2,
            column: 33,
          },
        },
        "Expected this to be 'string', but got 'number'",
      ),
      (
        Location {
          begin: Position {
            line: 2,
            column: 35,
          },
          end: Position {
            line: 2,
            column: 42,
          },
        },
        "Expected this to be 'number', but got 'string'",
      ),
      (
        Location {
          begin: Position {
            line: 2,
            column: 44,
          },
          end: Position {
            line: 2,
            column: 48,
          },
        },
        "Expected this to be 'string', but got 'boolean'",
      ),
    ];

    for (index, (location, message)) in expected.into_iter().enumerate() {
      assert_eq!(location, result.errors[index].location);
      assert_eq!(message, to_string_type_error(&result.errors[index]));
    }
  }
}

mod type_infer_builtins_string_format_should_support_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1740:type_infer_builtins_string_format_should_support_any`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_should_support_any

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_should_support_any() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: any = "world"
        print(string.format("Hello, %s!", x))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_format_should_support_any_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1752:type_infer_builtins_string_format_should_support_any_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_should_support_any_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_should_support_any_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local fmt = "Hello, %s!" :: any
        local x = "world" :: any
        print(string.format(fmt, x))
        print(string.format(fmt, "hello"))
        print(string.format(fmt, 5)) -- unchecked because the format string is `any`!
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_format_should_support_singleton_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1767:type_infer_builtins_string_format_should_support_singleton_types`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_should_support_singleton_types

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_should_support_singleton_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local fmt: "Hello, %s!" = "Hello, %s!"
        print(string.format(fmt, "hello"))
        print(string.format(fmt, 5)) -- should still produce an error since the expected type is `string`!
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(mismatch.wanted_type));
    assert_eq!("number", to_string_type_id(mismatch.given_type));
  }
}

mod type_infer_builtins_string_format_tostring_specifier {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:636:type_infer_builtins_string_format_tostring_specifier`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_tostring_specifier

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_tostring_specifier() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        string.format("%* %* %* %*", "string", 1, true, function() end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_format_tostring_specifier_type_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:646:type_infer_builtins_string_format_tostring_specifier_type_constraint`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_tostring_specifier_type_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_tostring_specifier_type_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x): string
            local _ = string.format("%*", x)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(string) -> string",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_builtins_string_format_trivial_arity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:854:type_infer_builtins_string_format_trivial_arity`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_trivial_arity

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_trivial_arity() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        string.format()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'string.format' expects at least 1 argument, but none are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_string_format_use_correct_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:865:type_infer_builtins_string_format_use_correct_argument`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_use_correct_argument

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_use_correct_argument() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = ("%s"):format("%d", "hello")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function expects 2 arguments, but 3 are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_string_format_use_correct_argument_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:876:type_infer_builtins_string_format_use_correct_argument_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_use_correct_argument_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_use_correct_argument2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = ("%s %d").format("%d %s", "A type error", 2)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_builtins_string_format_use_correct_argument_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:888:type_infer_builtins_string_format_use_correct_argument_3`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_format_use_correct_argument_3

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_format_use_correct_argument3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s1 = string.format("%d")
        local s2 = string.format("%d", 1)
        local s3 = string.format("%d", 1, 2)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function expects 2 arguments, but only 1 is specified",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Argument count mismatch. Function expects 2 arguments, but 3 are specified",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_builtins_string_lib_self_noself {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:945:type_infer_builtins_string_lib_self_noself`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> function split (Common/src/StringUtils.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_lib_self_noself

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_lib_self_noself() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local a1 = string.byte("abcdef", 2)
        local a2 = string.find("abcdef", "def")
        local a3 = string.gmatch("ab ab", "%a+")
        local a4 = string.gsub("abab", "ab", "cd")
        local a5 = string.len("abc")
        local a6 = string.match("12 ab", "%d+ %a+")
        local a7 = string.rep("a", 10)
        local a8 = string.sub("abcd", 1, 2)
        local a9 = string.split("a,b,c", ",")
        local a0 = string.packsize("ff")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_string_match {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1424:type_infer_builtins_string_match`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_string_match

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_string_match() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s: string = "hello"
        local p = s:match("foo")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("p")))
    );
  }
}

mod type_infer_builtins_strings_have_methods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:171:type_infer_builtins_strings_have_methods`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_strings_have_methods

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_strings_have_methods() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = ("RoactHostChangeEvent(%s)"):format("hello")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_builtins_table_clone_intersection_of_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1360:type_infer_builtins_table_clone_intersection_of_tables`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_clone_intersection_of_tables

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_clone_intersection_of_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type FIRST = {
            some: string,
        }

        type SECOND = FIRST & {
            thing: string,
        }

        local b: SECOND
        -- c's type used to be FIRST, but should be the full type of SECOND
        local c = table.clone(b)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let c_type = fixture.base.require_type_string(&String::from("c"));
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ some: string } & { thing: string }",
      to_string_type_id_to_string_options(c_type, &mut opts)
    );
    assert_eq!("FIRST & { thing: string }", to_string_type_id(c_type));
  }
}

mod type_infer_builtins_table_clone_persistent_skip {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1317:type_infer_builtins_table_clone_persistent_skip`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_clone_persistent_skip

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_clone_persistent_skip() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.clone(table)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_clone_should_not_break {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1706:type_infer_builtins_table_clone_should_not_break`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_clone_should_not_break

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_clone_should_not_break() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Immutable = {}

        function Immutable.Set(dictionary, key, value)
            local new = table.clone(dictionary)

            new[key] = value

            return new
        end

        return Immutable
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_clone_should_not_break_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1725:type_infer_builtins_table_clone_should_not_break_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_clone_should_not_break_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_clone_should_not_break_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function set(dictionary, key, value)
            local new = table.clone(dictionary)

            new[key] = value

            return new
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_clone_should_support_variadic_any_in_old_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1326:type_infer_builtins_table_clone_should_support_variadic_any_in_old_solver`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_clone_should_support_variadic_any_in_old_solver

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_clone_should_support_variadic_any_in_old_solver() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!nonstrict
        return function()
            return {}
        end
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local A = require(game.A)
        local _ = table.clone(A())
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_concat_returns_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:111:type_infer_builtins_table_concat_returns_string`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_concat_returns_string

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_concat_returns_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local r = table.concat({1,2,3,4}, ",", 2);
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("r")))
    );
  }
}

mod type_infer_builtins_table_dot_clone_type_states {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1682:type_infer_builtins_table_dot_clone_type_states`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_dot_clone_type_states

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_dot_clone_type_states() {
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
        local t1 = {}
        t1.x = 5
        local t2 = table.clone(t1)
        t2.y = 6
        t1.z = 3
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected_t1 = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ x: number, z: number }"
    } else {
      "{| x: number, z: number |}"
    };
    let expected_t2 = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ x: number, y: number }"
    } else {
      "{| x: number, y: number |}"
    };

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      expected_t1,
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("t1")),
        &mut opts
      )
    );
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      expected_t2,
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("t2")),
        &mut opts
      )
    );
  }
}

mod type_infer_builtins_table_dot_remove_optionally_returns_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:100:type_infer_builtins_table_dot_remove_optionally_returns_generic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_dot_remove_optionally_returns_generic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_dot_remove_optionally_returns_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = { 1 }
        local n = table.remove(t, 7)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("n")))
    );
  }
}

mod type_infer_builtins_table_freeze_does_not_retroactively_block_mutation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1212:type_infer_builtins_table_freeze_does_not_retroactively_block_mutation`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_does_not_retroactively_block_mutation

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_does_not_retroactively_block_mutation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t1 = {a = 42}

        t1.q = ":3"

        local tf1 = table.freeze(t1)

        local a = tf1.a
        local b = t1.a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let mut opts = ToStringOptions::new(true);
      assert_eq!(
        "{ read a: number, read q: string }",
        to_string_type_id_to_string_options(
          fixture.base.require_type_string(&String::from("t1")),
          &mut opts
        )
      );

      let mut opts = ToStringOptions::new(true);
      assert_eq!(
        "{ a: number, q: string }",
        to_string_type_id_to_string_options(
          fixture
            .base
            .require_type_at_position_position(Position { line: 3, column: 8 }),
          &mut opts
        )
      );

      let mut opts = ToStringOptions::new(true);
      assert_eq!(
        "{ read a: number, read q: string }",
        to_string_type_id_to_string_options(
          fixture.base.require_type_at_position_position(Position {
            line: 8,
            column: 18,
          }),
          &mut opts
        )
      );
    }

    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_builtins_table_freeze_errors_on_no_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1277:type_infer_builtins_table_freeze_errors_on_no_args`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_errors_on_no_args

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_errors_on_no_args() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        table.freeze()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(get_type_error::<CountMismatch>(&result.errors[0]).is_some());
  }
}

mod type_infer_builtins_table_freeze_errors_on_non_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1289:type_infer_builtins_table_freeze_errors_on_non_tables`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_errors_on_non_tables

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_errors_on_non_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        table.freeze(42)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    let expected_wanted = if !FFlag::DebugLuauForceOldSolver.get() {
      "table"
    } else {
      "{-  -}"
    };
    assert_eq!(expected_wanted, to_string_type_id(mismatch.wanted_type));
    assert_eq!("number", to_string_type_id(mismatch.given_type));
  }
}

mod type_infer_builtins_table_freeze_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:2064:type_infer_builtins_table_freeze_function`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_function

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(f: () -> ())
            table.freeze(f())
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'table.freeze' expects 1 argument, but none are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_table_freeze_generic_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:2046:type_infer_builtins_table_freeze_generic_pack`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_generic_pack

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_generic_pack() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_d::to_string_type_pack_id,
      records::type_pack_mismatch::TypePackMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo<T...>(...: T...)
            table.freeze(...)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tpm = type_error_data_ref::<TypePackMismatch>(&result.errors[0])
      .expect("expected TypePackMismatch");
    assert_eq!("table", to_string_type_pack_id(tpm.wanted_tp()));
    assert_eq!("T...", to_string_type_pack_id(tpm.given_tp()));
  }
}

mod type_infer_builtins_table_freeze_is_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1162:type_infer_builtins_table_freeze_is_generic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_is_generic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_is_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t1: {a: number} = {a = 42}
        local t2: {b: string} = {b = "hello"}
        local t3: {boolean} = {false, true}

        local tf1 = table.freeze(t1)
        local tf2 = table.freeze(t2)
        local tf3 = table.freeze(t3)

        local a = tf1.a
        local b = tf2.b
        local c = tf3[2]

        local d = tf1.b

        local a2 = t1.a
        local b2 = t2.b
        local c2 = t3[2]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected_error = if !FFlag::DebugLuauForceOldSolver.get() {
      "Key 'b' not found in table '{ read a: number }'"
    } else {
      "Key 'b' not found in table '{ a: number }'"
    };
    assert_eq!(expected_error, to_string_type_error(&result.errors[0]));
    assert_eq!(
      Location {
        begin: Position {
          line: 13,
          column: 18,
        },
        end: Position {
          line: 13,
          column: 23,
        },
      },
      result.errors[0].location
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ read a: number }",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 15,
          column: 19,
        }))
      );
      assert_eq!(
        "{ read b: string }",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 16,
          column: 19,
        }))
      );
      assert_eq!(
        "{boolean}",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 17,
          column: 19,
        }))
      );
    }

    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    let expected_d = if !FFlag::DebugLuauForceOldSolver.get() {
      "any"
    } else {
      "*error-type*"
    };
    assert_eq!(
      expected_d,
      to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a2")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("b2")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("c2")))
    );
  }
}

mod type_infer_builtins_table_freeze_no_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1972:type_infer_builtins_table_freeze_no_args`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_no_args

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_no_args() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.freeze()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'table.freeze' expects 1 argument, but none are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_table_freeze_no_generic_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1240:type_infer_builtins_table_freeze_no_generic_table`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_no_generic_table

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_no_generic_table() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type k = {
            read k: string,
        }

        function _(): k
            return table.freeze({
                k = "",
            })
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_on_any_should_not_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1942:type_infer_builtins_table_freeze_on_any_should_not_error`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_on_any_should_not_error

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_on_any_should_not_error() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(): any
            return true
        end

        table.freeze(foo())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_on_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1260:type_infer_builtins_table_freeze_on_metatable`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::meta (tests/Subtyping.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_on_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_on_metatable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local meta = {
            __index = function()
                return "foo"
            end
        }

        local myTable = setmetatable({}, meta)
        table.freeze(myTable)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_persistent_skip {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1308:type_infer_builtins_table_freeze_persistent_skip`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_persistent_skip

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_persistent_skip() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.freeze(table)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_with_type_check_should_not_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1957:type_infer_builtins_table_freeze_with_type_check_should_not_error`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_with_type_check_should_not_error

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_with_type_check_should_not_error() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function maybeFreeze(t: any)
            if type(t) == "table" then
                table.freeze(t)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_with_type_pack_should_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1984:type_infer_builtins_table_freeze_with_type_pack_should_error`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_with_type_pack_should_error

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_with_type_pack_should_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.freeze({x = 5}, {y = "hello"})
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'table.freeze' expects 1 argument, but 2 are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_table_freeze_with_variadic_any_should_not_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1996:type_infer_builtins_table_freeze_with_variadic_any_should_not_error`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_with_variadic_any_should_not_error

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_with_variadic_any_should_not_error() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function bar(): ...any
            return true
        end

        table.freeze(bar())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_freeze_with_variadic_non_error_suppressing_should_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:2011:type_infer_builtins_table_freeze_with_variadic_non_error_suppressing_should_error`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_freeze_with_variadic_non_error_suppressing_should_error

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_freeze_with_variadic_non_error_suppressing_should_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauTableFreezeCheckIsSubtype, true),
    ];
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function bar(): ...string
            return
        end

        table.freeze(bar())
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'table', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_table_insert_correctly_infers_type_of_array_2_args_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:398:type_infer_builtins_table_insert_correctly_infers_type_of_array_2_args_overload`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_insert_correctly_infers_type_of_array_2_args_overload

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_insert_correctly_infers_type_of_array_2_args_overload() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        table.insert(t, "foo")
        local s = t[1]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = fixture.base.get_builtins().string_type;
    let actual = fixture.base.require_type_string(&String::from("s"));
    assert_eq!(expected, actual);
  }
}

mod type_infer_builtins_table_insert_correctly_infers_type_of_array_3_args_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:410:type_infer_builtins_table_insert_correctly_infers_type_of_array_3_args_overload`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_insert_correctly_infers_type_of_array_3_args_overload

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_insert_correctly_infers_type_of_array_3_args_overload() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        table.insert(t, 1, "foo")
        local s = t[1]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_builtins_table_insert_into_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1816:type_infer_builtins_table_insert_into_any`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_builtins_table_insert_into_any

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_insert_into_any() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
table.insert(1::any, 2::any)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_insert_requires_all_fields {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1823:type_infer_builtins_table_insert_requires_all_fields`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_insert_requires_all_fields

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_insert_requires_all_fields() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function huh(): { { x: number, y: string } }
            local ret = {}
            while true do
                table.insert(ret, { x = 42 })
            end
            return ret
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_table_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:422:type_infer_builtins_table_pack`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_pack

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = table.pack(1, "foo", true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ [number]: boolean | number | string, n: number }",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_builtins_table_pack_reduce_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:447:type_infer_builtins_table_pack_reduce_1`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_table_pack_reduce_1

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_pack_reduce_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = table.pack(1, 2, true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ [number]: boolean | number, n: number }",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_builtins_table_pack_reduce_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:457:type_infer_builtins_table_pack_reduce_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_pack_reduce_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_pack_reduce_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = table.pack("a", "b", "c")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.base.require_type_string(&String::from("t"));
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ [number]: string | string | string, n: number }"
    } else {
      "{ [number]: string, n: number }"
    };
    assert_eq!(expected, to_string_type_id(ty));
  }
}

mod type_infer_builtins_table_pack_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:432:type_infer_builtins_table_pack_variadic`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_table_pack_variadic

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_table_pack_variadic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
function f(): (string, ...number)
    return "str", 2, 3, 4
end

local t = table.pack(f())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ [number]: number | string, n: number }",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_builtins_thread_is_a_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:512:type_infer_builtins_thread_is_a_type`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_thread_is_a_type

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_thread_is_a_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local co = coroutine.create(function() end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "thread",
      to_string_type_id(fixture.base.require_type_string(&String::from("co")))
    );
  }
}

mod type_infer_builtins_tonumber_returns_optional_number_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1025:type_infer_builtins_tonumber_returns_optional_number_type`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_infer_builtins_tonumber_returns_optional_number_type

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_tonumber_returns_optional_number_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local b: number = tonumber('asdf')
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be 'number', but got 'number?'; \n\
the 2nd component of the union is `nil`, which is not a subtype of `number`"
    } else {
      "Expected this to be 'number', but got 'number?'"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_builtins_tonumber_returns_optional_number_type_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1048:type_infer_builtins_tonumber_returns_optional_number_type_2`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_infer_builtins_tonumber_returns_optional_number_type_2

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_tonumber_returns_optional_number_type2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local b: number = tonumber('asdf') or 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_trivial_select {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:691:type_infer_builtins_trivial_select`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_trivial_select

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_trivial_select() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a:number = select(1, 42)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_typeof_unresolved_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1382:type_infer_builtins_typeof_unresolved_function`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_typeof_unresolved_function

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_typeof_unresolved_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: typeof(f)) end
        "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'f'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_builtins_variadic_return_to_single_parameter_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:2028:type_infer_builtins_variadic_return_to_single_parameter_function`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_variadic_return_to_single_parameter_function

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_variadic_return_to_single_parameter_function() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function bar(): ...string
            return
        end

        local function foo(x: string)
            print(x) -- nil
        end

        foo(bar())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_vector_lerp_should_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1916:type_infer_builtins_vector_lerp_should_not_crash`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_builtins_vector_lerp_should_not_crash

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_vector_lerp_should_not_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function half(x: number, y: number, z: number): vector
            return vector.lerp(vector.zero, vector.create(x, y, z), 0.5)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_write_only_table_assertion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1804:type_infer_builtins_write_only_table_assertion`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_builtins_write_only_table_assertion

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_write_only_table_assertion() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function accept(t: { write foo: number })
        end

        accept({ foo = "lol", foo = true })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_builtins_xpcall {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:659:type_infer_builtins_xpcall`
  //! Source: `tests/TypeInfer.builtins.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_builtins_xpcall

  #[cfg(test)]
  #[test]
  fn type_infer_builtins_xpcall() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local a, b, c = xpcall(
            function() return 5, true end,
            function(e) return 0, false end
        )
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}
