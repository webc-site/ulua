extern crate alloc;

mod type_infer_negations_cofinite_strings_can_be_compared_for_equality {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:51:type_infer_negations_cofinite_strings_can_be_compared_for_equality`
  //! Source: `tests/TypeInfer.negations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.negations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.negations.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_negations_cofinite_strings_can_be_compared_for_equality

  #[cfg(test)]
  #[test]
  fn type_infer_negations_cofinite_strings_can_be_compared_for_equality() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(e)
            if e == 'strictEqual' then
                e = 'strictEqualObject'
            end
            if e == 'deepStrictEqual' or e == 'strictEqual' then
            elseif e == 'notDeepStrictEqual' or e == 'notStrictEqual' then
            end
            return e
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

mod type_infer_negations_compare_cofinite_strings {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:69:type_infer_negations_compare_cofinite_strings`
  //! Source: `tests/TypeInfer.negations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.negations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.negations.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_negations_compare_cofinite_strings

  #[cfg(test)]
  #[test]
  fn type_infer_negations_compare_cofinite_strings() {
    use ulua_unit_test::records::negation_fixture::NegationFixture;

    let mut fixture = NegationFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local u : Not<"a">
local v : "b"
if u == v then
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_negations_negated_string_is_a_subtype_of_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:29:type_infer_negations_negated_string_is_a_subtype_of_string`
  //! Source: `tests/TypeInfer.negations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.negations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.negations.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_negations_negated_string_is_a_subtype_of_string

  #[cfg(test)]
  #[test]
  fn type_infer_negations_negated_string_is_a_subtype_of_string() {
    use ulua_unit_test::records::negation_fixture::NegationFixture;

    let mut fixture = NegationFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(arg: string) end
        local a: string & Not<"Hello">
        foo(a)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_negations_string_is_not_a_subtype_of_negated_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:40:type_infer_negations_string_is_not_a_subtype_of_negated_string`
  //! Source: `tests/TypeInfer.negations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.negations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.negations.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_negations_string_is_not_a_subtype_of_negated_string

  #[cfg(test)]
  #[test]
  fn type_infer_negations_string_is_not_a_subtype_of_negated_string() {
    use ulua_unit_test::records::negation_fixture::NegationFixture;

    let mut fixture = NegationFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(arg: string & Not<"hello">) end
        local a: string
        foo(a)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}
