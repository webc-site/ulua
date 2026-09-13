extern crate alloc;

mod type_infer_loops_any_type_in_for_loop_should_propagate {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1539:type_infer_loops_any_type_in_for_loop_should_propagate`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_loops_any_type_in_for_loop_should_propagate

  #[cfg(test)]
  #[test]
  fn type_infer_loops_any_type_in_for_loop_should_propagate() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function my_iter(): any
            return {}
        end

        for index: number, value: string in my_iter() do
            print(index, value)
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
        column: 18
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 25
      }))
    );
  }
}

mod type_infer_loops_cli_68448_iterators_need_not_accept_nil {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:966:type_infer_loops_cli_68448_iterators_need_not_accept_nil`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_loops_cli_68448_iterators_need_not_accept_nil

  #[cfg(test)]
  #[test]
  fn type_infer_loops_cli_68448_iterators_need_not_accept_nil() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
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
        local function makeEnum(members)
            local enum = {}
            for _, memberName in ipairs(members) do
                enum[memberName] = memberName
            end
            return enum
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a>({a}) -> { [a]: a }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("makeEnum")),
        &mut ToStringOptions::new(true),
      )
    );
  }
}

mod type_infer_loops_correctly_scope_locals_while {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:601:type_infer_loops_correctly_scope_locals_while`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_loops_correctly_scope_locals_while

  #[cfg(test)]
  #[test]
  fn type_infer_loops_correctly_scope_locals_while() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        while true do
            local a = 1
        end

        print(a) -- oops!
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

mod type_infer_loops_dcr_iteration_explore_raycast_minimization {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1005:type_infer_loops_dcr_iteration_explore_raycast_minimization`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_explore_raycast_minimization

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_explore_raycast_minimization() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local testResults = {}
        for _, testData in pairs(testResults) do
        end

        table.insert(testResults, {})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_dcr_iteration_fragmented_keys {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1061:type_infer_loops_dcr_iteration_fragmented_keys`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_fragmented_keys

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_fragmented_keys() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function isIndexKey(k, contiguousLength)
            return true
        end

        local function getTableLength(tbl)
            local length = 1
            local value = rawget(tbl, length)
            while value ~= nil do
                length += 1
                value = rawget(tbl, length)
            end
            return length - 1
        end

        local function rawpairs(t)
            return next, t, nil
        end

        local function getFragmentedKeys(tbl)
            local keys = {}
            local keysLength = 0
            local tableLength = getTableLength(tbl)
            for key, _ in rawpairs(tbl) do
                if not isIndexKey(key, tableLength) then
                    keysLength = keysLength + 1
                    keys[keysLength] = key
                end
            end
            return keys, keysLength, tableLength
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_dcr_iteration_minimized_fragmented_keys_1 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1018:type_infer_loops_dcr_iteration_minimized_fragmented_keys_1`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_minimized_fragmented_keys_1

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_1() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function rawpairs(t)
            return next, t, nil
        end

        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in rawpairs(tbl) do
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_dcr_iteration_minimized_fragmented_keys_2 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1035:type_infer_loops_dcr_iteration_minimized_fragmented_keys_2`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_minimized_fragmented_keys_2

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in next, tbl, nil do
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_dcr_iteration_minimized_fragmented_keys_3 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1048:type_infer_loops_dcr_iteration_minimized_fragmented_keys_3`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_minimized_fragmented_keys_3

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_3() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in pairs(tbl) do
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_dcr_iteration_on_never_gives_never {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1120:type_infer_loops_dcr_iteration_on_never_gives_never`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_dcr_iteration_on_never_gives_never

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_iteration_on_never_gives_never() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local iter: never
        local ans
        for xs in iter do
            ans = xs
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "nil"
    } else {
      "never"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("ans")))
    );
  }
}

mod type_infer_loops_dcr_xpath_candidates {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1099:type_infer_loops_dcr_xpath_candidates`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_loops_dcr_xpath_candidates

  #[cfg(test)]
  #[test]
  fn type_infer_loops_dcr_xpath_candidates() {
    use alloc::string::String;

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
        type Instance = {}
        local function findCandidates(instances: { Instance },  path: { string })
            for _, name in ipairs(path) do
            end
            return {}
        end

        local canditates = findCandidates({}, {})
        for _, canditate in ipairs(canditates) do end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_ensure_local_in_loop_does_not_escape {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1511:type_infer_loops_ensure_local_in_loop_does_not_escape`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_ensure_local_in_loop_does_not_escape

  #[cfg(test)]
  #[test]
  fn type_infer_loops_ensure_local_in_loop_does_not_escape() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = 42
        repeat
            local x = ""
        until true
        -- The local inside the loop should have no effect on the local
        -- outside the loop.
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_explicit_types_in_for_loop_should_propagate {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1560:type_infer_loops_explicit_types_in_for_loop_should_propagate`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_loops_explicit_types_in_for_loop_should_propagate

  #[cfg(test)]
  #[test]
  fn type_infer_loops_explicit_types_in_for_loop_should_propagate() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function my_iter(): {[number]: string}
            return {}
        end

        for index: number, value: string in my_iter() do
            print(index, value)
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
        column: 18
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 25
      }))
    );
  }
}

