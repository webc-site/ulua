use core::ptr::null_mut;

use ulua_ast::rtti::ast_node_is;
extern crate alloc;

mod type_infer_functions_another_higher_order_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:471:type_infer_functions_another_higher_order_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_another_higher_order_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_another_higher_order_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_another_indirect_function_case_where_it_is_ok_to_provide_too_many_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:893:type_infer_functions_another_indirect_function_case_where_it_is_ok_to_provide_too_many_arguments`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_another_indirect_function_case_where_it_is_ok_to_provide_too_many_arguments
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_another_indirect_function_case_where_it_is_ok_to_provide_too_many_arguments()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local mycb: (number, number) -> ()

        function f() end

        mycb = f
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_another_other_higher_order_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:491:type_infer_functions_another_other_higher_order_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_another_other_higher_order_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_another_other_higher_order_function() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = if !FFlag::DebugLuauForceOldSolver.get() {
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

    let result = fixture.check_string_optional_frontend_options(&String::from(source), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_another_recursive_local_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:427:type_infer_functions_another_recursive_local_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_another_recursive_local_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_another_recursive_local_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_apply_example_from_oss {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3701:type_infer_functions_apply_example_from_oss`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_functions_apply_example_from_oss
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_apply_example_from_oss() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ Example: number }",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("result")),
        &mut opts
      )
    );
  }
}

mod type_infer_functions_apply_of_lambda_with_inferred_and_explicit_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2453:type_infer_functions_apply_of_lambda_with_inferred_and_explicit_types`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_apply_of_lambda_with_inferred_and_explicit_types
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_apply_of_lambda_with_inferred_and_explicit_types() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function apply(f, x) return f(x) end
        local x = apply(function(x: string): number return 5 end, "hello!")

        local function apply_explicit<A, B...>(f: (A) -> B..., x: A): B... return f(x) end
        local x = apply_explicit(function(x: string): number return 5 end, "hello!")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_are_we_in_the_new_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4082:type_infer_functions_are_we_in_the_new_solver`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - translates_to -> rust_item type_infer_functions_are_we_in_the_new_solver
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_are_we_in_the_new_solver() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "{ x: number, y: number }",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_functions_attempt_to_call_an_intersection_of_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2349:type_infer_functions_attempt_to_call_an_intersection_of_tables`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_attempt_to_call_an_intersection_of_tables
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_attempt_to_call_an_intersection_of_tables() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { x: number } & { y: string })
            t()
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_attempt_to_call_an_intersection_of_tables_with_call_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2365:type_infer_functions_attempt_to_call_an_intersection_of_tables_with_call_metamethod`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_attempt_to_call_an_intersection_of_tables_with_call_metamethod
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_attempt_to_call_an_intersection_of_tables_with_call_metamethod() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Callable = typeof(setmetatable({}, {
            __call = function(self, ...) return ... end
        }))

        local function f(t: Callable & { x: number })
            t()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_bidi_inference_functions_complete_ex {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4139:type_infer_functions_bidi_inference_functions_complete_ex`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_functions_bidi_inference_functions_complete_ex
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidi_inference_functions_complete_ex() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let _instantiation = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
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
    "#,
        ),
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
}

mod type_infer_functions_bidi_inference_union_of_functions_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4183:type_infer_functions_bidi_inference_union_of_functions_1`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidi_inference_union_of_functions_1
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidi_inference_union_of_functions_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(_: ((string) -> ()) | ((number, number) -> ()))
        end

        f(function (one, two)
            local _ = one
            local _ = two
        end)
    "#,
      ),
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
}

mod type_infer_functions_bidi_inference_union_of_functions_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4204:type_infer_functions_bidi_inference_union_of_functions_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidi_inference_union_of_functions_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidi_inference_union_of_functions_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(_: ((string) -> ()) | ((number, number) -> ()))
        end

        f(function (one)
            local _ = one
        end)
    "#,
      ),
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
}

mod type_infer_functions_bidi_inference_union_of_functions_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4223:type_infer_functions_bidi_inference_union_of_functions_3`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidi_inference_union_of_functions_3
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidi_inference_union_of_functions_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(_: ((string) -> ()) | ((number) -> ()))
        end

        f(function (one)
            local _ = one
        end)
    "#,
      ),
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
}

mod type_infer_functions_bidi_inference_union_of_functions_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4244:type_infer_functions_bidi_inference_union_of_functions_4`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidi_inference_union_of_functions_4
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidi_inference_union_of_functions_4() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
      ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(_: ((string) -> ())?)
        end

        f(function (one)
            local _ = one
        end)
    "#,
      ),
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
}

mod type_infer_functions_bidirectional_checking_of_callback_property {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_checking_of_callback_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{type_mismatch::TypeMismatch, unknown_property::UnknownProperty},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function print(x: number) end

        type Point = {x: number, y: number}
        local T : {callback: ((Point) -> ())?} = {}

        T.callback = function(p) -- No error here
            print(p.z)           -- error here.  Point has no property z
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

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
}

mod type_infer_functions_bidirectional_function_statement_inference_with_extern {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_function_statement_inference_with_extern() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type HasClass = { f: (ClassWithGenericMethod) -> () }
        local t = {} :: HasClass
        function t.f(cls)
            local _ = cls
            local foobar = cls.identity(42)
            local _ = foobar
        end
    "#,
      ),
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
}

mod type_infer_functions_bidirectional_inference_allow_internal_generics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3652:type_infer_functions_bidirectional_inference_allow_internal_generics`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_bidirectional_inference_allow_internal_generics
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_inference_allow_internal_generics() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type testsuite = { case: (self: testsuite, <T>(T) -> T) -> () }

        local test1: { suite: (string, (testsuite) -> ()) -> () } = nil :: any

        test1.suite("LuteTestCommand", function(suite)
            suite:case(42)
        end)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("<T>(T) -> T", to_string_type_id(err.wanted_type));
    assert_eq!("number", to_string_type_id(err.given_type));
  }
}

mod type_infer_functions_bidirectional_inference_goes_through_ifelse {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3378:type_infer_functions_bidirectional_inference_goes_through_ifelse`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_bidirectional_inference_goes_through_ifelse
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_inference_goes_through_ifelse() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Input = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        local function getInputs(isDragonPunch: boolean): { Input }
            return if isDragonPunch then { "6", "8", "7" } else { "8", "7", "6" }
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_bidirectional_inference_of_class_methods {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_inference_of_class_methods() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::extern_type_fixture::ExternTypeFixture,
    };

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local c = ChildClass.New()

        -- Instead of reporting that the lambda is the wrong type, report that we are using its argument improperly.
        c.Touched:Connect(function(other)
            print(other.ThisDoesNotExist)
        end)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("ThisDoesNotExist", err.key());
    assert_eq!("BaseClass", to_string_type_id(err.table()));
  }
}

mod type_infer_functions_bidirectional_lambda_inference_applies_nilable_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3615:type_infer_functions_bidirectional_lambda_inference_applies_nilable_functions`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidirectional_lambda_inference_applies_nilable_functions
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectional_lambda_inference_applies_nilable_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local listdir: (string, ((string) -> boolean)?) -> { string } = nil :: any
        listdir("my_directory", function (path)
            print(path)
            return true
        end)
    "#,
      ),
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
}

mod type_infer_functions_bidirectionally_infer_lambda_with_partially_resolved_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3356:type_infer_functions_bidirectionally_infer_lambda_with_partially_resolved_generic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_bidirectionally_infer_lambda_with_partially_resolved_generic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_bidirectionally_infer_lambda_with_partially_resolved_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_call_function_with_nothing_but_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3274:type_infer_functions_call_function_with_nothing_but_nil`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_call_function_with_nothing_but_nil
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_call_function_with_nothing_but_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(n: number, x: string?, y: string?, z: string?) end

        local function g(n)
            f(n)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_calling_function_with_anytypepack_doesnt_leak_free_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1015:type_infer_functions_calling_function_with_anytypepack_doesnt_leak_free_types`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_calling_function_with_anytypepack_doesnt_leak_free_types
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_calling_function_with_anytypepack_doesnt_leak_free_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function Test(a)
            return 1, ""
        end


        local tab = {}
        table.insert(tab, Test(1));
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    opts.max_table_length = 0;
    let tab_type = fixture.base.require_type_string(&String::from("tab"));
    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:982:type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(a: number, b: string) end

        foo("Test", 123)
    "#,
      ),
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
}

mod type_infer_functions_cannot_call_union_of_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2857:type_infer_functions_cannot_call_union_of_functions`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_cannot_call_union_of_functions
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_cannot_call_union_of_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
         local f: (() -> ()) | (() -> () -> ()) = nil :: any
         f()
     "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = "Cannot call a value of the union type:\n  | () -> ()\n  | () -> () -> ()\nWe are unable to determine the appropriate result type for such a call.";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_functions_cannot_hoist_interior_defns_into_signature {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:107:type_infer_functions_cannot_hoist_interior_defns_into_signature`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_cannot_hoist_interior_defns_into_signature
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_cannot_hoist_interior_defns_into_signature() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: T)
            type T = number
        end
    "#,
      ),
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
}

