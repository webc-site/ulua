extern crate alloc;

mod type_infer_type_instantiations_anonymous_type_inferred {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:148:type_infer_type_instantiations_anonymous_type_inferred`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_anonymous_type_inferred

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_anonymous_type_inferred() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T, U>(): { a: T, b: U }
            return nil :: any
        end

        local correct: { a: number, b: string } = f<<number>>()
        local incorrect: { a: number, b: string } = f<<string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(7, result.errors[0].location.begin.line);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
      ));
    }
  }
}

mod type_infer_type_instantiations_as_expression_correct {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:19:type_infer_type_instantiations_as_expression_correct`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_as_expression_correct

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_as_expression_correct() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(): T
            return nil :: any
        end

        local correct = f<<number>>() + 5
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_type_instantiations_as_expression_incorrect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:38:type_infer_type_instantiations_as_expression_incorrect`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_as_expression_incorrect

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_as_expression_incorrect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(): T
            return nil :: any
        end

        local incorrect = f<<string>>() + 5
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Operator '+' could not be applied to operands of types string and number; there is no corresponding overload for __add",
          to_string_type_error(&result.errors[0])
        );
      } else {
        assert_eq!(
          "Expected this to be 'number', but got 'string'",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_as_stmt_correct {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:69:type_infer_type_instantiations_as_stmt_correct`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_as_stmt_correct

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_as_stmt_correct() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(a: T, b: T)
            return nil :: any
        end

        f<<number | string>>(1, "a")
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_type_instantiations_as_stmt_incorrect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:88:type_infer_type_instantiations_as_stmt_incorrect`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_as_stmt_incorrect

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_as_stmt_incorrect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(a: T, b: T)
            return nil :: any
        end

        f<<number | boolean>>(1, "a")
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'boolean | number', but got 'string';\n\
this is because\n\
\t * the 1st component of the union is `number`, and `string` is not a subtype of `number`\n\
\t * the 2nd component of the union is `boolean`, and `string` is not a subtype of `boolean`";
        let actual = to_string_type_error(&result.errors[0]);
        let expected_lines = expected.lines().map(str::trim).collect::<Vec<_>>();
        let actual_lines = actual.lines().map(str::trim).collect::<Vec<_>>();
        assert_eq!(expected_lines, actual_lines);
      } else {
        assert_eq!(
          "Expected this to be 'boolean | number', but got 'string'; none of the union options are compatible",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_dot_index_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:247:type_infer_type_instantiations_dot_index_call`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_dot_index_call

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_dot_index_call() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = {
            f = function<T>(): T
                return nil :: any
            end,
        }

        local correct: number = t.f<<number>>()
        local incorrect: number = t.f<<string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(9, result.errors[0].location.begin.line);
    }
  }
}

mod type_infer_type_instantiations_function_intersections {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:523:type_infer_type_instantiations_function_intersections`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record InstantiateGenericsOnNonFunction (Analysis/include/Luau/Error.h)
  //!   - calls -> record overloaded (Common/include/Luau/Variant.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_function_intersections

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_function_intersections() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local f: (<T>(T) -> T) & (<T>(T?) -> T) = nil :: any
        f<<number>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::InstantiateGenericsOnNonFunction(_)
      ));
      assert_eq!(3, result.errors[0].location.begin.line);
      assert_eq!(
        "Luau does not currently support explicitly instantiating an overloaded function type.",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_type_instantiations_incomplete_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:542:type_infer_type_instantiations_incomplete_type_packs`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_incomplete_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_incomplete_type_packs() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
            local f: <A, T...>() -> (A, T...) = nil :: any
            local correct: string, b: number, c: boolean = f<<string>>()
            local incorrect: number, b: number, c: boolean = f<<string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
      ));
      assert_eq!(3, result.errors[0].location.begin.line);
    }
  }
}

