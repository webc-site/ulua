extern crate alloc;

mod type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3797:type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - type_ref -> record MultipleNonviableOverloads (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly

  #[cfg(test)]
  #[test]
  fn type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly() {
    use alloc::string::String;

    use ulua_analysis::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
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
        local function stringByteList(str)
            local out = {}
            for i = 1, #str do
                table.insert(out, string.byte(str, i))
            end
            return table.concat(out, ",")
        end

        local x = stringByteList("xoo")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);
      assert!(
        result
          .errors
          .iter()
          .any(|error| type_error_data_ref::<MultipleNonviableOverloads>(error).is_some()),
        "{:?}",
        result.errors
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tables_a_free_shape_can_turn_into_a_scalar_if_it_is_compatible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3745:type_infer_tables_a_free_shape_can_turn_into_a_scalar_if_it_is_compatible`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_a_free_shape_can_turn_into_a_scalar_if_it_is_compatible

  #[cfg(test)]
  #[test]
  fn type_infer_tables_a_free_shape_can_turn_into_a_scalar_if_it_is_compatible() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(s): string
            local foo = s:lower()
            return s
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(string) -> string",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3761:type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible

  #[cfg(test)]
  #[test]
  fn type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(s): string
            local foo = s:absolutely_no_scalar_has_this_method()
            return s
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(3, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Parameter 's' has been reduced to never. This function is not callable with any possible value.",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Parameter 's' is required to be a subtype of '{ read absolutely_no_scalar_has_this_method: (never) -> (unknown, ...unknown) }' here.",
        to_string_type_error(&result.errors[1])
      );
      assert_eq!(
        "Parameter 's' is required to be a subtype of 'string' here.",
        to_string_type_error(&result.errors[2])
      );
      assert_eq!(
        "(never) -> string",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      let expected = "Expected this to be 'string', but got 't1 where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));

      assert_eq!(
        "<a, b...>(t1) -> string where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    }
  }
}

mod type_infer_tables_access_index_metamethod_that_returns_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3394:type_infer_tables_access_index_metamethod_that_returns_variadic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_tables_access_index_metamethod_that_returns_variadic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_access_index_metamethod_that_returns_variadic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Foo = {x: string}
        local t = {}
        setmetatable(t, {
            __index = function(x: string): ...Foo
                return {x = x}
            end
        })

        local foo = t.bar
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: string }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("foo")),
        &mut options,
      )
    );
  }
}

mod type_infer_tables_accidentally_checked_prop_in_opposite_branch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3473:type_infer_tables_accidentally_checked_prop_in_opposite_branch`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_accidentally_checked_prop_in_opposite_branch

  #[cfg(test)]
  #[test]
  fn type_infer_tables_accidentally_checked_prop_in_opposite_branch() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: number?}? = {x = nil}
        local u = t and t.x == 5 or t.x == 31337
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type '{ x: number? }?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("u")))
    );
  }
}

mod type_infer_tables_allow_indexing_into_error_or_not_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6604:type_infer_tables_allow_indexing_into_error_or_not_nil`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_allow_indexing_into_error_or_not_nil

  #[cfg(test)]
  #[test]
  fn type_infer_tables_allow_indexing_into_error_or_not_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function f(i: number, ...)
            local value = select(i, ...)
            local valueType = typeof(value)
            if value == nil then
            elseif valueType == "table" then
                for k = 1, #value do
                    local _ = value[k]
                end
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_any_when_indexing_into_an_unsealed_table_with_no_indexer_in_nonstrict_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1002:type_infer_tables_any_when_indexing_into_an_unsealed_table_with_no_indexer_in_nonstrict_mode`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_any_when_indexing_into_an_unsealed_table_with_no_indexer_in_nonstrict_mode

  #[cfg(test)]
  #[test]
  fn type_infer_tables_any_when_indexing_into_an_unsealed_table_with_no_indexer_in_nonstrict_mode()
  {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        local constants = {
            key1 = "value1",
            key2 = "value2"
        }

        local function getKey()
            return "key1"
        end

        local k1 = constants[getKey()]
    "#,
      ),
      None,
    );

    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("k1")))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_array_factory_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:902:type_infer_tables_array_factory_function`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_array_factory_function

  #[cfg(test)]
  #[test]
  fn type_infer_tables_array_factory_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function empty() return {} end
        local array: {string} = empty()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_array_of_callbacks_bidirectionally_inferred {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6464:type_infer_tables_array_of_callbacks_bidirectionally_inferred`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_array_of_callbacks_bidirectionally_inferred

  #[cfg(test)]
  #[test]
  fn type_infer_tables_array_of_callbacks_bidirectionally_inferred() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Actions : { [string]: (string?) -> number? } = {
            Foo = function (input)
                if input then
                    return 42
                else
                    return nil
                end
            end
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 21)))
    );
  }
}

mod type_infer_tables_assign_key_at_index_expr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:138:type_infer_tables_assign_key_at_index_expr`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - translates_to -> rust_item type_infer_tables_assign_key_at_index_expr

  #[cfg(test)]
  #[test]
  fn type_infer_tables_assign_key_at_index_expr() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {[string]: number})
            t["hello"] = 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position {
        line: 2,
        column: 19,
      }))
    );
  }
}

mod type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1049:type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t["a"] = "foo"

        local a = t.a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.get_builtins().string_type)
    );

    let t_type = fixture.require_type_string(&String::from("t"));
    let table_type = get_type_id::<TableType>(t_type).unwrap_or_else(|| {
      panic!("Expected a table but got {}", to_string_type_id(t_type));
    });

    assert!(table_type.indexer.is_none(), "{:?}", table_type.indexer);
    assert!(table_type.props.contains_key("a"));

    let a = table_type.props.get("a").expect("expected property a");
    let property_a = a.read_ty.expect("expected read type for property a");
    assert_eq!("string", to_string_type_id(property_a));
  }
}

mod type_infer_tables_augment_nested_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:114:type_infer_tables_augment_nested_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_augment_nested_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_augment_nested_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id, to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{table_type::TableType, to_string_options::ToStringOptions},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = { p = {} }
        t.p.foo = 'bar'
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = get_type_id::<TableType>(fixture.require_type_string(&String::from("t")))
      .expect("expected table type");
    let p_ty = t_type
      .props
      .get("p")
      .and_then(|prop| prop.read_ty)
      .expect("expected p read type");
    assert!(
      get_type_id::<TableType>(p_ty).is_some(),
      "expected p to have table type"
    );

    let mut opts = ToStringOptions::new(true);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ p: { foo: string } }"
    } else {
      "{| p: {| foo: string |} |}"
    };
    assert_eq!(
      expected,
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("t")),
        &mut opts
      )
    );
  }
}

mod type_infer_tables_augment_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:97:type_infer_tables_augment_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_augment_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_augment_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t.foo = 'bar'
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ foo: string }"
    } else {
      "{| foo: string |}"
    };
    assert_eq!(
      expected,
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("t")),
        &mut opts
      )
    );
  }
}

mod type_infer_tables_bad_insert_type_mismatch {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_bad_insert_type_mismatch() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function doInsert(t: { string })
            table.insert(t, true)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("boolean", to_string_type_id(err.given_type));
    assert_eq!("string", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_tables_basic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:73:type_infer_tables_basic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_tables_basic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_basic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id,
      records::{primitive_type::PrimitiveType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local t = {foo = \"bar\", baz = 9, quux = nil}"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = get_type_id::<TableType>(fixture.require_type_string(&String::from("t")))
      .expect("expected table type");

    let foo_ty = t_type
      .props
      .get("foo")
      .and_then(|prop| prop.read_ty)
      .expect("expected foo read type");
    assert_eq!(
      Some(PrimitiveType::STRING),
      fixture.get_primitive_type(foo_ty)
    );

    let baz_ty = t_type
      .props
      .get("baz")
      .and_then(|prop| prop.read_ty)
      .expect("expected baz read type");
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(baz_ty)
    );

    let quux_ty = t_type
      .props
      .get("quux")
      .and_then(|prop| prop.read_ty)
      .expect("expected quux read type");
    assert_eq!(
      Some(PrimitiveType::NIL_TYPE),
      fixture.get_primitive_type(quux_ty)
    );
  }
}

mod type_infer_tables_basic_data_like_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6769:type_infer_tables_basic_data_like_array`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_tables_basic_data_like_array

  #[cfg(test)]
  #[test]
  fn type_infer_tables_basic_data_like_array() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {
            {1, 2, 3},
            {4, 5, 6}
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{{number}}",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("t")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_bidirectional_inference_intersection_other_intersection_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6391:type_infer_tables_bidirectional_inference_intersection_other_intersection_example`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_inference_intersection_other_intersection_example

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_inference_intersection_other_intersection_example() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { foo: "a" }
        type B = { bar: "b" }
        type AB = A & B
        local t: AB = {
            foo = "a",
            bar = "b",
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bidirectional_inference_variadic_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6528:type_infer_tables_bidirectional_inference_variadic_type_pack`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_inference_variadic_type_pack

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_inference_variadic_type_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: { (...string) -> () } = {
            function (foobar)
                print(foobar)
            end
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 24))
      )
    );
  }
}

mod type_infer_tables_bidirectional_inference_works_through_intersections {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6381:type_infer_tables_bidirectional_inference_works_through_intersections`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_inference_works_through_intersections

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_inference_works_through_intersections() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type x = {} & ({ state: "1" } | { state: "2" })
        local x: x = { state = "2" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bidirectional_typechecking_with_write_only_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4453:type_infer_tables_bidirectional_typechecking_with_write_only_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_typechecking_with_write_only_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_typechecking_with_write_only_property() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {write x: number})
            t.x = 5
        end

        f({ x = 2 })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bidirectional_union_function_vs_primitive_property_discrimination {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7205:type_infer_tables_bidirectional_union_function_vs_primitive_property_discrimination`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_union_function_vs_primitive_property_discrimination

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_union_function_vs_primitive_property_discrimination() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type FnRecord = { handler: (number) -> string, label: string? }
        type StrRecord = { handler: string, label: string? }
        type Record = FnRecord | StrRecord

        local r: Record = {
            handler = function(input)
                return tostring(input)
            end,
            label = "test"
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(7, 34))
      )
    );
  }
}

mod type_infer_tables_bidirectional_union_mixed_table_and_non_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7165:type_infer_tables_bidirectional_union_mixed_table_and_non_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_union_mixed_table_and_non_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_union_mixed_table_and_non_table() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Response = string | { status: number, body: string }

        local r: Response = { status = 200, body = "ok" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bidirectional_union_non_singleton_discrimination {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7148:type_infer_tables_bidirectional_union_non_singleton_discrimination`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_union_non_singleton_discrimination

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_union_non_singleton_discrimination() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type NumericRecord = { value: number, label: string }
        type StringRecord = { value: string, flag: boolean }
        type Record = NumericRecord | StringRecord

        local r1: Record = { value = 42, label = "hello" }
        local r2: Record = { value = "hmmm", flag = true }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bidirectional_union_via_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7179:type_infer_tables_bidirectional_union_via_type_function`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Config (Config/include/Luau/Config.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_bidirectional_union_via_type_function

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bidirectional_union_via_type_function() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function Optional(t)
            return types.unionof(t, types.singleton(nil))
        end

        type Config = {
            host: string,
            port: number,
            verbose: boolean?,
        }

        local cfg: Optional<Config> = {
            host = "localhost",
            port = 8080,
            verbose = true,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_bigger_nested_table_causes_big_type_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5683:type_infer_tables_bigger_nested_table_causes_big_type_error`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function main (tests/main.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_bigger_nested_table_causes_big_type_error

  #[cfg(test)]
  #[test]
  fn type_infer_tables_bigger_nested_table_causes_big_type_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type File = {
            type: "file",
            name: string,
            content: string?,
        }

        type Dir = {
            type: "dir",
            name: string,
            children: { File | Dir }?,
        }


        type DirectoryChildren = { File | Dir }

        local newtree: DirectoryChildren = {
            {
                type = "dir",
                name = "src",
                children = {
                    {
                        type = "file",
                        path = "main.luau", -- I accidentally assign "path" instead of "name", causing a huge scary TypeError
                    }
                }
            }
        }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Table type '{ path: string, type: \"file\" }' not compatible with type 'File' because the former is missing field 'name'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
    assert_eq!(
      Location::new(Position::new(21, 20), Position::new(24, 21)),
      result.errors[0].location
    );
  }
}

mod type_infer_tables_builtin_table_names {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2122:type_infer_tables_builtin_table_names`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_builtin_table_names

  #[cfg(test)]
  #[test]
  fn type_infer_tables_builtin_table_names() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        os.h = 2
        string.k = 3
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot add property 'h' to table 'typeof(os)'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Cannot add property 'k' to table 'typeof(string)'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_tables_call_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:310:type_infer_tables_call_method`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_call_method

  #[cfg(test)]
  #[test]
  fn type_infer_tables_call_method() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.x = 0
        function T:method()
            return self.x
        end
        local a = T:method()
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_tables_call_method_with_explicit_self_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:325:type_infer_tables_call_method_with_explicit_self_argument`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_call_method_with_explicit_self_argument

  #[cfg(test)]
  #[test]
  fn type_infer_tables_call_method_with_explicit_self_argument() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.x = 0

        function T:method()
            return self.x
        end

        local a = T.method(T)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cannot_augment_sealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:167:type_infer_tables_cannot_augment_sealed_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_tables_cannot_augment_sealed_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cannot_augment_sealed_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{
        cannot_extend_table::{CannotExtendTable, CannotExtendTable_Context},
        to_string_options::ToStringOptions,
      },
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function mkt()
            return {prop=999}
        end

        local t = mkt()
        t.foo = 'bar'
    "#,
      ),
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      Location {
        begin: Position { line: 6, column: 8 },
        end: Position {
          line: 6,
          column: 13,
        },
      },
      result.errors[0].location
    );

    let error = type_error_data_ref::<CannotExtendTable>(&result.errors[0])
      .expect("expected CannotExtendTable");
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ prop: number }",
      to_string_type_id_to_string_options(error.table_type(), &mut opts)
    );
    assert_eq!("foo", error.prop());
    assert_eq!(CannotExtendTable_Context::Property, error.context());
  }
}

mod type_infer_tables_cannot_call_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2882:type_infer_tables_cannot_call_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_cannot_call_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cannot_call_tables() {
    use alloc::string::String;

    use ulua_analysis::records::cannot_call_non_function::CannotCallNonFunction;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
      .check_string_optional_frontend_options(&String::from("local foo = {}    foo()"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
      .expect("expected CannotCallNonFunction");
  }
}

mod type_infer_tables_cannot_change_type_of_table_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:223:type_infer_tables_cannot_change_type_of_table_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cannot_change_type_of_table_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cannot_change_type_of_table_prop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local t = {prop=999}   t.prop = 'hello'"),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cannot_change_type_of_unsealed_table_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:213:type_infer_tables_cannot_change_type_of_unsealed_table_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cannot_change_type_of_unsealed_table_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cannot_change_type_of_unsealed_table_prop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t.prop = 999
        t.prop = 'hello'
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cant_index_this {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5004:type_infer_tables_cant_index_this`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_cant_index_this

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cant_index_this() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::not_a_table::NotATable,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number = 9
        a[18] = "tomfoolery"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let not_a_table =
      type_error_data_ref::<NotATable>(&result.errors[0]).expect("expected NotATable");
    assert_eq!("number", to_string_type_id(not_a_table.ty));
  }
}

mod type_infer_tables_casting_sealed_tables_with_props_into_table_with_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1727:type_infer_tables_casting_sealed_tables_with_props_into_table_with_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_casting_sealed_tables_with_props_into_table_with_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_casting_sealed_tables_with_props_into_table_with_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type StringToStringMap = { [string]: string }
        function mkrt() return { ["foo"] = 1 } end
        local rt: StringToStringMap = mkrt()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::new(true);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "{ [string]: string }",
      to_string_type_id_to_string_options(tm.wanted_type, &mut options)
    );
    assert_eq!(
      "{ foo: number }",
      to_string_type_id_to_string_options(tm.given_type, &mut options)
    );
  }
}

mod type_infer_tables_casting_tables_with_props_into_table_with_indexer_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1744:type_infer_tables_casting_tables_with_props_into_table_with_indexer_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_casting_tables_with_props_into_table_with_indexer_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_casting_tables_with_props_into_table_with_indexer_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(x: {[string]: number, a: string}) end
        foo({ a = "" })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_casting_tables_with_props_into_table_with_indexer_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1754:type_infer_tables_casting_tables_with_props_into_table_with_indexer_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_casting_tables_with_props_into_table_with_indexer_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_casting_tables_with_props_into_table_with_indexer_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(a: {[string]: number, a: string}) end
        foo({ a = 1 })
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::new(true);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("string", to_string_type_id(tm.wanted_type));
      assert_eq!("number", to_string_type_id(tm.given_type));
    } else {
      assert_eq!(
        "{ [string]: number, a: string }",
        to_string_type_id_to_string_options(tm.wanted_type, &mut options)
      );
      assert_eq!(
        "{| [string]: number, a: number |}",
        to_string_type_id_to_string_options(tm.given_type, &mut options)
      );
    }
  }
}