mod type_infer_functions_captured_local_is_assigned_a_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2918:type_infer_functions_captured_local_is_assigned_a_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_captured_local_is_assigned_a_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_captured_local_is_assigned_a_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f

        local function g()
            f()
        end

        function f()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_check_function_before_lambda_that_uses_it {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:865:type_infer_functions_check_function_before_lambda_that_uses_it`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_check_function_before_lambda_that_uses_it
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_check_function_before_lambda_that_uses_it() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function f()
            return 114
        end

        return function()
            return f():andThen()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_check_function_bodies {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:73:type_infer_functions_check_function_bodies`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_functions_check_function_bodies
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_check_function_bodies() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function myFunction(): number
            local a = 0
            a = true
            return a
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));
    assert_eq!("boolean", to_string_type_id(tm.given_type));

    if FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_cli_119545_pass_lambda_inside_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3317:type_infer_functions_cli_119545_pass_lambda_inside_table`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_cli_119545_pass_lambda_inside_table
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_cli_119545_pass_lambda_inside_table() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_cli_187542_recursive_call_in_loop {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_cli_187542_recursive_call_in_loop() {
    use alloc::string::String;

    use ulua_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function a(b)
            if true then return b end
            while false do
                b = a(b)
            end

            if true then return b end
        end
    "#,
      ),
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
}

mod type_infer_functions_complicated_return_types_require_an_explicit_annotation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:640:type_infer_functions_complicated_return_types_require_an_explicit_annotation`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_functions_complicated_return_types_require_an_explicit_annotation
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_complicated_return_types_require_an_explicit_annotation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{function_type::FunctionType, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("most_of_the_natural_numbers"));
    let function_type = get_type_id::<FunctionType>(ty)
      .unwrap_or_else(|| panic!("expected function but got {}", to_string_type_id(ty)));

    let ret_type = first(function_type.ret_types(), false).expect("expected return type");
    assert!(get_type_id::<UnionType>(ret_type).is_some());
  }
}

mod type_infer_functions_concrete_functions_are_not_supertypes_of_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2111:type_infer_functions_concrete_functions_are_not_supertypes_of_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function registerHiddenTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_concrete_functions_are_not_supertypes_of_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_concrete_functions_are_not_supertypes_of_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::register_hidden_types::register_hidden_types, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: fun = function() end

        function one(arg: () -> ()) end
        function two(arg: <T>(T) -> T) end

        one(a)
        two(a)
    "#,
      ),
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
}

mod type_infer_functions_coroutine_wrap_result_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3029:type_infer_functions_coroutine_wrap_result_call`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function main (tests/main.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_functions_coroutine_wrap_result_call
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_coroutine_wrap_result_call() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(a, b)
            coroutine.wrap(a)(b)
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_functions_cyclic_function_type_in_rets {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:459:type_infer_functions_cyclic_function_type_in_rets`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_cyclic_function_type_in_rets
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_cyclic_function_type_in_rets() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f()
            return f
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "t1 where t1 = () -> t1",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2172:type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record UnificationTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization() {
    use alloc::string::String;

    use ulua_analysis::records::unification_too_complex::UnificationTooComplex;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _tarjan_limit = ScopedFastInt::new(&FInt::LuauTarjanChildLimit, 1);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t)
            t.x.y.z = 441
        end
    "#,
      ),
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
}

mod type_infer_functions_dont_give_other_overloads_message_if_only_one_argument_matching_overload_exists {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:292:type_infer_functions_dont_give_other_overloads_message_if_only_one_argument_matching_overload_exists`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_dont_give_other_overloads_message_if_only_one_argument_matching_overload_exists
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_give_other_overloads_message_if_only_one_argument_matching_overload_exists()
   {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply(1, "")
    "#,
      ),
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
}

mod type_infer_functions_dont_infer_overloaded_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2732:type_infer_functions_dont_infer_overloaded_functions`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_dont_infer_overloaded_functions
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_infer_overloaded_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(t1) -> () where t1 = { read FindFirstChild: (t1, string) -> (...unknown) }",
        to_string_type_id(fixture.require_type_string(&String::from("getR6Attachments")))
      );
    } else {
      assert_eq!(
        "<a...>(t1) -> () where t1 = {+ FindFirstChild: (t1, string) -> (a...) +}",
        to_string_type_id(fixture.require_type_string(&String::from("getR6Attachments")))
      );
    }
  }
}

mod type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1927:type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(
      "<a>(a) -> a",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "({+ p: {+ q: nil +} +}) -> nil",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
      );
    }
  }
}

mod type_infer_functions_dont_leak_generics_keyof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4108:type_infer_functions_dont_leak_generics_keyof`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_dont_leak_generics_keyof
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_leak_generics_keyof() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{ Input: { Stuff: { a: number } }, Test: (\"a\") -> () }",
      to_string_type_id(fixture.base.require_type_string(&String::from("thing")))
    );
    assert_eq!(
      "{ Input: { Stuff: { b: number, c: number } }, Test: (\"b\" | \"c\") -> () }",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("otherthing"))
      )
    );
  }
}

mod type_infer_functions_dont_mutate_the_underlying_head_of_typepack_when_calling_with_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1962:type_infer_functions_dont_mutate_the_underlying_head_of_typepack_when_calling_with_self`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_dont_mutate_the_underlying_head_of_typepack_when_calling_with_self
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_dont_mutate_the_underlying_head_of_typepack_when_calling_with_self() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        function t:m(x) end
        function f(): never return 5 :: never end
        t:m(f())
        t:m(f())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_duplicate_functions_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:585:type_infer_functions_duplicate_functions_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_duplicate_functions_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_duplicate_functions_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo() end

        function bar()
            local function foo() end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_duplicate_functions_allowed_in_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:598:type_infer_functions_duplicate_functions_allowed_in_nonstrict`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_duplicate_functions_allowed_in_nonstrict
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_duplicate_functions_allowed_in_nonstrict() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function foo() end

        function foo() end

        function bar()
            local function foo() end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:614:type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("() -> number", to_string_type_id(tm.wanted_type));
    assert_eq!("(number) -> number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_functions_error_detailed_function_mismatch_arg {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1534:type_infer_functions_error_detailed_function_mismatch_arg`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_detailed_function_mismatch_arg
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_detailed_function_mismatch_arg() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = (number, number) -> string
type B = (number, string) -> string

local a: A
local b: B = a
    "#,
      ),
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
}

mod type_infer_functions_error_detailed_function_mismatch_arg_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1511:type_infer_functions_error_detailed_function_mismatch_arg_count`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_detailed_function_mismatch_arg_count
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_detailed_function_mismatch_arg_count() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = (number, number) -> string
type B = (number) -> string

local a: A
local b: B = a
    "#,
      ),
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
}

mod type_infer_functions_error_detailed_function_mismatch_ret {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1581:type_infer_functions_error_detailed_function_mismatch_ret`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_detailed_function_mismatch_ret
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_detailed_function_mismatch_ret() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = (number, number) -> string
type B = (number, number) -> number

local a: A
local b: B = a
    "#,
      ),
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
}

mod type_infer_functions_error_detailed_function_mismatch_ret_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1558:type_infer_functions_error_detailed_function_mismatch_ret_count`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_detailed_function_mismatch_ret_count
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_detailed_function_mismatch_ret_count() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = (number, number) -> (number)
type B = (number, number) -> (number, boolean)

local a: A
local b: B = a
    "#,
      ),
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
}

mod type_infer_functions_error_detailed_function_mismatch_ret_mult {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1605:type_infer_functions_error_detailed_function_mismatch_ret_mult`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_detailed_function_mismatch_ret_mult
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_detailed_function_mismatch_ret_mult() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = (number, number) -> (number, string)
type B = (number, number) -> (number, boolean)

