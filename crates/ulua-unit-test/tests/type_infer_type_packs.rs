extern crate alloc;

use ulua_analysis::type_aliases::module_name_type::ModuleName;

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_cyclic_type_packs() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
--!nonstrict
_ += _(_,...)
repeat
_ += _(...)
until ... + _
"#,
    None,
  );

  let _result = fixture.check_string_optional_frontend_options(
    r#"
--!nonstrict
_ += _(_(...,...),_(...))
repeat
until _
"#,
    None,
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_detect_cyclic_typepacks() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type ( ... ) ( ) ;
        ( ... ) ( - - ... ) ( - ... )
        type = ( ... ) ;
        ( ... ) (  ) ( ... ) ;
        ( ... ) ""
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_dont_ice_if_a_type_pack_is_an_error() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        function f(s)
            print(s)
            return f
        end

        f("foo")("bar")
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_empty_varargs_should_return_nil_when_not_in_tail_position() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a, b = ..., 1
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_fuzz_typepack_iter_follow() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local _
local _ = _,_(),_(_)
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_fuzz_typepack_iter_follow_2() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function test(name, searchTerm)
    local found = string.find(name:lower(), searchTerm:lower())
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_generalize_expected_types_with_proper_scope() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _instantiate = ScopedFastFlag::new(&fflag::LuauInstantiateInSubtyping, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function f<TResult>(fn: () -> ...TResult): () -> ...TResult
            return function()
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_higher_order_function() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function apply(f, g, x)
            return f(g(x))
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "<a, b..., c...>((c...) -> (b...), (a) -> (c...), a) -> (b...)"
  } else {
    "<a, b..., c...>((b...) -> (c...), (a) -> (b...), a) -> (c...)"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("apply"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_infer_multi_return() {
  use ulua_analysis::{
    functions::{
      flatten_type_pack::flatten_type_pack_id, follow_type, get_type,
      to_string_to_string::to_string_type_id,
    },
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function take_two()
            return 2, 2
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let take_two_type = fixture.require_type_string("take_two");
  let take_two_type = get_type::get::<FunctionType>(take_two_type).expect("expected FunctionType");
  let (returns, tail) = flatten_type_pack_id(take_two_type.ret_types());

  assert_eq!(2, returns.len());
  assert_eq!("number", to_string_type_id(follow_type::follow(returns[0])));
  assert_eq!("number", to_string_type_id(follow_type::follow(returns[1])));
  assert!(tail.is_none());
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_last_element_of_return_statement_can_itself_be_a_pack() {
  use ulua_analysis::{
    functions::{
      flatten_type_pack::flatten_type_pack_id, follow_type, get_type,
      to_string_to_string::to_string_type_id,
    },
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function take_two()
            return 2, 2
        end

        function take_three()
            return 1, take_two()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let take_three_type = fixture.require_type_string("take_three");
  let take_three_type =
    get_type::get::<FunctionType>(take_three_type).expect("expected FunctionType");
  let (returns, tail) = flatten_type_pack_id(take_three_type.ret_types());

  assert_eq!(3, returns.len());
  assert_eq!("number", to_string_type_id(follow_type::follow(returns[0])));
  assert_eq!("number", to_string_type_id(follow_type::follow(returns[1])));
  assert_eq!("number", to_string_type_id(follow_type::follow(returns[2])));
  assert!(tail.is_none());
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_multiple_varargs_inference_are_not_confused() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(...)
            local a: string = ...

            return function(...)
                local b: number = ...
            end
        end

        f("foo", "bar")(1, 2)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_no_return_size_should_be_zero() {
  use ulua_analysis::{
    functions::{flatten_type_pack::flatten_type_pack_id, get_type},
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(a:any) return a end
        function g() return end
        function h() end

        g(h())
        f(g(),h())
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let f_type = fixture.require_type_string("f");
  let f_type = get_type::get::<FunctionType>(f_type).expect("expected FunctionType for f");
  assert_eq!(1, flatten_type_pack_id(f_type.ret_types()).0.len());

  let g_type = fixture.require_type_string("g");
  let g_type = get_type::get::<FunctionType>(g_type).expect("expected FunctionType for g");
  assert_eq!(0, flatten_type_pack_id(g_type.ret_types()).0.len());

  let h_type = fixture.require_type_string("h");
  let h_type = get_type::get::<FunctionType>(h_type).expect("expected FunctionType for h");
  assert_eq!(0, flatten_type_pack_id(h_type.ret_types()).0.len());
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_pack_tail_unification_check() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local a: () -> (number, ...string)
local b: () -> (number, ...boolean)
a = b
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'; \nit returns a tail of the variadic `boolean` in the latter type and `string` in the former type, and `boolean` is not a subtype of `string`"
  } else {
    "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'\ncaused by:\n  Expected this to be 'string', but got 'boolean'"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_parenthesized_varargs_returns_any() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local value

        local function f(...)
            value = ...
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "any",
    to_string_type_id(fixture.require_type_string("value"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_return_type_should_be_empty_if_nothing_is_returned() {
  use ulua_analysis::{
    functions::{flatten_type_pack::flatten_type_pack_id, get_type},
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f() end
        function g() return end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let f_type = fixture.require_type_string("f");
  let f_type = get_type::get::<FunctionType>(f_type).expect("expected FunctionType for f");
  assert_eq!(0, flatten_type_pack_id(f_type.ret_types()).0.len());

  let g_type = fixture.require_type_string("g");
  let g_type = get_type::get::<FunctionType>(g_type).expect("expected FunctionType for g");
  assert_eq!(0, flatten_type_pack_id(g_type.ret_types()).0.len());
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_self_and_varargs_should_work() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t = {}
        function t:f(...) end
        t:f(1)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_backwards_compatible() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type X<T> = () -> T
        type Y<T, U> = (T) -> U

        type A = X<(number)>
        type B = Y<(number), (boolean)>
        type C = Y<(number), boolean>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  for (name, expected) in [
    ("A", "() -> number"),
    ("B", "(number) -> boolean"),
    ("C", "(number) -> boolean"),
  ] {
    let ty = fixture
      .lookup_type(name)
      .unwrap_or_else(|| panic!("expected type alias {name}"));
    assert_eq!(expected, to_string_type_id(ty), "{name}");
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_export() {
  use alloc::string::String;

  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("Module/Types"),
    String::from(
      r#"
export type A<T, U = string> = { a: T, b: U }
export type B<T, U = T> = { a: T, b: U }
export type C<T, U = (T, T) -> string> = { a: T, b: U }
export type D<T, U = T, V = U> = { a: T, b: U, c: V }
export type E<T... = (string, number)> = { a: (T...) -> () }
export type F<T, U... = ...T> = { a: T, b: (U...) -> T }
export type G<T..., U... = ()> = { b: (U...) -> T... }
export type H<T... = ()> = { b: (T...) -> T... }
return {}
    "#,
    ),
  );

  let result_types = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/Types"), None);
  assert_eq!(0, result_types.errors.len(), "{:?}", result_types.errors);

  fixture.base.file_resolver.source.insert(
    String::from("Module/Users"),
    String::from(
      r#"
local Types = require(script.Parent.Types)

local a: Types.A<number>
local b: Types.B<number>
local c: Types.C<number>
local d: Types.D<number>
local e: Types.E<>
local eVoid: Types.E<()>
local f: Types.F<number>
local g: Types.G<...number>
local h: Types.H<>
    "#,
    ),
  );

  let result_users = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/Users"), None);
  assert_eq!(0, result_users.errors.len(), "{:?}", result_users.errors);

  for (name, expected) in [
    ("a", "A<number, string>"),
    ("b", "B<number, number>"),
    ("c", "C<number, (number, number) -> string>"),
    ("d", "D<number, number, number>"),
    ("e", "E<string, number>"),
    ("eVoid", "E<>"),
    ("f", "F<number, ...number>"),
    ("g", "G<...number, ()>"),
    ("h", "H<>"),
  ] {
    assert_eq!(
      expected,
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("Module/Users", name)
      ),
      "{name}"
    );
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_mixed_self() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U = T, V... = ...number, W... = (T, U, V...)> = { a: (T, U, V...) -> W... }
local a: Y<number>
local b: Y<number, string>
local c: Y<number, string, ...boolean>
local d: Y<number, string, ...boolean, ...() -> ()>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  for (name, expected) in [
    (
      "a",
      "Y<number, number, ...number, (number, number, ...number)>",
    ),
    (
      "b",
      "Y<number, string, ...number, (number, string, ...number)>",
    ),
    (
      "c",
      "Y<number, string, ...boolean, (number, string, ...boolean)>",
    ),
    ("d", "Y<number, string, ...boolean, ...() -> ()>"),
  ] {
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(name)),
      "{name}"
    );
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_chained() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U = T, V = U> = { a: T, b: U, c: V }

local a: Y<number>
local b: Y<number, string>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number, number, number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "Y<number, string, string>",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Y<T = T> = { a: T }
        local a: Y = { a = 2 }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!("Unknown type 'T'", to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors_2() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Y<T... = T...> = { a: (T...) -> () }
        local a: Y<>
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!("Unknown type 'T'", to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors_3() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Y<T = string, U... = ...string> = { a: (T) -> U... }
        local a: Y<...number>
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Type parameters must come before type pack parameters"
  } else {
    "Generic type 'Y<T, U...>' expects at least 1 type argument, but none are specified"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors_4() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Packed<T> = (T) -> T
        local a: Packed
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Generic type 'Packed<T>' expects 1 type argument, but none are specified"
  } else {
    "Type parameter list is required"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors_5() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Y<T, U = T, V> = { a: T }
        local a: Y<number>
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_errors_6() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Y<T..., U... = T..., V...> = { a: T }
        local a: Y<...number>
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_explicit() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U = string> = { a: T, b: U }

local a: Y<number, number> = { a = 2, b = 3 }
local b: Y<number> = { a = 2, b = "s" }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number, number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "Y<number, string>",
    to_string_type_id(fixture.require_type_string("b"))
  );

  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T = string> = { a: T }

local a: Y<number> = { a = 2 }
local b: Y<> = { a = "s" }
local c: Y = { a = "s" }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "Y<string>",
    to_string_type_id(fixture.require_type_string("b"))
  );
  assert_eq!(
    "Y<string>",
    to_string_type_id(fixture.require_type_string("c"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_pack_explicit() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T... = (string, number)> = { a: (T...) -> () }
local a: Y<>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<string, number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_pack_self_chained_tp() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T..., U... = T..., V... = U...> = { a: (T...) -> U..., b: (T...) -> V... }
local a: Y<number, string>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<(number, string), (number, string), (number, string)>",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_pack_self_tp() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T..., U... = T...> = { a: (T...) -> U... }
local a: Y<number, string>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<(number, string), (number, string)>",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_pack_self_ty() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U... = ...T> = { a: T, b: (U...) -> T }

local a: Y<number>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number, ...number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_self() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U = T> = { a: T, b: U }

local a: Y<number> = { a = 2, b = 3 }
local b: Y<string> = { a = "h", b = "s" }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number, number>",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "Y<string, string>",
    to_string_type_id(fixture.require_type_string("b"))
  );

  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T, U = (T, T) -> string> = { a: T, b: U }

local a: Y<number>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<number, (number, number) -> string>",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_default_type_skip_brackets() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T... = ...string> = (T...) -> number
local a: Y
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(...string) -> number",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_defaults_confusing_types() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A<T, U = T, V... = ...any, W... = V...> = (T, V...) -> (U, W...)
type B = A<string, (number)>
type C = A<string, (number), (boolean)>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let b = fixture.lookup_type("B").expect("expected type alias B");
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "(string, ...any) -> (number, ...any)",
    to_string_type_id_to_string_options(b, &mut opts)
  );

  let c = fixture.lookup_type("C").expect("expected type alias C");
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "(string, boolean) -> (number, boolean)",
    to_string_type_id_to_string_options(c, &mut opts)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_defaults_recursive_type() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type F<K = string, V = (K) -> ()> = (K) -> V
type R = { m: F<R> }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let r = fixture.lookup_type("R").expect("expected type alias R");
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "t1 where t1 = { m: (t1) -> (t1) -> () }",
    to_string_type_id_to_string_options(r, &mut opts)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_instantiated_but_missing_parameter_list() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Packed<T...> = (T...) -> T...
local a: Packed
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Generic type 'Packed<T...>' expects 1 type pack argument, but none are specified"
  } else {
    "Type parameter list is required"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_pack_explicit() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type X<T...> = (T...) -> (T...)

type A<S...> = X<(S...)>
type B = X<()>
type C = X<(number)>
type D = X<(number, string)>
type E = X<(...number)>
type F = X<(string, ...number)>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  for (name, expected) in [
    ("A", "(S...) -> (S...)"),
    ("B", "() -> ()"),
    ("C", "(number) -> number"),
    ("D", "(number, string) -> (number, string)"),
    ("E", "(...number) -> (...number)"),
    ("F", "(string, ...number) -> (string, ...number)"),
  ] {
    let ty = fixture
      .lookup_type(name)
      .unwrap_or_else(|| panic!("expected type alias {name}"));
    assert_eq!(expected, to_string_type_id(ty), "{name}");
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_pack_explicit_multi() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T..., U...> = (T...) -> (U...)

type A = Y<(number, string), (boolean)>
type B = Y<(), ()>
type C<S...> = Y<...string, (number, S...)>
type D<X...> = Y<X..., (number, string, X...)>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  for (name, expected) in [
    ("A", "(number, string) -> boolean"),
    ("B", "() -> ()"),
    ("C", "(...string) -> (number, S...)"),
    ("D", "(X...) -> (number, string, X...)"),
  ] {
    let ty = fixture
      .lookup_type(name)
      .unwrap_or_else(|| panic!("expected type alias {name}"));
    assert_eq!(expected, to_string_type_id(ty), "{name}");
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_pack_explicit_multi_tostring() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T..., U...> = { f: (T...) -> (U...) }

local a: Y<(number, string), (boolean)>
local b: Y<(), ()>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Y<(number, string), (boolean)>",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "Y<(), ()>",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_pack_multi() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Y<T..., U...> = (T...) -> (U...)
type A<S...> = Y<S..., S...>
type B<S...> = Y<(number, ...string), S...>

type Z<T, U...> = (T) -> (U...)
type E<S...> = Z<number, S...>
type F<S...> = Z<number, (string, S...)>

type W<T, U..., V...> = (T, U...) -> (T, V...)
type H<S..., R...> = W<number, S..., R...>
type I<S..., R...> = W<number, (string, S...), R...>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  for (name, expected) in [
    ("A", "(S...) -> (S...)"),
    ("B", "(number, ...string) -> (S...)"),
    ("E", "(number) -> (S...)"),
    ("F", "(number) -> (string, S...)"),
    ("H", "(number, S...) -> (number, R...)"),
    ("I", "(number, string, S...) -> (number, R...)"),
  ] {
    let ty = fixture
      .lookup_type(name)
      .unwrap_or_else(|| panic!("expected type alias {name}"));
    assert_eq!(expected, to_string_type_id(ty), "{name}");
  }
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_pack_variadic() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type X<T...> = (T...) -> (string, T...)

type D = X<...number>
type E = X<(number, ...string)>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let d = fixture.lookup_type("D").expect("expected type alias D");
  let e = fixture.lookup_type("E").expect("expected type alias E");
  assert_eq!("(...number) -> (string, ...number)", to_string_type_id(d));
  assert_eq!(
    "(number, ...string) -> (string, number, ...string)",
    to_string_type_id(e)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_packs() {
  use ulua_analysis::{
    functions::{
      get_type,
      to_string_to_string::{
        to_string_type_id, to_string_type_id_to_string_options,
        to_string_type_pack_id_to_string_options,
      },
    },
    records::{table_type::TableType, to_string_options::ToStringOptions},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Packed<T...> = (T...) -> T...
local a: Packed<>
local b: Packed<number>
local c: Packed<string, number>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let packed = fixture
    .lookup_type("Packed")
    .expect("expected type alias Packed");
  assert_eq!("(T...) -> (T...)", to_string_type_id(packed));
  assert_eq!(
    "() -> ()",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "(number) -> number",
    to_string_type_id(fixture.require_type_string("b"))
  );
  assert_eq!(
    "(string, number) -> (string, number)",
    to_string_type_id(fixture.require_type_string("c"))
  );

  let result = fixture.check_string_optional_frontend_options(
    r#"
-- (U..., T) cannot be parsed right now
type Packed<T, U...> = { f: (a: T, U...) -> (T, U...) }
local a: Packed<number>
local b: Packed<string, number>
local c: Packed<string, number, boolean>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let packed = fixture
    .lookup_type("Packed")
    .expect("expected type alias Packed");
  assert_eq!("Packed<T, U...>", to_string_type_id(packed));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ f: (T, U...) -> (T, U...) }",
    to_string_type_id_to_string_options(packed, &mut opts)
  );

  let a = fixture.require_type_string("a");
  let a_table = get_type::get::<TableType>(a).expect("expected TableType for a");
  assert_eq!("Packed<number>", to_string_type_id(a));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ f: (number) -> number }",
    to_string_type_id_to_string_options(a, &mut opts)
  );

  assert_eq!(1, a_table.instantiated_type_params.len());
  assert_eq!(1, a_table.instantiated_type_pack_params.len());
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "number",
    to_string_type_id_to_string_options(a_table.instantiated_type_params[0], &mut opts)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "()",
    to_string_type_pack_id_to_string_options(a_table.instantiated_type_pack_params[0], &mut opts)
  );

  let b = fixture.require_type_string("b");
  let b_table = get_type::get::<TableType>(b).expect("expected TableType for b");
  assert_eq!("Packed<string, number>", to_string_type_id(b));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ f: (string, number) -> (string, number) }",
    to_string_type_id_to_string_options(b, &mut opts)
  );

  assert_eq!(1, b_table.instantiated_type_params.len());
  assert_eq!(1, b_table.instantiated_type_pack_params.len());
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "string",
    to_string_type_id_to_string_options(b_table.instantiated_type_params[0], &mut opts)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "number",
    to_string_type_pack_id_to_string_options(b_table.instantiated_type_pack_params[0], &mut opts)
  );

  let c = fixture.require_type_string("c");
  let c_table = get_type::get::<TableType>(c).expect("expected TableType for c");
  assert_eq!("Packed<string, number, boolean>", to_string_type_id(c));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ f: (string, number, boolean) -> (string, number, boolean) }",
    to_string_type_id_to_string_options(c, &mut opts)
  );

  assert_eq!(1, c_table.instantiated_type_params.len());
  assert_eq!(1, c_table.instantiated_type_pack_params.len());
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "string",
    to_string_type_id_to_string_options(c_table.instantiated_type_params[0], &mut opts)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "number, boolean",
    to_string_type_pack_id_to_string_options(c_table.instantiated_type_pack_params[0], &mut opts)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_packs_errors() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut check_error = |source: &str, expected: &str| {
    let result = fixture.check_string_optional_frontend_options(source, None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  };

  check_error(
    r#"
type Packed<T, U, V...> = (T, U) -> (V...)
local b: Packed<number>
    "#,
    "Generic type 'Packed<T, U, V...>' expects at least 2 type arguments, but only 1 is specified",
  );

  check_error(
    r#"
type Packed<T, U> = (T, U) -> ()
type B<X...> = Packed<number, string, X...>
    "#,
    "Generic type 'Packed<T, U>' expects 0 type pack arguments, but 1 is specified",
  );

  check_error(
    r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other<S...> = Packed<S..., string>
    "#,
    "Type parameters must come before type pack parameters",
  );

  check_error(
    r#"
type Packed<T, U> = (T) -> U
type Other<S...> = Packed<number, S...>
    "#,
    "Generic type 'Packed<T, U>' expects 2 type arguments, but only 1 is specified",
  );

  check_error(
    r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other = Packed<>
    "#,
    "Generic type 'Packed<T..., U...>' expects 2 type pack arguments, but none are specified",
  );

  check_error(
    r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other = Packed<number, string>
    "#,
    "Generic type 'Packed<T..., U...>' expects 2 type pack arguments, but only 1 is specified",
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_packs_import() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options},
    records::to_string_options::ToStringOptions,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
export type Packed<T, U...> = { a: T, b: (U...) -> () }
return {}
    "#,
    ),
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture.base.check_string_optional_frontend_options(
    r#"
local Import = require(game.A)
local a: Import.Packed<number>
local b: Import.Packed<string, number>
local c: Import.Packed<string, number, boolean>
local d: { a: typeof(c) }
    "#,
    None,
  );
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let packed = fixture
    .base
    .lookup_imported_type("Import", "Packed")
    .expect("expected imported type Import.Packed");
  assert_eq!("Packed<T, U...>", to_string_type_id(packed));

  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: T, b: (U...) -> () }",
    to_string_type_id_to_string_options(packed, &mut opts)
  );

  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: number, b: () -> () }",
    to_string_type_id_to_string_options(fixture.base.require_type_string("a"), &mut opts,)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: string, b: (number) -> () }",
    to_string_type_id_to_string_options(fixture.base.require_type_string("b"), &mut opts,)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: string, b: (number, boolean) -> () }",
    to_string_type_id_to_string_options(fixture.base.require_type_string("c"), &mut opts,)
  );
  assert_eq!(
    "{ a: Packed<string, number, boolean> }",
    to_string_type_id(fixture.base.require_type_string("d"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_alias_type_packs_nested() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Packed1<T...> = (T...) -> (T...)
type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)
type Packed3<T...> = (Packed2<T...>, T...) -> (Packed2<T...>, T...)
type Packed4<T...> = (Packed3<T...>, T...) -> (Packed3<T...>, T...)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let packed = fixture
    .lookup_type("Packed4")
    .expect("expected type alias Packed4");
  assert_eq!(
    "((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...) -> \
((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...)",
    to_string_type_id(packed)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_pack_hidden_free_tail_infinite_growth() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!nonstrict
if _ then
    _[function(l0)end],l0 = _
elseif _ then
    return l0(nil)
elseif 1 / l0(nil) then
elseif _ then
    return #_,l0()
end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_pack_type_parameters() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options},
    records::to_string_options::ToStringOptions,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
export type Packed<T, U...> = { a: T, b: (U...) -> () }
return {}
    "#,
    ),
  );

  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local Import = require(game.A)
type Alias<S, T, R...> = Import.Packed<S, (T, R...)>
local a: Alias<string, number, boolean>

type B<X...> = Import.Packed<string, X...>
type C<X...> = Import.Packed<string, (number, X...)>
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let alias = fixture
    .base
    .lookup_type("Alias")
    .expect("expected type alias Alias");
  assert_eq!("Alias<S, T, R...>", to_string_type_id(alias));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: S, b: (T, R...) -> () }",
    to_string_type_id_to_string_options(alias, &mut opts)
  );

  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: string, b: (number, boolean) -> () }",
    to_string_type_id_to_string_options(fixture.base.require_type_string("a"), &mut opts,)
  );

  let b = fixture
    .base
    .lookup_type("B")
    .expect("expected type alias B");
  assert_eq!("B<X...>", to_string_type_id(b));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: string, b: (X...) -> () }",
    to_string_type_id_to_string_options(b, &mut opts)
  );

  let c = fixture
    .base
    .lookup_type("C")
    .expect("expected type alias C");
  assert_eq!("C<X...>", to_string_type_id(c));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ a: string, b: (number, X...) -> () }",
    to_string_type_id_to_string_options(c, &mut opts)
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = if !fflag::DebugLuauForceOldSolver.get() {
    Some(ScopedFastFlag::new(
      &fflag::LuauInstantiateInSubtyping,
      true,
    ))
  } else {
    None
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        local function wrapReject<TArg, TResult>(fn: (self: any, ...TArg) -> ...TResult): (self: any, ...TArg) -> ...TResult
            return function(self, ...)
                local arguments = { ... }
                local ok, result = pcall(function()
                    return fn(self, table.unpack(arguments))
                end)
                return result
            end
        end
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_type_param_overflow() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Two<T,U> = { a: T, b: U }
        local x: Two<number, string, number> = { a = 1, b = 'c' }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_unify_variadic_tails_in_arguments() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(...: string): number
            return 1
        end

        function bar(...: number): number
            return foo(...)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'string', but got 'number'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_unify_variadic_tails_in_arguments_free() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo<T...>(...: T...): T...
            return ...
        end

        function bar(...: number): boolean
            return foo(...)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Expected this to be 'boolean', but got '...number'; \nit has a tail of `...number`, which is not a subtype of `boolean`"
  } else {
    "Expected this to be 'boolean', but got 'number'"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack() {
  use ulua_analysis::functions::{begin_type_pack::begin, end_type_pack::end};
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function a(x) return 1 end
        a(...)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let main_module = fixture.get_main_module(false);
  let main_module = unsafe { &*main_module };
  assert!(main_module.has_module_scope());

  let module_scope = main_module.get_module_scope();
  let vararg_pack = module_scope
    .vararg_pack
    .expect("expected module scope vararg pack");

  let mut iter = begin(vararg_pack);
  let end_iter = end(vararg_pack);

  assert!(iter != end_iter);
  iter.advance();
  assert!(iter == end_iter);
  assert_eq!(None, iter.tail());
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_varargs_inference_through_multiple_scopes() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(...)
            do
                local a: string = ...
                local b: number = ...
            end
        end

        f("foo")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_variadic_argument_tail() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local _ = function():((...any)->(...any),()->())
    return function() end, function() end
end
for y in _() do
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_variadic_pack_syntax() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        local function foo(...: number)
        end

        foo(1, 2, 3, 4, 5, 6)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(...number) -> ()",
    to_string_type_id(fixture.require_type_string("foo"))
  );
}

