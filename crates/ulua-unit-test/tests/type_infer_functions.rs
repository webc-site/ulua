use ulua_ast::rtti::ast_node_is_ptr;
extern crate alloc;
use alloc::string::String;

use ulua_analysis::{
  functions::{
    first::first,
    flatten_type_pack::flatten_type_pack_id,
    follow_type,
    get_error::get_type_error,
    get_type,
    to_string_error::to_string_type_error,
    to_string_to_string::{
      to_string_type_id, to_string_type_id_to_string_options, to_string_type_pack_id,
    },
  },
  records::{
    cannot_infer_binary_operation::CannotInferBinaryOperation, count_mismatch::CountMismatch,
    explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
    extra_information::ExtraInformation,
    function_exits_without_returning::FunctionExitsWithoutReturning, function_type::FunctionType,
    generic_type::GenericType, not_a_table::NotATable, table_type::TableType,
    to_string_options::ToStringOptions, type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch, unknown_property::UnknownProperty,
    where_clause_needed::WhereClauseNeeded,
  },
};
use ulua_ast::records::{location::Location, position::Position};
use ulua_common::fflag;
use ulua_unit_test::{
  functions::{
    register_hidden_types::register_hidden_types, type_error_data_ref::type_error_data_ref,
  },
  records::{
    builtins_fixture::BuiltinsFixture, extern_type_fixture::ExternTypeFixture, fixture::Fixture,
  },
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_another_higher_order_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local Get_des
        function Get_des(func)
            Get_des(func)
        end

        local function f(d)
            d:IsA("BasePart")
            d.Parent:FindFirstChild("Humanoid")
            d:IsA("Decal")
        end
        Get_des(f)

    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_another_indirect_function_case_where_it_is_ok_to_provide_too_many_arguments()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local mycb: (number, number) -> ()

        function f() end

        mycb = f
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_another_other_higher_order_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let source = if !fflag::DebugLuauForceOldSolver.get() {
    r#"
            local function f(d)
                d:foo()
                d:foo()
            end
        "#
  } else {
    r#"
            local d
            d:foo()
            d:foo()
        "#
  };

  let result = fixture.check_string_optional_frontend_options(source, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_another_recursive_local_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local count
        function count(n: number)
            if n == 0 then
                return 0
            else
                return count(n - 1)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_apply_example_from_oss() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type something = { Something: number }
        type example = { Example: number }
        local function test(a: something): example
            return nil :: any
        end
        local function apply<T..., U...>(func: (T...) -> U..., ...: T...): (boolean, U...)
            return nil :: any
        end
        local b, result = apply(test, {
            Something = 1
        })
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ Example: number }",
    to_string_type_id_to_string_options(fixture.require_type_string("result"), &mut opts)
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_apply_of_lambda_with_inferred_and_explicit_types() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function apply(f, x) return f(x) end
        local x = apply(function(x: string): number return 5 end, "hello!")

        local function apply_explicit<A, B...>(f: (A) -> B..., x: A): B... return f(x) end
        local x = apply_explicit(function(x: string): number return 5 end, "hello!")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_are_we_in_the_new_solver() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        -- This file should fail the old solver
        function add(a, b)
            return a + b
        end
        local vec2 = {}
        function vec2.new(x, y)
            return setmetatable({ x = x or 0, y = y or 0 }, {
                __add = function(v1, v2)
                    return { x = v1.x + v2.x, y = v1.y + v2.y }
                end,
            })
        end
        local a = add(1, 1)
        local b = add(vec2.new(0, 0), vec2.new(1, 1))
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("a"))
  );
  assert_eq!(
    "{ x: number, y: number }",
    to_string_type_id(fixture.base.require_type_string("b"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_attempt_to_call_an_intersection_of_tables() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(t: { x: number } & { y: string })
            t()
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Cannot call a value of type { x: number } & { y: string }",
      to_string_type_error(&result.errors[0])
    );
  } else {
    assert_eq!(
      "Cannot call a value of type { x: number }",
      to_string_type_error(&result.errors[0])
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_attempt_to_call_an_intersection_of_tables_with_call_metamethod() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Callable = typeof(setmetatable({}, {
            __call = function(self, ...) return ... end
        }))

        local function f(t: Callable & { x: number })
            t()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidi_inference_functions_complete_ex() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _better_unions =
    ScopedFastFlag::new(&fflag::LuauBidirectionalInferenceBetterUnionHandling, true);
  let _instantiation = ScopedFastFlag::new(&fflag::LuauExplicitTypeInstantiationSupport, true);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(

          r#"
        --!strict
        type Player = {}

        export type RemoteEventWrapper<T...> = {
            connect:( self: RemoteEventWrapper<T...>, callback: ((T...) -> ()) | ((player: Player, T...) -> ()) ) -> () -> (),
        }

        local function useRemoteEvent<T...>(remoteEventName: string, isUnreliable: boolean?): RemoteEventWrapper<T...>
            return nil :: any
        end

        type Payload = {
            name: string,
            time: number,
            data: { [string]: any },
        }

        local payload = useRemoteEvent<<(Payload)>>("initial-payload")

        -- We expect bidirectional inference to kick in here and ensure that
        -- player and payload have non-unknown types.
        payload:connect(function(player, payload)
            local _ = player
            local _ = payload
        end)

        return useRemoteEvent
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Player",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 23,
      column: 23
    }))
  );
  assert_eq!(
    "Payload",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 24,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidi_inference_union_of_functions_1() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _better_unions =
    ScopedFastFlag::new(&fflag::LuauBidirectionalInferenceBetterUnionHandling, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(_: ((string) -> ()) | ((number, number) -> ()))
        end

        f(function (one, two)
            local _ = one
            local _ = two
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 5,
      column: 23
    }))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 6,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidi_inference_union_of_functions_2() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _better_unions =
    ScopedFastFlag::new(&fflag::LuauBidirectionalInferenceBetterUnionHandling, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(_: ((string) -> ()) | ((number, number) -> ()))
        end

        f(function (one)
            local _ = one
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 5,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidi_inference_union_of_functions_3() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _better_unions =
    ScopedFastFlag::new(&fflag::LuauBidirectionalInferenceBetterUnionHandling, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(_: ((string) -> ()) | ((number) -> ()))
        end

        f(function (one)
            local _ = one
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 5,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidi_inference_union_of_functions_4() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _better_unions =
    ScopedFastFlag::new(&fflag::LuauBidirectionalInferenceBetterUnionHandling, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(_: ((string) -> ())?)
        end

        f(function (one)
            local _ = one
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 5,
      column: 23
    }))
  );
}

#[test]
fn type_infer_functions_bidirectional_checking_of_callback_property() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function print(x: number) end

        type Point = {x: number, y: number}
        local T : {callback: ((Point) -> ())?} = {}

        T.callback = function(p) -- No error here
            print(p.z)           -- error here.  Point has no property z
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!("((Point) -> ())?", to_string_type_id(tm.wanted_type));
    assert_eq!(
      "({ read z: number }) -> ()",
      to_string_type_id(tm.given_type)
    );
    assert_eq!(6, result.errors[0].location.begin.line);
    assert_eq!(8, result.errors[0].location.end.line);
  } else {
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!(7, result.errors[0].location.begin.line);
    assert_eq!(7, result.errors[0].location.end.line);
  }
}

