extern crate alloc;

mod type_infer_any_type_in_function_argument_should_not_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2714:type_infer_any_type_in_function_argument_should_not_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_any_type_in_function_argument_should_not_error

  #[cfg(test)]
  #[test]
  fn type_infer_any_type_in_function_argument_should_not_error() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function f(u: string) end

        local t: {[any]: any} = {}

        for k in t do
            f(k)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_assert_allows_singleton_union_or_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1992:type_infer_assert_allows_singleton_union_or_intersection`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_assert_allows_singleton_union_or_intersection

  #[cfg(test)]
  #[test]
  fn type_infer_assert_allows_singleton_union_or_intersection() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = 42 :: | number
        local y = 42 :: & number
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_assert_table_freeze_constraint_solving {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2000:type_infer_assert_table_freeze_constraint_solving`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_assert_table_freeze_constraint_solving

  #[cfg(test)]
  #[test]
  fn type_infer_assert_table_freeze_constraint_solving() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f = table.freeze
        f(table)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_avoid_blocking_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1781:type_infer_avoid_blocking_type_function`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_avoid_blocking_type_function

  #[cfg(test)]
  #[test]
  fn type_infer_avoid_blocking_type_function() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function foo(a : string?)
            local b = a or ""
            return b:upper()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_avoid_double_reference_to_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1794:type_infer_avoid_double_reference_to_free_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_avoid_double_reference_to_free_type

  #[cfg(test)]
  #[test]
  fn type_infer_avoid_double_reference_to_free_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function wtf(name: string?)
            local message
            message = "invalid alternate fiber: " .. (name or "UNNAMED alternate")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_bad_iter_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1654:type_infer_bad_iter_metamethod`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_bad_iter_metamethod

  #[cfg(test)]
  #[test]
  fn type_infer_bad_iter_metamethod() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::cannot_call_non_function::CannotCallNonFunction,
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
        function iter(): unknown
            return nil
        end

        local a = {__iter = iter}
        setmetatable(a, a)

        for i in a do
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let ccnf = type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
        .expect("expected CannotCallNonFunction");
      assert_eq!("unknown", to_string_type_id(ccnf.ty()));
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1614:type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - translates_to -> rust_item type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload

  #[cfg(test)]
  #[test]
  fn type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function concat<T>(target: {T}, ...: {T} | T): {T}
            return (nil :: any) :: {T}
        end

        local res = concat({"alic"}, 1, 2)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    for error in &result.errors {
      assert_eq!(5, error.location.begin.line, "{:?}", result.errors);
    }
  }
}

mod type_infer_bidirectional_checking_of_higher_order_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1321:type_infer_bidirectional_checking_of_higher_order_function`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_bidirectional_checking_of_higher_order_function

  #[cfg(test)]
  #[test]
  fn type_infer_bidirectional_checking_of_higher_order_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function higher(cb: (number) -> ()) end

        higher(function(n)      -- no error here.  n : number
            local e: string = n -- error here.  n /: string
        end)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(4, result.errors[0].location.end.line);
  }
}

mod type_infer_bound_typepack_promote {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1115:type_infer_bound_typepack_promote`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_bound_typepack_promote

  #[cfg(test)]
  #[test]
  fn type_infer_bound_typepack_promote() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function p()
    local this = {}
    this.pf = foo()
    function this:IsActive() end
    function this:Start(o) end
    return this
end

local function h(tp, o)
    ep = tp
    tp:Start(o)
    tp.pf.Connect(function()
        ep:IsActive()
    end)
end

function on()
    local t = p()
    h(t)
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_captured_globals_are_not_blocked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2739:type_infer_captured_globals_are_not_blocked`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_captured_globals_are_not_blocked

  #[cfg(test)]
  #[test]
  fn type_infer_captured_globals_are_not_blocked() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _forbid_internal_types = ScopedFastFlag::new(&FFlag::DebugLuauForbidInternalTypes, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local Cancelled: boolean = false

        function Start()
            if Cancelled then
                return
            end
            Selection = 42
            local _ = function ()
                if Selection then
                end
            end
        end

        function Cancel()
            Selection = nil
        end

        return {}
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_check_block_recursion_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:409:type_infer_check_block_recursion_limit`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_check_block_recursion_limit

  #[cfg(test)]
  #[test]
  fn type_infer_check_block_recursion_limit() {
    use ulua_analysis::records::code_too_complex::CodeTooComplex;
    use ulua_common::{DFInt, FInt};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let limit: usize = if cfg!(debug_assertions) { 350 } else { 595 };

    let _luau_recursion_limit = ScopedFastInt::new(&FInt::LuauRecursionLimit, limit as i32 * 2);
    let _luau_check_recursion_limit =
      ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 - 100);
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
      &DFInt::LuauConstraintGeneratorRecursionLimit,
      limit as i32 - 100,
    );
    let _luau_subtyping_recursion_limit =
      ScopedFastInt::new(&DFInt::LuauSubtypingRecursionLimit, limit as i32 - 100);

    let mut fixture = Fixture::fixture_bool(false);
    let code = "do ".repeat(limit) + "local a = 1" + &" end".repeat(limit);
    let result = fixture.check_string_optional_frontend_options(&code, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CodeTooComplex>(&result.errors[0]).expect("expected CodeTooComplex");
  }
}

mod type_infer_check_expr_recursion_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:442:type_infer_check_expr_recursion_limit`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_check_expr_recursion_limit

  #[cfg(test)]
  #[test]
  fn type_infer_check_expr_recursion_limit() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::code_too_complex::CodeTooComplex,
    };
    use ulua_common::{DFInt, FInt};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let limit: usize = if cfg!(debug_assertions) { 250 } else { 500 };

    let _luau_recursion_limit = ScopedFastInt::new(&FInt::LuauRecursionLimit, limit as i32 * 2);
    let _luau_check_recursion_limit =
      ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 - 100);
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
      &DFInt::LuauConstraintGeneratorRecursionLimit,
      limit as i32 - 100,
    );
    let _luau_subtyping_recursion_limit =
      ScopedFastInt::new(&DFInt::LuauSubtypingRecursionLimit, limit as i32 - 100);

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
    let result = fixture.check_string_optional_frontend_options(&code, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<CodeTooComplex>(&result.errors[0]).is_some(),
      "Expected CodeTooComplex but got {}",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_check_type_infer_recursion_count {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:389:type_infer_check_type_infer_recursion_count`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_check_type_infer_recursion_count

  #[cfg(test)]
  #[test]
  fn type_infer_check_type_infer_recursion_count() {
    use alloc::string::String;

    use ulua_analysis::records::code_too_complex::CodeTooComplex;
    use ulua_common::FInt;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let limit: usize = if cfg!(debug_assertions) { 350 } else { 600 };
    let _sfi = ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32);

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from("function f() return ")
      + &"{a=".repeat(limit)
      + "'a'"
      + &"}".repeat(limit)
      + " end";
    let result = fixture.check_string_optional_frontend_options(&code, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CodeTooComplex>(&result.errors[0]).expect("expected CodeTooComplex");
  }
}