// Source: `tests/TypeInfer.typePacks.test.cpp`
#[test]
fn type_infer_type_packs_variadic_packs() {
  use ulua_analysis::{
    functions::{
      add_global_binding_builtin_definitions::add_global_binding_builtin_definitions, follow_type,
      freeze::freeze, unfreeze::unfreeze,
    },
    records::{
      function_type::FunctionType, type_mismatch::TypeMismatch,
      variadic_type_pack::VariadicTypePack,
    },
  };
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();

  let number_type = fixture.get_builtins().number_type;
  let string_type = fixture.get_builtins().string_type;

  let (foo_type, bar_type) = {
    let frontend = fixture.get_frontend();
    let arena = frontend.globals.global_types_mut();

    unfreeze(arena);

    let list_of_numbers = arena.add_type_pack_t(VariadicTypePack::new(number_type));
    let list_of_strings = arena.add_type_pack_t(VariadicTypePack::new(string_type));

    let foo_rets = arena.add_type_pack_initializer_list_type_id(&[number_type]);
    let foo_type = arena.add_type(FunctionType::function_type_new(
      list_of_numbers,
      foo_rets,
      None,
      false,
    ));

    let bar_args = arena.add_type_pack_vector_type_id_optional_type_pack_id(
      alloc::vec![number_type],
      Some(list_of_strings),
    );
    let bar_rets = arena.add_type_pack_initializer_list_type_id(&[number_type]);
    let bar_type = arena.add_type(FunctionType::function_type_new(
      bar_args, bar_rets, None, false,
    ));

    (foo_type, bar_type)
  };

  {
    let frontend = fixture.get_frontend();
    add_global_binding_builtin_definitions(&mut frontend.globals, "foo", foo_type, "@test");
    add_global_binding_builtin_definitions(&mut frontend.globals, "bar", bar_type, "@test");
    freeze(frontend.globals.global_types_mut());
  }

  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        foo(1, 2, 3, "foo")
        bar(1, "foo", "bar", 3)
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    Location {
      begin: Position {
        line: 3,
        column: 21
      },
      end: Position {
        line: 3,
        column: 26
      },
    },
    result.errors[0].location
  );
  assert_eq!(
    Location {
      begin: Position {
        line: 4,
        column: 29
      },
      end: Position {
        line: 4,
        column: 30
      },
    },
    result.errors[1].location
  );

  let first = type_error_data_ref::<TypeMismatch>(&result.errors[0])
    .expect("expected first error to be TypeMismatch");
  assert_eq!(number_type, follow_type::follow(first.wanted_type));
  assert_eq!(string_type, follow_type::follow(first.given_type));

  let second = type_error_data_ref::<TypeMismatch>(&result.errors[1])
    .expect("expected second error to be TypeMismatch");
  assert_eq!(string_type, follow_type::follow(second.wanted_type));
  assert_eq!(number_type, follow_type::follow(second.given_type));
}

