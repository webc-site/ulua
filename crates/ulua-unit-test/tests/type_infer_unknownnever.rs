extern crate alloc;

mod type_infer_unknownnever_array_like_table_of_never_is_inhabitable {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_array_like_table_of_never_is_inhabitable() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {never} = {}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_assign_to_global_which_is_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_assign_to_global_which_is_never() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        t = 5 :: never
        t = ""
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_assign_to_local_which_is_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_assign_to_local_which_is_never() {
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: never
        t = 3
    "#,
      ),
      None,
    );

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_unknownnever_assign_to_prop_which_is_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_assign_to_prop_which_is_never() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: never)
            t.x = 5
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_assign_to_subscript_which_is_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_assign_to_subscript_which_is_never() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: never)
            t[5] = 7
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_call_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_call_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: never = 5 :: never
        local x, y, z = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("z")))
    );
  }
}

mod type_infer_unknownnever_cast_from_never_does_not_error {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_cast_from_never_does_not_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: never): number
            return x :: number
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_compare_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_compare_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function cmp(x: nil, y: number)
            return x ~= nil and x > y and x < y -- infers boolean | never, which is normalized into boolean
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(nil, number) -> boolean",
      to_string_type_id(fixture.require_type_string(&String::from("cmp")))
    );
  }
}

mod type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators()
   {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function ord(x: nil, y)
            return x ~= nil and x > y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(nil, nil & ~nil) -> boolean",
        to_string_type_id(fixture.require_type_string(&String::from("ord")))
      );
    } else {
      assert_eq!(
        "<a>(nil, a) -> boolean",
        to_string_type_id(fixture.require_type_string(&String::from("ord")))
      );
    }
  }
}

mod type_infer_unknownnever_for_loop_over_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_for_loop_over_never() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        for i, v in (5 :: never) do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_index_on_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_index_on_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: never = 5 :: never
        local z = x.y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("z")))
    );
  }
}

mod type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    if !fflag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Disjoint = {foo: never, bar: unknown, tag: "ok"} | {foo: never, baz: unknown, tag: "err"}

        function f(disjoint: Disjoint)
            return disjoint.foo
        end

        local foo = f({foo = 5 :: never, bar = true, tag = "ok"})
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    if !fflag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Disjoint = {foo: string, bar: unknown, tag: "ok"} | {foo: never, baz: unknown, tag: "err"}

        function f(disjoint: Disjoint)
            return disjoint.foo
        end

        local foo = f({foo = 5 :: never, bar = true, tag = "ok"})
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_unknownnever_length_of_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_length_of_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = #({} :: never)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function num(x: number) end
        local function str(x: string) end
        local function cond(): boolean return false end

        local function f(a)
            if cond() then
                num(a)
            else
                str(a)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Parameter 'a' has been reduced to never. This function is not callable with any possible value.",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Parameter 'a' is required to be a subtype of 'number' here.",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Parameter 'a' is required to be a subtype of 'string' here.",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_infer_unknownnever_lti_permit_explicit_never_annotation {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_lti_permit_explicit_never_annotation() {
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function num(x: number) end
        local function str(x: string) end
        local function cond(): boolean return false end

        local function f(a: never)
            if cond() then
                num(a)
            else
                str(a)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_math_operators_and_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_math_operators_and_never() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function mul(x: nil, y)
            return x ~= nil and x * y -- infers boolean | never, which is normalized into boolean
        end
    "#,
      ),
      None,
    );

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(
        matches!(
          result.errors[0].data,
          TypeErrorData::ExplicitFunctionAnnotationRecommended(_)
        ),
        "{:?}",
        result.errors[0]
      );

      assert_eq!(
        "<a>(nil, a) -> false | mul<nil & ~nil, a>",
        to_string_type_id(fixture.require_type_string(&String::from("mul")))
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "<a>(nil, a) -> boolean",
        to_string_type_id(fixture.require_type_string(&String::from("mul")))
      );
    }
  }
}

mod type_infer_unknownnever_never_is_reflexive {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_never_is_reflexive() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: never)
            local foo: never = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_never_subtype_and_string_supertype {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_never_subtype_and_string_supertype() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: never)
            local foo: string = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_pick_never_from_variadic_type_pack {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_pick_never_from_variadic_type_pack() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(...: never)
            local x, y = (...)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_string_subtype_and_never_supertype {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_string_subtype_and_never_supertype() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string)
            local foo: never = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_string_subtype_and_unknown_supertype {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_string_subtype_and_unknown_supertype() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string)
            local foo: unknown = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_table_with_prop_of_type_never_is_also_reflexive {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_table_with_prop_of_type_never_is_also_reflexive() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: never} = {x = 5 :: never}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_table_with_prop_of_type_never_is_uninhabitable {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_table_with_prop_of_type_never_is_uninhabitable() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: never} = {}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable() {
    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f() return "foo", 5 :: never end

        local x, y, z = f()
    "#,
      ),
      None,
    );

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Function only returns 2 values, but 3 are required here",
        to_string_type_error(&result.errors[0])
      );

      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
      );
      assert_eq!(
        "nil",
        to_string_type_id(fixture.require_type_string(&String::from("z")))
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);

      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("z")))
      );
    }
  }
}

mod type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2 {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (string, never) return "", 5 :: never end
        local function g(): (never, string) return 5 :: never, "" end

        local x1, x2 = f()
        local y1, y2 = g()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("x1")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("x2")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("y1")))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("y2")))
      );
    } else {
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("x1")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("x2")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("y1")))
      );
      assert_eq!(
        "never",
        to_string_type_id(fixture.require_type_string(&String::from("y2")))
      );
    }
  }
}

mod type_infer_unknownnever_unary_minus_of_never {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_unary_minus_of_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = -(5 :: never)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_unknownnever_unknown_is_optional_because_it_too_encompasses_nil {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_unknown_is_optional_because_it_too_encompasses_nil() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: unknown} = {}
    "#,
      ),
      None,
    );
  }
}

mod type_infer_unknownnever_unknown_is_reflexive {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_unknown_is_reflexive() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            local foo: unknown = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unknownnever_unknown_subtype_and_string_supertype {
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`

  #[test]
  fn type_infer_unknownnever_unknown_subtype_and_string_supertype() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown)
            local foo: string = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}
