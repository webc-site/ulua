extern crate alloc;

// Source: `tests/TypeInfer.negations.test.cpp`
#[test]
fn type_infer_negations_cofinite_strings_can_be_compared_for_equality() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
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
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(string) -> string",
    to_string_type_id(fixture.require_type_string("f"))
  );
}

// Source: `tests/TypeInfer.negations.test.cpp`
#[test]
fn type_infer_negations_compare_cofinite_strings() {
  use ulua_unit_test::records::negation_fixture::NegationFixture;

  let mut fixture = NegationFixture::default();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local u : Not<"a">
local v : "b"
if u == v then
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.negations.test.cpp`
#[test]
fn type_infer_negations_negated_string_is_a_subtype_of_string() {
  use ulua_unit_test::records::negation_fixture::NegationFixture;

  let mut fixture = NegationFixture::default();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function foo(arg: string) end
        local a: string & Not<"Hello">
        foo(a)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.negations.test.cpp`
#[test]
fn type_infer_negations_string_is_not_a_subtype_of_negated_string() {
  use ulua_unit_test::records::negation_fixture::NegationFixture;

  let mut fixture = NegationFixture::default();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function foo(arg: string & Not<"hello">) end
        local a: string
        foo(a)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