mod type_infer_tables_casting_tables_with_props_into_table_with_indexer_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1779:type_infer_tables_casting_tables_with_props_into_table_with_indexer_4`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_casting_tables_with_props_into_table_with_indexer_4

  #[cfg(test)]
  #[test]
  fn type_infer_tables_casting_tables_with_props_into_table_with_indexer_4() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(a: {[string]: number, a: string}, i: string)
            return a[i]
        end
        local hi: number = foo({ a = "hi" }, "a") -- shouldn't typecheck since at runtime hi is "hi"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_casting_unsealed_tables_with_props_into_table_with_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1698:type_infer_tables_casting_unsealed_tables_with_props_into_table_with_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_casting_unsealed_tables_with_props_into_table_with_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_casting_unsealed_tables_with_props_into_table_with_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type StringToStringMap = { [string]: string }
        local rt: StringToStringMap = { ["foo"] = 1 }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::new(true);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        Location {
          begin: Position {
            line: 2,
            column: 50
          },
          end: Position {
            line: 2,
            column: 51
          },
        },
        result.errors[0].location
      );
      assert_eq!(
        "string",
        to_string_type_id_to_string_options(tm.wanted_type, &mut options)
      );
      assert_eq!(
        "number",
        to_string_type_id_to_string_options(tm.given_type, &mut options)
      );
    } else {
      assert_eq!(
        "{ [string]: string }",
        to_string_type_id_to_string_options(tm.wanted_type, &mut options)
      );
      assert_eq!(
        "{| [string]: string, foo: number |}",
        to_string_type_id_to_string_options(tm.given_type, &mut options)
      );
    }
  }
}

mod type_infer_tables_certain_properties_of_table_literal_arguments_can_be_covariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4145:type_infer_tables_certain_properties_of_table_literal_arguments_can_be_covariant`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_certain_properties_of_table_literal_arguments_can_be_covariant

  #[cfg(test)]
  #[test]
  fn type_infer_tables_certain_properties_of_table_literal_arguments_can_be_covariant() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: {[string]: string | {any} | nil })
            return a
        end

        local x = f({
            title = "Feature.VirtualEvents.EnableNotificationsModalTitle",
            body = "Feature.VirtualEvents.EnableNotificationsModalBody",
            notNow = "Feature.VirtualEvents.NotNowButton",
            getNotified = "Feature.VirtualEvents.GetNotifiedButton",
        })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_checked_prop_too_early {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3452:type_infer_tables_checked_prop_too_early`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_checked_prop_too_early

  #[cfg(test)]
  #[test]
  fn type_infer_tables_checked_prop_too_early() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: number?}? = {x = nil}
        local u = t.x and t or 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type '{ x: number? }?' could be nil",
      to_string_type_error(&result.errors[0])
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number | { read x: number, write x: number? }",
        to_string_type_id(fixture.require_type_string(&String::from("u")))
      );
    } else {
      assert_eq!(
        "number | { x: number? }",
        to_string_type_id(fixture.require_type_string(&String::from("u")))
      );
    }
  }
}

mod type_infer_tables_cli_119126_regression {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_119126_regression() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type literals = "foo" | "bar" | "foobar"

        local exampleA: {[literals]: string} = {
            foo = '1',
            bar = 2,
            foobar = 3,
        }
    "#,
      ),
      None,
    );

    assert_eq!(2, results.errors.len(), "{:?}", results.errors);
    for err in &results.errors {
      let mismatch = type_error_data_ref::<TypeMismatch>(err).expect("expected TypeMismatch");
      assert_eq!("string", to_string_type_id(mismatch.wanted_type));
      assert_eq!("number", to_string_type_id(mismatch.given_type));
    }
  }
}

mod type_infer_tables_cli_162179_avoid_exponential_blowup_in_normalization {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_162179_avoid_exponential_blowup_in_normalization() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut entries = String::new();
    for _ in 0..100 {
      entries.push_str("\"foo\",");
    }

    let source = String::from(
      r#"
        local res = { "#,
    ) + &entries
      + r#" }

        local function check(index: number)
            if res[index] == "foo" then
                print("found a foo!")
            end
        end
    "#;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(&source, None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cli_167052 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_167052() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Children = newproxy()
        local Macro: { [ string | typeof(Children) ]: true } = {
            ["_exec"] = true;
            ["_run"] = true;
            ["_init"] = true;
            ["_base"] = true;
            ["Class"] = true;
            ["_count"] = true;
            [Children] = true;
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cli_174304_allow_getmetatable_error_and_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6577:type_infer_tables_cli_174304_allow_getmetatable_error_and_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cli_174304_allow_getmetatable_error_and_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_174304_allow_getmetatable_error_and_table() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function instanceof(tbl: any, class: any): boolean
            if typeof(tbl) ~= "table" then
                return false
            end

            local ok, hasNew = pcall(function()
                return class.new ~= nil and tbl.new == class.new
            end)
            if ok and hasNew then
                return true
            end

            while typeof(tbl) == "table" do
                tbl = getmetatable(tbl)
                if typeof(tbl) == "table" then
                    tbl = tbl.__index
                end
            end

            return false
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cli_184926_bidi_inference_pushes_into_lambda_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6719:type_infer_tables_cli_184926_bidi_inference_pushes_into_lambda_return_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_cli_184926_bidi_inference_pushes_into_lambda_return_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_184926_bidi_inference_pushes_into_lambda_return_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MyType = { Func: (transformFunction: () -> ({number})) -> () }

        local myValue = {} :: MyType

        myValue.Func(function() return {} end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cli_186992_accidental_dropping_free_ty_bounds {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2381:type_infer_tables_cli_186992_accidental_dropping_free_ty_bounds`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cli_186992_accidental_dropping_free_ty_bounds

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_186992_accidental_dropping_free_ty_bounds() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local lines = {}
        table.insert(lines, table.concat({}, ""))
        print(table.concat(lines, "\n"))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{string}",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("lines")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_cli_84607_missing_prop_in_array_or_dict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4217:type_infer_tables_cli_84607_missing_prop_in_array_or_dict`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_cli_84607_missing_prop_in_array_or_dict

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cli_84607_missing_prop_in_array_or_dict() {
    use alloc::string::String;

    use ulua_analysis::records::{
      missing_properties::MissingProperties, type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fix = ScopedFastFlag::new(&FFlag::LuauFixIndexerSubtypingOrdering, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Thing = { name: string, prop: boolean }

        local arrayOfThings : {Thing} = {
            { name = "a" }
        }

        local dictOfThings : {[string]: Thing} = {
            a = { name = "a" }
        }
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      for error in &result.errors {
        let error =
          type_error_data_ref::<MissingProperties>(error).expect("expected MissingProperties");
        assert_eq!(1, error.properties().len());
        assert_eq!("prop", error.properties()[0].as_str());
      }
    } else {
      let error1 = type_error_data_ref::<MissingProperties>(&result.errors[0])
        .expect("expected MissingProperties");
      assert_eq!(1, error1.properties().len());
      assert_eq!("prop", error1.properties()[0].as_str());

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      let nested = mismatch.error.as_ref().expect("expected nested TypeError");
      let error2 =
        type_error_data_ref::<MissingProperties>(nested).expect("expected MissingProperties");
      assert_eq!(1, error2.properties().len());
      assert_eq!("prop", error2.properties()[0].as_str());
    }
  }
}

mod type_infer_tables_cmpeq_any_with_nil_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6804:type_infer_tables_cmpeq_any_with_nil_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cmpeq_any_with_nil_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cmpeq_any_with_nil_ok() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = {
    foo : { [string] : string}
}

type B = {
	parsed: A,
}

local x : B = (nil :: any)
local found = x.parsed.foo["any"] == nil -- errors
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cmpeq_any_with_nil_ok_in_if {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6873:type_infer_tables_cmpeq_any_with_nil_ok_in_if`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cmpeq_any_with_nil_ok_in_if

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cmpeq_any_with_nil_ok_in_if() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = {
    foo : { [string] : string}
}

type B = {
	parsed: A,
}

local x : B = (nil :: any)

if x.parsed.foo["any"] == nil then
end

"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cmpneq_any_with_nil_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6826:type_infer_tables_cmpneq_any_with_nil_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cmpneq_any_with_nil_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cmpneq_any_with_nil_ok() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = {
    foo : { [string] : string}
}

type B = {
	parsed: A,
}

local x : B = (nil :: any)
local found = x.parsed.foo["any"] ~= nil -- errors
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_cmpneq_any_with_nil_ok_in_if {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6848:type_infer_tables_cmpneq_any_with_nil_ok_in_if`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cmpneq_any_with_nil_ok_in_if

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cmpneq_any_with_nil_ok_in_if() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = {
    foo : { [string] : string}
}

type B = {
	parsed: A,
}

local x : B = (nil :: any)

if x.parsed.foo["any"] ~= nil then
end

"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_general {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2166:type_infer_tables_common_table_element_general`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_general

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_general() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Table = {
            a: number,
            b: number?
        }

        local Test: {Table} = {
            [2] = { a = 1 },
            [5] = { a = 2, b = 3 }
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_inner_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2185:type_infer_tables_common_table_element_inner_index`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_inner_index

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_inner_index() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = {
    a: number,
    b: number?
}

local Test: {{Table}} = {{
    { a = 1 },
    { a = 2, b = 3 }
},{
    { a = 3 },
    { a = 4, b = 3 }
}}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_inner_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2205:type_infer_tables_common_table_element_inner_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_inner_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_inner_prop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = {
    a: number,
    b: number?
}

local Test: {{x: Table, y: Table}} = {{
    x = { a = 1 },
    y = { a = 2, b = 3 }
},{
    x = { a = 3 },
    y = { a = 4 }
}}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_list {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2149:type_infer_tables_common_table_element_list`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_list

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_list() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = {
    a: number,
    b: number?
}

local Test: {Table} = {
    { a = 1 },
    { a = 2, b = 3 }
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_union_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2225:type_infer_tables_common_table_element_union_assignment`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_union_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_union_assignment() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo = {x: number | string}

local foos: {Foo} = {
    {x = 1234567},
    {x = "hello"},
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_union_in_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2273:type_infer_tables_common_table_element_union_in_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_union_in_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_union_in_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function foo(l: {{x: number | string}}) end

foo({
    {x = 1234567},
    {x = "hello"},
})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_union_in_call_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2287:type_infer_tables_common_table_element_union_in_call_tail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_union_in_call_tail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_union_in_call_tail() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Foo = {x: number | string}
        local function foo(l: {Foo}, ...: {Foo}) end

        foo(
            {{x = 1234567}, {x = "hello"}},
            {{x = 1234567}, {x = "hello"}},
            {{x = 1234567}, {x = "hello"}}
        )
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_common_table_element_union_in_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2306:type_infer_tables_common_table_element_union_in_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_common_table_element_union_in_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_common_table_element_union_in_prop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo = {x: number | string}
local t: { a: {Foo}, b: number } = {
    a = {
        {x = 1234567},
        {x = "hello"},
    },
    b = 5
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_compound_assignment_writes_lhs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6955:type_infer_tables_compound_assignment_writes_lhs`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_compound_assignment_writes_lhs

  #[cfg(test)]
  #[test]
  fn type_infer_tables_compound_assignment_writes_lhs() {
    use alloc::string::String;

    use ulua_analysis::records::property_access_violation::PropertyAccessViolation;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _visit_lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {
            read x: number
        }

        local foo: T = { x = 5 }
        foo.x += 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
  }
}

mod type_infer_tables_confusing_indexing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2779:type_infer_tables_confusing_indexing`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_confusing_indexing

  #[cfg(test)]
  #[test]
  fn type_infer_tables_confusing_indexing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {} & {p: number | string}
        local function f(t: T)
            return t.p
        end

        local foo = f({p = "string"})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_tables_cyclic_shifted_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4190:type_infer_tables_cyclic_shifted_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_cyclic_shifted_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_cyclic_shifted_tables() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x: a): a
          return x
        end

        -- Remove name from cyclic table
        local foo = id({})
        foo.foo = id({})
        foo.foo.foo = id({})
        foo.foo.foo.foo = id({})
        foo.foo.foo.foo.foo = foo

        local almostFoo = id({})
        almostFoo.foo = id({})
        almostFoo.foo.foo = id({})
        almostFoo.foo.foo.foo = id({})
        almostFoo.foo.foo.foo.foo = almostFoo
        -- Shift
        almostFoo = almostFoo.foo.foo
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_deeply_nested_classish_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5661:type_infer_tables_deeply_nested_classish_inference`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_tables_deeply_nested_classish_inference

  #[cfg(test)]
  #[test]
  fn type_infer_tables_deeply_nested_classish_inference() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(part, flag, params)
            local humanoid = part.Parent:FindFirstChild("Humanoid") or part.Parent.Parent:FindFirstChild("Humanoid")
            if humanoid.Parent:GetAttribute("Blocking") then
                if flag then
                    params.Found = { humanoid }
                else
                    humanoid:Think(1)
                end
            else
                humanoid:Think(2)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_method_for_a_builtin_sealed_table_must_fail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1356:type_infer_tables_defining_a_method_for_a_builtin_sealed_table_must_fail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_method_for_a_builtin_sealed_table_must_fail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_method_for_a_builtin_sealed_table_must_fail() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function string.m() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_method_for_a_local_sealed_table_must_fail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1374:type_infer_tables_defining_a_method_for_a_local_sealed_table_must_fail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_method_for_a_local_sealed_table_must_fail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_method_for_a_local_sealed_table_must_fail() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function mkt() return {x = 1} end
        local t = mkt()
        function t.m() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_method_for_a_local_unsealed_table_is_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1396:type_infer_tables_defining_a_method_for_a_local_unsealed_table_is_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_method_for_a_local_unsealed_table_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_method_for_a_local_unsealed_table_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {x = 1}
        function t.m() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_self_method_for_a_builtin_sealed_table_must_fail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1365:type_infer_tables_defining_a_self_method_for_a_builtin_sealed_table_must_fail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_self_method_for_a_builtin_sealed_table_must_fail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_self_method_for_a_builtin_sealed_table_must_fail() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function string:m() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_self_method_for_a_local_sealed_table_must_fail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1385:type_infer_tables_defining_a_self_method_for_a_local_sealed_table_must_fail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_self_method_for_a_local_sealed_table_must_fail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_self_method_for_a_local_sealed_table_must_fail() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function mkt() return {x = 1} end
        local t = mkt()
        function t:m() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_defining_a_self_method_for_a_local_unsealed_table_is_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1406:type_infer_tables_defining_a_self_method_for_a_local_unsealed_table_is_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_defining_a_self_method_for_a_local_unsealed_table_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_defining_a_self_method_for_a_local_unsealed_table_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {x = 1}
        function t:m() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_disable_singleton_inference_on_large_nested_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5878:type_infer_tables_disable_singleton_inference_on_large_nested_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_disable_singleton_inference_on_large_nested_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_disable_singleton_inference_on_large_nested_tables() {
    use alloc::string::String;

    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _limit = ScopedFastInt::new(&FInt::LuauPrimitiveInferenceInTableLimit, 2);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Word = "foo" | "bar"
        local words: {{ Word }} = {{ "foo", "bar", "foo" }}
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_disable_singleton_inference_on_large_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5864:type_infer_tables_disable_singleton_inference_on_large_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_disable_singleton_inference_on_large_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_disable_singleton_inference_on_large_tables() {
    use alloc::string::String;

    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _limit = ScopedFastInt::new(&FInt::LuauPrimitiveInferenceInTableLimit, 2);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Word = "foo" | "bar"
        local words: { Word } = { "foo", "bar", "foo" }
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1026:type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Compiler::getConstant (Compiler/src/Compiler.cpp)
  //!   - translates_to -> rust_item type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode

  #[cfg(test)]
  #[test]
  fn type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local constants = {
            key1 = "value1",
            key2 = "value2"
        }

        function getConstant(key)
            return constants[key]
        end

        local k1 = getConstant("key1")
    "#,
      ),
      None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "unknown"
    } else {
      "any"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("k1")))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_do_not_allow_laundering {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6735:type_infer_tables_do_not_allow_laundering`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_do_not_allow_laundering

  #[cfg(test)]
  #[test]
  fn type_infer_tables_do_not_allow_laundering() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _missing_properties_as_nil =
      ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function foo(t: {}): { x: nil }
            return t
        end

        local t: { x: number } = { x = 42 }
        local laundered = foo(t) -- via width subtyping
        laundered.x = nil
        assert(type(t) == "number")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_do_not_force_on_simple_bidirectional_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6406:type_infer_tables_do_not_force_on_simple_bidirectional_inference`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_do_not_force_on_simple_bidirectional_inference

  #[cfg(test)]
  #[test]
  fn type_infer_tables_do_not_force_on_simple_bidirectional_inference() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Role = "Citizen"

        local function getRoles(): { Role }
            return { 'Citizen' }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3196:type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::uninhabited_type_function::UninhabitedTypeFunction,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from("local x = setmetatable({})"), None);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
        .expect("expected UninhabitedTypeFunction");
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Argument count mismatch. Function 'setmetatable' expects 2 arguments, but only 1 is specified",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_tables_dont_extend_unsealed_tables_in_rvalue_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4062:type_infer_tables_dont_extend_unsealed_tables_in_rvalue_position`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_extend_unsealed_tables_in_rvalue_position

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_extend_unsealed_tables_in_rvalue_position() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local testDictionary = {
            FruitName = "Lemon",
            FruitColor = "Yellow",
            Sour = true
        }

        local print: any

        print(testDictionary[""])
    "#,
      ),
      None,
    );

    let ty = follow_type_id(fixture.require_type_string(&String::from("testDictionary")));
    let table = get_type_id::<TableType>(ty).expect("expected TableType");
    assert!(!table.props.contains_key(""));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tables_dont_hang_when_trying_to_look_up_in_cyclic_metatable_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2747:type_infer_tables_dont_hang_when_trying_to_look_up_in_cyclic_metatable_index`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_dont_hang_when_trying_to_look_up_in_cyclic_metatable_index

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_hang_when_trying_to_look_up_in_cyclic_metatable_index() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mt = {}
        local t = setmetatable({}, mt)
        mt.__index = t

        function mt:__tostring()
            return t.p
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 't' does not have key 'p'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3415:type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.file_resolver.source.insert(
      String::from("Module/Backend/Types"),
      String::from(
        r#"
        export type Fiber = {
            return_: Fiber?
        }
        return {}
    "#,
      ),
    );

    fixture.file_resolver.source.insert(
      String::from("Module/Backend"),
      String::from(
        r#"
        local Types = require(script.Types)
        type Fiber = Types.Fiber
        type ReactRenderer = { findFiberByHostInstance: () -> Fiber? }

        local function attach(renderer): ()
            local function getPrimaryFiber(fiber)
                local alternate = fiber.alternate
                return fiber
            end

            local function getFiberIDForNative()
                local fiber = renderer.findFiberByHostInstance()
                fiber = fiber.return_
                return getPrimaryFiber(fiber)
            end
        end

        function culprit(renderer: ReactRenderer): ()
            attach(renderer)
        end

        return culprit
    "#,
      ),
    );

    let _result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/Backend"), None);
  }
}

mod type_infer_tables_dont_leak_free_table_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3536:type_infer_tables_dont_leak_free_table_props`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_tables_dont_leak_free_table_props

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_leak_free_table_props() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_to_string_alt_c::to_string_type_id,
      to_string_to_string_alt_d::to_string_type_pack_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function a(state)
            print(state.blah)
        end

        local function b(state) -- The bug was that we inferred state: {blah: any, gwar: any}
            print(state.gwar)
        end

        return function()
            return function(state)
                a(state)
                b(state)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "({ read blah: unknown }) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "({ read gwar: unknown }) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
      );
      let module = unsafe { &*fixture.base.get_main_module(false) };
      assert_eq!(
        "(...any) -> ({ read blah: unknown, read gwar: unknown }) -> ()",
        to_string_type_pack_id(module.return_type)
      );
    } else {
      assert_eq!(
        "<a>({+ blah: a +}) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "<a>({+ gwar: a +}) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
      );
      let module = unsafe { &*fixture.base.get_main_module(false) };
      assert_eq!(
        "() -> <a, b>({+ blah: a, gwar: b +}) -> ()",
        to_string_type_pack_id(module.return_type)
      );
    }
  }
}

mod type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3069:type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Counter (tests/Parser.test.cpp)
  //!   - calls -> method TypeLevel::incr (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{
        function_type::FunctionType, metatable_type::MetatableType, table_type::TableType,
      },
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Counter = {}
        Counter.__index = Counter

        function Counter.new()
            local self = setmetatable({count=0}, Counter)
            return self
        end

        function Counter:incr()
            self.count = 1
            return self.count
        end

        local self = Counter.new()
        print(self:incr())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let counter_ty = fixture.base.require_type_string(&String::from("Counter"));
    let counter_type = get_type_id::<TableType>(counter_ty).unwrap_or_else(|| {
      panic!(
        "expected Counter TableType, got {}",
        to_string_type_id(counter_ty)
      )
    });

    let new_prop = counter_type.props.get("new").expect("expected Counter.new");
    let new_prop_read_ty = new_prop.read_ty.expect("expected Counter.new read type");
    let new_type = get_type_id::<FunctionType>(follow_type_id(new_prop_read_ty))
      .expect("expected Counter.new FunctionType");

    let new_ret_type = first(new_type.ret_types(), false).expect("expected Counter.new return");
    let new_ret = get_type_id::<MetatableType>(follow_type_id(new_ret_type))
      .expect("expected Counter.new MetatableType return");

    let new_ret_meta = get_type_id::<TableType>(follow_type_id(new_ret.metatable()))
      .expect("expected Counter.new metatable TableType");

    assert!(new_ret_meta.props.contains_key("incr"));
    assert_eq!(
      follow_type_id(new_ret.metatable()),
      follow_type_id(counter_ty)
    );
  }
}

mod type_infer_tables_dont_seal_an_unsealed_table_by_passing_it_to_a_function_that_takes_a_sealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:193:type_infer_tables_dont_seal_an_unsealed_table_by_passing_it_to_a_function_that_takes_a_sealed_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_seal_an_unsealed_table_by_passing_it_to_a_function_that_takes_a_sealed_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_seal_an_unsealed_table_by_passing_it_to_a_function_that_takes_a_sealed_table()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {[number]: number}
        function f(arg: T) end

        local B = {}
        f(B)
        function B:method() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_dont_suggest_exact_match_keys {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1533:type_infer_tables_dont_suggest_exact_match_keys`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_tables_dont_suggest_exact_match_keys

  #[cfg(test)]
  #[test]
  fn type_infer_tables_dont_suggest_exact_match_keys() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, to_string_error::to_string_type_error},
      records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
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
        local t = {}
        t.foO = 1
        print(t.Foo)
        t.Foo = 2
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let error = type_error_data_ref::<UnknownPropButFoundLikeProp>(&result.errors[0])
      .expect("expected UnknownPropButFoundLikeProp");
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_eq!(follow_type_id(t), follow_type_id(error.table()));
    assert_eq!("Foo", error.key());
    assert_eq!(1, error.candidates().len());
    assert!(error.candidates().contains("foO"));
    assert!(!error.candidates().contains("Foo"));
    assert_eq!(
      "Key 'Foo' not found in table 't'.  Did you mean 'foO'?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_duplicate_prop_references_share_same_result_type_and_constraint {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_duplicate_prop_references_share_same_result_type_and_constraint() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local tbl = {}
function f(x : number) : () end
function tbl:updateAmmoText()
    f(self.leadingZeros)
    local y = self.leadingZeros - 3
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_empty_union_container_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5480:type_infer_tables_empty_union_container_overflow`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_empty_union_container_overflow

  #[cfg(test)]
  #[test]
  fn type_infer_tables_empty_union_container_overflow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local CellRenderer = {}
        function CellRenderer:init(props)
            self._separators = {
                unhighlight = function()
                    local cellKey, prevCellKey = self.props.cellKey, self.props.prevCellKey
                    self.props.onUpdateSeparators({ cellKey, prevCellKey })
                end,
                updateProps = function (select, newProps)
                    local cellKey, prevCellKey = self.props.cellKey, self.props.prevCellKey
                    self.props.onUpdateSeparators({ if select == 'leading' then prevCellKey else cellKey })
                end
            }
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_error_detailed_indexer_key {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2528:type_infer_tables_error_detailed_indexer_key`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_detailed_indexer_key

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_detailed_indexer_key() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { [number]: string }
        type B = { [string]: string }

        local a: A = { 'a', 'b' }
        local b: B = a
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'B', but got 'A'; \nthe index type is `number` in the latter type and `string` in the former type, and `number` is not exactly `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be exactly 'B', but got 'A'
caused by:
  Property '[indexer key]' is not compatible.
Expected this to be exactly 'string', but got 'number'"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_tables_error_detailed_indexer_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2558:type_infer_tables_error_detailed_indexer_value`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_detailed_indexer_value

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_detailed_indexer_value() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { [number]: number }
        type B = { [number]: string }

        local a: A = { 1, 2, 3 }
        local b: B = a
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'B', but got 'A'; \nthe result of indexing is `number` in the latter type and `string` in the former type, and `number` is not exactly `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be exactly 'B', but got 'A'
caused by:
  Property '[indexer value]' is not compatible.
Expected this to be exactly 'string', but got 'number'"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_tables_error_detailed_metatable_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2458:type_infer_tables_error_detailed_metatable_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record First (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_error_detailed_metatable_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_detailed_metatable_prop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a1 = setmetatable({ x = 2, y = 3 }, { __call = function(s) end });
local b1 = setmetatable({ x = 2, y = "hello" }, { __call = function(s) end });
local c1: typeof(a1) = b1

local a2 = setmetatable({ x = 2, y = 3 }, { __call = function(s) end });
local b2 = setmetatable({ x = 2, y = 4 }, { __call = function(s, t) end });
local c2: typeof(a2) = b2
    "#,
      ),
      None,
    );

    let expected1 = r#"Expected this to be 'a1', but got 'b1'
caused by:
  Expected this to be exactly
	'{| x: number, y: number |}'
but got
	'{| x: number, y: string |}'
caused by:
  Property 'y' is not compatible.
Expected this to be exactly 'number', but got 'string'"#;
    let expected2 = r#"Expected this to be 'a2', but got 'b2'
caused by:
  Expected this to be exactly
	'{| __call: <a>(a) -> () |}'
but got
	'{| __call: <a, b>(a, b) -> () |}'
caused by:
  Property '__call' is not compatible.
Expected this to be exactly
	'<a>(a) -> ()'
but got
	'<a, b>(a, b) -> ()'; different number of generic type parameters"#;

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = "Expected this to be 'a1', but got 'b1'; \nin the table portion, accessing `y` results in `string` in the latter type and `number` in the former type, and `string` is not exactly `number`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(expected1, to_string_type_error(&result.errors[0]));
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    }
  }
}

mod type_infer_tables_error_detailed_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2392:type_infer_tables_error_detailed_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_detailed_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_detailed_prop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = { x: number, y: number }
type B = { x: number, y: string }

local a: A = { x = 123, y = 456 }
local b: B = a
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'B', but got 'A'; \naccessing `y` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be exactly 'B', but got 'A'
caused by:
  Property 'y' is not compatible.
Expected this to be exactly 'string', but got 'number'"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_tables_error_detailed_prop_nested {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2422:type_infer_tables_error_detailed_prop_nested`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_detailed_prop_nested

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_detailed_prop_nested() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type AS = { x: number, y: number }
type BS = { x: number, y: string }

type A = { a: boolean, b: AS }
type B = { a: boolean, b: BS }

local a: A = { a = false, b = { x = 123, y = 456 } }
local b: B = a
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'B', but got 'A'; \naccessing `b.y` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be exactly 'B', but got 'A'
caused by:
  Property 'b' is not compatible.
Expected this to be exactly 'BS', but got 'AS'
caused by:
  Property 'y' is not compatible.
Expected this to be exactly 'string', but got 'number'"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_tables_error_suppression_for_read_write {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7042:type_infer_tables_error_suppression_for_read_write`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_error_suppression_for_read_write

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_suppression_for_read_write() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { [string]: string }): { read foo: any, write foo: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ [string]: string }", to_string_type_id(err.given_type));
    assert_eq!(
      "{ read foo: any, write foo: number }",
      to_string_type_id(err.wanted_type)
    );
  }
}