// Source: `tests/TypeInfer.typePacks.test.cpp:1149`
#[test]
fn type_infer_type_packs_detect_cyclic_typepacks_2() {
  use ulua_analysis::{
    functions::to_string_error::to_string_type_error,
    records::function_exits_without_returning::FunctionExitsWithoutReturning,
    type_aliases::type_error_data::TypeErrorData,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function _(l0:((typeof((pcall)))|((((t0)->())|(typeof(-67108864)))|(any)))|(any),...):(((typeof(0))|(any))|(any),typeof(-67108864),any)
            xpcall(_,_,_)
            _(_,_,_)
        end
    "#,
    None,
  );

  // cpp 侧先 ignoreMissingAnnotations（滤除 TypeAnnotationRequired）再计数断言
  let errors: Vec<_> = result
    .errors
    .iter()
    .filter(|e| !matches!(&e.data, TypeErrorData::TypeAnnotationRequired(_)))
    .collect();
  assert_eq!(2, errors.len(), "{:?}", result.errors);
  assert_eq!("Unknown type 't0'", to_string_type_error(errors[0]));
  assert!(
    type_error_data_ref::<FunctionExitsWithoutReturning>(errors[1]).is_some(),
    "expected second error to be FunctionExitsWithoutReturning, got {:?}",
    errors[1]
  );
}
// 缺口（未移植，对照 `tests/TypeInfer.typePacks.test.cpp`，共 4 例）：
// - function_return_count_mismatch_reports_expected_return_pack（:978）、
//   function_return_count_mismatch_through_union_reports_expected_return_pack
//   （:1009）、function_return_count_mismatch_through_intersection_reports_
//   expected_return_pack（:1026）、nested_function_return_count_mismatch_
//   preserves_the_function_context（:1042）——依赖 FFlag
//   `LuauNewTypePathErrorMessages` 与 `renderTypePathPrefix` 新报错渲染系统，
//   ulua 未实现（同 type_path.rs 尾注记 8 例阻清单；type_infer_tables.rs:5068
//   亦注记该 flag 未同步）。
