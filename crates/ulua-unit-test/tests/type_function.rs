extern crate alloc;

mod type_function_a_tf_parameterized_on_a_solved_tf_is_solved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1819:type_function_a_tf_parameterized_on_a_solved_tf_is_solved`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum TypeFunctionInstanceState (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_function_a_tf_parameterized_on_a_solved_tf_is_solved

  #[cfg(test)]
  #[test]
  fn type_function_a_tf_parameterized_on_a_solved_tf_is_solved() {
    use alloc::string::String;
    use core::ptr::NonNull;

    use ulua_analysis::{
      enums::{polarity::Polarity, type_function_instance_state::TypeFunctionInstanceState},
      functions::{
        get_type_alt_j::get_type_id, reduce_type_functions_type_function::reduce_type_functions,
      },
      records::{generic_type::GenericType, type_function_instance_type::TypeFunctionInstanceType},
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let a = fixture
      .arena
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("A"),
        Polarity::Negative,
      ));
    let b = fixture
      .arena
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("B"),
        Polarity::Negative,
      ));

    let add_func = NonNull::from(&fixture.builtin_types.type_functions.add_func);
    let inner_add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![a, b],
            alloc::vec![],
        ),
    );
    let number_type = fixture.builtin_types.number_type;
    let outer_add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![number_type, inner_add_ty],
            alloc::vec![],
        ),
    );

    let _res = reduce_type_functions(
      outer_add_ty,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    let tfit = get_type_id::<TypeFunctionInstanceType>(outer_add_ty)
      .expect("expected TypeFunctionInstanceType");
    assert_eq!(TypeFunctionInstanceState::Solved, tfit.state());
  }
}

mod type_function_a_tf_parameterized_on_a_stuck_tf_is_stuck {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1836:type_function_a_tf_parameterized_on_a_stuck_tf_is_stuck`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum TypeFunctionInstanceState (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_function_a_tf_parameterized_on_a_stuck_tf_is_stuck

  #[cfg(test)]
  #[test]
  fn type_function_a_tf_parameterized_on_a_stuck_tf_is_stuck() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      enums::type_function_instance_state::TypeFunctionInstanceState,
      functions::{
        get_type_alt_j::get_type_id, reduce_type_functions_type_function::reduce_type_functions,
      },
      records::type_function_instance_type::TypeFunctionInstanceType,
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let add_func = NonNull::from(&fixture.builtin_types.type_functions.add_func);
    let buffer_type = fixture.builtin_types.buffer_type;
    let boolean_type = fixture.builtin_types.boolean_type;
    let inner_add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![buffer_type, boolean_type],
            alloc::vec![],
        ),
    );
    let number_type = fixture.builtin_types.number_type;
    let outer_add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![number_type, inner_add_ty],
            alloc::vec![],
        ),
    );

    let _res = reduce_type_functions(
      outer_add_ty,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    let tfit = get_type_id::<TypeFunctionInstanceType>(outer_add_ty)
      .expect("expected TypeFunctionInstanceType");
    assert_eq!(TypeFunctionInstanceState::Stuck, tfit.state());
  }
}