mod type_infer_tables_error_suppression_on_all_table_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7012:type_infer_tables_error_suppression_on_all_table_properties`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_suppression_on_all_table_properties

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_suppression_on_all_table_properties() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { a: string, b: number }): { a: any, b: any }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_error_supression_of_union_of_tables_should_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6975:type_infer_tables_error_supression_of_union_of_tables_should_work`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_error_supression_of_union_of_tables_should_work

  #[cfg(test)]
  #[test]
  fn type_infer_tables_error_supression_of_union_of_tables_should_work() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Foo<T> = { kind: "foo", foo: T }
        type Bar<T> = { kind: "bar", bar: T }
        type FooBar<T> = Foo<T> | Bar<T>

        local function f(x: Foo<number>): FooBar<any>
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_evil_table_unification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3185:type_infer_tables_evil_table_unification`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function split (Common/src/StringUtils.cpp)
  //!   - translates_to -> rust_item type_infer_tables_evil_table_unification

  #[cfg(test)]
  #[test]
  fn type_infer_tables_evil_table_unification() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _ = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
_ = ...
_:table(_,string)[_:gsub(_,...,n0)],_,_:gsub(_,string)[""],_:split(_,...,table)._,n0 = nil
do end
"#,
      ),
      None,
    );
  }
}

mod type_infer_tables_expected_indexer_from_table_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3615:type_infer_tables_expected_indexer_from_table_union`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_expected_indexer_from_table_union

  #[cfg(test)]
  #[test]
  fn type_infer_tables_expected_indexer_from_table_union() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(r#"local a: {[string]: {number | string}} = {a = {2, 's'}}"#),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(r#"local a: {[string]: {number | string}}? = {a = {2, 's'}}"#),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"local a: {[string]: {[string]: {string?}}?} = {["a"] = {["b"] = {"a", "b"}}}"#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_expected_indexer_value_type_extra {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3592:type_infer_tables_expected_indexer_value_type_extra`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_expected_indexer_value_type_extra

  #[cfg(test)]
  #[test]
  fn type_infer_tables_expected_indexer_value_type_extra() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { { x: boolean?, y: boolean? } }

        local l1: {[string]: X} = { key = { { x = true }, { y = true } } }
        local l2: {[any]: X} = { key = { { x = true }, { y = true } } }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_expected_indexer_value_type_extra_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3604:type_infer_tables_expected_indexer_value_type_extra_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_expected_indexer_value_type_extra_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_expected_indexer_value_type_extra_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = {[any]: string | boolean}

        local x: X = { key = "str" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_explicit_nil_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2012:type_infer_tables_explicit_nil_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_explicit_nil_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_explicit_nil_indexer() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_ast::records::{location::Location, position::Position};
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
        local function _(t: { [string]: number? }): number
            return t.hello
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 19
        },
        end: Position {
          line: 2,
          column: 26
        },
      },
      result.errors[0].location
    );
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  }
}

mod type_infer_tables_explicitly_typed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2588:type_infer_tables_explicitly_typed_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_explicitly_typed_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_explicitly_typed_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Super = { x : number }
type Sub = { x : number, y: number }
type HasSuper = { p : Super }
type HasSub = { p : Sub }
local a: HasSuper = { p = { x = 5, y = 7 }}
a.p = { x = 9 }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_explicitly_typed_table_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2605:type_infer_tables_explicitly_typed_table_error`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_explicitly_typed_table_error

  #[cfg(test)]
  #[test]
  fn type_infer_tables_explicitly_typed_table_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Super = { x : number }
type Sub = { x : number, y: number }
type HasSuper = { p : Super }
type HasSub = { p : Sub }
local tmp = { p = { x = 5, y = 7 }}
local a: HasSuper = tmp
a.p = { x = 9 }
-- needs to be an error because
local y: number = tmp.p.y
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'HasSuper', but got 'tmp'; \naccessing `p` results in `{ x: number, y: number }` in the latter type and `Super` in the former type, and `{ x: number, y: number }` is not exactly `Super`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be exactly 'HasSuper', but got 'tmp'
caused by:
  Property 'p' is not compatible.
Table type '{| x: number, y: number |}' not compatible with type 'Super' because the former has extra field 'y'"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_tables_explicitly_typed_table_with_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2640:type_infer_tables_explicitly_typed_table_with_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_explicitly_typed_table_with_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_explicitly_typed_table_with_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Super = { x : number }
        type Sub = { x : number, y: number }
        type HasSuper = { [string] : Super }
        type HasSub = { [string] : Sub }
        local a: HasSuper = { p = { x = 5, y = 7 }}
        a.p = { x = 9 }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_extend_unsealed_table_with_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4088:type_infer_tables_extend_unsealed_table_with_metatable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_extend_unsealed_table_with_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_extend_unsealed_table_with_metatable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = setmetatable({}, {
            __call = function(_, name: string?)
            end,
        })

        T.for_ = "for_"

        return T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_extremely_large_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5906:type_infer_tables_extremely_large_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Fixture (tests/Fixture.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record BuiltinsFixture (tests/Fixture.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
  //!   - type_ref -> record Counter (tests/Parser.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - translates_to -> rust_item type_infer_tables_extremely_large_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_extremely_large_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut source = String::from("local res = {\n");
    for _ in 0..10_000 {
      source.push_str("\"foo\",\n");
    }
    source.push('}');

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(&source, None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{string}",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("res")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_found_like_key_in_table_function_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1456:type_infer_tables_found_like_key_in_table_function_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_tables_found_like_key_in_table_function_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_found_like_key_in_table_function_call() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        function t.Foo() end

        t.fOo()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let error = type_error_data_ref::<UnknownPropButFoundLikeProp>(&result.errors[0])
      .expect("expected UnknownPropButFoundLikeProp");
    let t = fixture.require_type_string(&String::from("t"));
    assert_eq!(to_string_type_id(t), to_string_type_id(error.table()));
    assert_eq!("fOo", error.key());
    assert_eq!(1, error.candidates().len());
    assert!(error.candidates().contains("Foo"));
    assert_eq!(
      "Key 'fOo' not found in table 't'.  Did you mean 'Foo'?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_found_like_key_in_table_property_access {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1482:type_infer_tables_found_like_key_in_table_property_access`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_tables_found_like_key_in_table_property_access

  #[cfg(test)]
  #[test]
  fn type_infer_tables_found_like_key_in_table_property_access() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {X = 1}

        print(t.x)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let error = type_error_data_ref::<UnknownPropButFoundLikeProp>(&result.errors[0])
      .expect("expected UnknownPropButFoundLikeProp");
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_eq!(to_string_type_id(t), to_string_type_id(error.table()));
    assert_eq!("x", error.key());
    assert_eq!(1, error.candidates().len());
    assert!(error.candidates().contains("X"));
    assert_eq!(
      "Key 'x' not found in table 't'.  Did you mean 'X'?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_found_multiple_like_keys {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1507:type_infer_tables_found_multiple_like_keys`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_tables_found_multiple_like_keys

  #[cfg(test)]
  #[test]
  fn type_infer_tables_found_multiple_like_keys() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {Foo = 1, foO = 2}

        print(t.foo)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let error = type_error_data_ref::<UnknownPropButFoundLikeProp>(&result.errors[0])
      .expect("expected UnknownPropButFoundLikeProp");
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_eq!(to_string_type_id(t), to_string_type_id(error.table()));
    assert_eq!("foo", error.key());
    assert_eq!(2, error.candidates().len());
    assert!(error.candidates().contains("Foo"));
    assert!(error.candidates().contains("foO"));
    assert_eq!(
      "Key 'foo' not found in table 't'.  Did you mean one of 'Foo', 'foO'?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_free_rhs_table_can_also_be_bound {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2946:type_infer_tables_free_rhs_table_can_also_be_bound`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_free_rhs_table_can_also_be_bound

  #[cfg(test)]
  #[test]
  fn type_infer_tables_free_rhs_table_can_also_be_bound() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local o
        local v = o:i()

        function g(u)
            v = u
        end

        o:f(g)
        o:h()
        o:h()
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_free_types_with_sealed_table_upper_bounds_can_still_be_expanded {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_free_types_with_sealed_table_upper_bounds_can_still_be_expanded() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function bar(a: {x: number}) end

        function foo(a)
            bar(a)

            -- Here, a : A where A = never <: A <: {x: number}
            -- The upper bound of A is a sealed table, but we nevertheless want to extend it.
            a.nope()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ read nope: () -> (...unknown) } & { x: number }) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_tables_function_call_in_indexer_with_compound_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5745:type_infer_tables_function_call_in_indexer_with_compound_assign`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_function_call_in_indexer_with_compound_assign

  #[cfg(test)]
  #[test]
  fn type_infer_tables_function_call_in_indexer_with_compound_assign() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local _ = 7143424
        _[
            setfenv(
                ...,
                {
                    n0 = _,
                }
            )
        ] *= _
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_function_calls_can_produce_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:247:type_infer_tables_function_calls_can_produce_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_function_calls_can_produce_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_function_calls_can_produce_tables() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("function get_table() return {prop=999} end    get_table().prop = 0"),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_function_calls_produces_sealed_table_given_unsealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:253:type_infer_tables_function_calls_produces_sealed_table_given_unsealed_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - translates_to -> rust_item type_infer_tables_function_calls_produces_sealed_table_given_unsealed_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_function_calls_produces_sealed_table_given_unsealed_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f() return {} end
        f().foo = 'fail'
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_function_check_constraint_too_eager {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5297:type_infer_tables_function_check_constraint_too_eager`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_function_check_constraint_too_eager

  #[cfg(test)]
  #[test]
  fn type_infer_tables_function_check_constraint_too_eager() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function doTheThing(_: { [string]: unknown }) end
        doTheThing({
            ['foo'] = 5,
            ['bar'] = 'heyo',
        })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Input = { [string]: unknown }

        local i : Input = {
            [('%s'):format('3.14')]=5,
            ['stringField']='Heyo'
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Input = { [string]: unknown }

        local function doTheThing(_: Input) end

        doTheThing({
            [('%s'):format('3.14')]=5,
            ['stringField']='Heyo'
        })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_match_literal_type_crash_again {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5778:type_infer_tables_fuzz_match_literal_type_crash_again`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_match_literal_type_crash_again

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_match_literal_type_crash_again() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(_: { [string]: {unknown}} ) end
        f(
            {
                _ = { 42 },
                _ = { x = "foo" },
            }
        )
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_table_extra_prop_unification_can_bound_owner_to_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3954:type_infer_tables_fuzz_table_extra_prop_unification_can_bound_owner_to_string`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foreach (VM/src/ltablib.cpp)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_table_extra_prop_unification_can_bound_owner_to_string

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_table_extra_prop_unification_can_bound_owner_to_string() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
l0,_ = nil
_ = _,_[_.n5]._[_][_][_]._
_._.foreach[_],_ = _[_],_._
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_table_indexer_unification_can_bound_owner_to_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3943:type_infer_tables_fuzz_table_indexer_unification_can_bound_owner_to_string`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_table_indexer_unification_can_bound_owner_to_string

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_table_indexer_unification_can_bound_owner_to_string() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
sin,_ = nil
_ = _[_.sin][_._][_][_]._
_[_] = _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_table_unify_instantiated_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3976:type_infer_tables_fuzz_table_unify_instantiated_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_table_unify_instantiated_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_table_unify_instantiated_table() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _instantiate = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function _(...)
end
local function l0():typeof(_()()[_()()[_]])
end
return _[_()()[_]] <= _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_table_unify_instantiated_table_with_prop_realloc {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3993:type_infer_tables_fuzz_table_unify_instantiated_table_with_prop_realloc`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_table_unify_instantiated_table_with_prop_realloc

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_table_unify_instantiated_table_with_prop_realloc() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _instantiate = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function _(l0,l0)
do
_ = _().n0
end
l0(_()._,_)
end
_(_,function(...)
end)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_table_unify_prop_realloc {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4013:type_infer_tables_fuzz_table_unify_prop_realloc`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_table_unify_prop_realloc

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_table_unify_prop_realloc() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
n3,_ = nil
_ = _[""]._,_[l0][_._][{[_]=_,_=_,}][_G].number
_ = {_,}
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzz_typelevel_promote_on_changed_table_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3965:type_infer_tables_fuzz_typelevel_promote_on_changed_table_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foreach (VM/src/ltablib.cpp)
  //!   - translates_to -> rust_item type_infer_tables_fuzz_typelevel_promote_on_changed_table_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzz_typelevel_promote_on_changed_table_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
_._,_ = nil
_ = _.foreach[_]._,_[_.n5]._[_.foreach][_][_]._
_ = _._
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_fuzzer_normalization_preserves_tbl_scopes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5092:type_infer_tables_fuzzer_normalization_preserves_tbl_scopes`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
  //!   - translates_to -> rust_item type_infer_tables_fuzzer_normalization_preserves_tbl_scopes

  #[cfg(test)]
  #[test]
  fn type_infer_tables_fuzzer_normalization_preserves_tbl_scopes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
Module 'l0':
do end

Module 'l1':
local _ = {n0=nil,}
if if nil then _ then
if nil and (_)._ ~= (_)._ then
do end
while _ do
_ = _
do end
end
end
do end
end
local l0
while _ do
_ = nil
(_[_])._ %= `{# _}{bit32.extract(# _,1)}`
end

"#,
      ),
      None,
    );
  }
}

mod type_infer_tables_generalization_shouldnt_seal_table_in_len_function_fn {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:37:type_infer_tables_generalization_shouldnt_seal_table_in_len_function_fn`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_generalization_shouldnt_seal_table_in_len_function_fn

  #[cfg(test)]
  #[test]
  fn type_infer_tables_generalization_shouldnt_seal_table_in_len_function_fn() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local t = {}
for i = #t, 2, -1 do
    t[i] = t[i + 1]
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = get_type_id::<TableType>(fixture.base.require_type_string(&String::from("t")))
      .expect("expected TableType");
    let indexer = t_type.indexer.expect("expected indexer");

    assert_eq!(fixture.base.get_builtins().number_type, indexer.index_type);
    assert_eq!(
      fixture.base.get_builtins().unknown_type,
      follow_type_id(indexer.index_result_type)
    );
  }
}

mod type_infer_tables_generalize_table_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3027:type_infer_tables_generalize_table_argument`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_generalize_table_argument

  #[cfg(test)]
  #[test]
  fn type_infer_tables_generalize_table_argument() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(arr)
            local work = {}
            for i = 1, #arr do
                work[i] = arr[i]
            end

            return arr
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let foo_type = fixture.require_type_string(&String::from("foo"));
    let foo_type = get_type_id::<FunctionType>(foo_type).expect("expected foo FunctionType");

    let foo_arg1 = first(foo_type.arg_types(), false).expect("expected first foo argument");
    let foo_arg1_table = get_type_id::<TableType>(follow_type_id(foo_arg1))
      .expect("expected foo first argument TableType");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(TableState::Sealed, foo_arg1_table.state);
    } else {
      assert_eq!(TableState::Generic, foo_arg1_table.state);
    }
  }
}

mod type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5621:type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method lua_exception::getStatus (VM/src/ldo.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum Status (Analysis/src/Linter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getStatus(): string
            return "Yeah can you look in returned books?"
        end
        local function getPratchettStatus()
            return { isLate = true }
        end
        type Status = { isLate: boolean, daysLate: number? }
        local key1 = "Great Expecations"
        local key2 = "The Outsiders"
        local key3 = "Guards! Guards!"
        local books: { [string]: Status } = {
            [key1] = { isLate = true, daysLate = "coconut" },
            [key2] = getStatus(),
            [key3] = getPratchettStatus()
        }
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let err0 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err0.given_type));
    assert_eq!("number?", to_string_type_id(err0.wanted_type));
    assert_eq!(
      Location::new(Position::new(12, 49), Position::new(12, 58)),
      result.errors[0].location
    );

    let err1 =
      type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err1.given_type));
    assert_eq!("Status", to_string_type_id(err1.wanted_type));
    assert_eq!(
      Location::new(Position::new(13, 21), Position::new(13, 32)),
      result.errors[1].location
    );

    let err2 =
      type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!("{ isLate: boolean }", to_string_type_id(err2.given_type));
    assert_eq!("Status", to_string_type_id(err2.wanted_type));
    assert_eq!(
      Location::new(Position::new(14, 21), Position::new(14, 41)),
      result.errors[2].location
    );
  }
}

mod type_infer_tables_generic_table_instantiation_potential_regression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3872:type_infer_tables_generic_table_instantiation_potential_regression`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_generic_table_instantiation_potential_regression

  #[cfg(test)]
  #[test]
  fn type_infer_tables_generic_table_instantiation_potential_regression() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{missing_properties::MissingProperties, type_mismatch::TypeMismatch},
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

function f(x)
  x.p = 5
  return x
end
local g : ({ p : number, q : string }) -> ({ p : number, r : boolean }) = f
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let error =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "({ p: number, q: string }) -> { p: number, r: boolean }",
        to_string_type_id(error.wanted_type)
      );
      assert_eq!(
        "({ p: number }) -> { p: number }",
        to_string_type_id(error.given_type)
      );
    } else {
      let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
        .expect("expected MissingProperties");
      assert_eq!(1, error.properties().len());
      assert_eq!("r", error.properties()[0].as_str());
    }
  }
}

mod type_infer_tables_getmetatable_returns_pointer_to_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1563:type_infer_tables_getmetatable_returns_pointer_to_metatable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_getmetatable_returns_pointer_to_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_getmetatable_returns_pointer_to_metatable() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {x = 1}
        local mt = {__index = {y = 2}}
        setmetatable(t, mt)

        local returnedMT = getmetatable(t)
    "#,
      ),
      None,
    );

    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("mt"))),
      follow_type_id(
        fixture
          .base
          .require_type_string(&String::from("returnedMT")),
      )
    );
  }
}

mod type_infer_tables_give_up_after_one_metatable_index_look_up {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2764:type_infer_tables_give_up_after_one_metatable_index_look_up`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_give_up_after_one_metatable_index_look_up

  #[cfg(test)]
  #[test]
  fn type_infer_tables_give_up_after_one_metatable_index_look_up() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local data = { x = 5 }
        local t1 = setmetatable({}, { __index = data })
        local t2 = setmetatable({}, t1) -- note: must be t1, not a new table

        local x1 = t1.x -- ok
        local x2 = t2.x -- nope
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 't2' does not have key 'x'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_hide_table_error_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2099:type_infer_tables_hide_table_error_properties`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_hide_table_error_properties

  #[cfg(test)]
  #[test]
  fn type_infer_tables_hide_table_error_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local function f()
        local function mkt() return { x = 1 } end
        local t = mkt()

        function t.a() end
        function t.b() end

        return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot add property 'a' to table '{ x: number }'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Cannot add property 'b' to table '{ x: number }'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_tables_identify_all_problematic_table_fields {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4285:type_infer_tables_identify_all_problematic_table_fields`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_identify_all_problematic_table_fields

  #[cfg(test)]
  #[test]
  fn type_infer_tables_identify_all_problematic_table_fields() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {
            a: number,
            b: string,
            c: boolean,
        }

        local a: T = {
            a = "foo",
            b = false,
            c = 123,
        }
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let err0 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(8, 16), Position::new(8, 21)),
      result.errors[0].location
    );
    assert_eq!("string", to_string_type_id(err0.given_type));
    assert_eq!("number", to_string_type_id(err0.wanted_type));

    let err1 =
      type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(9, 16), Position::new(9, 21)),
      result.errors[1].location
    );
    assert_eq!("boolean", to_string_type_id(err1.given_type));
    assert_eq!("string", to_string_type_id(err1.wanted_type));

    let err2 =
      type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(10, 16), Position::new(10, 19)),
      result.errors[2].location
    );
    assert_eq!("number", to_string_type_id(err2.given_type));
    assert_eq!("boolean", to_string_type_id(err2.wanted_type));
  }
}

mod type_infer_tables_index_expression_is_checked_against_the_indexer_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:152:type_infer_tables_index_expression_is_checked_against_the_indexer_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_index_expression_is_checked_against_the_indexer_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_index_expression_is_checked_against_the_indexer_type() {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_extend_table::CannotExtendTable, type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {[boolean]: number})
            t["hello"] = 15
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      type_error_data_ref::<CannotExtendTable>(&result.errors[0])
        .expect("expected CannotExtendTable");
    } else {
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    }
  }
}

mod type_infer_tables_index_results_compare_to_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5073:type_infer_tables_index_results_compare_to_nil`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_index_results_compare_to_nil

  #[cfg(test)]
  #[test]
  fn type_infer_tables_index_results_compare_to_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function foo(tbl: {number})
            if tbl[2] == nil then
                print("foo")
            end

            if tbl[3] ~= nil then
                print("bar")
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_indexer_and_subsequent_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7228:type_infer_tables_indexer_and_subsequent_constraint`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_indexer_and_subsequent_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexer_and_subsequent_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        local function getnumberandabs(tbl, key: string)
            local x = tbl[key]
            return math.abs(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ [string]: number }, string) -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("getnumberandabs"))
      )
    );
  }
}

mod type_infer_tables_indexer_fn {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1111:type_infer_tables_indexer_fn`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_indexer_fn

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexer_fn() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local instanace = setmetatable({}, {__index=function() return 10 end})
        local b = instanace.somemethodwedonthave
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_tables_indexer_mismatch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:848:type_infer_tables_indexer_mismatch`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_indexer_mismatch

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexer_mismatch() {
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
        local t1: { [string]: string } = {}
        local t2: { [number]: number } = {}

        t2 = t1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let t1 = fixture.require_type_string(&String::from("t1"));
    let t2 = fixture.require_type_string(&String::from("t2"));

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{number}", to_string_type_id(tm.wanted_type));
    assert_eq!("{ [string]: string }", to_string_type_id(tm.given_type));

    assert_ne!(to_string_type_id(t1), to_string_type_id(t2));
  }
}