mod type_infer_type_instantiations_metatable_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:337:type_infer_type_instantiations_metatable_call`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record InstantiateGenericsOnNonFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_metatable_call

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_metatable_call() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = BuiltinsFixture::default();
      fixture.get_frontend();

      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = setmetatable({}, {
            __call = function<T>(self): T
                return nil :: any
            end,
        })

        t<<number>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::InstantiateGenericsOnNonFunction(_)
      ));
      assert_eq!(
        "Luau does not currently support explicitly instantiating a table with a `__call` metamethod. You may be able to work around this by creating a function that calls the table, and using that instead.",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_type_instantiations_method_call_incomplete {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:361:type_infer_type_instantiations_method_call_incomplete`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_method_call_incomplete

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_method_call_incomplete() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = {
            f = function<T, U>(self: any): T | U
                return nil :: any
            end,
        }

        local correct: number | string = t:f<<number>>()
        local incorrect: number | string = t:f<<boolean>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
      ));
      assert_eq!(9, result.errors[0].location.begin.line);
    }
  }
}

mod type_infer_type_instantiations_method_index_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:270:type_infer_type_instantiations_method_index_call`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_method_index_call

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_method_index_call() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = {
            f = function<T>(self: any): T
                return nil :: any
            end,
        }

        local correct: number = t:f<<number>>()
        local incorrect: number = t:f<<string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
      ));
      assert_eq!(9, result.errors[0].location.begin.line);
    }
  }
}

mod type_infer_type_instantiations_multiple_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:128:type_infer_type_instantiations_multiple_calls`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_multiple_calls

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_multiple_calls() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(): T
            return nil :: any
        end

        local a: number = f<<number>>()
        local b: string = f<<string>>()
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_type_instantiations_not_a_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:318:type_infer_type_instantiations_not_a_function`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record InstantiateGenericsOnNonFunction (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_not_a_function

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_not_a_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local oops = 3
        local stub = oops<<number>>
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::InstantiateGenericsOnNonFunction(_)
      ));
      assert_eq!(
        "Cannot instantiate type parameters on something without type parameters.",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_type_instantiations_replacing_generic_with_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:560:type_infer_type_instantiations_replacing_generic_with_generic`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_replacing_generic_with_generic

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_replacing_generic_with_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: <A, B>() -> (A, B) = nil :: any

        local function bar<T>()
            return foo<<T, number>>()
        end

        local baz, quxx = bar<<string>>()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("baz")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("quxx")))
    );
  }
}

mod type_infer_type_instantiations_stored_as_variable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:294:type_infer_type_instantiations_stored_as_variable`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_stored_as_variable

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_stored_as_variable() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>(): T
            return nil :: any
        end

        local fNumber = f<<number>>

        local correct: number = fNumber()
        local incorrect: string = fNumber()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
      ));
      assert_eq!(9, result.errors[0].location.begin.line);
    }
  }
}

mod type_infer_type_instantiations_too_many_provided {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:385:type_infer_type_instantiations_too_many_provided`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeInstantiationCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_too_many_provided

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_too_many_provided() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T>() end

        f<<number, string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeInstantiationCountMismatch(_)
      ));

      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Too many type parameters passed to 'f', which is typed as <T>(...any) -> (). Expected at most 1 type parameter, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      } else {
        assert_eq!(
          "Too many type parameters passed to 'f', which is typed as <T>() -> (). Expected at most 1 type parameter, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_too_many_provided_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:451:type_infer_type_instantiations_too_many_provided_method`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeInstantiationCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_too_many_provided_method

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_too_many_provided_method() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = {
            f = function<T>(self: any) end,
        }

        t:f<<number, string>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len());
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeInstantiationCountMismatch(_)
      ));
      assert_eq!(6, result.errors[0].location.begin.line);

      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Too many type parameters passed to function typed as <T>(any) -> (). Expected at most 1 type parameter, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      } else {
        assert_eq!(
          "Too many type parameters passed to 't.f', which is typed as <T>(any) -> (). Expected at most 1 type parameter, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_too_many_provided_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:418:type_infer_type_instantiations_too_many_provided_type_packs`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeInstantiationCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_too_many_provided_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_too_many_provided_type_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local function f<T...>(): (T...) end

        f<<(string, number), (true, false)>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeInstantiationCountMismatch(_)
      ));

      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Too many type parameters passed to 'f', which is typed as <T...>(...any) -> (T...). Expected at most 1 type pack, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      } else {
        assert_eq!(
          "Too many type parameters passed to 'f', which is typed as <T...>() -> (T...). Expected at most 1 type pack, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_too_many_type_packs_provided_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:487:type_infer_type_instantiations_too_many_type_packs_provided_method`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeInstantiationCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_too_many_type_packs_provided_method

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_too_many_type_packs_provided_method() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    for enabled in [true, false] {
      let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
      let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
      let mut fixture = Fixture::fixture_bool(false);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        --!strict
        local t = {
            f = function<T...>(self: any): (T...) end,
        }

        t:f<<(number, string), (true, false)>>()
        "#,
        ),
        None,
      );

      assert_eq!(1, result.errors.len());
      assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeInstantiationCountMismatch(_)
      ));
      assert_eq!(6, result.errors[0].location.begin.line);

      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "Too many type parameters passed to function typed as <T...>(any) -> (T...). Expected at most 1 type pack, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      } else {
        assert_eq!(
          "Too many type parameters passed to 't.f', which is typed as <T...>(any) -> (T...). Expected at most 1 type pack, but 2 provided.",
          to_string_type_error(&result.errors[0])
        );
      }
    }
  }
}

