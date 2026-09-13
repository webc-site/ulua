extern crate alloc;

mod type_infer_unknownnever_array_like_table_of_never_is_inhabitable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:104:type_infer_unknownnever_array_like_table_of_never_is_inhabitable`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_array_like_table_of_never_is_inhabitable

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:212:type_infer_unknownnever_assign_to_global_which_is_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_assign_to_global_which_is_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:194:type_infer_unknownnever_assign_to_local_which_is_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_assign_to_local_which_is_never

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_assign_to_local_which_is_never() {
    use ulua_common::FFlag;
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

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_unknownnever_assign_to_prop_which_is_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:223:type_infer_unknownnever_assign_to_prop_which_is_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_assign_to_prop_which_is_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:234:type_infer_unknownnever_assign_to_subscript_which_is_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_assign_to_subscript_which_is_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:180:type_infer_unknownnever_call_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_call_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:425:type_infer_unknownnever_cast_from_never_does_not_error`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_cast_from_never_does_not_error

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:368:type_infer_unknownnever_compare_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_compare_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:328:type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators()
   {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
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
    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:245:type_infer_unknownnever_for_loop_over_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_for_loop_over_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:168:type_infer_unknownnever_index_on_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_index_on_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:266:type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_never

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:286:type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:317:type_infer_unknownnever_length_of_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_length_of_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:380:type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:404:type_infer_unknownnever_lti_permit_explicit_never_annotation`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_lti_permit_explicit_never_annotation

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_lti_permit_explicit_never_annotation() {
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:344:type_infer_unknownnever_math_operators_and_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_math_operators_and_never

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_math_operators_and_never() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
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

    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:68:type_infer_unknownnever_never_is_reflexive`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_never_is_reflexive

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:57:type_infer_unknownnever_never_subtype_and_string_supertype`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_never_subtype_and_string_supertype

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:255:type_infer_unknownnever_pick_never_from_variadic_type_pack`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_pick_never_from_variadic_type_pack

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:46:type_infer_unknownnever_string_subtype_and_never_supertype`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_string_subtype_and_never_supertype

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:13:type_infer_unknownnever_string_subtype_and_unknown_supertype`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_string_subtype_and_unknown_supertype

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:95:type_infer_unknownnever_table_with_prop_of_type_never_is_also_reflexive`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_table_with_prop_of_type_never_is_also_reflexive

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:86:type_infer_unknownnever_table_with_prop_of_type_never_is_uninhabitable`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_table_with_prop_of_type_never_is_uninhabitable

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:113:type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable() {
    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
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

    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:140:type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2

  #[cfg(test)]
  #[test]
  fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
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

    if !FFlag::DebugLuauForceOldSolver.get() {
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:306:type_infer_unknownnever_unary_minus_of_never`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_unary_minus_of_never

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:79:type_infer_unknownnever_unknown_is_optional_because_it_too_encompasses_nil`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unknownnever_unknown_is_optional_because_it_too_encompasses_nil

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:35:type_infer_unknownnever_unknown_is_reflexive`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_unknown_is_reflexive

  #[cfg(test)]
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
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:24:type_infer_unknownnever_unknown_subtype_and_string_supertype`
  //! Source: `tests/TypeInfer.unknownnever.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_unknownnever_unknown_subtype_and_string_supertype

  #[cfg(test)]
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