mod type_infer_loops_for_in_loop {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:130:type_infer_loops_for_in_loop`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n
        local s
        for i, v in pairs({ "foo" }) do
            n = i
            s = v
            print(i, v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 18
        }))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 21
        }))
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
    }
  }
}

mod type_infer_loops_for_in_loop_annotations_apply_inside_lambdas {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1629:type_infer_loops_for_in_loop_annotations_apply_inside_lambdas`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_annotations_apply_inside_lambdas

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_annotations_apply_inside_lambdas() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function my_iter(): any
            return {}
        end

        for index: number in my_iter() do
            local fn = function()
                index = ""
            end
            fn()
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
  }
}

mod type_infer_loops_for_in_loop_annotations_apply_to_function_expressions {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1604:type_infer_loops_for_in_loop_annotations_apply_to_function_expressions`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_annotations_apply_to_function_expressions

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_annotations_apply_to_function_expressions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function my_iter(): any
            return {}
        end

        local function takesString(s: string) end

        for index: number in my_iter() do
            takesString(index)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err.wanted_type));
    assert_eq!("number", to_string_type_id(err.given_type));
  }
}

mod type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:333:type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values() {
    use alloc::string::String;

    use ulua_analysis::records::{
      count_mismatch::{CountMismatch, CountMismatchContext},
      type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function hasDivisors(value: number, table)
            return false
        end

        function prime_iter(state, index)
            while hasDivisors(index, state) do
                index += 1
            end

            state[index] = true
            return index
        end

        function primes1()
            return prime_iter, {}
        end

        function primes2()
            return prime_iter, {}, ""
        end

        function primes3()
            return prime_iter, {}, 2
        end

        for p in primes1() do print(p) end -- mismatch in argument count

        for p in primes2() do print(p) end -- mismatch in argument types, prime_iter takes {}, number, we are given {}, string

        for p in primes3() do print(p) end -- no error
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let acm =
      type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatchContext::Arg, acm.context());
    assert_eq!(2, acm.expected());
    assert_eq!(1, acm.actual());

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!(fixture.base.get_builtins().number_type, tm.wanted_type);
    assert_eq!(fixture.base.get_builtins().string_type, tm.given_type);
  }
}

mod type_infer_loops_for_in_loop_error_on_iterator_requiring_args_but_none_given {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:385:type_infer_loops_for_in_loop_error_on_iterator_requiring_args_but_none_given`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_error_on_iterator_requiring_args_but_none_given

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_error_on_iterator_requiring_args_but_none_given() {
    use alloc::string::String;

    use ulua_analysis::records::count_mismatch::{CountMismatch, CountMismatchContext};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function prime_iter(state, index)
            return 1
        end

        for p in prime_iter do print(p) end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let acm =
      type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatchContext::Arg, acm.context());
    assert_eq!(2, acm.expected());
    assert_eq!(0, acm.actual());
  }
}