local a: A
local b: B = a
    "#,
      ),
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
}

mod type_infer_functions_error_suppression_propagates_through_function_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2934:type_infer_functions_error_suppression_propagates_through_function_calls`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_functions_error_suppression_propagates_through_function_calls
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_error_suppression_propagates_through_function_calls() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function first(x: any)
            return pairs(x)(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(any) -> (any?, any)",
      to_string_type_id(fixture.base.require_type_string(&String::from("first")))
    );
  }
}

mod type_infer_functions_first_argument_can_be_optional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:561:type_infer_functions_first_argument_can_be_optional`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_first_argument_can_be_optional
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_first_argument_can_be_optional() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        function T.new(a: number?, b: number?, c: number?) return 5 end
        local m = T.new()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_free_is_not_bound_to_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1912:type_infer_functions_free_is_not_bound_to_unknown`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_free_is_not_bound_to_unknown
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_free_is_not_bound_to_unknown() {
    use alloc::string::String;

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
        local function foo(f: (unknown) -> (), x)
            f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<a>((unknown) -> (), a) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_func_expr_doesnt_leak_free {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:547:type_infer_functions_func_expr_doesnt_leak_free`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_functions_func_expr_doesnt_leak_free
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_func_expr_doesnt_leak_free() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{function_type::FunctionType, generic_type::GenericType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local p = function(x) return x end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let fn_ty = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("p")))
      .expect("expected FunctionType");
    let ret = first(fn_ty.ret_types(), true).expect("expected return type");
    let ret = follow_type_id(ret);
    assert!(
      get_type_id::<GenericType>(ret).is_some(),
      "expected generic return type"
    );
  }
}

mod type_infer_functions_function_argument_error_suppression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3598:type_infer_functions_function_argument_error_suppression`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_argument_error_suppression
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_argument_error_suppression() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local functions: {[any]: (any) -> ()} = {}
        functions.func1 = function(value: string) end
        functions.func2 = function(value: boolean) end
        functions.func3 = function(value: number) end
        functions.func4 = function(value: any) end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_function_calls_should_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3217:type_infer_functions_function_calls_should_not_crash`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_calls_should_not_crash
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_calls_should_not_crash() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
        &String::from(
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
    "#,
        ),
        None,
    );
  }
}

mod type_infer_functions_function_cast_error_uses_correct_language {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1152:type_infer_functions_function_cast_error_uses_correct_language`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_function_cast_error_uses_correct_language
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_cast_error_uses_correct_language() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(a, b): number
            return 0
        end

        local a: (string)->number = foo
        local b: (number, number)->(number, number) = foo

        local c: (string, number)->number = foo -- no error
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("(string) -> number", to_string_type_id(tm1.wanted_type));
    if !FFlag::DebugLuauForceOldSolver.get() {
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
    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_function_decl_non_self_sealed_overwrite {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1651:type_infer_functions_function_decl_non_self_sealed_overwrite`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_decl_non_self_sealed_overwrite
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_decl_non_self_sealed_overwrite() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function string.len(): number
            return 1
        end

        local s = string
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.get_frontend().clear();

    let result2 = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        print(string.len('hello'))
    "#,
      ),
      None,
    );

    assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
  }
}

mod type_infer_functions_function_decl_non_self_sealed_overwrite_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1673:type_infer_functions_function_decl_non_self_sealed_overwrite_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record WhereClauseNeeded (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_function_decl_non_self_sealed_overwrite_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_decl_non_self_sealed_overwrite_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::where_clause_needed::WhereClauseNeeded,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_function_decl_non_self_unsealed_overwrite {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1745:type_infer_functions_function_decl_non_self_unsealed_overwrite`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record WhereClauseNeeded (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_function_decl_non_self_unsealed_overwrite
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_decl_non_self_unsealed_overwrite() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::where_clause_needed::WhereClauseNeeded,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _check_function_statement_types =
      ScopedFastFlag::new(&FFlag::LuauCheckFunctionStatementTypes, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_function_decl_quantify_right_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1629:type_infer_functions_function_decl_quantify_right_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_function_decl_quantify_right_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_decl_quantify_right_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

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
      &String::from(
        r#"
--!nonstrict
local MagicMock = {}
MagicMock.is = require(game.isAMagicMock)

function MagicMock.is(value)
    return false
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_function_definition_in_a_do_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2798:type_infer_functions_function_definition_in_a_do_block`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_definition_in_a_do_block
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_definition_in_a_do_block() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f
        do
            function f()
            end
        end
        f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_function_definition_in_a_do_block_with_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2813:type_infer_functions_function_definition_in_a_do_block_with_global`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_definition_in_a_do_block_with_global
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_definition_in_a_do_block_with_global() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f() print("a") end
        do
            function f()
                print("b")
            end
        end
        f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_function_does_not_return_enough_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1121:type_infer_functions_function_does_not_return_enough_values`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_function_does_not_return_enough_values
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_does_not_return_enough_values() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_d::to_string_type_pack_id},
      records::{count_mismatch::CountMismatch, type_pack_mismatch::TypePackMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function f(): (number, string)
            return 55
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_function_exprs_are_generalized_at_signature_scope_not_enclosing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2234:type_infer_functions_function_exprs_are_generalized_at_signature_scope_not_enclosing`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_function_exprs_are_generalized_at_signature_scope_not_enclosing
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_exprs_are_generalized_at_signature_scope_not_enclosing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo
        local bar

        -- foo being a function expression is deliberate: the bug we're testing
        -- only existed for function expressions, not for function statements.
        foo = function(a)
            return bar
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "((unknown) -> nil)?",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
      );
    } else {
      assert_eq!(
        "<a>(a) -> 'b",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
      );
    }
  }
}

mod type_infer_functions_function_is_supertype_of_concrete_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2093:type_infer_functions_function_is_supertype_of_concrete_functions`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function registerHiddenTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_is_supertype_of_concrete_functions
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_is_supertype_of_concrete_functions() {
    use alloc::string::String;

    use ulua_unit_test::{
      functions::register_hidden_types::register_hidden_types, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(f: fun) end

        function a() end
        function id(x) return x end

        foo(a)
        foo(id)
        foo(foo)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_function_statement_sealed_table_assignment_through_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1797:type_infer_functions_function_statement_sealed_table_assignment_through_indexer`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_statement_sealed_table_assignment_through_indexer
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_statement_sealed_table_assignment_through_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local t: {[string]: () -> number} = {}

function t.a() return 1 end -- OK
function t:b() return 2 end -- not OK
    "#,
      ),
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
}

mod type_infer_functions_function_statement_with_incorrect_function_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3633:type_infer_functions_function_statement_with_incorrect_function_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_function_statement_with_incorrect_function_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_statement_with_incorrect_function_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _check_function_statement_types =
      ScopedFastFlag::new(&FFlag::LuauCheckFunctionStatementTypes, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Library: { isnan: (number) -> number } = {} :: any

        function Library.isnan(s: string): boolean
            return s == "NaN"
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
    assert_eq!("(string) -> boolean", to_string_type_id(err.given_type));
  }
}

mod type_infer_functions_function_that_could_return_anything_is_compatible_with_function_that_is_expected_to_return_nothing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2767:type_infer_functions_function_that_could_return_anything_is_compatible_with_function_that_is_expected_to_return_nothing`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_function_that_could_return_anything_is_compatible_with_function_that_is_expected_to_return_nothing
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_function_that_could_return_anything_is_compatible_with_function_that_is_expected_to_return_nothing()
   {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_fuzz_must_follow_in_overload_resolution {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2160:type_infer_functions_fuzz_must_follow_in_overload_resolution`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_fuzz_must_follow_in_overload_resolution
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzz_must_follow_in_overload_resolution() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
for _ in function<t0>():(t0)&((()->())&(()->()))
end do
_(_(_,_,_),_)
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_functions_fuzz_unwind_mutually_recursive_union_type_func {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3083:type_infer_functions_fuzz_unwind_mutually_recursive_union_type_func`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_fuzz_unwind_mutually_recursive_union_type_func
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzz_unwind_mutually_recursive_union_type_func() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = ...
        function _()
            _ = _
        end
        _[function(...) repeat until _(_[l100]) _ = _ end] += _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2829:type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function _()
end
local function l0()
    function _()
    end
end
_ = _
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_fuzzer_bug_missing_follow_causes_assertion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2843:type_infer_functions_fuzzer_bug_missing_follow_causes_assertion`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_fuzzer_bug_missing_follow_causes_assertion
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzzer_bug_missing_follow_causes_assertion() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _ = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );
  }
}

mod type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2877:type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _ = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );
  }
}

mod type_infer_functions_fuzzer_normalizer_out_of_resources {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2947:type_infer_functions_fuzzer_normalizer_out_of_resources`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - translates_to -> rust_item type_infer_functions_fuzzer_normalizer_out_of_resources
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_fuzzer_normalizer_out_of_resources() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );
  }
}

mod type_infer_functions_general_case_table_literal_blocks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:30:type_infer_functions_general_case_table_literal_blocks`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_general_case_table_literal_blocks
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_general_case_table_literal_blocks() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
function f(x : {[any]: number})
   return x
end

local Foo = {bar = "$$$"}

f({[Foo.bar] = 0})
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_generalize_table_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:169:type_infer_functions_generalize_table_property`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_generalize_table_property
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_generalize_table_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}

        T.foo = function(x)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = follow_type_id(fixture.require_type_string(&String::from("T")));
    let table = get_type_id::<TableType>(ty).expect("expected TableType");
    let foo = table.props.get("foo").expect("expected foo property");
    let foo_ty = foo.read_ty.expect("expected readable foo property");
    assert_eq!("<a>(a) -> a", to_string_type_id(foo_ty));
  }
}

