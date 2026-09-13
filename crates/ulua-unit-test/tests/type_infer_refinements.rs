extern crate alloc;

mod type_infer_refinements_a_and_b_or_a_and_c {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:276:type_infer_refinements_a_and_b_or_a_and_c`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_a_and_b_or_a_and_c

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_a_and_b_or_a_and_c() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: string?, b: number?, c: boolean)
            if (a and b) or (a and c) then
                local foo = a
                local bar = b
                local baz = c
            else
                local foo = a
                local bar = b
                local baz = c
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "boolean",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    } else {
      assert_eq!(
        "true",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    }
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 28)))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
    );
  }
}

mod type_infer_refinements_and_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:207:type_infer_refinements_and_constraint`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_and_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_and_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: string?, b: number?)
            if a and b then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
    );
  }
}

mod type_infer_refinements_and_or_peephole_refinement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1362:type_infer_refinements_and_or_peephole_refinement`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_and_or_peephole_refinement

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_and_or_peephole_refinement() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function len(a: {any})
            return a and #a or nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_apply_refinements_on_astexprindexexpr_whose_subscript_expr_is_constant_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1276:type_infer_refinements_apply_refinements_on_astexprindexexpr_whose_subscript_expr_is_constant_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_apply_refinements_on_astexprindexexpr_whose_subscript_expr_is_constant_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_apply_refinements_on_astexprindexexpr_whose_subscript_expr_is_constant_string()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { [string]: { prop: number }? }
        local t: T = {}

        if t["hello"] then
            local foo = t["hello"].prop
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_assert_a_to_be_truthy_then_assert_a_to_be_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1065:type_infer_refinements_assert_a_to_be_truthy_then_assert_a_to_be_number`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_assert_a_to_be_truthy_then_assert_a_to_be_number

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_assert_a_to_be_truthy_then_assert_a_to_be_number() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: (number | string)?
        assert(a)
        local b = a
        assert(type(a) == "number")
        local c = a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number | string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 18))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 18))
      )
    );
  }
}

mod type_infer_refinements_assert_and_typeof_refinement_context {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3033:type_infer_refinements_assert_and_typeof_refinement_context`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_refinements_assert_and_typeof_refinement_context

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_assert_and_typeof_refinement_context() {
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

        local x = {} :: unknown

        if typeof(x) == "table" then
            assert(typeof(x.transform) == "function")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_assert_call_should_not_refine_despite_typeof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3067:type_infer_refinements_assert_call_should_not_refine_despite_typeof`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_assert_call_should_not_refine_despite_typeof

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_assert_call_should_not_refine_despite_typeof() {
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
        --!strict
        local function foo(_: any)
            return true
        end

        local function f(x: unknown)
            if typeof(x) == "table" then
                assert(foo(typeof(x.bar)))
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'table' does not have key 'bar'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_refinements_assert_non_binary_expressions_actually_resolve_constraints {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:587:type_infer_refinements_assert_non_binary_expressions_actually_resolve_constraints`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_assert_non_binary_expressions_actually_resolve_constraints

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_assert_non_binary_expressions_actually_resolve_constraints() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: string? = "hello"
        assert(foo)
        local bar: string = foo
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1687:type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never()
   {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local weld: WeldConstraint = nil :: any
        assert(weld.Part8)
        print(weld)
        assert(weld.Part8.Name == "RootPart")
        local part8 = assert(weld.Part8)
        local pos = part8.Position
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      to_string_type_error(&result.errors[0]),
      "Key 'Part8' not found in external type 'WeldConstraint'"
    );

    assert_eq!(
      "WeldConstraint",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 15))
      )
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "any",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(6, 29))
        )
      );
    } else {
      assert_eq!(
        "*error-type*",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(6, 29))
        )
      );
    }
  }
}

mod type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1666:type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never()
  {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local weld: WeldConstraint = nil :: any
        assert(weld.Part1)
        print(weld) -- hover type incorrectly becomes `never`
        assert(weld.Part1.Name == "RootPart")
        local part1 = assert(weld.Part1)
        local pos = part1.Position
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() && FFlag::LuauExternTypesNormalizeWithShapes.get() {
      assert_eq!(
        "WeldConstraint & { read Part1: ~(false?) }",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 15))
        )
      );
    } else {
      assert_eq!(
        "WeldConstraint",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 15))
        )
      );
    }
    assert_eq!(
      "Vector3",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(6, 29))
      )
    );
  }
}

mod type_infer_refinements_call_an_incompatible_function_after_using_typeguard {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:494:type_infer_refinements_call_an_incompatible_function_after_using_typeguard`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_refinements_call_an_incompatible_function_after_using_typeguard

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_call_an_incompatible_function_after_using_typeguard() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number)
            return x
        end

        local function g(x: unknown)
            if type(x) == "string" then
                f(x)
            end
        end

        local function h(x: any)
            if type(x) == "string" then
                f(x)
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        Location::new(Position::new(7, 18), Position::new(7, 19)),
        result.errors[0].location
      );
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        Location::new(Position::new(7, 18), Position::new(7, 19)),
        result.errors[0].location
      );

      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[1])
      );
      assert_eq!(
        Location::new(Position::new(13, 18), Position::new(13, 19)),
        result.errors[1].location
      );
    }
  }
}

mod type_infer_refinements_call_to_undefined_method_is_not_a_refinement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:467:type_infer_refinements_call_to_undefined_method_is_not_a_refinement`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_refinements_call_to_undefined_method_is_not_a_refinement

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_call_to_undefined_method_is_not_a_refinement() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                if x.foo() then
                end
            end
            return (nil :: never)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let unknown_prop = type_error_data_ref::<UnknownProperty>(&result.errors[0])
      .unwrap_or_else(|| panic!("expected UnknownProperty, got {:?}", result.errors[0]));
    assert_eq!("foo", unknown_prop.key());
    assert_eq!("table", to_string_type_id(unknown_prop.table()));

    assert_eq!(
      Location::new(Position::new(3, 19), Position::new(3, 24)),
      result.errors[0].location
    );
  }
}

mod type_infer_refinements_cannot_call_a_function_single {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2684:type_infer_refinements_cannot_call_a_function_single`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_cannot_call_a_function_single

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cannot_call_a_function_single() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{
        builtins_fixture::BuiltinsFixture,
        refinement_extern_type_fixture::RefinementExternTypeFixture,
      },
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function invokeDisconnect(d: unknown)
            if type(d) == "function" then
                d()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "The type function is not precise enough for us to determine the appropriate result type of this call.",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_refinements_cannot_call_a_function_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2717:type_infer_refinements_cannot_call_a_function_union`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cannot_call_a_function_union

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cannot_call_a_function_union() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{
        builtins_fixture::BuiltinsFixture,
        refinement_extern_type_fixture::RefinementExternTypeFixture,
      },
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Disconnectable = {
            Disconnect: (self: Disconnectable) -> (...any);
        } | {
            disconnect: (self: Disconnectable) -> (...any)
        } | ExternScriptConnection

        local x: Disconnectable = workspace.ChildAdded:Connect(function()
            print("child added")
        end)

        if type(x.Disconnect) == "function" then
            x:Disconnect()
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    // FIXME CLI-157125: It's a bit clowny that we return a union of
    // functions containing `function` here, but it looks like a side
    // effect of how we execute `hasProp`.
    let expected_error = String::from("Cannot call a value of type function in union:\n")
      + "  ((ExternScriptConnection) -> ()) | function | t2 where t1 = ExternScriptConnection | { Disconnect: t2 } | { "
      + "disconnect: (t1) -> (...any) } ; t2 = (t1) -> (...any)";

    assert_eq!(to_string_type_error(&result.errors[1]), expected_error);
  }
}

mod type_infer_refinements_cat_or_dog_through_a_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2033:type_infer_refinements_cat_or_dog_through_a_local`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cat_or_dog_through_a_local

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cat_or_dog_through_a_local() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Cat = { tag: "cat", catfood: string }
        type Dog = { tag: "dog", dogfood: string }
        type Animal = Cat | Dog

        local function f(animal: Animal)
            local tag = animal.tag
            if tag == "dog" then
                local dog = animal
            elseif tag == "cat" then
                local cat = animal
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Cat | Dog",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 28)))
    );
    assert_eq!(
      "Cat | Dog",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(10, 28)))
    );
  }
}