mod type_function_a_type_function_parameterized_on_generics_is_solved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1804:type_function_a_type_function_parameterized_on_generics_is_solved`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum TypeFunctionInstanceState (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_function_a_type_function_parameterized_on_generics_is_solved

  #[cfg(test)]
  #[test]
  fn type_function_a_type_function_parameterized_on_generics_is_solved() {
    use alloc::string::String;
    use core::ptr::NonNull;

    use ulua_analysis::{
      enums::{polarity::Polarity, type_function_instance_state::TypeFunctionInstanceState},
      functions::{
        get_type_alt_j::get_type_id, reduce_type_functions_type_function::reduce_type_functions,
      },
      records::{generic_type::GenericType, type_function_instance_type::TypeFunctionInstanceType},
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let a = fixture
      .arena
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("A"),
        Polarity::Negative,
      ));
    let b = fixture
      .arena
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("B"),
        Polarity::Negative,
      ));

    let add_func = NonNull::from(&fixture.builtin_types.type_functions.add_func);
    let add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![a, b],
            alloc::vec![],
        ),
    );

    let _res = reduce_type_functions(
      add_ty,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    let tfit =
      get_type_id::<TypeFunctionInstanceType>(add_ty).expect("expected TypeFunctionInstanceType");
    assert_eq!(TypeFunctionInstanceState::Solved, tfit.state());
  }
}

mod type_function_add_function_at_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:196:type_function_add_function_at_work`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_add_function_at_work

  #[cfg(test)]
  #[test]
  fn type_function_add_function_at_work() {
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
        local function add(a, b)
            return a + b
        end

        local a = add(1, 2)
        local b = add(1, "foo")
        local c = add("foo", 1)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "add<number, string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "add<string, number>",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Operator '+' could not be applied to operands of types string and number; there is no corresponding overload for __add",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_basic_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:68:type_function_basic_type_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_basic_type_function

  #[cfg(test)]
  #[test]
  fn type_function_basic_type_function() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = Swap<number>
        type B = Swap<string>
        type C = Swap<boolean>

        local x = 123
        local y: Swap<typeof(x)> = "foo"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_alias(&String::from("A")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_alias(&String::from("B")))
    );
    assert_eq!(
      "Swap<boolean>",
      to_string_type_id(fixture.base.require_type_alias(&String::from("C")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
    assert_eq!(
      "Type function instance Swap<boolean> is uninhabited",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_binary_type_function_works_with_default_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:650:type_function_binary_type_function_works_with_default_argument`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_binary_type_function_works_with_default_argument

  #[cfg(test)]
  #[test]
  fn type_function_binary_type_function_works_with_default_argument() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type result = mul<number>

        local function thunk(): result return 5 * 4 end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "() -> number",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_string(&String::from("thunk"))
      )
    );
  }
}

mod type_function_cli_184124_recursive_restraint_violation_from_devforum {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1949:type_function_cli_184124_recursive_restraint_violation_from_devforum`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_cli_184124_recursive_restraint_violation_from_devforum

  #[cfg(test)]
  #[test]
  fn type_function_cli_184124_recursive_restraint_violation_from_devforum() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type TypeA<A... = ()> = { Func: (self: TypeA<A...>, func: (A...) -> ()) -> () }
        type TypeB<A = any> = { Value: TypeA<TypeB<A>> }
        local value = {} :: TypeB
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_cyclic_add_function_at_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:225:type_function_cyclic_add_function_at_work`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_cyclic_add_function_at_work

  #[cfg(test)]
  #[test]
  fn type_function_cyclic_add_function_at_work() {
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
        type T = add<number | T, number>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_cyclic_concat_function_at_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:758:type_function_cyclic_concat_function_at_work`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_cyclic_concat_function_at_work

  #[cfg(test)]
  #[test]
  fn type_function_cyclic_concat_function_at_work() {
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
        type T = concat<string | T, string>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_cyclic_metatable_should_not_crash_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1042:type_function_cyclic_metatable_should_not_crash_index`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_cyclic_metatable_should_not_crash_index

  #[cfg(test)]
  #[test]
  fn type_function_cyclic_metatable_should_not_crash_index() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        local mt = {}
        local t = setmetatable({}, mt)
        mt.__index = t

        function mt:__tostring()
            return t.p
        end

        type IndexFromT = index<typeof(t), "p">
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 't' does not have key 'p'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Property '\"p\"' does not exist on type 't'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_didnt_quite_exceed_distributivity_limits {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:804:type_function_didnt_quite_exceed_distributivity_limits`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_didnt_quite_exceed_distributivity_limits

  #[cfg(test)]
  #[test]
  fn type_function_didnt_quite_exceed_distributivity_limits() {
    use alloc::string::String;

    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_int::ScopedFastInt,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let _limit = ScopedFastInt::new(&DFInt::LuauTypeFamilyApplicationCartesianProductLimit, 20);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type A with
            function __mul(self, rhs: unknown): A
        end

        declare extern type B with
            function __mul(self, rhs: unknown): B
        end

        declare extern type C with
            function __mul(self, rhs: unknown): C
        end

        declare extern type D with
            function __mul(self, rhs: unknown): D
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = mul<A | B | C | D, A | B | C | D>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_ensure_equivalence_with_distributivity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:838:type_function_ensure_equivalence_with_distributivity`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_ensure_equivalence_with_distributivity

  #[cfg(test)]
  #[test]
  fn type_function_ensure_equivalence_with_distributivity() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type A with
            function __mul(self, rhs: unknown): A
        end

        declare extern type B with
            function __mul(self, rhs: unknown): B
        end

        declare extern type C with
            function __mul(self, rhs: unknown): C
        end

        declare extern type D with
            function __mul(self, rhs: unknown): D
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = mul<A | B, C | D>
        type U = mul<A, C> | mul<A, D> | mul<B, C> | mul<B, D>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "A | B",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
    assert_eq!(
      "A | A | B | B",
      to_string_type_id(fixture.base.require_type_alias(&String::from("U")))
    );
  }
}

mod type_function_error_suppression_should_work_on_type_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1656:type_function_error_suppression_should_work_on_type_functions`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
  //!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_error_suppression_should_work_on_type_functions

  #[cfg(test)]
  #[test]
  fn type_function_error_suppression_should_work_on_type_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        local Colours = {
            Red = 1,
            Blue = 2,
            Green = 3,
            Taupe = 4,
        }

        -- namespace mixup here, Colours isn't a type, it's a normal identifier
        export type Colour = keyof<Colours>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type 'Colours'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_exceeded_distributivity_limits {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:771:type_function_exceeded_distributivity_limits`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_exceeded_distributivity_limits

  #[cfg(test)]
  #[test]
  fn type_function_exceeded_distributivity_limits() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_int::ScopedFastInt,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let _limit = ScopedFastInt::new(&DFInt::LuauTypeFamilyApplicationCartesianProductLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type A with
            function __mul(self, rhs: unknown): A
        end

        declare extern type B with
            function __mul(self, rhs: unknown): B
        end

        declare extern type C with
            function __mul(self, rhs: unknown): C
        end

        declare extern type D with
            function __mul(self, rhs: unknown): D
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = mul<A | B | C | D, A | B | C | D>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::UninhabitedTypeFunction(_)
    ));
  }
}

mod type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1677:type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function

  #[cfg(test)]
  #[test]
  fn type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    };
    use ulua_unit_test::{
      functions::has_error::has_error, records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local function f()
            local a
            local b

            local c = a + b

            print(c + d)
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
      !has_error::<ConstraintSolvingIncompleteError>(&result),
      "{:?}",
      result.errors
    );
    assert_eq!(
      "() -> ()",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_function_function_as_fn_arg {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:109:type_function_function_as_fn_arg`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_function_function_as_fn_arg

  #[cfg(test)]
  #[test]
  fn type_function_function_as_fn_arg() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local swapper: <T>(Swap<T>) -> T
        local a = swapper(123)
        local b = swapper(false)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Expected this to be unreachable, but got 'number'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be unreachable, but got 'boolean'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_function_as_fn_ret {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:90:type_function_function_as_fn_ret`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_function_as_fn_ret

  #[cfg(test)]
  #[test]
  fn type_function_function_as_fn_ret() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local swapper: <T>(T) -> Swap<T>
        local a = swapper(123)
        local b = swapper("foo")
        local c = swapper(false)
    "#,
      ),
      None,
    );

    let a_ty = to_string_type_id(fixture.base.require_type_string(&String::from("a")));
    let b_ty = to_string_type_id(fixture.base.require_type_string(&String::from("b")));
    let c_ty = to_string_type_id(fixture.base.require_type_string(&String::from("c")));

    assert_eq!(
      1,
      result.errors.len(),
      "a={a_ty}, b={b_ty}, c={c_ty}, errors={:?}",
      result.errors
    );
    assert_eq!("string", a_ty);
    assert_eq!("number", b_ty);
    assert_eq!("Swap<boolean>", c_ty);
    assert_eq!(
      "Type function instance Swap<boolean> is uninhabited",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_function_internal_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:175:type_function_function_internal_functions`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_function_internal_functions

  #[cfg(test)]
  #[test]
  fn type_function_function_internal_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f0: <T>(T) -> (() -> T)
        local f: <T>(T) -> (() -> Swap<T>)
        local a = f(1)
        local b = f("a")
        local c = f(true)
        local d = f0(1)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> string",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "() -> number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "() -> Swap<boolean>",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "Type function instance Swap<boolean> is uninhabited",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_fuzz_len_type_function_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1403:type_function_fuzz_len_type_function_follow`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - translates_to -> rust_item type_function_fuzz_len_type_function_follow

  #[cfg(test)]
  #[test]
  fn type_function_fuzz_len_type_function_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _ = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _
        _ = true
        for l0=_,_,# _ do
        end
        for l0=_,_ do
        if _ then
        _ += _
        end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:743:type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - translates_to -> rust_item type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type

  #[cfg(test)]
  #[test]
  fn type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type() {
    use alloc::string::String;

    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    let mut fixture = TypeFunctionFixture::new();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
Module 'l0':
local _ = (67108864)(_ >= _).insert
do end
do end
_(...,_(_,_(_()),_()))
(67108864)()()
_(_ ~= _ // _,l0)(_(_({n0,})),_(_),_)
_(setmetatable(_,{[...]=_,}))

"#,
      ),
      None,
    );
  }
}

mod type_function_getmetatable_respects_metatable_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1594:type_function_getmetatable_respects_metatable_metamethod`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_getmetatable_respects_metatable_metamethod

  #[cfg(test)]
  #[test]
  fn type_function_getmetatable_respects_metatable_metamethod() {
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
        local metatable = { __metatable = "Test" }
        local obj = setmetatable({x = 1, y = 2, z = 3}, metatable)
        type Metatable = getmetatable<typeof(obj)>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_alias(&String::from("Metatable")))
    );
  }
}

mod type_function_getmetatable_returns_correct_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1532:type_function_getmetatable_returns_correct_metatable`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable

  #[cfg(test)]
  #[test]
  fn type_function_getmetatable_returns_correct_metatable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      records::to_string_options::ToStringOptions,
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
        local metatable = { __index = { w = 4 } }
        local obj = setmetatable({x = 1, y = 2, z = 3}, metatable)
        type Metatable = getmetatable<typeof(obj)>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "{ __index: { w: number } }",
      to_string_type_id_to_string_options_mut(
        fixture.base.require_type_alias(&String::from("Metatable")),
        ToStringOptions::new(true)
      )
    );
  }
}

mod type_function_getmetatable_returns_correct_metatable_for_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1573:type_function_getmetatable_returns_correct_metatable_for_string`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable_for_string

  #[cfg(test)]
  #[test]
  fn type_function_getmetatable_returns_correct_metatable_for_string() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_metatable_type::get_metatable_type_id_not_null_builtin_types,
        to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      },
      records::to_string_options::ToStringOptions,
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
        type Metatable = getmetatable<string>
        type Metatable2 = getmetatable<"foo">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let builtins = unsafe { &*fixture.base.builtin_types };
    let expected = get_metatable_type_id_not_null_builtin_types(builtins.string_type(), builtins)
      .expect("expected string metatable");
    let expected = to_string_type_id_to_string_options_mut(expected, ToStringOptions::new(true));

    assert_eq!(
      expected,
      to_string_type_id_to_string_options_mut(
        fixture.base.require_type_alias(&String::from("Metatable")),
        ToStringOptions::new(true)
      )
    );
    assert_eq!(
      expected,
      to_string_type_id_to_string_options_mut(
        fixture.base.require_type_alias(&String::from("Metatable2")),
        ToStringOptions::new(true)
      )
    );
  }
}

mod type_function_getmetatable_returns_correct_metatable_for_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1548:type_function_getmetatable_returns_correct_metatable_for_union`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable_for_union

  #[cfg(test)]
  #[test]
  fn type_function_getmetatable_returns_correct_metatable_for_union() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_metatable_type::get_metatable_type_id_not_null_builtin_types,
        to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      },
      records::{
        intersection_type::IntersectionType, to_string_options::ToStringOptions,
        type_arena::TypeArena, union_type::UnionType,
      },
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
        type Identity = setmetatable<{}, {}>
        type Metatable = getmetatable<string | Identity>
        type IntersectMetatable = getmetatable<string & Identity>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let builtins = unsafe { &*fixture.base.builtin_types };
    let string_metatable =
      get_metatable_type_id_not_null_builtin_types(builtins.string_type(), builtins)
        .expect("expected string metatable");
    let mut arena = TypeArena::default();

    let expected_union = arena.add_type(UnionType {
      options: alloc::vec![string_metatable, builtins.empty_table_type()],
    });
    assert_eq!(
      to_string_type_id_to_string_options_mut(expected_union, ToStringOptions::new(true)),
      to_string_type_id_to_string_options_mut(
        fixture.base.require_type_alias(&String::from("Metatable")),
        ToStringOptions::new(true)
      )
    );

    let expected_intersection = arena.add_type(IntersectionType {
      parts: alloc::vec![string_metatable, builtins.empty_table_type()],
    });
    assert_eq!(
      to_string_type_id_to_string_options_mut(expected_intersection, ToStringOptions::new(true)),
      to_string_type_id_to_string_options_mut(
        fixture
          .base
          .require_type_alias(&String::from("IntersectMetatable")),
        ToStringOptions::new(true)
      )
    );
  }
}

mod type_function_getmetatable_type_function_returns_nil_if_no_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1505:type_function_getmetatable_type_function_returns_nil_if_no_metatable`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_getmetatable_type_function_returns_nil_if_no_metatable

  #[cfg(test)]
  #[test]
  fn type_function_getmetatable_type_function_returns_nil_if_no_metatable() {
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
        type TableWithNoMetatable = getmetatable<{}>
        type NumberWithNoMetatable = getmetatable<number>
        type BooleanWithNoMetatable = getmetatable<boolean>
        type BooleanLiteralWithNoMetatable = getmetatable<true>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("TableWithNoMetatable"))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("NumberWithNoMetatable"))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("BooleanWithNoMetatable"))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("BooleanLiteralWithNoMetatable"))
      )
    );
  }
}