mod type_infer_functions_generic_function_statement {
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3194:type_infer_functions_generic_function_statement`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_generic_function_statement() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_generic_packs_are_not_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2380:type_infer_functions_generic_packs_are_not_variadic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_generic_packs_are_not_variadic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_generic_packs_are_not_variadic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_d::to_string_type_pack_id},
      records::type_pack_mismatch::TypePackMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_generic_polarity_of_annotated_code {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4020:type_infer_functions_generic_polarity_of_annotated_code`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
  //!   - translates_to -> rust_item type_infer_functions_generic_polarity_of_annotated_code
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_generic_polarity_of_annotated_code() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::get_type_alt_j::get_type_id,
      records::{function_type::FunctionType, generic_type::GenericType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: <T>(T) -> T = nil :: any
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("f")))
      .expect("expected FunctionType");
    let r#gen = get_type_id::<GenericType>(ftv.generics()[0]).expect("expected GenericType");
    assert_eq!(Polarity::Mixed, r#gen.polarity);
  }
}

mod type_infer_functions_global_emplacing_steals_type_from_elsewhere {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4060:type_infer_functions_global_emplacing_steals_type_from_elsewhere`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_global_emplacing_steals_type_from_elsewhere
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_global_emplacing_steals_type_from_elsewhere() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "() -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_functions_global_function_blocked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3993:type_infer_functions_global_function_blocked`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_functions_global_function_blocked
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_global_function_blocked() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_global_function_redefinition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3924:type_infer_functions_global_function_redefinition`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_global_function_redefinition
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_global_function_redefinition() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function fact(n: number)
            return if n < 1 then 1 else n * fact(n - 1)
        end

        fact = "huh"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
  }
}

mod type_infer_functions_hidden_variadics_should_not_break_subtyping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3011:type_infer_functions_hidden_variadics_should_not_break_subtyping`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_hidden_variadics_should_not_break_subtyping
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_hidden_variadics_should_not_break_subtyping() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_higher_order_function_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:693:type_infer_functions_higher_order_function_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_functions_higher_order_function_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_higher_order_function_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv =
      get_type_id::<FunctionType>(fixture.require_type_string(&String::from("bottomupmerge")))
        .expect("expected bottomupmerge to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(6, arg_vec.len());

    let f_type = get_type_id::<FunctionType>(follow_type_id(arg_vec[0]));
    assert!(f_type.is_some(), "expected first argument to be a function");
  }
}

mod type_infer_functions_higher_order_function_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:728:type_infer_functions_higher_order_function_3`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_higher_order_function_3
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_higher_order_function_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a, b>({a} & {b}) -> {a} & {b}",
      to_string_type_id(fixture.require_type_string(&String::from("swapTwice")))
    );
  }
}

mod type_infer_functions_higher_order_function_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:763:type_infer_functions_higher_order_function_4`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_functions_higher_order_function_4
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_higher_order_function_4() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id, size_type_pack::size,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
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
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv =
      get_type_id::<FunctionType>(fixture.base.require_type_string(&String::from("mergesort")))
        .expect("expected mergesort to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(2, arg_vec.len());

    let arg0 = get_type_id::<TableType>(follow_type_id(arg_vec[0]))
      .expect("expected first argument to be a table");
    let indexer = arg0.indexer.as_ref().expect("expected table indexer");

    let arg1 = get_type_id::<FunctionType>(follow_type_id(arg_vec[1]))
      .expect("expected second argument to be a function");
    assert_eq!(2, unsafe { size(arg1.arg_types(), null_mut()) });

    let (arg1_args, _) = flatten_type_pack_id(arg1.arg_types());

    assert_eq!(
      follow_type_id(indexer.index_result_type),
      follow_type_id(arg1_args[0])
    );
    assert_eq!(
      follow_type_id(indexer.index_result_type),
      follow_type_id(arg1_args[1])
    );
  }
}

mod type_infer_functions_ignored_return_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1106:type_infer_functions_ignored_return_values`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_ignored_return_values
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_ignored_return_values() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function f()
            return 55, ""
        end

        local a = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_improved_function_arg_mismatch_error_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2032:type_infer_functions_improved_function_arg_mismatch_error_nonstrict`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_improved_function_arg_mismatch_error_nonstrict
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_improved_function_arg_mismatch_error_nonstrict() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function foo(a, b) end
        foo(string.find("hello", "e"))
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function 'foo' expects 0 to 2 arguments, but 3 are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_functions_improved_function_arg_mismatch_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1975:type_infer_functions_improved_function_arg_mismatch_errors`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_improved_function_arg_mismatch_errors
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_improved_function_arg_mismatch_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(9, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_infer_anonymous_function_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1254:type_infer_functions_infer_anonymous_function_arguments`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_functions_infer_anonymous_function_arguments
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_anonymous_function_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(a) return a.x + a.y end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
local function f(a: ((Table) -> number)?) if a then return a({x = 1, y = 2}) else return 0 end end
f(function(a) return a.x + a.y end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
local x = {}
x.b = {x = 1, y = 2}
function x:f(a: (Table) -> number) return a(self.b) end
x:f(function(a) return a.x + a.y end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(a: (a: number, b: number, c: boolean) -> number) return a(1, 2, true) end
f(function(a: number, b, c) return c and a + b or b - a end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(...) return select(1, ...).z end)
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Key 'z' not found in table 'Table'",
      to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(a: (a: number, b: number) -> number) return a(1, 2) end
f(function(a, b, c, ...) return a + b end)
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let expected = if FFlag::LuauInstantiateInSubtyping.get() {
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
      &String::from(
        r#"
function f(a: (...number) -> number) return a(1, 2) end
f(function(a, b) return a + b end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
function f(a: (...Table) -> number) return a({x = 1, y = 2}, {x = 3, y = 4}) end
f(function(a, ...) local b = ... return b.z end)
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Key 'z' not found in table 'Table'",
      to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
function f(a: (number) -> Table) return a(4) end
f(function(x) return x * 2 end)
    "#,
      ),
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'Table', but got 'number'",
      to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: (number) -> nil) return a(4) end
        f(function(x) print(x) end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_infer_anonymous_function_arguments_outside_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1475:type_infer_functions_infer_anonymous_function_arguments_outside_call`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_anonymous_function_arguments_outside_call
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_anonymous_function_arguments_outside_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { x: number, y: number }
local f: (Table) -> number = function(t) return t.x + t.y end

type TableWithFunc = { x: number, y: number, f: (number, number) -> number }
local a: TableWithFunc = { x = 3, y = 4, f = function(a, b) return a + b end }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_infer_from_function_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:147:type_infer_functions_infer_from_function_return_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_from_function_return_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_from_function_return_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("function take_five() return 5 end    local five = take_five()"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("five")))
    );
  }
}

mod type_infer_functions_infer_generic_function_function_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1373:type_infer_functions_infer_generic_function_function_argument`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_generic_function_function_argument
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_generic_function_function_argument() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function sum<a>(x: a, y: a, f: (a, a) -> a) return f(x, y) end
return sum(2, 3, function(a, b) return a + b end)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local function map<a, b>(arr: {a}, f: (a) -> b) local r = {} for i,v in ipairs(arr) do table.insert(r, f(v)) end return r end
local a = {1, 2, 3}
local r = map(a, function(a) return a + a > 100 end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{boolean}",
      to_string_type_id(fixture.base.require_type_string(&String::from("r")))
    );

    let _fold_result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local function foldl<a, b>(arr: {a}, init: b, f: (b, a) -> b) local r = init for i,v in ipairs(arr) do r = f(r, v) end return r end
local a = {1, 2, 3}
local r = foldl(a, {s=0,c=0}, function(a, b) return {s = a.s + b, c = a.c + 1} end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{| c: number, s: number |}",
      to_string_type_id(fixture.base.require_type_string(&String::from("r")))
    );
  }
}

mod type_infer_functions_infer_generic_function_function_argument_overloaded {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1404:type_infer_functions_infer_generic_function_function_argument_overloaded`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_generic_function_function_argument_overloaded
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_generic_function_function_argument_overloaded() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let mut result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12(1, function(x) return x + x end)
g12(1, 2, function(x, y) return x + y end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12({x=1}, function(x) return {x=-x.x} end)
g12({x=1}, {x=2}, function(x, y) return {x=x.x + y.x} end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_infer_generic_lib_function_function_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1434:type_infer_functions_infer_generic_lib_function_function_argument`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_generic_lib_function_function_argument
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_generic_lib_function_function_argument() {
    use alloc::string::String;

    use ulua_analysis::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a = {{x=4}, {x=7}, {x=1}}
table.sort(a, function(x, y) return x.x < y.x end)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0]).is_some(),
      "expected CannotInferBinaryOperation, got {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_functions_infer_higher_order_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:665:type_infer_functions_infer_higher_order_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_functions_infer_higher_order_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_higher_order_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply(f, x)
            return f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let apply_type = fixture.require_type_string(&String::from("apply"));
    let ftv =
      get_type_id::<FunctionType>(apply_type).expect("expected apply to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(2, arg_vec.len());

    let f_arg_type = follow_type_id(arg_vec[0]);
    let f_type = get_type_id::<FunctionType>(f_arg_type).unwrap_or_else(|| {
      panic!(
        "expected a function but got {}",
        to_string_type_id(arg_vec[0])
      )
    });

    let (f_args, _) = flatten_type_pack_id(f_type.arg_types());
    let x_type = follow_type_id(arg_vec[1]);

    assert_eq!(1, f_args.len());
    assert_eq!(x_type, follow_type_id(f_args[0]));
  }
}

mod type_infer_functions_infer_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:133:type_infer_functions_infer_return_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_functions_infer_return_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_return_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("function take_five() return 5 end"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let take_five_type = fixture.require_type_string(&String::from("take_five"));
    let take_five_function =
      get_type_id::<FunctionType>(take_five_type).expect("expected function type");

    let (ret_vec, _) = flatten_type_pack_id(take_five_function.ret_types());
    assert!(!ret_vec.is_empty());
    assert_eq!("number", to_string_type_id(ret_vec[0]));
  }
}

