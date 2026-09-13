extern crate alloc;

mod type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:281:type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        table.insert()
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "No overload for function accepts 0 arguments.",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!("MainModule", result.errors[1].module_name);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Available overloads: <V>({V}, V) -> (); and <V>({V}, number, V) -> ()"
    } else {
      "Available overloads: ({'a}, 'a) -> (); and ({'a}, number, 'a) -> ()"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[1]));
  }
}

mod type_infer_try_unify_cli_50320_follow_in_any_unification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:319:type_infer_try_unify_cli_50320_follow_in_any_unification`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - translates_to -> rust_item type_infer_try_unify_cli_50320_follow_in_any_unification

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_cli_50320_follow_in_any_unification() {
    use ulua_analysis::records::function_type::FunctionType;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();

    let free = fixture.free_type_pack();
    let target = fixture.type_pack(Vec::new());
    let func = fixture
      .arena
      .add_type(FunctionType::function_type_new(free, free, None, false));
    let any = fixture.get_builtins().any_type;

    fixture
      .state
      .try_unify_type_pack_id_type_pack_id_bool(free, target, false);

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(func, any, false, false, None);

    assert!(!fixture.state.failure);
    assert!(
      fixture.state.errors.is_empty(),
      "{:?}",
      fixture.state.errors
    );
  }
}

mod type_infer_try_unify_compatible_functions_are_unified {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:42:type_infer_try_unify_compatible_functions_are_unified`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_try_unify_compatible_functions_are_unified

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_compatible_functions_are_unified() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::default();
    let number_type = fixture.get_builtins().number_type;
    let arg_one = fixture.fresh_type();
    let arg_two = fixture.fresh_type();
    let ret_two = fixture.fresh_type();
    let function_one = fixture.function_type(vec![arg_one], vec![number_type]);
    let function_two = fixture.function_type(vec![arg_two], vec![ret_two]);

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        function_two,
        function_one,
        false,
        false,
        None,
      );
    assert!(!fixture.state.failure);
    assert!(
      fixture.state.errors.is_empty(),
      "{:?}",
      fixture.state.errors
    );

    fixture.state.log.commit();

    assert_eq!(
      to_string_type_id(function_one),
      to_string_type_id(function_two)
    );
  }
}

mod type_infer_try_unify_free_tail_is_grown_properly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:298:type_infer_try_unify_free_tail_is_grown_properly`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item type_infer_try_unify_free_tail_is_grown_properly

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_free_tail_is_grown_properly() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().number_type;

    let three_numbers = fixture.type_pack(vec![number, number, number]);
    let free_tail = fixture.free_type_pack();
    let number_and_free_tail = fixture.type_pack_with_tail(vec![number], free_tail);

    let errors = fixture.state.can_unify_type_pack_id_type_pack_id_bool(
      number_and_free_tail,
      three_numbers,
      false,
    );

    assert!(errors.is_empty(), "{:?}", errors);
  }
}

mod type_infer_try_unify_fuzz_tail_unification_issue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:353:type_infer_try_unify_fuzz_tail_unification_issue`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - translates_to -> rust_item type_infer_try_unify_fuzz_tail_unification_issue

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_fuzz_tail_unification_issue() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let any = fixture.get_builtins().any_type;

    let variadic_any = fixture.variadic_type_pack(any);
    let pack_tmp = fixture.type_pack_with_tail(vec![any], variadic_any);
    let pack_sub = fixture.type_pack_with_tail(vec![any, any], pack_tmp);

    let free_ty = fixture.fresh_type();
    let free_tp = fixture.free_type_pack();
    let pack_super = fixture.type_pack_with_tail(vec![free_ty], free_tp);

    fixture
      .state
      .try_unify_type_pack_id_type_pack_id_bool(pack_sub, pack_super, false);
  }
}

mod type_infer_try_unify_fuzz_unify_any_should_check_log {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:366:type_infer_try_unify_fuzz_unify_any_should_check_log`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_try_unify_fuzz_unify_any_should_check_log

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_fuzz_unify_any_should_check_log() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
repeat
_._,_ = nil
until _
local l0:(any)&(typeof(_)),l0:(any)|(any) = _,_
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_incompatible_functions_are_preserved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:62:type_infer_try_unify_incompatible_functions_are_preserved`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_try_unify_incompatible_functions_are_preserved

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_incompatible_functions_are_preserved() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::default();
    let number_type = fixture.get_builtins().number_type;
    let string_type = fixture.get_builtins().string_type;
    let arg_one = fixture.fresh_type();
    let arg_two = fixture.fresh_type();
    let function_one = fixture.function_type(vec![arg_one], vec![number_type]);
    let function_one_saved = to_string_type_id(function_one);
    let function_two = fixture.function_type(vec![arg_two], vec![string_type]);
    let function_two_saved = to_string_type_id(function_two);

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        function_two,
        function_one,
        false,
        false,
        None,
      );

    assert!(fixture.state.failure);
    assert!(!fixture.state.errors.is_empty());
    assert_eq!(function_one_saved, to_string_type_id(function_one));
    assert_eq!(function_two_saved, to_string_type_id(function_two));
  }
}