mod type_function_has_prop_on_irreducible_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1639:type_function_has_prop_on_irreducible_type_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_has_prop_on_irreducible_type_function

  #[cfg(test)]
  #[test]
  fn type_function_has_prop_on_irreducible_type_function() {
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
local test = "a" + "b"
print(test.a)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Operator '+' could not be applied to operands of types string and string; there is no corresponding overload for __add",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'add<string, string>' does not have key 'a'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_index_of_any_is_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:901:type_function_index_of_any_is_any`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_index_of_any_is_any

  #[cfg(test)]
  #[test]
  fn type_function_index_of_any_is_any() {
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
        type T = index<any, "a">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_index_should_not_crash_on_cyclic_stuff {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:914:type_function_index_should_not_crash_on_cyclic_stuff`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_index_should_not_crash_on_cyclic_stuff

  #[cfg(test)]
  #[test]
  fn type_function_index_should_not_crash_on_cyclic_stuff() {
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
        local PlayerData = {}

        type Keys = index<typeof(PlayerData), true>

        local function UpdateData(key: Keys)
            PlayerData[key] = 4
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "index<PlayerData, true>",
      to_string_type_id(fixture.base.require_type_alias(&String::from("Keys")))
    );
  }
}

mod type_function_index_should_not_crash_on_cyclic_stuff_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:933:type_function_index_should_not_crash_on_cyclic_stuff_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_index_should_not_crash_on_cyclic_stuff_2

  #[cfg(test)]
  #[test]
  fn type_function_index_should_not_crash_on_cyclic_stuff_2() {
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
        local PlayerData = {}

        type Keys = index<typeof(PlayerData), number>

        local function UpdateData(key: Keys)
            PlayerData[key] = 4
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_alias(&String::from("Keys")))
    );
  }
}

mod type_function_index_should_not_crash_on_cyclic_stuff_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:954:type_function_index_should_not_crash_on_cyclic_stuff_3`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_index_should_not_crash_on_cyclic_stuff_3

  #[cfg(test)]
  #[test]
  fn type_function_index_should_not_crash_on_cyclic_stuff_3() {
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
        local PlayerData = {
            Coins = 0,
            Level = 1,
            Exp = 0,
            MapExp = 100,
        }

        type Keys = index<typeof(PlayerData), true>

        local function UpdateData(key: Keys, value)
            PlayerData[key] = value
        end

        UpdateData("Coins", 2)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_alias(&String::from("Keys")))
    );
  }
}

mod type_function_index_type_function_errors_w_bad_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1087:type_function_index_type_function_errors_w_bad_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_errors_w_bad_indexer

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_errors_w_bad_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string, b: number, c: boolean}
        type errType1 = index<MyObject, "d">
        type errType2 = index<MyObject, boolean>
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"d\"' does not exist on type 'MyObject'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Property 'boolean' does not exist on type 'MyObject'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_index_type_function_errors_w_var_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1146:type_function_index_type_function_errors_w_var_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_errors_w_var_indexer

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_errors_w_var_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string, b: number, c: boolean}
        local key = "a"

        type errType1 = index<MyObject, key>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type 'key'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_index_type_function_rfc_alternative_section {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1199:type_function_index_type_function_rfc_alternative_section`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_rfc_alternative_section

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_rfc_alternative_section() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string}
        type MyObject2 = {a: string, b: number}

        local function edgeCase(param: MyObject)
            type unknown_type = index<typeof(param), "b">
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"b\"' does not exist on type 'MyObject'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_index_type_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:981:type_function_index_type_function_works`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_index_type_function_works

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = {a: string, b: number, c: boolean}
        type IdxAType = index<MyObject, "a">
        type IdxBType = index<MyObject, keyof<MyObject>>

        local function ok(idx: IdxAType): string return idx end
        local function ok2(idx: IdxBType): string | number | boolean return idx end
        local function err(idx: IdxAType): boolean return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("boolean", to_string_type_id(tm.wanted_type));
        assert_eq!("string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_index_type_function_works_on_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1217:type_function_index_type_function_works_on_extern_types`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_on_extern_types

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_on_extern_types() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type KeysOfMyObject = index<BaseClass, "BaseField">

        local function ok(idx: KeysOfMyObject): number return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_index_type_function_works_on_extern_types_with_parents {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1231:type_function_index_type_function_works_on_extern_types_with_parents`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_on_extern_types_with_parents

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_on_extern_types_with_parents() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type KeysOfMyObject = index<ChildClass, "BaseField">

        local function ok(idx: KeysOfMyObject): number return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_index_type_function_works_on_function_metamethods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1103:type_function_index_type_function_works_on_function_metamethods`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_on_function_metamethods

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_on_function_metamethods() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      records::to_string_options::ToStringOptions,
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
        type Foo = {x: string}
        local t = {}
        setmetatable(t, {
            __index = function(x: string): Foo
                return {x = x}
            end
        })

        type Bar = index<typeof(t), "bar">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "{ x: string }",
      to_string_type_id_to_string_options_mut(
        fixture.base.require_type_alias(&String::from("Bar")),
        ToStringOptions::new(true)
      )
    );
  }
}