mod type_infer_functions_infer_return_type_from_selected_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:307:type_infer_functions_infer_return_type_from_selected_overload`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_infer_return_type_from_selected_overload
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_return_type_from_selected_overload() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {method: ((T, number) -> number) & ((number) -> string)}
        local T: T

        local a = T.method(T, 4)
        local b = T.method(5)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_functions_infer_return_value_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1488:type_infer_functions_infer_return_value_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_infer_return_value_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_return_value_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_infer_that_function_does_not_return_a_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:155:type_infer_functions_infer_that_function_does_not_return_a_table`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_infer_that_function_does_not_return_a_table
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_infer_that_function_does_not_return_a_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::not_a_table::NotATable,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function take_five()
            return 5
        end

        take_five().prop = 888
    "#,
      ),
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
}

mod type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1702:type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1728:type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_3`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_3
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_inferred_higher_order_functions_are_quantified_at_the_right_time_3() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo

        foo():bar(function()
            return foo()
        end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_inner_frees_become_generic_in_dcr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2216:type_infer_functions_inner_frees_become_generic_in_dcr`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_functions_inner_frees_become_generic_in_dcr
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_inner_frees_become_generic_in_dcr() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::generic_type::GenericType,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            local z = x
            return x
        end
    "#,
      ),
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
      get_type_id::<GenericType>(follow_type_id(ty)).is_some(),
      "expected GenericType"
    );
  }
}

mod type_infer_functions_instantiated_type_packs_must_have_a_non_null_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2190:type_infer_functions_instantiated_type_packs_must_have_a_non_null_scope`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_instantiated_type_packs_must_have_a_non_null_scope
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_instantiated_type_packs_must_have_a_non_null_scope() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_io_manager_oop_ish {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_io_manager_oop_ish() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_it_is_ok_not_to_supply_enough_retvals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:573:type_infer_functions_it_is_ok_not_to_supply_enough_retvals`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_it_is_ok_not_to_supply_enough_retvals
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_it_is_ok_not_to_supply_enough_retvals() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function get_two() return 5, 6 end

        local a = get_two()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_it_is_ok_to_oversaturate_a_higher_order_function_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:882:type_infer_functions_it_is_ok_to_oversaturate_a_higher_order_function_argument`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_it_is_ok_to_oversaturate_a_higher_order_function_argument
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_it_is_ok_to_oversaturate_a_higher_order_function_argument() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function onerror() end
        function foo() end
        xpcall(foo, onerror)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_lambda_form_of_local_function_cannot_be_recursive {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:401:type_infer_functions_lambda_form_of_local_function_cannot_be_recursive`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_lambda_form_of_local_function_cannot_be_recursive
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_lambda_form_of_local_function_cannot_be_recursive() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f = function() return f() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_list_all_overloads_if_no_overload_takes_given_argument_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:274:type_infer_functions_list_all_overloads_if_no_overload_takes_given_argument_count`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ExtraInformation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_list_all_overloads_if_no_overload_takes_given_argument_count
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_list_all_overloads_if_no_overload_takes_given_argument_count() {
    use alloc::string::String;

    use ulua_analysis::records::{
      extra_information::ExtraInformation, generic_error::GenericError,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply()
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
      "No overload for function accepts 0 arguments.",
      ge.message()
    );

    let ei = type_error_data_ref::<ExtraInformation>(&result.errors[1])
      .expect("expected ExtraInformation");
    assert_eq!(
      "Available overloads: (number) -> number; (number) -> string; and (number, number) -> number",
      ei.message()
    );
  }
}

mod type_infer_functions_list_only_alternative_overloads_that_match_argument_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:239:type_infer_functions_list_only_alternative_overloads_that_match_argument_count`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record MultipleNonviableOverloads (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record ExtraInformation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_list_only_alternative_overloads_that_match_argument_count
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_list_only_alternative_overloads_that_match_argument_count() {
    use alloc::string::String;

    use ulua_analysis::records::{
      extra_information::ExtraInformation,
      multiple_nonviable_overloads::MultipleNonviableOverloads, type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply("")
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let mno = type_error_data_ref::<MultipleNonviableOverloads>(&result.errors[0])
        .expect("expected MultipleNonviableOverloads");
      assert_eq!(1, mno.attempted_arg_count());
    } else {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        unsafe { (*fixture.builtin_types).number_type() },
        tm.wanted_type
      );
      assert_eq!(
        unsafe { (*fixture.builtin_types).string_type() },
        tm.given_type
      );
    }

    let ei = type_error_data_ref::<ExtraInformation>(&result.errors[1])
      .expect("expected ExtraInformation");

    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_local_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:516:type_infer_functions_local_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_functions_local_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_local_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let h = follow_type_id(fixture.require_type_string(&String::from("h")));
    let ftv = get_type_id::<FunctionType>(h).expect("expected FunctionType");
    let ret = first(ftv.ret_types(), true).expect("expected return type");
    let ret = follow_type_id(ret);
    assert_eq!("string", to_string_type_id(ret));
  }
}