mod type_infer_type_instantiations_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:171:type_infer_type_instantiations_type_packs`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_type_packs() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    --!strict
    local function f<T..., U...>(...: T...): U... end

    local a: number, b: string = f<<(boolean, {}), (number, string)>>(true, {})
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_instantiations_type_packs_incorrect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:209:type_infer_type_instantiations_type_packs_incorrect`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_type_packs_incorrect

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_type_packs_incorrect() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    --!strict
    local function f<T..., U...>(...: T...): U... end

    local a: number, b: string = f<<(boolean, {}), (number, string)>>(true, "uh oh")
    "#,
      ),
      None,
    );

    assert!(
      result
        .errors
        .iter()
        .any(|error| matches!(error.data, TypeErrorData::TypeMismatch(_))),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_type_instantiations_type_packs_incorrect_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:227:type_infer_type_instantiations_type_packs_incorrect_method`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_type_packs_incorrect_method

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_type_packs_incorrect_method() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    --!strict
    local t: {
        f: <T..., U...>(self: any, T...) -> U...,
    } = nil :: any

    local a: number, b: string = t:f<<(boolean, {}), (number, string)>>(true, "uh oh")
    "#,
      ),
      None,
    );

    assert!(
      result
        .errors
        .iter()
        .any(|error| matches!(error.data, TypeErrorData::TypeMismatch(_))),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_type_instantiations_type_packs_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:189:type_infer_type_instantiations_type_packs_method`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_type_packs_method

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_type_packs_method() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    --!strict
    local t: {
        f: <T..., U...>(self: any, T...) -> U...,
    } = nil :: any

    local a: number, b: string = t:f<<(boolean, {}), (number, string)>>(true, {})
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:583:type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _visit_call_type_args = ScopedFastFlag::new(&FFlag::LuauVisitCallTypeArgsInDfg, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        function t:f<T, U>() end

        local x = 5
        globl = 42

        t:f<<typeof(x), string>>()
        t:f<<number, typeof(x)>>()
        t:f<<typeof(globl), unknown>>()
        t:f<<typeof(t.f), unknown>>()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::UnknownSymbol(_)
    ));
  }
}

mod type_infer_type_instantiations_typeof_local_in_type_pack_no_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:608:type_infer_type_instantiations_typeof_local_in_type_pack_no_crash`
  //! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_type_instantiations_typeof_local_in_type_pack_no_crash

  #[cfg(test)]
  #[test]
  fn type_infer_type_instantiations_typeof_local_in_type_pack_no_crash() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _visit_call_type_args = ScopedFastFlag::new(&FFlag::LuauVisitCallTypeArgsInDfg, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        function t:f<T...>() end

        local x = 5

        t:f<<(string, typeof(x))>>()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}