mod type_function_index_type_function_works_on_function_metamethods_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1126:type_function_index_type_function_works_on_function_metamethods_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_on_function_metamethods_2

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_on_function_metamethods_2() {
    use alloc::string::String;

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
        type Foo = {x: string}
        local t = {}
        setmetatable(t, {
            __index = function(x: string): Foo
                return {x = x}
            end
        })

        type Bar = index<typeof(t), number>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_function_index_type_function_works_w_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1027:type_function_index_type_function_works_w_array`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_w_array

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_w_array() {
    use alloc::string::String;

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
        local MyObject = {"hello", 1, true}
        type IdxAType = index<typeof(MyObject), number>

        local function ok(idx: IdxAType): string | number | boolean return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_index_type_function_works_w_generic_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1065:type_function_index_type_function_works_w_generic_types`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_w_generic_types

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_w_generic_types() {
    use alloc::string::String;

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
        local function access<T, K>(tbl: T & {}, key: K): index<T, K>
            return tbl[key]
        end

        local subjects = {
            english = "boring",
            math = "fun"
        }

        local key: "english" = "english"
        local a: string = access(subjects, key)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_index_type_function_works_w_index_metatables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1245:type_function_index_type_function_works_w_index_metatables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_w_index_metatables

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_w_index_metatables() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        local exampleClass = { Foo = "text", Bar = true }

        local exampleClass2 = setmetatable({ Foo = 8 }, { __index = exampleClass })
        type exampleTy2 = index<typeof(exampleClass2), "Foo">
        local function ok(idx: exampleTy2): number return idx end

        local exampleClass3 = setmetatable({ Bar = 5 }, { __index = exampleClass })
        type exampleTy3 = index<typeof(exampleClass3), "Foo">
        local function ok2(idx: exampleTy3): string return idx end

        type exampleTy4 = index<typeof(exampleClass3), "Foo" | "Bar">
        local function ok3(idx: exampleTy4): string | number return idx end

        type errTy = index<typeof(exampleClass2), "Car">
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"Car\"' does not exist on type 'exampleClass2'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_index_type_function_works_w_union_type_indexee {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1180:type_function_index_type_function_works_w_union_type_indexee`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_w_union_type_indexee

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_w_union_type_indexee() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string, b: number, c: boolean}
        type MyObject2 = {a: number}

        type idxTypeA = index<MyObject | MyObject2, "a">
        local function ok(idx: idxTypeA): string | number return idx end

        type errType = index<MyObject | MyObject2, "b">
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"b\"' does not exist on type 'MyObject | MyObject2'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_index_type_function_works_w_union_type_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1162:type_function_index_type_function_works_w_union_type_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_index_type_function_works_w_union_type_indexer

  #[cfg(test)]
  #[test]
  fn type_function_index_type_function_works_w_union_type_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string, b: number, c: boolean}

        type idxType = index<MyObject, "a" | "b">
        local function ok(idx: idxType): string | number return idx end

        type errType = index<MyObject, "a" | "d">
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"a\" | \"d\"' does not exist on type 'MyObject'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_index_wait_for_pending_no_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1004:type_function_index_wait_for_pending_no_crash`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_index_wait_for_pending_no_crash

  #[cfg(test)]
  #[test]
  fn type_function_index_wait_for_pending_no_crash() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local PlayerData = {
            Coins = 0,
            Level = 1,
            Exp = 0,
            MaxExp = 100
        }
        type Keys = index<typeof(PlayerData), keyof<typeof(PlayerData)>>
        -- This function makes it think that there's going to be a pending expansion
        local function UpdateData(key: Keys, value)
            PlayerData[key] = value
        end
        UpdateData("Coins", 2)
    "#,
      ),
      None,
    );
  }
}

mod type_function_internal_functions_raise_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:281:type_function_internal_functions_raise_errors`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_function_internal_functions_raise_errors

  #[cfg(test)]
  #[test]
  fn type_function_internal_functions_raise_errors() {
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
        local function innerSum(a, b)
            local _ = a + b
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Operator '+' could not be applied to operands of types unknown and unknown; there is no corresponding overload for __add",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_keyof_oss_crash_gh_1161 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:719:type_function_keyof_oss_crash_gh_1161`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_keyof_oss_crash_gh_1161

  #[cfg(test)]
  #[test]
  fn type_function_keyof_oss_crash_gh_1161() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        local EnumVariants = {
            ["a"] = 1, ["b"] = 2, ["c"] = 3
        }

        type EnumKey = keyof<typeof(EnumVariants)>

        function fnA<T>(i: T): keyof<T> end

        function fnB(i: EnumKey) end

        local result = fnA(EnumVariants)
        fnB(result)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      matches!(
        &result.errors[0].data,
        TypeErrorData::FunctionExitsWithoutReturning(_)
      ),
      "{:?}",
      result.errors[0]
    );
  }
}

mod type_function_keyof_rfc_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:688:type_function_keyof_rfc_example`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_rfc_example

  #[cfg(test)]
  #[test]
  fn type_function_keyof_rfc_example() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        local animals = {
            cat = { speak = function() print "meow" end },
            dog = { speak = function() print "woof woof" end },
            monkey = { speak = function() print "oo oo" end },
            fox = { speak = function() print "gekk gekk" end }
        }

        type AnimalType = keyof<typeof(animals)>

        function speakByType(animal: AnimalType)
            animals[animal].speak()
        end

        speakByType("dog") -- ok
        speakByType("cactus") -- errors
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "\"cat\" | \"dog\" | \"fox\" | \"monkey\"",
          to_string_type_id(tm.wanted_type)
        );
        assert_eq!("\"cactus\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_should_not_assert_on_empty_string_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1723:type_function_keyof_should_not_assert_on_empty_string_props`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_keyof_should_not_assert_on_empty_string_props

  #[cfg(test)]
  #[test]
  fn type_function_keyof_should_not_assert_on_empty_string_props() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            one: boolean
            [""]: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type FoobarKeys = keyof<Foobar>;
        export type TableKeys = keyof<{ [""]: string, two: boolean }>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "\"\" | \"one\"",
      to_string_type_id(fixture.base.require_type_alias(&String::from("FoobarKeys")))
    );
    assert_eq!(
      "\"\" | \"two\"",
      to_string_type_id(fixture.base.require_type_alias(&String::from("TableKeys")))
    );
  }
}

mod type_function_keyof_single_entry_no_uniontype {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:386:type_function_keyof_single_entry_no_uniontype`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_keyof_single_entry_no_uniontype

  #[cfg(test)]
  #[test]
  fn type_function_keyof_single_entry_no_uniontype() {
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
        local tbl_A = { abc = "value" }
        local tbl_B = { a1 = nil, ["a2"] = nil }

        type keyof_A = keyof<typeof(tbl_A)>
        type keyof_B = keyof<typeof(tbl_B)>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "\"abc\"",
      to_string_type_id(fixture.base.require_type_alias(&String::from("keyof_A")))
    );
    assert_eq!(
      "\"a1\" | \"a2\"",
      to_string_type_id(fixture.base.require_type_alias(&String::from("keyof_B")))
    );
  }
}

mod type_function_keyof_type_function_common_subset_if_union_of_differing_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:622:type_function_keyof_type_function_common_subset_if_union_of_differing_extern_types`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_keyof_type_function_common_subset_if_union_of_differing_extern_types

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_common_subset_if_union_of_differing_extern_types() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type KeysOfMyObject = keyof<BaseClass | Vector2>