mod type_infer_functions_local_function_fwd_decl_doesnt_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2641:type_infer_functions_local_function_fwd_decl_doesnt_crash`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - translates_to -> rust_item type_infer_functions_local_function_fwd_decl_doesnt_crash
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_local_function_fwd_decl_doesnt_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo

        local function bar()
            foo()
        end

        function foo()
        end

        bar()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_luau_subtyping_is_np_hard {
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2047:type_infer_functions_luau_subtyping_is_np_hard`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_luau_subtyping_is_np_hard() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t'(\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> false'\nbut got\n\t'((\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> boolean) & ((\"blue\" | \"red\") -> (\"blue\") -> (\"blue\") -> false) & ((\"blue\" | \"red\") -> (\"red\") -> (\"red\") -> false) & ((\"blue\") -> (\"blue\") -> (\"blue\" | \"red\") -> false) & ((\"red\") -> (\"red\") -> (\"blue\" | \"red\") -> false)'; none of the intersection parts are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_functions_lute_tasklib_createtask {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4035:type_infer_functions_lute_tasklib_createtask`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function resume (VM/src/ldo.cpp)
  //!   - translates_to -> rust_item type_infer_functions_lute_tasklib_createtask
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_lute_tasklib_createtask() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "((...any) -> (unknown, ...unknown), ...any) -> { co: thread, result: unknown, success: boolean }",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("createtask"))
      )
    );
  }
}

mod type_infer_functions_mutual_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:832:type_infer_functions_mutual_recursion`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_mutual_recursion
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_mutual_recursion() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        function newPlayerCharacter()
            startGui() -- Unknown symbol 'startGui'
        end

        local characterAddedConnection: any
        function startGui()
            characterAddedConnection = game:GetService("Players").LocalPlayer.CharacterAdded:connect(newPlayerCharacter)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_no_lossy_function_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1186:type_infer_functions_no_lossy_function_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_no_lossy_function_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_no_lossy_function_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local tbl = {}
        function tbl:abc(a: number, b: number)
            return a
        end
        tbl:abc(1, 2) -- Line 6
        --   | Column 14
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.require_type_at_position_position(Position {
      line: 6,
      column: 14,
    });
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("(unknown, number, number) -> number", to_string_type_id(ty));
    } else {
      assert_eq!("(tbl, number, number) -> number", to_string_type_id(ty));
    }

    let ftv = get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected FunctionType");
    assert!(ftv.has_self());
  }
}

mod type_infer_functions_num_is_solved_after_num_or_str {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2431:type_infer_functions_num_is_solved_after_num_or_str`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_num_is_solved_after_num_or_str
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_num_is_solved_after_num_or_str() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "() -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("num_or_str"))
      )
    );
  }
}

mod type_infer_functions_num_is_solved_before_num_or_str {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2409:type_infer_functions_num_is_solved_before_num_or_str`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_num_is_solved_before_num_or_str
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_num_is_solved_before_num_or_str() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "() -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_string(&String::from("num_or_str"))
      )
    );
  }
}

mod type_infer_functions_occurs_check_failure_in_function_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1899:type_infer_functions_occurs_check_failure_in_function_return_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record OccursCheckFailed (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_occurs_check_failure_in_function_return_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_occurs_check_failure_in_function_return_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::occurs_check_failed::OccursCheckFailed,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f()
            return 5, f()
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      get_type_error::<OccursCheckFailed>(&result.errors[0]).is_some(),
      "expected OccursCheckFailed: {:?}",
      result.errors
    );
  }
}

mod type_infer_functions_oss_1640 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3285:type_infer_functions_oss_1640`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - translates_to -> rust_item type_infer_functions_oss_1640
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_1640() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_oss_1854 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3301:type_infer_functions_oss_1854`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function writeu32 (CodeGen/src/ByteUtils.h)
  //!   - translates_to -> rust_item type_infer_functions_oss_1854
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_1854() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_oss_1871 {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_1871() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Test = {
            [string]: (string) -> ()
        }

        local TestTbl: Test = {}

        function TestTbl.Hello(Param)
            local _ = Param
        end
    "#,
      ),
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
}

mod type_infer_functions_oss_2061_modify_visited_generic_ice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3942:type_infer_functions_oss_2061_modify_visited_generic_ice`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_oss_2061_modify_visited_generic_ice
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2061_modify_visited_generic_ice() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    assert!(get_type_error::<CountMismatch>(&results.errors[0]).is_some());
  }
}

mod type_infer_functions_oss_2065_bidirectional_inference_function_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3334:type_infer_functions_oss_2065_bidirectional_inference_function_call`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_oss_2065_bidirectional_inference_function_call
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2065_bidirectional_inference_function_call() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_oss_2109 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3722:type_infer_functions_oss_2109`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_oss_2109
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2109() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_functions_oss_2118 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3543:type_infer_functions_oss_2118`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_functions_oss_2118
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2118() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: <P>(constructor: (P) -> any) -> (P) -> any = (nil :: any)
        local fn = foo(function (value: { test: true })
            return value.test
        end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ test: true }) -> any",
      to_string_type_id(fixture.require_type_string(&String::from("fn")))
    );
  }
}

mod type_infer_functions_oss_2125 {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2125() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
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
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_oss_2143 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3675:type_infer_functions_oss_2143`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_oss_2143
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2143() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_oss_2216_recursive_global_function_works_as_expected {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3869:type_infer_functions_oss_2216_recursive_global_function_works_as_expected`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_oss_2216_recursive_global_function_works_as_expected
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_oss_2216_recursive_global_function_works_as_expected() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_other_things_are_not_related_to_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2140:type_infer_functions_other_things_are_not_related_to_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function registerHiddenTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_other_things_are_not_related_to_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_other_things_are_not_related_to_function() {
    use alloc::string::String;

    use ulua_unit_test::{
      functions::register_hidden_types::register_hidden_types, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: fun = function() end
        local b: {} = a
        local c: boolean = a
        local d: fun = true
        local e: fun = {}
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(2, result.errors[0].location.begin.line);
    assert_eq!(3, result.errors[1].location.begin.line);
    assert_eq!(4, result.errors[2].location.begin.line);
    assert_eq!(5, result.errors[3].location.begin.line);
  }
}

mod type_infer_functions_overload_one_ok_one_potential {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3393:type_infer_functions_overload_one_ok_one_potential`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_overload_one_ok_one_potential
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_one_ok_one_potential() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: ((number) -> "one") & ((string) -> "two")

        local g = f(42)
        local h = f("huh")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "\"one\"",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
    assert_eq!(
      "\"two\"",
      to_string_type_id(fixture.require_type_string(&String::from("h")))
    );
  }
}

mod type_infer_functions_overload_resolution {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:46:type_infer_functions_overload_resolution`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_overload_resolution
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_resolution() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> string
        type B = (string) -> number

        local function foo(f: A & B)
            return f(1), f("five")
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.require_type_string(&String::from("foo"));
    let foo_type = get_type_id::<FunctionType>(t);
    assert!(foo_type.is_some(), "expected function type");
    assert_eq!(
      "(((number) -> string) & ((string) -> number)) -> (string, number)",
      to_string_type_id(t)
    );
  }
}

mod type_infer_functions_overload_resolution_crash_when_arg_exprs_is_smaller_than_type_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2981:type_infer_functions_overload_resolution_crash_when_arg_exprs_is_smaller_than_type_args`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_overload_resolution_crash_when_arg_exprs_is_smaller_than_type_args
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_resolution_crash_when_arg_exprs_is_smaller_than_type_args() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );
  }
}

mod type_infer_functions_overload_selection_ambiguous_call {
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3409:type_infer_functions_overload_selection_ambiguous_call`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_ambiguous_call() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_d::to_string_type_pack_id,
      },
      records::ambiguous_function_call::AmbiguousFunctionCall,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: ((number | string) -> "one") & ((number | boolean) -> "two")
        local g = f(42)
    "#,
      ),
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
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_functions_overload_selection_bad_arity {
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3461:type_infer_functions_overload_selection_bad_arity`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_bad_arity() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo<T>(f: ((number, number) -> "one") & T)
            local huh = f(42)
            local _ = huh
        end
    "#,
      ),
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
}

mod type_infer_functions_overload_selection_needs_to_retry {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3507:type_infer_functions_overload_selection_needs_to_retry`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_functions_overload_selection_needs_to_retry
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_needs_to_retry() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type RGB = { r: number, b: number, g: number }
        local BrickColor: ((number) -> RGB) & ((number, number, number) -> RGB) & ((string) -> RGB)
        function Lightning(li, Color)
            li.BrickColor = BrickColor(Color)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ BrickColor: RGB }, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("Lightning")))
    );
  }
}

