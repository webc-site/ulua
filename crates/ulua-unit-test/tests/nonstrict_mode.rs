extern crate alloc;

mod nonstrict_mode_allow_error_type_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:354:nonstrict_mode_allow_error_type_nonstrict`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_allow_error_type_nonstrict

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_allow_error_type_nonstrict() {
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauNonStrictModeUseErrorSupressingTag, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local sublist: any
        if sublist then
            for _, entry in sublist do
                local _ = string.upper(entry)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_delay_function_does_not_require_its_argument_to_return_anything {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:246:nonstrict_mode_delay_function_does_not_require_its_argument_to_return_anything`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_delay_function_does_not_require_its_argument_to_return_anything

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_delay_function_does_not_require_its_argument_to_return_anything() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function delay(ms: number?, cb: () -> ()): () end

        delay(50, function() end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_error_in_union_suppresses {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:368:nonstrict_mode_error_in_union_suppresses`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_error_in_union_suppresses

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_error_in_union_suppresses() {
    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local sublist: any
        if sublist then
            local subitem = sublist.item
            local _ = string.upper(subitem)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_for_in_iterator_variables_are_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:205:nonstrict_mode_for_in_iterator_variables_are_any`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_for_in_iterator_variables_are_any

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_for_in_iterator_variables_are_any() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function requires_a_table(arg: {}) end
        function requires_a_number(arg: number) end

        local T = {}
        for a, b in pairs(T) do
            requires_a_table(a)
            requires_a_table(b)
            requires_a_number(a)
            requires_a_number(b)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_function_parameters_are_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:78:nonstrict_mode_function_parameters_are_any`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_function_parameters_are_any

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_function_parameters_are_any() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function f(arg)
            arg = 9
            arg:concat(4)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_inconsistent_module_return_types_are_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:259:nonstrict_mode_inconsistent_module_return_types_are_ok`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_inconsistent_module_return_types_are_ok

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_inconsistent_module_return_types_are_ok() {
    use ulua_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        local FFlag: any

        if FFlag.get('SomeFlag') then
            return {foo='bar'}
        else
            return function(prop)
                return 'bar'
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let main_module = fixture.get_main_module(false);
    let return_type = unsafe { (*main_module).return_type };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("{ foo: string }", to_string_type_pack_id(return_type));
    } else {
      assert_eq!("any", to_string_type_pack_id(return_type));
    }
  }
}

mod nonstrict_mode_inconsistent_return_types_are_ok {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:91:nonstrict_mode_inconsistent_return_types_are_ok`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_inconsistent_return_types_are_ok

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_inconsistent_return_types_are_ok() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function f()
            if 1 then
                return 4
            else
                return 'hello'
            end
            return 'one', 'two'
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_infer_nullary_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:28:nonstrict_mode_infer_nullary_function`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item nonstrict_mode_infer_nullary_function

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_infer_nullary_function() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function foo(x, y) end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let foo_type = fixture.require_type_string(&String::from("foo"));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("(unknown, unknown) -> ()", to_string_type_id(foo_type));
    } else {
      assert_eq!("(any, any) -> (...any)", to_string_type_id(foo_type));
    }
  }
}

mod nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:45:nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function getMinCardCountForWidth(width)
            if width < 513 then
                return 3
            else
                return 8, 'jellybeans'
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let t = fixture.require_type_string(&String::from("getMinCardCountForWidth"));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("(number) -> number", to_string_type_id(t));
    } else {
      assert_eq!("(any) -> (...any)", to_string_type_id(t));
    }
  }
}