        local function ok(idx: KeysOfMyObject): never return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_keyof_type_function_common_subset_if_union_of_differing_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:451:type_function_keyof_type_function_common_subset_if_union_of_differing_tables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_type_function_common_subset_if_union_of_differing_tables

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_common_subset_if_union_of_differing_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = { x: number, y: number, z: number }
        type MyOtherObject = { w: number, y: number, z: number }
        type KeysOfMyObject = keyof<MyObject | MyOtherObject>

        local function err(idx: KeysOfMyObject): "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"z\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_type_function_errors_if_it_has_nonclass_part {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:605:type_function_keyof_type_function_errors_if_it_has_nonclass_part`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_function_keyof_type_function_errors_if_it_has_nonclass_part

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_errors_if_it_has_nonclass_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type KeysOfMyObject = keyof<BaseClass | boolean>

        local function err(idx: KeysOfMyObject): "BaseMethod" | "BaseField" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'BaseClass | boolean' does not have keys, so 'keyof<BaseClass | boolean>' is invalid",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'BaseClass | boolean' does not have keys, so 'keyof<BaseClass | boolean>' is invalid",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_keyof_type_function_errors_if_it_has_nontable_part {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:405:type_function_keyof_type_function_errors_if_it_has_nontable_part`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_function_keyof_type_function_errors_if_it_has_nontable_part

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_errors_if_it_has_nontable_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = { x: number, y: number, z: number }
        type KeysOfMyObject = keyof<MyObject | boolean>

        local function err(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_keyof_type_function_never_for_empty_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:472:type_function_keyof_type_function_never_for_empty_table`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_keyof_type_function_never_for_empty_table

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_never_for_empty_table() {
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
        type KeyofEmpty = keyof<{}>

        local foo = ((nil :: any) :: KeyofEmpty)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_function_keyof_type_function_string_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:423:type_function_keyof_type_function_string_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_type_function_string_indexer

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_string_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = { x: number, y: number, z: number }
        type MyOtherObject = { [string]: number }
        type KeysOfMyOtherObject = keyof<MyOtherObject>
        type KeysOfMyObjects = keyof<MyObject | MyOtherObject>

        local function ok(idx: KeysOfMyOtherObject): "z" return idx end
        local function err(idx: KeysOfMyObjects): "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"z\"", to_string_type_id(tm.wanted_type));
        assert_eq!("string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }

    match &result.errors[1].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"z\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"x\" | \"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_type_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:342:type_function_keyof_type_function_works`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_type_function_works

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = { x: number, y: number, z: number }
        type KeysOfMyObject = keyof<MyObject>

        local function ok(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
        local function err(idx: KeysOfMyObject): "x" | "y" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"x\" | \"y\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"x\" | \"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_type_function_works_on_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:585:type_function_keyof_type_function_works_on_extern_types`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_type_function_works_on_extern_types

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_works_on_extern_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type KeysOfMyObject = keyof<BaseClass>

        local function ok(idx: KeysOfMyObject): "BaseMethod" | "BaseField" | "Touched" return idx end
        local function err(idx: KeysOfMyObject): "BaseMethod" return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"BaseMethod\"", to_string_type_id(tm.wanted_type));
        assert_eq!(
          "\"BaseField\" | \"BaseMethod\" | \"Touched\"",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_type_function_works_with_metatables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:363:type_function_keyof_type_function_works_with_metatables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_keyof_type_function_works_with_metatables

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_works_with_metatables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        local metatable = { __index = {w = 1} }
        local obj = setmetatable({x = 1, y = 2, z = 3}, metatable)
        type MyObject = typeof(obj)
        type KeysOfMyObject = keyof<MyObject>

        local function ok(idx: KeysOfMyObject): "w" | "x" | "y" | "z" return idx end
        local function err(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"x\" | \"y\" | \"z\"", to_string_type_id(tm.wanted_type));
        assert_eq!(
          "\"w\" | \"x\" | \"y\" | \"z\"",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_keyof_type_function_works_with_parent_extern_types_too {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:636:type_function_keyof_type_function_works_with_parent_extern_types_too`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_keyof_type_function_works_with_parent_extern_types_too

  #[cfg(test)]
  #[test]
  fn type_function_keyof_type_function_works_with_parent_extern_types_too() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type KeysOfMyObject = keyof<ChildClass>

        local function ok(idx: KeysOfMyObject): "BaseField" | "BaseMethod" | "Method" | "Touched" return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_len_typefun_on_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1622:type_function_len_typefun_on_metatable`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_function_len_typefun_on_metatable

  #[cfg(test)]
  #[test]
  fn type_function_len_typefun_on_metatable() {
    use alloc::string::String;

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
local t = setmetatable({}, { __mode = "v" })

local function f()
    table.insert(t, {})
    print(#t * 100)
end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_mul_function_with_union_of_multiplicatives {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:238:type_function_mul_function_with_union_of_multiplicatives`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_mul_function_with_union_of_multiplicatives

  #[cfg(test)]
  #[test]
  fn type_function_mul_function_with_union_of_multiplicatives() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Vec2 with
            function __mul(self, rhs: number): Vec2
        end

        declare extern type Vec3 with
            function __mul(self, rhs: number): Vec3
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = mul<Vec2 | Vec3, number>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Vec2 | Vec3",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_mul_function_with_union_of_multiplicatives_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:261:type_function_mul_function_with_union_of_multiplicatives_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_mul_function_with_union_of_multiplicatives_2

  #[cfg(test)]
  #[test]
  fn type_function_mul_function_with_union_of_multiplicatives_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Vec3 with
            function __mul(self, rhs: number): Vec3
            function __mul(self, rhs: Vec3): Vec3
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = mul<number | Vec3, Vec3>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Vec3",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_or_a_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1792:type_function_or_a_b`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_function_or_a_b

  #[cfg(test)]
  #[test]
  fn type_function_or_a_b() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      functions::reduce_type_functions_type_function::reduce_type_functions,
      records::type_function_instance_type::TypeFunctionInstanceType,
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let scope = fixture.tfc.scope.as_ptr();
    let a_type = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, scope);
    let b_type = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, scope);

    let or_func = NonNull::from(&fixture.builtin_types.type_functions.or_func);
    let or_type = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            or_func,
            alloc::vec![a_type, b_type],
            alloc::vec![],
        ),
    );

    let res = reduce_type_functions(
      or_type,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    assert_eq!(1, res.reduced_types.size());
  }
}

mod type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1959:type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_1`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_1

  #[cfg(test)]
  #[test]
  fn type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        local MyClass = {}
        local MyClassMetatable = table.freeze({  __index = MyClass })

        type MyClass = setmetatable<{ name: string }, typeof(MyClassMetatable)>

        function MyClass.new(name: string): MyClass
            return setmetatable({ name = name }, MyClassMetatable)
        end

        function MyClass.hello(self: MyClass): string
            return `Hello, {self.name}!`
        end

        local instance = MyClass.new("World")
        local g = instance:hello()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("g")))
    );
  }
}

mod type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1984:type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2

  #[cfg(test)]
  #[test]
  fn type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        local MyClass = {}
        local MyClassMetatable = { __index = MyClass }
        table.freeze(MyClassMetatable)

        type CommonFields<T> = { read name: T }
        type MyClass = setmetatable<CommonFields<string>, typeof(MyClassMetatable)>

        function MyClass.new(name: string): MyClass
            return setmetatable({ name = name }, MyClassMetatable)
        end

        function MyClass.hello(self: MyClass): string
            return `Hello, {self.name}!`
        end

        local instance = MyClass.new("World")
        local g = instance:hello()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("g")))
    );
  }
}

mod type_function_oss_2114_type_instantiation_on_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:2011:type_function_oss_2114_type_instantiation_on_type_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_oss_2114_type_instantiation_on_type_function

  #[cfg(test)]
  #[test]
  fn type_function_oss_2114_type_instantiation_on_type_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _type_instantiation =
      ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type function id(t: type): type
            return t
        end

        local function fn<T>(): id<T>
            return nil :: any
        end

        local y = fn<<number>>()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_function_oss_2144_type_instantiation_on_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:2035:type_function_oss_2144_type_instantiation_on_type_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_oss_2144_type_instantiation_on_type_function

  #[cfg(test)]
  #[test]
  fn type_function_oss_2144_type_instantiation_on_type_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _type_instantiation =
      ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type ST = {
            Member1: number,
            Name: string
        }

        local function access<T>(t, field: T): index<ST, T>
            return t[field]
        end

        local t: any = {}
        local _b = access<<"Member1">>(t, "Member1")
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("_b")))
    );
  }
}

mod type_function_rawget_type_function_errors_w_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1376:type_function_rawget_type_function_errors_w_extern_types`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_rawget_type_function_errors_w_extern_types

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_errors_w_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type PropsOfMyObject = rawget<BaseClass, "BaseField">
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Property '\"BaseField\"' does not exist on type 'BaseClass'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_rawget_type_function_errors_w_var_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1307:type_function_rawget_type_function_errors_w_var_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_errors_w_var_indexer

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_errors_w_var_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = {a: string, b: number, c: boolean}
        local key = "a"
        type errType1 = rawget<MyObject, key>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type 'key'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_rawget_type_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1271:type_function_rawget_type_function_works`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = {a: string, b: number, c: boolean}
        type RawAType = rawget<MyObject, "a">
        type RawBType = rawget<MyObject, keyof<MyObject>>
        local function ok(idx: RawAType): string return idx end
        local function ok2(idx: RawBType): string | number | boolean return idx end
        local function err(idx: RawAType): boolean return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("boolean", to_string_type_id(tm.wanted_type));
        assert_eq!("string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_rawget_type_function_works_w_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1293:type_function_rawget_type_function_works_w_array`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works_w_array

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works_w_array() {
    use alloc::string::String;

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
        local MyObject = {"hello", 1, true}
        type RawAType = rawget<typeof(MyObject), number>
        local function ok(idx: RawAType): string | number | boolean return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_rawget_type_function_works_w_index_metatables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1356:type_function_rawget_type_function_works_w_index_metatables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works_w_index_metatables

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works_w_index_metatables() {
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
        local exampleClass = { Foo = "text", Bar = true }
        local exampleClass2 = setmetatable({ Foo = 8 }, { __index = exampleClass })
        type exampleTy2 = rawget<typeof(exampleClass2), "Foo">
        local function ok(idx: exampleTy2): number return idx end
        local exampleClass3 = setmetatable({ Bar = 5 }, { __index = exampleClass })
        type nil_type = rawget<typeof(exampleClass3), "Foo">
        type number_type = rawget<typeof(exampleClass3), "Bar" | "Foo">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_alias(&String::from("nil_type")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("number_type"))
      )
    );
  }
}

mod type_function_rawget_type_function_works_w_queried_key_absent {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1389:type_function_rawget_type_function_works_w_queried_key_absent`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works_w_queried_key_absent

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works_w_queried_key_absent() {
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
        type MyObject = {a: string}
        type T = rawget<MyObject, "b">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
  }
}

mod type_function_rawget_type_function_works_w_union_type_indexee {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1339:type_function_rawget_type_function_works_w_union_type_indexee`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works_w_union_type_indexee

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works_w_union_type_indexee() {
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
        type MyObject = {a: string, b: number, c: boolean}
        type MyObject2 = {a: number}
        type rawTypeA = rawget<MyObject | MyObject2, "a">
        local function ok(idx: rawTypeA): string | number return idx end
        type number_type = rawget<MyObject | MyObject2, "b">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("number_type"))
      )
    );
  }
}