mod type_infer_loops_for_in_loop_on_error {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:295:type_infer_loops_for_in_loop_on_error`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_on_error

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_on_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            gobble.prop = x.otherprop
        end

        local p
        for _, part in i_am_not_defined do
            p = part
            f(part)
            part.thirdprop = false
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "*error-type*?"
    } else {
      "*error-type*"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("p")))
    );
  }
}

mod type_infer_loops_for_in_loop_on_non_function {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:319:type_infer_loops_for_in_loop_on_non_function`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_on_non_function

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_on_non_function() {
    use alloc::string::String;

    use ulua_analysis::records::cannot_call_non_function::CannotCallNonFunction;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local bad_iter = 5

        for a in bad_iter() do
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
      .expect("expected CannotCallNonFunction");
  }
}

mod type_infer_loops_for_in_loop_should_fail_with_non_function_iterator {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:226:type_infer_loops_for_in_loop_should_fail_with_non_function_iterator`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_should_fail_with_non_function_iterator

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_should_fail_with_non_function_iterator() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = "bar"
        for i, v in foo do
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot call a value of type string",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_loops_for_in_loop_where_iteratee_is_free {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:646:type_infer_loops_for_in_loop_where_iteratee_is_free`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_where_iteratee_is_free

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_where_iteratee_is_free() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function _:_(...)
        end

        repeat
            if _ then
            else
                _ = ...
            end
        until _

        for _ in _() do
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_loops_for_in_loop_with_custom_iterator {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:443:type_infer_loops_for_in_loop_with_custom_iterator`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_custom_iterator

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_with_custom_iterator() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function primes()
            return function (state: number) end,  2
        end

        for p, q in primes do
            q = ""
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
    assert_eq!(fixture.get_builtins().string_type, tm.given_type);
  }
}

mod type_infer_loops_for_in_loop_with_incompatible_args_to_iterator {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:407:type_infer_loops_for_in_loop_with_incompatible_args_to_iterator`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_incompatible_args_to_iterator

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_with_incompatible_args_to_iterator() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function my_iter(state: string, index: number)
            return state, index
        end

        local my_state = {}
        local first_index = "first"

        -- Type errors here.  my_state and first_index cannot be passed to my_iter
        for a, b in my_iter, my_state, first_index do
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      Location {
        begin: Position {
          line: 9,
          column: 20,
        },
        end: Position {
          line: 9,
          column: 27,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_loops_for_in_loop_with_next {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:158:type_infer_loops_for_in_loop_with_next`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_next

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_with_next() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n
        local s
        for i, v in next, { "foo" } do
            n = i
            s = v
            print(i, v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 18
        }))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 21
        }))
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
    }
  }
}

mod type_infer_loops_for_in_loop_with_next_and_multiple_elements {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:185:type_infer_loops_for_in_loop_with_next_and_multiple_elements`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_next_and_multiple_elements

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_with_next_and_multiple_elements() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local n
        local s
        for i, v in next, { "foo", "bar" } do
            n = i
            s = v
            print(i, v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 18
        }))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 6,
          column: 21
        }))
      );
    } else {
      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
      );
      assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
      );
    }
  }
}