mod type_infer_tables_indexer_on_sealed_table_must_unify_with_free_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:936:type_infer_tables_indexer_on_sealed_table_must_unify_with_free_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_indexer_on_sealed_table_must_unify_with_free_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexer_on_sealed_table_must_unify_with_free_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function F(t): {number}
            t[4] = "hi"
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_indexer_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1098:type_infer_tables_indexer_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_indexer_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexer_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local clazz = {a="hello"}
        local instanace = setmetatable({}, {__index=clazz})
        local b = instanace.a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_tables_indexers_get_quantified_too {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:703:type_infer_tables_indexers_get_quantified_too`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_indexers_get_quantified_too

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexers_get_quantified_too() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{function_type::FunctionType, generic_type::GenericType, table_type::TableType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function swap(p)
            local temp = p[0]
            p[0] = p[1]
            p[1] = temp
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "<a>({a}) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("swap")))
      );
    } else {
      let ftv = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("swap")))
        .expect("expected FunctionType");

      let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
      assert_eq!(1, arg_vec.len());

      let ttv = get_type_id::<TableType>(follow_type_id(arg_vec[0])).expect("expected TableType");
      let indexer = ttv.indexer.as_ref().expect("expected table indexer");

      assert_eq!("number", to_string_type_id(indexer.index_type));

      let index_result_type = follow_type_id(indexer.index_result_type);
      let generic = get_type_id::<GenericType>(index_result_type);
      assert!(
        generic.is_some(),
        "Expected generic but got {}",
        to_string_type_id(index_result_type)
      );
    }
  }
}

mod type_infer_tables_indexers_quantification_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:740:type_infer_tables_indexers_quantification_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_indexers_quantification_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexers_quantification_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function mergesort(arr)
            local p = arr[0]
            return arr
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("mergesort")))
      .expect("expected FunctionType");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(1, arg_vec.len());
    let arg_type =
      get_type_id::<TableType>(follow_type_id(arg_vec[0])).expect("expected argument TableType");

    let (ret_vec, _) = flatten_type_pack_id(ftv.ret_types());
    assert_eq!(1, ret_vec.len());
    let ret_type =
      get_type_id::<TableType>(follow_type_id(ret_vec[0])).expect("expected return TableType");

    assert_eq!(arg_type.state, ret_type.state);
    assert_eq!(arg_vec[0], ret_vec[0]);
  }
}

mod type_infer_tables_indexing_branching_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5185:type_infer_tables_indexing_branching_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_indexing_branching_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexing_branching_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local test = if true then { "meow", "woof" } else { 4, 81 }
        local test2 = test[1]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "number | string | string"
    } else {
      "number | string"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("test2")))
    );
  }
}

mod type_infer_tables_indexing_branching_table_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5209:type_infer_tables_indexing_branching_table_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_indexing_branching_table_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexing_branching_table_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local test = if true then {} else {}
        local test2 = test[1]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "unknown | unknown"
    } else {
      "any"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("test2")))
    );
  }
}

mod type_infer_tables_indexing_from_a_table_should_prefer_properties_when_possible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:968:type_infer_tables_indexing_from_a_table_should_prefer_properties_when_possible`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_indexing_from_a_table_should_prefer_properties_when_possible

  #[cfg(test)]
  #[test]
  fn type_infer_tables_indexing_from_a_table_should_prefer_properties_when_possible() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(): { a: string, [string]: number }
            error("e")
        end

        local t = f()

        local a1 = t.a
        local a2 = t["a"]

        local b1 = t.b
        local b2 = t["b"]

        local some_indirection_variable = "foo"
        local c = t[some_indirection_variable]

        local d = t[1]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("a1")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("a2")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b1")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b2")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );

    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("Expected a TypeMismatch");
  }
}

mod type_infer_tables_inequality_operators_imply_exactly_matching_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1328:type_infer_tables_inequality_operators_imply_exactly_matching_types`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_inequality_operators_imply_exactly_matching_types

  #[cfg(test)]
  #[test]
  fn type_infer_tables_inequality_operators_imply_exactly_matching_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function abs(n)
            if n < 0 then
                return -n
            else
                return n
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("abs")))
    );
  }
}

mod type_infer_tables_infer_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:655:type_infer_tables_infer_array`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_array

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_array() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t[1] = 'one'
        t[2] = 'two'
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ttv = get_type_id::<TableType>(fixture.require_type_string(&String::from("t")))
      .expect("expected TableType");
    let indexer = ttv.indexer.as_ref().expect("expected table indexer");

    assert_eq!("number", to_string_type_id(indexer.index_type));
    assert_eq!("string", to_string_type_id(indexer.index_result_type));
  }
}

mod type_infer_tables_infer_array_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:683:type_infer_tables_infer_array_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_array_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_array_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local buttonVector = {}

        function createButton( actionName, functionInfoTable )
            local position = nil
            for i = 1,#buttonVector do
                if buttonVector[i] == "empty" then
                    position = i
                    break
                end
            end
            return position
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_infer_indexer_for_left_unsealed_table_from_right_hand_table_with_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:881:type_infer_tables_infer_indexer_for_left_unsealed_table_from_right_hand_table_with_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_indexer_for_left_unsealed_table_from_right_hand_table_with_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_indexer_for_left_unsealed_table_from_right_hand_table_with_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): { [number]: string } return {} end

        local t = {}
        t = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_infer_indexer_from_array_like_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:771:type_infer_tables_infer_indexer_from_array_like_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_indexer_from_array_like_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_indexer_from_array_like_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {"one", "two", "three"}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ttv = get_type_id::<TableType>(fixture.require_type_string(&String::from("t")))
      .expect("expected TableType");
    let indexer = ttv.indexer.as_ref().expect("expected table indexer");

    assert_eq!("number", to_string_type_id(indexer.index_type));
    assert_eq!("string", to_string_type_id(indexer.index_result_type));
  }
}

mod type_infer_tables_infer_indexer_from_its_function_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:870:type_infer_tables_infer_indexer_from_its_function_return_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_indexer_from_its_function_return_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_indexer_from_its_function_return_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): { [number]: string }
            return {}
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_infer_indexer_from_its_variable_type_and_unifiable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:822:type_infer_tables_infer_indexer_from_its_variable_type_and_unifiable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_infer_indexer_from_its_variable_type_and_unifiable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_indexer_from_its_variable_type_and_unifiable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::{table_type::TableType, type_mismatch::TypeMismatch},
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t1: { [string]: string } = {}
        local t2 = { "bar" }

        t2 = t1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    let t2_ty = fixture.require_type_string(&String::from("t2"));
    let t_ty = get_type_id::<TableType>(t2_ty).unwrap_or_else(|| {
      panic!("Expected a table but got {}", to_string_type_id(t2_ty));
    });

    let indexer = t_ty.indexer.as_ref().expect("expected table indexer");
    assert_eq!("number", to_string_type_id(indexer.index_type));
    assert_eq!("string", to_string_type_id(indexer.index_result_type));
  }
}

mod type_infer_tables_infer_indexer_from_value_property_in_literal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:789:type_infer_tables_infer_indexer_from_value_property_in_literal`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_indexer_from_value_property_in_literal

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_indexer_from_value_property_in_literal() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function Symbol(n)
            return { __name=n }
        end

        function f()
            return {
                [Symbol("hello")] = true,
                x = 0,
                y = 0
            }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("f")))
      .expect("expected FunctionType");
    let ret_type_id = first(f_type.ret_types(), false).expect("expected return type");
    let ret_type =
      get_type_id::<TableType>(follow_type_id(ret_type_id)).expect("expected TableType");

    let indexer = ret_type.indexer.as_ref().expect("expected table indexer");
    assert_eq!("{ __name: string }", to_string_type_id(indexer.index_type));
  }
}

mod type_infer_tables_infer_type_when_indexing_from_a_table_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:953:type_infer_tables_infer_type_when_indexing_from_a_table_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_infer_type_when_indexing_from_a_table_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_type_when_indexing_from_a_table_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {string})
            return t[1]
        end

        local s = f({})
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

mod type_infer_tables_infer_write_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4365:type_infer_tables_infer_write_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_infer_write_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_infer_write_property() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t)
            t.y = 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ y: number }) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_tables_inference_in_constructor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5500:type_infer_tables_inference_in_constructor`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_inference_in_constructor

  #[cfg(test)]
  #[test]
  fn type_infer_tables_inference_in_constructor() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function new(y)
            local t: { x: number } = { x = y }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> { x: number }",
      to_string_type_id(fixture.require_type_string(&String::from("new")))
    );
  }
}

mod type_infer_tables_inferred_properties_of_a_table_should_start_with_the_same_type_level_of_that_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3515:type_infer_tables_inferred_properties_of_a_table_should_start_with_the_same_type_level_of_that_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_inferred_properties_of_a_table_should_start_with_the_same_type_level_of_that_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_inferred_properties_of_a_table_should_start_with_the_same_type_level_of_that_table()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local T = {}

        local function f(prop)
            T[1] = {
                prop = prop,
            }
        end

        local function g()
            local l = T[1].prop
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_inferring_crazy_table_should_also_be_quick {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3130:type_infer_tables_inferring_crazy_table_should_also_be_quick`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_tables_inferring_crazy_table_should_also_be_quick

  #[cfg(test)]
  #[test]
  fn type_infer_tables_inferring_crazy_table_should_also_be_quick() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function f(U)
            U(w:s(an):c()():c():U(s):c():c():U(s):c():U(s):cU()):c():U(s):c():U(s):c():c():U(s):c():U(s):cU()
        end
    "#,
        ),
        None,
    );

    let module = fixture.get_main_module(false);
    let type_count = unsafe { (*module).internal_types.types.size() };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(
        500 >= type_count,
        "expected at most 500 internal types, got {type_count}"
      );
    } else {
      assert!(
        100 >= type_count,
        "expected at most 100 internal types, got {type_count}"
      );
    }
  }
}

mod type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5034:type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop() {
    use alloc::string::String;

    use ulua_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t)
            local res = {}

            for k, a in t do
                res[k] = f(a)
                res[k] = a
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
        .expect("expected FunctionExitsWithoutReturning");
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tables_instantiate_table_cloning {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3213:type_infer_tables_instantiate_table_cloning`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_instantiate_table_cloning

  #[cfg(test)]
  #[test]
  fn type_infer_tables_instantiate_table_cloning() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
local l0:any,l61:t0<t32> = _,math
while _ do
_()
end
function _():t0<t0>
end
type t0<t32> = any
"#,
      ),
      None,
    );

    let ty = fixture.base.require_type_string(&String::from("math"));
    let ttv = get_type_id::<TableType>(ty)
      .unwrap_or_else(|| panic!("expected math TableType, got {}", to_string_type_id(ty)));
    assert!(ttv.instantiated_type_params.is_empty());
  }
}

mod type_infer_tables_instantiate_table_cloning_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3234:type_infer_tables_instantiate_table_cloning_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_instantiate_table_cloning_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_instantiate_table_cloning_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type X<T> = T
type K = X<typeof(math)>
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.base.require_type_string(&String::from("math"));
    let ttv = get_type_id::<TableType>(ty)
      .unwrap_or_else(|| panic!("expected math TableType, got {}", to_string_type_id(ty)));
    assert!(ttv.instantiated_type_params.is_empty());
  }
}

mod type_infer_tables_instantiate_table_cloning_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3251:type_infer_tables_instantiate_table_cloning_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_instantiate_table_cloning_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_instantiate_table_cloning_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X<T> = T
local a = {}
a.x = 4
local b: X<typeof(a)>
a.y = 5
local c: X<typeof(a)>
c = b
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("a"));
    let ttv = get_type_id::<TableType>(ty)
      .unwrap_or_else(|| panic!("expected a TableType, got {}", to_string_type_id(ty)));
    assert_eq!(0, ttv.instantiated_type_params.len());
  }
}

mod type_infer_tables_instantiate_tables_at_scope_level {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3113:type_infer_tables_instantiate_tables_at_scope_level`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_instantiate_tables_at_scope_level

  #[cfg(test)]
  #[test]
  fn type_infer_tables_instantiate_tables_at_scope_level() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local Option = {}
        Option.__index = Option

        function Option.Is(obj)
            return (type(obj) == "table" and getmetatable(obj) == Option)
        end

        return Option
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_instantiated_metatable_frozen_table_clone_mutation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4962:type_infer_tables_instantiated_metatable_frozen_table_clone_mutation`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_instantiated_metatable_frozen_table_clone_mutation

  #[cfg(test)]
  #[test]
  fn type_infer_tables_instantiated_metatable_frozen_table_clone_mutation() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/worker"),
        String::from(
            r#"
type WorkerImpl<T..., R...> = {
    destroy: (self: Worker<T..., R...>) -> boolean,
}

type WorkerProps = { id: number }

export type Worker<T..., R...> = typeof(setmetatable({} :: WorkerProps, {} :: WorkerImpl<T..., R...>))

return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/library"),
      String::from(
        r#"
local Worker = require(game.worker)

export type Worker<T..., R...> = Worker.Worker<T..., R...>

return {}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/library"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_intersection_of_indexers_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7242:type_infer_tables_intersection_of_indexers_1`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_intersection_of_indexers_1

  #[cfg(test)]
  #[test]
  fn type_infer_tables_intersection_of_indexers_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { [string | number]: string } & { [string | number]: unknown }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_intersection_of_indexers_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7258:type_infer_tables_intersection_of_indexers_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_intersection_of_indexers_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_intersection_of_indexers_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { [string | number]: never } & { [string | number]: string }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_intersection_of_indexers_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7274:type_infer_tables_intersection_of_indexers_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_intersection_of_indexers_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_intersection_of_indexers_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { good: boolean } & { [string]: string }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_intersection_of_read_only_and_read_write_indexer_allows_writes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4780:type_infer_tables_intersection_of_read_only_and_read_write_indexer_allows_writes`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_intersection_of_read_only_and_read_write_indexer_allows_writes

  #[cfg(test)]
  #[test]
  fn type_infer_tables_intersection_of_read_only_and_read_write_indexer_allows_writes() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function readOk(t: {read [string]: number} & {[string]: number | string})
            local _x: number = t["k"]
        end
        local function writeOk(t: {read [string]: number} & {[string]: number | string})
            t["k"] = 1
        end
        local function writeFails(t: {read [string]: number} & {[string]: number | string})
            t["k"] = "hello"
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));
    assert_eq!("string", to_string_type_id(tm.given_type));
  }
}

mod type_infer_tables_intersection_of_read_only_indexers_is_read_only {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4757:type_infer_tables_intersection_of_read_only_indexers_is_read_only`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_intersection_of_read_only_indexers_is_read_only

  #[cfg(test)]
  #[test]
  fn type_infer_tables_intersection_of_read_only_indexers_is_read_only() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::property_access_violation::{
        PropertyAccessViolation, PropertyAccessViolation_Context,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function readOk(t: {read [string]: number} & {read [string]: number | string})
            local _x: number = t["k"]
        end
        local function writeFails(t: {read [string]: number} & {read [string]: number | string})
            t["k"] = 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let av = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
    assert_eq!(
      "{ read [string]: number | string } & { read [string]: number }",
      to_string_type_id(av.table())
    );
    assert_eq!("k", av.key());
    assert_eq!(PropertyAccessViolation_Context::CannotWrite, av.context());
  }
}

mod type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2324:type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound

  #[cfg(test)]
  #[test]
  fn type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound()
   {
    use alloc::string::String;

    use ulua_analysis::records::{
      explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
      type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::has_error::has_error, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(
      &FFlag::LuauInstantiateInSubtyping,
      !FFlag::DebugLuauForceOldSolver.get(),
    );

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = {}
        function t.m(x) return x end
        local a : string = t.m("hi")
        local b : number = t.m(5)
        local u : { m : (number)->number } = t -- This shouldn't typecheck
        u.m = function(x) return 1+x end
        local c : string = t.m("hi")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert!(
        has_error::<ExplicitFunctionAnnotationRecommended>(&result),
        "{:?}",
        result.errors
      );
      assert!(has_error::<TypeMismatch>(&result), "{:?}", result.errors);
    } else {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tables_invariant_table_properties_means_instantiating_tables_in_call_is_unsound {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3828:type_infer_tables_invariant_table_properties_means_instantiating_tables_in_call_is_unsound`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_invariant_table_properties_means_instantiating_tables_in_call_is_unsound

  #[cfg(test)]
  #[test]
  fn type_infer_tables_invariant_table_properties_means_instantiating_tables_in_call_is_unsound() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _instantiate = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = {}
        function t.m<T>(x: T) return x end
        local a : string = t.m("hi")
        local b : number = t.m(5)
        function f(x : { m : (number)->number })
            x.m = function(x: number) return 1+x end
        end

        f(t) -- This shouldn't typecheck

        local c : string = t.m("hi")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location::new(Position::new(10, 10), Position::new(10, 11)),
        result.errors[0].location
      );
      let err =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "{ m: (number) -> number }",
        to_string_type_id(err.wanted_type)
      );

      let mut options = ToStringOptions {
        exhaustive: true,
        ..Default::default()
      };
      assert_eq!(
        "{ m: <T>(T) -> T }",
        to_string_type_id_to_string_options(err.given_type, &mut options)
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tables_ipairs_adds_an_unbounded_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5056:type_infer_tables_ipairs_adds_an_unbounded_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_ipairs_adds_an_unbounded_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_ipairs_adds_an_unbounded_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local a = {}
        ipairs(a)
    "#,
      ),
      None,
    );

    let mut options = ToStringOptions::new(true);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{unknown}"
    } else {
      "{'a}"
    };
    assert_eq!(
      expected,
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("a")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_key_setting_inference_given_nil_upper_bound {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1981:type_infer_tables_key_setting_inference_given_nil_upper_bound`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_key_setting_inference_given_nil_upper_bound

  #[cfg(test)]
  #[test]
  fn type_infer_tables_key_setting_inference_given_nil_upper_bound() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function setkey_object(t: { [string]: number }, v)
            t.foo = v
            t.foo = nil
        end
        local function setkey_constindex(t: { [string]: number }, v)
            t["foo"] = v
            t["foo"] = nil
        end
        local function setkey_unknown(t: { [string]: number }, k, v)
            t[k] = v
            t[k] = nil
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ [string]: number }, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("setkey_object")))
    );
    assert_eq!(
      "({ [string]: number }, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("setkey_constindex")))
    );
    assert_eq!(
      "({ [string]: number }, string, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("setkey_unknown")))
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function on_number(v: number): () end
        local function setkey_object(t: { [string]: number }, v)
            t.foo = v
            on_number(v)
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ [string]: number }, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("setkey_object")))
    );
  }
}

mod type_infer_tables_large_data_like_array_can_simplify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6782:type_infer_tables_large_data_like_array_can_simplify`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_large_data_like_array_can_simplify

  #[cfg(test)]
  #[test]
  fn type_infer_tables_large_data_like_array_can_simplify() {
    use alloc::{format, string::String};

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut source = String::from("local function get()\n");
    source.push_str("\treturn { \n");
    for i in 0..200 {
      source.push_str(&format!("\t\t{{{}, {}, {}}},\n", i, i + 1, i + 2));
      if (i % 2) != 0 {
        source.push_str(&format!("\t\t{{ foo = {} }},\n", i));
      } else {
        source.push_str(&format!("\t\t{{ bar = {} }},\n", i));
      }
    }
    source.push_str("\t}\n");
    source.push_str("end\n");

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(&source, None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> {{ bar: number } | { foo: number } | {number}}",
      to_string_type_id(fixture.require_type_string(&String::from("get")))
    );
  }
}

mod type_infer_tables_large_table_inference_does_not_bleed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5890:type_infer_tables_large_table_inference_does_not_bleed`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_large_table_inference_does_not_bleed

  #[cfg(test)]
  #[test]
  fn type_infer_tables_large_table_inference_does_not_bleed() {
    use alloc::string::String;

    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _limit = ScopedFastInt::new(&FInt::LuauPrimitiveInferenceInTableLimit, 2);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Word = "foo" | "bar"
        local words: { Word } = { "foo", "bar", "foo" }
        local otherWords: { Word } = {"foo"}
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    for err in &result.errors {
      assert_eq!(2, err.location.begin.line);
    }
  }
}

mod type_infer_tables_leaking_bad_metatable_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3644:type_infer_tables_leaking_bad_metatable_errors`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_leaking_bad_metatable_errors

  #[cfg(test)]
  #[test]
  fn type_infer_tables_leaking_bad_metatable_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a = setmetatable({}, 1)
local b = a.x
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Metatable was not a table",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'a' does not have key 'x'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_tables_length_of_array_is_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5225:type_infer_tables_length_of_array_is_number`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_length_of_array_is_number

  #[cfg(test)]
  #[test]
  fn type_infer_tables_length_of_array_is_number() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function TestFunc(ranges: {number}): number
            if true then
                ranges = {} :: {number}
            end
            local numRanges: number = #ranges
            return numRanges
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_length_operator_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2714:type_infer_tables_length_operator_intersection`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_length_operator_intersection

  #[cfg(test)]
  #[test]
  fn type_infer_tables_length_operator_intersection() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: {number} & {z:string} -- mixed tables are evil
local y = #x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_length_operator_non_table_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2724:type_infer_tables_length_operator_non_table_union`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_length_operator_non_table_union

  #[cfg(test)]
  #[test]
  fn type_infer_tables_length_operator_non_table_union() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: {number} | any | string
local y = #x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_length_operator_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2704:type_infer_tables_length_operator_union`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_length_operator_union

  #[cfg(test)]
  #[test]
  fn type_infer_tables_length_operator_union() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: {number} | {string}
local y = #x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_length_operator_union_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2734:type_infer_tables_length_operator_union_errors`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_length_operator_union_errors

  #[cfg(test)]
  #[test]
  fn type_infer_tables_length_operator_union_errors() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: {number} | number | string
local y = #x
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_luau_asser_t_arg_exprs_doesnt_trigger_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:56:type_infer_tables_luau_asser_t_arg_exprs_doesnt_trigger_assert`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_luau_asser_t_arg_exprs_doesnt_trigger_assert

  #[cfg(test)]
  #[test]
  fn type_infer_tables_luau_assert_arg_exprs_doesnt_trigger_assert() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local FadeValue = {}
function FadeValue.new(finalCallback)
	local self = setmetatable({}, FadeValue)
	self.finalCallback = finalCallback
	return self
end

function FadeValue:destroy()
	self.finalCallback()
	self.finalCallback = nil
end
"#,
      ),
      None,
    );
  }
}

mod type_infer_tables_luau_polyfill_array_includes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4126:type_infer_tables_luau_polyfill_array_includes`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_luau_polyfill_array_includes

  #[cfg(test)]
  #[test]
  fn type_infer_tables_luau_polyfill_array_includes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Array<T> = { [number]: T }

function indexOf<T>(array: Array<T>, searchElement: any, fromIndex: number?): number
	return -1
end

return function<T>(array: Array<T>, searchElement: any, fromIndex: number?): boolean
	return -1 ~= indexOf(array, searchElement, fromIndex)