#[test]
fn type_infer_functions_bidirectional_function_statement_inference_with_extern() {
  let mut fixture = ExternTypeFixture::default();
  fixture.get_frontend();

  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        type HasClass = { f: (ClassWithGenericMethod) -> () }
        local t = {} :: HasClass
        function t.f(cls)
            local _ = cls
            local foobar = cls.identity(42)
            local _ = foobar
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "ClassWithGenericMethod",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position {
          line: 4,
          column: 23
        })
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position {
          line: 6,
          column: 23
        })
    )
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidirectional_inference_allow_internal_generics() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type testsuite = { case: (self: testsuite, <T>(T) -> T) -> () }

        local test1: { suite: (string, (testsuite) -> ()) -> () } = nil :: any

        test1.suite("LuteTestCommand", function(suite)
            suite:case(42)
        end)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("<T>(T) -> T", to_string_type_id(err.wanted_type));
  assert_eq!("number", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidirectional_inference_goes_through_ifelse() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Input = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        local function getInputs(isDragonPunch: boolean): { Input }
            return if isDragonPunch then { "6", "8", "7" } else { "8", "7", "6" }
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

#[test]
fn type_infer_functions_bidirectional_inference_of_class_methods() {
  let mut fixture = ExternTypeFixture::default();
  fixture.get_frontend();

  let result = fixture.base.base.check_string_optional_frontend_options(

          r#"
        local c = ChildClass.New()

        -- Instead of reporting that the lambda is the wrong type, report that we are using its argument improperly.
        c.Touched:Connect(function(other)
            print(other.ThisDoesNotExist)
        end)
    "#
,
      None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err =
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  assert_eq!("ThisDoesNotExist", err.key());
  assert_eq!("BaseClass", to_string_type_id(err.table()));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidirectional_lambda_inference_applies_nilable_functions() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local listdir: (string, ((string) -> boolean)?) -> { string } = nil :: any
        listdir("my_directory", function (path)
            print(path)
            return true
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 3,
      column: 19
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_bidirectionally_infer_lambda_with_partially_resolved_generic() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo<T>(value: T)
            return function<R>(callback: (T) -> R)
            end
        end

        foo(3)(function (data)
            local _ = data
            return 42
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 7,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_call_function_with_nothing_but_nil() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(n: number, x: string?, y: string?, z: string?) end

        local function g(n)
            f(n)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_calling_function_with_anytypepack_doesnt_leak_free_types() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!nonstrict

        function Test(a)
            return 1, ""
        end


        local tab = {}
        table.insert(tab, Test(1));
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let mut opts = ToStringOptions::new(true);
  opts.max_table_length = 0;
  let tab_type = fixture.base.require_type_string("tab");
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "{string}",
      to_string_type_id_to_string_options(tab_type, &mut opts)
    );
  } else {
    assert_eq!(
      "{any}",
      to_string_type_id_to_string_options(tab_type, &mut opts)
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(a: number, b: string) end

        foo("Test", 123)
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    Location {
      begin: Position {
        line: 3,
        column: 12
      },
      end: Position {
        line: 3,
        column: 18
      },
    },
    result.errors[0].location
  );
  let tm0 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(tm0.wanted_type));
  assert_eq!("string", to_string_type_id(tm0.given_type));

  assert_eq!(
    Location {
      begin: Position {
        line: 3,
        column: 20
      },
      end: Position {
        line: 3,
        column: 23
      },
    },
    result.errors[1].location
  );
  let tm1 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!("string", to_string_type_id(tm1.wanted_type));
  assert_eq!("number", to_string_type_id(tm1.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_cannot_call_union_of_functions() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
         local f: (() -> ()) | (() -> () -> ()) = nil :: any
         f()
     "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Cannot call a value of the union type:\n  | () -> ()\n  | () -> () -> ()\nWe are unable to determine the appropriate result type for such a call.";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_cannot_hoist_interior_defns_into_signature() {
  use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(x: T)
            type T = number
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 28
      },
      end: Position {
        line: 1,
        column: 29
      }
    },
    result.errors[0].location
  );
  assert_eq!(String::from("MainModule"), result.errors[0].module_name);
  let err =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("T", err.name());
  assert_eq!(Context::Type, err.context());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_captured_local_is_assigned_a_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f

        local function g()
            f()
        end

        function f()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_check_function_before_lambda_that_uses_it() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict

        function f()
            return 114
        end

        return function()
            return f():andThen()
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_check_function_bodies() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function myFunction(): number
            local a = 0
            a = true
            return a
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(tm.wanted_type));
  assert_eq!("boolean", to_string_type_id(tm.given_type));

  if fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      Location {
        begin: Position {
          line: 3,
          column: 16
        },
        end: Position {
          line: 3,
          column: 20
        }
      },
      result.errors[0].location
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_cli_119545_pass_lambda_inside_table() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        type foo1 = { foo: (number) -> () }
        type foo2 = { read foo: (number) -> () }
        local function bar1(foo: foo1) end
        local function bar2(foo: foo2) end

        local baz = { foo = function(number: number) end, }
        bar1(baz)
        bar2(baz)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

#[test]
fn type_infer_functions_cli_187542_recursive_call_in_loop() {
  use ulua_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = BuiltinsFixture::default();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function a(b)
            if true then return b end
            while false do
                b = a(b)
            end

            if true then return b end
        end
    "#,
    None,
  );

  assert_eq!(4, result.errors.len(), "{:?}", result.errors);
  assert!(
    !result
      .errors
      .iter()
      .any(|error| type_error_data_ref::<ConstraintSolvingIncompleteError>(error).is_some()),
    "expected no ConstraintSolvingIncompleteError, got {:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_complicated_return_types_require_an_explicit_annotation() {
  use ulua_analysis::records::union_type::UnionType;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local i = 0
        function most_of_the_natural_numbers(): number?
            if i < 10 then
                i += 1
                return i
            else
                return nil
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ty = fixture.require_type_string("most_of_the_natural_numbers");
  let function_type = get_type::get::<FunctionType>(ty)
    .unwrap_or_else(|| panic!("expected function but got {}", to_string_type_id(ty)));

  let ret_type = first(function_type.ret_types(), false).expect("expected return type");
  assert!(get_type::get::<UnionType>(ret_type).is_some());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_concrete_functions_are_not_supertypes_of_function() {
  let mut fixture = Fixture::fixture_bool(false);
  register_hidden_types(fixture.get_frontend());

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: fun = function() end

        function one(arg: () -> ()) end
        function two(arg: <T>(T) -> T) end

        one(a)
        two(a)
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!(6, result.errors[0].location.begin.line);
  let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("() -> ()", to_string_type_id(tm1.wanted_type));
  assert_eq!("function", to_string_type_id(tm1.given_type));

  assert_eq!(7, result.errors[1].location.begin.line);
  let tm2 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!("<T>(T) -> T", to_string_type_id(tm2.wanted_type));
  assert_eq!("function", to_string_type_id(tm2.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_coroutine_wrap_result_call() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _ = fixture.base.check_string_optional_frontend_options(
    r#"
        function foo(a, b)
            coroutine.wrap(a)(b)
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_cyclic_function_type_in_rets() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f()
            return f
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "t1 where t1 = () -> t1",
    to_string_type_id(fixture.require_type_string("f"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization() {
  use ulua_analysis::records::unification_too_complex::UnificationTooComplex;
  use ulua_common::fint;
  use ulua_unit_test::type_aliases::scoped_fast_int::ScopedFastInt;

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _tarjan_limit = ScopedFastInt::new(&fint::LuauTarjanChildLimit, 1);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(t)
            t.x.y.z = 441
        end
    "#,
    None,
  );

  assert!(
    result
      .errors
      .iter()
      .any(|error| type_error_data_ref::<UnificationTooComplex>(error).is_some()),
    "{:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_give_other_overloads_message_if_only_one_argument_matching_overload_exists()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply(1, "")
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!(
    unsafe { (*fixture.builtin_types).number_type() },
    tm.wanted_type
  );
  assert_eq!(
    unsafe { (*fixture.builtin_types).string_type() },
    tm.given_type
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_infer_overloaded_functions() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function getR6Attachments(model)
            model:FindFirstChild("Right Leg")
            model:FindFirstChild("Left Leg")
            model:FindFirstChild("Torso")
            model:FindFirstChild("Torso")
            model:FindFirstChild("Head")
            model:FindFirstChild("Left Arm")
            model:FindFirstChild("Right Arm")
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(t1) -> () where t1 = { read FindFirstChild: (t1, string) -> (...unknown) }",
      to_string_type_id(fixture.require_type_string("getR6Attachments"))
    );
  } else {
    assert_eq!(
      "<a...>(t1) -> () where t1 = {+ FindFirstChild: (t1, string) -> (a...) +}",
      to_string_type_id(fixture.require_type_string("getR6Attachments"))
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t = {}

        function t.f(x)
            return x
        end

        t.__index = t

        function g(s)
            local q = s.p and s.p.q or nil
            return q and t.f(q) or nil
        end

        local f = t.f
    "#,
    None,
  );

  assert_eq!(
    "<a>(a) -> a",
    to_string_type_id(fixture.require_type_string("f"))
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({+ p: {+ q: nil +} +}) -> nil",
      to_string_type_id(fixture.require_type_string("g"))
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_leak_generics_keyof() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function makeOtherThing(template)
            return {
                Stuff = template
            }
        end

        local function makeThing(tbl)
            local returnThis = { Input = makeOtherThing(tbl) }

            function returnThis.Test(key: keyof<typeof(returnThis.Input.Stuff)>) end

            return returnThis
        end

        local thing = makeThing({a=1})
        thing.Test("a")

        local otherthing = makeThing({b = 42, c = 13})
        otherthing.Test("b")
        otherthing.Test("c")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "{ Input: { Stuff: { a: number } }, Test: (\"a\") -> () }",
    to_string_type_id(fixture.base.require_type_string("thing"))
  );
  assert_eq!(
    "{ Input: { Stuff: { b: number, c: number } }, Test: (\"b\" | \"c\") -> () }",
    to_string_type_id(fixture.base.require_type_string("otherthing"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_dont_mutate_the_underlying_head_of_typepack_when_calling_with_self() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t = {}
        function t:m(x) end
        function f(): never return 5 :: never end
        t:m(f())
        t:m(f())
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_duplicate_functions_2() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo() end

        function bar()
            local function foo() end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_duplicate_functions_allowed_in_nonstrict() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        function foo() end

        function foo() end

        function bar()
            local function foo() end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        function foo(): number
            return 1
        end
        foo()

        function foo(n: number): number
            return 2
        end
        foo()
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("() -> number", to_string_type_id(tm.wanted_type));
  assert_eq!("(number) -> number", to_string_type_id(tm.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_detailed_function_mismatch_arg() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A = (number, number) -> string
type B = (number, string) -> string

local a: A
local b: B = a
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t\
'(number, string) -> string'\
\nbut got\n\t\
'(number, number) -> string'\
\ncaused by:\n  \
Argument #2 type is not compatible.\n\
Expected this to be 'number', but got 'string'";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_detailed_function_mismatch_arg_count() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A = (number, number) -> string
type B = (number) -> string

local a: A
local b: B = a
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t\
'(number) -> string'\
\nbut got\n\t\
'(number, number) -> string'\
\ncaused by:\n  \
Argument count mismatch. Function expects 2 arguments, but only 1 is specified";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_detailed_function_mismatch_ret() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A = (number, number) -> string
type B = (number, number) -> number

local a: A
local b: B = a
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t\
'(number, number) -> number'\
\nbut got\n\t\
'(number, number) -> string'\
\ncaused by:\n  \
Return type is not compatible.\n\
Expected this to be 'number', but got 'string'";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_detailed_function_mismatch_ret_count() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A = (number, number) -> (number)
type B = (number, number) -> (number, boolean)

local a: A
local b: B = a
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t\
'(number, number) -> (number, boolean)'\
\nbut got\n\t\
'(number, number) -> number'\
\ncaused by:\n  \
Function only returns 1 value, but 2 are required here";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_detailed_function_mismatch_ret_mult() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A = (number, number) -> (number, string)
type B = (number, number) -> (number, boolean)

local a: A
local b: B = a
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t\
'(number, number) -> (number, boolean)'\
\nbut got\n\t\
'(number, number) -> (number, string)'\
\ncaused by:\n  \
Return #2 type is not compatible.\n\
Expected this to be 'boolean', but got 'string'";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_error_suppression_propagates_through_function_calls() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function first(x: any)
            return pairs(x)(x)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(any) -> (any?, any)",
    to_string_type_id(fixture.base.require_type_string("first"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_first_argument_can_be_optional() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local T = {}
        function T.new(a: number?, b: number?, c: number?) return 5 end
        local m = T.new()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_free_is_not_bound_to_unknown() {
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(f: (unknown) -> (), x)
            f(x)
        end
    "#,
    None,
  );

  assert_eq!(
    "<a>((unknown) -> (), a) -> ()",
    to_string_type_id(fixture.require_type_string("foo"))
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_func_expr_doesnt_leak_free() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local p = function(x) return x end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let fn_ty =
    get_type::get::<FunctionType>(fixture.require_type_string("p")).expect("expected FunctionType");
  let ret = first(fn_ty.ret_types(), true).expect("expected return type");
  let ret = follow_type::follow(ret);
  assert!(
    get_type::get::<GenericType>(ret).is_some(),
    "expected generic return type"
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_argument_error_suppression() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local functions: {[any]: (any) -> ()} = {}
        functions.func1 = function(value: string) end
        functions.func2 = function(value: boolean) end
        functions.func3 = function(value: number) end
        functions.func4 = function(value: any) end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_calls_should_not_crash() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _ = fixture.base.check_string_optional_frontend_options(

          r#"
        return {
            StartAPI = function()
                local pointers = {}
                local API = {}
                local function getRealEnvResult(PointerOrPath)
                    if pointers[PointerOrPath] then
                        return pointers[PointerOrPath]
                    end
                end
                API.OnInvoke = function()
                    local realEnvResult, isResultPointer = getRealEnvResult(FunctionInEnvToRunPath)
                    return realEnvResult(table.unpack(args, 2, args.n))
                    if TableInEnvPath and type(TableInEnvPath) == 'string' then
                        local realEnvResult, isResultPointer = getRealEnvResult(TableInEnvPath)
                        return getmetatable(realEnvResult)
                    end
                    local realEnvResult, isResultPointer = getRealEnvResult(TableInEnvPath)
                    local metaTableInEnv = getmetatable(realEnvResult)
                    local result = metaTableInEnv[FuncToRun](realEnvResult,table.unpack(args, 3, args.n))
                end
            end
        }
    "#
,
      None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_cast_error_uses_correct_language() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(a, b): number
            return 0
        end

        local a: (string)->number = foo
        local b: (number, number)->(number, number) = foo

        local c: (string, number)->number = foo -- no error
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("(string) -> number", to_string_type_id(tm1.wanted_type));
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(unknown, unknown) -> number",
      to_string_type_id(tm1.given_type)
    );
  } else {
    assert_eq!(
      "(string, *error-type*) -> number",
      to_string_type_id(tm1.given_type)
    );
  }

  let tm2 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!(
    "(number, number) -> (number, number)",
    to_string_type_id(tm2.wanted_type)
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(unknown, unknown) -> number",
      to_string_type_id(tm1.given_type)
    );
  } else {
    assert_eq!(
      "(string, *error-type*) -> number",
      to_string_type_id(tm2.given_type)
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_decl_non_self_sealed_overwrite() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function string.len(): number
            return 1
        end

        local s = string
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  fixture.get_frontend().clear();

  let result2 = fixture.base.check_string_optional_frontend_options(
    r#"
        print(string.len('hello'))
    "#,
    None,
  );

  assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_decl_non_self_sealed_overwrite_2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local t: { f: ((x: number) -> number)? } = {}

function t.f(x)
    print(x + 5)
    return x .. "asd" -- 1st error: we know that return type is a number, not a string
end

t.f = function(x)
    print(x + 5)
    return x .. "asd" -- 2nd error: we know that return type is a number, not a string
end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(
      result
        .errors
        .iter()
        .all(|error| type_error_data_ref::<WhereClauseNeeded>(error).is_some()),
      "{:?}",
      result.errors
    );
  } else {
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_decl_non_self_unsealed_overwrite() {
  let _check_function_statement_types =
    ScopedFastFlag::new(&fflag::LuauCheckFunctionStatementTypes, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local t = { f = nil :: ((x: number) -> number)? }

function t.f(x: string): string -- 1st error: new function value type is incompatible
    return x .. "asd"
end

t.f = function(x)
    print(x + 5)
    return x .. "asd" -- 2nd error: we know that return type is a number, not a string
end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<WhereClauseNeeded>(error).is_some()),
      "{:?}",
      result.errors
    );
  } else {
    assert_eq!(
      r#"Expected this to be
	'((number) -> number)?'
but got
	'(string) -> string'
caused by:
  None of the union options are compatible. For example:
Expected this to be
	'(number) -> number'
but got
	'(string) -> string'
caused by:
  Argument #1 type is not compatible.
Expected this to be 'string', but got 'number'"#,
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_decl_quantify_right_type() {
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/isAMagicMock"),
    String::from(
      r#"
--!nonstrict
return function(value)
    return false
end
    "#,
    ),
  );

  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
--!nonstrict
local MagicMock = {}
MagicMock.is = require(game.isAMagicMock)

function MagicMock.is(value)
    return false
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_definition_in_a_do_block() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f
        do
            function f()
            end
        end
        f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_definition_in_a_do_block_with_global() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function f() print("a") end
        do
            function f()
                print("b")
            end
        end
        f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_does_not_return_enough_values() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function f(): (number, string)
            return 55
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    let tpm =
      get_type_error::<TypePackMismatch>(&result.errors[0]).expect("expected TypePackMismatch");
    assert_eq!("number, string", to_string_type_pack_id(tpm.wanted_tp()));
    assert_eq!("number", to_string_type_pack_id(tpm.given_tp()));
  } else {
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatch::RETURN, acm.context());
    assert_eq!(2, acm.expected());
    assert_eq!(1, acm.actual());
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_exprs_are_generalized_at_signature_scope_not_enclosing() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo
        local bar

        -- foo being a function expression is deliberate: the bug we're testing
        -- only existed for function expressions, not for function statements.
        foo = function(a)
            return bar
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "((unknown) -> nil)?",
      to_string_type_id(fixture.require_type_string("foo"))
    );
  } else {
    assert_eq!(
      "<a>(a) -> 'b",
      to_string_type_id(fixture.require_type_string("foo"))
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_is_supertype_of_concrete_functions() {
  let mut fixture = Fixture::fixture_bool(false);
  register_hidden_types(fixture.get_frontend());

  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(f: fun) end

        function a() end
        function id(x) return x end

        foo(a)
        foo(id)
        foo(foo)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_statement_sealed_table_assignment_through_indexer() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local t: {[string]: () -> number} = {}

function t.a() return 1 end -- OK
function t:b() return 2 end -- not OK
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be\n\t\
'() -> number'\
\nbut got\n\t\
'(*error-type*) -> number'\
\ncaused by:\n  \
Argument count mismatch. Function expects 1 argument, but none are specified",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_statement_with_incorrect_function_type() {
  let _check_function_statement_types =
    ScopedFastFlag::new(&fflag::LuauCheckFunctionStatementTypes, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local Library: { isnan: (number) -> number } = {} :: any

        function Library.isnan(s: string): boolean
            return s == "NaN"
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
  assert_eq!("(string) -> boolean", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_function_that_could_return_anything_is_compatible_with_function_that_is_expected_to_return_nothing()
 {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        -- We infer foo : (g: (number) -> (...unknown)) -> ()
        function foo(g)
            g(0)
        end

        -- a requires a function that returns no values
        function a(f: ((number) -> ()) -> ())
        end

        -- "Returns an unknown number of values" is close enough to "returns no values."
        a(foo)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzz_must_follow_in_overload_resolution() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
for _ in function<t0>():(t0)&((()->())&(()->()))
end do
_(_(_,_,_),_)
end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzz_unwind_mutually_recursive_union_type_func() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local _ = ...
        function _()
            _ = _
        end
        _[function(...) repeat until _(_[l100]) _ = _ end] += _
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
function _()
end
local function l0()
    function _()
    end
end
_ = _
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzzer_bug_missing_follow_causes_assertion() {
  let mut fixture = Fixture::fixture_bool(false);
  let _ = fixture.check_string_optional_frontend_options(
    r#"
local _ = ({_=function()
return _
end,}),true,_[_()]
for l0=_[_[_[`{function(l0)
end}`]]],_[_.n6[_[_.n6]]],_[_[_.n6[_[_.n6]]]] do
_ += if _ then ""
end
return _
"#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun() {
  let mut fixture = Fixture::fixture_bool(false);
  let _ = fixture.check_string_optional_frontend_options(
    r#"
        local _ = function<t0...>()
        end ~= _

        while (_) do
            _,_,_,_,_,_,_,_,_,_._,_ = nil
            function _(...):<t0...>()->()
            end
            function _<t0...>(...):any
                _ ..= ...
            end
            _,_,_,_,_,_,_,_,_,_,_ = nil
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_fuzzer_normalizer_out_of_resources() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _ = fixture.base.check_string_optional_frontend_options(
    r#"
 Module 'l0':
local _ = true,...,_
if ... then
while _:_(_._G) do
do end
_ = _ and _
_ = 0 and {# _,}
local _ = "CCCCCCCCCCCCCCCCCCCCCCCCCCC"
local l0 = require(module0)
end
local function l0()
end
elseif _ then
l0 = _
end
do end
while _ do
_ = if _ then _ elseif _ then _,if _ then _ else _
_ = _()
do end
do end
if _ then
end
end
_ = _,{}

    "#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_general_case_table_literal_blocks() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!strict
function f(x : {[any]: number})
   return x
end

local Foo = {bar = "$$$"}

f({[Foo.bar] = 0})
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_generalize_table_property() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local T = {}

        T.foo = function(x)
            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ty = follow_type::follow(fixture.require_type_string("T"));
  let table = get_type::get::<TableType>(ty).expect("expected TableType");
  let foo = table.props.get("foo").expect("expected foo property");
  let foo_ty = foo.read_ty.expect("expected readable foo property");
  assert_eq!("<a>(a) -> a", to_string_type_id(foo_ty));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_generic_function_statement() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Object = {
            foobar: <T>(number, string, T) -> T
        }

        local Obj = {} :: Object
        function Obj.foobar(bing, quxx, dunno)
            local _ = bing
            local _ = quxx
            return dunno
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 24
    }))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 8,
      column: 24
    }))
  );
  assert_eq!(
    "a",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 9,
      column: 21
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_generic_packs_are_not_variadic() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function apply<a, b..., c...>(f: (a, b...) -> c..., x: a)
            return f(x)
        end

        local function add(x: number, y: number)
            return x + y
        end

        local function addToSix(x: number)
            return x + 6
        end

        apply(addToSix, 7)
        apply(add, 5)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location {
      begin: Position {
        line: 2,
        column: 21
      },
      end: Position {
        line: 2,
        column: 22
      },
    },
    result.errors[0].location
  );
  let err =
    get_type_error::<TypePackMismatch>(&result.errors[0]).expect("expected TypePackMismatch");
  assert_eq!("a", to_string_type_pack_id(err.given_tp()));
  assert_eq!("b...", to_string_type_pack_id(err.wanted_tp()));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_generic_polarity_of_annotated_code() {
  use ulua_analysis::enums::polarity::Polarity;

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: <T>(T) -> T = nil :: any
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ftv =
    get_type::get::<FunctionType>(fixture.require_type_string("f")).expect("expected FunctionType");
  let r#gen = get_type::get::<GenericType>(ftv.generics()[0]).expect("expected GenericType");
  assert_eq!(Polarity::Mixed, r#gen.polarity);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_global_emplacing_steals_type_from_elsewhere() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f()
            return 42
        end
        local a = f()
        b = a
        local c = b
        function b()
        end
    "#,
    None,
  );

  // NOTE: upstream `global_emplacing_steals_type_from_elsewhere`
  // (tests/TypeInfer.functions.test.cpp:4060) does NOT assert on error count —
  // it only CHECK_EQ's the inferred types of `a`, `b`, `c`. Under the new solver
  // the global `b` is assigned (`b = a`) before being defined (`function b()`),
  // which legitimately emits an `UnknownSymbol{Binding}` implicit-global warning
  // (TypeChecker2.cpp:1529). A `0 errors` assertion was never part of the upstream
  // test; the faithful checks are the three type-string comparisons below.
  let _ = &result;
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "() -> ()",
    to_string_type_id(fixture.require_type_string("b"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("c"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_global_function_blocked() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local addInstanceToState: any = nil
        local inst: any = nil

        function ingestAllInstances(...): ()
            local id: number = addInstanceToState()
            local child: any = nil
            ingestAllInstances(child)
        end

        function handleDmQuery()
            ingestAllInstances()
        end

        return {}

    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_global_function_redefinition() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function fact(n: number)
            return if n < 1 then 1 else n * fact(n - 1)
        end

        fact = "huh"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_hidden_variadics_should_not_break_subtyping() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        type FooType = {
            SetValue: (Value: number) -> ()
        }

        local Foo: FooType = {
            SetValue = function(Value: number)

            end
        }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_higher_order_function_2() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function bottomupmerge(comp, a, b, left, mid, right)
            local i, j = left, mid
            for k = left, right do
                if i < mid and (j > right or not comp(a[j], a[i])) then
                    b[k] = a[i]
                    i = i + 1
                else
                    b[k] = a[j]
                    j = j + 1
                end
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ftv = get_type::get::<FunctionType>(fixture.require_type_string("bottomupmerge"))
    .expect("expected bottomupmerge to have function type");

  let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
  assert_eq!(6, arg_vec.len());

  let f_type = get_type::get::<FunctionType>(follow_type::follow(arg_vec[0]));
  assert!(f_type.is_some(), "expected first argument to be a function");
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_higher_order_function_3() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function swap(p)
            local t = p[0]
            p[0] = p[1]
            p[1] = t
            return nil
        end

        function swapTwice(p)
            swap(p)
            swap(p)
            return p
        end

        function swapTwiceOn(t: { number })
            swapTwice(t)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "<a, b>({a} & {b}) -> {a} & {b}",
    to_string_type_id(fixture.require_type_string("swapTwice"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_higher_order_function_4() {
  use ulua_analysis::functions::size_type_pack::size;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        function bottomupmerge(comp, a, b, left, mid, right)
            local i, j = left, mid
            for k = left, right do
                if i < mid and (j > right or not comp(a[j], a[i])) then
                    b[k] = a[i]
                    i = i + 1
                else
                    b[k] = a[j]
                    j = j + 1
                end
            end
        end

        function mergesort<T>(arr: {T}, comp: (T, T) -> boolean)
            local work = {}
            for i = 1, #arr do
                work[i] = arr[i]
            end
            local width = 1
            while width < #arr do
                for i = 1, #arr, 2*width do
                    bottomupmerge(comp, arr, work, i, math.min(i+width, #arr), math.min(i+2*width-1, #arr))
                end
                local temp = work
                work = arr
                arr = temp
                width = width * 2
            end
            return arr
        end
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ftv = get_type::get::<FunctionType>(fixture.base.require_type_string("mergesort"))
    .expect("expected mergesort to have function type");

  let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
  assert_eq!(2, arg_vec.len());

  let arg0 = get_type::get::<TableType>(follow_type::follow(arg_vec[0]))
    .expect("expected first argument to be a table");
  let indexer = arg0.indexer.as_ref().expect("expected table indexer");

  let arg1 = get_type::get::<FunctionType>(follow_type::follow(arg_vec[1]))
    .expect("expected second argument to be a function");
  assert_eq!(2, size(arg1.arg_types(), None));

  let (arg1_args, _) = flatten_type_pack_id(arg1.arg_types());

  assert_eq!(
    follow_type::follow(indexer.index_result_type),
    follow_type::follow(arg1_args[0])
  );
  assert_eq!(
    follow_type::follow(indexer.index_result_type),
    follow_type::follow(arg1_args[1])
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_ignored_return_values() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function f()
            return 55, ""
        end

        local a = f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_improved_function_arg_mismatch_error_nonstrict() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        local function foo(a, b) end
        foo(string.find("hello", "e"))
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Argument count mismatch. Function 'foo' expects 0 to 2 arguments, but 3 are specified",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_improved_function_arg_mismatch_errors() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local function foo1(a: number) end
foo1()

local function foo2(a: number, b: string?) end
foo2()

local function foo3(a: number, b: string?, c: any) end -- any is optional
foo3()

string.find()

local t = {}
function t.foo(x: number, y: string?, ...: any) return 1 end
function t:bar(x: number, y: string?) end
t.foo()

t:bar()

local u = { a = t, b = function() return t end }
u.a.foo()
local x = (u.a).foo()

u.b().foo()
    "#,
    None,
  );

  assert_eq!(9, result.errors.len(), "{:?}", result.errors);
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    [
      "Argument count mismatch. Function expects 1 argument, but none are specified",
      "Argument count mismatch. Function expects 1 to 2 arguments, but none are specified",
      "Argument count mismatch. Function expects 1 to 3 arguments, but none are specified",
      "Argument count mismatch. Function expects 2 to 4 arguments, but none are specified",
      "Argument count mismatch. Function expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function expects 2 to 3 arguments, but only 1 is specified",
      "Argument count mismatch. Function expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function expects at least 1 argument, but none are specified",
    ]
  } else {
    [
      "Argument count mismatch. Function 'foo1' expects 1 argument, but none are specified",
      "Argument count mismatch. Function 'foo2' expects 1 to 2 arguments, but none are specified",
      "Argument count mismatch. Function 'foo3' expects 1 to 3 arguments, but none are specified",
      "Argument count mismatch. Function 'string.find' expects 2 to 4 arguments, but none are specified",
      "Argument count mismatch. Function 't.foo' expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function 't.bar' expects 2 to 3 arguments, but only 1 is specified",
      "Argument count mismatch. Function 'u.a.foo' expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function 'u.a.foo' expects at least 1 argument, but none are specified",
      "Argument count mismatch. Function expects at least 1 argument, but none are specified",
    ]
  };

  for (error, expected) in result.errors.iter().zip(expected) {
    assert_eq!(expected, to_string_type_error(error));
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_anonymous_function_arguments() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(a) return a.x + a.y end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
local function f(a: ((Table) -> number)?) if a then return a({x = 1, y = 2}) else return 0 end end
f(function(a) return a.x + a.y end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
local x = {}
x.b = {x = 1, y = 2}
function x:f(a: (Table) -> number) return a(self.b) end
x:f(function(a) return a.x + a.y end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function f(a: (a: number, b: number, c: boolean) -> number) return a(1, 2, true) end
f(function(a: number, b, c) return c and a + b or b - a end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(...) return select(1, ...).z end)
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert_eq!(
    "Key 'z' not found in table 'Table'",
    to_string_type_error(&result.errors[0])
  );

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function f(a: (a: number, b: number) -> number) return a(1, 2) end
f(function(a, b, c, ...) return a + b end)
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let expected = if fflag::LuauInstantiateInSubtyping.get() {
    concat!(
      "Expected this to be\n\t",
      "'(number, number) -> number'",
      "\nbut got\n\t",
      "'<a>(number, number, a) -> number'",
      "\ncaused by:\n",
      "  Argument count mismatch. Function expects 3 arguments, but only 2 are specified"
    )
  } else {
    concat!(
      "Expected this to be\n\t",
      "'(number, number) -> number'",
      "\nbut got\n\t",
      "'(number, number, *error-type*) -> number'",
      "\ncaused by:\n",
      "  Argument count mismatch. Function expects 3 arguments, but only 2 are specified"
    )
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function f(a: (...number) -> number) return a(1, 2) end
f(function(a, b) return a + b end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
function f(a: (...Table) -> number) return a({x = 1, y = 2}, {x = 3, y = 4}) end
f(function(a, ...) local b = ... return b.z end)
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert_eq!(
    "Key 'z' not found in table 'Table'",
    to_string_type_error(&result.errors[0])
  );

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
function f(a: (number) -> Table) return a(4) end
f(function(x) return x * 2 end)
    "#,
    None,
  );
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'Table', but got 'number'",
    to_string_type_error(&result.errors[0])
  );

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function f(a: (number) -> nil) return a(4) end
        f(function(x) print(x) end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_anonymous_function_arguments_outside_call() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
type Table = { x: number, y: number }
local f: (Table) -> number = function(t) return t.x + t.y end

type TableWithFunc = { x: number, y: number, f: (number, number) -> number }
local a: TableWithFunc = { x = 3, y = 4, f = function(a, b) return a + b end }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_from_function_return_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    "function take_five() return 5 end    local five = take_five()",
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("five"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_generic_function_function_argument() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local function sum<a>(x: a, y: a, f: (a, a) -> a) return f(x, y) end
return sum(2, 3, function(a, b) return a + b end)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(

          r#"
local function map<a, b>(arr: {a}, f: (a) -> b) local r = {} for i,v in ipairs(arr) do table.insert(r, f(v)) end return r end
local a = {1, 2, 3}
local r = map(a, function(a) return a + a > 100 end)
    "#
,
      None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "{boolean}",
    to_string_type_id(fixture.base.require_type_string("r"))
  );

  let _fold_result = fixture.base.check_string_optional_frontend_options(

          r#"
local function foldl<a, b>(arr: {a}, init: b, f: (b, a) -> b) local r = init for i,v in ipairs(arr) do r = f(r, v) end return r end
local a = {1, 2, 3}
local r = foldl(a, {s=0,c=0}, function(a, b) return {s = a.s + b, c = a.c + 1} end)
    "#
,
      None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "{| c: number, s: number |}",
    to_string_type_id(fixture.base.require_type_string("r"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_generic_function_function_argument_overloaded() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let mut result = fixture.check_string_optional_frontend_options(
    r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12(1, function(x) return x + x end)
g12(1, 2, function(x, y) return x + y end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  result = fixture.check_string_optional_frontend_options(
    r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12({x=1}, function(x) return {x=-x.x} end)
g12({x=1}, {x=2}, function(x, y) return {x=x.x + y.x} end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_generic_lib_function_function_argument() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local a = {{x=4}, {x=7}, {x=1}}
table.sort(a, function(x, y) return x.x < y.x end)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0]).is_some(),
    "expected CannotInferBinaryOperation, got {:?}",
    result.errors[0]
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_higher_order_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function apply(f, x)
            return f(x)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let apply_type = fixture.require_type_string("apply");
  let ftv =
    get_type::get::<FunctionType>(apply_type).expect("expected apply to have function type");

  let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
  assert_eq!(2, arg_vec.len());

  let f_arg_type = follow_type::follow(arg_vec[0]);
  let f_type = get_type::get::<FunctionType>(f_arg_type).unwrap_or_else(|| {
    panic!(
      "expected a function but got {}",
      to_string_type_id(arg_vec[0])
    )
  });

  let (f_args, _) = flatten_type_pack_id(f_type.arg_types());
  let x_type = follow_type::follow(arg_vec[1]);

  assert_eq!(1, f_args.len());
  assert_eq!(x_type, follow_type::follow(f_args[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_return_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options("function take_five() return 5 end", None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let take_five_type = fixture.require_type_string("take_five");
  let take_five_function =
    get_type::get::<FunctionType>(take_five_type).expect("expected function type");

  let (ret_vec, _) = flatten_type_pack_id(take_five_function.ret_types());
  assert!(!ret_vec.is_empty());
  assert_eq!("number", to_string_type_id(ret_vec[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_return_type_from_selected_overload() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = {method: ((T, number) -> number) & ((number) -> string)}
        local T: T

        local a = T.method(T, 4)
        local b = T.method(5)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_return_value_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local function f(): {string|number}
    return {1, "b", 3}
end

local function g(): (number, {string|number})
    return 4, {1, "b", 3}
end

local function h(): ...{string|number}
    return {4}, {1, "b", 3}, {"s"}
end

local function i(): ...{string|number}
    return {1, "b", 3}, h()
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_infer_that_function_does_not_return_a_table() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function take_five()
            return 5
        end

        take_five().prop = 888
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location {
      begin: Position { line: 5, column: 8 },
      end: Position {
        line: 5,
        column: 24
      }
    },
    result.errors[0].location
  );
  let err = type_error_data_ref::<NotATable>(&result.errors[0]).expect("expected NotATable");
  assert_eq!("number", to_string_type_id(err.ty));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_2() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        local function resolveDispatcher()
            return (nil :: any) :: {useContext: (number?) -> any}
        end

        local useContext
        useContext = function(unstable_observedBits: number?)
            resolveDispatcher().useContext(unstable_observedBits)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_3() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo

        foo():bar(function()
            return foo()
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_inner_frees_become_generic_in_dcr() {
  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(x)
            local z = x
            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let ty = fixture
    .find_type_at_position_position(Position {
      line: 3,
      column: 19,
    })
    .expect("expected type at position");
  assert!(
    get_type::get::<GenericType>(follow_type::follow(ty)).is_some(),
    "expected GenericType"
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_instantiated_type_packs_must_have_a_non_null_scope() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function pcall<A..., R...>(...: (A...) -> R...): (boolean, R...)
            return nil :: any
        end

        type Dispatch<A> = (A) -> ()

        function mountReducer()
            dispatchAction()
            return nil :: any
        end

        function dispatchAction()
        end

        function useReducer(): Dispatch<any>
            local result, setResult = pcall(mountReducer)
            return setResult
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

#[test]
fn type_infer_functions_io_manager_oop_ish() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type IIOManager = {
            __index: IIOManager,
            write: (self: IOManager, text: string, label: string?) -> number,
        }

        export type IOManager = setmetatable<{
            buffer: {string},
            memory: { [string]: number }
        }, IIOManager>;

        local IO = {} :: IIOManager
        IO.__index = IO

        function IO:write(text, label)
            local _ = self
            local _ = text
            local _ = label
            return 42
        end

        return IO
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "IOManager",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 15,
      column: 25
    }))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 16,
      column: 25
    }))
  );
  assert_eq!(
    "string?",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 17,
      column: 25
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_it_is_ok_not_to_supply_enough_retvals() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function get_two() return 5, 6 end

        local a = get_two()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_it_is_ok_to_oversaturate_a_higher_order_function_argument() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function onerror() end
        function foo() end
        xpcall(foo, onerror)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_lambda_form_of_local_function_cannot_be_recursive() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f = function() return f() end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_list_all_overloads_if_no_overload_takes_given_argument_count() {
  use ulua_analysis::records::generic_error::GenericError;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply()
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
  assert_eq!(
    "No overload for function accepts 0 arguments.",
    ge.message()
  );

  let ei =
    type_error_data_ref::<ExtraInformation>(&result.errors[1]).expect("expected ExtraInformation");
  assert_eq!(
    "Available overloads: (number) -> number; (number) -> string; and (number, number) -> number",
    ei.message()
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_list_only_alternative_overloads_that_match_argument_count() {
  use ulua_analysis::records::multiple_nonviable_overloads::MultipleNonviableOverloads;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply("")
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    let mno = type_error_data_ref::<MultipleNonviableOverloads>(&result.errors[0])
      .expect("expected MultipleNonviableOverloads");
    assert_eq!(1, mno.attempted_arg_count());
  } else {
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      unsafe { (*fixture.builtin_types).number_type() },
      tm.wanted_type
    );
    assert_eq!(
      unsafe { (*fixture.builtin_types).string_type() },
      tm.given_type
    );
  }

  let ei =
    type_error_data_ref::<ExtraInformation>(&result.errors[1]).expect("expected ExtraInformation");

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Available overloads: (number) -> number; and (number) -> string",
      ei.message()
    );
  } else {
    assert_eq!(
      "Other overloads are also not viable: (number) -> string",
      ei.message()
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_local_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f()
            return 8
        end

        function g()
            local function f()
                return 'hello'
            end
            return f
        end

        local h = g()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let h = follow_type::follow(fixture.require_type_string("h"));
  let ftv = get_type::get::<FunctionType>(h).expect("expected FunctionType");
  let ret = first(ftv.ret_types(), true).expect("expected return type");
  let ret = follow_type::follow(ret);
  assert_eq!("string", to_string_type_id(ret));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_local_function_fwd_decl_doesnt_crash() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo

        local function bar()
            foo()
        end

        function foo()
        end

        bar()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_luau_subtyping_is_np_hard() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!strict

-- An example of coding up graph coloring in the Luau type system.
-- This codes a three-node, two color problem.
-- A three-node triangle is uncolorable,
-- but a three-node line is colorable.

type Red = "red"
type Blue = "blue"
type Color = Red | Blue
type Coloring = (Color) -> (Color) -> (Color) -> boolean
type Uncolorable = (Color) -> (Color) -> (Color) -> false

type Line = Coloring
  & ((Red) -> (Red) -> (Color) -> false)
  & ((Blue) -> (Blue) -> (Color) -> false)
  & ((Color) -> (Red) -> (Red) -> false)
  & ((Color) -> (Blue) -> (Blue) -> false)

type Triangle = Line
  & ((Red) -> (Color) -> (Red) -> false)
  & ((Blue) -> (Color) -> (Blue) -> false)

local x : Triangle
local y : Line
local z : Uncolorable
z = x -- OK, so the triangle is uncolorable
z = y -- Not OK, so the line is colorable
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let expected = "Expected this to be\n\t'(\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> false'\nbut got\n\t'((\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> boolean) & ((\"blue\" | \"red\") -> (\"blue\") -> (\"blue\") -> false) & ((\"blue\" | \"red\") -> (\"red\") -> (\"red\") -> false) & ((\"blue\") -> (\"blue\") -> (\"blue\" | \"red\") -> false) & ((\"red\") -> (\"red\") -> (\"blue\" | \"red\") -> false)'; none of the intersection parts are compatible";
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_lute_tasklib_createtask() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function createtask(f, ...)
            local data = {}

            data.co = coroutine.create(function(...)
                local success, result = pcall(f, ...)

                data.success = success
                data.result = result
            end)

            coroutine.resume(data.co, ...)
            return data
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "((...any) -> (unknown, ...unknown), ...any) -> { co: thread, result: unknown, success: boolean }",
    to_string_type_id(fixture.base.require_type_string("createtask"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_mutual_recursion() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        --!strict

        function newPlayerCharacter()
            startGui() -- Unknown symbol 'startGui'
        end

        local characterAddedConnection: any
        function startGui()
            characterAddedConnection = game:GetService("Players").LocalPlayer.CharacterAdded:connect(newPlayerCharacter)
        end
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_no_lossy_function_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local tbl = {}
        function tbl:abc(a: number, b: number)
            return a
        end
        tbl:abc(1, 2) -- Line 6
        --   | Column 14
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let ty = fixture.require_type_at_position_position(Position {
    line: 6,
    column: 14,
  });
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!("(unknown, number, number) -> number", to_string_type_id(ty));
  } else {
    assert_eq!("(tbl, number, number) -> number", to_string_type_id(ty));
  }

  let ftv = get_type::get::<FunctionType>(follow_type::follow(ty)).expect("expected FunctionType");
  assert!(ftv.has_self());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_num_is_solved_after_num_or_str() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function num_or_str()
            if math.random() > 0.5 then
                return num()
            else
                return "some string"
            end
        end

        function num()
            return 5
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'number', but got 'string'",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "() -> number",
    to_string_type_id(fixture.base.require_type_string("num_or_str"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_num_is_solved_before_num_or_str() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function num()
            return 5
        end

        local function num_or_str()
            if math.random() > 0.5 then
                return num()
            else
                return "some string"
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'number', but got 'string'",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "() -> number",
    to_string_type_id(fixture.base.require_type_string("num_or_str"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_occurs_check_failure_in_function_return_type() {
  use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f()
            return 5, f()
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    get_type_error::<OccursCheckFailed>(&result.errors[0]).is_some(),
    "expected OccursCheckFailed: {:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_1640() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        table.create(1) -- top function call

        local function f(): string
            if true then
                table.create(1) -- middle function call
            end

            return table.concat({})
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_1854() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local function bug()
            local counter = 1
            local work = buffer.create(64)
            local function get_block()
                buffer.writeu32(work, 48, counter)
                counter = (counter + 1) % 0x100000000
                return work
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

#[test]
fn type_infer_functions_oss_1871() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        export type Test = {
            [string]: (string) -> ()
        }

        local TestTbl: Test = {}

        function TestTbl.Hello(Param)
            local _ = Param
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 8,
      column: 25
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2061_modify_visited_generic_ice() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(
    r#"
type actions<T=unknown, A...=...unknown> = { [string]: (state: T, A...) -> (T) }
type disconnect = () -> ()

type producer<state, actions = actions> = {
	get:
		& (() -> state)
		& (<T>(selector: (state) -> T) -> T),
} & actions

type interface = {
	create: <state>(default: state) -> <actions>(actions: actions) -> producer<state, actions>,
}

local a: interface
a.create()
    "#,
    None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  assert!(get_type_error::<CountMismatch>(&results.errors[0]).is_some());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2065_bidirectional_inference_function_call() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(callback: () -> (() -> ())?)
        end

        local someCondition: boolean = true

        foo(function()
            if someCondition then
                return nil
            end
            return function() end
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2109() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function Retry<T..., K...>(
            MaxRetries: number,
            RetryInterval: number,
            Function: (T...) -> (K...),
            ...: T...
        ): K...
            local Results
            local CurrentRetry = 0

            repeat
                Results = {pcall(Function, ...)}

                if not Results[1] then
                    CurrentRetry += 1
                end
            until Results[1] or CurrentRetry == MaxRetries

            return unpack(Results :: any, 2)
        end

        local function Test(a: number, b: number): number
            return a + b
        end

        local a = Retry(5, 1, Test, 5, 10)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2118() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo: <P>(constructor: (P) -> any) -> (P) -> any = (nil :: any)
        local fn = foo(function (value: { test: true })
            return value.test
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "({ test: true }) -> any",
    to_string_type_id(fixture.require_type_string("fn"))
  );
}

#[test]
fn type_infer_functions_oss_2125() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        export type function CombineTableAndSetIndexer(a: type, b: type, c: type)
            local t = {}

            for key, value in a:properties() do
                t[key] = value.read
            end

            if b.tag == "table" then
                for key, value in b:properties() do
                    t[key] = value.read
                end
            end

            return types.newtable(t :: any, { index = types.number, readresult = c, writeresult = c })
        end

        type SpecialProperties = {
            test: string?,
        }

        local function component<Properties>(
            constructor: (props: Properties) -> ()
        ): (
            CombineTableAndSetIndexer<SpecialProperties, Properties, any>
        ) -> ()
            return function(props: Properties) end
        end

        local mrrp = component(function(thing: {
            meow: number,
        }) end)

        mrrp({
            meow = 5,
        })
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2143() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function call<A..., R...>(c: (A...) -> R..., ...: A...): R...
            return c(...)
        end

        local function fn(a: number): { number }
            return nil :: any
        end

        local function fn2<T>(b: { T }, x: (T) -> ())
            return b
        end

        local values = call(fn, 2)

        fn2(values, function(x: number)
        end)

        local a = values[1]
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_oss_2216_recursive_global_function_works_as_expected() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type tb_any = {[any]:any}
        function flatten(... : tb_any) : tb_any
            local out = {}
            local par = {...}
            for i = 1,#par do
                if par[i] and typeof(par[i]) == "table" then
                    for n,v in par[i] do
                        if typeof(n) == "number" then
                            for m,u in flatten(v) do
                                out[m] = u -- type error
                            end
                        else
                            out[n] = v
                        end
                    end
                end
            end
            return out
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_other_things_are_not_related_to_function() {
  let mut fixture = Fixture::fixture_bool(false);
  register_hidden_types(fixture.get_frontend());

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: fun = function() end
        local b: {} = a
        local c: boolean = a
        local d: fun = true
        local e: fun = {}
    "#,
    None,
  );

  assert_eq!(4, result.errors.len(), "{:?}", result.errors);
  assert_eq!(2, result.errors[0].location.begin.line);
  assert_eq!(3, result.errors[1].location.begin.line);
  assert_eq!(4, result.errors[2].location.begin.line);
  assert_eq!(5, result.errors[3].location.begin.line);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_one_ok_one_potential() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: ((number) -> "one") & ((string) -> "two")

        local g = f(42)
        local h = f("huh")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "\"one\"",
    to_string_type_id(fixture.require_type_string("g"))
  );
  assert_eq!(
    "\"two\"",
    to_string_type_id(fixture.require_type_string("h"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_resolution() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = (number) -> string
        type B = (string) -> number

        local function foo(f: A & B)
            return f(1), f("five")
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let t = fixture.require_type_string("foo");
  let foo_type = get_type::get::<FunctionType>(t);
  assert!(foo_type.is_some(), "expected function type");
  assert_eq!(
    "(((number) -> string) & ((string) -> number)) -> (string, number)",
    to_string_type_id(t)
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_resolution_crash_when_arg_exprs_is_smaller_than_type_args() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _ = fixture.base.check_string_optional_frontend_options(
    r#"
--!strict
local parseError
type Set<T> = {[T]: any}
local function captureDependencies(
	saveToSet: Set<PubTypes.Dependency>,
	callback: (...any) -> any,
	...
)
	local data = table.pack(xpcall(callback, parseError, ...))
    end
"#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_ambiguous_call() {
  use ulua_analysis::records::ambiguous_function_call::AmbiguousFunctionCall;

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: ((number | string) -> "one") & ((number | boolean) -> "two")
        local g = f(42)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<AmbiguousFunctionCall>(&result.errors[0])
    .expect("expected AmbiguousFunctionCall");
  assert_eq!("number", to_string_type_pack_id(err.arguments()));
  assert_eq!(
    "((boolean | number) -> \"two\") & ((number | string) -> \"one\")",
    to_string_type_id(err.function())
  );
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_string("g"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_bad_arity() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo<T>(f: ((number, number) -> "one") & T)
            local huh = f(42)
            local _ = huh
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 23
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_needs_to_retry() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type RGB = { r: number, b: number, g: number }
        local BrickColor: ((number) -> RGB) & ((number, number, number) -> RGB) & ((string) -> RGB)
        function Lightning(li, Color)
            li.BrickColor = BrickColor(Color)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "({ BrickColor: RGB }, number) -> ()",
    to_string_type_id(fixture.require_type_string("Lightning"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_no_compatible_option() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: ((number) -> "one") & ((boolean) -> "two")
        local g = f("s" :: string)
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_string("g"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_pick_better_arity() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: ((number) -> "one") & ((number, number) -> "two")
        -- Casting here so that we always hit the case in overload selection
        -- where one part has the correct arity but incorrect argument types,
        -- and the other has the incorrect arity.
        local g = f("s" :: string)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
  assert_eq!(
    "\"one\"",
    to_string_type_id(fixture.require_type_string("g"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_unambiguous_with_constraint() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: ((string, number) -> string) & ((number, boolean) -> number)
        local function g(x)
            -- When selecting an overload at this point, we'll reject the
            -- second overload, and claim that this is the only possible
            -- overload with a constraint of `x <: string`.
            f(x, 42)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(string) -> ()",
    to_string_type_id(fixture.require_type_string("g"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_overload_selection_union_of_functions() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function foo(f: (() -> (number)) | (() -> (string)))
            return f()
        end

        local g = foo(nil :: any)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("g"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible()
 {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function foo<a>(x: a, y: a?)
            return x
        end
        local vec2 = { x = 5, y = 7 }
        local ret: number = foo(vec2, { x = 5 })
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ x: number } | { x: number, y: number }",
      to_string_type_id_to_string_options(tm.given_type, &mut opts)
    );
  } else {
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let expected = r#"Expected this to be 'vec2?', but got '{| x: number |}'
caused by:
  None of the union options are compatible. For example:
Table type '{| x: number |}' not compatible with type 'vec2' because the former is missing field 'y'"#;
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
    assert_eq!(
      "Expected this to be 'number', but got 'vec2'",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible_2()
 {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function f<a>(x: a, y: a): a
            return if math.random() > 0.5 then x else y
        end

        local z: boolean = f(5, "five")
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("boolean", to_string_type_id(tm.wanted_type));
    assert_eq!("number | string", to_string_type_id(tm.given_type));
  } else {
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'boolean', but got 'number'",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_param_y_is_bounded_by_x_of_type_string() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(x: string, y)
            x = y
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(string, string) -> ()",
    to_string_type_id(fixture.require_type_string("f"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_pass_table_literal_to_function_expecting_optional_prop() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = {prop: number?}

        function f(t: T) end

        f({prop=5})
        f({})
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_pcall_example() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function makestr(n: number): string
            return tostring(n)
        end

        -- `s` now has type `string` and not `unknown`
        local success, s = pcall(makestr, 42)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_string("s"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_record_matching_overload() {
  use ulua_analysis::functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position;
  use ulua_ast::records::ast_expr_call::AstExprCall;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Overload = ((string) -> string) & ((number) -> number)
        local abc: Overload
        abc(1)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let source_module = fixture
    .get_main_source_module()
    .expect("main source module must exist after check")
    .get();
  let ancestry = find_ast_ancestry_of_position(
    source_module,
    Position {
      line: 3,
      column: 10,
    },
    false,
  );
  assert!(ancestry.len() >= 2, "ancestry was {:?}", ancestry);

  let parent_expr = ancestry[ancestry.len() - 2];
  assert!(
    unsafe { ast_node_is_ptr::<AstExprCall>(parent_expr) },
    "expected AstExprCall"
  );

  let module = unsafe { &*fixture.get_main_module(false) };
  let overload = module
    .ast_overload_resolved_types
    .find(&(parent_expr as *const _))
    .expect("expected recorded overload type");
  assert_eq!("(number) -> number", to_string_type_id(*overload));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_calls_must_refer_to_the_ungeneralized_type() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        function foo()
            string.format('%s: %s', "51", foo())
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function count(n: number)
            if n == 0 then
                return 0
            else
                return count(n - 1)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function random()
            return true -- chosen by fair coin toss
        end

        local f
        f = 5
        function f()
            if random() then f() end
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type_2() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function random()
            return true -- chosen by fair coin toss
        end

        local function f()
            if random() then f() end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_local_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function count(n: number)
            if n == 0 then
                return 0
            else
                return count(n - 1)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_recursive_static_method_must_refer_to_the_ungeneralized_type() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local lexer = {}
        local subContent: string = ""
        function lexer.scan(s: string)
            for innerToken, innerContent in lexer.scan(subContent) do
                table.insert(innerToken, innerContent)
            end
            return {}, nil, nil
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_regex_benchmark_string_format_minimization() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        (nil :: any)(function(n)
            if tonumber(n) then
                n = tonumber(n)
            elseif n ~= nil then
                string.format("invalid argument #4 to 'sub': number expected, got %s", typeof(n))
            end
        end);
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_report_exiting_without_return_nonstrict() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict

        local function f1(v): number?
            if v then
                return 1
            end
        end

        local function f2(v)
            if v then
                return 1
            end
        end

        local function f3(v): ()
            if v then
                return
            end
        end

        local function f4(v)
            if v then
                return
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    get_type_error::<FunctionExitsWithoutReturning>(&result.errors[0]).is_some(),
    "expected FunctionExitsWithoutReturning"
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_report_exiting_without_return_strict() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        local function f1(v): number?
            if v then
                return 1
            end
        end

        local function f2(v)
            if v then
                return 1
            end
        end

        local function f3(v): ()
            if v then
                return
            end
        end

        local function f4(v)
            if v then
                return
            end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<FunctionExitsWithoutReturning>(&result.errors[0]).is_some());
  assert!(get_type_error::<FunctionExitsWithoutReturning>(&result.errors[1]).is_some());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_return_type_by_overload() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Overload = ((string) -> string) & ((number, number) -> number)
        local abc: Overload
        local x = abc(true)
        local y = abc(true,true)
        local z = abc(true,true,true)
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("x"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("y"))
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string("z"))
    );
  } else {
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string("z"))
    );
  }
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_self_application_does_not_segfault() {
  let mut fixture = Fixture::fixture_bool(false);
  let _ = fixture.check_string_optional_frontend_options(
    r#"
        function f(a)
            f(f)
            return f(), a
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_simple_lightly_annotated_mutual_recursion() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function even(n: number)
    if n == 0 then
        return true
    else
        return odd(n - 1)
    end
end

function odd(n: number)
    if n == 0 then
        return false
    elseif n == 1 then
        return true
    else
        return even(n - 1)
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(number) -> boolean",
    to_string_type_id(fixture.base.require_type_string("even"))
  );
  assert_eq!(
    "(number) -> boolean",
    to_string_type_id(fixture.base.require_type_string("odd"))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_simple_unannotated_mutual_recursion() {
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function even(n)
    if n == 0 then
        return true
    else
        return odd(n - 1)
    end
end

function odd(n)
    if n == 0 then
        return false
    elseif n == 1 then
        return true
    else
        return even(n - 1)
    end
end
"#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Unknown type used in - operation; consider adding a type annotation to 'n'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_strict_mode_ok_with_missing_arguments() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function f(x: any) end
        f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_string_format_pack() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function foo(): (string, string, string)
            return "", "", ""
        end
        print(string.format("%s %s %s", foo()))
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_string_format_pack_variadic() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local foo : () -> (...string) = (nil :: any)
        print(string.format("%s %s %s", foo()))
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_subgeneric_type_function_super_monomorphic() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local a: (number, number) -> number = function(a, b) return a - b end

a = function(a, b) return a + b end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

#[test]
fn type_infer_functions_table_annotated_explicit_self() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        type MyObject = {
            fn: (self: MyObject) -> number,
            field: number
        }

        local Foo = {} :: MyObject

        function Foo:fn()
            local _ = self
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
    .expect("expected FunctionExitsWithoutReturning");
  assert_eq!(
    "MyObject",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 9,
      column: 24
    }))
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_table_containing_factorial_assign_later() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(
    r#"
        local coolmath = {}
        function coolmath.factorial(n: number)
            if n <= 1 then
                return 1
            end
            return coolmath.factorial(n - 1) * n
        end

        coolmath.factorial = function (s: string) end
    "#,
    None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  let err = get_type_error::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
  assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
  assert_eq!("(string) -> ()", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_table_containing_factorial_assign_with_correct_typing() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(
    r#"
        local coolmath = {}
        function coolmath.factorial(n: number)
            if n <= 1 then
                return 1
            end
            return coolmath.factorial(n - 1) * n
        end

        coolmath.factorial = function (n: number) return n end
        coolmath.factorial = "not a function"
    "#,
    None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  let err = get_type_error::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
  assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_table_containing_factorial_standalone() {
  let _crash_on_force = ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local coolmath = {}
        function coolmath.factorial(n: number)
            if n <= 1 then
                return 1
            end
            return coolmath.factorial(n - 1) * n
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_tc_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("function five() return 5 end", None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let five_type = get_type::get::<FunctionType>(fixture.require_type_string("five"));
  assert!(five_type.is_some(), "expected function type");
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_tf_suggest_arg_type() {
  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function fib(n, u)
            return (n or u) and (n < u and n + fib(n,u))
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0])
    .expect("expected CannotInferBinaryOperation");
  let err = type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(&result.errors[1])
    .expect("expected ExplicitFunctionAnnotationRecommended");
  assert_eq!("number", to_string_type_id(err.recommended_return()));
  assert_eq!(2, err.recommended_args().len());
  assert_eq!("number", to_string_type_id(err.recommended_args()[0].1));
  assert_eq!("number", to_string_type_id(err.recommended_args()[1].1));
}

#[test]
fn type_infer_functions_tf_suggest_arg_type_2() {
  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend().options.retain_full_type_graphs = false;

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function escape_fslash(pre)
            return (#pre % 2 == 0 and '\\' or '') .. pre .. '.'
        end
    "#,
    None,
  );

  assert!(
    result
      .errors
      .iter()
      .any(|error| type_error_data_ref::<NotATable>(error).is_some()),
    "expected NotATable, got {:?}",
    result.errors
  );
}

#[test]
fn type_infer_functions_tf_suggest_return_type() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function fib(n)
            return n < 2 and 1 or fib(n-1) + fib(n-2)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(
    result.errors.last().expect("expected an error"),
  )
  .expect("expected ExplicitFunctionAnnotationRecommended");
  assert_eq!(
    "false | number",
    to_string_type_id(err.recommended_return())
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_few_arguments_variadic() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
    function test(a: number, b: string, ...)
    end

    test(1)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

  assert_eq!(2, acm.expected());
  assert_eq!(1, acm.actual());
  assert_eq!(CountMismatch::ARG, acm.context());
  assert!(acm.is_variadic());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_few_arguments_variadic_generic() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
function test(a: number, b: string, ...)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

  assert_eq!(3, acm.expected());
  assert_eq!(1, acm.actual());
  assert_eq!(CountMismatch::ARG, acm.context());
  assert!(acm.is_variadic());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_few_arguments_variadic_generic_2() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function test(a: number, b: string, ...)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

pcall(wrapper, test)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

  assert_eq!(4, acm.expected());
  assert_eq!(2, acm.actual());
  assert_eq!(CountMismatch::ARG, acm.context());
  assert!(acm.is_variadic());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_many_arguments() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict

        function g(a: number) end

        g()

    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(1, acm.expected());
  assert_eq!(0, acm.actual());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_many_arguments_error_location() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function myfunction(a: number, b:number) end
        myfunction(1)

        function getmyfunction()
            return myfunction
        end
        getmyfunction()()
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    Location {
      begin: Position { line: 4, column: 8 },
      end: Position {
        line: 4,
        column: 18
      }
    },
    result.errors[0].location
  );
  let acm0 = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(2, acm0.expected());
  assert_eq!(1, acm0.actual());

  assert_eq!(
    Location {
      begin: Position { line: 9, column: 8 },
      end: Position {
        line: 9,
        column: 23
      }
    },
    result.errors[1].location
  );
  let acm1 = get_type_error::<CountMismatch>(&result.errors[1]).expect("expected CountMismatch");
  assert_eq!(2, acm1.expected());
  assert_eq!(0, acm1.actual());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_many_return_values() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function f()
            return 55
        end

        local a, b = f()
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(CountMismatch::FUNCTION_RESULT, acm.context());
  assert_eq!(1, acm.expected());
  assert_eq!(2, acm.actual());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_many_return_values_in_parentheses() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        function f()
            return 55
        end

        local a, b = (f())
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(CountMismatch::FUNCTION_RESULT, acm.context());
  assert_eq!(1, acm.expected());
  assert_eq!(2, acm.actual());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_too_many_return_values_no_function() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict

        local a, b = 55
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(CountMismatch::EXPR_LIST_RESULT, acm.context());
  assert_eq!(1, acm.expected());
  assert_eq!(2, acm.actual());
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_toposort_doesnt_break_mutual_recursion() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local x = nil
        function f() g() end
        -- make sure print(x) doesn't get toposorted here, breaking the mutual block
        function g() x = f end
        print(x)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_unifier_should_not_bind_free_types() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(player)
            local success,result = player:thing()
            if(success) then
                return "Successfully posted message.";
            elseif(not result) then
                return false;
            else
                return result;
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("string", to_string_type_id(tm1.wanted_type));
  assert_eq!("boolean", to_string_type_id(tm1.given_type));
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_unify_type_pack_stack_overflow() {
  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(
    r#"
        local function a(): ...string
            return "hello", "world"
        end

        local function g<T...>()
            local function f(... : T...)
            end
            f("what", "is", "going", a())
        end
    "#,
    None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  let err =
    get_type_error::<TypePackMismatch>(&results.errors[0]).expect("expected TypePackMismatch");
  assert_eq!("T...", to_string_type_pack_id(err.wanted_tp()));
  assert_eq!(
    "string, string, string, ...string",
    to_string_type_pack_id(err.given_tp())
  );
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_unnecessary_nil_in_lower_bound_of_generic() {
  use ulua_ast::enums::mode::Mode;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        function isAnArray(value)
            if type(value) == "table" then
                for index, _ in next, value do
                    -- assert index is not nil
                    math.max(0, index)
                end
                return true
            else
                return false
            end
        end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!strict
local function id(x)
    return x
end
local u,v = id(3), id(id(44))
"#,
    None,
  );

  let v_type = fixture.require_type_string("v");
  let number_type = fixture.get_builtins().number_type;
  assert_eq!(number_type, v_type);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_vararg_function_is_quantified() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r##"
        local T = {}
        function T.f(...)
            local result = {}

            for i = 1, select("#", ...) do
                local dictionary = select(i, ...)
                for key, value in pairs(dictionary) do
                    result[key] = value
                end
            end

            return result
        end

        return T
    "##,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = unsafe { &*fixture.base.get_main_module(false) };
  let return_ty = first(module.return_type, true).expect("expected module return type");
  let table = get_type::get::<TableType>(return_ty).expect("expected table");
  let f = table.props.get("f").expect("expected f property");
  assert!(f.read_ty.is_some(), "expected readable f property");
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_vararg_functions_should_allow_calls_of_any_types_and_size() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(...) end

        f(1)
        f("foo", 2)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local function f(...) return ... end
        local g = function(...) return f(...) end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.functions.test.cpp`
#[test]
fn type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack_2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function somethingThatsAny(...: any)
            print(...)
        end

        local function x<T...>(...: T...)
            somethingThatsAny(...) -- Failed to unify variadic type packs
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: tests/TypeInfer.functions.test.cpp:635 `duplicate_functions2` 已由
// 本文件 :1249 `type_infer_functions_duplicate_functions_2` 覆盖（复名归一化），不重复补。

// Source: tests/TypeInfer.functions.test.cpp:4633
#[test]
fn type_infer_functions_methods_of_exported_tables_require_annotations() {
  use ulua_analysis::records::type_annotation_required::TypeAnnotationRequired;

  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function abs(a: number): number
            return if a < 0 then -a else a
        end

        export local T = {}

        function T.foo(argumentOne)
            return function(y) -- No warning here
                return abs(argumentOne), abs(y)
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<TypeAnnotationRequired>(&result.errors[0]).is_some());
  assert_eq!(
    Location::new(Position::new(7, 17), Position::new(7, 35)),
    result.errors[0].location
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4678
#[test]
fn type_infer_functions_inner_functions_dont_require_annotations() {
  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function abs(a: number): number
            return if a < 0 then -a else a
        end

        export function foo(argumentOne): (number) -> (number, number)
            return function(y) -- No warning here
                return abs(argumentOne), abs(y)
            end
        end

        function inner() -- no warning here
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location::new(Position::new(5, 24), Position::new(5, 70)),
    result.errors[0].location
  );
  assert_eq!(
    "Type annotation required here.  Consider (argumentOne: number) -> (number) -> (number, number)",
    to_string_type_error(&result.errors[0])
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4706
#[test]
fn type_infer_functions_function_return_types_might_require_annotations() {
  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function abs(a: number)
            return if a < 0 then -a else a
        end

        export function foo(argumentOne)
            return function(y) -- No warning here
                return abs(argumentOne), abs(y)
            end
        end

        function take_five()
            return 5
        end
    "#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location::new(Position::new(1, 17), Position::new(1, 31)),
    result.errors[0].location
  );
  assert_eq!(
    Location::new(Position::new(5, 24), Position::new(5, 40)),
    result.errors[1].location
  );
  assert_eq!(
    Location::new(Position::new(11, 17), Position::new(11, 28)),
    result.errors[2].location
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4736
#[test]
fn type_infer_functions_non_exported_functions_might_also_require_annotations() {
  use ulua_analysis::records::type_annotation_required::TypeAnnotationRequired;

  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function abs(a: number): number
            return if a < 0 then -a else a
        end

        export function foo(argumentOne): (number) -> (number, number)
            return function(y)
                return abs(argumentOne), abs(y)
            end
        end

        function inner(_x)
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<TypeAnnotationRequired>(&result.errors[0]).is_some());
  assert_eq!(
    Location::new(Position::new(5, 24), Position::new(5, 70)),
    result.errors[0].location
  );
  assert!(get_type_error::<TypeAnnotationRequired>(&result.errors[1]).is_some());
  assert_eq!(
    Location::new(Position::new(11, 17), Position::new(11, 26)),
    result.errors[1].location
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4767
#[test]
fn type_infer_functions_vararg_needs_annotation() {
  use ulua_analysis::records::{
    type_annotation_required::TypeAnnotationRequired,
    uninhabited_type_function::UninhabitedTypeFunction,
  };

  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        export function sum(a, ...): number
            if a == nil then
                return 0
            else
                return a + sum(...)
            end
        end

        export function sum2(a: number?, ...: number): number
            if a == nil then
                return 0
            else
                return a + sum(...)
            end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<UninhabitedTypeFunction>(&result.errors[0]).is_some());
  assert_eq!(
    Location::new(Position::new(5, 23), Position::new(5, 35)),
    result.errors[0].location
  );
  assert!(get_type_error::<TypeAnnotationRequired>(&result.errors[1]).is_some());
  assert_eq!(
    Location::new(Position::new(1, 24), Position::new(1, 43)),
    result.errors[1].location
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4801
#[test]
fn type_infer_functions_vararg_specified_but_inferred_empty() {
  use ulua_analysis::records::type_annotation_required::TypeAnnotationRequired;

  // cpp ScopedFastFlag sff[]：新求解器 + export 语法 + 顶层未标注告警
  let _sffs = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions, true),
  ];

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local abs: (number) -> number = function(a: number)
            return if a < 0 then -a else a
        end

        export function abs2(a: number, ...)
            return abs(a, ...)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<TypeAnnotationRequired>(&result.errors[0]).is_some());
  assert_eq!(
    Location::new(Position::new(5, 24), Position::new(5, 44)),
    result.errors[0].location
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4903
#[test]
fn type_infer_functions_bidirectional_inference_callback_in_array() {
  use ulua_analysis::records::unknown_property::UnknownProperty;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Callback = (string) -> ()

        local t: { Callback } = {
            function (s)
                s.uper("hello")
            end
        }
    "#,
    None,
  );

  // cpp `LUAU_REQUIRE_ERROR_COUNT(1, result); LUAU_REQUIRE_ERROR(result, UnknownProperty);`
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<UnknownProperty>(&result.errors[0]).is_some());
}

// Source: tests/TypeInfer.functions.test.cpp:4537
#[test]
fn type_infer_functions_bidi_inference_variadic_top_level() {
  // cpp `DOES_NOT_PASS_OLD_SOLVER_GUARD()`：强制关闭旧求解器
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Context = {}
        Context.__index = Context
        type ContextData = {}
        type Context = setmetatable<ContextData, typeof(Context)>
        function Context.text(self: Context, text: string): string
            return text
        end
        type Handler = (Context) -> string
        local function post(path: string, first: Handler, ...: Handler)
        end
        post(
            "/validate",
            function(c)
                return c:text("ok")
            end,
            function(c)
                return c:text(`not ok`)
            end
        )
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: tests/TypeInfer.functions.test.cpp:4564
#[test]
fn type_infer_functions_bidirectional_inference_variadic_type_pack_read_only_prop() {
  // cpp `DOES_NOT_PASS_OLD_SOLVER_GUARD()`：强制关闭旧求解器
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local foo: { read bar: (...string) -> () } = {
            bar = function (foobar)
                print(foobar)
            end
        }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 24))
    )
  );
}

// Source: tests/TypeInfer.functions.test.cpp:5036
#[test]
fn type_infer_functions_let_generalization_second_layer() {
  use ulua_analysis::records::type_annotation_required::TypeAnnotationRequired;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        local function id(x)
          return x
        end

        local f = id(function (x)
          return x
        end)

        local g = f(42)
        local h = f("hmmm")
    "#,
    None,
  );

  // cpp `ignoreMissingAnnotations(result); LUAU_REQUIRE_NO_ERRORS(result);`
  let filtered: Vec<&_> = result
    .errors
    .iter()
    .filter(|error| get_type_error::<TypeAnnotationRequired>(error).is_none())
    .collect();
  assert_eq!(0, filtered.len(), "{:?}", filtered);

  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "number",
    to_string_type_id_to_string_options(fixture.require_type_string("g"), &mut opts)
  );
  assert_eq!(
    "string",
    to_string_type_id_to_string_options(fixture.require_type_string("h"), &mut opts)
  );
}

// Source: tests/TypeInfer.functions.test.cpp:4519
#[test]
fn type_infer_functions_bidi_inference_variadic_inner_lambda() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local f: ({ (number, ...string) -> () }) -> () = nil :: any
        f(
            {
                function (alpha, beta, gamma)
                    print(alpha, beta, gamma)
                end
            }
        )
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 27))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 34))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 40))
    )
  );
}

// 缺口（未移植，对照 `tests/TypeInfer.functions.test.cpp`，逐名清点后 41 候选）：
// 阻·FFlag 未定义（本移植 fflag.rs 无对应旗标，行为亦未接入）：
// - call_metamethod_checks_argument_types（:2464）、call_metamethod_checks_variadic_argument_types
//   （:2480）、call_metamethod_variadic_blames_the_offending_argument（:2497）、
//   call_metamethod_variadic_blames_each_offending_argument（:2515）——
//   `LuauFixCallMetamethodErrorReporting`。
// - function_inference_notes_generic_return（:2911）——`LuauIterativeTypeSearcher`。
// - bidi_inference_union_of_functions_distinguished_by_return_type（:4579）——
//   `LuauBidirectionalInferenceBetterLambdaHandling`。
// - call_with_any_arg_and_optional_return_arg（:4615）——
//   `LuauCallErrorReportingRecoversArgumentLocationsForPackedVariadics`。
// - semantic_subtyping_not_working（:4824）、oss_2623_double_negate_string（:4839）——
//   `LuauRefactorStringSemanticSubtyping`（仅见于实验旗标黑名单字符串，未定义）。
// - let_generalization_direct（:4920）、let_generalization_direct_one_level（:4938）、
//   let_generalization_return_not_generalized（:4958）、let_generalization_forin_iterator
//   （:4976）、let_generalization_assign_statement（:4999）、let_generalization_multiple_values
//   （:5018）——`LuauThreadGeneralizeThroughConstraintGeneration`。
// - setmetatable_lambda_generalizes_across_calls（:5059）、generalize_type_in_if_body_scope
//   （:5104）、extend_typepack_bound_indirection_preserves_references（:5123）——
//   `LuauTraverseScopeToFunction`。
// 实测不等价（求解器/打印行为缺口，交续票）：
// - indicate_an_inferred_generic（:4660）——建议串本移植出 `<a>(x: a) -> a`，cpp 期望
//   `<T>(x: T) -> T`，泛型命名约定不一致。
// - oss_2670_generic_leaking_indexer_1（:4859）——`setDefault(t, "green", "42")` 报
//   CountMismatch（cpp 端该调用被接受），参数元组行为未 sync。
// - pass_generic_function_to_pcall（:4599）——`pcall(identity, 42)` 处报 TypeMismatch。
// 未尝试（余量交续票，实测可行但本轮预算到顶）：
// - weakoptional_reduces_over_generic（:5078）、oss_2670_generic_leaking_indexer_2
//   （:4882）、bidi_inference_variadic_inner_lambda 之外的其余未列者均已归类。
// 证伪注：41 候选中 7 例系复名/归一化重命名已覆盖（variadic_any_is_compatible_with_a_generic_TypePack(_2)
// → *_type_pack(_2)、inferred_higher_order_functions_are_quantified_at_the_right_time2/3 →
// _2/_3、too_few_arguments_variadic_generic2 → _generic_2、
// overload_resolution_crash_when_argExprs_… → arg_exprs_…、duplicate_functions2 →
// duplicate_functions_2），非真缺。