mod type_infer_loops_for_in_loop_with_zero_iterators_dcr {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:258:type_infer_loops_for_in_loop_with_zero_iterators_dcr`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_zero_iterators_dcr

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_loop_with_zero_iterators_dcr() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function no_iter() end
        for key in no_iter() do end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
      "for..in loops require at least one value to iterate over.  Got zero",
      err.message()
    );
  }
}

mod type_infer_loops_for_in_require {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1334:type_infer_loops_for_in_require`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_require

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_require() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        for _ in require do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_in_surprising_iterator {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1321:type_infer_loops_for_in_surprising_iterator`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_surprising_iterator

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_surprising_iterator() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function broken(): (...() -> ())
    return function() end, function() end
end

for p in broken() do print(p) end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_in_with_a_custom_iterator_should_type_check {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:273:type_infer_loops_for_in_with_a_custom_iterator_should_type_check`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_in_with_a_custom_iterator_should_type_check

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_with_a_custom_iterator_should_type_check() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function range(l, h): () -> number
            return function()
                return l
            end
        end

        for n: string in range(1, 10) do
            print(n)
        end
    "#,
      ),
      None,
    );

    if FFlag::LuauPropagateTypeAnnotationsInForInLoops.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_loops_for_in_with_an_iterator_of_type_any {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:213:type_infer_loops_for_in_with_an_iterator_of_type_any`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_with_an_iterator_of_type_any

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_with_an_iterator_of_type_any() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local it: any
        local a, b
        for i, v in it do
            a, b = i, v
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_in_with_generic_next {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:778:type_infer_loops_for_in_with_generic_next`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_with_generic_next

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_with_generic_next() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        for k: number, v: number in next, {1, 2, 3} do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_in_with_just_one_iterator_is_ok {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:239:type_infer_loops_for_in_with_just_one_iterator_is_ok`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_in_with_just_one_iterator_is_ok

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_in_with_just_one_iterator_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function keys(dictionary)
            local new = {}
            local index = 1

            for key in pairs(dictionary) do
                new[index] = key
                index = index + 1
            end

            return new
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_loop {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:24:type_infer_loops_for_loop`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_loop

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_loop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local q
        for i=0, 50, 2 do
            q = i
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "number?"
    } else {
      "number"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("q")))
    );
  }
}

mod type_infer_loops_for_loop_lower_bound_is_string {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:938:type_infer_loops_for_loop_lower_bound_is_string`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_loop_lower_bound_is_string

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_loop_lower_bound_is_string() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        for i: unknown = 1, 10 do end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_for_loop_lower_bound_is_string_2 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:947:type_infer_loops_for_loop_lower_bound_is_string_2`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_for_loop_lower_bound_is_string_2

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_loop_lower_bound_is_string_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        for i: never = 1, 10 do end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be unreachable, but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_loops_for_loop_lower_bound_is_string_3 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:957:type_infer_loops_for_loop_lower_bound_is_string_3`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_for_loop_lower_bound_is_string_3

  #[cfg(test)]
  #[test]
  fn type_infer_loops_for_loop_lower_bound_is_string_3() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        for i: number | string = 1, 10 do end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_forin_metatable_iter_mm {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1273:type_infer_loops_forin_metatable_iter_mm`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_forin_metatable_iter_mm

  #[cfg(test)]
  #[test]
  fn type_infer_loops_forin_metatable_iter_mm() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Iterable<T...> = typeof(setmetatable({}, {} :: {
            __iter: (Iterable<T...>) -> () -> T...
        }))

        for i, v in {} :: Iterable<...number> do
            print(i, v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 18
      }))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 21
      }))
    );
  }
}

mod type_infer_loops_forin_metatable_no_iter_mm {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1255:type_infer_loops_forin_metatable_no_iter_mm`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_forin_metatable_no_iter_mm

  #[cfg(test)]
  #[test]
  fn type_infer_loops_forin_metatable_no_iter_mm() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = setmetatable({1, 2, 3}, {})

        for i, v in t do
            print(i, v)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 4,
        column: 18
      }))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 4,
        column: 21
      }))
    );
  }
}

mod type_infer_loops_fuzz_fail_missing_instantitation_follow {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:765:type_infer_loops_fuzz_fail_missing_instantitation_follow`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_fuzz_fail_missing_instantitation_follow

  #[cfg(test)]
  #[test]
  fn type_infer_loops_fuzz_fail_missing_instantitation_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        function _(l0:number)
        return _
        end
        for _ in _(8) do
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_loops_incorrect_type_annotation_types_in_loop_should_propagate_with_errors {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1581:type_infer_loops_incorrect_type_annotation_types_in_loop_should_propagate_with_errors`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_incorrect_type_annotation_types_in_loop_should_propagate_with_errors

  #[cfg(test)]
  #[test]
  fn type_infer_loops_incorrect_type_annotation_types_in_loop_should_propagate_with_errors() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::LuauPropagateTypeAnnotationsInForInLoops, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        function my_iter(): any
            return {}
        end
        for index: number, value: string in my_iter() do
            index = ""
            print(index)
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
  }
}

mod type_infer_loops_ipairs_produces_integral_indices {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:631:type_infer_loops_ipairs_produces_integral_indices`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_ipairs_produces_integral_indices

  #[cfg(test)]
  #[test]
  fn type_infer_loops_ipairs_produces_integral_indices() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local key
        for i, e in ipairs({}) do key = i end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "number?"
    } else {
      "number"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("key")))
    );
  }
}