end

    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_magic_functions_bidirectionally_inferred {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5337:type_infer_tables_magic_functions_bidirectionally_inferred`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_magic_functions_bidirectionally_inferred

  #[cfg(test)]
  #[test]
  fn type_infer_tables_magic_functions_bidirectionally_inferred() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getStuff(): (string, number, string)
            return "hello", 42, "world"
        end
        local t: { [string]: number } = {
            [select(1, getStuff())] = select(2, getStuff()),
            [select(3, getStuff())] = select(2, getStuff())
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getStuff(): (string, number, string)
            return "hello", 42, "world"
        end
        local t: { [string]: number } = {
            [select(1, getStuff())] = select(2, getStuff()),
            [select(3, getStuff())] = select(3, getStuff())
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err0 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(6, 38), Position::new(6, 59)),
      result.errors[0].location
    );
    assert_eq!("string", to_string_type_id(err0.given_type));
    assert_eq!("number", to_string_type_id(err0.wanted_type));
  }
}

mod type_infer_tables_meta_add {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1122:type_infer_tables_meta_add`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_meta_add

  #[cfg(test)]
  #[test]
  fn type_infer_tables_meta_add() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mt = {
            __add = function(l, r)
                return l
            end
        }
        local a = setmetatable({}, mt)
        local b = setmetatable({}, mt)
        local c = a + b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_meta_add_both_ways {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1156:type_infer_tables_meta_add_both_ways`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_meta_add_both_ways

  #[cfg(test)]
  #[test]
  fn type_infer_tables_meta_add_both_ways() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      follow_type::follow_type_id, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type VectorMt = { __add: (Vector, number) -> Vector }
        local vectorMt: VectorMt
        type Vector = typeof(setmetatable({}, vectorMt))
        local a: Vector

        local b = a + 2
        local c = 2 + a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Vector",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_meta_add_both_ways_lti {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1174:type_infer_tables_meta_add_both_ways_lti`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_meta_add_both_ways_lti

  #[cfg(test)]
  #[test]
  fn type_infer_tables_meta_add_both_ways_lti() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      follow_type::follow_type_id, to_string_to_string_alt_c::to_string_type_id,
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
        local vectorMt = {}

        function vectorMt.__add(self: Vector, other: number)
            return self
        end

        type Vector = typeof(setmetatable({}, vectorMt))
        local a: Vector = setmetatable({}, vectorMt)

        local b = a + 2
        local c = 2 + a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Vector",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_meta_add_inferred {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1143:type_infer_tables_meta_add_inferred`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_meta_add_inferred

  #[cfg(test)]
  #[test]
  fn type_infer_tables_meta_add_inferred() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        setmetatable(a, {__add=function(a,b) return b end} )
        local c = a + a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      follow_type_id(fixture.base.require_type_string(&String::from("a"))),
      follow_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_metatable_mismatch_should_fail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1576:type_infer_tables_metatable_mismatch_should_fail`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_metatable_mismatch_should_fail

  #[cfg(test)]
  #[test]
  fn type_infer_tables_metatable_mismatch_should_fail() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
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
        local t1 = {x = 1}
        local mt1 = {__index = {y = 2}}
        setmetatable(t1, mt1)

        local t2 = {x = 1}
        local mt2 = {__index = function() return nil end}
        setmetatable(t2, mt2)

        t1 = t2
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      fixture.base.require_type_string(&String::from("t1")),
      tm.wanted_type
    );
    assert_eq!(
      fixture.base.require_type_string(&String::from("t2")),
      tm.given_type
    );
  }
}

mod type_infer_tables_metatable_table_assertion_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5143:type_infer_tables_metatable_table_assertion_crash`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Index (Analysis/include/Luau/TypePath.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_metatable_table_assertion_crash

  #[cfg(test)]
  #[test]
  fn type_infer_tables_metatable_table_assertion_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local NexusInstance = {}
        function NexusInstance:__InitMetaMethods(): ()
            local Metatable = {}
            local OriginalIndexTable = getmetatable(self).__index
            setmetatable(self, Metatable)

            Metatable.__newindex = function(_, Index: string, Value: any): ()
                --Return if the new and old values are the same.
                if self[Index] == Value then
                end
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_metatable_union_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5271:type_infer_tables_metatable_union_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_tables_metatable_union_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_metatable_union_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        local function set(key, value)
            local Message = {}
            function Message.new(message)
                local self = message or {}
                setmetatable(self, Message)
                return self
            end
            local self = Message.new(nil)
            self[key] = value
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Cannot add indexer to table '{ @metatable t1, (nil & ~(false?)) | {  } } where t1 = { new: <a>(a) -> { @metatable t1, (a & ~(false?)) | {  } } }'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_tables_missing_fields_bidirectional_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5594:type_infer_tables_missing_fields_bidirectional_inference`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_missing_fields_bidirectional_inference

  #[cfg(test)]
  #[test]
  fn type_infer_tables_missing_fields_bidirectional_inference() {
    use alloc::string::String;

    use ulua_analysis::records::missing_properties::MissingProperties;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Book = { title: string, author: string }
        local b: Book = { title = "The Odyssey" }
        local t: { Book } = {
            { title = "The Illiad", author = "Homer" },
            { title = "Inferno", author = "Virgil" },
            { author = "Virgil" },
        }
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let err0 = type_error_data_ref::<MissingProperties>(&result.errors[0])
      .expect("expected MissingProperties");
    assert_eq!(1, err0.properties().len());
    assert_eq!("author", err0.properties()[0].as_str());
    assert_eq!(
      Location::new(Position::new(2, 24), Position::new(2, 49)),
      result.errors[0].location
    );

    let err1 = type_error_data_ref::<MissingProperties>(&result.errors[1])
      .expect("expected MissingProperties");
    assert_eq!(1, err1.properties().len());
    assert_eq!("title", err1.properties()[0].as_str());
    assert_eq!(
      Location::new(Position::new(6, 12), Position::new(6, 33)),
      result.errors[1].location
    );
  }
}

mod type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1620:type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred

  #[cfg(test)]
  #[test]
  fn type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::{
        metatable_type::MetatableType, table_type::TableType, type_mismatch::TypeMismatch,
      },
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
        local t = {x = 1}

        local a = {x = 1}
        local b = {__index = {y = 2}}
        setmetatable(a, b)

        t = a
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let a = fixture.base.require_type_string(&String::from("a"));
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_ne!(to_string_type_id(a), to_string_type_id(t));

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(t, tm.wanted_type);
    assert_eq!(a, tm.given_type);

    get_type_id::<MetatableType>(a).unwrap_or_else(|| {
      panic!(
        "expected metatable type for a, got {}",
        to_string_type_id(a)
      );
    });
    get_type_id::<TableType>(t).unwrap_or_else(|| {
      panic!("expected table type for t, got {}", to_string_type_id(t));
    });
  }
}

mod type_infer_tables_mixed_properties_and_indexers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3146:type_infer_tables_mixed_properties_and_indexers`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_mixed_properties_and_indexers

  #[cfg(test)]
  #[test]
  fn type_infer_tables_mixed_properties_and_indexers() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x = {}
x.a = "a"
x[0] = true
x.b = 37
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_mixed_tables_are_ok_for_any_key {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_mixed_tables_are_ok_for_any_key() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: { [any]: unknown } = {
            Key = "sorry",
            "A",
            "B",
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_mixed_tables_are_ok_when_explicit {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_mixed_tables_are_ok_when_explicit() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: { [number | string]: unknown } = {
            Key = "sorry",
            "A",
            "B",
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_mixed_tables_with_implicit_numbered_keys {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3571:type_infer_tables_mixed_tables_with_implicit_numbered_keys`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_mixed_tables_with_implicit_numbered_keys

  #[cfg(test)]
  #[test]
  fn type_infer_tables_mixed_tables_with_implicit_numbered_keys() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { [string]: number } = { 5, 6, 7 }
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      for error in &result.errors {
        assert_eq!(
          "Unexpected array-like table item: the indexer key type of this table is not `number`.",
          to_string_type_error(error)
        );
      }
    } else {
      assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[1])
      );
      assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[2])
      );
    }
  }
}

mod type_infer_tables_multiple_fields_from_fuzzer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5420:type_infer_tables_multiple_fields_from_fuzzer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_tables_multiple_fields_from_fuzzer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_multiple_fields_from_fuzzer() {
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
        function _(l0,l0) _(_,{n0=_,n0=_,},if l0:n0()[_] then _)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_multiple_fields_in_literal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5392:type_infer_tables_multiple_fields_in_literal`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_multiple_fields_in_literal

  #[cfg(test)]
  #[test]
  fn type_infer_tables_multiple_fields_in_literal() {
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
        type Foo = {
            [string]: {
                Min: number,
                Max: number
            }
        }
        local Foos: Foo = {
            ["Foo"] = {
                Min = -1,
                Max = 1
            },
            ["Foo"] = {
                Min = -1,
                Max = 1
            }
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_mymovie_read_write_tables_bug {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4923:type_infer_tables_mymovie_read_write_tables_bug`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_mymovie_read_write_tables_bug

  #[cfg(test)]
  #[test]
  fn type_infer_tables_mymovie_read_write_tables_bug() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MockedResponseBody = string | (() -> MockedResponseBody)
        type MockedResponse = { type: 'body', body: MockedResponseBody } | { type: 'error' }

        local function mockedResponseToHttpResponse(mockedResponse: MockedResponse)
            assert(mockedResponse.type == 'body', 'Mocked response is not a body')
            if typeof(mockedResponse.body) == 'string' then
            else
                return mockedResponseToHttpResponse(mockedResponse)
            end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_mymovie_read_write_tables_bug_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4942:type_infer_tables_mymovie_read_write_tables_bug_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_mymovie_read_write_tables_bug_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_mymovie_read_write_tables_bug_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MockedResponse = { type: 'body' } | { type: 'error' }

        local function mockedResponseToHttpResponse(mockedResponse: MockedResponse)
            assert(mockedResponse.type == 'body', 'Mocked response is not a body')

            if typeof(mockedResponse.body) == 'string' then
            elseif typeof(mockedResponse.body) == 'table' then
            else
                return mockedResponseToHttpResponse(mockedResponse)
            end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_narrow_table_literal_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5810:type_infer_tables_narrow_table_literal_check`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_narrow_table_literal_check

  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local dict: { code1: boolean } = {
            code1 = 123,
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.given_type));
    assert_eq!("boolean", to_string_type_id(tm.wanted_type));
  }
}

mod type_infer_tables_narrow_table_literal_check_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5845:type_infer_tables_narrow_table_literal_check_assignment`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_narrow_table_literal_check_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check_assignment() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local d1: { code1: boolean } = {
            code1 = true,
        }
        d1 = { code1 = 42 }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(5, 23), Position::new(5, 25)),
      result.errors[0].location
    );
    assert_eq!("number", to_string_type_id(tm.given_type));
    assert_eq!("boolean", to_string_type_id(tm.wanted_type));
  }
}

mod type_infer_tables_narrow_table_literal_check_call {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check_call() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function take(_: { foo: string? }) end

        take({ foo = "bar" })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_narrow_table_literal_check_call_incorrect {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check_call_incorrect() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function take(_: { foo: string?, bing: number }) end

        take({ foo = "bar", bing = true })
    "#,
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);

    let err =
      type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(3, 35), Position::new(3, 39)),
      results.errors[0].location
    );
    assert_eq!("boolean", to_string_type_id(err.given_type));
    assert_eq!("number", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_tables_narrow_table_literal_check_call_singleton {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check_call_singleton() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function take(_: { foo: "foo" }) end

        take({ foo = "foo" })
    "#,
      ),
      None,
    );

    assert_eq!(0, results.errors.len(), "{:?}", results.errors);
  }
}

mod type_infer_tables_narrow_table_literal_check_regression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5827:type_infer_tables_narrow_table_literal_check_regression`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_narrow_table_literal_check_regression

  #[cfg(test)]
  #[test]
  fn type_infer_tables_narrow_table_literal_check_regression() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local d1: { code1: boolean } = {
            code1 = true,
        }
        local d2: { [string]: number } = d1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ code1: boolean }", to_string_type_id(tm.given_type));
    assert_eq!("{ [string]: number }", to_string_type_id(tm.wanted_type));
  }
}

mod type_infer_tables_new_solver_supports_read_write_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4380:type_infer_tables_new_solver_supports_read_write_properties`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_new_solver_supports_read_write_properties

  #[cfg(test)]
  #[test]
  fn type_infer_tables_new_solver_supports_read_write_properties() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type W = {read x: number}
        type X = {write x: boolean}

        type Y = {read ["prop"]: boolean}
        type Z = {write ["prop"]: string}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_nice_error_when_trying_to_fetch_property_of_boolean {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1344:type_infer_tables_nice_error_when_trying_to_fetch_property_of_boolean`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_nice_error_when_trying_to_fetch_property_of_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_tables_nice_error_when_trying_to_fetch_property_of_boolean() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = true
        local b = a.some_prop
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'boolean' does not have key 'some_prop'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_nil_assign_doesnt_hit_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2903:type_infer_tables_nil_assign_doesnt_hit_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_nil_assign_doesnt_hit_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_nil_assign_doesnt_hit_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local a = {} a[0] = 7  a[0] = nil"),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_nil_assign_doesnt_hit_no_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2927:type_infer_tables_nil_assign_doesnt_hit_no_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_nil_assign_doesnt_hit_no_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_nil_assign_doesnt_hit_no_indexer() {
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
        local a = {a=1, b=2}
        a['a'] = nil
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 17
        },
        end: Position {
          line: 2,
          column: 20
        },
      },
      result.errors[0].location
    );

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    let builtins = fixture.get_builtins();
    assert_eq!(builtins.number_type, tm.wanted_type);
    assert_eq!(builtins.nil_type, tm.given_type);
  }
}

mod type_infer_tables_no_error_suppression_for_single_bad_type_mismatch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6993:type_infer_tables_no_error_suppression_for_single_bad_type_mismatch`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_no_error_suppression_for_single_bad_type_mismatch

  #[cfg(test)]
  #[test]
  fn type_infer_tables_no_error_suppression_for_single_bad_type_mismatch() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { a: string, b: number }): { a: any, b: boolean }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "{ a: string, b: number }",
      to_string_type_id(err.given_type)
    );
    assert_eq!("{ a: any, b: boolean }", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_tables_ok_to_add_property_to_free_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:578:type_infer_tables_ok_to_add_property_to_free_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_ok_to_add_property_to_free_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_ok_to_add_property_to_free_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function fn(d)
            d:Method()
            d.prop = true
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_ok_to_provide_a_subtype_during_construction {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2027:type_infer_tables_ok_to_provide_a_subtype_during_construction`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_ok_to_provide_a_subtype_during_construction

  #[cfg(test)]
  #[test]
  fn type_infer_tables_ok_to_provide_a_subtype_during_construction() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string | number = 1
        local t = {a, 1}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{number | string}",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("t")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1906:type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr

  #[cfg(test)]
  #[test]
  fn type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): { [string]: number }
            return { ["foo"] = 1 }
        end

        f()["foo"] = nil
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(
            t: {known_prop: boolean, [string]: number},
            key: string
        )
            t[key] = nil
            t["hello"] = nil
            t.undefined = nil
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {known_prop: boolean, [string]: number, })
            t.known_prop = nil
        end
    "#,
      ),
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 27
        },
        end: Position {
          line: 2,
          column: 30
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      "Expected this to be 'boolean', but got 'nil'",
      to_string_type_error(&result.errors[0])
    );

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type FancyHashtable with
            [string]: number
            real_property: string
        end
     "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function removekey(fh: FancyHashtable, other_key: string)
            fh["hmmm"] = nil
            fh[other_key] = nil
            fh.dne = nil
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function removekey(fh: FancyHashtable)
            fh.real_property = nil
        end
    "#,
      ),
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 31
        },
        end: Position {
          line: 2,
          column: 34
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      "Expected this to be 'string', but got 'nil'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_ok_to_set_nil_on_generic_map {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1965:type_infer_tables_ok_to_set_nil_on_generic_map`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_ok_to_set_nil_on_generic_map

  #[cfg(test)]
  #[test]
  fn type_infer_tables_ok_to_set_nil_on_generic_map() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MyMap<K, V> = { [K]: V }
        function set<K, V>(m: MyMap<K, V>, k: K, v: V)
            m[k] = v
        end
        function unset<K, V>(m: MyMap<K, V>, k: K)
            m[k] = nil
        end
        local m: MyMap<string, boolean> = {}
        set(m, "foo", true)
        unset(m, "foo")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_okay_to_add_property_to_unsealed_tables_by_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:591:type_infer_tables_okay_to_add_property_to_unsealed_tables_by_assignment`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_okay_to_add_property_to_unsealed_tables_by_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_tables_okay_to_add_property_to_unsealed_tables_by_assignment() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t = { u = {} }
        t = { u = { p = 37 } }
        t = { u = { q = "hi" } }
        local x = t.u.p
        local y = t.u.q
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_tables_okay_to_add_property_to_unsealed_tables_by_function_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:610:type_infer_tables_okay_to_add_property_to_unsealed_tables_by_function_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_okay_to_add_property_to_unsealed_tables_by_function_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_okay_to_add_property_to_unsealed_tables_by_function_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function get(x) return x.opts["MYOPT"] end
        function set(x,y) x.opts["MYOPT"] = y end
        local t = { opts = {} }
        set(t,37)
        local x = get(t)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_one_correct_one_suppressed_table_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7027:type_infer_tables_one_correct_one_suppressed_table_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_one_correct_one_suppressed_table_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_one_correct_one_suppressed_table_property() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { a: string, b: number }): { a: any, b: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_only_ascribe_synthetic_names_at_module_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2076:type_infer_tables_only_ascribe_synthetic_names_at_module_scope`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_only_ascribe_synthetic_names_at_module_scope

  #[cfg(test)]
  #[test]
  fn type_infer_tables_only_ascribe_synthetic_names_at_module_scope() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local TopLevel = {}
        local foo

        for i = 1, 10 do
            local SubScope = { 1, 2, 3 }
            foo = SubScope
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "TopLevel",
      to_string_type_id(fixture.require_type_string(&String::from("TopLevel")))
    );

    let expected_foo = if !FFlag::DebugLuauForceOldSolver.get() {
      "{number}?"
    } else {
      "{number}"
    };
    assert_eq!(
      expected_foo,
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_tables_oop_indexer_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1075:type_infer_tables_oop_indexer_works`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oop_indexer_works

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oop_indexer_works() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local clazz = {}
        clazz.__index = clazz

        function clazz:speak()
            return "hi"
        end

        function clazz.new()
            return setmetatable({}, clazz)
        end

        local me = clazz.new()
        local words = me:speak()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("words")))
    );
  }
}

mod type_infer_tables_oop_polymorphic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1229:type_infer_tables_oop_polymorphic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oop_polymorphic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oop_polymorphic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local animal = {}
        animal.__index = animal
        function animal:isAlive() return true end
        function animal:speed() return 10 end

        local pelican = {}
        setmetatable(pelican, animal)
        pelican.__index = pelican
        function pelican:movement() return "fly" end
        function pelican:speed() return 30 end

        function pelican.new(name)
            local s = {}
            setmetatable(s, pelican)
            s.name = name
            return s
        end

        local scoops = pelican.new("scoops")

        local alive = scoops:isAlive()
        local at = scoops.isAlive
        local movement = scoops:movement()
        local name = scoops.name
        local speed = scoops:speed()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("alive")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("movement")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("name")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("speed")))
    );
  }
}

mod type_infer_tables_open_table_unification_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:419:type_infer_tables_open_table_unification_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_open_table_unification_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_open_table_unification_2() {
    use alloc::string::String;

    use ulua_analysis::records::missing_properties::MissingProperties;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        a.x = 99

        function a:method()
            return self.y
        end
        a:method()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
      .expect("expected MissingProperties");
    assert_eq!(1, error.properties().len());
    assert_eq!("y", error.properties()[0].as_str());
    assert_eq!(
      Location {
        begin: Position { line: 7, column: 8 },
        end: Position { line: 7, column: 9 },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_tables_open_table_unification_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:447:type_infer_tables_open_table_unification_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_open_table_unification_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_open_table_unification_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id(x)
            return x
        end

        function foo(o)
            id(o.bar)
            id(o.baz)
        end
    "#,
      ),
      None,
    );

    let foo_type = fixture.require_type_string(&String::from("foo"));
    let foo_fn = get_type_id::<FunctionType>(foo_type).expect("expected FunctionType");

    let (foo_args, _) = flatten_type_pack_id(foo_fn.arg_types());
    assert_eq!(1, foo_args.len());

    let arg0 = foo_args[0];
    let arg0_table = get_type_id::<TableType>(follow_type_id(arg0)).expect("expected TableType");

    assert!(arg0_table.props.contains_key("bar"));
    assert!(arg0_table.props.contains_key("baz"));
  }
}

mod type_infer_tables_optional_function_in_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5540:type_infer_tables_optional_function_in_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_optional_function_in_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_optional_function_in_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { (() -> ())? } = {
            function() end,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { ((number) -> ())? } = {
            function(_: string) end,
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(2, 12), Position::new(2, 35)),
      result.errors[0].location
    );
    assert_eq!("(string) -> ()", to_string_type_id(err.given_type));
    assert_eq!("((number) -> ())?", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_tables_optional_property_with_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5462:type_infer_tables_optional_property_with_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_optional_property_with_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_optional_property_with_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type t = {
            key: boolean?,
            time: number,
        }

        local function num(): number
            return 0
        end

        local _: t = {
            time = num(),
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1344 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1344() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type t = {
        	value: string?,
        }

        local t: t = {}

        if not t.value then
        	t.value = ""
        end

        local s: string? = nil

        if not s then
        	s = ""
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1450 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1450() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local keycodes = {
            Alt = 2,
            Space = 3,
            Tab = 4,
        }

        type Keycode = keyof<typeof(keycodes)>
        local function sendInput(keycodes: { Keycode })
            print(keycodes)
        end

        sendInput({"Alt"}) -- shouldn't error
        sendInput(
            {
                "Alt",
                "Space",
                "Ctrl", -- should error
            }
        )
    "#,
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location::new(Position::new(17, 16), Position::new(17, 22)),
      results.errors[0].location
    );
    assert_eq!(
      "\"Alt\" | \"Space\" | \"Tab\"",
      to_string_type_id(err.wanted_type)
    );
    assert_eq!("\"Ctrl\"", to_string_type_id(err.given_type));
  }
}

mod type_infer_tables_oss_1483 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6486:type_infer_tables_oss_1483`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_1483

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1483() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Form = "do-not-register" | (() -> ())

        local function observer(register: () -> Form) end

        observer(function()
            if math.random() > 0.5 then
                return "do-not-register"
            end
            return function() end
        end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1543_optional_generic_param {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5583:type_infer_tables_oss_1543_optional_generic_param`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_1543_optional_generic_param

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1543_optional_generic_param() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type foo<T> = { bar: T? }

        local foo: foo<any> = { bar = "foobar" }
        local foo: foo<any> = { }
        local foo: foo<nil> = { }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1596_expression_in_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5564:type_infer_tables_oss_1596_expression_in_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_1596_expression_in_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1596_expression_in_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type foo = {abc: number?}
        local x: foo = {abc = 100}
        local y: foo = {abc = 10 * 10}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1615_parametrized_type_alias {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5573:type_infer_tables_oss_1615_parametrized_type_alias`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Node (Analysis/src/TopoSortStatements.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_1615_parametrized_type_alias

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1615_parametrized_type_alias() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Pair<Node> = { sep: {}? }
        local a: Pair<{}> = {
            sep = nil,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1651 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1651() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local MyModule = {}
        MyModule._isEnabled = true :: boolean

        assert(MyModule._isEnabled, `type solver`)
        MyModule._isEnabled = false
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1684 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6633:type_infer_tables_oss_1684`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_1684

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1684() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local targetConfig = { ["Bag of coins"] = {}, }

        type TargetConfig = typeof(targetConfig)
        type Targets = keyof<TargetConfig>

        type QuestConfig = { target: Targets, }

        -- All of the table members that aren't "Bag of coins" should
        -- have errors.
        local questConfig: { [string]: QuestConfig  } = {
            ["Works as intended"] = { target = "Bag of coins" },
            ["Also works as intended "] = { target = "Not bag of coins" },
            ["Should warn 1"] = { target = "Also not a bag of coins" },
            ["Should warn 2"] = { target = "Still not a bag of coins" },
        }
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    for error in &result.errors {
      type_error_data_ref::<TypeMismatch>(error).expect("expected TypeMismatch");
    }
  }
}

mod type_infer_tables_oss_1797_intersection_of_tables_arent_disjoint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5958:type_infer_tables_oss_1797_intersection_of_tables_arent_disjoint`
  //! Source: `tests/TypeInfer.tables.test.cpp`

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1797_intersection_of_tables_arent_disjoint() {
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

        export type Foo = {
            foo: string,
        }

        export type Bar = Foo & {
            copy: (...any) -> any
        }

        local function _test(nd: { bar: Bar? })
            local bar = nd.bar
            if not bar then
                return
            end
            print(bar)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Foo & { copy: (...any) -> any }",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(16, 20))
      )
    );
  }
}

mod type_infer_tables_oss_1838 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1838() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local myTable = {}
        myTable.foo = {}
        myTable.foo.bar = {}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1859 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1859() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type Cat = {
            name: string,
            age: number,
            actions: {
                otherfield: string,
                meow: () -> string,
            }
        }

        local function new(): Cat
            local self = {}
            self.name = "Taz"
            self.age = 12
            self.actions = {}
            self.actions.meow = function() return "meow" end
            -- We're missing `otherfield` here so we should complain.
            return self
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Cat", to_string_type_id(err.wanted_type));
    assert_eq!(
      "{ actions: { meow: (...any) -> string }, age: number, name: string }",
      to_string_type_id(err.given_type)
    );
  }
}