mod type_infer_refinements_check_refinement_to_primitive_and_compare {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2250:type_infer_refinements_check_refinement_to_primitive_and_compare`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_check_refinement_to_primitive_and_compare

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_check_refinement_to_primitive_and_compare() {
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
        local function comesAfterLuau(word)
            return type(word) == "string" and word > "luau"
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(unknown) -> boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("comesAfterLuau"))
      )
    );
  }
}

mod type_infer_refinements_cli_120460_table_access_on_phi_node {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2869:type_infer_refinements_cli_120460_table_access_on_phi_node`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cli_120460_table_access_on_phi_node

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cli_120460_table_access_on_phi_node() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function foo(bar: string): string
            local baz: boolean = true
            if baz then
                local _ = (bar:sub(1))
            else
                local _ = (bar:sub(1))
            end
            return bar:sub(2) -- previously this would be `...never`
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_cli_140033_refine_union_of_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2700:type_infer_refinements_cli_140033_refine_union_of_extern_types`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_refinements_cli_140033_refine_union_of_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cli_140033_refine_union_of_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function getImageLabel(vars: { Instance }): Folder | Part | nil
            for _, item in vars do
                if item:IsA("Folder") or item:IsA("Part") then
                    return item
                end
            end
            return nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder | Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_cli_181100_fast_track_refinement_against_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3153:type_infer_refinements_cli_181100_fast_track_refinement_against_unknown`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cli_181100_fast_track_refinement_against_unknown

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cli_181100_fast_track_refinement_against_unknown() {
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
        --!strict

        local Class = {}
        Class.__index = Class

        type Class = setmetatable<{ A: number }, typeof(Class)>

        function Class.Foo(x: Class, y: Class, z: Class)
            if y == z then
                return
            end
            local bar = y.A
            print(bar)
        end
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
          .require_type_at_position_position(Position::new(13, 19))
      )
    );
  }
}

mod type_infer_refinements_cli_181549_refined_string_should_be_subtype_of_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3180:type_infer_refinements_cli_181549_refined_string_should_be_subtype_of_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cli_181549_refined_string_should_be_subtype_of_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cli_181549_refined_string_should_be_subtype_of_string() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
      local hello : string = "world"

      if hello == "" then
          return
      end

      string.find(hello, "bye")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3195:type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum State (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type States = "Closed" | "Closing" | "Opening" | "Open"
        export type MyType<A = any> = {
            State: States,
            IsOpen: boolean,
            Open: (self: MyType<A>) -> (),
        }

        local value = {} :: MyType

        function value:Open()
            if self.IsOpen == true then
            elseif self.State == "Closing" or self.State == "Opening" then
                -- Prior, this line errored as we were erroneously refining
                -- `self` with `{ State: "Closing" | "Opening" }` rather
                -- than `{ read State: "Closing" | "Opening" }
                self:Open()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_conditional_refinement_should_stay_error_suppressing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2200:type_infer_refinements_conditional_refinement_should_stay_error_suppressing`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_conditional_refinement_should_stay_error_suppressing

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_conditional_refinement_should_stay_error_suppressing() {
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
        local function test(element: any?)
            if element then
                local owner = element._owner
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_correctly_lookup_a_shadowed_local_that_which_was_previously_refined {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1228:type_infer_refinements_correctly_lookup_a_shadowed_local_that_which_was_previously_refined`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_correctly_lookup_a_shadowed_local_that_which_was_previously_refined

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_correctly_lookup_a_shadowed_local_that_which_was_previously_refined() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: string? = "hi"
        assert(foo)
        local foo: number = 5
        print(foo:sub(1, 1))
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Type 'number' does not have key 'sub'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1242:type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {x: string | number}
        local t: T? = {x = "hi"}
        if t then
            if type(t.x) == "string" then
                local foo = t.x
            end
        end
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
          .require_type_at_position_position(Position::new(5, 30))
      )
    );
  }
}

mod type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1259:type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined_2`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined_2

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { x: { y: number }? }

        local function f(t: T?)
            if t and t.x then
                local foo = t.x.y
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 32)))
    );
  }
}

mod type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1985:type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {string}, s: string)
            local v1 = t[5]
            local v2 = v1

            if typeof(v1) == "nil" then
                local foo = v1
            else
                local foo = v1
            end

            if typeof(v2) == "nil" then
                local foo = v2
            else
                local foo = v2
            end

            if typeof(s) == "nil" then
                local foo = s -- line 18
            else
                local foo = s -- line 20
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(8, 28))
      )
    );

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(12, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(14, 28))
      )
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "nil & string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(18, 28))
        )
      );
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(20, 28))
        )
      );
    } else {
      assert_eq!(
        "nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(18, 28))
        )
      );
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(20, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_discriminate_from_isa_of_x {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1424:type_infer_refinements_discriminate_from_isa_of_x`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_discriminate_from_isa_of_x

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_discriminate_from_isa_of_x() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {tag: "Part", x: Part} | {tag: "Folder", x: Folder}

        local function f(t: T)
            if t.x:IsA("Part") then
                local foo = t
            else
                local bar = t
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      r#"{ tag: "Part", x: Part }"#,
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
    assert_eq!(
      r#"{ tag: "Folder", x: Folder }"#,
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  }
}

mod type_infer_refinements_discriminate_from_truthiness_of_x {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1290:type_infer_refinements_discriminate_from_truthiness_of_x`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_discriminate_from_truthiness_of_x

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_discriminate_from_truthiness_of_x() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {tag: "missing", x: nil} | {tag: "exists", x: string}

        local function f(t: T)
            if t.x then
                local foo = t
            else
                local bar = t
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        r#"{ tag: "exists", x: string }"#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
      assert_eq!(
        r#"{ tag: "missing", x: nil }"#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
      );
    } else {
      assert_eq!(
        r#"{ tag: "exists", x: string }"#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
      assert_eq!(
        r#"{ tag: "exists", x: string } | { tag: "missing", x: nil }"#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
      );
    }
  }
}

mod type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1391:type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { ok: true, value: T }
        type Err<E> = { ok: false, error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function apply<T, E>(t: Result<T, E>, f: (T) -> (), g: (E) -> ())
            if t.ok then
                f(t.value)
            else
                g(t.error)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_discriminate_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1318:type_infer_refinements_discriminate_tag`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_discriminate_tag

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_discriminate_tag() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Cat = {tag: "Cat", name: string, catfood: string}
        type Dog = {tag: "Dog", name: string, dogfood: string}
        type Animal = Cat | Dog

        local function f(animal: Animal)
            if animal.tag == "Cat" then
                local cat = animal
            elseif animal.tag == "Dog" then
                local dog = animal
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Cat",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 33)))
    );
    assert_eq!(
      "Dog",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 33)))
    );
  }
}

mod type_infer_refinements_discriminate_tag_with_implicit_else {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1340:type_infer_refinements_discriminate_tag_with_implicit_else`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_discriminate_tag_with_implicit_else

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_discriminate_tag_with_implicit_else() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Cat = {tag: "Cat", name: string, catfood: string}
        type Dog = {tag: "Dog", name: string, dogfood: string}
        type Animal = Cat | Dog

        local function f(animal: Animal)
            if animal.tag == "Cat" then
                local cat = animal
            else
                local dog = animal
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Cat",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 33)))
    );
    assert_eq!(
      "Dog",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 33)))
    );
  }
}

mod type_infer_refinements_either_number_or_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1016:type_infer_refinements_either_number_or_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_either_number_or_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_either_number_or_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: any, y: unknown)
            if type(x) == "number" or type(x) == "string" then
                local foo = x
            end
            if type(y) == "number" or type(y) == "string" then
                local foo = y
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "*error-type* | number | string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    } else {
      assert_eq!(
        "number | string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    }
    assert_eq!(
      "number | string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
  }
}

mod type_infer_refinements_eliminate_subclasses_of_instance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1535:type_infer_refinements_eliminate_subclasses_of_instance`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_eliminate_subclasses_of_instance

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_eliminate_subclasses_of_instance() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part | Folder | string)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder | Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1854:type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", err: E }
        type Result<T, E> = Ok<T> | Err<E>

        function and_then<T, U, E>(r: Result<T, E>, f: (T) -> U): Result<U, E>
            if r.tag == "ok" then
                return { tag = "ok", value = f(r.value) }
            else
                return r
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_ensure_t_after_return_references_all_reachable_points {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2330:type_infer_refinements_ensure_t_after_return_references_all_reachable_points`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_ensure_t_after_return_references_all_reachable_points

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_ensure_t_after_return_references_all_reachable_points() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
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
        local t = {}

        local function f(k: string)
            if t[k] ~= nil then
                return
            end

            t[k] = 5
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let ty = fixture
      .base
      .require_type_at_position_position(Position::new(8, 12));
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ [string]: number }",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    } else {
      assert_eq!(
        "{| [string]: number |}",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    }
  }
}