mod type_infer_loops_iter_constraint_before_loop_body {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:545:type_infer_loops_iter_constraint_before_loop_body`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_iter_constraint_before_loop_body

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iter_constraint_before_loop_body() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {
    	    fields = {},
        }

        function f()
            for u, v in pairs(T.fields) do
                T.fields[u] = nil
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_iter_mm_results_are_lvalue {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1239:type_infer_loops_iter_mm_results_are_lvalue`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_iter_mm_results_are_lvalue

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iter_mm_results_are_lvalue() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = setmetatable({}, {
            __iter = function()
                return pairs({1, 2, 3})
            end,
        })

        for k, v in foo do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_iterate_array_of_singletons {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1220:type_infer_loops_iterate_array_of_singletons`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_iterate_array_of_singletons

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iterate_array_of_singletons() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Direction = "Left" | "Right" | "Up" | "Down"
        local Instructions: { Direction } = { "Left", "Down" }

        for _, step in Instructions do
            local dir: Direction = step
            print(dir)
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

mod type_infer_loops_iterate_over_free_table {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:988:type_infer_loops_iterate_over_free_table`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_iterate_over_free_table

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iterate_over_free_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function print(x) end

        function dump(tbl)
            print(tbl.whatever)
            for k, v in tbl do
                print(k)
                print(v)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_iterate_over_properties {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1141:type_infer_loops_iterate_over_properties`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_iterate_over_properties

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iterate_over_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f()
            local t = { p = 5, q = "hello" }
            for k, v in t do
                return k, v
            end

            error("")
        end

        local k, v = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("k")))
    );
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("v")))
    );
  }
}

mod type_infer_loops_iterate_over_properties_nonstrict {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1165:type_infer_loops_iterate_over_properties_nonstrict`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_iterate_over_properties_nonstrict

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iterate_over_properties_nonstrict() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local function f()
            local t = { p = 5, q = "hello" }
            for k, v in t do
                return k, v
            end

            error("")
        end

        local k, v = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_iteration_no_table_passed {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:45:type_infer_loops_iteration_no_table_passed`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_loops_iteration_no_table_passed

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iteration_no_table_passed() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"

type Iterable = typeof(setmetatable(
    {},
    {}::{
        __iter: (self: Iterable) -> (any, number) -> (number, string)
    }
))

local t: Iterable

for a, b in t do end
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
      "__iter metamethod must return (next[, table[, state]])",
      ge.message()
    );
  }
}

mod type_infer_loops_iteration_preserves_error_suppression {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1293:type_infer_loops_iteration_preserves_error_suppression`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_loops_iteration_preserves_error_suppression

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iteration_preserves_error_suppression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _v1 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function first(x: any)
            for k, v in pairs(x) do
                print(k, v)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "*error-type* | ~nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 3,
        column: 22
      }))
    );
    assert_eq!(
      "any",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 3,
        column: 25
      }))
    );
  }
}