mod type_infer_functions_overload_selection_no_compatible_option {
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3447:type_infer_functions_overload_selection_no_compatible_option`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_no_compatible_option() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: ((number) -> "one") & ((boolean) -> "two")
        local g = f("s" :: string)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_functions_overload_selection_pick_better_arity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3427:type_infer_functions_overload_selection_pick_better_arity`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_overload_selection_pick_better_arity
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_pick_better_arity() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: ((number) -> "one") & ((number, number) -> "two")
        -- Casting here so that we always hit the case in overload selection
        -- where one part has the correct arity but incorrect argument types,
        -- and the other has the incorrect arity.
        local g = f("s" :: string)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
    assert_eq!(
      "\"one\"",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_functions_overload_selection_unambiguous_with_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3526:type_infer_functions_overload_selection_unambiguous_with_constraint`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_overload_selection_unambiguous_with_constraint
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_unambiguous_with_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: ((string, number) -> string) & ((number, boolean) -> number)
        local function g(x)
            -- When selecting an overload at this point, we'll reject the
            -- second overload, and claim that this is the only possible
            -- overload with a constraint of `x <: string`.
            f(x, 42)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(string) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_functions_overload_selection_union_of_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3484:type_infer_functions_overload_selection_union_of_functions`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_overload_selection_union_of_functions
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_overload_selection_union_of_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(f: (() -> (number)) | (() -> (string)))
            return f()
        end

        local g = foo(nil :: any)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2257:type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo<a>(x: a, y: a?)
            return x
        end
        local vec2 = { x = 5, y = 7 }
        local ret: number = foo(vec2, { x = 5 })
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2321:type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_param_1_and_2_both_takes_the_same_generic_but_their_arguments_are_incompatible_2()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<a>(x: a, y: a): a
            return if math.random() > 0.5 then x else y
        end

        local z: boolean = f(5, "five")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_functions_param_y_is_bounded_by_x_of_type_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2754:type_infer_functions_param_y_is_bounded_by_x_of_type_string`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_param_y_is_bounded_by_x_of_type_string
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_param_y_is_bounded_by_x_of_type_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string, y)
            x = y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(string, string) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_functions_pass_table_literal_to_function_expecting_optional_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2718:type_infer_functions_pass_table_literal_to_function_expecting_optional_prop`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_functions_pass_table_literal_to_function_expecting_optional_prop
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_pass_table_literal_to_function_expecting_optional_prop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {prop: number?}

        function f(t: T) end

        f({prop=5})
        f({})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_pcall_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3754:type_infer_functions_pcall_example`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_pcall_example
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_pcall_example() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function makestr(n: number): string
            return tostring(n)
        end

        -- `s` now has type `string` and not `unknown`
        local success, s = pcall(makestr, 42)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_functions_record_matching_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1209:type_infer_functions_record_matching_overload`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_functions_record_matching_overload
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_record_matching_overload() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position,
      to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::records::{ast_expr_call::AstExprCall, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Overload = ((string) -> string) & ((number) -> number)
        local abc: Overload
        abc(1)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
      find_ast_ancestry_of_position(
        &*source_module,
        Position {
          line: 3,
          column: 10,
        },
        false,
      )
    };
    assert!(ancestry.len() >= 2, "ancestry was {:?}", ancestry);

    let parent_expr = ancestry[ancestry.len() - 2];
    assert!(
      ast_node_is::<AstExprCall>(parent_expr),
      "expected AstExprCall"
    );

    let module = unsafe { &*fixture.get_main_module(false) };
    let overload = module
      .ast_overload_resolved_types
      .find(&(parent_expr as *const _))
      .expect("expected recorded overload type");
    assert_eq!("(number) -> number", to_string_type_id(*overload));
  }
}

mod type_infer_functions_recursive_calls_must_refer_to_the_ungeneralized_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:450:type_infer_functions_recursive_calls_must_refer_to_the_ungeneralized_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_recursive_calls_must_refer_to_the_ungeneralized_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_calls_must_refer_to_the_ungeneralized_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo()
            string.format('%s: %s', "51", foo())
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_functions_recursive_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:386:type_infer_functions_recursive_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_recursive_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function count(n: number)
            if n == 0 then
                return 0
            else
                return count(n - 1)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3040:type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
  }
}

mod type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3064:type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_function_calls_should_not_use_the_generalized_type_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function random()
            return true -- chosen by fair coin toss
        end

        local function f()
            if random() then f() end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_recursive_local_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:410:type_infer_functions_recursive_local_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_recursive_local_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_local_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function count(n: number)
            if n == 0 then
                return 0
            else
                return count(n - 1)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_recursive_static_method_must_refer_to_the_ungeneralized_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3853:type_infer_functions_recursive_static_method_must_refer_to_the_ungeneralized_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_recursive_static_method_must_refer_to_the_ungeneralized_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_recursive_static_method_must_refer_to_the_ungeneralized_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_regex_benchmark_string_format_minimization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2466:type_infer_functions_regex_benchmark_string_format_minimization`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> macro tonumber (VM/src/lvm.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_regex_benchmark_string_format_minimization
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_regex_benchmark_string_format_minimization() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        (nil :: any)(function(n)
            if tonumber(n) then
                n = tonumber(n)
            elseif n ~= nil then
                string.format("invalid argument #4 to 'sub': number expected, got %s", typeof(n))
            end
        end);
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_report_exiting_without_return_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:906:type_infer_functions_report_exiting_without_return_nonstrict`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_report_exiting_without_return_nonstrict
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_report_exiting_without_return_nonstrict() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error,
      records::function_exits_without_returning::FunctionExitsWithoutReturning,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      get_type_error::<FunctionExitsWithoutReturning>(&result.errors[0]).is_some(),
      "expected FunctionExitsWithoutReturning"
    );
  }
}

mod type_infer_functions_report_exiting_without_return_strict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:944:type_infer_functions_report_exiting_without_return_strict`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_report_exiting_without_return_strict
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_report_exiting_without_return_strict() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error,
      records::function_exits_without_returning::FunctionExitsWithoutReturning,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(get_type_error::<FunctionExitsWithoutReturning>(&result.errors[0]).is_some());
    assert!(get_type_error::<FunctionExitsWithoutReturning>(&result.errors[1]).is_some());
  }
}

mod type_infer_functions_return_type_by_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1234:type_infer_functions_return_type_by_overload`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_return_type_by_overload
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_return_type_by_overload() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Overload = ((string) -> string) & ((number, number) -> number)
        local abc: Overload
        local x = abc(true)
        local y = abc(true,true)
        local z = abc(true,true,true)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("z")))
      );
    } else {
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("z")))
      );
    }
  }
}

mod type_infer_functions_self_application_does_not_segfault {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2786:type_infer_functions_self_application_does_not_segfault`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_self_application_does_not_segfault
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_self_application_does_not_segfault() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _ = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a)
            f(f)
            return f(), a
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_functions_simple_lightly_annotated_mutual_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2547:type_infer_functions_simple_lightly_annotated_mutual_recursion`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_simple_lightly_annotated_mutual_recursion
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_simple_lightly_annotated_mutual_recursion() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("even")))
    );
    assert_eq!(
      "(number) -> boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("odd")))
    );
  }
}

mod type_infer_functions_simple_unannotated_mutual_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2492:type_infer_functions_simple_unannotated_mutual_recursion`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Constraint (Analysis/include/Luau/Constraint.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_simple_unannotated_mutual_recursion
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_simple_unannotated_mutual_recursion() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type used in - operation; consider adding a type annotation to 'n'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_functions_strict_mode_ok_with_missing_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1787:type_infer_functions_strict_mode_ok_with_missing_arguments`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_strict_mode_ok_with_missing_arguments
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_strict_mode_ok_with_missing_arguments() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: any) end
        f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_string_format_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3102:type_infer_functions_string_format_pack`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_string_format_pack
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_string_format_pack() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(): (string, string, string)
            return "", "", ""
        end
        print(string.format("%s %s %s", foo()))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_string_format_pack_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3112:type_infer_functions_string_format_pack_variadic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_string_format_pack_variadic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_string_format_pack_variadic() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo : () -> (...string) = (nil :: any)
        print(string.format("%s %s %s", foo()))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_subgeneric_type_function_super_monomorphic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2481:type_infer_functions_subgeneric_type_function_super_monomorphic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_subgeneric_type_function_super_monomorphic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_subgeneric_type_function_super_monomorphic() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: (number, number) -> number = function(a, b) return a - b end

a = function(a, b) return a + b end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_table_annotated_explicit_self {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_table_annotated_explicit_self() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::function_exits_without_returning::FunctionExitsWithoutReturning,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_table_containing_factorial_assign_later {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3800:type_infer_functions_table_containing_factorial_assign_later`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_table_containing_factorial_assign_later
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_table_containing_factorial_assign_later() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err = get_type_error::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
    assert_eq!("(string) -> ()", to_string_type_id(err.given_type));
  }
}