mod type_infer_refinements_ex {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2282:type_infer_refinements_ex`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_refinements_ex

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_ex() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function f(x: string | number)
    if typeof((x)) == "string" then
        local y = x
    end
end
"#,
      ),
      None,
    );

    let t = fixture
      .base
      .require_type_at_position_position(Position::new(3, 18));
    assert_eq!("string", to_string_type_id(t));
  }
}

mod type_infer_refinements_fail_to_refine_a_property_of_subscript_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2076:type_infer_refinements_fail_to_refine_a_property_of_subscript_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_fail_to_refine_a_property_of_subscript_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_fail_to_refine_a_property_of_subscript_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Foo = { foo: number? }
        local function f(t: {Foo})
            if t[1].foo then
                local foo = t[1].foo
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 34)))
    );
  }
}

mod type_infer_refinements_falsiness_of_truthy_predicate_narrows_into_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1820:type_infer_refinements_falsiness_of_truthy_predicate_narrows_into_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_falsiness_of_truthy_predicate_narrows_into_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_falsiness_of_truthy_predicate_narrows_into_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {number})
            local x = t[1]
            if not x then
                local foo = x
            else
                local bar = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(4, 28))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
  }
}

mod type_infer_refinements_foo_call_should_not_refine {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3048:type_infer_refinements_foo_call_should_not_refine`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_foo_call_should_not_refine

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_foo_call_should_not_refine() {
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
        --!strict

        local x = {} :: unknown
        local function foo(_: boolean) end

        if typeof(x) == "table" then
            foo(typeof(x.transform) == "function")
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'table' does not have key 'transform'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2918:type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record SimplifyConstraint (Analysis/include/Luau/Constraint.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
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
        local function track(instance): boolean
            local isBasePart = instance:IsA("BasePart")
            local isCharacter = false
            if not isBasePart then
                isCharacter = instance:FindFirstChildOfClass("Humanoid") and instance:FindFirstChild("HumanoidRootPart")
            end
            return isCharacter
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
  }
}

mod type_infer_refinements_free_type_is_equal_to_an_lvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:684:type_infer_refinements_free_type_is_equal_to_an_lvalue`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
  //!   - calls -> method RefinementExternTypeFixture::getFrontend (tests/TypeInfer.refinements.test.cpp)
  //!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
  //!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_free_type_is_equal_to_an_lvalue

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_free_type_is_equal_to_an_lvalue() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
        type_arena::TypeArena, unifier_shared_state::UnifierSharedState,
      },
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a, b: string?)
            if a == b then
                local foo, bar = a, b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "unknown",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
      );

      let (builtin_types, ice_handler) = {
        let frontend = fixture.get_frontend();
        (
          frontend.builtin_types,
          &mut frontend.ice_handler as *mut InternalErrorReporter,
        )
      };
      let mut arena = TypeArena::default();
      let mut state = UnifierSharedState::new(ice_handler);
      let mut normalizer = Normalizer::new(
        &mut arena as *mut TypeArena,
        builtin_types,
        &mut state as *mut UnifierSharedState,
        SolverMode::New,
        false,
      );
      let ty = fixture.require_type_at_position_position(Position::new(3, 36));
      let normalized = normalizer.normalize(ty);
      assert_eq!(
        "string?",
        to_string_type_id(normalizer.type_from_normal(normalized.as_ref()))
      );
    } else {
      assert_eq!(
        "a",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
      );
      assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
      );
    }
  }
}

mod type_infer_refinements_function_call_with_colon_after_refining_not_to_be_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2115:type_infer_refinements_function_call_with_colon_after_refining_not_to_be_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_function_call_with_colon_after_refining_not_to_be_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_function_call_with_colon_after_refining_not_to_be_nil() {
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
        export type Observer<T> = {
            read complete: ((self: Observer<T>) -> ())?,
        }

        local function _f(handler: Observer<any>)
            assert(handler.complete ~= nil)
            handler:complete() -- incorrectly gives Value of type '((Observer<any>) -> ())?' could be nil
            handler.complete(handler) -- works fine, both forms should avoid the error
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_function_calls_are_not_nillable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2610:type_infer_refinements_function_calls_are_not_nillable`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_function_calls_are_not_nillable

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_function_calls_are_not_nillable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local BEFORE_SLASH_PATTERN = "^(.*)[\\/]"
        function operateOnPath(path: string): string?
            local fileName = string.gsub(path, BEFORE_SLASH_PATTERN, "")
            if string.match(fileName, "^init%.") then
                return "path=" .. fileName
            end
            return nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_fuzz_filtered_refined_types_are_followed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1873:type_infer_refinements_fuzz_filtered_refined_types_are_followed`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_fuzz_filtered_refined_types_are_followed

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_fuzz_filtered_refined_types_are_followed() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local _
do
local _ = _ ~= _ or _ or _
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_globals_can_be_narrowed_too {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2215:type_infer_refinements_globals_can_be_narrowed_too`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_refinements_globals_can_be_narrowed_too

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_globals_can_be_narrowed_too() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        if typeof(string) == 'string' then
            local foo = string
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string & typeof(string)",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(2, 24))
        )
      );
    } else {
      assert_eq!(
        "never",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(2, 24))
        )
      );
    }
  }
}

mod type_infer_refinements_impossible_type_narrow_is_not_an_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:533:type_infer_refinements_impossible_type_narrow_is_not_an_error`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_impossible_type_narrow_is_not_an_error

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_impossible_type_narrow_is_not_an_error() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {string} = {"a", "b", "c"}
        local v = t[4]
        if not v then
            t[4] = "d"
        else
            print(v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_index_on_a_refined_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:574:type_infer_refinements_index_on_a_refined_property`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_index_on_a_refined_property

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_index_on_a_refined_property() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: {y: string}?} = {x = {y = "hello!"}}

        if t.x then
            print(t.x.y)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_inline_if_conditional_context {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2974:type_infer_refinements_inline_if_conditional_context`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_inline_if_conditional_context

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_inline_if_conditional_context() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type Value<T> = {
            kind: "value",
            value: T
        }

        local function peek<T>(state: Value<T> | T): T
            return if typeof(state) == "table" and state.kind == "value"
                then (state :: Value<T>).value :: T
                else state :: T
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_invert_is_truthy_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:171:type_infer_refinements_invert_is_truthy_constraint`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_invert_is_truthy_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_invert_is_truthy_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v: string?)
            if not v then
                local s = v
            else
                local s = v
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
    );
  }
}

mod type_infer_refinements_invert_is_truthy_constraint_ifelse_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1132:type_infer_refinements_invert_is_truthy_constraint_ifelse_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_infer_refinements_invert_is_truthy_constraint_ifelse_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_invert_is_truthy_constraint_ifelse_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v:string?)
            return if not v then tostring(v) else v
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(2, 42))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(2, 50))
      )
    );
  }
}

mod type_infer_refinements_invert_is_truthy_constraint_while_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1198:type_infer_refinements_invert_is_truthy_constraint_while_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_invert_is_truthy_constraint_while_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_invert_is_truthy_constraint_while_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v:string?)
            while not v do
                local foo = v
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

mod type_infer_refinements_is_truthy_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:153:type_infer_refinements_is_truthy_constraint`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_is_truthy_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_is_truthy_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v: string?)
            if v then
                local s = v
            else
                local s = v
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
    );
  }
}

mod type_infer_refinements_is_truthy_constraint_ifelse_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1118:type_infer_refinements_is_truthy_constraint_ifelse_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_infer_refinements_is_truthy_constraint_ifelse_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_is_truthy_constraint_ifelse_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v:string?)
            return if v then v else tostring(v)
        end
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
          .require_type_at_position_position(Position::new(2, 29))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(2, 45))
      )
    );
  }
}

mod type_infer_refinements_is_truthy_constraint_while_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1183:type_infer_refinements_is_truthy_constraint_while_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_is_truthy_constraint_while_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_is_truthy_constraint_while_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v:string?)
            while v do
                local foo = v
            end
        end
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
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

mod type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1638:type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x): Instance
            if x:IsA("Folder") then
                local foo = x
            else
                local foo = x
            end

            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    } else {
      assert_eq!(
        "Instance",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "Instance",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_len_operator_in_if_is_just_a_proposition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2940:type_infer_refinements_len_operator_in_if_is_just_a_proposition`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_refinements_len_operator_in_if_is_just_a_proposition

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_len_operator_in_if_is_just_a_proposition() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Pool = { x : number }
local pool = p :: Pool
if #pool then
    local y = pool
end
"#,
      ),
      None,
    );

    let ty = fixture.require_type_at_position_position(Position::new(4, 14));
    assert_ne!("never", to_string_type_id(ty));
  }
}

