extern crate alloc;

mod type_infer_type_packs_cyclic_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:996:type_infer_type_packs_cyclic_type_packs`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_cyclic_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_cyclic_type_packs() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
_ += _(_,...)
repeat
_ += _(...)
until ... + _
"#,
      ),
      None,
    );

    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
_ += _(_(...,...),_(...))
repeat
until _
"#,
      ),
      None,
    );
  }
}

mod type_infer_type_packs_detect_cyclic_typepacks {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1015:type_infer_type_packs_detect_cyclic_typepacks`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_detect_cyclic_typepacks

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_detect_cyclic_typepacks() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type ( ... ) ( ) ;
        ( ... ) ( - - ... ) ( - ... )
        type = ( ... ) ;
        ( ... ) (  ) ( ... ) ;
        ( ... ) ""
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_dont_ice_if_a_type_pack_is_an_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:983:type_infer_type_packs_dont_ice_if_a_type_pack_is_an_error`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_dont_ice_if_a_type_pack_is_an_error

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_dont_ice_if_a_type_pack_is_an_error() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function f(s)
            print(s)
            return f
        end

        f("foo")("bar")
    "#,
      ),
      None,
    );
  }
}

mod type_infer_type_packs_empty_varargs_should_return_nil_when_not_in_tail_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:40:type_infer_type_packs_empty_varargs_should_return_nil_when_not_in_tail_position`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_empty_varargs_should_return_nil_when_not_in_tail_position

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_empty_varargs_should_return_nil_when_not_in_tail_position() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a, b = ..., 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_fuzz_typepack_iter_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1123:type_infer_type_packs_fuzz_typepack_iter_follow`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_fuzz_typepack_iter_follow

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_fuzz_typepack_iter_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local _
local _ = _,_(),_(_)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_fuzz_typepack_iter_follow_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1133:type_infer_type_packs_fuzz_typepack_iter_follow_2`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_fuzz_typepack_iter_follow_2

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_fuzz_typepack_iter_follow_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function test(name, searchTerm)
    local found = string.find(name:lower(), searchTerm:lower())
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_generalize_expected_types_with_proper_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1106:type_infer_type_packs_generalize_expected_types_with_proper_scope`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_generalize_expected_types_with_proper_scope

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_generalize_expected_types_with_proper_scope() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _instantiate = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<TResult>(fn: () -> ...TResult): () -> ...TResult
            return function()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_higher_order_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:88:type_infer_type_packs_higher_order_function`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_higher_order_function

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_higher_order_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply(f, g, x)
            return f(g(x))
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "<a, b..., c...>((c...) -> (b...), (a) -> (c...), a) -> (b...)"
    } else {
      "<a, b..., c...>((b...) -> (c...), (a) -> (b...), a) -> (c...)"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("apply")))
    );
  }
}

mod type_infer_type_packs_infer_multi_return {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:18:type_infer_type_packs_infer_multi_return`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_type_packs_infer_multi_return

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_infer_multi_return() {
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
        function take_two()
            return 2, 2
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let take_two_type = fixture.require_type_string(&String::from("take_two"));
    let take_two_type = get_type_id::<FunctionType>(take_two_type).expect("expected FunctionType");
    let (returns, tail) = flatten_type_pack_id(take_two_type.ret_types());

    assert_eq!(2, returns.len());
    assert_eq!("number", to_string_type_id(follow_type_id(returns[0])));
    assert_eq!("number", to_string_type_id(follow_type_id(returns[1])));
    assert!(tail.is_none());
  }
}

mod type_infer_type_packs_last_element_of_return_statement_can_itself_be_a_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:60:type_infer_type_packs_last_element_of_return_statement_can_itself_be_a_pack`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method PathBuilder::rets (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_last_element_of_return_statement_can_itself_be_a_pack

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_last_element_of_return_statement_can_itself_be_a_pack() {
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
        function take_two()
            return 2, 2
        end

        function take_three()
            return 1, take_two()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let take_three_type = fixture.require_type_string(&String::from("take_three"));
    let take_three_type =
      get_type_id::<FunctionType>(take_three_type).expect("expected FunctionType");
    let (returns, tail) = flatten_type_pack_id(take_three_type.ret_types());

    assert_eq!(3, returns.len());
    assert_eq!("number", to_string_type_id(follow_type_id(returns[0])));
    assert_eq!("number", to_string_type_id(follow_type_id(returns[1])));
    assert_eq!("number", to_string_type_id(follow_type_id(returns[2])));
    assert!(tail.is_none());
  }
}

mod type_infer_type_packs_multiple_varargs_inference_are_not_confused {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:160:type_infer_type_packs_multiple_varargs_inference_are_not_confused`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_multiple_varargs_inference_are_not_confused

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_multiple_varargs_inference_are_not_confused() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(...)
            local a: string = ...

            return function(...)
                local b: number = ...
            end
        end

        f("foo", "bar")(1, 2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_no_return_size_should_be_zero {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:119:type_infer_type_packs_no_return_size_should_be_zero`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_type_packs_no_return_size_should_be_zero

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_no_return_size_should_be_zero() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{flatten_type_pack::flatten_type_pack_id, get_type_alt_j::get_type_id},
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a:any) return a end
        function g() return end
        function h() end