mod type_infer_checking_should_not_ice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:537:type_infer_checking_should_not_ice`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_checking_should_not_ice

  #[cfg(test)]
  #[test]
  fn type_infer_checking_should_not_ice() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        f,g = ...
        f(g(...))[...] = nil
        f,xpcall = ...
        local value = g(...)(g(...))
    "#,
      ),
      None,
    );

    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("value")))
    );
  }
}

mod type_infer_cli_39932_use_unifier_in_ensure_methods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:732:type_infer_cli_39932_use_unifier_in_ensure_methods`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_cli_39932_use_unifier_in_ensure_methods

  #[cfg(test)]
  #[test]
  fn type_infer_cli_39932_use_unifier_in_ensure_methods() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: {number|number} = {1, 2, 3}
        local y = x[1] - x[2]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_cli_50041_committing_txnlog_in_apollo_client_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1142:type_infer_cli_50041_committing_txnlog_in_apollo_client_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_cli_50041_committing_txnlog_in_apollo_client_error

  #[cfg(test)]
  #[test]
  fn type_infer_cli_50041_committing_txnlog_in_apollo_client_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        --!nolint

        type FieldSpecifier = {
            fieldName: string,
        }

        type ReadFieldOptions = FieldSpecifier & { from: number? }

        type Policies = {
            getStoreFieldName: (self: Policies, fieldSpec: FieldSpecifier) -> string,
        }

        local Policies = {}

        local function foo(p: Policies)
        end

        function Policies:getStoreFieldName(specifier: FieldSpecifier): string
            return ""
        end

        function Policies:readField(options: ReadFieldOptions)
            local _ = self:getStoreFieldName(options)
            foo(self)
        end
    "#,
      ),
      None,
    );

    if FFlag::LuauInstantiateInSubtyping.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = concat!(
        "Expected this to be exactly 'Policies' from 'MainModule', but got 'Policies' from 'MainModule'",
        "\ncaused by:\n",
        "  Property 'getStoreFieldName' is not compatible.\n",
        "Expected this to be exactly\n\t",
        "'(Policies, FieldSpecifier) -> string'",
        "\nbut got\n\t",
        "'(Policies, FieldSpecifier & { from: number? }) -> ('a, b...)'",
        "\ncaused by:\n",
        "  Argument #2 type is not compatible.\n",
        "Expected this to be exactly\n\t",
        "'FieldSpecifier & { from: number? }'",
        "\nbut got\n\t",
        "'FieldSpecifier'",
        "\ncaused by:\n",
        "  Not all intersection parts are compatible.\n",
        "Table type 'FieldSpecifier' not compatible with type '{ from: number? }' because the former has extra field 'fieldName'"
      );
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_concat_string_with_string_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1942:type_infer_concat_string_with_string_union`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_concat_string_with_string_union

  #[cfg(test)]
  #[test]
  fn type_infer_concat_string_with_string_union() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function concat_stuff(x: string, y : string | number)
            return x .. y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_config_reader_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2301:type_infer_config_reader_example`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record Config (Config/include/Luau/Config.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_config_reader_example

  #[cfg(test)]
  #[test]
  fn type_infer_config_reader_example() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/ConfigReader"),
      String::from(
        r#"
        --!strict
        local ConfigReader = {}
        ConfigReader.Defaults = {}

        local Defaults = ConfigReader.Defaults
        local Config = ConfigReader.Defaults

        function ConfigReader:read(config_name: string)
            if Config[config_name] ~= nil then
                return Config[config_name]
            elseif Defaults[config_name] ~= nil then
                return Defaults[config_name]
            else
                error(config_name .. " must be defined in Config")
            end
        end


        function ConfigReader:getFullConfigWithDefaults()
            local config = {}
            for key, val in pairs(ConfigReader.Defaults) do
                config[key] = val
            end
            for key, val in pairs(Config) do
                config[key] = val
            end
            return config
        end

        return ConfigReader
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/Util"),
      String::from(
        r#"
        --!strict
        local ConfigReader = require(script.Parent.ConfigReader)
        local _ = ConfigReader:read("foobar")()
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Util"), None);
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_constraint_generation_recursion_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2631:type_infer_constraint_generation_recursion_limit`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_constraint_generation_recursion_limit

  #[cfg(test)]
  #[test]
  fn type_infer_constraint_generation_recursion_limit() {
    use alloc::string::String;

    use ulua_common::{DFInt, FFlag, FInt};
    use ulua_unit_test::{
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _check_recursion_limit = ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, 5);
    let _constraint_generator_recursion_limit =
      ScopedFastInt::new(&DFInt::LuauConstraintGeneratorRecursionLimit, 5);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        if true then
        elseif true then
        elseif true then
        elseif true then
        else
        local x = 1
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1474:type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - type_ref -> record Config (Config/include/Luau/Config.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
  //!   - translates_to -> rust_item type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other

  #[cfg(test)]
  #[test]
  fn type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type React_Ref<ElementType> = { current: ElementType } | ((ElementType) -> ())

        type React_AbstractComponent<Config, Instance> = {
            render: ((ref: React_Ref<Instance>) -> nil)
        }

        local createElement : <P, T>(React_AbstractComponent<P, T>) -> ()

        function ScrollView:render()
            local one = table.unpack(
                if true then a else b
            )

            createElement(one)
            createElement(one)
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_correctly_scope_locals_do {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:520:type_infer_correctly_scope_locals_do`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_correctly_scope_locals_do

  #[cfg(test)]
  #[test]
  fn type_infer_correctly_scope_locals_do() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        do
            local a = 1
        end

        local b = a -- oops!
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let us =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("a", us.name());
  }
}

mod type_infer_crazy_complexity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:299:type_infer_crazy_complexity`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_crazy_complexity

  #[cfg(test)]
  #[test]
  fn type_infer_crazy_complexity() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        A:A():A():A():A():A():A():A():A():A():A():A()
    "#,
      ),
      None,
    );
  }
}

mod type_infer_cyclic_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:552:type_infer_cyclic_follow`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_cyclic_follow

  #[cfg(test)]
  #[test]
  fn type_infer_cyclic_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
l0,table,_,_,_ = ...
_,_,_,_.time(...)._.n0,l0,_ = function(l0)
end,_.__index,(_),_.time(_.n0 or _,...)
for l0=...,_,"" do
end
_ += not _
do end
"#,
      ),
      None,
    );
  }
}

mod type_infer_cyclic_follow_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:566:type_infer_cyclic_follow_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_cyclic_follow_2

  #[cfg(test)]
  #[test]
  fn type_infer_cyclic_follow_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
n13,_,table,_,l0,_,_ = ...
_,n0[(_)],_,_._(...)._.n39,l0,_._ = function(l84,...)
end,_.__index,"",_,l0._(nil)
for l0=...,table.n5,_ do
end
_:_(...).n1 /= _
do
_(_ + _)
do end
end
"#,
      ),
      None,
    );
  }
}

mod type_infer_cyclic_unification_aborts_eventually {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2028:type_infer_cyclic_unification_aborts_eventually`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_cyclic_unification_aborts_eventually

  #[cfg(test)]
  #[test]
  fn type_infer_cyclic_unification_aborts_eventually() {
    use alloc::string::String;

    use ulua_analysis::records::code_too_complex::CodeTooComplex;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      functions::has_error::has_error,
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let _instantiate_in_subtyping = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);
    let _type_pack_loop_limit = ScopedFastInt::new(&FInt::LuauTypeInferTypePackLoopLimit, 100);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(r#"pcall(table.unpack({pcall}))"#),
      None,
    );

    assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
  }
}

mod type_infer_dcr_delays_expansion_of_function_containing_blocked_parameter_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1408:type_infer_dcr_delays_expansion_of_function_containing_blocked_parameter_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_dcr_delays_expansion_of_function_containing_blocked_parameter_type

  #[cfg(test)]
  #[test]
  fn type_infer_dcr_delays_expansion_of_function_containing_blocked_parameter_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local b: any

        function f(x)
            local a = b[1] or 'Cn'
            local c = x[1]

            if a:sub(1, #c) == c then
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_dont_ice_on_astexprerror {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:764:type_infer_dont_ice_on_astexprerror`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_dont_ice_on_astexprerror

  #[cfg(test)]
  #[test]
  fn type_infer_dont_ice_on_astexprerror() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = -;
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_dont_ice_when_failing_the_occurs_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:275:type_infer_dont_ice_when_failing_the_occurs_check`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_dont_ice_when_failing_the_occurs_check

  #[cfg(test)]
  #[test]
  fn type_infer_dont_ice_when_failing_the_occurs_check() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local s
        s(s, 'a')
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_dont_report_type_errors_within_an_ast_expr_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:753:type_infer_dont_report_type_errors_within_an_ast_expr_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_dont_report_type_errors_within_an_ast_expr_error

  #[cfg(test)]
  #[test]
  fn type_infer_dont_report_type_errors_within_an_ast_expr_error() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = foo:
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_dont_report_type_errors_within_an_ast_stat_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:742:type_infer_dont_report_type_errors_within_an_ast_stat_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_dont_report_type_errors_within_an_ast_stat_error

  #[cfg(test)]
  #[test]
  fn type_infer_dont_report_type_errors_within_an_ast_stat_error() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        foo
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_exponential_blowup_from_copying_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:349:type_infer_exponential_blowup_from_copying_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_exponential_blowup_from_copying_types

  #[cfg(test)]
  #[test]
  fn type_infer_exponential_blowup_from_copying_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        -- An example of exponential blowup in number of types
        -- The problem is that if we define function f(a) return x end
        -- then this has type <t>(t)->T where x:T
        -- *but* it copies T each time f is applied
        -- so { left = f("hi"), right = f(5) }
        -- has type { left : T_L, right : T_R }
        -- where T_L and T_R are copies of T.
        -- x0 : T0 where T0 = {}
        local x0 = {}
        -- f0 : <t>(t)->T0
        local function f0(a) return x0 end
        -- x1 : T1 where T1 = { left : T0_L, right : T0_R }
        local x1 = { left = f0("hi"), right = f0(5) }
        -- f1 : <t>(t)->T1
        local function f1(a) return x1 end
        -- x2 : T2 where T2 = { left : T1_L, right : T1_R }
        local x2 = { left = f1("hi"), right = f1(5) }
        -- f2 : <t>(t)->T2
        local function f2(a) return x2 end
        -- etc etc
        local x3 = { left = f2("hi"), right = f2(5) }
        local function f3(a) return x3 end
        local x4 = { left = f3("hi"), right = f3(5) }
        return x4
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    assert!(
      5 >= module.interface_types.types.size(),
      "interface type count was {}",
      module.interface_types.types.size()
    );
  }
}

mod type_infer_expr_statement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:185:type_infer_expr_statement`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_expr_statement