mod type_infer_refinements_limit_complexity_of_arithmetic_type_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2780:type_infer_refinements_limit_complexity_of_arithmetic_type_functions`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_limit_complexity_of_arithmetic_type_functions

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_limit_complexity_of_arithmetic_type_functions() {
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
        local Hermite = {}

        function Hermite:__init(p0, p1, m0, m1)
            self[1] = {
                p0.x;
                p0.y;
                p0.z;
            }
            self[2] = {
                m0.x;
                m0.y;
                m0.z;
            }
            self[3] = {
                3*(p1.x - p0.x) - 2*m0.x - m1.x;
                3*(p1.y - p0.y) - 2*m0.y - m1.y;
                3*(p1.z - p0.z) - 2*m0.z - m1.z;
            }
        end

        return Hermite
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_refinements_long_disjunction_of_refinements_should_not_trip_recursion_counter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2352:type_infer_refinements_long_disjunction_of_refinements_should_not_trip_recursion_counter`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_long_disjunction_of_refinements_should_not_trip_recursion_counter

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_long_disjunction_of_refinements_should_not_trip_recursion_counter() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function(obj)
    if script.Parent.SeatNumber.Value == "1D" or
    script.Parent.SeatNumber.Value == "2D" or
    script.Parent.SeatNumber.Value == "3D" or
    script.Parent.SeatNumber.Value == "4D" or
    script.Parent.SeatNumber.Value == "5D" or
    script.Parent.SeatNumber.Value == "6D" or
    script.Parent.SeatNumber.Value == "7D" or
    script.Parent.SeatNumber.Value == "8D" or
    script.Parent.SeatNumber.Value == "9D" or
    script.Parent.SeatNumber.Value == "10D" or
    script.Parent.SeatNumber.Value == "11D" or
    script.Parent.SeatNumber.Value == "12D" or
    script.Parent.SeatNumber.Value == "13D" or
    script.Parent.SeatNumber.Value == "14D" or
    script.Parent.SeatNumber.Value == "15D" or
    script.Parent.SeatNumber.Value == "16D" or
    script.Parent.SeatNumber.Value == "1C" or
    script.Parent.SeatNumber.Value == "2C" or
    script.Parent.SeatNumber.Value == "3C" or
    script.Parent.SeatNumber.Value == "4C" or
    script.Parent.SeatNumber.Value == "5C" or
    script.Parent.SeatNumber.Value == "6C" or
    script.Parent.SeatNumber.Value == "7C" or
    script.Parent.SeatNumber.Value == "8C" or
    script.Parent.SeatNumber.Value == "9C" or
    script.Parent.SeatNumber.Value == "10C" or
    script.Parent.SeatNumber.Value == "11C" or
    script.Parent.SeatNumber.Value == "12C" or
    script.Parent.SeatNumber.Value == "13C" or
    script.Parent.SeatNumber.Value == "14C" or
    script.Parent.SeatNumber.Value == "15C" or
    script.Parent.SeatNumber.Value == "16C" then
end
"#,
      ),
      None,
    );
  }
}

mod type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2232:type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction() {
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
        local function isIndexKey(k, contiguousLength)
            return type(k) == "number"
                and k <= contiguousLength -- nothing out of bounds
                and 1 <= k -- nothing illegal for array indices
                and math.floor(k) == k -- no float keys
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction_variant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2266:type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction_variant`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction_variant

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction_variant() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function isIndexKey(k, contiguousLength: number)
            return type(k) == "number"
                and k <= contiguousLength -- nothing out of bounds
                and 1 <= k -- nothing illegal for array indices
                and math.floor(k) == k -- no float keys
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_lvalue_is_equal_to_a_term {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:619:type_infer_refinements_lvalue_is_equal_to_a_term`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_lvalue_is_equal_to_a_term

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_lvalue_is_equal_to_a_term() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: (string | number)?)
            if a == 1 then
                local foo = a
            else
                local foo = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  }
}

mod type_infer_refinements_lvalue_is_equal_to_another_lvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:598:type_infer_refinements_lvalue_is_equal_to_another_lvalue`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_lvalue_is_equal_to_another_lvalue

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_lvalue_is_equal_to_another_lvalue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: (string | number)?, b: boolean?)
            if a == b then
                local foo, bar = a, b
            else
                local foo, bar = a, b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
    );
    assert_eq!(
      "boolean?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
    );

    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 33)))
    );
    assert_eq!(
      "boolean?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 36)))
    );
  }
}

mod type_infer_refinements_lvalue_is_not_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:663:type_infer_refinements_lvalue_is_not_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_lvalue_is_not_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_lvalue_is_not_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: (string | number)?)
            if a ~= nil then
                local foo = a
            else
                local foo = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number | string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "nil",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    } else {
      assert_eq!(
        "(number | string)?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    }
  }
}

mod type_infer_refinements_many_refinements_on_val {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2160:type_infer_refinements_many_refinements_on_val`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_many_refinements_on_val

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_many_refinements_on_val() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function is_nan(val: any): boolean
            return type(val) == "number" and val ~= val
        end

        local function is_js_boolean(val: any): boolean
            return not not val and val ~= 0 and val ~= "" and not is_nan(val)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "(any) -> boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("is_nan")))
    );
    assert_eq!(
      "(any) -> boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("is_js_boolean"))
      )
    );
  }
}

mod type_infer_refinements_merge_should_be_fully_agnostic_of_hashmap_ordering {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1081:type_infer_refinements_merge_should_be_fully_agnostic_of_hashmap_ordering`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_merge_should_be_fully_agnostic_of_hashmap_ordering

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_merge_should_be_fully_agnostic_of_hashmap_ordering() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(b: string | { x: string }, a)
            assert(type(a) == "string")
            assert(type(b) == "string" or type(b) == "table")

            if type(b) == "string" then
                local foo = b
            end
        end
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
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
  }
}

mod type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2392:type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or
    script.Parent.SeatNumber.Value == "2D" or
    script.Parent.SeatNumber.Value == "3D" or
    script.Parent.SeatNumber.Value == "4D" or
    script.Parent.SeatNumber.Value == "5D" or
    script.Parent.SeatNumber.Value == "6D" or
    script.Parent.SeatNumber.Value == "7D" or
    script.Parent.SeatNumber.Value == "8D" or
    script.Parent.SeatNumber.Value == "9D" or
    script.Parent.SeatNumber.Value == "10D" or
    script.Parent.SeatNumber.Value == "11D" or
    script.Parent.SeatNumber.Value == "12D" or
    script.Parent.SeatNumber.Value == "13D" or
    script.Parent.SeatNumber.Value == "14D" or
    script.Parent.SeatNumber.Value == "15D" or
    script.Parent.SeatNumber.Value == "16D" or
    script.Parent.SeatNumber.Value == "1C" or
    script.Parent.SeatNumber.Value == "2C" or
    script.Parent.SeatNumber.Value == "3C" or
    script.Parent.SeatNumber.Value == "4C" or
    script.Parent.SeatNumber.Value == "5C" or
    script.Parent.SeatNumber.Value == "6C" or
    script.Parent.SeatNumber.Value == "7C" or
    script.Parent.SeatNumber.Value == "8C" or
    script.Parent.SeatNumber.Value == "9C" or
    script.Parent.SeatNumber.Value == "10C" or
    script.Parent.SeatNumber.Value == "11C" or
    script.Parent.SeatNumber.Value == "12C" or
    script.Parent.SeatNumber.Value == "13C" or
    script.Parent.SeatNumber.Value == "14C" or
    script.Parent.SeatNumber.Value == "15C" or
    script.Parent.SeatNumber.Value == "16C" then
    end)
"#,
      ),
      None,
    );
  }
}

mod type_infer_refinements_mutate_prop_of_some_refined_symbol {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2295:type_infer_refinements_mutate_prop_of_some_refined_symbol`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_refinements_mutate_prop_of_some_refined_symbol

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_mutate_prop_of_some_refined_symbol() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function instances(): {Instance} error("") end
        local function vec3(x, y, z): Vector3 error("") end

        for _, object in ipairs(instances()) do
            if object:IsA("Part") then
                object.Position = vec3(1, 2, 3)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_mutate_prop_of_some_refined_symbol_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2311:type_infer_refinements_mutate_prop_of_some_refined_symbol_2`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_mutate_prop_of_some_refined_symbol_2

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_mutate_prop_of_some_refined_symbol_2() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Result<T, E> = never
            | { tag: "ok", value: T }
            | { tag: "err", error: E }

        local function results(): {Result<number, string>} error("") end

        for _, res in ipairs(results()) do
            if res.tag == "ok" then
                res.value = 7
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_narrow_boolean_to_true_or_false {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1373:type_infer_refinements_narrow_boolean_to_true_or_false`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_narrow_boolean_to_true_or_false

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_narrow_boolean_to_true_or_false() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: boolean)
            if x then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "true",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "false",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  }
}