        g(h())
        f(g(),h())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    let f_type = get_type_id::<FunctionType>(f_type).expect("expected FunctionType for f");
    assert_eq!(1, flatten_type_pack_id(f_type.ret_types()).0.len());

    let g_type = fixture.require_type_string(&String::from("g"));
    let g_type = get_type_id::<FunctionType>(g_type).expect("expected FunctionType for g");
    assert_eq!(0, flatten_type_pack_id(g_type.ret_types()).0.len());

    let h_type = fixture.require_type_string(&String::from("h"));
    let h_type = get_type_id::<FunctionType>(h_type).expect("expected FunctionType for h");
    assert_eq!(0, flatten_type_pack_id(h_type.ret_types()).0.len());
  }
}

mod type_infer_type_packs_pack_tail_unification_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:918:type_infer_type_packs_pack_tail_unification_check`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_pack_tail_unification_check

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_pack_tail_unification_check() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local a: () -> (number, ...string)
local b: () -> (number, ...boolean)
a = b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'; \nit returns a tail of the variadic `boolean` in the latter type and `string` in the former type, and `boolean` is not a subtype of `string`"
    } else {
      "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'\ncaused by:\n  Expected this to be 'string', but got 'boolean'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_parenthesized_varargs_returns_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:177:type_infer_type_packs_parenthesized_varargs_returns_any`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_parenthesized_varargs_returns_any

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_parenthesized_varargs_returns_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local value

        local function f(...)
            value = ...
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("value")))
    );
  }
}

mod type_infer_type_packs_return_type_should_be_empty_if_nothing_is_returned {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:104:type_infer_type_packs_return_type_should_be_empty_if_nothing_is_returned`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_type_packs_return_type_should_be_empty_if_nothing_is_returned

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_return_type_should_be_empty_if_nothing_is_returned() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{flatten_type_pack::flatten_type_pack_id, get_type_alt_j::get_type_id},
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f() end
        function g() return end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    let f_type = get_type_id::<FunctionType>(f_type).expect("expected FunctionType for f");
    assert_eq!(0, flatten_type_pack_id(f_type.ret_types()).0.len());

    let g_type = fixture.require_type_string(&String::from("g"));
    let g_type = get_type_id::<FunctionType>(g_type).expect("expected FunctionType for g");
    assert_eq!(0, flatten_type_pack_id(g_type.ret_types()).0.len());
  }
}

mod type_infer_type_packs_self_and_varargs_should_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:49:type_infer_type_packs_self_and_varargs_should_work`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_self_and_varargs_should_work

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_self_and_varargs_should_work() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        function t:f(...) end
        t:f(1)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_type_alias_backwards_compatible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:544:type_infer_type_packs_type_alias_backwards_compatible`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_backwards_compatible

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_backwards_compatible() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X<T> = () -> T
        type Y<T, U> = (T) -> U

        type A = X<(number)>
        type B = Y<(number), (boolean)>
        type C = Y<(number), boolean>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    for (name, expected) in [
      ("A", "() -> number"),
      ("B", "(number) -> boolean"),
      ("C", "(number) -> boolean"),
    ] {
      let ty = fixture
        .lookup_type(&String::from(name))
        .unwrap_or_else(|| panic!("expected type alias {name}"));
      assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
  }
}

mod type_infer_type_packs_type_alias_default_export {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:835:type_infer_type_packs_type_alias_default_export`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_export

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_export() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
      .check_module_name_optional_frontend_options(&String::from("Module/Types"), None);
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
      .check_module_name_optional_frontend_options(&String::from("Module/Users"), None);
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
            .require_type_module_name_string("Module/Users", &String::from(name))
        ),
        "{name}"
      );
    }
  }
}