mod type_infer_loops_iteration_regression_issue_69967 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:73:type_infer_loops_iteration_regression_issue_69967`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_iteration_regression_issue_69967

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iteration_regression_issue_69967() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Iterable = typeof(setmetatable(
            {},
            {}::{
                __iter: (self: Iterable) -> () -> (number, string)
            }
        ))

        local t: Iterable

        for a, b in t do end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_iteration_regression_issue_69967_alt {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:94:type_infer_loops_iteration_regression_issue_69967_alt`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_iteration_regression_issue_69967_alt

  #[cfg(test)]
  #[test]
  fn type_infer_loops_iteration_regression_issue_69967_alt() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Iterable = typeof(setmetatable(
            {},
            {}::{
                __iter: (self: Iterable) -> () -> (number, string)
            }
        ))

        local t: Iterable
        local x, y

        for a, b in t do
            x = a
            y = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let (expected_x, expected_y) = if !FFlag::DebugLuauForceOldSolver.get() {
      ("number?", "string?")
    } else {
      ("number", "string")
    };
    assert_eq!(
      expected_x,
      to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
      expected_y,
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_loop_iter_basic {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:788:type_infer_loops_loop_iter_basic`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_basic

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_basic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {string} = {}
        local key
        for k: number in t do
        end
        for k: number, v: string in t do
        end
        for k, v in t do
            key = k
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "number?"
    } else {
      "number"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("key")))
    );
  }
}

mod type_infer_loops_loop_iter_metamethod_nil {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:857:type_infer_loops_loop_iter_metamethod_nil`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_metamethod_nil

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_metamethod_nil() {
    // Upstream keeps this case disabled under #if 0.
  }
}

mod type_infer_loops_loop_iter_metamethod_not_enough_returns {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:874:type_infer_loops_loop_iter_metamethod_not_enough_returns`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_metamethod_not_enough_returns

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_metamethod_not_enough_returns() {
    // Upstream keeps this case disabled under #if 0.
  }
}

mod type_infer_loops_loop_iter_metamethod_ok {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:896:type_infer_loops_loop_iter_metamethod_ok`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_metamethod_ok

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_metamethod_ok() {
    // Upstream keeps this case disabled under #if 0.
  }
}

mod type_infer_loops_loop_iter_metamethod_ok_with_inference {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:914:type_infer_loops_loop_iter_metamethod_ok_with_inference`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_metamethod_ok_with_inference

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_metamethod_ok_with_inference() {
    // Upstream keeps this case disabled under #if 0.
  }
}

mod type_infer_loops_loop_iter_no_indexer_nonstrict {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:846:type_infer_loops_loop_iter_no_indexer_nonstrict`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_no_indexer_nonstrict

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_no_indexer_nonstrict() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
        local t = {}
        for k, v in t do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_loop_iter_no_indexer_strict {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:832:type_infer_loops_loop_iter_no_indexer_strict`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_no_indexer_strict

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_no_indexer_strict() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        for k, v in t do
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_loop_iter_trailing_nil {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:815:type_infer_loops_loop_iter_trailing_nil`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_loop_iter_trailing_nil

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_iter_trailing_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {string} = {}
        local extra
        for k, v, e in t do
            extra = e
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_string(&String::from("extra")))
    );
  }
}

mod type_infer_loops_loop_typecheck_crash_on_empty_optional {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:748:type_infer_loops_loop_typecheck_crash_on_empty_optional`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_loops_loop_typecheck_crash_on_empty_optional

  #[cfg(test)]
  #[test]
  fn type_infer_loops_loop_typecheck_crash_on_empty_optional() {
    use alloc::string::String;

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
        local t = {}
        for _ in t do
            for _ in assert(missing()) do
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_lti_fuzzer_uninitialized_loop_crash {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1209:type_infer_loops_lti_fuzzer_uninitialized_loop_crash`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_lti_fuzzer_uninitialized_loop_crash

  #[cfg(test)]
  #[test]
  fn type_infer_loops_lti_fuzzer_uninitialized_loop_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        for l0=_,_ do
            return _()
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_oss_1413 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1359:type_infer_loops_oss_1413`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_oss_1413

  #[cfg(test)]
  #[test]
  fn type_infer_loops_oss_1413() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function KahanSum(values: {number}): number
            local sum: number = 0
            local compensator: number = 0
            for _, value in values do
                local y = value - compensator
                local t = sum + y
                compensator = (t - sum) - y
                sum = t
            end
            return sum
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function HistogramString(values: {number})
            local histogram = {}
            values = table.clone(values)
            table.sort(values)

            local count = #values
            local range = (count - 1)

            local digitIndex = range // 2 + 1
            while digitIndex < count and values[digitIndex] == 0 do
                digitIndex = count - ((count - digitIndex) // 2)
            end
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function fun1()
            local foo = 1
            local bar = foo - foo + foo
            while false do
                foo = bar
            end
        end
        local function fun2()
            local foo = 1
            while false do
                local bar = foo - foo + foo
                foo = bar
            end
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_oss_1480 {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1344:type_infer_loops_oss_1480`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_loops_oss_1480