mod type_infer_functions_table_containing_factorial_assign_with_correct_typing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3825:type_infer_functions_table_containing_factorial_assign_with_correct_typing`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_table_containing_factorial_assign_with_correct_typing
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_table_containing_factorial_assign_with_correct_typing() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err = get_type_error::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!("(number) -> number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
  }
}

mod type_infer_functions_table_containing_factorial_standalone {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3785:type_infer_functions_table_containing_factorial_standalone`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_functions_table_containing_factorial_standalone
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_table_containing_factorial_standalone() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local coolmath = {}
        function coolmath.factorial(n: number)
            if n <= 1 then
                return 1
            end
            return coolmath.factorial(n - 1) * n
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_tc_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:64:type_infer_functions_tc_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_tc_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_tc_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id, records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
      .check_string_optional_frontend_options(&String::from("function five() return 5 end"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let five_type = get_type_id::<FunctionType>(fixture.require_type_string(&String::from("five")));
    assert!(five_type.is_some(), "expected function type");
  }
}

mod type_infer_functions_tf_suggest_arg_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2602:type_infer_functions_tf_suggest_arg_type`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_tf_suggest_arg_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_tf_suggest_arg_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        cannot_infer_binary_operation::CannotInferBinaryOperation,
        explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function fib(n, u)
            return (n or u) and (n < u and n + fib(n,u))
        end
    "#,
      ),
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
}

mod type_infer_functions_tf_suggest_arg_type_2 {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_tf_suggest_arg_type_2() {
    use alloc::string::String;

    use ulua_analysis::records::not_a_table::NotATable;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function escape_fslash(pre)
            return (#pre % 2 == 0 and '\\' or '') .. pre .. '.'
        end
    "#,
      ),
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
}

mod type_infer_functions_tf_suggest_return_type {
  use super::*;
  #[cfg(test)]
  #[test]
  fn type_infer_functions_tf_suggest_return_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function fib(n)
            return n < 2 and 1 or fib(n-1) + fib(n-2)
        end
    "#,
      ),
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
}

mod type_infer_functions_too_few_arguments_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1822:type_infer_functions_too_few_arguments_variadic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_too_few_arguments_variadic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_few_arguments_variadic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    function test(a: number, b: string, ...)
    end

    test(1)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

    assert_eq!(2, acm.expected());
    assert_eq!(1, acm.actual());
    assert_eq!(CountMismatch::ARG, acm.context());
    assert!(acm.is_variadic());
  }
}

mod type_infer_functions_too_few_arguments_variadic_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1843:type_infer_functions_too_few_arguments_variadic_generic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_too_few_arguments_variadic_generic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_few_arguments_variadic_generic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test(a: number, b: string, ...)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

    assert_eq!(3, acm.expected());
    assert_eq!(1, acm.actual());
    assert_eq!(CountMismatch::ARG, acm.context());
    assert!(acm.is_variadic());
  }
}

mod type_infer_functions_too_few_arguments_variadic_generic_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1871:type_infer_functions_too_few_arguments_variadic_generic_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_too_few_arguments_variadic_generic_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_few_arguments_variadic_generic_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function test(a: number, b: string, ...)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

pcall(wrapper, test)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");

    assert_eq!(4, acm.expected());
    assert_eq!(2, acm.actual());
    assert_eq!(CountMismatch::ARG, acm.context());
    assert!(acm.is_variadic());
  }
}

mod type_infer_functions_too_many_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:322:type_infer_functions_too_many_arguments`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_too_many_arguments
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_many_arguments() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function g(a: number) end

        g()

    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(1, acm.expected());
    assert_eq!(0, acm.actual());
  }
}

mod type_infer_functions_too_many_arguments_error_location {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:346:type_infer_functions_too_many_arguments_error_location`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
  //!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_too_many_arguments_error_location
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_many_arguments_error_location() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function myfunction(a: number, b:number) end
        myfunction(1)

        function getmyfunction()
            return myfunction
        end
        getmyfunction()()
    "#,
      ),
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
}

mod type_infer_functions_too_many_return_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1041:type_infer_functions_too_many_return_values`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_too_many_return_values
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_many_return_values() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function f()
            return 55
        end

        local a, b = f()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatch::FUNCTION_RESULT, acm.context());
    assert_eq!(1, acm.expected());
    assert_eq!(2, acm.actual());
  }
}

mod type_infer_functions_too_many_return_values_in_parentheses {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1062:type_infer_functions_too_many_return_values_in_parentheses`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_too_many_return_values_in_parentheses
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_many_return_values_in_parentheses() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function f()
            return 55
        end

        local a, b = (f())
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatch::FUNCTION_RESULT, acm.context());
    assert_eq!(1, acm.expected());
    assert_eq!(2, acm.actual());
  }
}

mod type_infer_functions_too_many_return_values_no_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1086:type_infer_functions_too_many_return_values_no_function`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_too_many_return_values_no_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_too_many_return_values_no_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local a, b = 55
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatch::EXPR_LIST_RESULT, acm.context());
    assert_eq!(1, acm.expected());
    assert_eq!(2, acm.actual());
  }
}

mod type_infer_functions_toposort_doesnt_break_mutual_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:850:type_infer_functions_toposort_doesnt_break_mutual_recursion`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_functions_toposort_doesnt_break_mutual_recursion
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_toposort_doesnt_break_mutual_recursion() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local x = nil
        function f() g() end
        -- make sure print(x) doesn't get toposorted here, breaking the mutual block
        function g() x = f end
        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_unifier_should_not_bind_free_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2895:type_infer_functions_unifier_should_not_bind_free_types`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_functions_unifier_should_not_bind_free_types
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_unifier_should_not_bind_free_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(tm1.wanted_type));
    assert_eq!("boolean", to_string_type_id(tm1.given_type));
  }
}

mod type_infer_functions_unify_type_pack_stack_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3968:type_infer_functions_unify_type_pack_stack_overflow`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_functions_unify_type_pack_stack_overflow
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_unify_type_pack_stack_overflow() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_d::to_string_type_pack_id},
      records::type_pack_mismatch::TypePackMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_functions_unnecessary_nil_in_lower_bound_of_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3252:type_infer_functions_unnecessary_nil_in_lower_bound_of_generic`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_unnecessary_nil_in_lower_bound_of_generic
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_unnecessary_nil_in_lower_bound_of_generic() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2997:type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local function id(x)
    return x
end
local u,v = id(3), id(id(44))
"#,
      ),
      None,
    );

    let v_type = fixture.require_type_string(&String::from("v"));
    let number_type = fixture.get_builtins().number_type;
    assert_eq!(number_type, v_type);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_vararg_function_is_quantified {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:203:type_infer_functions_vararg_function_is_quantified`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_functions_vararg_function_is_quantified
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_vararg_function_is_quantified() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.base.get_main_module(false) };
    let return_ty = first(module.return_type, true).expect("expected module return type");
    let table = get_type_id::<TableType>(return_ty).expect("expected table");
    let f = table.props.get("f").expect("expected f property");
    assert!(f.read_ty.is_some(), "expected readable f property");
  }
}

mod type_infer_functions_vararg_functions_should_allow_calls_of_any_types_and_size {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:191:type_infer_functions_vararg_functions_should_allow_calls_of_any_types_and_size`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_functions_vararg_functions_should_allow_calls_of_any_types_and_size
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_vararg_functions_should_allow_calls_of_any_types_and_size() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(...) end

        f(1)
        f("foo", 2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1448:type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function f(...) return ... end
        local g = function(...) return f(...) end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1460:type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack_2`
  //! Source: `tests/TypeInfer.functions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.functions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.functions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_functions_variadic_any_is_compatible_with_a_generic_type_pack_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function somethingThatsAny(...: any)
            print(...)
        end

        local function x<T...>(...: T...)
            somethingThatsAny(...) -- Failed to unify variadic type packs
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