mod type_infer_tables_oss_1888_and_or_subscriptable {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1888_and_or_subscriptable() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type CachedValue<T> = {
            future: any,
            timestamp: number,
            ttl: number?,
        }

        type Cache<T> = { [string]: CachedValue<T> }
        type CacheMap = { [string]: Cache<any> }

        local _caches: CacheMap = {}

        local CacheManager = {}

        function CacheManager:has(cacheName: string, id: string): boolean
            local cache = _caches[cacheName]
            local entry = cache and cache[id]
            return entry ~= nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1890 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6924:type_infer_tables_oss_1890`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_1890

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1890() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type ListConfig<T> = {
            items: T,
            each: (item: T) -> any,
            a: string?,
        }

        local function test_fn<T>(p: ListConfig<T>)
            return nil :: any
        end

        local a = test_fn {
            items = "a",
            each = function(item: string)
                return item
            end,
            a = "a",
        }

        a = test_fn {
            items = "a",
            each = function(item: string)
                return item
            end,
        }

    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1910 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6507:type_infer_tables_oss_1910`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_1910

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1910() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Implementation = {
            on_thing: (something: boolean) -> (),
        }

        local a: Implementation = {
            on_thing = function(something)
                local _ = something
            end,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 29)))
    );
  }
}

mod type_infer_tables_oss_1914_access_after_assignment_with_assertion {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1914_access_after_assignment_with_assertion() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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

        type WallHolder = {
            __type: "Model",
            Wall: {
                __type: "BasePart",
                age: number,
            },
        }

        local walls = {
            { name = "Part1" },
            { name = "Part2" },
            { name = "Wall" },
        }

        local baseWall: WallHolder?
        for _, wall in walls do
            if wall.name == "Wall" then
                baseWall = wall :: WallHolder
            end
        end
        assert(baseWall, "Failed to get base wall when creating room props")

        local myAge = baseWall.Wall.age
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("myAge")))
    );
  }
}

mod type_infer_tables_oss_1924 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1924() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { [string]: "s" } = {
            key = "s",
            other_key = "t",
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("\"s\"", to_string_type_id(err.wanted_type));
    assert_eq!("\"t\"", to_string_type_id(err.given_type));
  }
}

mod type_infer_tables_oss_1935 {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1935() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Drawing = {
            update: (() -> boolean)?,
        }

        type Counter = {
            count: number,
        }

        function update(): boolean
            return true
        end

        return function(): Drawing & Counter
            return {
                count = 34,
                update = update,
            }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1947_partial {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6912:type_infer_tables_oss_1947_partial`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_1947_partial

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1947_partial() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo<T>(bar: { qux: T, baz: string? }) end
        foo { qux = "string", baz = "a" }
        foo { qux = "string", baz = nil }
        foo { qux = "string" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_1953 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6445:type_infer_tables_oss_1953`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MissingUnionProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_1953

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1953() {
    use alloc::string::String;

    use ulua_analysis::records::missing_union_property::MissingUnionProperty;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { kind: "a" }
        type B = { kind: "b" }

        local function foo<T>(fn: () -> A | B | T)
            local v = fn()
            return v and v.kind
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<MissingUnionProperty>(&result.errors[0])
      .expect("expected MissingUnionProperty");
    assert_eq!("kind", err.key());
  }
}

mod type_infer_tables_oss_1986 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6899:type_infer_tables_oss_1986`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_1986

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_1986() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A<T> = { s: T, n: number? }

        local function f<T>(_a: A<T>)
            return
        end

        f({ s = "hello", n = 1 })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_2017 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6419:type_infer_tables_oss_2017`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_oss_2017

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_2017() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local class = {}
        class.__index = class

        type Role = string
        local ROLES_PLAYERS_REQUIRED: {[Role]: number} = {}

        function class.distributeRoles()
            for _, role: Role in class.getAllRoles() do
                local _ = ROLES_PLAYERS_REQUIRED[role]
            end
        end

        function class.getAllRoles(): { Role }
            return { 'Citizen', 'Mafia', 'Detective', 'Bodyguard', 'Jester' }
        end

        return class
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_oss_2094_push_type_constraint_should_always_complete {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6664:type_infer_tables_oss_2094_push_type_constraint_should_always_complete`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_oss_2094_push_type_constraint_should_always_complete

  #[cfg(test)]
  #[test]
  fn type_infer_tables_oss_2094_push_type_constraint_should_always_complete() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Interface<T> = {
            _t: T,
        }

        type function TypeFn(t: type): type
            return t
        end

        local function new<T>(t: T): Interface<TypeFn<T>>
            return {
                _t = nil :: any,
            }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_pairs_parameters_are_not_unsealed_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3489:type_infer_tables_pairs_parameters_are_not_unsealed_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_pairs_parameters_are_not_unsealed_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_pairs_parameters_are_not_unsealed_tables() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(l0:{n0:any})
            _ = pairs
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_another_parameter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4880:type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_another_parameter`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_another_parameter

  #[cfg(test)]
  #[test]
  fn type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_another_parameter() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t1, t2)
            t1[5] = 7 -- 't1 <: {number}
            t2 = t1   -- 't1 <: 't2
            t1[5] = 7 -- 't1 <: {number}
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({number} & {number}, unknown) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4861:type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_string`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_string

  #[cfg(test)]
  #[test]
  fn type_infer_tables_parameter_was_set_an_indexer_and_bounded_by_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t)
            local s: string = t
            t[5] = 7
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Parameter 't' has been reduced to never. This function is not callable with any possible value.",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Parameter 't' is required to be a subtype of 'string' here.",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Parameter 't' is required to be a subtype of '{number}' here.",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2794:type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: {x: number, y: number, [any]: any} | {y: number}

        function f(t)
            t.y = 1
            return t
        end

        local b = f(a)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ y: number }"
    } else {
      "{- y: number -}"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2815:type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_pass_a_union_of_tables_to_a_function_that_requires_a_table_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: {y: number} | {x: number, y: number, [any]: any}

        function f(t)
            t.y = 1
            return t
        end

        local b = f(a)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ y: number }"
    } else {
      "{- y: number -}"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_tables_pass_incompatible_union_to_a_generic_table_without_crashing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1417:type_infer_tables_pass_incompatible_union_to_a_generic_table_without_crashing`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_pass_incompatible_union_to_a_generic_table_without_crashing

  #[cfg(test)]
  #[test]
  fn type_infer_tables_pass_incompatible_union_to_a_generic_table_without_crashing() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- must be in this specific order, and with (roughly) those exact properties!
        type A = {x: number, [any]: any} | {}

        function f(t)
            t.y = 1
        end

        function g(a: A)
            f(a)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  }
}

mod type_infer_tables_passing_compatible_unions_to_a_generic_table_without_crashing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1439:type_infer_tables_passing_compatible_unions_to_a_generic_table_without_crashing`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_passing_compatible_unions_to_a_generic_table_without_crashing

  #[cfg(test)]
  #[test]
  fn type_infer_tables_passing_compatible_unions_to_a_generic_table_without_crashing() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number, y: number, [any]: any} | {y: number}

        function f(t)
            t.y = 1
        end

        function g(a: A)
            f(a)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_persistent_sealed_table_is_immutable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2135:type_infer_tables_persistent_sealed_table_is_immutable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_persistent_sealed_table_is_immutable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_persistent_sealed_table_is_immutable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function os:bad() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot add property 'bad' to table 'typeof(os)'",
      to_string_type_error(&result.errors[0])
    );

    let os_type = fixture.base.require_type_string(&String::from("os"));
    let os_table = get_type_id::<TableType>(os_type).unwrap_or_else(|| {
      panic!(
        "expected table type for os, got {}",
        to_string_type_id(os_type)
      );
    });
    assert!(!os_table.props.contains_key("bad"));
  }
}

mod type_infer_tables_prop_access_on_key_whose_types_mismatches {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3622:type_infer_tables_prop_access_on_key_whose_types_mismatches`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_prop_access_on_key_whose_types_mismatches

  #[cfg(test)]
  #[test]
  fn type_infer_tables_prop_access_on_key_whose_types_mismatches() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {number} = {}
        local x = t.x
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Key 'x' not found in table '{number}'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_prop_access_on_unions_of_indexers_where_key_whose_types_mismatches {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3633:type_infer_tables_prop_access_on_unions_of_indexers_where_key_whose_types_mismatches`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_prop_access_on_unions_of_indexers_where_key_whose_types_mismatches

  #[cfg(test)]
  #[test]
  fn type_infer_tables_prop_access_on_unions_of_indexers_where_key_whose_types_mismatches() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { [number]: number } | { [boolean]: number } = {}
        local u = t.x
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type '{ [boolean]: number } | {number}' does not have key 'x'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_property_lookup_through_tabletypevar_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1601:type_infer_tables_property_lookup_through_tabletypevar_metatable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_property_lookup_through_tabletypevar_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_property_lookup_through_tabletypevar_metatable() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {x = 1}
        local mt = {__index = {y = 2}}
        setmetatable(t, mt)

        print(t.x)
        print(t.y)
        print(t.z)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let up =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("z", up.key());
  }
}

mod type_infer_tables_read_and_write_only_indexers_are_unsupported {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4348:type_infer_tables_read_and_write_only_indexers_are_unsupported`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_read_and_write_only_indexers_are_unsupported

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_and_write_only_indexers_are_unsupported() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {read [string]: number}
        type U = {write [string]: boolean}
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "read keyword is illegal here",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location::new(Position::new(1, 18), Position::new(1, 22)),
      result.errors[0].location
    );
    assert_eq!(
      "write keyword is illegal here",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      Location::new(Position::new(2, 18), Position::new(2, 23)),
      result.errors[1].location
    );
  }
}

mod type_infer_tables_read_and_write_only_table_properties_are_unsupported_type_infer_tables_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4324:type_infer_tables_read_and_write_only_table_properties_are_unsupported`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_read_and_write_only_table_properties_are_unsupported

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_and_write_only_table_properties_are_unsupported() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type W = {read x: number}
        type X = {write x: boolean}

        type Y = {read ["prop"]: boolean}
        type Z = {write ["prop"]: string}
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "read keyword is illegal here",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location::new(Position::new(1, 18), Position::new(1, 22)),
      result.errors[0].location
    );
    assert_eq!(
      "write keyword is illegal here",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      Location::new(Position::new(2, 18), Position::new(2, 23)),
      result.errors[1].location
    );
    assert_eq!(
      "read keyword is illegal here",
      to_string_type_error(&result.errors[2])
    );
    assert_eq!(
      Location::new(Position::new(4, 18), Position::new(4, 22)),
      result.errors[2].location
    );
    assert_eq!(
      "write keyword is illegal here",
      to_string_type_error(&result.errors[3])
    );
    assert_eq!(
      Location::new(Position::new(5, 18), Position::new(5, 23)),
      result.errors[3].location
    );
  }
}

mod type_infer_tables_read_and_write_only_table_properties_are_unsupported_type_infer_tables_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4581:type_infer_tables_read_and_write_only_table_properties_are_unsupported`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_read_and_write_only_table_properties_are_unsupported

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_and_write_only_table_properties_are_unsupported() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type W = {read x: number}
        type X = {write x: boolean}

        type Y = {read ["prop"]: boolean}
        type Z = {write ["prop"]: string}
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "read keyword is illegal here",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location::new(Position::new(1, 18), Position::new(1, 22)),
      result.errors[0].location
    );
    assert_eq!(
      "write keyword is illegal here",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      Location::new(Position::new(2, 18), Position::new(2, 23)),
      result.errors[1].location
    );
    assert_eq!(
      "read keyword is illegal here",
      to_string_type_error(&result.errors[2])
    );
    assert_eq!(
      Location::new(Position::new(4, 18), Position::new(4, 22)),
      result.errors[2].location
    );
    assert_eq!(
      "write keyword is illegal here",
      to_string_type_error(&result.errors[3])
    );
    assert_eq!(
      Location::new(Position::new(5, 18), Position::new(5, 23)),
      result.errors[3].location
    );
  }
}

mod type_infer_tables_read_from_write_only_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4468:type_infer_tables_read_from_write_only_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_read_from_write_only_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_from_write_only_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{
        property_access_violation::{PropertyAccessViolation, PropertyAccessViolation_Context},
        to_string_options::ToStringOptions,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {write x: number})
            local foo = t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property x of table '{ write x: number }' is write-only",
      to_string_type_error(&result.errors[0])
    );

    let pav = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{ write x: number }",
      to_string_type_id_to_string_options(pav.table(), &mut options)
    );
    assert_eq!("x", pav.key());
    assert_eq!(PropertyAccessViolation_Context::CannotRead, pav.context());
  }
}

mod type_infer_tables_read_only_array_shorthand {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4678:type_infer_tables_read_only_array_shorthand`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_array_shorthand

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_array_shorthand() {
    use alloc::string::String;

    use ulua_analysis::records::property_access_violation::{
      PropertyAccessViolation, PropertyAccessViolation_Context,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {read number} = {1, 2, 3}
        local x: number = t[1]
        t[1] = 4
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let pav = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
    assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
  }
}

mod type_infer_tables_read_only_indexer_basic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4605:type_infer_tables_read_only_indexer_basic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_basic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_basic() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {read [string]: number}
        type A = {read number}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_read_only_indexer_cannot_cover_readwrite_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4738:type_infer_tables_read_only_indexer_cannot_cover_readwrite_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_cannot_cover_readwrite_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_cannot_cover_readwrite_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local ro: {read [string]: number} = {}
        local t: {foo: number} = ro
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ foo: number }", to_string_type_id(tm.wanted_type));
    assert_eq!(
      "{ read [string]: number }",
      to_string_type_id(tm.given_type)
    );
  }
}

mod type_infer_tables_read_only_indexer_covariance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4634:type_infer_tables_read_only_indexer_covariance`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_covariance

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_covariance() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local rw: {[string]: number} = {}
        local ro: {read [string]: number} = rw
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_read_only_indexer_not_subtype_of_readwrite {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4647:type_infer_tables_read_only_indexer_not_subtype_of_readwrite`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_not_subtype_of_readwrite

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_not_subtype_of_readwrite() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local ro: {read [string]: number} = {}
        local rw: {[string]: number} = ro
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ [string]: number }", to_string_type_id(tm.wanted_type));
    assert_eq!(
      "{ read [string]: number }",
      to_string_type_id(tm.given_type)
    );
  }
}

mod type_infer_tables_read_only_indexer_read_allowed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4726:type_infer_tables_read_only_indexer_read_allowed`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_read_allowed

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_read_allowed() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {read [string]: number} = {}
        local x: number = t["k"]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_read_only_indexer_tostring {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4714:type_infer_tables_read_only_indexer_tostring`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_tostring

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_tostring() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {read [string]: number} = {}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ read [string]: number }",
      to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_tables_read_only_indexer_value_covariance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4665:type_infer_tables_read_only_indexer_value_covariance`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_value_covariance

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_value_covariance() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local narrow: {read [string]: number} = {}
        local wide: {read [string]: number | string} = narrow
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_read_only_indexer_value_not_contravariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4696:type_infer_tables_read_only_indexer_value_not_contravariant`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_value_not_contravariant

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_value_not_contravariant() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local wide: {read [string]: number | string} = {}
        local narrow: {read [string]: number} = wide
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "{ read [string]: number }",
      to_string_type_id(tm.wanted_type)
    );
    assert_eq!(
      "{ read [string]: number | string }",
      to_string_type_id(tm.given_type)
    );
  }
}

mod type_infer_tables_read_only_indexer_write_rejected {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4618:type_infer_tables_read_only_indexer_write_rejected`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_indexer_write_rejected

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_indexer_write_rejected() {
    use alloc::string::String;

    use ulua_analysis::records::property_access_violation::{
      PropertyAccessViolation, PropertyAccessViolation_Context,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {read [string]: number} = {}
        t["k"] = 1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let pav = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
    assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
  }
}

mod type_infer_tables_read_only_property_reads {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5372:type_infer_tables_read_only_property_reads`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_read_only_property_reads

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_property_reads() {
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
        type readonlyTable = {read id: number}
        local t:readonlyTable = {id = 1}

        local _:{number} = {[t.id] = 1}
        local _:{number} = {[t.id::number] = 1}

        local arr:{number} = {}
        arr[t.id] = 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_read_only_property_subtype_mismatch_error_message {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4525:type_infer_tables_read_only_property_subtype_mismatch_error_message`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_property_subtype_mismatch_error_message

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_property_subtype_mismatch_error_message() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _property_modifier_mismatch_errors =
      ScopedFastFlag::new(&FFlag::LuauPropertyModifierMismatchErrors, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { read woof: number }): { woof: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Expected this to be\n\t'{ woof: number }'\nbut got\n\t'{ read woof: number }'; \n`woof` is a read-only property in the latter type, but the former type requires a read-write property",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_read_only_property_with_type_mismatch_reports_both_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4505:type_infer_tables_read_only_property_with_type_mismatch_reports_both_errors`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_read_only_property_with_type_mismatch_reports_both_errors

  #[cfg(test)]
  #[test]
  fn type_infer_tables_read_only_property_with_type_mismatch_reports_both_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _property_modifier_mismatch_errors =
      ScopedFastFlag::new(&FFlag::LuauPropertyModifierMismatchErrors, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { read woof: string }): { woof: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let msg = to_string_type_error(&result.errors[0]);
    assert!(
      msg.contains(
        "accessing `woof` results in `string` in the latter type and `number` in the former type"
      ),
      "{}",
      msg
    );
    assert!(
        msg.contains("`woof` is a read-only property in the latter type, but the former type requires a read-write property"),
        "{}",
        msg
    );
  }
}

mod type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2038:type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table()
  {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_extend_table::CannotExtendTable, unknown_property::UnknownProperty,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function mkA() return {"value"} end
        local A = mkA()
        A.B = "Hello"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let cet = type_error_data_ref::<CannotExtendTable>(&result.errors[0])
        .unwrap_or_else(|| panic!("Expected CannotExtendTable but got {:?}", result.errors[0]));
      assert_eq!("B", cet.prop());
    } else {
      let up = type_error_data_ref::<UnknownProperty>(&result.errors[0])
        .unwrap_or_else(|| panic!("Expected UnknownProperty but got {:?}", result.errors[0]));
      assert_eq!("B", up.key());
    }
  }
}

mod type_infer_tables_record_location_of_inserted_table_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3273:type_infer_tables_record_location_of_inserted_table_properties`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_record_location_of_inserted_table_properties

  #[cfg(test)]
  #[test]
  fn type_infer_tables_record_location_of_inserted_table_properties() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        a.foo = 1234
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a_type = fixture.require_type_string(&String::from("a"));
    let table_type = get_type_id::<TableType>(a_type)
      .unwrap_or_else(|| panic!("expected TableType, got {}", to_string_type_id(a_type)));
    let prop = table_type.props.get("foo").expect("expected property foo");

    assert_eq!(
      Some(Location::new(Position::new(2, 10), Position::new(2, 13))),
      prop.location
    );
  }
}

mod type_infer_tables_recursive_metatable_type_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2655:type_infer_tables_recursive_metatable_type_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_recursive_metatable_type_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_recursive_metatable_type_call() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local b
b = setmetatable({}, {__call = b})
b()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot call a value of type t1 where t1 = { @metatable {| __call: t1 |}, {|  |} }",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_refined_thing_can_be_an_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4845:type_infer_tables_refined_thing_can_be_an_array`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_refined_thing_can_be_an_array

  #[cfg(test)]
  #[test]
  fn type_infer_tables_refined_thing_can_be_an_array() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(x, y)
            if x then
                return x[1]
            else
                return y
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a>({a}, a) -> a",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_tables_report_sensible_error_when_adding_a_value_to_a_nonexistent_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:229:type_infer_tables_report_sensible_error_when_adding_a_value_to_a_nonexistent_prop`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_report_sensible_error_when_adding_a_value_to_a_nonexistent_prop

  #[cfg(test)]
  #[test]
  fn type_infer_tables_report_sensible_error_when_adding_a_value_to_a_nonexistent_prop() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t.foo[1] = 'one'
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("t", to_string_type_id(err.table()));
    assert_eq!("foo", err.key());
  }
}