mod type_function_rawget_type_function_works_w_union_type_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1323:type_function_rawget_type_function_works_w_union_type_indexer`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_function_rawget_type_function_works_w_union_type_indexer

  #[cfg(test)]
  #[test]
  fn type_function_rawget_type_function_works_w_union_type_indexer() {
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
        type MyObject = {a: string, b: number, c: boolean}
        type rawType = rawget<MyObject, "a" | "b">
        local function ok(idx: rawType): string | number return idx end
        type string_type = rawget<MyObject, "a" | "d">
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(
        fixture
          .base
          .require_type_alias(&String::from("string_type"))
      )
    );
  }
}

mod type_function_rawkeyof_type_function_common_subset_if_union_of_differing_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:549:type_function_rawkeyof_type_function_common_subset_if_union_of_differing_tables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_rawkeyof_type_function_common_subset_if_union_of_differing_tables

  #[cfg(test)]
  #[test]
  fn type_function_rawkeyof_type_function_common_subset_if_union_of_differing_tables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = { x: number, y: number, z: number }
        type MyOtherObject = { w: number, y: number, z: number }
        type KeysOfMyObject = rawkeyof<MyObject | MyOtherObject>

        local function err(idx: KeysOfMyObject): "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"z\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_rawkeyof_type_function_errors_if_it_has_nontable_part {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:531:type_function_rawkeyof_type_function_errors_if_it_has_nontable_part`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_function_rawkeyof_type_function_errors_if_it_has_nontable_part

  #[cfg(test)]
  #[test]
  fn type_function_rawkeyof_type_function_errors_if_it_has_nontable_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type MyObject = { x: number, y: number, z: number }
        type KeysOfMyObject = rawkeyof<MyObject | boolean>

        local function err(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type 'MyObject | boolean' does not have keys, so 'rawkeyof<MyObject | boolean>' is invalid",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'MyObject | boolean' does not have keys, so 'rawkeyof<MyObject | boolean>' is invalid",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_rawkeyof_type_function_ignores_metatables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:508:type_function_rawkeyof_type_function_ignores_metatables`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_rawkeyof_type_function_ignores_metatables

  #[cfg(test)]
  #[test]
  fn type_function_rawkeyof_type_function_ignores_metatables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        local metatable = { __index = {w = 1} }
        local obj = setmetatable({x = 1, y = 2, z = 3}, metatable)
        type MyObject = typeof(obj)
        type KeysOfMyObject = rawkeyof<MyObject>

        local function ok(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
        local function err(idx: KeysOfMyObject): "x" | "y" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"x\" | \"y\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"x\" | \"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_rawkeyof_type_function_never_for_empty_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:570:type_function_rawkeyof_type_function_never_for_empty_table`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_rawkeyof_type_function_never_for_empty_table

  #[cfg(test)]
  #[test]
  fn type_function_rawkeyof_type_function_never_for_empty_table() {
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
        type RawkeyofEmpty = rawkeyof<{}>

        local foo = ((nil :: any) :: RawkeyofEmpty)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_function_rawkeyof_type_function_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:487:type_function_rawkeyof_type_function_works`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_rawkeyof_type_function_works

  #[cfg(test)]
  #[test]
  fn type_function_rawkeyof_type_function_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type MyObject = { x: number, y: number, z: number }
        type KeysOfMyObject = rawkeyof<MyObject>

        local function ok(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
        local function err(idx: KeysOfMyObject): "x" | "y" return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"x\" | \"y\"", to_string_type_id(tm.wanted_type));
        assert_eq!("\"x\" | \"y\" | \"z\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_recursive_restraint_violation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1886:type_function_recursive_restraint_violation`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record RecursiveRestraintViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation() {
    use alloc::string::String;

    use ulua_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::type_function_fixture::TypeFunctionFixture,
    };

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type a<T> = {a<{T}>}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0])
      .expect("expected RecursiveRestraintViolation");
  }
}

