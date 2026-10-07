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

// 缺口（未移植，对照 `tests/TypeInfer.negations.test.cpp`，共 2 例；tw-10 复核）：
// - subtyping_path_is_valid_for_union（:84）/ subtype_path_is_valid_for_intersections（:102）
//   —— cpp 依赖 FFlag `LuauNewTypePathErrorMessages` + `LuauFixSuperNegationTypePaths`
//   （Rust fflag.rs 均未定义）。tw-10 去旗标探针实测：两例均报 2 错（cpp 1 错），
//   首错为 InternalError("Subtyping test returned a reasoning with an invalid path")
//   而非新式 type-path 文案（"Expected this to be '~(false?)', ... `false` cannot be
//   `~(false?)`"）。否定类型的 subtyping 路径重构未同步，文案断言不可达。