mod type_infer_refinements_narrow_from_subclasses_of_instance_or_string_or_vector_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1553:type_infer_refinements_narrow_from_subclasses_of_instance_or_string_or_vector_3`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_narrow_from_subclasses_of_instance_or_string_or_vector_3

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_narrow_from_subclasses_of_instance_or_string_or_vector_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part | Folder | string | Vector3)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder | Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Vector3 | string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_narrow_property_of_a_bounded_variable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:755:type_infer_refinements_narrow_property_of_a_bounded_variable`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_narrow_property_of_a_bounded_variable

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_narrow_property_of_a_bounded_variable() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t
        local u: {x: number?} = {x = nil}
        t = u

        if t.x then
            local foo: number = t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_non_conditional_context_in_if_should_not_refine {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3088:type_infer_refinements_non_conditional_context_in_if_should_not_refine`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_non_conditional_context_in_if_should_not_refine

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_non_conditional_context_in_if_should_not_refine() {
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
        local function bing(_: any) end
        local function foobar(x: unknown)
            assert(typeof(x) == "table")
            if bing(x.foo) then
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'table' does not have key 'foo'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_refinements_nonnil_refinement_on_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2551:type_infer_refinements_nonnil_refinement_on_generic`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_nonnil_refinement_on_generic

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_nonnil_refinement_on_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function printOptional<T>(item: T?, printer: (T) -> string): string
            if item ~= nil then
                return printer(item)
            else
                return ""
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "T & ~nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 31))
        )
      );
    } else {
      assert_eq!(
        "T",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 31))
        )
      );
    }
  }
}

mod type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:788:type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {"hello"}
        local v = t[2]
        if type(v) == "nil" then
            local foo = v
        else
            local foo = v
        end

        if not (type(v) ~= "nil") then
            local foo = v
        else
            local foo = v
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "nil & string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(4, 24))
        )
      );
      assert_eq!(
        "string & ~nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 24))
        )
      );

      assert_eq!(
        "nil & string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(10, 24))
        )
      );
      assert_eq!(
        "string & ~nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(12, 24))
        )
      );
    } else {
      assert_eq!(
        "nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(4, 24))
        )
      );
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 24))
        )
      );

      assert_eq!(
        "nil",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(10, 24))
        )
      );
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(12, 24))
        )
      );
    }
  }
}

mod type_infer_refinements_not_a_and_not_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:982:type_infer_refinements_not_a_and_not_b`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_a_and_not_b

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_a_and_not_b() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: number?, b: number?)
            if (not a) and (not b) then
                local foo = a
                local bar = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
  }
}

mod type_infer_refinements_not_a_and_not_b_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:999:type_infer_refinements_not_a_and_not_b_2`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_a_and_not_b_2

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_a_and_not_b_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: number?, b: number?)
            if not (a or b) then
                local foo = a
                local bar = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
  }
}

mod type_infer_refinements_not_a_or_not_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:948:type_infer_refinements_not_a_or_not_b`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_a_or_not_b

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_a_or_not_b() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: number?, b: number?)
            if (not a) or (not b) then
                local foo = a
                local bar = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
  }
}

mod type_infer_refinements_not_a_or_not_b_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:965:type_infer_refinements_not_a_or_not_b_2`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_a_or_not_b_2

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_a_or_not_b_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: number?, b: number?)
            if not (a and b) then
                local foo = a
                local bar = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
  }
}

mod type_infer_refinements_not_and_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:230:type_infer_refinements_not_and_constraint`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_and_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_and_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: string?, b: number?)
            if not (a and b) then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
    );
  }
}

mod type_infer_refinements_not_t_or_some_prop_of_t {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1038:type_infer_refinements_not_t_or_some_prop_of_t`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_not_t_or_some_prop_of_t

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_not_t_or_some_prop_of_t() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {x: boolean}?)
            if not t or t.x then
                local foo = t
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "({ read x: ~(false?) } & { x: boolean })?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
      );
    } else {
      assert_eq!(
        "{ x: boolean }?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
      );
    }
  }
}

mod type_infer_refinements_or_predicate_with_truthy_predicates {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:253:type_infer_refinements_or_predicate_with_truthy_predicates`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_or_predicate_with_truthy_predicates

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_or_predicate_with_truthy_predicates() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: string?, b: number?)
            if a or b then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
    );
  }
}

mod type_infer_refinements_oss_1451 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2663:type_infer_refinements_oss_1451`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_oss_1451

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_oss_1451() {
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
        type Part = {
            HasTag: (Part, string) -> boolean,
            Name: string,
        }
        local myList = {} :: {Part}
        local nextPart = (table.remove(myList)) :: Part

        if nextPart:HasTag("foo") then
          return
        end

        print(nextPart.Name)

    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_oss_1517_equality_doesnt_add_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2992:type_infer_refinements_oss_1517_equality_doesnt_add_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_oss_1517_equality_doesnt_add_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_oss_1517_equality_doesnt_add_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MyType = {
            data: any
        }

        local function createMyType(): MyType
            local obj = { data = {} }
            return obj
        end

        local function testTypeInference()
            local a: MyType = createMyType()
            local b: MyType = createMyType()

            if a == b then
                local c: MyType = b
                local value = b.data
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_oss_1528_method_calls_are_not_nillable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2624:type_infer_refinements_oss_1528_method_calls_are_not_nillable`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_oss_1528_method_calls_are_not_nillable

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_oss_1528_method_calls_are_not_nillable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type RunService = {
            IsRunning: (RunService) -> boolean
        }
        type Game = {
            GetRunService: (Game) -> RunService
        }
        local function getServices(g: Game): RunService
            local service = g:GetRunService()
            if service:IsRunning() then
                return service
            end
            error("Oh no! The service isn't running!")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_oss_1687_equality_shouldnt_leak_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2643:type_infer_refinements_oss_1687_equality_shouldnt_leak_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_oss_1687_equality_shouldnt_leak_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_oss_1687_equality_shouldnt_leak_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function returns_two(): number
            return 2
        end

        function is_two(num: number): boolean
            return num==2
        end

        local my_number = returns_two()

        if my_number == 2 then
            is_two(my_number) --type error, my_number: number?
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_oss_1835 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2749:type_infer_refinements_oss_1835`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_refinements_oss_1835

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_oss_1835() {
    use alloc::string::String;

    use ulua_analysis::records::optional_value_access::OptionalValueAccess;
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
        local t: {name: string}? = nil

        function f()
            local name = if t then t.name else "name"
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t: {name: string}? = nil

        function f()
            if t then end
            local name = if t then t.name else "name"
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {name: string}? = nil
        if t then end
        print(t.name)
        local name = if t then t.name else "name"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<OptionalValueAccess>(&result.errors[0])
      .expect("expected OptionalValueAccess");
  }
}

mod type_infer_refinements_parenthesized_expressions_are_followed_through {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:189:type_infer_refinements_parenthesized_expressions_are_followed_through`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_parenthesized_expressions_are_followed_through

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_parenthesized_expressions_are_followed_through() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(v: string?)
            if (not v) then
                local s = v
            else
                local s = v
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
    );
  }
}

mod type_infer_refinements_prove_that_dataflow_analysis_isnt_doing_alias_tracking_yet {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2056:type_infer_refinements_prove_that_dataflow_analysis_isnt_doing_alias_tracking_yet`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_prove_that_dataflow_analysis_isnt_doing_alias_tracking_yet

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_prove_that_dataflow_analysis_isnt_doing_alias_tracking_yet() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(tag: "cat" | "dog")
            local tag2 = tag

            if tag2 == "cat" then
                local foo = tag
            else
                local foo = tag
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      r#""cat" | "dog""#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
    assert_eq!(
      r#""cat" | "dog""#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
    );
  }
}

mod type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1927:type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Id<T> = T

        local function f(x: Id<Id<Part | Folder> | Id<string>>)
            if typeof(x) ~= "string" and x:IsA("Part") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
    assert_eq!(
      "Folder | string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1946:type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function hof(f: (Instance) -> ()) end

        hof(function(inst)
            if inst:IsA("Part") then
                local foo = inst
            else
                local foo = inst
            end
        end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Instance & ~Part",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(7, 28))
        )
      );
    } else {
      assert_eq!(
        "Instance",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(7, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_refine_a_property_not_to_be_nil_through_an_intersection_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1410:type_infer_refinements_refine_a_property_not_to_be_nil_through_an_intersection_table`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_a_property_not_to_be_nil_through_an_intersection_table

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_a_property_not_to_be_nil_through_an_intersection_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {} & {f: ((string) -> string)?}
        local function f(t: T, x)
            if t.f then
                t.f(x)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_refine_a_property_of_some_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1968:type_infer_refinements_refine_a_property_of_some_global`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_a_property_of_some_global

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_a_property_of_some_global() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        foo = { bar = 5 :: number? }

        if foo.bar then
            local bar = foo.bar
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 30)))
      );
    }
  }
}