mod type_infer_try_unify_incompatible_tables_are_preserved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:111:type_infer_try_unify_incompatible_tables_are_preserved`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method Property::type_DEPRECATED (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_incompatible_tables_are_preserved

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_incompatible_tables_are_preserved() {
    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::default();
    let number_type = fixture.get_builtins().number_type;
    let string_type = fixture.get_builtins().string_type;
    let foo_one = fixture.fresh_type();
    let foo_two = fixture.fresh_type();
    let table_one = fixture.unsealed_table_type(&[("foo", foo_one), ("bar", number_type)]);
    let table_two = fixture.unsealed_table_type(&[("foo", foo_two), ("bar", string_type)]);

    let table_one_foo = fixture.table_prop_type(table_one, "foo");
    let table_two_foo = fixture.table_prop_type(table_two, "foo");
    assert_ne!(table_one_foo, table_two_foo);

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        table_two, table_one, false, false, None,
      );

    assert!(fixture.state.failure);
    assert_eq!(1, fixture.state.errors.len(), "{:?}", fixture.state.errors);
    assert_ne!(follow_type_id(table_one_foo), follow_type_id(table_two_foo));
  }
}

mod type_infer_try_unify_members_of_failed_typepack_unification_are_unified_with_error_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:188:type_infer_try_unify_members_of_failed_typepack_unification_are_unified_with_error_type`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_try_unify_members_of_failed_typepack_unification_are_unified_with_error_type

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_members_of_failed_typepack_unification_are_unified_with_error_type() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg: number) end
        local a
        local b
        f(a, b)
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
      "*error-type*",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_try_unify_primitives_unify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:31:type_infer_try_unify_primitives_unify`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_try_unify_primitives_unify

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_primitives_unify() {
    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::default();
    let number_one = fixture.arena.add_type(PrimitiveType {
      r#type: PrimitiveType::NUMBER,
      metatable: None,
    });
    let number_two = fixture.arena.add_type(PrimitiveType {
      r#type: PrimitiveType::NUMBER,
      metatable: None,
    });

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        number_two, number_one, false, false, None,
      );

    assert!(!fixture.state.failure);
    assert!(
      fixture.state.errors.is_empty(),
      "{:?}",
      fixture.state.errors
    );
  }
}

mod type_infer_try_unify_recursive_metatable_getmatchtag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:308:type_infer_try_unify_recursive_metatable_getmatchtag`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_try_unify_recursive_metatable_getmatchtag

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_recursive_metatable_getmatchtag() {
    use ulua_analysis::{
      records::{
        metatable_type::MetatableType, table_type::TableType, r#type::Type, union_type::UnionType,
      },
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();

    let redirect = fixture.fresh_type();
    let table = fixture.arena.add_type(TableType::new());
    let metatable = fixture.arena.add_type(MetatableType::new(redirect, table));

    unsafe {
      (*(redirect as *mut Type)).ty = TypeVariant::Bound(metatable);
    }

    let number = fixture.get_builtins().number_type;
    let variant = fixture.arena.add_type(UnionType {
      options: vec![metatable, number],
    });

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        metatable, variant, false, false, None,
      );
  }
}

mod type_infer_try_unify_result_of_failed_typepack_unification_is_constrained {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:205:type_infer_try_unify_result_of_failed_typepack_unification_is_constrained`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_try_unify_result_of_failed_typepack_unification_is_constrained

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_result_of_failed_typepack_unification_is_constrained() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg: number) return arg end
        local a
        local b
        local c = f(a, b)
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
      "*error-type*",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_try_unify_table_unification_full_restart_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:378:type_infer_try_unify_table_unification_full_restart_recursion`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_table_unification_full_restart_recursion

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_table_unification_full_restart_recursion() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local A, B, C, D

E = function(a, b)
    local mt = getmetatable(b)
    if mt.tm:bar(A) == nil and mt.tm:bar(B) == nil then end
    if mt.foo == true then D(b, 3) end
    mt.foo:call(false, b)
end

A = function(a, b)
    local mt = getmetatable(b)
    if mt.foo == true then D(b, 3) end
    C(mt, 3)
end

B = function(a, b)
    local mt = getmetatable(b)
    if mt.foo == true then D(b, 3) end
    C(mt, 3)
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_tables_can_be_unified {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:86:type_infer_try_unify_tables_can_be_unified`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method Property::type_DEPRECATED (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_tables_can_be_unified

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_tables_can_be_unified() {
    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::default();
    let foo_one = fixture.fresh_type();
    let foo_two = fixture.fresh_type();
    let table_one = fixture.unsealed_table_type(&[("foo", foo_one)]);
    let table_two = fixture.unsealed_table_type(&[("foo", foo_two)]);

    let table_one_foo = fixture.table_prop_type(table_one, "foo");
    let table_two_foo = fixture.table_prop_type(table_two, "foo");
    assert_ne!(table_one_foo, table_two_foo);

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(
        table_two, table_one, false, false, None,
      );

    assert!(!fixture.state.failure);
    assert!(
      fixture.state.errors.is_empty(),
      "{:?}",
      fixture.state.errors
    );

    fixture.state.log.commit();

    assert_eq!(follow_type_id(table_one_foo), follow_type_id(table_two_foo));
  }
}

mod type_infer_try_unify_txnlog_preserves_pack_owner {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:342:type_infer_try_unify_txnlog_preserves_pack_owner`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item type_infer_try_unify_txnlog_preserves_pack_owner

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_txnlog_preserves_pack_owner() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let a = fixture.free_type_pack();
    let b = fixture.get_builtins().any_type_pack;

    fixture
      .state
      .try_unify_type_pack_id_type_pack_id_bool(a, b, false);
    fixture.state.log.commit();

    assert_eq!(
      unsafe { (&*a).owning_arena() },
      &mut *fixture.arena as *mut _
    );
  }
}