mod nonstrict_mode_inline_table_props_are_also_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:186:nonstrict_mode_inline_table_props_are_also_any`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_inline_table_props_are_also_any

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_inline_table_props_are_also_any() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local T = {
            one = 1,
            two = 'two',
            three = function() return 3 end
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let ty = fixture.require_type_string(&String::from("T"));
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ one: number, three: () -> number, two: string }",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    } else {
      assert_eq!(
        "{| one: any, three: () -> (...any), two: any |}",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    }
  }
}

mod nonstrict_mode_local_tables_are_not_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:137:nonstrict_mode_local_tables_are_not_any`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_local_tables_are_not_any

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_local_tables_are_not_any() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local T = {}
        function T:method() end
        function T.staticmethod() end

        T.method()
        T:staticmethod()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "This function does not take self. Did you mean to use a dot instead of a colon?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod nonstrict_mode_locals_are_any_by_default {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:108:nonstrict_mode_locals_are_any_by_default`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_locals_are_any_by_default

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_locals_are_any_by_default() {
    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local m = 55
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("m"));
    if !FFlag::DebugLuauForceOldSolver.get() {
      let mut opts = ToStringOptions::new(true);
      assert_eq!("number", to_string_type_id_to_string_options(ty, &mut opts));
    } else {
      assert_eq!("any", to_string_type_id(ty));
    }
  }
}

mod nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:336:nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict() {
    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true),
    ];

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
        math.abs("pls")
    "#,
      ),
      None,
    );

    assert_eq!(2, results.errors.len(), "{:?}", results.errors);
    assert!(matches!(
      results.errors[0].data,
      TypeErrorData::CheckedFunctionCallError(_)
    ));
    assert!(matches!(
      results.errors[1].data,
      TypeErrorData::ConstraintSolvingIncompleteError(_)
    ));
  }
}

mod nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:155:nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local T = {}
        function T:method(x: number) end
        T.method(5)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "This function must be called with self. Did you mean to use a colon instead of a dot?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod nonstrict_mode_parameters_having_type_any_are_optional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:123:nonstrict_mode_parameters_having_type_any_are_optional`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_parameters_having_type_any_are_optional

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_parameters_having_type_any_are_optional() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function f(a, b)
            return a
        end

        f(5)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_return_annotation_is_still_checked {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:67:nonstrict_mode_return_annotation_is_still_checked`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_return_annotation_is_still_checked

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_return_annotation_is_still_checked() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(x): number return 'hello' end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_ne!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod nonstrict_mode_returning_insufficient_return_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:284:nonstrict_mode_returning_insufficient_return_values`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_returning_insufficient_return_values

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_returning_insufficient_return_values() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function foo(): (boolean, string?)
            if true then
                return true, "hello"
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

mod nonstrict_mode_returning_too_many_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:301:nonstrict_mode_returning_too_many_values`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_returning_too_many_values

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_returning_too_many_values() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        function foo(): boolean
            if true then
                return true, "hello"
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

mod nonstrict_mode_standalone_constraint_solving_incomplete_is_hidden_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:318:nonstrict_mode_standalone_constraint_solving_incomplete_is_hidden_nonstrict`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item nonstrict_mode_standalone_constraint_solving_incomplete_is_hidden_nonstrict

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_standalone_constraint_solving_incomplete_is_hidden_nonstrict() {
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

    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
    "#,
      ),
      None,
    );

    assert_eq!(0, results.errors.len(), "{:?}", results.errors);
  }
}

mod nonstrict_mode_table_dot_insert_and_recursive_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:224:nonstrict_mode_table_dot_insert_and_recursive_calls`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item nonstrict_mode_table_dot_insert_and_recursive_calls

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_table_dot_insert_and_recursive_calls() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function populateListFromIds(list, normalizedData)
            local newList = {}

            for _, value in ipairs(list) do
                if type(value) == "table" then
                    table.insert(newList, populateListFromIds(value, normalizedData))
                else
                    table.insert(newList, normalizedData[value])
                end
            end

            return newList
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod nonstrict_mode_table_props_are_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:170:nonstrict_mode_table_props_are_any`
  //! Source: `tests/NonstrictMode.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/NonstrictMode.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/NonstrictMode.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item nonstrict_mode_table_props_are_any

  #[cfg(test)]
  #[test]
  fn nonstrict_mode_table_props_are_any() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local T = {}
        T.foo = 55
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let ty = fixture.require_type_string(&String::from("T"));
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ foo: number }",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    } else {
      assert_eq!(
        "{| foo: any |}",
        to_string_type_id_to_string_options(ty, &mut opts)
      );
    }
  }
}