mod type_infer_refinements_refine_any_and_unknown_should_still_be_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3135:type_infer_refinements_refine_any_and_unknown_should_still_be_any`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_any_and_unknown_should_still_be_any

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_any_and_unknown_should_still_be_any() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local REACT_FRAGMENT_TYPE = (nil :: any)
        local function typeOf(object: any)
            local __type = object.type

            if __type == REACT_FRAGMENT_TYPE then
                return __type
            else
                return __type
                    and typeof(__type) == "table"
                    and __type["$$typeof"]
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_refine_boolean {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1769:type_infer_refinements_refine_boolean`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_boolean() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number | boolean)
            if typeof(x) == "boolean" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_buffer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1803:type_infer_refinements_refine_buffer`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_buffer

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_buffer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number | buffer)
            if typeof(x) == "buffer" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "buffer",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_by_no_refine_should_always_reduce {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2813:type_infer_refinements_refine_by_no_refine_should_always_reduce`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_by_no_refine_should_always_reduce

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_by_no_refine_should_always_reduce() {
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
        function foo(t): boolean return true end

        function select<K, V>(t: { [K]: V }, columns: { K }): { [K]: V }
            local result = {}
            if foo(t) then
                for k, v in t do
                    if table.find(columns, k) then
                        result[k] = v -- was TypeError: Type function instance refine<intersect<K, ~nil>, *no-refine*> is uninhabited
                    end
                end
            else
                for k, v in pairs(t) do
                    if table.find(columns, k) then
                        result[k] = v
                    end
                end
            end
            return result
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_refine_param_of_type_folder_or_part_without_using_typeof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1620:type_infer_refinements_refine_param_of_type_folder_or_part_without_using_typeof`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_param_of_type_folder_or_part_without_using_typeof

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_param_of_type_folder_or_part_without_using_typeof() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part | Folder)
            if x:IsA("Folder") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_param_of_type_instance_without_using_typeof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1602:type_infer_refinements_refine_param_of_type_instance_without_using_typeof`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_param_of_type_instance_without_using_typeof

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_param_of_type_instance_without_using_typeof() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Instance)
            if x:IsA("Folder") then
                local foo = x
            elseif typeof(x) == "table" then
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_the_correct_types_opposite_of_when_a_is_not_number_or_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1100:type_infer_refinements_refine_the_correct_types_opposite_of_when_a_is_not_number_or_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_the_correct_types_opposite_of_when_a_is_not_number_or_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_the_correct_types_opposite_of_when_a_is_not_number_or_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: string | number | boolean)
            if type(a) ~= "number" and type(a) ~= "string" then
                local foo = a
            else
                local foo = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "number | string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_the_correct_types_opposite_of_while_a_is_not_number_or_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1213:type_infer_refinements_refine_the_correct_types_opposite_of_while_a_is_not_number_or_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_the_correct_types_opposite_of_while_a_is_not_number_or_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_the_correct_types_opposite_of_while_a_is_not_number_or_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: string | number | boolean)
            while type(a) ~= "number" and type(a) ~= "string" do
                local foo = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_thread {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1786:type_infer_refinements_refine_thread`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_thread

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_thread() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number | thread)
            if typeof(x) == "thread" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "thread",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_refine_unknown_to_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2178:type_infer_refinements_refine_unknown_to_table`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table() {
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
        local function f(a: unknown)
            if typeof(a) == "table" then
                for i, v in a do
                    return i, v
                end
            end

            error("")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "(unknown) -> (~nil, unknown)",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_refinements_refine_unknown_to_table_then_clone_it {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1907:type_infer_refinements_refine_unknown_to_table_then_clone_it`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_clone_it

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table_then_clone_it() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                local cloned: {} = table.clone(x)
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_refinements_refine_unknown_to_table_then_take_the_length {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1885:type_infer_refinements_refine_unknown_to_table_then_take_the_length`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_take_the_length

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table_then_take_the_length() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                local len = #x
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "table",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 29))
        )
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "unknown",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 29))
        )
      );
    }
  }
}

mod type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:403:type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
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
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                -- this should error, `x.foo` is an unknown property
                if typeof(x.foo.bar) == "string" then
                    return x.foo.bar
                end
            end

            return nil
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let up = type_error_data_ref::<UnknownProperty>(&result.errors[0])
        .expect("expected UnknownProperty");
      assert_eq!("bar", up.key());
      assert_eq!("unknown", to_string_type_id(up.table()));
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      for error in &result.errors {
        let up = type_error_data_ref::<UnknownProperty>(error)
          .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
        assert_eq!("foo", up.key());
        assert_eq!("unknown", to_string_type_id(up.table()));
      }
    }
  }
}

mod type_infer_refinements_refine_unknown_to_table_then_test_a_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:374:type_infer_refinements_refine_unknown_to_table_then_test_a_prop`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_test_a_prop

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table_then_test_a_prop() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
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
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                if typeof(x.foo) == "string" then
                    return x.foo
                end
            end

            return nil
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      for error in &result.errors {
        let up = type_error_data_ref::<UnknownProperty>(error)
          .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
        assert_eq!("foo", up.key());
        assert_eq!("unknown", to_string_type_id(up.table()));
      }
    }
  }
}

mod type_infer_refinements_refine_unknown_to_table_then_test_a_tested_nested_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:438:type_infer_refinements_refine_unknown_to_table_then_test_a_tested_nested_prop`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_test_a_tested_nested_prop

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknown_to_table_then_test_a_tested_nested_prop() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
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
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                if typeof(x.foo) == "table" and typeof(x.foo.bar) == "string" then
                    return x.foo.bar
                end
            end

            return nil
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(3, result.errors.len(), "{:?}", result.errors);

      for error in &result.errors {
        let up = type_error_data_ref::<UnknownProperty>(error)
          .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
        assert_eq!("foo", up.key());
        assert_eq!("unknown", to_string_type_id(up.table()));
      }
    }
  }
}

mod type_infer_refinements_refine_unknowns {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1743:type_infer_refinements_refine_unknowns`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refine_unknowns

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refine_unknowns() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            if type(x) == "string" then
                local foo = x
            else
                local bar = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "~string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    } else {
      assert_eq!(
        "string",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "unknown",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_refinements_from_and_should_not_refine_to_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2885:type_infer_refinements_refinements_from_and_should_not_refine_to_never`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Config (Config/include/Luau/Config.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refinements_from_and_should_not_refine_to_never

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refinements_from_and_should_not_refine_to_never() {
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
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Config with
            KeyboardEnabled: boolean
            MouseEnabled: boolean
        end
    "#,
      ),
      false,
    );

    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local config: Config
        local function serialize()
            if config.KeyboardEnabled and config.MouseEnabled then
                return 0
            else
                print(config)
                return 1
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, results.errors.len(), "{:?}", results.errors);

    let expected = if FFlag::LuauExternTypesNormalizeWithShapes.get() {
      "(Config & { read KeyboardEnabled: false? }) | (Config & { read MouseEnabled: false? })"
    } else {
      "Config"
    };
    assert_eq!(
      expected,
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 24))
      )
    );
  }
}

mod type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2432:type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or script.Parent.SeatNumber.Value == "2D" or script.Parent.SeatNumber.Value == "3D" or script.Parent.SeatNumber.Value == "4D" or script.Parent.SeatNumber.Value == "5D" or script.Parent.SeatNumber.Value == "6D" or script.Parent.SeatNumber.Value == "7D" or script.Parent.SeatNumber.Value == "8D" or script.Parent.SeatNumber.Value == "9D" or script.Parent.SeatNumber.Value == "10D" or script.Parent.SeatNumber.Value == "11D" or script.Parent.SeatNumber.Value == "12D" or script.Parent.SeatNumber.Value == "13D" or script.Parent.SeatNumber.Value == "14D" or script.Parent.SeatNumber.Value == "15D" or script.Parent.SeatNumber.Value == "16D" or script.Parent.SeatNumber.Value == "1C" or script.Parent.SeatNumber.Value == "2C" or script.Parent.SeatNumber.Value == "3C" or script.Parent.SeatNumber.Value == "4C" or script.Parent.SeatNumber.Value == "5C" or script.Parent.SeatNumber.Value == "6C" or script.Parent.SeatNumber.Value == "7C" or script.Parent.SeatNumber.Value == "8C" or script.Parent.SeatNumber.Value == "9C" or script.Parent.SeatNumber.Value == "10C" or script.Parent.SeatNumber.Value == "11C" or script.Parent.SeatNumber.Value == "12C" or script.Parent.SeatNumber.Value == "13C" or script.Parent.SeatNumber.Value == "14C" or script.Parent.SeatNumber.Value == "15C" or script.Parent.SeatNumber.Value == "16C" then
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	else
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
			print("Allowed")
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	end
end)
"#,
        ),
        None,
    );
  }
}

