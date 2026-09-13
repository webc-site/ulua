extern crate alloc;

mod type_infer_union_types_allow_more_specific_assign {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_allow_more_specific_assign() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: number | string, b: (number | string)?)
            b = a
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_allow_specific_assign {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_allow_specific_assign() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a:number|string = 22
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_bounds_propagate_into_free_union_bounds {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_bounds_propagate_into_free_union_bounds() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(
      &FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
      true,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function unwrap<T>(a: T?): T
            if a == nil then
                error("Unexpected nil!")
            end
            return a
        end

        local b = unwrap(42)
        local c = unwrap(true)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
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

mod type_infer_union_types_disallow_less_specific_assign {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_disallow_less_specific_assign() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: number, b: number | string)
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_dont_allow_cyclic_unions_to_be_inferred {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_dont_allow_cyclic_unions_to_be_inferred() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function f(a, b)
            a:g(b or {})
            a:g(b)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_error_detailed_optional {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_error_detailed_optional() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X = { x: number }

local a: X? = { w = 4 }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Table type '{ w: number }' not compatible with type 'X' because the former is missing field 'x'"
    } else {
      r#"Expected this to be 'X?', but got 'a'
caused by:
  None of the union options are compatible. For example:
Table type 'a' not compatible with type 'X' because the former is missing field 'x'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_error_detailed_union_all {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_error_detailed_union_all() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { x: number }
        type Y = { y: number }
        type Z = { z: number }

        type XYZ = X | Y | Z

        local a: XYZ = { w = 4 }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'X | Y | Z', but got '{ w: number }'; \n\
this is because \n\
\t * the 1st component of the union is `X`, and `{ w: number }` is not a subtype of `X`\n\
\t * the 2nd component of the union is `Y`, and `{ w: number }` is not a subtype of `Y`\n\
\t * the 3rd component of the union is `Z`, and `{ w: number }` is not a subtype of `Z`\n";
      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
      assert_eq!(
        "Expected this to be 'X | Y | Z', but got 'a'; none of the union options are compatible",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_union_types_error_detailed_union_part {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_error_detailed_union_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X = { x: number }
type Y = { y: number }
type Z = { z: number }

type XYZ = X | Y | Z

function f(a: XYZ)
    local b: { w: number } = a
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be '{ w: number }', but got 'X | Y | Z'; \n",
        "this is because \n\t",
        " * the 1st component of the union is `X`, which is not a subtype of `{ w: number }`\n\t",
        " * the 2nd component of the union is `Y`, which is not a subtype of `{ w: number }`\n\t",
        " * the 3rd component of the union is `Z`, which is not a subtype of `{ w: number }`"
      )
    } else {
      r#"Expected this to be '{ w: number }', but got 'X | Y | Z'
caused by:
  Not all union options are compatible.
Table type 'X' not compatible with type '{ w: number }' because the former is missing field 'w'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_error_optional_argument_enforces_type {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_error_optional_argument_enforces_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        error("message", "2")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_error_takes_optional_arguments {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_error_takes_optional_arguments() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        error("message")
        error("message", 2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_fuzzer_union_with_one_part_assertion {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_fuzzer_union_with_one_part_assertion() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = {},nil
repeat

_,_ = if _.number == "" or _.number or _._ then
             _
      elseif _.__index == _._G then
            tostring
      elseif _ then
             _
      else
           ``,_._G

until _._
    "#,
      ),
      None,
    );
  }
}

mod type_infer_union_types_generic_function_with_optional_arg {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_generic_function_with_optional_arg() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<T>(x : T?) : {T}
            local result = {}
            if x then
                result[1] = x
            end
            return result
        end
        local t : {string} = f(nil)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_handle_multiple_optionals {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_handle_multiple_optionals() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: string??)
            if a then
                print(a:sub(1,1))
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_index_on_a_union_type_with_missing_property {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_with_missing_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::missing_union_property::MissingUnionProperty,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {}

        function f(t: A | B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mup = get_type_error::<MissingUnionProperty>(&result.errors[0]);
    assert!(mup.is_some());
    assert_eq!(
      "Key 'x' is missing from 'B' in the type 'A | B'",
      to_string_type_error(&result.errors[0])
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "(A | B) -> number"
    } else {
      "(A | B) -> *error-type*"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_index_on_a_union_type_with_mixed_types {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_with_mixed_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {x: string}

        function f(t: A | B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A | B) -> number | string",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_index_on_a_union_type_with_one_optional_property {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_with_one_optional_property() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {x: number?}

        function f(t: A | B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A | B) -> number?",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_index_on_a_union_type_with_one_property_of_type_any {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_with_one_property_of_type_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {x: any}

        function f(t: A | B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A | B) -> any",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_index_on_a_union_type_with_property_guaranteed_to_exist {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_with_property_guaranteed_to_exist() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {x: number}

        function f(t: A | B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A | B) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_index_on_a_union_type_works_at_arbitrary_depth {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_index_on_a_union_type_works_at_arbitrary_depth() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: {y: {z: {thing: number}}}}
        type B = {x: {y: {z: {thing: string}}}}

        function f(t: A | B)
            return t.x.y.z.thing
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A | B) -> number | string",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_indexing_into_a_cyclic_union_doesnt_crash {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_indexing_into_a_cyclic_union_doesnt_crash() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{as_mutable_type::as_mutable_type_id, freeze::freeze, unfreeze::unfreeze},
      records::{
        builtin_types::BuiltinTypes, frontend::Frontend, scope::Scope, table_indexer::TableIndexer,
        table_type::TableType, type_fun::TypeFun, type_level::TypeLevel, union_type::UnionType,
      },
      type_aliases::{props_type::Props, type_variant::TypeVariant},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let builtins_ptr = fixture.get_builtins() as *mut BuiltinTypes;

    unsafe {
      let frontend = &mut *frontend_ptr;
      let global_scope = frontend.globals.global_scope();
      let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
      let arena = frontend.globals.global_types_mut();

      unfreeze(arena);

      let bad_cyclic_union_ty =
        arena.fresh_type_not_null_builtin_types_scope(&*builtins_ptr, global_scope_ptr);
      let number_type = (*builtins_ptr).number_type;
      let props: Props = Default::default();
      let number_array_ty = arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &props,
          Some(TableIndexer {
            index_type: number_type,
            index_result_type: number_type,
            is_read_only: false,
          }),
          TypeLevel::default(),
          global_scope_ptr,
          TableState::Sealed,
        ),
      );

      (*as_mutable_type_id(bad_cyclic_union_ty)).ty = TypeVariant::Union(UnionType {
        options: vec![bad_cyclic_union_ty, number_array_ty],
      });

      (*global_scope_ptr).exported_type_bindings.insert(
        String::from("BadCyclicUnion"),
        TypeFun::type_fun_type_id(bad_cyclic_union_ty),
      );

      freeze(arena);
    }

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: BadCyclicUnion)
            return x[0]
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_less_greedy_unification_with_union_types {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_less_greedy_unification_with_union_types() {
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
        local function f(t): { x: number } | { x: string }
            local x = t.x
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a>(({ read x: a } & { x: number }) | ({ read x: a } & { x: string })) -> { x: number } | { x: string }",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_less_greedy_unification_with_union_types_2 {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_less_greedy_unification_with_union_types_2() {
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
        local function f(t: { x: number } | { x: string })
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ x: number } | { x: string }) -> number | string",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_lookup_prop_of_intersection_containing_unions {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_lookup_prop_of_intersection_containing_unions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function mergeOptions<T>(options: T & ({} | {}))
            return options.variables
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let unknown_prop =
      get_type_error::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("variables", unknown_prop.key());
  }
}

mod type_infer_union_types_optional_any {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_any() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(sup : any?, sub : number)
            sup = sub
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_optional_arguments {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_arguments() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a:string, b:string?)
        end
        f("s")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_optional_arguments_table {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_arguments_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a:{a:string, b:string?}
        a = {a="ok"}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_optional_arguments_table_2 {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_arguments_table_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a:{a:string, b:string}
        a = {a=""}
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_union_types_optional_assignment_errors {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_assignment_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number }
        function f(a: A?)
            a.x = 2
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_optional_assignment_errors_2 {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_assignment_errors_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number } & { y: number }
        function f(a: A?)
            a.x = 2
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type '({ x: number } & { y: number })?' could be nil",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_optional_call_error {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_call_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> number
        function f(a: A?)
            local b = a(4)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type '((number) -> number)?' could be nil",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_optional_field_access_error {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_field_access_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number }
        function f(b: A?)
            local c = b.x
            local d = b.y
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Key 'y' not found in table 'A'",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_infer_union_types_optional_index_error {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_index_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {number}
        function f(a: A?)
            local b = a[1]
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_optional_iteration {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_iteration() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function foo(values: {number}?)
    local s = 0
    for _, value in values do
        s += value
    end
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type '{number}?' could be nil",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_optional_length_error {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_length_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {number}
        function f(a: A?)
            local b = #a
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Operator '#' could not be applied to operand of type A?; there is no corresponding overload for __len",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_union_types_optional_missing_key_error_details {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_missing_key_error_details() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number, y: number }
        type B = { x: number, y: number }
        type C = { x: number }
        type D = { x: number }

        function f(a: A | B | C | D)
            local y = a.y
            local z = a.z
        end

        function g(c: A | B | C | D | nil)
            local d = c.y
        end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Key 'y' is missing from 'C', 'D' in the type 'A | B | C | D'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Type 'A | B | C | D' does not have key 'z'",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Value of type '(A | B | C | D)?' could be nil",
      to_string_type_error(&result.errors[2])
    );
    assert_eq!(
      "Key 'y' is missing from 'C', 'D' in the type 'A | B | C | D'",
      to_string_type_error(&result.errors[3])
    );
  }
}

mod type_infer_union_types_optional_union_follow {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_union_follow() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local y: number? = 2
        local x = y
        function f(a: number, b: number?, c: number?) return -a end
        return f()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(1, acm.expected());
    assert_eq!(0, acm.actual());
    assert!(!acm.is_variadic());
  }
}

mod type_infer_union_types_optional_union_functions {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_union_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        function a.foo(x:number, y:number) return x + y end
        type A = typeof(a)
        function f(b: A?)
            return b.foo(1, 2)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "(A?) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_optional_union_members {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_union_members() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = { a = { x = 1, y = 2 }, b = 3 }
        type A = typeof(a)
        function f(b: A?)
            return b.a.y
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "(A?) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_optional_union_methods {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_optional_union_methods() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        function a:foo(x:number, y:number) return x + y end
        type A = typeof(a)
        function f(b: A?)
            return b:foo(1, 2)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'A?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "(A?) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_union_types_return_types_can_be_disjoint {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_return_types_can_be_disjoint() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id, records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local count = 0
        function most_of_the_natural_numbers(): number?
            if count < 10 then
                count = count + 1
                return count
            else
                return nil
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let utv = get_type_id::<FunctionType>(
      fixture.require_type_string(&String::from("most_of_the_natural_numbers")),
    );
    assert!(utv.is_some());
  }
}

mod type_infer_union_types_return_types_can_be_disjoint_using_compound_assignment {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_return_types_can_be_disjoint_using_compound_assignment() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id, records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local count = 0
        function most_of_the_natural_numbers(): number?
            if count < 10 then
                -- count = count + 1
                count += 1
                return count
            else
                return nil
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let utv = get_type_id::<FunctionType>(
      fixture.require_type_string(&String::from("most_of_the_natural_numbers")),
    );
    assert!(utv.is_some());
  }
}

mod type_infer_union_types_suppress_errors_for_prop_lookup_of_a_union_that_includes_error {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_suppress_errors_for_prop_lookup_of_a_union_that_includes_error() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::register_hidden_types::register_hidden_types, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: err | Not<nil>)
            local b = a.foo
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_table_union_write_indirect {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_table_union_write_indirect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number, y: (number) -> string } | { z: number, y: (number) -> string }

        function f(a: A)
            function a.y(x)
                return tostring(x * 2)
            end

            function a.y(x: string): number
                return tonumber(x) or 0
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((number) -> string) | ((number) -> string)'\
\nbut got\n\t\
'(string) -> number'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_unify_sealed_table_union_check {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_unify_sealed_table_union_check() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
 -- the difference between this and unify_unsealed_table_union_check is the type annotation on x
local t = { x = 3, y = true }
local x: { x: number } = t
type A = number?
type B = string?
local y: { x: number, y: A | B }
-- Shouldn't typecheck!
y = x
-- If it does, we can convert any type to any other type
y.y = 5
local oh : boolean = t.y
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_union_types_unify_unsealed_table_union_check {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_unify_unsealed_table_union_check() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x = { x = 3 }
type A = number?
type B = string?
local y: { x: number, y: A | B }
y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x = { x = 3 }

local a: number? = 2
local y = {}
y.x = 2
y.y = a

y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_union_equality_comparisons {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_equality_comparisons() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = number | string | nil
        type B = number | nil
        type C = number | boolean

        function f(a: A, b: B, c: C)
            local n = 1

            local x = a == b
            local y = a == n
            local z = a == c
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_union_function_any_args {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_function_any_args() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(sup : ((...any) -> (...any))?, sub : ((number) -> (...any)))
            sup = sub
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_union_of_functions {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : (number) -> number?)
            local y : ((number?) -> number?) | ((number) -> number) = x -- OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_union_of_functions_mentioning_generic_typepacks {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_mentioning_generic_typepacks() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...>()
            function g(x : (number, a...) -> (number?, a...))
                local y : ((number | string, a...) -> (number, a...)) | ((number?, a...) -> (nil, a...)) = x -- OK
                local z : ((number) -> number) | ((number?, a...) -> (number?, a...)) = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((number) -> number) | ((number?, a...) -> (number?, a...))'\
\nbut got\n\t\
'(number, a...) -> (number?, a...)'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_union_of_functions_mentioning_generics {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_mentioning_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a,b>()
            function g(x : (a) -> a?)
                local y : ((a?) -> nil) | ((a) -> a) = x -- OK
                local z : ((b?) -> nil) | ((b) -> b) = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be '((b) -> b) | ((b?) -> nil)', but got '(a) -> a?'; none of the union options are compatible",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_union_types_union_of_functions_with_mismatching_arg_arities {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_with_mismatching_arg_arities() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : (number) -> number?)
            local y : ((number?) -> number) | ((number | string) -> nil) = x -- OK
            local z : ((number, string?) -> number) | ((number) -> nil) = x -- Not OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((number) -> nil) | ((number, string?) -> number)'\
\nbut got\n\t\
'(number) -> number?'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_union_of_functions_with_mismatching_arg_variadics {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_with_mismatching_arg_variadics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : (number) -> ())
            local y : ((number?) -> ()) | ((...number) -> ()) = x -- OK
            local z : ((number?) -> ()) | ((...number?) -> ()) = x -- Not OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be\n\
\t'((...number?) -> ()) | ((number?) -> ())'\n\
but got\n\
\t'(number) -> ()'; \n\
this is because \n\
\t * it takes `number` and in the 2nd component of the union, the function takes a tail of `...number?`, and `number` is not a supertype of `...number?`\n\
\t * it takes the 1st entry in the type pack is `number` and in the 1st component of the union, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`";
      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = "Expected this to be\n\t\
'((...number?) -> ()) | ((number?) -> ())'\
\nbut got\n\t\
'(number) -> ()'; none of the union options are compatible";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_union_types_union_of_functions_with_mismatching_result_arities {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_with_mismatching_result_arities() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : () -> (number | string))
            local y : (() -> number) | (() -> string) = x -- OK
            local z : (() -> number) | (() -> (string, string)) = x -- Not OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'(() -> (string, string)) | (() -> number)'\
\nbut got\n\t\
'() -> number | string'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_union_of_functions_with_mismatching_result_variadics {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_with_mismatching_result_variadics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : () -> (number?, ...number))
            local y : (() -> (...number)) | (() -> nil) = x -- OK
            local z : (() -> (...number)) | (() -> number) = x -- OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'(() -> (...number)) | (() -> number)'\
\nbut got\n\t\
'() -> (number?, ...number)'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_union_of_functions_with_variadics {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_functions_with_variadics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : (...nil) -> (...number?))
            local y : ((...string?) -> (...number)) | ((...number?) -> nil) = x -- OK
            local z : ((...string?) -> (...number)) | ((...string?) -> nil) = x -- OK
        end
     "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((...string?) -> (...number)) | ((...string?) -> nil)'\
\nbut got\n\t\
'(...nil) -> (...number?)'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_union_types_union_of_generic_functions {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_generic_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : <a>(a) -> a?)
            local y : (<a>(a?) -> a?) | (<b>(b) -> b) = x -- Not OK
        end
     "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_union_types_union_of_generic_typepack_functions {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_of_generic_typepack_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : <a...>(number, a...) -> (number?, a...))
            local y : (<a...>(number?, a...) -> (number?, a...)) | (<b...>(number, b...) -> (number, b...)) = x -- Not OK
        end
     "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_union_types_union_table_any_property {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_table_any_property() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            -- x : X
            -- sup : { p : { q : X } }?
            local sup = if true then { p = { q = x } } else nil
            local sub : { p : any }
            sup = nil
            sup = sub
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_union_types_union_true_and_false {
  //! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_union_types_union_true_and_false() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : boolean)
            local y1 : (true | false) = x -- OK
            local y2 : (true | false | (string & number)) = x -- OK
            local y3 : (true | (string & number) | false) = x -- OK
            local y4 : (true | (boolean & true) | false) = x -- OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