  #[cfg(test)]
  #[test]
  fn type_infer_loops_oss_1480() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Part = { Parent: Part? }
        type Instance = Part

        local part = {} :: Part

        local currentParent: Instance? = part.Parent
        while currentParent ~= nil do
            currentParent = currentParent.Parent
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_oss_1851_union_of_many_strings {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1526:type_infer_loops_oss_1851_union_of_many_strings`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_loops_oss_1851_union_of_many_strings

  #[cfg(test)]
  #[test]
  fn type_infer_loops_oss_1851_union_of_many_strings() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type union = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

local example: { [union]: number } = {}

for key in example do
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_pairs_should_not_retroactively_add_an_indexer {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1184:type_infer_loops_pairs_should_not_retroactively_add_an_indexer`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_loops_pairs_should_not_retroactively_add_an_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_loops_pairs_should_not_retroactively_add_an_indexer() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local prices = {
            hat = 1,
            bat = 2,
        }
        print(prices.wwwww)
        for _, _ in pairs(prices) do
        end
        print(prices.wwwww)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_loops_properly_infer_iteratee_is_a_free_table {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:580:type_infer_loops_properly_infer_iteratee_is_a_free_table`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_loops_properly_infer_iteratee_is_a_free_table

  #[cfg(test)]
  #[test]
  fn type_infer_loops_properly_infer_iteratee_is_a_free_table() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
    use ulua_ast::records::{location::Location, position::Position};
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
        for iter in pairs({}) do
            iter:g().p = true
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
      assert_eq!(
        Location {
          begin: Position {
            line: 2,
            column: 12,
          },
          end: Position {
            line: 2,
            column: 18,
          },
        },
        result.errors[0].location
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_loops_rbxl_place_file_crash_for_wrong_constraints {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:562:type_infer_loops_rbxl_place_file_crash_for_wrong_constraints`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_rbxl_place_file_crash_for_wrong_constraints

  #[cfg(test)]
  #[test]
  fn type_infer_loops_rbxl_place_file_crash_for_wrong_constraints() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local VehicleParameters = {
    -- These are default values in the case the package structure is broken
	StrutSpringStiffnessFront = 28000,
}

local function updateFromConfiguration()
	for property, value in pairs(VehicleParameters) do
        VehicleParameters[property] = value
	end
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_repeat_is_linearish {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1490:type_infer_loops_repeat_is_linearish`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_repeat_is_linearish

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_is_linearish() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        if math.random () > 0.5 then
            x = ""
            repeat
                error("spooky scary error")
            until true
        end
        -- The repeat in the above branch unconditionally fires the error, so
        -- this should _always_ be `nil`
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_repeat_loop {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:480:type_infer_loops_repeat_loop`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_repeat_loop

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_loop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local i
        repeat
            i = 'hi'
        until true
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "string?"
    } else {
      "string"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("i")))
    );
  }
}