  #[cfg(test)]
  #[test]
  fn type_infer_expr_statement() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result =
      fixture.check_string_optional_frontend_options(&String::from("local foo = 5    foo()"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_follow_on_new_types_in_substitution {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1275:type_infer_follow_on_new_types_in_substitution`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_follow_on_new_types_in_substitution

  #[cfg(test)]
  #[test]
  fn type_infer_follow_on_new_types_in_substitution() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local obj = {}

        function obj:Method()
            self.fieldA = function(object)
                if object.a then
                    self.arr[object] = true
                elseif object.b then
                    self.fieldB[object] = object:Connect(function(arg)
                        self.arr[arg] = nil
                    end)
                end
            end
        end

        return obj
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1016:type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level

  #[cfg(test)]
  #[test]
  fn type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        if _ then
            _[_], _ = nil
            _()
        end

        local aaa = function():typeof(_) return 1 end

        if aaa then
            while _() do
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_fuzz_assert_table_freeze_constraint_solving {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2009:type_infer_fuzz_assert_table_freeze_constraint_solving`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_fuzz_assert_table_freeze_constraint_solving

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_assert_table_freeze_constraint_solving() {
    use alloc::string::String;

    use ulua_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::has_error::has_error, records::builtins_fixture::BuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function l0()
        end
        for l0 in false do
        _ = (if _ then table)
        repeat
        do end
        _:freeze(table)
        until if _ then {{n0=_,},(_:freeze()._[_]),}
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
      !has_error::<ConstraintSolvingIncompleteError>(&result),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_fuzz_avoid_singleton_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2730:type_infer_fuzz_avoid_singleton_union`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzz_avoid_singleton_union

  #[cfg(test)]
  #[test]
  // Separate subsystem (NOT the for-in iterator cluster): on this fuzz input the
  // type checker builds a *structurally self-referential* UnionType (`U = {T, X}`
  // where one option follows back to `U` itself — a membership cycle, not a
  // `BoundType` chain, so `follow`'s Floyd cycle-detector cannot see it). Any
  // recursive type predicate that descends union options — e.g. `is_string`
  // (Analysis/src/Type.cpp:199, a faithful 1:1 of C++ `isString`'s
  // `std::all_of(begin(utv), end(utv), isString)`) — then recurses forever and
  // overflows the stack. C++ avoids this only by never constructing such a union
  // for this input; the defect is in the cyclic-union *construction* during the
  // fuzzed `if/elseif`/`setmetatable` expression check, a different subsystem from
  // for-in iteration. (Previously latent: the old-solver `check(AstStatForIn)` was
  // a no-op stub, so iteration never reached `findMetatableEntry`/`isString` on the
  // cyclic type; now that the for-in check is faithfully ported, the pre-existing
  // hazard becomes reachable.) Re-ignored with precise cause per the task's
  // separate-subsystem allowance; fixing it requires de-cycling union construction.
  fn type_infer_fuzz_avoid_singleton_union() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if true then _ else {},if (_) then _ elseif "" then {} elseif _ then {} elseif _ then _ else {}
        for l0,l2 in setmetatable(_,_),l0,_ do
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzz_dont_double_solve_compound_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1972:type_infer_fuzz_dont_double_solve_compound_assignment`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_fuzz_dont_double_solve_compound_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_dont_double_solve_compound_assignment() {
    use alloc::string::String;

    use ulua_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::has_error::has_error, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = {}
        _[function<t0...>(...)
            _[function(...)
                _[_] %= _
                _ = {}
                _ = (- _)()
            end] %= _
            _[_] %= _
        end] %= true
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
      !has_error::<ConstraintSolvingIncompleteError>(&result),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_fuzz_free_table_type_change_during_index_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1372:type_infer_fuzz_free_table_type_change_during_index_check`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_fuzz_free_table_type_change_during_index_check

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_free_table_type_change_during_index_check() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = nil
while _["" >= _] do
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzz_generalize_one_remove_type_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2042:type_infer_fuzz_generalize_one_remove_type_assert`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_fuzz_generalize_one_remove_type_assert

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_generalize_one_remove_type_assert() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = {_ = _}, l0
        _ += _
        while _ do
            while _[_] do
                if _.n0 then
                    _ = _
                else
                    _ = _
                    return _
                end
                do
                    while _ do
                        _, _ = nil
                    end
                    return function()
                    end
                end
                while _[_] do
                    _ = _._VERSION, ""
                end
            end
            local _
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzz_generalize_one_remove_type_assert_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2076:type_infer_fuzz_generalize_one_remove_type_assert_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_fuzz_generalize_one_remove_type_assert_2

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_generalize_one_remove_type_assert_2() {
    use alloc::string::String;

    use ulua_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::has_error::has_error, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = {n0 = _.n0}, -_, _
        _ += _.n0
        _ /= _[_]
        while _.n110 do
            while _._ do
                while _ do
                    while _ do
                        _ = _
                    end
                end
                while _[_] do
                    function _()
                    end
                end
            end
            while ... do
            end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
      !has_error::<ConstraintSolvingIncompleteError>(&result),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_fuzz_global_self_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1843:type_infer_fuzz_global_self_assignment`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzz_global_self_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_global_self_assignment() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(&String::from("_ = _"), None);
  }
}

mod type_infer_fuzz_local_before_declaration_ice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1953:type_infer_fuzz_local_before_declaration_ice`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_fuzz_local_before_declaration_ice

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_local_before_declaration_ice() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{count_mismatch::CountMismatch, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _
        table.freeze(_, _)
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let err0 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("nil", to_string_type_id(err0.given_type));
    assert_eq!("table", to_string_type_id(err0.wanted_type));

    let err1 =
      type_error_data_ref::<CountMismatch>(&result.errors[1]).expect("expected CountMismatch");
    assert_eq!(1, err1.expected());
    assert_eq!(2, err1.actual());
  }
}

mod type_infer_fuzz_missing_follow_table_freeze {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2123:type_infer_fuzz_missing_follow_table_freeze`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_fuzz_missing_follow_table_freeze

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_missing_follow_table_freeze() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        if _:freeze(_)[_][_] then
        else
        do end
        end
        if _:freeze((nil))[_][_] then
        else
        do end
        end
        _ = table,true,_(lower)
        do end
        _:freeze()[_] += {} > _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzz_simplify_combinatorial_explosion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2106:type_infer_fuzz_simplify_combinatorial_explosion`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_fuzz_simplify_combinatorial_explosion

  #[cfg(test)]
  #[test]
  fn type_infer_fuzz_simplify_combinatorial_explosion() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
_ = {[_[`{_ + ...}`]]=_,_,[{_=nil,[_._G]=false,}]={[_[_[_]][_][_ / ...]]=_,[...]=false,_,},[_[_][_][_]]=l255,},""
local _
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
_ = {[(_G)]=_,[_[_[_]][_[_]][nil][_]]={_G=_,},_[_[_]][_][_],n0={[_]=_,_G=_,},248,}
local _
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_allow_failing_to_bind_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2916:type_infer_fuzzer_allow_failing_to_bind_generic`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_allow_failing_to_bind_generic

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_allow_failing_to_bind_generic() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function test(arg1, arg2)
            local fun1 = test(test)
            local fun2 = test(test())
            fun1(arg2, fun2)
        end

        test()
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_attach_polarity_to_ret_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2878:type_infer_fuzzer_attach_polarity_to_ret_free_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_attach_polarity_to_ret_free_type

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_attach_polarity_to_ret_free_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        FOO =
            {
                [1 // setmetatable({}, FOO)] = 2,
                __idiv = function(lhs, rhs, ...) return ... end,
            }
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_avoid_double_negation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2140:type_infer_fuzzer_avoid_double_negation`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_fuzzer_avoid_double_negation

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_avoid_double_negation() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = _
repeat
do end
while 0 do
do
_ = _[0]
_._ *= _
end
if _ then
elseif "" then
end
_ = _[0]
_ = ""
end
_ = ""
until _
while false do
do
_ = _[0]
do end
end
_ = ""
return if _ then _,_
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2847:type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        if if _ then _ else nil then
            local l0 = require(module0)
            _ = l0
        elseif _ then
            function _(l0:true,...)
            end
        else
        end
        _ = l0
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local l0 = require(module0)
        local l10 = require(module0)
        do end
        for l0=_,_,true do
        end
        do
        local l0 = require(module0)
        _ = l0
        local l10 = require(module0)
        function _()
        end
        end
        local l10 = require(module0)
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_bind_generic_sigsegv {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2929:type_infer_fuzzer_bind_generic_sigsegv`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_bind_generic_sigsegv

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_bind_generic_sigsegv() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function test(arg1, arg2)
            local fun = test()
            local fun2 = fun(nil, test(test()))
            fun2(test(test)())
        end

        local f = test()
        f(nil, test())
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_derived_unsound_loops {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1928:type_infer_fuzzer_derived_unsound_loops`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_derived_unsound_loops

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_derived_unsound_loops() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        for _ in ... do
            repeat
                _ = 42
            until _
            repeat
                _ = _ + 2
            until _
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_found_this {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1048:type_infer_fuzzer_found_this`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_found_this

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_found_this() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        l0, _ = nil

        local function p()
            _()
        end

        a = _(
            function():(typeof(p),typeof(_))
            end
        )[nil]
    "#,
      ),
      None,
    );
  }
}

mod type_infer_fuzzer_found_this_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1069:type_infer_fuzzer_found_this_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_found_this_2

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_found_this_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _
        if _ then
            _ = _
            while _() do
                _ = # _
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_fuzzer_global_type_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2943:type_infer_fuzzer_global_type_inference`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_global_type_inference

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_global_type_inference() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        A = A
        A = A
        function A()
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_has_indexer_can_create_cyclic_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2173:type_infer_fuzzer_has_indexer_can_create_cyclic_union`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_has_indexer_can_create_cyclic_union

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_has_indexer_can_create_cyclic_union() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = nil
        repeat
            _ = {[true] = _[_]}
            do
                repeat
                    _ = {[_[l0]] = _[_]}
                    return
                until #next(_) < _
            end
            local l0 = require(module0)
        until #_[_](_) < next(_)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_infer_divergent_rw_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2424:type_infer_fuzzer_infer_divergent_rw_props`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_infer_divergent_rw_props

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_infer_divergent_rw_props() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        return function(l0:{_:(any)&(any),write _:any,})
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_instantiate_iter_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2953:type_infer_fuzzer_instantiate_iter_function`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_instantiate_iter_function

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_instantiate_iter_function() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _polarity = ScopedFastFlag::new(&FFlag::LuauInstantiationUsesPolarity, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function iterfunc(l0)
            return l0()
        end
        for _, _ in setmetatable({}, { __iter = iterfunc }) do
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_fuzzer_missing_follow_in_assign_index_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2396:type_infer_fuzzer_missing_follow_in_assign_index_constraint`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_assign_index_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_missing_follow_in_assign_index_constraint() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        _._G = nil
        for _ in ... do
        break
        end
        for _ in function<t0,t0,t0>(l0)
        _,_._,l0 = l0,_,_._
        local _ = l0,{[_]=_,}
        _[{nil=_,}](_)
        end,{[_]=_,} do
        end
        _ -= _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_missing_follow_in_checking_generic_mapping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2893:type_infer_fuzzer_missing_follow_in_checking_generic_mapping`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_checking_generic_mapping

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_missing_follow_in_checking_generic_mapping() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _<U...,M...>(l0,l0,l0,l0,)
            l0(_(rshift),_()(_(if _ then _),))
            _()(_(_(_)))
        end
        _()(_()(_(true,_)),)
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _<Y...,U...,M...>(l0:any,l0,l0,...)
            _()(_,_()(_(_()),_))
            do end
        end
        do end
        _()(_(""),{})
        do end
        for _ in ... do
        end
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_missing_follow_in_function_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2838:type_infer_fuzzer_missing_follow_in_function_call`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_function_call

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_missing_follow_in_function_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        do end
        _ = if _ then true elseif _ then if _ then _ elseif _ then 2 .. {} elseif _._ then l0 else _ elseif _ then if ... then _ elseif {} then `` elseif _ then {_G=_,}
        type t0<),A,)...> = ({_G:any,write n0:any,write _:any<<A...>()->()>,write [any]:""""""""""""""""""""userda290013136ta:(0x000062900131369029001313690"""})|(l0.any)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_missing_follow_in_instantiation_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2768:type_infer_fuzzer_missing_follow_in_instantiation_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_instantiation_2

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_missing_follow_in_instantiation2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if {l0._,} then if _ then _ elseif rawset({[_]=_,[{_._,}]=_,}) then _ else {_._,} elseif rawset(_) then (true),""
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_missing_type_pack_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2548:type_infer_fuzzer_missing_type_pack_follow`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_fuzzer_missing_type_pack_follow

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_missing_type_pack_follow() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = {[0]=_,}
while _ do
do
local l2 = require(module0)
end
end
do end
function _(l0:typeof(_),l0,l0)
local l0 = require(module0)
_()(l0(),_,_(_())((_)))
do end
end
_()(_(if nil then _))("",_,_(_,(_)))
do end
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = {_,}
while _ do
do
do end
end
end
_ = nil
function _(l0,l0,l0)
local l0 = require(module0)
_()(_(),_,_(_())(_,true)(_,_),l0)
do end
end
_()(_())("",_.n0,_,_(_,true,(_)))
do end
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_occurs_check_stack_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2413:type_infer_fuzzer_occurs_check_stack_overflow`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_fuzzer_occurs_check_stack_overflow

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_occurs_check_stack_overflow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        _ = if _ then _
        for l0 in ... do
        type t0 = (()->((t0<t0...>)->())|(any))|(typeof(_))
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_pack_check_missing_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1894:type_infer_fuzzer_pack_check_missing_follow`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_pack_check_missing_follow

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_pack_check_missing_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
_ = n255
function _()
setmetatable(_)[_[xpcall(_,setmetatable(_,_()))]] /= xpcall(_,_)
_.n16(_,_)[_[_]] *= _
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_fuzzer_simplify_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2212:type_infer_fuzzer_simplify_crash`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_simplify_crash

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_simplify_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        if _ then
            _ = nil
        else if _ and _ then
            _ = nil
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_simplify_is_check_on_bound_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2223:type_infer_fuzzer_simplify_is_check_on_bound_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_simplify_is_check_on_bound_type

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_simplify_is_check_on_bound_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        _[if _ then false],_,_._,log10 = {{[_]={_,},_G=not function():true
        _ = nil
        end,},[_[_ + true][_][_]]=_,sort=_,},_
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_simplify_table_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2190:type_infer_fuzzer_simplify_table_indexer`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_simplify_table_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_simplify_table_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        _[_] += true
        _ = {
            [{
                [_] = _[_][if ... then _ else _](),
                [-1795162112] = function()
                end,
                [{
                    _G = function()
                    end
                }] = _(_(true)),
                _G = _
            }] = _,
            [_[not _][_]] = _(),
            _
        }

    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_fuzzer_unify_with_free_missing_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1906:type_infer_fuzzer_unify_with_free_missing_follow`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_fuzzer_unify_with_free_missing_follow

  #[cfg(test)]
  #[test]
  fn type_infer_fuzzer_unify_with_free_missing_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
for _ in ... do
repeat
local function l0(l0)
end
_ = l0["aaaa"]
repeat
_ = true,_("")
_ = _[_]
until _
until _
repeat
_ = if _ then _,_()
_ = _[_]
until _
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_getmetatable_infer_any_param {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1880:type_infer_getmetatable_infer_any_param`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_getmetatable_infer_any_param

  #[cfg(test)]
  #[test]
  fn type_infer_getmetatable_infer_any_param() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function check(x): any
            return getmetatable(x)
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(unknown) -> any",
        to_string_type_id(fixture.base.require_type_string(&String::from("check")))
      );
    } else {
      assert_eq!(
        "({ @metatable any, {+  +} }) -> any",
        to_string_type_id(fixture.base.require_type_string(&String::from("check")))
      );
    }
  }
}

mod type_infer_getmetatable_infer_any_ret {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1869:type_infer_getmetatable_infer_any_ret`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_getmetatable_infer_any_ret

  #[cfg(test)]
  #[test]
  fn type_infer_getmetatable_infer_any_ret() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function spooky(x: any)
            return getmetatable(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(any) -> any",
      to_string_type_id(fixture.base.require_type_string(&String::from("spooky")))
    );
  }
}

mod type_infer_getmetatable_works_with_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1852:type_infer_getmetatable_works_with_any`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_getmetatable_works_with_any

  #[cfg(test)]
  #[test]
  fn type_infer_getmetatable_works_with_any() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        return {
            new = function(name: string)
                local self = newproxy(true) :: any

                getmetatable(self).__tostring = function()
                    return "Hello, I am " .. name
                end

                return self
            end,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_globals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:472:type_infer_globals`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_globals

  #[cfg(test)]
  #[test]
  fn type_infer_globals() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        foo = true
        foo = "now i'm a string!"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_globals_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:487:type_infer_globals_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_globals_2

  #[cfg(test)]
  #[test]
  fn type_infer_globals_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        foo = function() return 1 end
        foo = "now i'm a string!"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("() -> (...any)", to_string_type_id(tm.wanted_type));
    assert_eq!("string", to_string_type_id(tm.given_type));
    assert_eq!(
      "() -> (...any)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_globals_are_banned_in_strict_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:506:type_infer_globals_are_banned_in_strict_mode`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_globals_are_banned_in_strict_mode

  #[cfg(test)]
  #[test]
  fn type_infer_globals_are_banned_in_strict_mode() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        foo = true
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let us =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("foo", us.name());
  }
}

mod type_infer_handle_self_referential_has_prop_constraints {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1511:type_infer_handle_self_referential_has_prop_constraints`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_handle_self_referential_has_prop_constraints

  #[cfg(test)]
  #[test]
  fn type_infer_handle_self_referential_has_prop_constraints() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function calculateTopBarHeight(props)
        end
        local function isTopPage(props)
            local topMostOpaquePage
            if props.avatarRoute then
                topMostOpaquePage = props.avatarRoute.opaque.name
            else
                topMostOpaquePage = props.opaquePage
            end
        end

        function TopBarContainer:updateTopBarHeight(prevProps, prevState)
            calculateTopBarHeight(self.props)
            isTopPage(self.props)
            local topMostOpaquePage
            if self.props.avatarRoute then
                topMostOpaquePage = self.props.avatarRoute.opaque.name
                --                  ^--------------------------------^
            else
                topMostOpaquePage = self.props.opaquePage
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_if_statement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:191:type_infer_if_statement`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_if_statement

  #[cfg(test)]
  #[test]
  fn type_infer_if_statement() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a
        local b

        if true then
            a = 'hello'
        else
            b = 999
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    } else {
      assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    }
  }
}

mod type_infer_if_then_else_bidirectional_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2474:type_infer_if_then_else_bidirectional_inference`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_if_then_else_bidirectional_inference

  #[cfg(test)]
  #[test]
  fn type_infer_if_then_else_bidirectional_inference() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type foo = {
            bar: (() -> string)?,
        }
        local qux: foo = if false then {} else 10
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.given_type));
    assert_eq!("foo", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_if_then_else_two_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2492:type_infer_if_then_else_two_errors`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_if_then_else_two_errors

  #[cfg(test)]
  #[test]
  fn type_infer_if_then_else_two_errors() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{missing_properties::MissingProperties, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type foo = {
            bar: () -> string,
        }
        local qux: foo = if false then {} else 10
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    let err1 = type_error_data_ref::<MissingProperties>(&result.errors[0])
      .expect("expected MissingProperties");
    assert_eq!("foo", to_string_type_id(err1.super_type()));
    assert_eq!("{  }", to_string_type_id(err1.sub_type()));

    let err2 =
      type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!("foo", to_string_type_id(err2.wanted_type));
    assert_eq!("number", to_string_type_id(err2.given_type));
  }
}

mod type_infer_index_expr_should_be_checked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:701:type_infer_index_expr_should_be_checked`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_index_expr_should_be_checked

  #[cfg(test)]
  #[test]
  fn type_infer_index_expr_should_be_checked() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: any

        print(foo[(true).x])
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let up =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("boolean", to_string_type_id(up.table()));
    assert_eq!("x", up.key());
  }
}

mod type_infer_indexing_a_cyclic_intersection_does_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1082:type_infer_indexing_a_cyclic_intersection_does_not_crash`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_indexing_a_cyclic_intersection_does_not_crash

  #[cfg(test)]
  #[test]
  fn type_infer_indexing_a_cyclic_intersection_does_not_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _
        if _ then
            while nil do
                _ = _
            end
        end
        if _[if _ then ""] then
            while nil do
                _ = if _ then ""
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_infer_assignment_value_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:880:type_infer_infer_assignment_value_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_infer_assignment_value_types

  #[cfg(test)]
  #[test]
  fn type_infer_infer_assignment_value_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: (number, number) -> number = function(a, b) return a - b end

a = function(a, b) return a + b end

local b: {number|string}
local c: {number|string}
b, c = {2, "s"}, {"b", 4}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_infer_assignment_value_types_mutable_lval {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:895:type_infer_infer_assignment_value_types_mutable_lval`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_infer_assignment_value_types_mutable_lval

  #[cfg(test)]
  #[test]
  fn type_infer_infer_assignment_value_types_mutable_lval() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a = {}
a.x = 2
a = setmetatable(a, { __call = function(x) end })
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_infer_in_nocheck_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:157:type_infer_infer_in_nocheck_mode`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_infer_in_nocheck_mode

  #[cfg(test)]
  #[test]
  fn type_infer_infer_in_nocheck_mode() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nocheck
        function f(x)
            return x
        end
         -- we get type information even if there's type errors
        f(1, 2)
    "#,
      ),
      None,
    );

    assert_eq!(
      "(any) -> (...any)",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_infer_locals_via_assignment_from_its_call_site {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:133:type_infer_infer_locals_via_assignment_from_its_call_site`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_infer_locals_via_assignment_from_its_call_site

  #[cfg(test)]
  #[test]
  fn type_infer_infer_locals_via_assignment_from_its_call_site() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a
        function f(x) a = x end
        f(1)
        f("foo")
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "unknown",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "(unknown) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    }
  }
}

mod type_infer_infer_locals_with_nil_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:105:type_infer_infer_locals_with_nil_value`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_infer_locals_with_nil_value

  #[cfg(test)]
  #[test]
  fn type_infer_infer_locals_with_nil_value() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::primitive_type::PrimitiveType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local f = nil; f = 'hello world'"),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("string?", to_string_type_id(f_type));
    } else {
      assert_eq!(
        Some(PrimitiveType::STRING),
        fixture.get_primitive_type(f_type)
      );
    }
  }
}

mod type_infer_infer_locals_with_nil_value_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:121:type_infer_infer_locals_with_nil_value_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_infer_locals_with_nil_value_2

  #[cfg(test)]
  #[test]
  fn type_infer_infer_locals_with_nil_value_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 2
        local b = a,nil
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
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_infer_through_group_expr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:906:type_infer_infer_through_group_expr`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_infer_through_group_expr

  #[cfg(test)]
  #[test]
  fn type_infer_infer_through_group_expr() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function f(a: (number, number) -> number) return a(1, 3) end
f(((function(a, b) return a + b end)))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_infer_type_assertion_value_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:869:type_infer_infer_type_assertion_value_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_infer_type_assertion_value_type

  #[cfg(test)]
  #[test]
  fn type_infer_infer_type_assertion_value_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function f()
    return {4, "b", 3} :: {string|number}
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_infer_types_of_globals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1807:type_infer_infer_types_of_globals`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_infer_types_of_globals

  #[cfg(test)]
  #[test]
  fn type_infer_infer_types_of_globals() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        foo = 5
        print(foo)
    "#,
      ),
      None,
    );

    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 3,
        column: 14,
      }))
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'foo'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_invalide_deprecated_attribute_doesn_t_chrash_checker {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:690:type_infer_invalide_deprecated_attribute_doesn_t_chrash_checker`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_invalide_deprecated_attribute_doesn_t_chrash_checker

  #[cfg(test)]
  #[test]
  fn type_infer_invalide_deprecated_attribute_doesn_t_chrash_checker() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
@[deprecated{ reason = reasonString }]
function hello(x: number, y: number): number
    return x + y
end"#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_is_safe_integer_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2352:type_infer_is_safe_integer_example`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> function isInteger (Analysis/src/Type.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_is_safe_integer_example

  #[cfg(test)]
  #[test]
  fn type_infer_is_safe_integer_example() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/isInteger"),
      String::from(
        r#"
        --!strict
        return function(value)
            return type(value) == "number" and value ~= math.huge and value == math.floor(value)
        end
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/MAX_SAFE_INTEGER"),
      String::from(
        r#"
        --!strict
        return 42
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/Util"),
      String::from(
        r#"
        --!strict
        local isInteger = require(script.Parent.isInteger)
        local MAX_SAFE_INTEGER = require(script.Parent.MAX_SAFE_INTEGER)
        return function(value)
        	return isInteger(value) and math.abs(value) <= MAX_SAFE_INTEGER
        end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Util"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1338:type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict

  #[cfg(test)]
  #[test]
  fn type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function validate(stats, hits, misses)
            local checked = {}

            for _,l in ipairs(hits) do
                if not (stats[l] and stats[l] > 0) then
                    return false, string.format("expected line %d to be hit", l)
                end
                checked[l] = true
            end

            for _,l in ipairs(misses) do
                if not (stats[l] and stats[l] == 0) then
                    return false, string.format("expected line %d to be missed", l)
                end
                checked[l] = true
            end

            for k,v in pairs(stats) do
                if type(k) == "number" and not checked[k] then
                    return false, string.format("expected line %d to be absent", k)
                end
            end

            return true
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_item_2236_iterate_over_table_with_values_as_optional_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2819:type_infer_item_2236_iterate_over_table_with_values_as_optional_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_item_2236_iterate_over_table_with_values_as_optional_types

  #[cfg(test)]
  #[test]
  fn type_infer_2236_iterate_over_table_with_values_as_optional_types() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauRefineNilFromTableIndexerResultType, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    ];

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local t: { number? } = {}

        for _, v in t do
            local x: number = v
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_iterate_over_local_table_with_optional_indexer_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2797:type_infer_iterate_over_local_table_with_optional_indexer_values`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_iterate_over_local_table_with_optional_indexer_values

  #[cfg(test)]
  #[test]
  fn type_infer_iterate_over_local_table_with_optional_indexer_values() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauRefineNilFromTableIndexerResultType, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    ];

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type TypeA = {Value: any}

        local list = {} :: {[string]: TypeA?}

        for index, a in list do
            a.Value = 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_iterate_over_table_with_optional_indexer_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2775:type_infer_iterate_over_table_with_optional_indexer_values`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_iterate_over_table_with_optional_indexer_values

  #[cfg(test)]
  #[test]
  fn type_infer_iterate_over_table_with_optional_indexer_values() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauRefineNilFromTableIndexerResultType, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    ];

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Bar = {x: number}
        type Foo = {[string]: Bar?}

        function printAllClassNames(foo: Foo)
            for _, value in foo do
                print(value.x)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_leading_ampersand {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1704:type_infer_leading_ampersand`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_leading_ampersand

  #[cfg(test)]
  #[test]
  fn type_infer_leading_ampersand() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Amp = & string
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_alias(&String::from("Amp")))
    );
  }
}

mod type_infer_leading_ampersand_no_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1725:type_infer_leading_ampersand_no_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_leading_ampersand_no_type

  #[cfg(test)]
  #[test]
  fn type_infer_leading_ampersand_no_type() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Amp = &
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected type, got <eof>",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_alias(&String::from("Amp")))
    );
  }
}

mod type_infer_leading_bar {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1683:type_infer_leading_bar`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_leading_bar

  #[cfg(test)]
  #[test]
  fn type_infer_leading_bar() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Bar = | number
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_alias(&String::from("Bar")))
    );
  }
}

mod type_infer_leading_bar_no_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1714:type_infer_leading_bar_no_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_leading_bar_no_type

  #[cfg(test)]
  #[test]
  fn type_infer_leading_bar_no_type() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Bar = |
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected type, got <eof>",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_alias(&String::from("Bar")))
    );
  }
}

mod type_infer_leading_bar_question_mark {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1693:type_infer_leading_bar_question_mark`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_leading_bar_question_mark

  #[cfg(test)]
  #[test]
  fn type_infer_leading_bar_question_mark() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Bar = |?
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected type, got '?'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "*error-type*?",
      to_string_type_id(fixture.require_type_alias(&String::from("Bar")))
    );
  }
}

mod type_infer_lti_must_record_contributing_locations {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1578:type_infer_lti_must_record_contributing_locations`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_lti_must_record_contributing_locations

  #[cfg(test)]
  #[test]
  fn type_infer_lti_must_record_contributing_locations() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, get_type_alt_j::get_type_id},
      records::function_type::FunctionType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a)
            if math.random() > 0.5 then
                math.abs(a)
            else
                string.len(a)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let fn_ty = fixture.base.require_type_string(&String::from("f"));
    let function = get_type_id::<FunctionType>(fn_ty).expect("expected f to have a function type");

    let arg_ty = first(function.arg_types(), false).expect("expected first argument");
    let module = unsafe { &*fixture.base.get_main_module(false) };
    let locations = module
      .upper_bound_contributors
      .find(&arg_ty)
      .expect("expected upper-bound contributors for f argument");
    assert_eq!(2, locations.len());
  }
}

mod type_infer_luau_resolves_symbols_the_same_way_lua_does {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:773:type_infer_luau_resolves_symbols_the_same_way_lua_does`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_luau_resolves_symbols_the_same_way_lua_does

  #[cfg(test)]
  #[test]
  fn type_infer_luau_resolves_symbols_the_same_way_lua_does() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function Funky()
            local a: number = foo
        end

        local foo: string = 'hello'
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  }
}

mod type_infer_multiple_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1823:type_infer_multiple_assignment`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_multiple_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_multiple_assignment() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function requireString(arg: string) end
        local function requireNumber(arg: number) end

        local function f(): ...number end

        local w: "a", x, y, z = "a", 1, f()
        requireString(w)
        requireNumber(x)
        requireNumber(y)
        requireNumber(z)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_nested_functions_can_depend_on_outer_generics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2651:type_infer_nested_functions_can_depend_on_outer_generics`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_nested_functions_can_depend_on_outer_generics

  #[cfg(test)]
  #[test]
  fn type_infer_nested_functions_can_depend_on_outer_generics() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function name<P>(arg1: P)
            return function(what: P) return what end
        end

        local funcTest = name(nil)
        local out = funcTest(1) -- Doesn't report type mismatch error anymore
    "#,
      ),
      None,
    );

    assert_eq!(
      "(nil) -> nil",
      to_string_type_id(fixture.require_type_string(&String::from("funcTest")))
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("nil", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_no_heap_use_after_free_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:852:type_infer_no_heap_use_after_free_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_no_heap_use_after_free_error

  #[cfg(test)]
  #[test]
  fn type_infer_no_heap_use_after_free_error() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        _ += _:n0(xpcall,_)
        local l0
        do end
        while _ do
            function _:_()
                _ += _(_._(_:n0(xpcall,_)))
            end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_no_infinite_loop_when_trying_to_unify_uh_this {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:838:type_infer_no_infinite_loop_when_trying_to_unify_uh_this`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_no_infinite_loop_when_trying_to_unify_uh_this

  #[cfg(test)]
  #[test]
  fn type_infer_no_infinite_loop_when_trying_to_unify_uh_this() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(l22,l0):((((boolean)|(t0))|(t0))&(()->(()->(()->()->{},(t0<t22>)|(t0)),any)))
            return function():t0<t0>
            end
        end
        type t0<t0> = ((typeof(_))|(any))|(typeof(_))
        _()
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_no_stack_overflow_from_isoptional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:790:type_infer_no_stack_overflow_from_isoptional`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record OccursCheckFailed (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_no_stack_overflow_from_isoptional

  #[cfg(test)]
  #[test]
  fn type_infer_no_stack_overflow_from_isoptional() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::occurs_check_failed::OccursCheckFailed,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(l0:t0): (any, ()->())
            return 0,_
        end

        type t0 = t0 | {}
        _(nil)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let t0 = fixture
      .lookup_type(&String::from("t0"))
      .expect("expected type alias t0");
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("any", to_string_type_id(t0));
    } else {
      assert_eq!("*error-type*", to_string_type_id(t0));
    }

    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<OccursCheckFailed>(error).is_some()),
      "expected OccursCheckFailed: {:?}",
      result.errors
    );
  }
}

mod type_infer_no_stack_overflow_from_isoptional_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:822:type_infer_no_stack_overflow_from_isoptional_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_no_stack_overflow_from_isoptional_2

  #[cfg(test)]
  #[test]
  fn type_infer_no_stack_overflow_from_isoptional_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(l0:({})|(t0)):((((typeof((xpcall)))|(t96<t0>))|(t13))&(t96<t0>),()->typeof(...))
            return 0,_
        end

        type t0<t107> = ((typeof((_G)))|(({})|(t0)))|(t0)
        _(nil)

        local t: ({})|(t0)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_non_standalone_constraint_solving_incomplete_is_hidden {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2531:type_infer_non_standalone_constraint_solving_incomplete_is_hidden`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_non_standalone_constraint_solving_incomplete_is_hidden

  #[cfg(test)]
  #[test]
  fn type_infer_non_standalone_constraint_solving_incomplete_is_hidden() {
    use alloc::string::String;

    use ulua_analysis::records::{
      constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
      type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true),
    ];

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
        local x: number = true
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<ConstraintSolvingIncompleteError>(&result.errors[0])
      .expect("expected ConstraintSolvingIncompleteError");
    type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  }
}

mod type_infer_obvious_type_error_in_nocheck_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:175:type_infer_obvious_type_error_in_nocheck_mode`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_obvious_type_error_in_nocheck_mode

  #[cfg(test)]
  #[test]
  fn type_infer_obvious_type_error_in_nocheck_mode() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nocheck
        local x: string = 5
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_occurs_check_does_not_recurse_forever_if_asked_to_traverse_a_cyclic_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:287:type_infer_occurs_check_does_not_recurse_forever_if_asked_to_traverse_a_cyclic_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_occurs_check_does_not_recurse_forever_if_asked_to_traverse_a_cyclic_type

  #[cfg(test)]
  #[test]
  fn type_infer_occurs_check_does_not_recurse_forever_if_asked_to_traverse_a_cyclic_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
         --!strict
        function u(t, w)
            u(u, t)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_occurs_isnt_always_failure {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1395:type_infer_occurs_isnt_always_failure`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_occurs_isnt_always_failure

  #[cfg(test)]
  #[test]
  fn type_infer_occurs_isnt_always_failure() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(x, c)                   -- x : X
    local y = if c then x else nil -- y : X?
    local z = if c then x else nil -- z : X?
    y = z
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oss_1815_verbatim {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2445:type_infer_oss_1815_verbatim`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oss_1815_verbatim

  #[cfg(test)]
  #[test]
  fn type_infer_oss_1815_verbatim() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local item: "foo" = "bar"
        item = if true then "foo" else "foo"

        local item2: "foo" = if true then "doge" else "doge2"
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 28
        },
        end: Position {
          line: 2,
          column: 33
        },
      },
      result.errors[0].location
    );
    let err1 =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err1.wanted_type));
    assert_eq!("\"bar\"", to_string_type_id(err1.given_type));

    assert_eq!(
      Location {
        begin: Position {
          line: 5,
          column: 42
        },
        end: Position {
          line: 5,
          column: 48
        },
      },
      result.errors[1].location
    );
    let err2 =
      type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err2.wanted_type));
    assert_eq!("\"doge\"", to_string_type_id(err2.given_type));

    assert_eq!(
      Location {
        begin: Position {
          line: 5,
          column: 54
        },
        end: Position {
          line: 5,
          column: 61
        },
      },
      result.errors[2].location
    );
    let err3 =
      type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err3.wanted_type));
    assert_eq!("\"doge2\"", to_string_type_id(err3.given_type));
  }
}

mod type_infer_promote_tail_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1549:type_infer_promote_tail_type_packs`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_promote_tail_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_promote_tail_type_packs() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local A: any = nil

        local C
        local D = A(
            A({}, {
                __call = function(a): string
                    local E: string = C(a)
                    return E
                end
            }),
            {
                F = function(s: typeof(C))
                end
            }
        )

        function C(b: any): string
            return ''
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_react_lua_follow_free_type_ub {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1736:type_infer_react_lua_follow_free_type_ub`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias Component (Analysis/include/Luau/TypePath.h)
  //!   - translates_to -> rust_item type_infer_react_lua_follow_free_type_ub

  #[cfg(test)]
  #[test]
  fn type_infer_react_lua_follow_free_type_ub() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        return function(Roact)
            local Tree = Roact.Component:extend("Tree")

            function Tree:render()
                local breadth, components, depth, id, wrap =
                    self.props.breadth, self.props.components, self.props.depth, self.props.id, self.props.wrap
                local Box = components.Box
                if depth == 0 then
                    Roact.createElement(Box, {})
                else
                    Roact.createElement(Tree, {})
                end

            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_read_table_type_refinements_persist_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2434:type_infer_read_table_type_refinements_persist_scope`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_read_table_type_refinements_persist_scope

  #[cfg(test)]
  #[test]
  fn type_infer_read_table_type_refinements_persist_scope() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
_ = {n0=_,},if _._ then ... else if _[if _ then _ else ({nil,})].setmetatable then if _ then _ elseif l0 then ... elseif _.n0 then _ elseif function<A>(l0)
return _._G,_
end then _._G else ...
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1427:type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
  //!   - translates_to -> rust_item type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter

  #[cfg(test)]
  #[test]
  fn type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local TRUE: true = true

        local function matches(value, t: true)
            if value then
                return true
            end
        end

        local function readValue(breakpoint)
            if matches(breakpoint, TRUE) then
                readValue(breakpoint)
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(unknown) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
      );
    } else {
      assert_eq!(
        "<a>(a) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
      );
    }
  }
}

mod type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1451:type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter_2

  #[cfg(test)]
  #[test]
  fn type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function readValue(breakpoint)
            if type(breakpoint) == 'number' then
                readValue(breakpoint)
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(unknown) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
      );
    } else {
      assert_eq!(
        "(number) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
      );
    }
  }
}

mod type_infer_recursive_metatable_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1099:type_infer_recursive_metatable_crash`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_recursive_metatable_crash

  #[cfg(test)]
  #[test]
  fn type_infer_recursive_metatable_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function getIt()
    local y
    y = setmetatable({}, y)
    return y
end
local a = getIt()
local b = getIt()
local c = a or b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_regexp_hang {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2232:type_infer_regexp_hang`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::negate (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_regexp_hang

  #[cfg(test)]
  #[test]
  fn type_infer_regexp_hang() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local outln, group_id, verb_flags = {}, {}, {
    newline = 1,
    newline_seq = 1,
    not_empty = 0
}
if not escape_c then
elseif escape_c >= 48 and escape_c <= 57 then
elseif escape_c == 69 then
elseif escape_c == 81 then
elseif escape_c == 78 then
    if codes[i] ~= 125 or i == start_i then
    end
    table.insert(outln, code_point)
elseif escape_c == 80 or escape_c == 112 then
    if script_set then
    elseif not valid_categories[c_name]then
    else
        table.insert(outln, { 'category', negate, c_name })
    end
elseif escape_c == 103 and (codes[i + 1] == 123 or codes[i + 1] >= 48 and codes[i + 1] <= 57)then
elseif escape_c == 111 then
elseif escape_c == 120 then
else
    table.insert(outln, esc_char or escape_c)
end

for i, v in ipairs(outln)do
    if type(v) == 'table' and (v[1] == 40 or v[1] == 'quantifier' and type(v[5]) == 'table' and v[5][1] == 40)then
        v = v[5]
    elseif type(v) == 'table' and (v[1] == 'backref' or v[1] == 'recurmatch')then
        for i1, v1 in ipairs(outln)do
            break
        end
    end
end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_self_bound_due_to_compound_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2273:type_infer_self_bound_due_to_compound_assign`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_self_bound_due_to_compound_assign

  #[cfg(test)]
  #[test]
  fn type_infer_self_bound_due_to_compound_assign() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Camera with
            CameraType: string
            CFrame: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function MT_UPDATE(CAMERA: Camera, Enum: any, totalOffsets: number, focusToCFrame: number, magnitude: number)
            if CAMERA.CameraType ~= Enum.CameraType.Custom then
                return
            end

            local goalCFrame = (CAMERA.CFrame) * totalOffsets
            if goalCFrame ~= CAMERA.CFrame then
                goalCFrame -= (focusToCFrame * magnitude) -- Offset the goalCFrame the raycast direction based on the cutoff distance.
            end
        end

        return {}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_should_be_able_to_infer_this_without_stack_overflowing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:334:type_infer_should_be_able_to_infer_this_without_stack_overflowing`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_should_be_able_to_infer_this_without_stack_overflowing

  #[cfg(test)]
  #[test]
  fn type_infer_should_be_able_to_infer_this_without_stack_overflowing() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x, y)
            return x or y
        end

        local function dont_crash(x, y)
            local z: typeof(f(x, y)) = f(x, y)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_standalone_constraint_solving_incomplete_is_hidden {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2514:type_infer_standalone_constraint_solving_incomplete_is_hidden`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_standalone_constraint_solving_incomplete_is_hidden

  #[cfg(test)]
  #[test]
  fn type_infer_standalone_constraint_solving_incomplete_is_hidden() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true),
      ScopedFastFlag::new(
        &FFlag::DebugLuauAlwaysShowConstraintSolvingIncomplete,
        false,
      ),
    ];

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_statements_are_topologically_sorted {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:218:type_infer_statements_are_topologically_sorted`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_statements_are_topologically_sorted

  #[cfg(test)]
  #[test]
  fn type_infer_statements_are_topologically_sorted() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo()
            return bar(999), bar("hi")
        end

        function bar(i)
            return i
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_stringify_nested_unions_with_optionals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:717:type_infer_stringify_nested_unions_with_optionals`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_stringify_nested_unions_with_optionals

  #[cfg(test)]
  #[test]
  fn type_infer_stringify_nested_unions_with_optionals() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local a: number | (string | boolean) | nil
        local b: number = a
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
    assert_eq!(
      "(boolean | number | string)?",
      to_string_type_id(tm.given_type)
    );
  }
}

mod type_infer_tc_after_error_recovery {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:611:type_infer_tc_after_error_recovery`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tc_after_error_recovery

  #[cfg(test)]
  #[test]
  fn type_infer_tc_after_error_recovery() {
    use alloc::string::String;

    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x =
        local a = 7
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let a_type = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(a_type)
    );
  }
}

mod type_infer_tc_after_error_recovery_no_assert {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:624:type_infer_tc_after_error_recovery_no_assert`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tc_after_error_recovery_no_assert

  #[cfg(test)]
  #[test]
  fn type_infer_tc_after_error_recovery_no_assert() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("function +() local _ = true end"),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_tc_after_error_recovery_no_replacement_name_in_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:630:type_infer_tc_after_error_recovery_no_replacement_name_in_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_after_error_recovery_no_replacement_name_in_error

  #[cfg(test)]
  #[test]
  fn type_infer_tc_after_error_recovery_no_replacement_name_in_error() {
    use alloc::string::String;

    use ulua_analysis::enums::solver_mode::SolverMode;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    {
      ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
      fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
          SolverMode::New
        } else {
          SolverMode::Old
        });
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            --!strict
            local t = { x = 10, y = 20 }
            return t.
        "#,
        ),
        None,
      );
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }

    {
      fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
          SolverMode::New
        } else {
          SolverMode::Old
        });
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            --!strict
            export type = number
            export type = string
        "#,
        ),
        None,
      );
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    {
      ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
      fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
          SolverMode::New
        } else {
          SolverMode::Old
        });
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            --!strict
            function string.() end
        "#,
        ),
        None,
      );
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }

    {
      fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
          SolverMode::New
        } else {
          SolverMode::Old
        });
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            --!strict
            local function () end
            local function () end
        "#,
        ),
        None,
      );
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    {
      fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
          SolverMode::New
        } else {
          SolverMode::Old
        });
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            --!strict
            local dm = {}
            function dm.() end
            function dm.() end
        "#,
        ),
        None,
      );
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_tc_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:59:type_infer_tc_error`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tc_error

  #[cfg(test)]
  #[test]
  fn type_infer_tc_error() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local a = 7   local b = 'hi'   a = b"),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location {
          begin: Position {
            line: 0,
            column: 35
          },
          end: Position {
            line: 0,
            column: 36
          }
        },
        result.errors[0].location
      );

      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
      assert_eq!(fixture.get_builtins().string_type, tm.given_type);
    }
  }
}

mod type_infer_tc_error_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:79:type_infer_tc_error_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_tc_error_2

  #[cfg(test)]
  #[test]
  fn type_infer_tc_error_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result =
      fixture.check_string_optional_frontend_options(&String::from("local a = 7   a = 'hi'"), None);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number | string",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location {
          begin: Position {
            line: 0,
            column: 18
          },
          end: Position {
            line: 0,
            column: 22
          }
        },
        result.errors[0].location
      );

      let a_type = fixture.require_type_string(&String::from("a"));
      let string_type = fixture.get_builtins().string_type;
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(a_type, tm.wanted_type);
      assert_eq!(string_type, tm.given_type);
    }
  }
}

mod type_infer_tc_hello_world {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:42:type_infer_tc_hello_world`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tc_hello_world

  #[cfg(test)]
  #[test]
  fn type_infer_tc_hello_world() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(&String::from("local a = 7"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_tc_if_else_expressions_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:916:type_infer_tc_if_else_expressions_1`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_1

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(r#"local a = if true then "true" else "false""#),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_tc_if_else_expressions_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:925:type_infer_tc_if_else_expressions_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_2

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local a = if false then "a" elseif false then "b" else "c"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_tc_if_else_expressions_expected_type_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:944:type_infer_tc_if_else_expressions_expected_type_1`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_expected_type_1

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_expected_type_1() {
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
type X = {number | string}
local a: X = if true then {"1", 2, 3} else {4, 5, 6}
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{number | string}",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("a")),
        &mut opts
      )
    );
  }
}

mod type_infer_tc_if_else_expressions_expected_type_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:955:type_infer_tc_if_else_expressions_expected_type_2`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_expected_type_2

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_expected_type_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: number? = if true then 1 else nil
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tc_if_else_expressions_expected_type_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:964:type_infer_tc_if_else_expressions_expected_type_3`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_expected_type_3

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_expected_type_3() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function times<T>(n: any, f: () -> T)
    local result: {T} = {}
    local res = f()
    table.insert(result, if true then res else n)
    return result
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tc_if_else_expressions_type_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:936:type_infer_tc_if_else_expressions_type_union`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_tc_if_else_expressions_type_union

  #[cfg(test)]
  #[test]
  fn type_infer_tc_if_else_expressions_type_union() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(r#"local a: number? = if true then 42 else nil"#),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "number?",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("a")),
        &mut opts
      )
    );
  }
}

mod type_infer_tc_interpolated_string_basic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:978:type_infer_tc_interpolated_string_basic`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_interpolated_string_basic

  #[cfg(test)]
  #[test]
  fn type_infer_tc_interpolated_string_basic() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: string = `hello {"world"}`
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tc_interpolated_string_constant_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:998:type_infer_tc_interpolated_string_constant_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_tc_interpolated_string_constant_type

  #[cfg(test)]
  #[test]
  fn type_infer_tc_interpolated_string_constant_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: "hello" = `hello`
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tc_interpolated_string_with_invalid_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:987:type_infer_tc_interpolated_string_with_invalid_expression`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_tc_interpolated_string_with_invalid_expression

  #[cfg(test)]
  #[test]
  fn type_infer_tc_interpolated_string_with_invalid_expression() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number) end

        local foo: string = `hello {f("uh oh")}`
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_tc_propagation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:50:type_infer_tc_propagation`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_tc_propagation

  #[cfg(test)]
  #[test]
  fn type_infer_tc_propagation() {
    use alloc::string::String;

    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
      .check_string_optional_frontend_options(&String::from("local a = 7   local b = a"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b_type = fixture.require_type_string(&String::from("b"));
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(b_type)
    );
  }
}

mod type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2588:type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
  //!   - translates_to -> rust_item type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type

  #[cfg(test)]
  #[test]
  fn type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = Fixture::default();
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local any = nil :: any

        function f1(x)
            x:m()
            local _ = x.A.p.a
        end

        function f2(x)
            local _ = x.d
        end

        function f3(x)
            local a = ""
            a = x.d.p
            local _ = undef[x.a]
        end

        function f4(x)
            f2(x)
            if undef and x and x:m() then
                any(x)
                return
            end
            f3(x)
            for _, v in any.x do
                local a = x[v].p
            end
            a.b = x
            if x.q ~= nil then
                f1(x) -- things go bad here
            end
        end

        return f4
    "#,
      ),
      None,
    );
  }
}

mod type_infer_type_errors_infer_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:307:type_infer_type_errors_infer_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_errors_infer_types

  #[cfg(test)]
  #[test]
  fn type_infer_type_errors_infer_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_property::UnknownProperty,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local err = (true).x
        local c = err.Parent.Reward.GetChildren
        local d = err.Parent.Reward
        local e = err.Parent
        local f = err
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("boolean", to_string_type_id(err.table()));
    assert_eq!("x", err.key());

    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
      );
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("d")))
      );
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("e")))
      );
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
    }
  }
}

mod type_infer_type_infer_cache_limit_normalizer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1262:type_infer_type_infer_cache_limit_normalizer`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_type_infer_cache_limit_normalizer

  #[cfg(test)]
  #[test]
  fn type_infer_type_infer_cache_limit_normalizer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _normalize_cache_limit = ScopedFastInt::new(&FInt::LuauNormalizeCacheLimit, 10);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x : ((number) -> number) & ((string) -> string) & ((nil) -> nil) & (({}) -> {})
        local y : (number | string | nil | {}) -> (number | string | nil | {}) = x
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Code is too complex to typecheck! Consider simplifying the code around this area",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_type_infer_recursion_limit_no_ice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1207:type_infer_type_infer_recursion_limit_no_ice`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_type_infer_recursion_limit_no_ice

  #[cfg(test)]
  #[test]
  fn type_infer_type_infer_recursion_limit_no_ice() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 2);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function complex()
          function _(l0:t0): (any, ()->())
              return 0,_
          end
          type t0 = t0 | {}
          _(nil)
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Type contains a self-recursive construct that cannot be resolved",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        "Code is too complex to typecheck! Consider simplifying the code around this area",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_type_infer_recursion_limit_normalizer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1229:type_infer_type_infer_recursion_limit_normalizer`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Fixture::validateErrors (tests/Fixture.cpp)
  //!   - calls -> method Fixture::getErrors (tests/Fixture.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_type_infer_recursion_limit_normalizer

  #[cfg(test)]
  #[test]
  fn type_infer_type_infer_recursion_limit_normalizer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 10);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a,b,c,d,e,f,g,h,i,j>()
            local x : a&b&c&d&e&f&g&h&(i?)
            local y : (a&b&c&d&e&f&g&h&i)? = x
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let too_complex =
      "Code is too complex to typecheck! Consider simplifying the code around this area";

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(3, result.errors.len(), "{:?}", result.errors);
      let expected_locations = [
        Location {
          begin: Position {
            line: 2,
            column: 22,
          },
          end: Position {
            line: 2,
            column: 42,
          },
        },
        Location {
          begin: Position {
            line: 3,
            column: 22,
          },
          end: Position {
            line: 3,
            column: 42,
          },
        },
        Location {
          begin: Position {
            line: 3,
            column: 22,
          },
          end: Position {
            line: 3,
            column: 41,
          },
        },
      ];

      for (error, expected_location) in result.errors.iter().zip(expected_locations.iter()) {
        assert_eq!(*expected_location, error.location);
        assert_eq!(too_complex, to_string_type_error(error));
      }
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location {
          begin: Position {
            line: 3,
            column: 12,
          },
          end: Position {
            line: 3,
            column: 46,
          },
        },
        result.errors[0].location
      );
      assert_eq!(too_complex, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_type_remover_heap_use_after_free {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2378:type_infer_type_remover_heap_use_after_free`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_type_remover_heap_use_after_free

  #[cfg(test)]
  #[test]
  fn type_infer_type_remover_heap_use_after_free() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if l0.n0.n0 then {n4(...,setmetatable(setmetatable(_),_)),_ == _,} elseif _.ceil._ then _ elseif _ then not _
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        do
        _ = if _[_] then {[_(``)]="y",} elseif _ then _ elseif _[_] then "" elseif _ then _ elseif _[_] then {} elseif _[_] then false else ""
        end
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local l249 = require(module0)
        _,_ = {[`{_}`]=_,[_._G._]=(_)(),[_["" + _]._G]={_=_,_=_,[_._G[_]._]=_G,},},_,(_)()
    "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_typechecking_in_type_guards {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1383:type_infer_typechecking_in_type_guards`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_typechecking_in_type_guards

  #[cfg(test)]
  #[test]
  fn type_infer_typechecking_in_type_guards() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local a = type(foo) == 'nil'
local b = typeof(foo) ~= 'nil'
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'foo'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Unknown global 'foo'; consider assigning to it first",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_typeof_cannot_refine_builtin_alias {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1635:type_infer_typeof_cannot_refine_builtin_alias`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_typeof_cannot_refine_builtin_alias

  #[cfg(test)]
  #[test]
  fn type_infer_typeof_cannot_refine_builtin_alias() {
    use alloc::{string::String, sync::Arc};
    use core::ptr::null_mut;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{freeze::freeze, unfreeze::unfreeze},
      records::{scope::Scope, table_type::TableType, type_fun::TypeFun, type_level::TypeLevel},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let frontend = fixture.get_frontend();

    unsafe {
      let global_scope = frontend.globals.global_scope();
      let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
      let arena = frontend.globals.global_types_mut();

      unfreeze(arena);

      let global_table_ty = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Sealed,
        TypeLevel::default(),
        null_mut(),
      ));

      (*global_scope_ptr).exported_type_bindings.insert(
        String::from("GlobalTable"),
        TypeFun::type_fun_type_id(global_table_ty),
      );

      freeze(arena);
    }

    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(x)
            if typeof(x) == 'GlobalTable' then
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_types_stored_in_ast_resolved_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1298:type_infer_types_stored_in_ast_resolved_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_types_stored_in_ast_resolved_types

  #[cfg(test)]
  #[test]
  fn type_infer_types_stored_in_ast_resolved_types() {
    use alloc::string::String;

    use ulua_analysis::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;
    use ulua_ast::{
      records::{ast_expr_function::AstExprFunction, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type alias = typeof("hello")
        local function foo(param: alias)
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let node = unsafe {
      find_node_at_position_source_module_position(
        &*fixture.get_main_source_module(),
        Position {
          line: 2,
          column: 16,
        },
      )
    };
    assert!(!node.is_null());

    let ty = fixture
      .lookup_type(&String::from("alias"))
      .expect("expected alias type");

    let func = unsafe { ast_node_as::<AstExprFunction>(node) };
    assert!(!func.is_null());
    assert_eq!(1, unsafe { (*func).args.len() });

    let arg = unsafe {
      *(*func)
        .args
        .as_slice()
        .first()
        .expect("expected function arg")
    };
    let annotation = unsafe { (*arg).annotation };
    assert!(!annotation.is_null());

    let module = unsafe { &*fixture.get_main_module(false) };
    assert_eq!(
      Some(&ty),
      module.ast_resolved_types.find(&(annotation as *const _))
    );
  }
}

mod type_infer_unify_nearly_identical_recursive_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:234:type_infer_unify_nearly_identical_recursive_types`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_unify_nearly_identical_recursive_types

  #[cfg(test)]
  #[test]
  fn type_infer_unify_nearly_identical_recursive_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local o
        o:method()

        local p
        p:method()

        o = p
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_unterminated_function_body_causes_constraint_generator_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2672:type_infer_unterminated_function_body_causes_constraint_generator_crash`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_unterminated_function_body_causes_constraint_generator_crash

  #[cfg(test)]
  #[test]
  fn type_infer_unterminated_function_body_causes_constraint_generator_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
export type t = {
	func : typeof(
		function
	)
}

export type t1 = t12

export type t2 = {}

export type t3 = {
	foo:number
	bar:number
}

export type t4 = "foobar"

export type t5 = string

export type t6 = number

export type t7 = "foobar"

export type t8 = "foobar"

export type t9 = typeof(1)

export type t10 = typeof(1)

export type t11 = typeof(1)

export type t12 = {
	b:number
	pb:number
}
"#,
      ),
      None,
    );
  }
}

mod type_infer_visit_error_nodes_in_lvalue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1759:type_infer_visit_error_nodes_in_lvalue`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TypeIterator::descend (Analysis/include/Luau/Type.h)
  //!   - calls -> macro lvalue (VM/src/lobject.h)
  //!   - translates_to -> rust_item type_infer_visit_error_nodes_in_lvalue

  #[cfg(test)]
  #[test]
  fn type_infer_visit_error_nodes_in_lvalue() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        (::,
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_warn_on_lowercase_parent_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:251:type_infer_warn_on_lowercase_parent_property`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record DeprecatedApiUsed (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_warn_on_lowercase_parent_property

  #[cfg(test)]
  #[test]
  fn type_infer_warn_on_lowercase_parent_property() {
    use alloc::string::String;

    use ulua_analysis::records::deprecated_api_used::DeprecatedApiUsed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local M = require(script.parent.DoesNotMatter)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let ed = type_error_data_ref::<DeprecatedApiUsed>(&result.errors[0])
      .expect("expected DeprecatedApiUsed");
    assert_eq!("parent", ed.symbol);
  }
}

mod type_infer_weird_case {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:265:type_infer_weird_case`
  //! Source: `tests/TypeInfer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_weird_case

  #[cfg(test)]
  #[test]
  fn type_infer_weird_case() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f() return 4 end
        local d = math.deg(f())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