mod type_infer_type_packs_type_alias_default_mixed_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:745:type_infer_type_packs_type_alias_default_mixed_self`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_mixed_self

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_mixed_self() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U = T, V... = ...number, W... = (T, U, V...)> = { a: (T, U, V...) -> W... }
local a: Y<number>
local b: Y<number, string>
local c: Y<number, string, ...boolean>
local d: Y<number, string, ...boolean, ...() -> ()>
    "#,
      ),
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
        to_string_type_id(fixture.require_type_string(&String::from(name))),
        "{name}"
      );
    }
  }
}

mod type_infer_type_packs_type_alias_default_type_chained {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:681:type_infer_type_packs_type_alias_default_type_chained`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_chained

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_chained() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U = T, V = U> = { a: T, b: U, c: V }

local a: Y<number>
local b: Y<number, string>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number, number, number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Y<number, string, string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:763:type_infer_type_packs_type_alias_default_type_errors`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Y<T = T> = { a: T }
        local a: Y = { a = 2 }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!("Unknown type 'T'", to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_type_alias_default_type_errors_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:776:type_infer_type_packs_type_alias_default_type_errors_2`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors_2

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Y<T... = T...> = { a: (T...) -> () }
        local a: Y<>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!("Unknown type 'T'", to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_type_alias_default_type_errors_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:787:type_infer_type_packs_type_alias_default_type_errors_3`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors_3

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Y<T = string, U... = ...string> = { a: (T) -> U... }
        local a: Y<...number>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Type parameters must come before type pack parameters"
    } else {
      "Generic type 'Y<T, U...>' expects at least 1 type argument, but none are specified"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_type_alias_default_type_errors_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:801:type_infer_type_packs_type_alias_default_type_errors_4`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors_4

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors_4() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Packed<T> = (T) -> T
        local a: Packed
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Generic type 'Packed<T>' expects 1 type argument, but none are specified"
    } else {
      "Type parameter list is required"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_type_alias_default_type_errors_5 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:815:type_infer_type_packs_type_alias_default_type_errors_5`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors_5

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors_5() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Y<T, U = T, V> = { a: T }
        local a: Y<number>
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_type_alias_default_type_errors_6 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:825:type_infer_type_packs_type_alias_default_type_errors_6`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_errors_6

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_errors_6() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Y<T..., U... = T..., V...> = { a: T }
        local a: Y<...number>
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_type_alias_default_type_explicit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:627:type_infer_type_packs_type_alias_default_type_explicit`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_explicit

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_explicit() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U = string> = { a: T, b: U }

local a: Y<number, number> = { a = 2, b = 3 }
local b: Y<number> = { a = 2, b = "s" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number, number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Y<number, string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T = string> = { a: T }

local a: Y<number> = { a = 2 }
local b: Y<> = { a = "s" }
local c: Y = { a = "s" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Y<string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Y<string>",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_pack_explicit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:696:type_infer_type_packs_type_alias_default_type_pack_explicit`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_pack_explicit

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_pack_explicit() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T... = (string, number)> = { a: (T...) -> () }
local a: Y<>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<string, number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_pack_self_chained_tp {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:733:type_infer_type_packs_type_alias_default_type_pack_self_chained_tp`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_pack_self_chained_tp

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_pack_self_chained_tp() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T..., U... = T..., V... = U...> = { a: (T...) -> U..., b: (T...) -> V... }
local a: Y<number, string>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<(number, string), (number, string), (number, string)>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_pack_self_tp {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:721:type_infer_type_packs_type_alias_default_type_pack_self_tp`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_pack_self_tp

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_pack_self_tp() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T..., U... = T...> = { a: (T...) -> U... }
local a: Y<number, string>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<(number, string), (number, string)>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_pack_self_ty {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:708:type_infer_type_packs_type_alias_default_type_pack_self_ty`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_pack_self_ty

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_pack_self_ty() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U... = ...T> = { a: T, b: (U...) -> T }

local a: Y<number>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number, ...number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:656:type_infer_type_packs_type_alias_default_type_self`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_self

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_self() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U = T> = { a: T, b: U }

local a: Y<number> = { a = 2, b = 3 }
local b: Y<string> = { a = "h", b = "s" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number, number>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Y<string, string>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T, U = (T, T) -> string> = { a: T, b: U }

local a: Y<number>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<number, (number, number) -> string>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_default_type_skip_brackets {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:880:type_infer_type_packs_type_alias_default_type_skip_brackets`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_skip_brackets

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_default_type_skip_brackets() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T... = ...string> = (T...) -> number
local a: Y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(...string) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_type_packs_type_alias_defaults_confusing_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:892:type_infer_type_packs_type_alias_defaults_confusing_types`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_defaults_confusing_types

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_defaults_confusing_types() {
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
type A<T, U = T, V... = ...any, W... = V...> = (T, V...) -> (U, W...)
type B = A<string, (number)>
type C = A<string, (number), (boolean)>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b = fixture
      .lookup_type(&String::from("B"))
      .expect("expected type alias B");
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "(string, ...any) -> (number, ...any)",
      to_string_type_id_to_string_options(b, &mut opts)
    );

    let c = fixture
      .lookup_type(&String::from("C"))
      .expect("expected type alias C");
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "(string, boolean) -> (number, boolean)",
      to_string_type_id_to_string_options(c, &mut opts)
    );
  }
}

mod type_infer_type_packs_type_alias_defaults_recursive_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:906:type_infer_type_packs_type_alias_defaults_recursive_type`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_defaults_recursive_type

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_defaults_recursive_type() {
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
type F<K = string, V = (K) -> ()> = (K) -> V
type R = { m: F<R> }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let r = fixture
      .lookup_type(&String::from("R"))
      .expect("expected type alias R");
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "t1 where t1 = { m: (t1) -> (t1) -> () }",
      to_string_type_id_to_string_options(r, &mut opts)
    );
  }
}

mod type_infer_type_packs_type_alias_instantiated_but_missing_parameter_list {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:613:type_infer_type_packs_type_alias_instantiated_but_missing_parameter_list`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_instantiated_but_missing_parameter_list

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_instantiated_but_missing_parameter_list() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Packed<T...> = (T...) -> T...
local a: Packed
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Generic type 'Packed<T...>' expects 1 type pack argument, but none are specified"
    } else {
      "Type parameter list is required"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_type_alias_type_pack_explicit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:487:type_infer_type_packs_type_alias_type_pack_explicit`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_explicit

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_pack_explicit() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X<T...> = (T...) -> (T...)

type A<S...> = X<(S...)>
type B = X<()>
type C = X<(number)>
type D = X<(number, string)>
type E = X<(...number)>
type F = X<(string, ...number)>
    "#,
      ),
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
        .lookup_type(&String::from(name))
        .unwrap_or_else(|| panic!("expected type alias {name}"));
      assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
  }
}

mod type_infer_type_packs_type_alias_type_pack_explicit_multi {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:510:type_infer_type_packs_type_alias_type_pack_explicit_multi`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_explicit_multi

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_pack_explicit_multi() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T..., U...> = (T...) -> (U...)

type A = Y<(number, string), (boolean)>
type B = Y<(), ()>
type C<S...> = Y<...string, (number, S...)>
type D<X...> = Y<X..., (number, string, X...)>
    "#,
      ),
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
        .lookup_type(&String::from(name))
        .unwrap_or_else(|| panic!("expected type alias {name}"));
      assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
  }
}