mod type_infer_refinements_refinements_should_not_affect_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2135:type_infer_refinements_refinements_should_not_affect_assignment`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function similar (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refinements_should_not_affect_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refinements_should_not_affect_assignment() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: unknown = true
        if a == true then
            a = 'not even remotely similar to a boolean'
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_refinements_should_preserve_error_suppression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2147:type_infer_refinements_refinements_should_preserve_error_suppression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_refinements_refinements_should_preserve_error_suppression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refinements_should_preserve_error_suppression() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: any = {}
        local b
        if typeof(a) == "table" then
           b = a.field
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_refinements_table_intersection_limits {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2454:type_infer_refinements_refinements_table_intersection_limits`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function checkpoint (Analysis/src/ConstraintGenerator.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_refinements_table_intersection_limits

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_refinements_table_intersection_limits() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Dir = {
    a: number?, b: number?, c: number?, d: number?, e: number?, f: number?,
    g: number?, h: number?, i: number?, j: number?, k: number?, l: number?,
    m: number?, n: number?, o: number?, p: number?, q: number?, r: number?,
}

local function test(dirs: {Dir})
    for k, dir in dirs
        local success, message = pcall(function()
            assert(dir.a == nil or type(dir.a) == "number")
            assert(dir.b == nil or type(dir.b) == "number")
            assert(dir.c == nil or type(dir.c) == "number")
            assert(dir.d == nil or type(dir.d) == "number")
            assert(dir.e == nil or type(dir.e) == "number")
            assert(dir.f == nil or type(dir.f) == "number")
            assert(dir.g == nil or type(dir.g) == "number")
            assert(dir.h == nil or type(dir.h) == "number")
            assert(dir.i == nil or type(dir.i) == "number")
            assert(dir.j == nil or type(dir.j) == "number")
            assert(dir.k == nil or type(dir.k) == "number")
            assert(dir.l == nil or type(dir.l) == "number")
            assert(dir.m == nil or type(dir.m) == "number")
            assert(dir.n == nil or type(dir.n) == "number")
            assert(dir.o == nil or type(dir.o) == "number")
            assert(dir.p == nil or type(dir.p) == "number")
            assert(dir.q == nil or type(dir.q) == "number")
            assert(dir.r == nil or type(dir.r) == "number")
            assert(dir.t == nil or type(dir.t) == "number")
            assert(dir.u == nil or type(dir.u) == "number")
            assert(dir.v == nil or type(dir.v) == "number")
            local checkpoint = dir

            checkpoint.w = 1
        end)
        assert(success)
    end
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_refinements_string_not_equal_to_string_or_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:732:type_infer_refinements_string_not_equal_to_string_or_nil`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_string_not_equal_to_string_or_nil

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_string_not_equal_to_string_or_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {string} = {"hello"}

        local a: string = t[1]
        local b: string? = nil
        if a ~= b then
            local foo, bar = a, b
        else
            local foo, bar = a, b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 29)))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 32)))
    );

    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 29)))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 32)))
    );
  }
}

mod type_infer_refinements_table_name_index_without_prior_assignment_from_branch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2846:type_infer_refinements_table_name_index_without_prior_assignment_from_branch`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_refinements_table_name_index_without_prior_assignment_from_branch

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_table_name_index_without_prior_assignment_from_branch() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::optional_value_access::OptionalValueAccess,
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
        local GetDictionary : (unknown, boolean) -> { Player: {} }? = nil :: any

        local CharEntry = GetDictionary(nil, false)
        if not CharEntry then
            CharEntry = GetDictionary(nil, true)
        end

        local x = CharEntry.Player
    "#,
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    type_error_data_ref::<OptionalValueAccess>(&results.errors[0])
      .expect("expected OptionalValueAccess");
    assert_eq!(
      "{  }",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_refinements_term_is_equal_to_an_lvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:637:type_infer_refinements_term_is_equal_to_an_lvalue`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_term_is_equal_to_an_lvalue

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_term_is_equal_to_an_lvalue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: (string | number)?)
            if "hello" == a then
                local foo = a
            else
                local foo = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        r#""hello""#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
      );
      assert_eq!(
        r#"((string & ~"hello") | number)?"#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    } else {
      assert_eq!(
        r#""hello""#,
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
      );
      assert_eq!(
        "(number | string)?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
    }
  }
}

mod type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2589:type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Item = {}

        local function predicate(value: Item): boolean
            return true
        end

        local function checkValue(value: Item)
            if predicate(value) then
                local _ = value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Item",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 27)))
    );
    assert_eq!(
      "Item",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
    );
  }
}

mod type_infer_refinements_truthy_constraint_on_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:550:type_infer_refinements_truthy_constraint_on_properties`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_truthy_constraint_on_properties

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_truthy_constraint_on_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: number?} = {x = 1}

        if t.x then
            local t2 = t
            local foo = t.x
        end

        local bar = t.x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ read x: number, write x: number? }",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 23)))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
      );
    }

    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_string(&String::from("bar")))
    );
  }
}

mod type_infer_refinements_truthy_refinement_on_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2570:type_infer_refinements_truthy_refinement_on_generic`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_truthy_refinement_on_generic

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_truthy_refinement_on_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function printOptional<T>(item: T?, printer: (T) -> string): string
            if item then
                return printer(item)
            else
                return ""
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "T & ~(false?)",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 31))
        )
      );
    } else {
      assert_eq!(
        "T",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 31))
        )
      );
    }
  }
}

mod type_infer_refinements_type_annotations_arent_relevant_when_doing_dataflow_analysis {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2091:type_infer_refinements_type_annotations_arent_relevant_when_doing_dataflow_analysis`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_type_annotations_arent_relevant_when_doing_dataflow_analysis

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_annotations_arent_relevant_when_doing_dataflow_analysis() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function s() return "hello" end

        local function f(t: {string})
            local s1: string = t[5]
            local s2: string = s()

            if typeof(s1) == "nil" and typeof(s2) == "nil" then
                local foo = s1
                local bar = s2
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(8, 28))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(9, 28))
      )
    );
  }
}

mod type_infer_refinements_type_assertion_expr_carry_its_constraints {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:307:type_infer_refinements_type_assertion_expr_carry_its_constraints`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_assertion_expr_carry_its_constraints

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_assertion_expr_carry_its_constraints() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function g(a: number?, b: string?)
            if (a :: any) and (b :: any) then
                local x = a
                local y = b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
      );
      assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
      );
    }
  }
}

mod type_infer_refinements_type_comparison_ifelse_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1146:type_infer_refinements_type_comparison_ifelse_expression`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_comparison_ifelse_expression

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_comparison_ifelse_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function returnOne(x)
            return 1
        end

        function f(v:any)
            return if typeof(v) == "number" then v else returnOne(v)
        end

        function g(v:unknown)
            return if typeof(v) == "number" then v else returnOne(v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "*error-type* | number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 49))
        )
      );
      assert_eq!(
        "*error-type* | ~number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 66))
        )
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 49))
        )
      );
      assert_eq!(
        "any",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(6, 66))
        )
      );
    }

    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(10, 49))
      )
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "~number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(10, 66))
        )
      );
    } else {
      assert_eq!(
        "unknown",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(10, 66))
        )
      );
    }
  }
}

mod type_infer_refinements_type_function_reduction_with_union_type_application {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3105:type_infer_refinements_type_function_reduction_with_union_type_application`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_function_reduction_with_union_type_application

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_function_reduction_with_union_type_application() {
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
        local lastTick = 0
        local jumpAnimTime = 0
        local toolAnimTime = 0

        function move(time, tool, animStringValueObject)
            local deltaTime = time - lastTick
            lastTick = time

            if jumpAnimTime > 0 then
                jumpAnimTime = jumpAnimTime - deltaTime
            end

            if animStringValueObject then
                toolAnimTime = time + .3
            end

            if time > toolAnimTime then
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_type_guard_can_filter_for_intersection_of_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:884:type_infer_refinements_type_guard_can_filter_for_intersection_of_tables`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_guard_can_filter_for_intersection_of_tables

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_guard_can_filter_for_intersection_of_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type XYCoord = {x: number} & {y: number}
        local function f(t: XYCoord?)
            if type(t) == "table" then
                local foo = t
            else
                local foo = t
            end
        end
    "#,
      ),
      None,
    );

    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    let table_ty = fixture
      .base
      .require_type_at_position_position(Position::new(4, 28));
    assert_eq!(
      "{ x: number } & { y: number }",
      to_string_type_id_to_string_options(table_ty, &mut opts)
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
  }
}