mod type_infer_try_unify_txnlog_preserves_type_owner {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:331:type_infer_try_unify_txnlog_preserves_type_owner`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item type_infer_try_unify_txnlog_preserves_type_owner

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_txnlog_preserves_type_owner() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let a = fixture.fresh_type();
    let b = fixture.get_builtins().number_type;

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(a, b, false, false, None);
    fixture.state.log.commit();

    assert_eq!(unsafe { (*a).owning_arena }, &mut *fixture.arena as *mut _);
  }
}

mod type_infer_try_unify_typepack_unification_should_trim_free_tails {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:223:type_infer_try_unify_typepack_unification_should_trim_free_tails`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_try_unify_typepack_unification_should_trim_free_tails

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_typepack_unification_should_trim_free_tails() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function f(v: number)
            if v % 2 == 0 then
                return true
            end
        end

        return function()
            return (f(1))
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> boolean",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_try_unify_uninhabited_intersection_sub_anything {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:154:type_infer_try_unify_uninhabited_intersection_sub_anything`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_uninhabited_intersection_sub_anything

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_uninhabited_intersection_sub_anything() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg : string & number) : boolean
          return arg
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_uninhabited_intersection_sub_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:144:type_infer_try_unify_uninhabited_intersection_sub_never`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_uninhabited_intersection_sub_never

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_uninhabited_intersection_sub_never() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg : string & number) : never
          return arg
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_uninhabited_table_sub_anything {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:176:type_infer_try_unify_uninhabited_table_sub_anything`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_uninhabited_table_sub_anything

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_uninhabited_table_sub_anything() {
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg : { prop : string & number }) : boolean
          return arg
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_uninhabited_table_sub_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:164:type_infer_try_unify_uninhabited_table_sub_never`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_try_unify_uninhabited_table_sub_never

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_uninhabited_table_sub_never() {
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(arg : { prop : string & number }) : never
          return arg
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_try_unify_variadic_tails_respect_progress {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:252:type_infer_try_unify_variadic_tails_respect_progress`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - translates_to -> rust_item type_infer_try_unify_variadic_tails_respect_progress

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_variadic_tails_respect_progress() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().number_type;
    let string = fixture.get_builtins().string_type;
    let boolean = fixture.get_builtins().boolean_type;

    let variadic_pack = fixture.variadic_type_pack(boolean);
    let a = fixture.type_pack(vec![number, string, boolean, boolean]);
    let b = fixture.type_pack_with_tail(vec![number, string], variadic_pack);

    fixture
      .state
      .try_unify_type_pack_id_type_pack_id_bool(b, a, false);

    assert!(!fixture.state.failure);
    assert!(fixture.state.errors.is_empty());
  }
}

mod type_infer_try_unify_variadic_type_pack_unification {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:242:type_infer_try_unify_variadic_type_pack_unification`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
  //!   - translates_to -> rust_item type_infer_try_unify_variadic_type_pack_unification

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_variadic_type_pack_unification() {
    use ulua_unit_test::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().number_type;
    let string = fixture.get_builtins().string_type;

    let test_pack = fixture.type_pack(vec![number, string]);
    let variadic_pack = fixture.variadic_type_pack(number);

    fixture
      .state
      .try_unify_type_pack_id_type_pack_id_bool(test_pack, variadic_pack, false);

    assert!(fixture.state.failure);
    assert!(!fixture.state.errors.is_empty());
  }
}

mod type_infer_try_unify_variadics_should_use_reversed_properly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:263:type_infer_try_unify_variadics_should_use_reversed_properly`
  //! Source: `tests/TypeInfer.tryUnify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_try_unify_variadics_should_use_reversed_properly

  #[cfg(test)]
  #[test]
  fn type_infer_try_unify_variadics_should_use_reversed_properly() {
    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function f<T>(...: T): ...T
            return ...
        end

        local x: string = f(1)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch =
      get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(mismatch.given_type));
    assert_eq!("string", to_string_type_id(mismatch.wanted_type));
  }
}