mod type_infer_tables_result_is_always_any_if_lhs_is_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1280:type_infer_tables_result_is_always_any_if_lhs_is_any`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_result_is_always_any_if_lhs_is_any

  #[cfg(test)]
  #[test]
  fn type_infer_tables_result_is_always_any_if_lhs_is_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Vector3MT = {
            __add: (Vector3MT, Vector3MT) -> Vector3MT,
            __mul: (Vector3MT, Vector3MT|number) -> Vector3MT
        }

        local Vector3: {new: (number?, number?, number?) -> Vector3MT}
        local Vector3MT: Vector3MT
        setmetatable(Vector3, Vector3MT)

        type CFrameMT = {
            __mul: (CFrameMT, Vector3MT|CFrameMT) -> Vector3MT|CFrameMT
        }

        local CFrame: {
            Angles:(number, number, number) -> CFrameMT
        }
        local CFrameMT: CFrameMT
        setmetatable(CFrame, CFrameMT)

        local n: any
        local a = (n + Vector3.new(0, 1.5, 0)) * CFrame.Angles(0, math.pi/2, 0)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_tables_result_is_bool_for_equality_operators_if_lhs_is_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1311:type_infer_tables_result_is_bool_for_equality_operators_if_lhs_is_any`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_result_is_bool_for_equality_operators_if_lhs_is_any

  #[cfg(test)]
  #[test]
  fn type_infer_tables_result_is_bool_for_equality_operators_if_lhs_is_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(): (any, number)
            return 5, 7
        end

        local a: any, b: number = f()

        local c = a < b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_result_like_tagged_union {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_result_like_tagged_union() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local function retry(func: (...any) -> ...any): { type: "ok", value: any } | { type: "failed" }
    local success: boolean, result: any = func()

    if success then
        return { type = "ok", value = result }
    else
        return { type = "failed" }
    end
end

return retry
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_returning_mismatched_optional_in_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5521:type_infer_tables_returning_mismatched_optional_in_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_returning_mismatched_optional_in_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_returning_mismatched_optional_in_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Numbers = { str = ( "" :: string ) }
        local function FuncB(): { Value: number? }
            return {
                Value = Numbers.str
            }
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err.given_type));
    assert_eq!("number?", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_tables_returning_optional_in_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5511:type_infer_tables_returning_optional_in_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_returning_optional_in_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_returning_optional_in_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Numbers = { zero = 0 }
        local function FuncA(): { Value: number? }
            return { Value = Numbers.zero }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_right_table_missing_key {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1655:type_infer_tables_right_table_missing_key`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_right_table_missing_key

  #[cfg(test)]
  #[test]
  fn type_infer_tables_right_table_missing_key() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(...)
        end
        local l7 = not _,function(l0)
        _ += _((_) or {function(...)
        end,["z"]=_,} or {},(function(l43,...)
        end))
        _ += 0 < {}
        end
        repeat
        until _
        local l0 = n4,_((_) or {} or {[30976]=_,},({}))
    "#,
      ),
      None,
    );

    let _ = result.errors.len();
  }
}

mod type_infer_tables_right_table_missing_key_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1675:type_infer_tables_right_table_missing_key_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_right_table_missing_key_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_right_table_missing_key_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::missing_properties::{Context, MissingProperties},
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {}): { [string]: string, a: string }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mp = type_error_data_ref::<MissingProperties>(&result.errors[0])
      .unwrap_or_else(|| panic!("Expected MissingProperties but got {:?}", result.errors[0]));
    assert_eq!(Context::Missing, mp.context());
    assert_eq!(1, mp.properties().len());
    assert_eq!("a", mp.properties()[0].as_str());

    assert_eq!(
      "{ [string]: string, a: string }",
      to_string_type_id(mp.super_type())
    );
    assert_eq!("{  }", to_string_type_id(mp.sub_type()));
  }
}

mod type_infer_tables_scalar_is_a_subtype_of_a_compatible_polymorphic_shape_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3656:type_infer_tables_scalar_is_a_subtype_of_a_compatible_polymorphic_shape_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_scalar_is_a_subtype_of_a_compatible_polymorphic_shape_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_scalar_is_a_subtype_of_a_compatible_polymorphic_shape_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(s)
            return s:lower()
        end

        f("foo" :: string)
        f("bar" :: "bar")
        f("baz" :: "bar" | "baz")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3674:type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(s)
            return s:absolutely_no_scalar_has_this_method()
        end

        f("foo" :: string)
        f("bar" :: "bar")
        f("baz" :: "bar" | "baz")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(4, result.errors.len(), "{:?}", result.errors);

      for error in &result.errors {
        let tm = type_error_data_ref::<TypeMismatch>(error).expect("expected TypeMismatch");
        assert_eq!("typeof(string)", to_string_type_id(tm.given_type));
        assert_eq!(
          "t1 where t1 = { read absolutely_no_scalar_has_this_method: (t1) -> (a...) }",
          to_string_type_id(tm.wanted_type)
        );
      }
    } else {
      assert_eq!(3, result.errors.len(), "{:?}", result.errors);

      let expected1 = "Expected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got 'string'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
      assert_eq!(expected1, to_string_type_error(&result.errors[0]));

      let expected2 = "Expected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got '\"bar\"'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));

      let expected3 = "Expected this to be\n\t't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}'\nbut got\n\t'\"bar\" | \"baz\"'\ncaused by:\n  Not all union options are compatible.\nExpected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got '\"bar\"'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
      assert_eq!(expected3, to_string_type_error(&result.errors[2]));
    }
  }
}

mod type_infer_tables_sealed_table_indexers_must_unify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:914:type_infer_tables_sealed_table_indexers_must_unify`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_sealed_table_indexers_must_unify

  #[cfg(test)]
  #[test]
  fn type_infer_tables_sealed_table_indexers_must_unify() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: {number}): {string}
            return a
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be '{string}', but got '{number}'; \n\
the result of indexing is `number` in the latter type and `string` in the former type, \
and `number` is not exactly `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("Expected a TypeMismatch");
    }
  }
}

mod type_infer_tables_sealed_table_value_can_infer_an_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:893:type_infer_tables_sealed_table_value_can_infer_an_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_sealed_table_value_can_infer_an_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_sealed_table_value_can_infer_an_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { a: string, [number]: string } = { a = "foo" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_setindexer_multiple_tables_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5019:type_infer_tables_setindexer_multiple_tables_intersection`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_setindexer_multiple_tables_intersection

  #[cfg(test)]
  #[test]
  fn type_infer_tables_setindexer_multiple_tables_intersection() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { [string]: number } & { [thread]: boolean }, x)
            local k = "a"
            t[k] = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ [string]: number } & { [thread]: boolean }, never) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3158:type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Fixture (tests/Fixture.h)
  //!   - calls -> method DfgScope::inherit (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
  //!   - translates_to -> rust_item type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types

  #[cfg(test)]
  #[test]
  fn type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let parent_global_scope = fixture.get_frontend().globals.global_scope();

    {
      let mut fix = Fixture::fixture_bool(false);
      fix
        .get_frontend()
        .globals
        .set_global_scope(parent_global_scope.clone());

      let _ = fix.check_string_optional_frontend_options(
        &String::from(
          r#"
--!nonstrict
type MT = typeof(setmetatable)
function wtf(arg: {MT}): typeof(table)
    arg = wtf(arg)
end
"#,
        ),
        None,
      );
    }

    let frontend = fixture.get_frontend();
    let global_scope = frontend.globals.global_scope();
    for binding in global_scope.bindings.values() {
      let _ = to_string_type_id(binding.type_id);
    }
  }
}

mod type_infer_tables_setmetatable_has_a_side_effect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3904:type_infer_tables_setmetatable_has_a_side_effect`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_setmetatable_has_a_side_effect

  #[cfg(test)]
  #[test]
  fn type_infer_tables_setmetatable_has_a_side_effect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mt = {
            __add = function(x, y)
                return 123
            end,
        }

        local foo = {}
        setmetatable(foo, mt)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ @metatable mt, foo }",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_tables_setprop_on_a_mutating_local_in_both_loops_and_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4988:type_infer_tables_setprop_on_a_mutating_local_in_both_loops_and_functions`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_setprop_on_a_mutating_local_in_both_loops_and_functions

  #[cfg(test)]
  #[test]
  fn type_infer_tables_setprop_on_a_mutating_local_in_both_loops_and_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = 5

        while (_) do
            _._ = nil
            function _()
                _ = nil
            end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_shorter_array_types_actually_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2065:type_infer_tables_shorter_array_types_actually_work`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_shorter_array_types_actually_work

  #[cfg(test)]
  #[test]
  fn type_infer_tables_shorter_array_types_actually_work() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local A: {string | number}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{number | string}",
      to_string_type_id(fixture.require_type_string(&String::from("A")))
    );
  }
}

mod type_infer_tables_should_not_unblock_table_type_twice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3008:type_infer_tables_should_not_unblock_table_type_twice`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_tables_should_not_unblock_table_type_twice

  #[cfg(test)]
  #[test]
  fn type_infer_tables_should_not_unblock_table_type_twice() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local timer = peek(timerQueue)
        while timer ~= nil do
            if timer.startTime <= currentTime then
                timer.isQueued = true
            end
            timer = peek(timerQueue)
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_show_not_a_table_error_when_indexing_into_non_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6621:type_infer_tables_show_not_a_table_error_when_indexing_into_non_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_show_not_a_table_error_when_indexing_into_non_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_show_not_a_table_error_when_indexing_into_non_table() {
    use alloc::string::String;

    use ulua_analysis::records::not_a_table::NotATable;
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
        local function f(t: number | boolean)
            t[0] = "huh"
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<NotATable>(&result.errors[0]).expect("expected NotATable");
  }
}

mod type_infer_tables_simple_method_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4265:type_infer_tables_simple_method_definition`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_tables_simple_method_definition

  #[cfg(test)]
  #[test]
  fn type_infer_tables_simple_method_definition() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}

        function T:m()
            return 5
        end

        return T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let mut options = ToStringOptions::new(true);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ m: (unknown) -> number }",
        to_string_type_pack_id_to_string_options(module.return_type, &mut options)
      );
    } else {
      assert_eq!(
        "{ m: <a>(a) -> number }",
        to_string_type_pack_id_to_string_options(module.return_type, &mut options)
      );
    }
  }
}

mod type_infer_tables_stop_refining_new_table_indices_for_non_primitive_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5763:type_infer_tables_stop_refining_new_table_indices_for_non_primitive_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_stop_refining_new_table_indices_for_non_primitive_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_stop_refining_new_table_indices_for_non_primitive_tables() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo:{val:number} = {val = 1}
        if foo.vall then
            local bar = foo.vall
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  }
}

mod type_infer_tables_string_indexer_satisfies_read_only_property {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_string_indexer_satisfies_read_only_property() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(t: { [string]: number }): { read X: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_subproperties_can_also_be_covariantly_tested {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4163:type_infer_tables_subproperties_can_also_be_covariantly_tested`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_subproperties_can_also_be_covariantly_tested

  #[cfg(test)]
  #[test]
  fn type_infer_tables_subproperties_can_also_be_covariantly_tested() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {
            [string]: {[string]: (string | number)?}
        }

        function f(t: T)
            return t
        end

        local x = f({
            subprop={x="hello"}
        })

        local y = f({
            subprop={x=41}
        })

        local z = f({
            subprop={}
        })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_subtyping_with_a_metatable_table_path {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5240:type_infer_tables_subtyping_with_a_metatable_table_path`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_subtyping_with_a_metatable_table_path

  #[cfg(test)]
  #[test]
  fn type_infer_tables_subtyping_with_a_metatable_table_path() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
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
        type self = {} & {}
        type Class = typeof(setmetatable())
        local function _(): Class
            return setmetatable({}::self, {})
        end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      Location::new(Position::new(2, 21), Position::new(2, 43)),
      result.errors[0].location
    );
    assert_eq!(
      "Type function instance setmetatable<unknown, unknown> is uninhabited",
      to_string_type_error(&result.errors[0])
    );

    assert_eq!(
      Location::new(Position::new(2, 28), Position::new(2, 40)),
      result.errors[1].location
    );
    assert_eq!(
      "Argument count mismatch. Function expects 2 arguments, but none are specified",
      to_string_type_error(&result.errors[1])
    );

    assert_eq!(
      Location::new(Position::new(3, 8), Position::new(5, 11)),
      result.errors[2].location
    );
    assert_eq!(
      "Type function instance setmetatable<unknown, unknown> is uninhabited",
      to_string_type_error(&result.errors[2])
    );

    let expected = "Expected this to be 'setmetatable<unknown, unknown>', but got '{ @metatable {  }, {  } & {  } }'; \n\
the 1st entry in the type pack is `{ @metatable {  }, {  } & {  } }` and in the 1st entry in the type packreduces to \
`never`, and `{ @metatable {  }, {  } & {  } }` is not a subtype of `never`";
    assert_eq!(expected, to_string_type_error(&result.errors[3]));
  }
}

mod type_infer_tables_table_access_indexer_fails_with_missing_key {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6703:type_infer_tables_table_access_indexer_fails_with_missing_key`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_table_access_indexer_fails_with_missing_key

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_access_indexer_fails_with_missing_key() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type List = "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.Val1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("_")))
    );
  }
}

mod type_infer_tables_table_access_indexer_via_name_expr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6688:type_infer_tables_table_access_indexer_via_name_expr`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_table_access_indexer_via_name_expr

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_access_indexer_via_name_expr() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.Val1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("_")))
    );
  }
}

mod type_infer_tables_table_call_metamethod_basic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3304:type_infer_tables_table_call_metamethod_basic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_table_call_metamethod_basic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_call_metamethod_basic() {
    use alloc::string::String;

    use ulua_analysis::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = setmetatable({
            a = 1,
        }, {
            __call = function(self, b: number)
                return self.a * b
            end,
        })

        local foo = a(12)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(&result.errors[0])
        .expect("expected ExplicitFunctionAnnotationRecommended");
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }

    let foo_type = fixture.base.require_type_string(&String::from("foo"));
    let number_type = unsafe { (*fixture.base.builtin_types).number_type };
    assert_eq!(number_type, foo_type);
  }
}

mod type_infer_tables_table_call_metamethod_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3357:type_infer_tables_table_call_metamethod_generic`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_call_metamethod_generic

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_call_metamethod_generic() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = setmetatable({}, {
            __call = function<T>(self, b: T)
                return b
            end,
        })

        local foo = a(12)
        local bar = a("bar")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let foo_type = fixture.base.require_type_string(&String::from("foo"));
    let bar_type = fixture.base.require_type_string(&String::from("bar"));
    let number_type = unsafe { (*fixture.base.builtin_types).number_type };
    let string_type = unsafe { (*fixture.base.builtin_types).string_type };

    assert_eq!(number_type, foo_type);
    assert_eq!(string_type, bar_type);
  }
}

mod type_infer_tables_table_call_metamethod_must_be_callable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3331:type_infer_tables_table_call_metamethod_must_be_callable`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_table_call_metamethod_must_be_callable

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_call_metamethod_must_be_callable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::cannot_call_non_function::CannotCallNonFunction,
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
        local a = setmetatable({}, {
            __call = 123,
        })

        local foo = a()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Cannot call a value of type a",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        Location::new(Position::new(5, 20), Position::new(5, 21)),
        result.errors[0].location
      );
      let error = type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
        .expect("expected CannotCallNonFunction");
      assert_eq!("number", to_string_type_id(error.ty()));
    }
  }
}

mod type_infer_tables_table_freeze_musnt_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5446:type_infer_tables_table_freeze_musnt_assert`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_freeze_musnt_assert

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_freeze_musnt_assert() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local m = {}
        function m.foo()
           local self = { entries = entries, _caches = {}}
           local self = setmetatable(self, {})
           table.freeze(self)
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_table_function_check_use_after_free {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3498:type_infer_tables_table_function_check_use_after_free`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_table_function_check_use_after_free

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_function_check_use_after_free() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local t = {}

function t.x(value)
    for k,v in pairs(t) do end
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_indexing_error_location {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3291:type_infer_tables_table_indexing_error_location`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_table_indexing_error_location

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_indexing_error_location() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local foo = {42}
local bar: number?
local baz = foo[bar]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location::new(Position::new(3, 16), Position::new(3, 19)),
      result.errors[0].location
    );
  }
}

mod type_infer_tables_table_inference_one_incorrect_member {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6757:type_infer_tables_table_inference_one_incorrect_member`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_inference_one_incorrect_member

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_inference_one_incorrect_member() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function makeTable(x): { x: number, y: string }
            return { x = x, y = true }
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> { x: number, y: string }",
      to_string_type_id(fixture.require_type_string(&String::from("makeTable")))
    );
  }
}

mod type_infer_tables_table_insert_any_and_true {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_insert_any_and_true() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.insert({} :: any, true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_insert_array_of_any {
  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_insert_array_of_any() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        table.insert({} :: { any }, 42)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_insert_should_cope_with_optional_properties_in_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2351:type_infer_tables_table_insert_should_cope_with_optional_properties_in_nonstrict`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_table_insert_should_cope_with_optional_properties_in_nonstrict

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_insert_should_cope_with_optional_properties_in_nonstrict() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local buttons = {}
        table.insert(buttons, { a = 1 })
        table.insert(buttons, { a = 2, b = true })
        table.insert(buttons, { a = 3 })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_insert_should_cope_with_optional_properties_in_strict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2364:type_infer_tables_table_insert_should_cope_with_optional_properties_in_strict`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_table_insert_should_cope_with_optional_properties_in_strict

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_insert_should_cope_with_optional_properties_in_strict() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local buttons = {}
        table.insert(buttons, { a = 1 })
        table.insert(buttons, { a = 2, b = true })
        table.insert(buttons, { a = 3 })
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5161:type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type cs = { GetTagged : (cs, string) -> any}
local destroyQueue: {any} = {} -- pair of (time, coin)
local tick : () -> any
local CS : cs
local DESTROY_DELAY
local function SpawnCoin()
	local spawns = CS:GetTagged('CoinSpawner')
	local n : any
	local StartPos = spawns[n].CFrame
	local Coin = script.Coin:Clone()
	Coin.CFrame = StartPos
	Coin.Parent = workspace.Coins

	table.insert(destroyQueue, {tick() + DESTROY_DELAY, Coin})
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_length {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2890:type_infer_tables_table_length`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tables_table_length

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_length() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        local s = #t
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.require_type_string(&String::from("t"));
    assert!(
      get_type_id::<TableType>(t).is_some(),
      "expected table type, got {}",
      to_string_type_id(t)
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_tables_table_literal_inference_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5119:type_infer_tables_table_literal_inference_assert`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_literal_inference_assert

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_literal_inference_assert() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local buttons = {
            buttons = {};
        }

        buttons.Button = {
            call = nil;
            lightParts = nil;
            litPropertyOverrides = nil;
            model = nil;
            pivot = nil;
            unlitPropertyOverrides = nil;
        }
        buttons.Button.__index = buttons.Button

        local lightFuncs: { (self: types.Button, lit: boolean) -> nil } = {
            ['\x00'] = function(self: types.Button, lit: boolean)
        end;
        }
    "#,
      ),
      None,
    );
  }
}

mod type_infer_tables_table_param_width_subtyping_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:476:type_infer_tables_table_param_width_subtyping_1`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_param_width_subtyping_1

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_param_width_subtyping_1() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(o)
            local a = o.x
            local b = o.y
            return o
        end

        foo({x=55, y=nil, w=3.14159})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_param_width_subtyping_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:491:type_infer_tables_table_param_width_subtyping_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_table_param_width_subtyping_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_param_width_subtyping_2() {
    use alloc::string::String;

    use ulua_analysis::records::missing_properties::MissingProperties;
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
        function foo(o)
            string.lower(o.bar)
            string.lower(o.baz)
        end

        foo({bar='bar'})
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
      .expect("expected MissingProperties");
    assert_eq!(1, error.properties().len());
    assert_eq!("baz", error.properties()[0].as_str());
  }
}

mod type_infer_tables_table_param_width_subtyping_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:512:type_infer_tables_table_param_width_subtyping_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_table_param_width_subtyping_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_param_width_subtyping_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::missing_properties::MissingProperties,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _missing_properties_as_nil =
      ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.bar = 'hello'
        function T:method()
            local a = self.baz
        end
        T:method()
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() && FFlag::LuauSubtypingMissingPropertiesAsNil.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location {
          begin: Position { line: 6, column: 8 },
          end: Position { line: 6, column: 9 },
        },
        result.errors[0].location
      );

      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Expected this to be '{ read baz: unknown }', but got 'T'",
          to_string_type_error(&result.errors[0])
        );
      } else {
        let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
          .expect("expected MissingProperties");
        assert_eq!(1, error.properties().len());
        assert_eq!("baz", error.properties()[0].as_str());
      }
    }
  }
}