mod type_infer_type_packs_type_alias_type_pack_explicit_multi_tostring {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:529:type_infer_type_packs_type_alias_type_pack_explicit_multi_tostring`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_explicit_multi_tostring

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_pack_explicit_multi_tostring() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Y<T..., U...> = { f: (T...) -> (U...) }

local a: Y<(number, string), (boolean)>
local b: Y<(), ()>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Y<(number, string), (boolean)>",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Y<(), ()>",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_type_packs_type_alias_type_pack_multi {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:459:type_infer_type_packs_type_alias_type_pack_multi`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_multi

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_pack_multi() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
        .lookup_type(&String::from(name))
        .unwrap_or_else(|| panic!("expected type alias {name}"));
      assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
  }
}

mod type_infer_type_packs_type_alias_type_pack_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:440:type_infer_type_packs_type_alias_type_pack_variadic`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_variadic

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_pack_variadic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X<T...> = (T...) -> (string, T...)

type D = X<...number>
type E = X<(number, ...string)>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let d = fixture
      .lookup_type(&String::from("D"))
      .expect("expected type alias D");
    let e = fixture
      .lookup_type(&String::from("E"))
      .expect("expected type alias E");
    assert_eq!("(...number) -> (string, ...number)", to_string_type_id(d));
    assert_eq!(
      "(number, ...string) -> (string, number, ...string)",
      to_string_type_id(e)
    );
  }
}

mod type_infer_type_packs_type_alias_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:290:type_infer_type_packs_type_alias_type_packs`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
        to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
      },
      records::{table_type::TableType, to_string_options::ToStringOptions},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Packed<T...> = (T...) -> T...