mod type_infer_loops_repeat_loop_assignment {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1449:type_infer_loops_repeat_loop_assignment`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_repeat_loop_assignment

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_loop_assignment() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        repeat
            x = 42
        until math.random() > 0.5
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_repeat_loop_assignment_with_break {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1462:type_infer_loops_repeat_loop_assignment_with_break`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_repeat_loop_assignment_with_break

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_loop_assignment_with_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        repeat
            x = 42
        until math.random() > 0.5
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_repeat_loop_condition_binds_to_its_block {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:497:type_infer_loops_repeat_loop_condition_binds_to_its_block`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_repeat_loop_condition_binds_to_its_block

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_loop_condition_binds_to_its_block() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        repeat
            local x = true
        until x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_repeat_unconditionally_fires_error {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1475:type_infer_loops_repeat_unconditionally_fires_error`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_loops_repeat_unconditionally_fires_error

  #[cfg(test)]
  #[test]
  fn type_infer_loops_repeat_unconditionally_fires_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        repeat
            x = 42
        until true
        -- `x` should unconditionally be `number` here as the assignment
        -- above will _always_ run.
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_loops_symbols_in_repeat_block_should_not_be_visible_beyond_until_condition {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:508:type_infer_loops_symbols_in_repeat_block_should_not_be_visible_beyond_until_condition`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_loops_symbols_in_repeat_block_should_not_be_visible_beyond_until_condition

  #[cfg(test)]
  #[test]
  fn type_infer_loops_symbols_in_repeat_block_should_not_be_visible_beyond_until_condition() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        repeat
            local x = true
        until x

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_trivial_ipairs_usage {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:618:type_infer_loops_trivial_ipairs_usage`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_trivial_ipairs_usage

  #[cfg(test)]
  #[test]
  fn type_infer_loops_trivial_ipairs_usage() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local next, t, s = ipairs({1, 2, 3})
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({number}, number) -> (number?, number)",
      to_string_type_id(fixture.base.require_type_string(&String::from("next")))
    );
    assert_eq!(
      "{number}",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
  }
}

mod type_infer_loops_try_dispatch_iterable_function_under_constrained_loop_should_not_assert {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1311:type_infer_loops_try_dispatch_iterable_function_under_constrained_loop_should_not_assert`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_try_dispatch_iterable_function_under_constrained_loop_should_not_assert

  #[cfg(test)]
  #[test]
  fn type_infer_loops_try_dispatch_iterable_function_under_constrained_loop_should_not_assert() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function foo(Instance)
    for _, Child in next, Instance:GetChildren() do
    end
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_loops_unreachable_code_after_infinite_loop {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:666:type_infer_loops_unreachable_code_after_infinite_loop`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_unreachable_code_after_infinite_loop

  #[cfg(test)]
  #[test]
  fn type_infer_loops_unreachable_code_after_infinite_loop() {
    use alloc::string::String;

    use ulua_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            function unreachablecodepath(a): number
                while true do
                    if a then return 10 end
                end
                -- unreachable
            end
            unreachablecodepath(4)
        "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            function reachablecodepath(a): number
                while true do
                    if a then break end
                    return 10
                end

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
      .expect("expected FunctionExitsWithoutReturning");

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            function unreachablecodepath(a): number
                repeat
                    if a then return 10 end
                until false

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            function reachablecodepath(a, b): number
                repeat
                    if a then break end

                    if b then return 10 end
                until false

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
      ),
      None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
      .expect("expected FunctionExitsWithoutReturning");

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
            function unreachablecodepath(a: number?): number
                repeat
                    return 10
                until a ~= nil

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_loops_varlist_declared_by_for_in_loop_should_be_free {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:521:type_infer_loops_varlist_declared_by_for_in_loop_should_be_free`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_loops_varlist_declared_by_for_in_loop_should_be_free

  #[cfg(test)]
  #[test]
  fn type_infer_loops_varlist_declared_by_for_in_loop_should_be_free() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
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
        local T = {}

        function T.f(p)
            for i, v in pairs(p) do
                T.f(v)
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_loops_while_loop {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:463:type_infer_loops_while_loop`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_loops_while_loop

  #[cfg(test)]
  #[test]
  fn type_infer_loops_while_loop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local i
        while true do
            i = 8
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "number?"
    } else {
      "number"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("i")))
    );
  }
}

mod type_infer_loops_while_loop_assign_different_type {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1428:type_infer_loops_while_loop_assign_different_type`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_loops_while_loop_assign_different_type

  #[cfg(test)]
  #[test]
  fn type_infer_loops_while_loop_assign_different_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function takesString(_: string) end
        local function takesNil(_: nil) end
        local function foo()
            local x = ""
            takesString(x)
            while math.random () > 0.5 do
                x = nil
                takesNil(x)
            end
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> string?",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_loops_while_loop_error_in_body {
  //! Ported from `tests/TypeInfer.loops.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1409:type_infer_loops_while_loop_error_in_body`
  //! Source: `tests/TypeInfer.loops.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.loops.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.loops.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_loops_while_loop_error_in_body

  #[cfg(test)]
  #[test]
  fn type_infer_loops_while_loop_error_in_body() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo()
            local x = ""
            while math.random () > 0.5 do
                x = nil
                error("why did you make x nil tho")
            end
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> string",
      to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
  }
}