mod type_infer_tables_table_read_any_counts_as_read_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7062:type_infer_tables_table_read_any_counts_as_read_nil`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_read_any_counts_as_read_nil

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_read_any_counts_as_read_nil() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _missing_properties_as_nil =
      ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);
    let _better_error_suppression =
      ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {}): { read foo: any }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_simple_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3375:type_infer_tables_table_simple_call`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_table_simple_call

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_simple_call() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = setmetatable({ x = 2 }, {
            __call = function(self)
                return (self.x :: number) * 2 -- should work without annotation in the future
            end
        })
        local b = a()
        local c = a(2) -- too many arguments
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'a' expects 1 argument, but 2 are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_table_subtyping_error_suppression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4395:type_infer_tables_table_subtyping_error_suppression`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_error_suppression

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_error_suppression() {
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
        function one(tbl: {x: any}) end
        function two(tbl: {x: string}) one(tbl) end -- ok, string <: any and any <: string

        function three(tbl: {x: any, y: string}) end
        function four(tbl: {x: string, y: string}) three(tbl) end -- ok, string <: any, any <: string, string <: string
        function five(tbl: {x: string, y: number}) three(tbl) end -- error, string <: any, any <: string, but number </: string
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ x: any, y: string }", to_string_type_id(tm.wanted_type));
    assert_eq!("{ x: string, y: number }", to_string_type_id(tm.given_type));
  }
}

mod type_infer_tables_table_subtyping_shouldn_t_add_optional_properties_to_sealed_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2671:type_infer_tables_table_subtyping_shouldn_t_add_optional_properties_to_sealed_tables`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Constant::getString (Compiler/src/ConstantFolding.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_shouldn_t_add_optional_properties_to_sealed_tables

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_shouldn_t_add_optional_properties_to_sealed_tables() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function setNumber(t: { p: number? }, x:number) t.p = x end
        local function getString(t: { p: string? }):string return t.p or "" end
        -- This shouldn't type-check!
        local function oh(x:number): string
          local t: {} = {}
          setNumber(t, x)
          return getString(t)
        end
        local s: string = oh(37)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_subtyping_with_extra_props_dont_report_multiple_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1849:type_infer_tables_table_subtyping_with_extra_props_dont_report_multiple_errors`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_with_extra_props_dont_report_multiple_errors

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_with_extra_props_dont_report_multiple_errors() {
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
        function mkvec3() return {x = 1, y = 2, z = 3} end
        function mkvec1() return {x = 1} end

        local vec3: {{x: number, y: number, z: number}} = {mkvec3()}
        local vec1: {{x: number}} = {mkvec1()}

        vec1 = vec3
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!("{{ x: number }}", to_string_type_id(tm.wanted_type));
    assert_eq!(
      "{{ x: number, y: number, z: number }}",
      to_string_type_id(tm.given_type)
    );
  }
}

mod type_infer_tables_table_subtyping_with_extra_props_is_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1870:type_infer_tables_table_subtyping_with_extra_props_is_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_with_extra_props_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_with_extra_props_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local vec3 = {x = 1, y = 2, z = 3}
        local vec1 = {x = 1}

        vec1 = vec3
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1792:type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::missing_properties::{Context, MissingProperties},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(vec1: {x: number}): {x: number, y: number, z: number}
            return vec1
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected =
        "Expected this to be\n\t'{ x: number, y: number, z: number }'\nbut got\n\t'{ x: number }'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let mp = type_error_data_ref::<MissingProperties>(&result.errors[0])
        .unwrap_or_else(|| panic!("Expected MissingProperties but got {:?}", result.errors[0]));
      assert_eq!(Context::Missing, mp.context());
      assert_eq!(2, mp.properties().len());
      assert_eq!("y", mp.properties()[0].as_str());
      assert_eq!("z", mp.properties()[1].as_str());

      assert_eq!(
        "{ x: number, y: number, z: number }",
        to_string_type_id(mp.super_type())
      );
      assert_eq!("{ x: number }", to_string_type_id(mp.sub_type()));
    }
  }
}

mod type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1824:type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        missing_properties::{Context, MissingProperties},
        type_mismatch::TypeMismatch,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MixedTable = {[number]: number, x: number}
        local t: MixedTable = {"fail"}
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

      assert_eq!("number", to_string_type_id(tm.wanted_type));
      assert_eq!("string", to_string_type_id(tm.given_type));
    }

    let mp = type_error_data_ref::<MissingProperties>(&result.errors[1])
      .expect("expected MissingProperties");
    assert_eq!(Context::Missing, mp.context());
    assert_eq!(1, mp.properties().len());
    assert_eq!("x", mp.properties()[0].as_str());
  }
}

mod type_infer_tables_table_unification_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:560:type_infer_tables_table_unification_4`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_unification_4

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_unification_4() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(o)
            if o.prop then
                return o
            else
                return {prop=false}
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_unifies_into_map {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2962:type_infer_tables_table_unifies_into_map`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_tables_table_unifies_into_map

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_unifies_into_map() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Instance: any
        local UDim2: any

        function Create(instanceType)
            return function(data)
                local obj = Instance.new(instanceType)
                for k, v in pairs(data) do
                    if type(k) == 'number' then
                        --v.Parent = obj
                    else
                        obj[k] = v
                    end
                end
                return obj
            end
        end

        local topbarShadow = Create'ImageLabel'{
            Name = "TopBarShadow";
            Size = UDim2.new(1, 0, 0, 3);
            Position = UDim2.new(0, 0, 1, 0);
            Image = "rbxasset://textures/ui/TopBar/dropshadow.png";
            BackgroundTransparency = 1;
            Active = false;
            Visible = false;
        };

    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_table_with_intersection_containing_lambda {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6551:type_infer_tables_table_with_intersection_containing_lambda`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_with_intersection_containing_lambda

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_with_intersection_containing_lambda() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Arg = { foo: string }

        type TypeA = { foo: string, method: (arg: Arg) -> any }

        type TypeB = TypeA & {}

        local bar: TypeB = {
            foo = "wow!",
            method = function(arg)
                local _ = arg
                return nil
            end,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{ foo: string }",
      to_string_type_id_to_string_options(
        fixture.require_type_at_position_position(Position::new(10, 28)),
        &mut options
      )
    );
  }
}

mod type_infer_tables_table_writes_introduce_write_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4805:type_infer_tables_table_writes_introduce_write_properties`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_table_writes_introduce_write_properties

  #[cfg(test)]
  #[test]
  fn type_infer_tables_table_writes_introduce_write_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function oc(player, speaker)
            local head = speaker.Character:FindFirstChild('Head')
            speaker.Character = player[1].Character
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a>({{ read Character: t1 }}, { Character: t1 }) -> () where t1 = { read FindFirstChild: (t1, string) -> (a, ...unknown) }",
      to_string_type_id(fixture.require_type_string(&String::from("oc")))
    );
  }
}

mod type_infer_tables_tables_can_have_both_metatables_and_indexers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4826:type_infer_tables_tables_can_have_both_metatables_and_indexers`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_tables_tables_can_have_both_metatables_and_indexers

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tables_can_have_both_metatables_and_indexers() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        a[1] = 5
        a[2] = 17

        local t = {}
        setmetatable(a, t)

        local c = a[1]
        print(a[1])
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_tables_tables_get_names_from_their_locals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2997:type_infer_tables_tables_get_names_from_their_locals`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_tables_get_names_from_their_locals

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tables_get_names_from_their_locals() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T",
      to_string_type_id(fixture.require_type_string(&String::from("T")))
    );
  }
}

mod type_infer_tables_tables_routing_bidirectional_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7079:type_infer_tables_tables_routing_bidirectional_inference`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - translates_to -> rust_item type_infer_tables_tables_routing_bidirectional_inference

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tables_routing_bidirectional_inference() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type ReceivedRequest = {
            method: string,
            path: string,
            body: string,
            query: { [string]: string },
            headers: { [string]: string },
            params: { [string]: string },
        }

        export type ServerResponse = string | {
            status: number?,
            body: string?,
            headers: { [string]: string }?,
        }

        export type RouteHandler = Handler | ServerResponse

        export type MethodRoutes = {
            GET: RouteHandler?,
            POST: RouteHandler?,
            PUT: RouteHandler?,
            DELETE: RouteHandler?,
            PATCH: RouteHandler?,
            HEAD: RouteHandler?,
            OPTIONS: RouteHandler?,
        }

        export type RouteEntry = RouteHandler | MethodRoutes

        export type Routes = { [string]: RouteEntry }

        export type Server = {
            hostname: string,
            port: number,
            close: () -> (),
            upgrade: (self: Server, req: ReceivedRequest) -> boolean,
        }

        export type Handler = (request: ReceivedRequest, server: Server) -> ServerResponse?

        local routes: Routes? = {
            ["/health"] = "ok",
            ["/json"] = {
                status = 200,
                headers = { ["Content-Type"] = "application/json" },
                body = '{"ok":true}',
            },
            ["/hello"] = function(req)
                local _ = req
                return { status = 200, body = "hello" }
            end,
        }

    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "ReceivedRequest",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(49, 28)))
    );
  }
}

mod type_infer_tables_tables_should_be_fully_populated {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3924:type_infer_tables_tables_should_be_fully_populated`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_tables_should_be_fully_populated

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tables_should_be_fully_populated() {
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
        local t = {
            x = 5 :: NonexistingTypeWhichEndsUpReturningAnErrorType,
            y = 5
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ x: *error-type*, y: number }",
        to_string_type_id_to_string_options(
          fixture.base.require_type_string(&String::from("t")),
          &mut options,
        )
      );
    } else {
      assert_eq!(
        "{| x: *error-type*, y: number |}",
        to_string_type_id_to_string_options(
          fixture.base.require_type_string(&String::from("t")),
          &mut options,
        )
      );
    }
  }
}

mod type_infer_tables_tc_member_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:262:type_infer_tables_tc_member_function`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_tc_member_function

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tc_member_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local T = {}  function T:foo() return 5 end"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let table_type = get_type_id::<TableType>(fixture.require_type_string(&String::from("T")))
      .expect("expected table type");

    let foo_ty = table_type
      .props
      .get("foo")
      .and_then(|prop| prop.read_ty)
      .expect("expected foo read type");
    let method_type = get_type_id::<FunctionType>(follow_type_id(foo_ty));
    assert!(method_type.is_some(), "expected function type");
  }
}

mod type_infer_tables_tc_member_function_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:278:type_infer_tables_tc_member_function_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_tables_tc_member_function_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_tc_member_function_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local T = {U={}}  function T.U:foo() return 5 end"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let table_type = get_type_id::<TableType>(fixture.require_type_string(&String::from("T")))
      .expect("expected table type");

    let u_type = table_type
      .props
      .get("U")
      .and_then(|prop| prop.read_ty)
      .expect("expected U read type");
    let u_table = get_type_id::<TableType>(u_type).expect("expected U table");

    let foo_ty = u_table
      .props
      .get("foo")
      .and_then(|prop| prop.read_ty)
      .expect("expected foo read type");
    let method_type =
      get_type_id::<FunctionType>(follow_type_id(foo_ty)).expect("expected function type");

    let (method_args, _) = flatten_type_pack_id(method_type.arg_types());
    assert_eq!(1, method_args.len());
  }
}

mod type_infer_tables_test_indexing_into_unsealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7338:type_infer_tables_test_indexing_into_unsealed_table`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_test_indexing_into_unsealed_table

  #[cfg(test)]
  #[test]
  fn type_infer_tables_test_indexing_into_unsealed_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local key1: string, key2: number
        local tbl = {}
        tbl[key1] = 42
        local val = tbl[key2]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err.wanted_type));
    assert_eq!("number", to_string_type_id(err.given_type));

    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{ [string]: number }",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("tbl")),
        &mut options
      )
    );
  }
}

mod type_infer_tables_top_table_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2689:type_infer_tables_top_table_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_top_table_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_top_table_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Table = { [any] : any }
        type HasTable = { p: Table? }
        type HasHasTable = { p: HasTable? }
        local t : Table = { p = 5 }
        local u : HasTable = { p = { p = 5 } }
        local v : HasHasTable = { p = { p = { p = 5 } } }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4104:type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r##"
        local None = newproxy(true)
        local mt = getmetatable(None)
        mt.__tostring = function()
            return "Object.None"
        end

        function assign(...)
            for index = 1, select("#", ...) do
                local rest = select(index, ...)

                if rest ~= nil and typeof(rest) == "table" then
                    for key, value in pairs(rest) do
                    end
                end
            end
        end
    "##,
      ),
      None,
    );
  }
}

mod type_infer_tables_type_mismatch_in_dict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5793:type_infer_tables_type_mismatch_in_dict`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_type_mismatch_in_dict

  #[cfg(test)]
  #[test]
  fn type_infer_tables_type_mismatch_in_dict() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local dict: {[string]: boolean} = {
            code1 = true,
            code2 = 123,
        }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("boolean", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_tables_type_mismatch_on_massive_table_is_cut_short {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1882:type_infer_tables_type_mismatch_on_massive_table_is_cut_short`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_type_mismatch_on_massive_table_is_cut_short

  #[cfg(test)]
  #[test]
  fn type_infer_tables_type_mismatch_on_massive_table_is_cut_short() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FInt;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let _sfis = ScopedFastInt::new(&FInt::LuauTableTypeMaximumStringifierLength, 40);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {a: number,b: number, c: number, d: number, e: number, f: number} = nil :: any
        t = 1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!(
      "{ a: number, b: number, c: number, d: number, e: number, ... 1 more ... }",
      to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
    assert_eq!("number", to_string_type_id(tm.given_type));
    assert_eq!(
      "Expected this to be '{ a: number, b: number, c: number, d: number, e: number, ... 1 more ... }', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_unification_of_unions_in_a_self_referential_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1200:type_infer_tables_unification_of_unions_in_a_self_referential_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_unification_of_unions_in_a_self_referential_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_unification_of_unions_in_a_self_referential_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::metatable_type::MetatableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {}
        type AMT = { __mul: (A, A | number) -> A }
        local a: A
        local amt: AMT
        setmetatable(a, amt)

        type B = {}
        type BMT = { __mul: (B, A | B | number) -> A }
        local b: B
        local bmt: BMT
        setmetatable(b, bmt)

        a = b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let amtv = get_type_id::<MetatableType>(fixture.base.require_type_string(&String::from("a")))
      .expect("expected MetatableType for a");
    assert_eq!(
      follow_type_id(amtv.metatable()),
      follow_type_id(fixture.base.require_type_string(&String::from("amt")))
    );

    let bmtv = get_type_id::<MetatableType>(fixture.base.require_type_string(&String::from("b")))
      .expect("expected MetatableType for b");
    assert_eq!(
      follow_type_id(bmtv.metatable()),
      follow_type_id(fixture.base.require_type_string(&String::from("bmt")))
    );
  }
}

mod type_infer_tables_unifying_tables_shouldnt_uaf_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2836:type_infer_tables_unifying_tables_shouldnt_uaf_1`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - translates_to -> rust_item type_infer_tables_unifying_tables_shouldnt_uaf_1

  #[cfg(test)]
  #[test]
  fn type_infer_tables_unifying_tables_shouldnt_uaf_1() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
-- This example produced a UAF at one point, caused by pointers to table types becoming
-- invalidated by child unifiers. (Calling log.concat can cause pointers to become invalid.)
type _Entry = {
    a: number,

    middle: (self: _Entry) -> (),

    z: number
}

export type AnyEntry = _Entry

local Entry = {}
Entry.__index = Entry

function Entry:dispose()
    self:middle()
    forgetChildren(self) -- unify free with sealed AnyEntry
end

function forgetChildren(parent: AnyEntry)
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_unifying_tables_shouldnt_uaf_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2866:type_infer_tables_unifying_tables_shouldnt_uaf_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_unifying_tables_shouldnt_uaf_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_unifying_tables_shouldnt_uaf_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
-- Another example that UAFd, this time found by fuzzing.
local _
do
_._ *= (_[{n0=_[{[{[_]=_,}]=_,}],}])[_]
_ = (_.n0)
end
_._ *= (_[false])[_]
_ = (_.cos)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_union_of_indexers_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7290:type_infer_tables_union_of_indexers_1`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_union_of_indexers_1

  #[cfg(test)]
  #[test]
  fn type_infer_tables_union_of_indexers_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { [string | number]: never } | { [string | number]: string }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_union_of_indexers_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7306:type_infer_tables_union_of_indexers_2`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_union_of_indexers_2

  #[cfg(test)]
  #[test]
  fn type_infer_tables_union_of_indexers_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { [string | number]: unknown } | { [string | number]: string }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_union_of_indexers_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7322:type_infer_tables_union_of_indexers_3`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_union_of_indexers_3

  #[cfg(test)]
  #[test]
  fn type_infer_tables_union_of_indexers_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl: { [string | number]: boolean | string } | { [string | number]: boolean | number }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean | number | string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_tables_unsafe_bidirectional_mutation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5724:type_infer_tables_unsafe_bidirectional_mutation`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_unsafe_bidirectional_mutation

  #[cfg(test)]
  #[test]
  fn type_infer_tables_unsafe_bidirectional_mutation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type F = {
            _G: () -> ()
        }
        function _()
            return
        end
        local function h(f: F) end
        h({
            _G = {},
            _G = _,
        })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_updating_sealed_table_prop_is_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:207:type_infer_tables_updating_sealed_table_prop_is_ok`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_tables_updating_sealed_table_prop_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_tables_updating_sealed_table_prop_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local t = {prop=999}    t.prop = 0"),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_used_colon_correctly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:366:type_infer_tables_used_colon_correctly`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_used_colon_correctly

  #[cfg(test)]
  #[test]
  fn type_infer_tables_used_colon_correctly() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local upVector = {}
        function upVector:Dot(lookVector)
            return 8
        end
        local v = math.abs(upVector:Dot(5))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_used_colon_instead_of_dot {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:394:type_infer_tables_used_colon_instead_of_dot`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FunctionDoesNotTakeSelf (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_used_colon_instead_of_dot

  #[cfg(test)]
  #[test]
  fn type_infer_tables_used_colon_instead_of_dot() {
    use alloc::string::String;

    use ulua_analysis::records::function_does_not_take_self::FunctionDoesNotTakeSelf;
    use ulua_unit_test::{functions::has_error::has_error, records::fixture::Fixture};

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.x = 0
        function T.method()
            return 5
        end
        local a = T:method()
    "#,
      ),
      None,
    );

    assert!(
      has_error::<FunctionDoesNotTakeSelf>(&result),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_tables_used_dot_instead_of_colon {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:341:type_infer_tables_used_dot_instead_of_colon`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FunctionRequiresSelf (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_used_dot_instead_of_colon

  #[cfg(test)]
  #[test]
  fn type_infer_tables_used_dot_instead_of_colon() {
    use alloc::string::String;

    use ulua_analysis::records::function_requires_self::FunctionRequiresSelf;
    use ulua_unit_test::{functions::has_error::has_error, records::fixture::Fixture};

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.x = 0
        function T:method()
            return self.x
        end
        local a = T.method()
    "#,
      ),
      None,
    );

    assert!(
      has_error::<FunctionRequiresSelf>(&result),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_tables_used_dot_instead_of_colon_but_correctly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:380:type_infer_tables_used_dot_instead_of_colon_but_correctly`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_used_dot_instead_of_colon_but_correctly

  #[cfg(test)]
  #[test]
  fn type_infer_tables_used_dot_instead_of_colon_but_correctly() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.x = 0
        function T:method(arg1, arg2)
            return self.x
        end
        local a = T.method(T, 6, 7)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_user_defined_table_types_are_named {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1267:type_infer_tables_user_defined_table_types_are_named`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_user_defined_table_types_are_named

  #[cfg(test)]
  #[test]
  fn type_infer_tables_user_defined_table_types_are_named() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Vector3 = {x: number, y: number}

        local v: Vector3 = {x = 5, y = 7}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Vector3",
      to_string_type_id(fixture.require_type_string(&String::from("v")))
    );
  }
}

mod type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4024:type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type

  #[cfg(test)]
  #[test]
  fn type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{optional_value_access::OptionalValueAccess, table_type::TableType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local events = {}
        local mockObserveEvent = function(_, key, callback)
            events[key] = callback
        end

        events['FriendshipNotifications']({
            EventArgs = {
                UserId2 = '2'
            },
            Type = 'FriendshipDeclined'
        })
    "#,
      ),
      None,
    );

    let ty = follow_type_id(fixture.require_type_string(&String::from("events")));
    let tt = get_type_id::<TableType>(ty)
      .unwrap_or_else(|| panic!("expected table but got {}", to_string_type_id(ty)));

    assert!(tt.props.is_empty());
    let indexer = tt.indexer.expect("expected indexer");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!("unknown", to_string_type_id(indexer.index_type));
      type_error_data_ref::<OptionalValueAccess>(&result.errors[0])
        .expect("expected OptionalValueAccess");
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!("string", to_string_type_id(indexer.index_type));
    }
  }
}

mod type_infer_tables_width_subtyping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:625:type_infer_tables_width_subtyping`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_width_subtyping

  #[cfg(test)]
  #[test]
  fn type_infer_tables_width_subtyping() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function f(x : { q : number })
           x.q = 8
        end
        local t : { q : number, r : string } = { q = 8, r = "hi" }
        f(t)
        local x : string = t.r
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_width_subtyping_needs_covariance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:640:type_infer_tables_width_subtyping_needs_covariance`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tables_width_subtyping_needs_covariance

  #[cfg(test)]
  #[test]
  fn type_infer_tables_width_subtyping_needs_covariance() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function f(x : { p : { q : number }})
           x.p = { q = 8, r = 5 }
        end
        local t : { p : { q : number, r : string } } = { p = { q = 8, r = "hi" } }
        f(t) -- Shouldn't typecheck
        local x : string = t.p.r -- x is 5
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tables_write_annotations_are_supported_with_the_new_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4569:type_infer_tables_write_annotations_are_supported_with_the_new_solver`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_write_annotations_are_supported_with_the_new_solver

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_annotations_are_supported_with_the_new_solver() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {write foo: number})
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_write_only_property_subtype_mismatch_error_message {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4547:type_infer_tables_write_only_property_subtype_mismatch_error_message`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_tables_write_only_property_subtype_mismatch_error_message

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_only_property_subtype_mismatch_error_message() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _property_modifier_mismatch_errors =
      ScopedFastFlag::new(&FFlag::LuauPropertyModifierMismatchErrors, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { write woof: number }): { woof: number }
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Expected this to be\n\t'{ woof: number }'\nbut got\n\t'{ write woof: number }'; \n`woof` is a write-only property in the latter type, but the former type requires a read-write property",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_write_only_table_field_duplicate {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5431:type_infer_tables_write_only_table_field_duplicate`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_write_only_table_field_duplicate

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_only_table_field_duplicate() {
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
        type WriteOnlyTable = { write x: number }
        local wo: WriteOnlyTable = {
            x = 42,
            x = 13,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_write_to_read_only_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4418:type_infer_tables_write_to_read_only_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_write_to_read_only_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_to_read_only_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{
        property_access_violation::{PropertyAccessViolation, PropertyAccessViolation_Context},
        to_string_options::ToStringOptions,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {read x: number})
            t.x = 5
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property x of table '{ read x: number }' is read-only",
      to_string_type_error(&result.errors[0])
    );

    let pav = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
      .expect("expected PropertyAccessViolation");
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{ read x: number }",
      to_string_type_id_to_string_options(pav.table(), &mut options)
    );
    assert_eq!("x", pav.key());
    assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
  }
}

mod type_infer_tables_write_to_union_property_not_all_present {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4899:type_infer_tables_write_to_union_property_not_all_present`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - type_ref -> record CannotAssignToNever (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum Reason (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tables_write_to_union_property_not_all_present

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_to_union_property_not_all_present() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::reason::Reason, functions::to_string_to_string_alt_c::to_string_type_id,
      records::cannot_assign_to_never::CannotAssignToNever,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Animal = {tag: "Cat", meow: boolean} | {tag: "Dog", woof: boolean}
        function f(t: Animal)
            t.tag = "Dog"
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let tm = type_error_data_ref::<CannotAssignToNever>(&result.errors[0])
      .expect("expected CannotAssignToNever");
    assert_eq!(fixture.get_builtins().string_type, tm.rhs_type());
    assert_eq!(Reason::PropertyNarrowed, tm.reason());
    assert_eq!(2, tm.cause().len());
    assert_eq!("\"Cat\"", to_string_type_id(tm.cause()[0]));
    assert_eq!("\"Dog\"", to_string_type_id(tm.cause()[1]));
  }
}

mod type_infer_tables_write_to_unusually_named_read_only_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4490:type_infer_tables_write_to_unusually_named_read_only_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tables_write_to_unusually_named_read_only_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_to_unusually_named_read_only_property() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {read ["hello world"]: number})
            t["hello world"] = 5
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property \"hello world\" of table '{ read [\"hello world\"]: number }' is read-only",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_tables_write_to_write_only_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4440:type_infer_tables_write_to_write_only_property`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_tables_write_to_write_only_property

  #[cfg(test)]
  #[test]
  fn type_infer_tables_write_to_write_only_property() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {write x: number})
            t.x = 5
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tables_wrong_assign_does_hit_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2908:type_infer_tables_wrong_assign_does_hit_indexer`
  //! Source: `tests/TypeInfer.tables.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tables.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tables.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_tables_wrong_assign_does_hit_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_tables_wrong_assign_does_hit_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
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
        local a = {}
        a[0] = 7
        a[0] = 't'
        a[0] = nil
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 3,
          column: 15
        },
        end: Position {
          line: 3,
          column: 18
        },
      },
      result.errors[0].location
    );
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number?", to_string_type_id(tm.wanted_type));
    assert_eq!(fixture.get_builtins().string_type, tm.given_type);
  }
}