local a: Packed<>
local b: Packed<number>
local c: Packed<string, number>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
      .lookup_type(&String::from("Packed"))
      .expect("expected type alias Packed");
    assert_eq!("(T...) -> (T...)", to_string_type_id(packed));
    assert_eq!(
      "() -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "(number) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "(string, number) -> (string, number)",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
-- (U..., T) cannot be parsed right now
type Packed<T, U...> = { f: (a: T, U...) -> (T, U...) }
local a: Packed<number>
local b: Packed<string, number>
local c: Packed<string, number, boolean>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
      .lookup_type(&String::from("Packed"))
      .expect("expected type alias Packed");
    assert_eq!("Packed<T, U...>", to_string_type_id(packed));
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ f: (T, U...) -> (T, U...) }",
      to_string_type_id_to_string_options(packed, &mut opts)
    );

    let a = fixture.require_type_string(&String::from("a"));
    let a_table = get_type_id::<TableType>(a).expect("expected TableType for a");
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

    let b = fixture.require_type_string(&String::from("b"));
    let b_table = get_type_id::<TableType>(b).expect("expected TableType for b");
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

    let c = fixture.require_type_string(&String::from("c"));
    let c_table = get_type_id::<TableType>(c).expect("expected TableType for c");
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
}

mod type_infer_type_packs_type_alias_type_packs_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:562:type_infer_type_packs_type_alias_type_packs_errors`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_errors

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_packs_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut check_error = |source: &str, expected: &str| {
      let result = fixture.check_string_optional_frontend_options(&String::from(source), None);
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
}

mod type_infer_type_packs_type_alias_type_packs_import {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:354:type_infer_type_packs_type_alias_type_packs_import`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupImportedType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_import

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_packs_import() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
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
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Import = require(game.A)
local a: Import.Packed<number>
local b: Import.Packed<string, number>
local c: Import.Packed<string, number, boolean>
local d: { a: typeof(c) }
    "#,
      ),
      None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let packed = fixture
      .base
      .lookup_imported_type(&String::from("Import"), &String::from("Packed"))
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
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("a")),
        &mut opts,
      )
    );
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string, b: (number) -> () }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("b")),
        &mut opts,
      )
    );
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string, b: (number, boolean) -> () }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("c")),
        &mut opts,
      )
    );
    assert_eq!(
      "{ a: Packed<string, number, boolean> }",
      to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );
  }
}