mod type_infer_refinements_type_guard_can_filter_for_overloaded_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:905:type_infer_refinements_type_guard_can_filter_for_overloaded_function`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_type_guard_can_filter_for_overloaded_function

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_guard_can_filter_for_overloaded_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type SomeOverloadedFunction = ((number) -> string) & ((string) -> number)
        local function f(g: SomeOverloadedFunction?)
            if type(g) == "function" then
                local foo = g
            else
                local foo = g
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "((number) -> string) & ((string) -> number)",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(4, 28))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 28))
      )
    );
  }
}

mod type_infer_refinements_type_guard_narrowed_into_nothingness {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:924:type_infer_refinements_type_guard_narrowed_into_nothingness`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_guard_narrowed_into_nothingness

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_guard_narrowed_into_nothingness() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: {x: number})
            if type(t) ~= "table" then
                local foo = t
                error(("Expected a table, got %s"):format(type(t)))
            end

            return t.x + 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ x: number } & ~table",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    } else {
      assert_eq!(
        "never",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_type_narrow_but_the_discriminant_type_isnt_a_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1509:type_infer_refinements_type_narrow_but_the_discriminant_type_isnt_a_class`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_type_narrow_but_the_discriminant_type_isnt_a_class

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_narrow_but_the_discriminant_type_isnt_a_class() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string | number | Instance | Vector3)
            if type(x) == "any" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "never",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "Instance | Vector3 | number | string",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    } else {
      assert_eq!(
        "*error-type*",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
      assert_eq!(
        "*error-type*",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_type_narrow_for_all_the_userdata {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1491:type_infer_refinements_type_narrow_for_all_the_userdata`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_type_narrow_for_all_the_userdata

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_narrow_for_all_the_userdata() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string | number | Instance | Vector3)
            if type(x) == "userdata" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Instance | Vector3",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "number | string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_type_narrow_to_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:770:type_infer_refinements_type_narrow_to_vector`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_type_narrow_to_vector

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_narrow_to_vector() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x)
            if type(x) == "vector" then
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "unknown & vector",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    } else {
      assert_eq!(
        "*error-type*",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_type_vector_refine {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3219:type_infer_refinements_type_vector_refine`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_refinements_type_vector_refine

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_type_vector_refine() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
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
        function foo(x: unknown)
            if type(x) == "vector" then
                local y = x.y
                local z = y.bad
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  }
}

mod type_infer_refinements_typeguard_cast_free_table_to_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1445:type_infer_refinements_typeguard_cast_free_table_to_vector`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method RefinementExternTypeFixture::getFrontend (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_cast_free_table_to_vector

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_cast_free_table_to_vector() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode, functions::to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    // CLI-115286 - Refining via type(x) == 'vector' does not work in the new solver
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(vec)
            local X, Y, Z = vec.X, vec.Y, vec.Z

            if type(vec) == "vector" then
                local foo = vec
            elseif typeof(vec) == "Instance" then
                local foo = vec
            else
                local foo = vec
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    // type(vec) == "vector"
    assert_eq!(
      "Vector3",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );

    // typeof(vec) == "Instance"
    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );

    // type(vec) ~= "vector" and typeof(vec) ~= "Instance"
    assert_eq!(
      "{+ X: a, Y: b, Z: c +}",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(9, 28))
      )
    );
  }
}

mod type_infer_refinements_typeguard_cast_instance_or_vector_3_to_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1473:type_infer_refinements_typeguard_cast_instance_or_vector_3_to_vector`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_cast_instance_or_vector_3_to_vector

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_cast_instance_or_vector_3_to_vector() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Instance | Vector3)
            if typeof(x) == "Vector3" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Vector3",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Instance",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_typeguard_doesnt_leak_to_elseif {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1726:type_infer_refinements_typeguard_doesnt_leak_to_elseif`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_doesnt_leak_to_elseif

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_doesnt_leak_to_elseif() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a)
           if type(a) == "boolean" then
                local a1 = a
            elseif a.fn() then
                local a2 = a
            else
                local a3 = a
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_typeguard_in_assert_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:356:type_infer_refinements_typeguard_in_assert_position`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_in_assert_position

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_in_assert_position() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a)
            assert(type(a) == "number")
            local b = a
            return b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "<a>(a) -> a & number",
        to_string_type_id(fixture.base.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(
        "<a>(a) -> number",
        to_string_type_id(fixture.base.require_type_string(&String::from("f")))
      );
    }
  }
}

mod type_infer_refinements_typeguard_in_if_condition_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:333:type_infer_refinements_typeguard_in_if_condition_position`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method IrRegAllocX64::preserve (CodeGen/src/IrRegAllocX64.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_in_if_condition_position

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_in_if_condition_position() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(s: any, t: unknown)
            if type(s) == "number" then
                local n = s
            end
            if type(t) == "number" then
                local n = t
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "*error-type* | number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 26))
        )
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(
          fixture
            .base
            .require_type_at_position_position(Position::new(3, 26))
        )
      );
    }
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 26))
      )
    );
  }
}

mod type_infer_refinements_typeguard_narrows_for_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:866:type_infer_refinements_typeguard_narrows_for_functions`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_narrows_for_functions

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_narrows_for_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function weird(x: string | ((number) -> string))
            if type(x) == "function" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "(number) -> string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_typeguard_narrows_for_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:848:type_infer_refinements_typeguard_narrows_for_table`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_narrows_for_table

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_narrows_for_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string | {x: number} | {y: boolean})
            if type(x) == "table" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "{ x: number } | { y: boolean }",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_typeguard_not_to_be_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:830:type_infer_refinements_typeguard_not_to_be_string`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeguard_not_to_be_string

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeguard_not_to_be_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string | number | boolean)
            if type(x) ~= "string" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "boolean | number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_typeof_instance_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2516:type_infer_refinements_typeof_instance_error`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeof_instance_error

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeof_instance_error() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part)
            if typeof(x) == "Instance" then
                local foo : Folder = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_typeof_instance_isa_refinement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2529:type_infer_refinements_typeof_instance_isa_refinement`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeof_instance_isa_refinement

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeof_instance_isa_refinement() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part | Folder | string)
            if typeof(x) == "Instance" then
                local foo = x
                if foo:IsA("Folder") then
                    local bar = foo
                end
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder | Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Folder",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 32))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(8, 28))
      )
    );
  }
}

mod type_infer_refinements_typeof_instance_refinement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2498:type_infer_refinements_typeof_instance_refinement`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_typeof_instance_refinement

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeof_instance_refinement() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Instance | Vector3)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Instance",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Vector3",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

mod type_infer_refinements_typeof_refinement_context {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3016:type_infer_refinements_typeof_refinement_context`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_refinements_typeof_refinement_context

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_typeof_refinement_context() {
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

        local x = {} :: unknown

        if typeof(x) == "table" then
            if typeof(x.transform) == "function" then
            	local y = x.transform
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_refinements_unknown_lvalue_is_not_synonymous_with_other_on_not_equal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:716:type_infer_refinements_unknown_lvalue_is_not_synonymous_with_other_on_not_equal`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_unknown_lvalue_is_not_synonymous_with_other_on_not_equal

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_unknown_lvalue_is_not_synonymous_with_other_on_not_equal() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: any, b: {x: number}?)
            if a ~= b then
                local foo, bar = a, b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
    );
    assert_eq!(
      "{ x: number }?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
    );
  }
}

mod type_infer_refinements_unm_operator_is_just_a_proposition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2957:type_infer_refinements_unm_operator_is_just_a_proposition`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_refinements_unm_operator_is_just_a_proposition

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_unm_operator_is_just_a_proposition() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Pool = { x : number }
local pool = p :: Pool
if -pool then
    local y = pool
end
"#,
      ),
      None,
    );

    let ty = fixture.require_type_at_position_position(Position::new(4, 14));
    assert_ne!("never", to_string_type_id(ty));
  }
}

mod type_infer_refinements_what_nonsensical_condition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1839:type_infer_refinements_what_nonsensical_condition`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_what_nonsensical_condition

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_what_nonsensical_condition() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x)
            if type(x) == "string" and type(x) == "number" then
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

mod type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1571:type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    // CLI-117136 - this code doesn't finish constraint solving and has blocked types in the output
    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        local function f(x)
            if typeof(x) == "Instance" and x:IsA("Folder") then
                local foo = x
            elseif typeof(x) == "table" then
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Folder & Instance & {-  -}",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
      assert_eq!(
        "(~Folder | ~Instance) & {-  -} & never",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(7, 28))
        )
      );
    } else {
      assert_eq!(
        "Folder",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(5, 28))
        )
      );
      assert_eq!(
        "any",
        to_string_type_id(
          fixture
            .base
            .base
            .require_type_at_position_position(Position::new(7, 28))
        )
      );
    }
  }
}

mod type_infer_refinements_x_is_not_instance_or_else_not_part {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1708:type_infer_refinements_x_is_not_instance_or_else_not_part`
  //! Source: `tests/TypeInfer.refinements.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_refinements_x_is_not_instance_or_else_not_part

  #[cfg(test)]
  #[test]
  fn type_infer_refinements_x_is_not_instance_or_else_not_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture,
      refinement_extern_type_fixture::RefinementExternTypeFixture,
    };

    let mut fixture = RefinementExternTypeFixture {
      base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: Part | Folder | string)
            if typeof(x) ~= "Instance" or not x:IsA("Part") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Folder | string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}