mod type_function_recursive_restraint_violation_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1896:type_function_recursive_restraint_violation_1`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record RecursiveRestraintViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation_1

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation_1() {
    use alloc::string::String;

    use ulua_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::type_function_fixture::TypeFunctionFixture,
    };

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type b<T> = {b<T | string>}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0])
      .expect("expected RecursiveRestraintViolation");
  }
}

mod type_function_recursive_restraint_violation_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1906:type_function_recursive_restraint_violation_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record RecursiveRestraintViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation_2

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation_2() {
    use alloc::string::String;

    use ulua_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::type_function_fixture::TypeFunctionFixture,
    };

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type c<T> = {c<T & string>}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0])
      .expect("expected RecursiveRestraintViolation");
  }
}

mod type_function_recursive_restraint_violation_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1916:type_function_recursive_restraint_violation_3`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record RecursiveRestraintViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation_3

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation_3() {
    use alloc::string::String;

    use ulua_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::type_function_fixture::TypeFunctionFixture,
    };

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type d<T> = (d<T | string>) -> ()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0])
      .expect("expected RecursiveRestraintViolation");
  }
}

mod type_function_recursive_restraint_violation_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1926:type_function_recursive_restraint_violation_4`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation_4

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation_4() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { B<any> }

        type B<T> = { method: (B<T>) -> () }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_recursive_restraint_violation_with_defaults {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1935:type_function_recursive_restraint_violation_with_defaults`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record RecursiveRestraintViolation (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_function_recursive_restraint_violation_with_defaults

  #[cfg(test)]
  #[test]
  fn type_function_recursive_restraint_violation_with_defaults() {
    use alloc::string::String;

    use ulua_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type B<T = number> = { B<number> }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some(),
      "{:?}",
      result.errors[0]
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 31,
        },
        end: Position {
          line: 1,
          column: 40,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_function_reduce_cyclic_add {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:2061:type_function_reduce_cyclic_add`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
  //!   - calls -> function emplaceType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_function_reduce_cyclic_add

  #[cfg(test)]
  #[test]
  fn type_function_reduce_cyclic_add() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      functions::{
        as_mutable_type::as_mutable_type_id,
        reduce_type_functions_type_function::reduce_type_functions,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{
        blocked_type::BlockedType, type_function_instance_type::TypeFunctionInstanceType,
        union_type::UnionType,
      },
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let root = fixture.arena.add_type(BlockedType::default());
    let number_type = fixture.builtin_types.number_type;
    let lhs = fixture.arena.add_type(UnionType {
      options: alloc::vec![number_type, root],
    });
    let rhs = fixture.arena.add_type(UnionType {
      options: alloc::vec![number_type, root],
    });
    let add_func = NonNull::from(&fixture.builtin_types.type_functions.add_func);
    let add_tfit = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![lhs, rhs],
            alloc::vec![],
        ),
    );
    unsafe {
      (*as_mutable_type_id(root)).ty = TypeVariant::Bound(add_tfit);
    }

    let res = reduce_type_functions(
      root,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    assert_eq!("number", to_string_type_id(root));
    assert_eq!(3, res.reduced_types.size());
    assert_eq!(0, res.errors.len(), "{:?}", res.errors);
    assert_eq!(0, res.irreducible_types.size());
    assert_eq!(0, res.blocked_types.size());
  }
}

mod type_function_reduce_degenerate_refinement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1852:type_function_reduce_degenerate_refinement`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - calls -> function emplaceType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_function_reduce_degenerate_refinement

  #[cfg(test)]
  #[test]
  fn type_function_reduce_degenerate_refinement() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      functions::{
        as_mutable_type::as_mutable_type_id,
        reduce_type_functions_type_function::reduce_type_functions,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{blocked_type::BlockedType, type_function_instance_type::TypeFunctionInstanceType},
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_ast::records::location::Location;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::tf_fixture::TfFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = TfFixture::default();

    let root = fixture.arena.add_type(BlockedType::default());
    let refine_func = NonNull::from(&fixture.builtin_types.type_functions.refine_func);
    let unknown_type = fixture.builtin_types.unknown_type;
    let refinement = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            refine_func,
            alloc::vec![root, unknown_type],
            alloc::vec![],
        ),
    );

    unsafe {
      (*as_mutable_type_id(root)).ty = TypeVariant::Bound(refinement);
    }
    let _res = reduce_type_functions(
      refinement,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      true,
    );

    assert_eq!("unknown", to_string_type_id(refinement));
  }
}

mod type_function_reduce_union_of_error_nil_table_with_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1872:type_function_reduce_union_of_error_nil_table_with_table`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_reduce_union_of_error_nil_table_with_table

  #[cfg(test)]
  #[test]
  fn type_function_reduce_union_of_error_nil_table_with_table() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      functions::{
        reduce_type_functions_type_function::reduce_type_functions,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType},
    };
    use ulua_ast::records::location::Location;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::tf_fixture::TfFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = TfFixture::default();

    let error_type = fixture.builtin_types.error_type;
    let nil_type = fixture.builtin_types.nil_type;
    let table_type = fixture.builtin_types.table_type;
    let union_type = fixture.arena.add_type(UnionType {
      options: alloc::vec![error_type, nil_type, table_type],
    });

    let refine_func = NonNull::from(&fixture.builtin_types.type_functions.refine_func);
    let refinement = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            refine_func,
            alloc::vec![union_type, table_type],
            alloc::vec![],
        ),
    );

    let _res = reduce_type_functions(
      refinement,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      true,
    );

    assert_eq!("*error-type* | table", to_string_type_id(refinement));
  }
}

mod type_function_refine_g_false {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1777:type_function_refine_g_false`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
  //!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
  //!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
  //!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
  //!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_function_refine_g_false

  #[cfg(test)]
  #[test]
  fn type_function_refine_g_false() {
    use core::ptr::NonNull;

    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::reduce_type_functions_type_function::reduce_type_functions,
      records::{generic_type::GenericType, type_function_instance_type::TypeFunctionInstanceType},
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::tf_fixture::TfFixture;

    let mut fixture = TfFixture::default();
    let g = fixture
      .arena
      .add_type(GenericType::generic_type_scope_polarity(
        fixture.tfc.scope.as_ptr(),
        Polarity::Negative,
      ));

    let refine_func = NonNull::from(&fixture.builtin_types.type_functions.refine_func);
    let truthy_type = fixture.builtin_types.truthy_type;
    let refine_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            refine_func,
            alloc::vec![g, truthy_type],
            alloc::vec![],
        ),
    );

    let res = reduce_type_functions(
      refine_ty,
      Location::default(),
      NonNull::from(&mut *fixture.tfc),
      false,
    );

    assert_eq!(1, res.reduced_types.size());
    assert_eq!(0, res.errors.len(), "{:?}", res.errors);
    assert_eq!(0, res.irreducible_types.size());
    assert_eq!(0, res.blocked_types.size());
  }
}

mod type_function_resolve_deep_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:127:type_function_resolve_deep_functions`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_resolve_deep_functions

  #[cfg(test)]
  #[test]
  fn type_function_resolve_deep_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Swap<Swap<Swap<string>>>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
  }
}

mod type_function_setmetatable_type_function_assigns_correct_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1419:type_function_setmetatable_type_function_assigns_correct_metatable`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_setmetatable_type_function_assigns_correct_metatable

  #[cfg(test)]
  #[test]
  fn type_function_setmetatable_type_function_assigns_correct_metatable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id,
        to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{metatable_type::MetatableType, to_string_options::ToStringOptions},
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
        type Identity = setmetatable<{}, { __index: {} }>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let id = fixture.base.require_type_alias(&String::from("Identity"));
    assert_eq!(
      "{ @metatable { __index: {  } }, {  } }",
      to_string_type_id_to_string_options_mut(id, ToStringOptions::new(true))
    );
    let mt = get_type_id::<MetatableType>(id)
      .unwrap_or_else(|| panic!("expected MetatableType, got {}", to_string_type_id(id)));
    assert_eq!("{ __index: {  } }", to_string_type_id(mt.metatable()));
  }
}