mod type_infer_type_packs_type_alias_type_packs_nested {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:420:type_infer_type_packs_type_alias_type_packs_nested`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_nested

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_alias_type_packs_nested() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Packed1<T...> = (T...) -> (T...)
type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)
type Packed3<T...> = (Packed2<T...>, T...) -> (Packed2<T...>, T...)
type Packed4<T...> = (Packed3<T...>, T...) -> (Packed3<T...>, T...)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
      .lookup_type(&String::from("Packed4"))
      .expect("expected type alias Packed4");
    assert_eq!(
      "((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...) -> \
((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...)",
      to_string_type_id(packed)
    );
  }
}

mod type_infer_type_packs_type_pack_hidden_free_tail_infinite_growth {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:260:type_infer_type_packs_type_pack_hidden_free_tail_infinite_growth`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_pack_hidden_free_tail_infinite_growth

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_pack_hidden_free_tail_infinite_growth() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_type_pack_type_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:385:type_infer_type_packs_type_pack_type_parameters`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_pack_type_parameters

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_pack_type_parameters() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
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
      &String::from(
        r#"
local Import = require(game.A)
type Alias<S, T, R...> = Import.Packed<S, (T, R...)>
local a: Alias<string, number, boolean>

type B<X...> = Import.Packed<string, X...>
type C<X...> = Import.Packed<string, (number, X...)>
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let alias = fixture
      .base
      .lookup_type(&String::from("Alias"))
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
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("a")),
        &mut opts,
      )
    );

    let b = fixture
      .base
      .lookup_type(&String::from("B"))
      .expect("expected type alias B");
    assert_eq!("B<X...>", to_string_type_id(b));
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string, b: (X...) -> () }",
      to_string_type_id_to_string_options(b, &mut opts)
    );

    let c = fixture
      .base
      .lookup_type(&String::from("C"))
      .expect("expected type alias C");
    assert_eq!("C<X...>", to_string_type_id(c));
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string, b: (number, X...) -> () }",
      to_string_type_id_to_string_options(c, &mut opts)
    );
  }
}

mod type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1085:type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = if !FFlag::DebugLuauForceOldSolver.get() {
      Some(ScopedFastFlag::new(
        &FFlag::LuauInstantiateInSubtyping,
        true,
      ))
    } else {
      None
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
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
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_type_param_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1144:type_infer_type_packs_type_param_overflow`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_type_param_overflow

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_type_param_overflow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Two<T,U> = { a: T, b: U }
        local x: Two<number, string, number> = { a = 1, b = 'c' }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_unify_variadic_tails_in_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1043:type_infer_type_packs_unify_variadic_tails_in_arguments`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_unify_variadic_tails_in_arguments

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_unify_variadic_tails_in_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(...: string): number
            return 1
        end

        function bar(...: number): number
            return foo(...)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_type_packs_unify_variadic_tails_in_arguments_free {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1061:type_infer_type_packs_unify_variadic_tails_in_arguments_free`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_unify_variadic_tails_in_arguments_free

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_unify_variadic_tails_in_arguments_free() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo<T...>(...: T...): T...
            return ...
        end

        function bar(...: number): boolean
            return foo(...)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be 'boolean', but got '...number'; \nit has a tail of `...number`, which is not a subtype of `boolean`"
    } else {
      "Expected this to be 'boolean', but got 'number'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:955:type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - calls -> method Module::hasModuleScope (Analysis/src/Module.cpp)
  //!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack()
  {
    use alloc::string::String;

    use ulua_analysis::functions::{begin_type_pack::begin, end_type_pack::end};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function a(x) return 1 end
        a(...)
    "#,
      ),
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

    assert!(iter.operator_ne(&end_iter));
    iter.operator_inc();
    assert!(iter.operator_eq(&end_iter));
    assert_eq!(None, iter.tail());
  }
}

mod type_infer_type_packs_varargs_inference_through_multiple_scopes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:144:type_infer_type_packs_varargs_inference_through_multiple_scopes`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_varargs_inference_through_multiple_scopes

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_varargs_inference_through_multiple_scopes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(...)
            do
                local a: string = ...
                local b: number = ...
            end
        end

        f("foo")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_variadic_argument_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:277:type_infer_type_packs_variadic_argument_tail`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_packs_variadic_argument_tail

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_variadic_argument_tail() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local _ = function():((...any)->(...any),()->())
    return function() end, function() end
end
for y in _() do
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_type_packs_variadic_pack_syntax {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:245:type_infer_type_packs_variadic_pack_syntax`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_type_packs_variadic_pack_syntax

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_variadic_pack_syntax() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local function foo(...: number)
        end

        foo(1, 2, 3, 4, 5, 6)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(...number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_type_packs_variadic_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:192:type_infer_type_packs_variadic_packs`
  //! Source: `tests/TypeInfer.typePacks.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_packs_variadic_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_packs_variadic_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        add_global_binding_builtin_definitions::add_global_binding_builtin_definitions,
        follow_type::follow_type_id, freeze::freeze, unfreeze::unfreeze,
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
      &String::from(
        r#"
        --!strict

        foo(1, 2, 3, "foo")
        bar(1, "foo", "bar", 3)
    "#,
      ),
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
    assert_eq!(number_type, follow_type_id(first.wanted_type));
    assert_eq!(string_type, follow_type_id(first.given_type));

    let second = type_error_data_ref::<TypeMismatch>(&result.errors[1])
      .expect("expected second error to be TypeMismatch");
    assert_eq!(string_type, follow_type_id(second.wanted_type));
    assert_eq!(number_type, follow_type_id(second.given_type));
  }
}
