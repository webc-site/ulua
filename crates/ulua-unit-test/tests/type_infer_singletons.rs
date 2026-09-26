extern crate alloc;

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_bidirectionally_infer_indexers_errored() {
  use ulua_analysis::{functions::get_error::get_type_error, records::type_mismatch::TypeMismatch};
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        export type T = "foo" | "bar" | "toto"

        local getOpposite: { [number]: T } = {
            ["foo"] = "bar",
            ["bar"] = "toto",
            ["toto"] = "foo"
        }
    "#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  for error in &result.errors {
    get_type_error::<TypeMismatch>(error).expect("expected TypeMismatch");
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_bool_singleton_subtype() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: true = true
        local b: boolean = a
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_bool_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: true = true
        local b: false = false
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_bool_singletons_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: true = false
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'true', but got 'false'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_cli_163481_any_indexer_pushes_type() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        type test = "A"
        type test2 = "A"|"B"|"C"

        local t: { [any]: test } = { A = "A" }

        local t2: { [any]: test2 } = {
            A = "A",
            B = "B",
            C = "C"
        }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_cli_184125() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type MyTypeA = {Value: true}
        type MyTypeB = {Value: false}

        local function Func(input: number) : (MyTypeA | MyTypeB)
            if input == 1 then
                return {Value = true}
            else
                return {Value = false}
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_enums_using_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type MyEnum = "foo" | "bar" | "baz"
        local a : MyEnum = "foo"
        local b : MyEnum = "bar"
        local c : MyEnum = "baz"
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_enums_using_singletons_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type MyEnum = "foo" | "bar" | "baz"
        local a : MyEnum = "bang"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    // dev 折叠波次误裁了两处行尾空格:本路径真实输出为 `"; " + "\nthis is because "`
    // （Reasonings::fmt 与 operator_call_54 的 `"; "` 拼接,cpp 同源语义),golden 须含行尾空格。
    let expected = r#"Expected this to be '"bar" | "baz" | "foo"', but got '"bang"'; 
this is because 
	 * the 1st component of the union is `"foo"`, and `"bang"` is not a subtype of `"foo"`
	 * the 2nd component of the union is `"bar"`, and `"bang"` is not a subtype of `"bar"`
	 * the 3rd component of the union is `"baz"`, and `"bang"` is not a subtype of `"baz"`"#;
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  } else {
    assert_eq!(
      r#"Expected this to be '"bar" | "baz" | "foo"', but got '"bang"'; none of the union options are compatible"#,
      to_string_type_error(&result.errors[0])
    );
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_enums_using_singletons_subtyping() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type MyEnum1 = "foo" | "bar"
        type MyEnum2 = MyEnum1 | "baz"
        local a : MyEnum1 = "foo"
        local b : MyEnum2 = a
        local c : string = b
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_error_detailed_tagged_union_mismatch_bool() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Good = { success: true, result: string }
type Bad = { success: false, error: string }
type Result = Good | Bad

local a: Result = { success = false, result = 'something' }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Table type '{ result: string, success: false }' not compatible with type 'Bad' because the former is missing field 'error'",
      to_string_type_error(&result.errors[0])
    );
  } else {
    let expected = "Expected this to be 'Bad | Good', but got 'a'
caused by:
  None of the union options are compatible. For example:
Table type 'a' not compatible with type 'Bad' because the former is missing field 'error'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_error_detailed_tagged_union_mismatch_string() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Cat = { tag: 'cat', catfood: string }
type Dog = { tag: 'dog', dogfood: string }
type Animal = Cat | Dog

local a: Animal = { tag = 'cat', cafood = 'something' }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      r#"Table type '{ cafood: string, tag: "cat" }' not compatible with type 'Cat' because the former is missing field 'catfood'"#,
      to_string_type_error(&result.errors[0])
    );
  } else {
    let expected = "Expected this to be 'Cat | Dog', but got 'a'
caused by:
  None of the union options are compatible. For example:
Table type 'a' not compatible with type 'Cat' because the former is missing field 'catfood'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_function_args_infer_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!strict
type Phase = "A" | "B" | "C"
local function f(e : Phase) : number
    return 0
end
local e = f("B")
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_function_call_with_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(a: true, b: "foo") end
        f(true, "foo")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_function_call_with_singletons_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(a: true, b: "foo") end
        f(true, "bar")
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    r#"Expected this to be '"foo"', but got '"bar"'"#,
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_functions_are_not_to_be_widened() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(my_enum: "A" | "B") end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#"("A" | "B") -> ()"#,
    to_string_type_id(fixture.require_type_string("foo"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_if_then_else_expression_singleton_options() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
      r#"
type Cat = { tag: 'cat', catfood: string }
type Dog = { tag: 'dog', dogfood: string }
type Animal = Cat | Dog

local a: Animal = if true then { tag = 'cat', catfood = 'something' } else { tag = 'dog', dogfood = 'other' }
    "#,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_indexer_can_be_union_of_singletons() {
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Target = "A" | "B"

        type Test = {[Target]: number}

        local test: Test = {}

        test.A = 2
        test.C = 4
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(8u32, result.errors[0].location.begin.line);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_indexing_on_string_singletons() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: string = "hi"
        if a == "hi" then
            local x = a:byte()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 22
    }))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_indexing_on_union_of_string_singletons() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: string = "hi"
        if a == "hi" or a == "bye" then
            local x = a:byte()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""bye" | "hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 22
    }))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_no_widening_from_callsites() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Direction = "North" | "East" | "West" | "South"

        local function direction(): Direction
            return "North"
        end

        local d: Direction = direction()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_oss_1773() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        export type T = "foo" | "bar" | "toto"

        local object: T = "foo"

        local getOpposite: {[T]: T} = {
            ["foo"] = "bar",
            ["bar"] = "toto",
            ["toto"] = "foo"
        }

        local function hello()
            local x = getOpposite[object]

            if x then
                object = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_oss_2010() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo<T>(my_enum: "" | T): T
            return my_enum :: T
        end

        local var = foo("meow")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""meow""#,
    to_string_type_id(fixture.require_type_string("var"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_oss_2010_but_with_booleans() {
  use ulua_analysis::{
    functions::{get_error::get_type_error, to_string_to_string::to_string_type_id},
    records::type_mismatch::TypeMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo<T>(my_enum: true | T): T
            return my_enum :: T
        end

        local function bar<T>(my_enum: true & T): T
            return my_enum :: T
        end

        local var1 = foo(true)
        local var2 = foo(false)

        local var3 = bar(true)
        local var4 = bar(false)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("false & true", to_string_type_id(err.wanted_type));
  assert_eq!("false", to_string_type_id(err.given_type));

  assert_eq!(
    "unknown",
    to_string_type_id(fixture.require_type_string("var1"))
  );
  assert_eq!(
    "false",
    to_string_type_id(fixture.require_type_string("var2"))
  );
  assert_eq!(
    "true",
    to_string_type_id(fixture.require_type_string("var3"))
  );
  assert_eq!(
    "false",
    to_string_type_id(fixture.require_type_string("var4"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_oss_2018() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
      r#"
        local rule: { rule: "AppendTextComment" } | { rule: "Other" } = { rule = "AppendTextComment" }
    "#,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_overloaded_function_call_with_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(a, b) end
        local g : ((true, string) -> ()) & ((false, number) -> ()) = (f::any)
        g(true, "foo")
        g(false, 37)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_overloaded_function_call_with_singletons_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(g: ((true, string) -> ()) & ((false, number) -> ()))
            g(true, 37)
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "None of the overloads for function that accept 2 arguments are compatible.",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Available overloads: (true, string) -> (); and (false, number) -> ()",
      to_string_type_error(&result.errors[1])
    );
  } else {
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Other overloads are also not viable: (false, number) -> ()",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_overloaded_function_resolution_singleton_parameters() {
  use ulua_analysis::{
    functions::{get_type, to_string_to_string::to_string_type_id},
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = ("A") -> string
        type B = ("B") -> number

        local function foo(f: A & B)
            return f("A"), f("B")
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let t = fixture.require_type_string("foo");
  get_type::get::<FunctionType>(t).expect("expected foo to be a function type");
  assert_eq!(
    r#"((("A") -> string) & (("B") -> number)) -> (string, number)"#,
    to_string_type_id(t)
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_parametric_tagged_union_alias() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Ok<T> = {success: true, result: T}
        type Err<T> = {success: false, error: T}
        type Result<O, E> = Ok<O> | Err<E>

        local a : Result<string, number> = {success = false, result = "hotdogs"}
        -- local b : Result<string, number> = {success = true, result = "hotdogs"}
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Table type '{ result: string, success: false }' not compatible with type 'Err<number>' because the former is missing field 'error'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_return_type_of_f_is_not_widened() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(f, x): "hello"? -- anyone there?
            return if x == "hi"
                then f(x)
                else nil
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 23
    }))
  );
  assert_eq!(
    r#"<a, b, c...>((string) -> (a, c...), b) -> "hello"?"#,
    to_string_type_id(fixture.require_type_string("foo"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_singleton_type_mismatch_via_variable() {
  use ulua_analysis::{functions::get_error::get_type_error, records::type_mismatch::TypeMismatch};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local c = "c"
        local x: "a" = c
        local y: "a" | "b" = c
        local z: "a"? = c
        local w: "a" | "b" = "c"
    "#,
    None,
  );

  assert_eq!(4, result.errors.len(), "{:?}", result.errors);
  for error in &result.errors {
    get_type_error::<TypeMismatch>(error).expect("expected TypeMismatch");
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_singletons_stick_around_under_assignment() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Foo = {
            kind: "Foo",
        }

        local foo = (nil :: any) :: Foo

        print(foo.kind == "Bar") -- type of equality refines to `false`
        local kind = foo.kind
        print(kind == "Bar") -- type of equality refines to `false`
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singleton_function_call() {
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x = "a"
        function f(x: "a") end
        f(x)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singleton_subtype() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: "foo" = "foo"
        local b: string = a
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singleton_subtype_multi_assignment() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: "foo" = "foo"
        local b: string, c: number = a, 10
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: "foo" = "foo"
        local b: "bar" = "bar"
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singletons_escape_chars() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: "\n" = "\000\r"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    r#"Expected this to be '"\n"', but got '"\000\r"'"#,
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_string_singletons_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: "foo" = "bar"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    r#"Expected this to be '"foo"', but got '"bar"'"#,
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_has_a_boolean() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local t={a=1,b=false}
    "#,
    None,
  );

  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "{ a: number, b: boolean }"
  } else {
    "{| a: number, b: boolean |}"
  };
  assert_eq!(
    expected,
    to_string_type_id_to_string_options(fixture.require_type_string("t"), &mut opts)
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_literal_with_singleton_union_values() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t1: {[string]: "a" | "b"} = { a = "a", b = "b" }
        local t2: {[string]: "a" | true} = { a = "a", b = true }
        local t3: {[string]: "a" | nil} = { a = "a" }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_properties_alias_or_parens_is_indexer() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        type S = "bar"
        type T = {
            [("foo")] : number,
            [S] : string,
        }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Cannot have more than one table indexer",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_properties_singleton_strings() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        type T = {
            ["foo"] : number,
            ["$$bar"] : string,
            baz : boolean
        }
        local t: T =  {
            ["foo"] = 37,
            ["$$bar"] = "hi",
            baz = true
        }
        local a: number = t.foo
        local b: string = t["$$bar"]
        local c: boolean = t.baz
        t.foo = 5
        t["$$bar"] = "lo"
        t.baz = false
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_properties_singleton_strings_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        type T = {
            ["$$bar"] : string,
        }
        local t: T =  {
            ["$$bar"] = "hi",
        }
        t["$$bar"] = 5
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'string', but got 'number'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_table_properties_type_error_escapes() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local x: { ["<>"] : number }
        x = { ["\n"] = 5 }
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    r#"Table type '{ ["\n"]: number }' not compatible with type '{ ["<>"]: number }' because the former is missing field '<>'"#,
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_tagged_union_in_ternary() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Result = { type: "ok", value: unknown } | { type: "error" }

        local function coinflip(): boolean return true end

        local function readFromDB(): Result
            return if coinflip() then { type = "ok", value = 42 } else { type = "error" }
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_tagged_unions_immutable_tag() {
  use ulua_analysis::{
    enums::reason::Reason,
    functions::{get_error::get_type_error, to_string_to_string::to_string_type_id},
    records::cannot_assign_to_never::CannotAssignToNever,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Dog = { tag: "Dog", howls: boolean }
        type Cat = { tag: "Cat", meows: boolean }
        type Animal = Dog | Cat
        local a: Animal = { tag = "Cat", meows = true }
        a.tag = "Dog"
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "expected errors");
  if !fflag::DebugLuauForceOldSolver.get() {
    let tm = get_type_error::<CannotAssignToNever>(&result.errors[0])
      .expect("expected CannotAssignToNever");
    assert_eq!(fixture.get_builtins().string_type, tm.rhs_type());
    assert_eq!(Reason::PropertyNarrowed, tm.reason());
    assert_eq!(2, tm.cause().len());
    assert_eq!(r#""Dog""#, to_string_type_id(tm.cause()[0]));
    assert_eq!(r#""Cat""#, to_string_type_id(tm.cause()[1]));
  }
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_tagged_unions_using_singletons() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Dog = { tag: "Dog", howls: boolean }
        type Cat = { tag: "Cat", meows: boolean }
        type Animal = Dog | Cat
        local a : Dog = { tag = "Dog", howls = true }
        local b : Animal = { tag = "Cat", meows = true }
        local c : Animal = a
        c = b
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_tagged_unions_using_singletons_mismatch() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Dog = { tag: "Dog", howls: boolean }
        type Cat = { tag: "Cat", meows: boolean }
        type Animal = Dog | Cat
        local a : Animal = { tag = "Cat", howls = true }
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "expected errors");
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_taking_the_length_of_string_singleton() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: string = "hi"
        if a == "hi" then
            local x = #a
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_taking_the_length_of_union_of_string_singleton() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: string = "hi"
        if a == "hi" or a == "bye" then
            local x = #a
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""bye" | "hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_widen_the_supertype_if_it_is_free_and_subtype_has_singleton() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(f, x)
            if x == "hi" then
                f(x)
                f("foo")
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""hi""#,
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 18
    }))
  );
  assert_eq!(
    "<a, b...>((string) -> (b...), a) -> ()",
    to_string_type_id(fixture.require_type_string("foo"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_widening_happens_almost_everywhere() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo: "foo" = "foo"
        local copy = foo
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    r#""foo""#
  } else {
    "string"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("copy"))
  );
}

// Source: `tests/TypeInfer.singletons.test.cpp`
#[test]
fn type_infer_singletons_widening_happens_almost_everywhere_except_for_tables() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Cat = {tag: "Cat", meows: boolean}
        type Dog = {tag: "Dog", barks: boolean}
        type Animal = Cat | Dog

        local function f(tag: "Cat" | "Dog"): Animal?
            if tag == "Cat" then
                local result = {tag = tag, meows = true}
                return result
            elseif tag == "Dog" then
                local result = {tag = tag, barks = true}
                return result
            else
                return nil
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.singletons.test.cpp:866`
//
// 形参注解 `typeof("hello")` 把单例阻塞在函数边界内，调用点传 "hello"
// 字面量必须无错误。
#[test]
fn type_infer_singletons_singleton_when_type_is_blocked() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function id(x: typeof("hello")) return x end

        local function foobar()
            return id("hello")
        end
    "#,
    None,
  );

  // cpp 侧先 ignoreMissingAnnotations（滤除 TypeAnnotationRequired）再断言无错误
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// 缺口（未移植）：`tests/TypeInfer.singletons.test.cpp:851` 的
// `pass_singleton_through_to_identity`（新 solver）——恒等函数 `id("hello")` 的
// 返回须原样保留 "hello" 单例；本移植实测返回被宽化为 `string` 并报
// TypePackMismatch（string is not a subtype of "hello"），属新 solver 单例保留
// （恒等函数返回单例不宽化）的实现缺口，待补齐后应补回该用例。