mod type_function_setmetatable_type_function_assigns_correct_metatable_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1437:type_function_setmetatable_type_function_assigns_correct_metatable_2`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_setmetatable_type_function_assigns_correct_metatable_2

  #[cfg(test)]
  #[test]
  fn type_function_setmetatable_type_function_assigns_correct_metatable_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id,
        to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{metatable_type::MetatableType, to_string_options::ToStringOptions},
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
        type Identity = setmetatable<{}, { __index: {} }>
        type FooBar = setmetatable<{}, Identity>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let id = fixture.base.require_type_alias(&String::from("Identity"));
    assert_eq!(
      "{ @metatable { __index: {  } }, {  } }",
      to_string_type_id_to_string_options_mut(id, ToStringOptions::new(true))
    );
    let mt = get_type_id::<MetatableType>(id).unwrap_or_else(|| {
      panic!(
        "expected Identity MetatableType, got {}",
        to_string_type_id(id)
      )
    });
    assert_eq!("{ __index: {  } }", to_string_type_id(mt.metatable()));

    let foobar = fixture.base.require_type_alias(&String::from("FooBar"));
    let mt2 = get_type_id::<MetatableType>(foobar).unwrap_or_else(|| {
      panic!(
        "expected FooBar MetatableType, got {}",
        to_string_type_id(foobar)
      )
    });
    assert_eq!(
      "{ @metatable { __index: {  } }, {  } }",
      to_string_type_id_to_string_options_mut(mt2.metatable(), ToStringOptions::new(true))
    );
  }
}

mod type_function_setmetatable_type_function_errors_on_invalid_set {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1481:type_function_setmetatable_type_function_errors_on_invalid_set`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_setmetatable_type_function_errors_on_invalid_set

  #[cfg(test)]
  #[test]
  fn type_function_setmetatable_type_function_errors_on_invalid_set() {
    use alloc::string::String;

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
        type Identity = setmetatable<string, {}>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_function_setmetatable_type_function_errors_on_metatable_with_metatable_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1461:type_function_setmetatable_type_function_errors_on_metatable_with_metatable_metamethod`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_setmetatable_type_function_errors_on_metatable_with_metatable_metamethod

  #[cfg(test)]
  #[test]
  fn type_function_setmetatable_type_function_errors_on_metatable_with_metatable_metamethod() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id,
        to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{metatable_type::MetatableType, to_string_options::ToStringOptions},
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
        type Identity = setmetatable<{}, { __metatable: "blocked" }>
        type Bad = setmetatable<Identity, { __index: {} }>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let id = fixture.base.require_type_alias(&String::from("Identity"));
    assert_eq!(
      "{ @metatable { __metatable: \"blocked\" }, {  } }",
      to_string_type_id_to_string_options_mut(id, ToStringOptions::new(true))
    );
    let mt = get_type_id::<MetatableType>(id)
      .unwrap_or_else(|| panic!("expected MetatableType, got {}", to_string_type_id(id)));
    assert_eq!(
      "{ __metatable: \"blocked\" }",
      to_string_type_id(mt.metatable())
    );
  }
}

mod type_function_setmetatable_type_function_errors_on_nontable_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1493:type_function_setmetatable_type_function_errors_on_nontable_metatable`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_setmetatable_type_function_errors_on_nontable_metatable

  #[cfg(test)]
  #[test]
  fn type_function_setmetatable_type_function_errors_on_nontable_metatable() {
    use alloc::string::String;

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
        type Identity = setmetatable<{}, string>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_function_table_internal_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:156:type_function_table_internal_functions`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_table_internal_functions

  #[cfg(test)]
  #[test]
  fn type_function_table_internal_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: <T>({T}) -> {Swap<T>}
        local a = t({1, 2, 3})
        local b = t({"a", "b", "c"})
        local c = t({true, false, true})
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{string}",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "{number}",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "{Swap<boolean>}",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "Type function instance Swap<boolean> is uninhabited",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_type_function_correct_cycle_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1610:type_function_type_function_correct_cycle_check`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_type_function_correct_cycle_check

  #[cfg(test)]
  #[test]
  fn type_function_type_function_correct_cycle_check() {
    use alloc::string::String;

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
type foo<T> = { a: add<T, T>, b : add<T, T> }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_type_functions_can_be_shadowed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:299:type_function_type_functions_can_be_shadowed`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_function_type_functions_can_be_shadowed

  #[cfg(test)]
  #[test]
  fn type_function_type_functions_can_be_shadowed() {
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
        type add<T> = string -- shadow add

        -- this should be ok
        function hi(f: add<unknown>)
            return string.format("hi %s", f)
        end

        -- this should still work totally fine (and use the real type function)
        function plus(a, b)
            return a + b
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "(string) -> string",
      to_string_type_id(fixture.base.require_type_string(&String::from("hi")))
    );
    assert_eq!(
      "<a, b>(a, b) -> add<a, b>",
      to_string_type_id(fixture.base.require_type_string(&String::from("plus")))
    );
  }
}

mod type_function_type_functions_inhabited_with_normalization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:324:type_function_type_functions_inhabited_with_normalization`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_function_type_functions_inhabited_with_normalization

  #[cfg(test)]
  #[test]
  fn type_function_type_functions_inhabited_with_normalization() {
    use alloc::string::String;

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
        local useGridConfig : any
        local columns = useGridConfig("columns", {}) or 1
        local gutter = useGridConfig('gutter', {}) or 0
        local margin = useGridConfig('margin', {}) or 0
        return function(frameAbsoluteWidth: number)
            local cellAbsoluteWidth = (frameAbsoluteWidth - 2 * margin + gutter) / columns - gutter
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_undefined_add_application {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1705:type_function_undefined_add_application`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_undefined_add_application

  #[cfg(test)]
  #[test]
  fn type_function_undefined_add_application() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        function add<A, B>(a: A, b: B): add<A, B>
            return a + b
        end

        local s = add(5, "hello")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      matches!(
        &result.errors[0].data,
        TypeErrorData::UninhabitedTypeFunction(_)
      ),
      "{:?}",
      result.errors[0]
    );
  }
}

mod type_function_unsolvable_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:140:type_function_unsolvable_function`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_function_unsolvable_function

  #[cfg(test)]
  #[test]
  fn type_function_unsolvable_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_function_fixture::TypeFunctionFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = TypeFunctionFixture::new();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local impossible: <T>(Swap<T>) -> Swap<Swap<T>>
        local a = impossible(123)
        local b = impossible(true)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be unreachable, but got 'number'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be unreachable, but got 'boolean'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_vector_2_multiply_is_overloaded {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:665:type_function_vector_2_multiply_is_overloaded`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_vector_2_multiply_is_overloaded

  #[cfg(test)]
  #[test]
  fn type_function_vector_2_multiply_is_overloaded() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local v = Vector2.New(1, 2)

        local v2 = v * 1.5
        local v3 = v * v
        local v4 = v * "Hello" -- line 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(5, result.errors[0].location.begin.line);
    assert_eq!(5, result.errors[0].location.end.line);

    assert_eq!(
      "Vector2",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("v2")))
    );
    assert_eq!(
      "Vector2",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("v3")))
    );
    assert_eq!(
      "mul<Vector2, string>",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("v4")))
    );
  }
}

mod type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:871:type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited`
  //! Source: `tests/TypeFunction.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - translates_to -> rust_item type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited

  #[cfg(test)]
  #[test]
  fn type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited() {
    use alloc::string::String;

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

local Debounce = false
local Active = false

local function Use(Mode)

	if Mode ~= nil then

		if Mode == false and Active == false then
			return
		else
			Active = not Mode
		end

		Debounce = false
	end
	Active = not Active

end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}
