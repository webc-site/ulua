extern crate alloc;

mod ast_query_ac_ast_ancestry_at_number_const {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:240:ast_query_ac_ast_ancestry_at_number_const`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_ac_ast_ancestry_at_number_const

  #[cfg(test)]
  #[test]
  fn ast_query_ac_ast_ancestry_at_number_const() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
print(3.)
    "#,
      ),
      None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
      find_ancestry_at_position_for_autocomplete(&*source_module, Position { line: 1, column: 8 })
    };

    assert!(ancestry.len() >= 2);
    assert!(unsafe { !ast_node_as::<AstExprConstantNumber>(*ancestry.last().unwrap()).is_null() });
  }
}

mod ast_query_ac_ast_ancestry_in_workspace_colon {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:262:ast_query_ac_ast_ancestry_in_workspace_colon`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_ac_ast_ancestry_in_workspace_colon

  #[cfg(test)]
  #[test]
  fn ast_query_ac_ast_ancestry_in_workspace_colon() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
print(workspace:)
    "#,
      ),
      None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
      find_ancestry_at_position_for_autocomplete(
        &*source_module,
        Position {
          line: 1,
          column: 16,
        },
      )
    };

    assert!(ancestry.len() >= 2);
    assert!(unsafe { !ast_node_as::<AstExprIndexName>(*ancestry.last().unwrap()).is_null() });
  }
}

mod ast_query_ac_ast_ancestry_in_workspace_dot {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:251:ast_query_ac_ast_ancestry_in_workspace_dot`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_ac_ast_ancestry_in_workspace_dot

  #[cfg(test)]
  #[test]
  fn ast_query_ac_ast_ancestry_in_workspace_dot() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
print(workspace.)
    "#,
      ),
      None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
      find_ancestry_at_position_for_autocomplete(
        &*source_module,
        Position {
          line: 1,
          column: 16,
        },
      )
    };

    assert!(ancestry.len() >= 2);
    assert!(unsafe { !ast_node_as::<AstExprIndexName>(*ancestry.last().unwrap()).is_null() });
  }
}

mod ast_query_ast_ancestry_at_eof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:227:ast_query_ast_ancestry_at_eof`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_ast_ancestry_at_eof

  #[cfg(test)]
  #[test]
  fn ast_query_ast_ancestry_at_eof() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
if true then
    "#,
      ),
      None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
      find_ast_ancestry_of_position(&*source_module, Position { line: 2, column: 4 }, false)
    };

    assert!(ancestry.len() >= 2);
    let parent_stat = ancestry[ancestry.len() - 2];
    assert!(unsafe { !ast_node_as::<AstStatIf>(parent_stat).is_null() });
  }
}

mod ast_query_binding {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:26:ast_query_binding`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item ast_query_binding

  #[cfg(test)]
  #[test]
  fn ast_query_binding() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    let global = fixture.get_doc_symbol(
      r#"
        local a = string.sub()
    "#,
      Position {
        line: 1,
        column: 21,
      },
    );

    assert_eq!(global, Some(String::from("@luau/global/string")));
  }
}

mod ast_query_class_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:83:ast_query_class_method`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_class_method

  #[cfg(test)]
  #[test]
  fn ast_query_class_method() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            function bar(self, x: string): number
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        local x: Foo = Foo.new()
        x:bar("asdf")
    "#,
      Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(symbol, Some(String::from("@test/globaltype/Foo.bar")));
  }
}

mod ast_query_event_callback_arg {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:50:ast_query_event_callback_arg`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_event_callback_arg

  #[cfg(test)]
  #[test]
  fn ast_query_event_callback_arg() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare function Connect(fn: (string) -> ())
    "#,
      ),
      false,
    );

    let substring = fixture.get_doc_symbol(
      r#"
        Connect(function(abc)
        end)
    "#,
      Position {
        line: 1,
        column: 27,
      },
    );

    assert_eq!(
      substring,
      Some(String::from("@test/global/Connect/param/0/param/0"))
    );
  }
}

mod ast_query_find_binding_at_position_global_start_of_file {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:390:ast_query_find_binding_at_position_global_start_of_file`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> function findBindingAtPosition (Analysis/src/AstQuery.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item ast_query_find_binding_at_position_global_start_of_file

  #[cfg(test)]
  #[test]
  fn ast_query_find_binding_at_position_global_start_of_file() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.get_frontend();
    fixture
      .base
      .base
      .check_string_optional_frontend_options(&String::from("local x = string.char(1)"), None);
    let pos = Position {
      line: 0,
      column: 12,
    };

    let module = fixture.base.base.get_main_module(false);
    let source_module = fixture.base.base.get_main_source_module();
    let binding = unsafe { find_binding_at_position(&*module, &*source_module, pos) };

    assert!(binding.is_some());
    assert_eq!(
      binding.unwrap().location,
      Location {
        begin: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 0 },
      }
    );
  }
}

mod ast_query_find_expr_ancestry {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:376:ast_query_find_expr_ancestry`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_find_expr_ancestry

  #[cfg(test)]
  #[test]
  fn ast_query_find_expr_ancestry() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl = {}
        function tbl:abc() end
    "#,
      ),
      None,
    );
    let pos = Position {
      line: 2,
      column: 29,
    };

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe { find_ast_ancestry_of_position(&*source_module, pos, false) };

    assert!(!ancestry.is_empty());
    assert!(unsafe { !ast_node_as::<AstExprFunction>(*ancestry.last().unwrap()).is_null() });
  }
}

mod ast_query_find_name_ancestry {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:362:ast_query_find_name_ancestry`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_find_name_ancestry

  #[cfg(test)]
  #[test]
  fn ast_query_find_name_ancestry() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tbl = {}
        function tbl:abc() end
    "#,
      ),
      None,
    );
    let pos = Position {
      line: 2,
      column: 18,
    };

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe { find_ast_ancestry_of_position(&*source_module, pos, false) };

    assert!(!ancestry.is_empty());
    assert!(unsafe { !ast_node_as::<AstExprLocal>(*ancestry.last().unwrap()).is_null() });
  }
}

mod ast_query_include_types_ancestry {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:349:ast_query_include_types_ancestry`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item ast_query_include_types_ancestry

  #[cfg(test)]
  #[test]
  fn ast_query_include_types_ancestry() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(&String::from("local x: number = 4;"), None);
    let pos = Position {
      line: 0,
      column: 10,
    };

    let source_module = fixture.get_main_source_module();
    let ancestry_no_types = unsafe { find_ast_ancestry_of_position(&*source_module, pos, false) };
    let ancestry_types = unsafe { find_ast_ancestry_of_position(&*source_module, pos, true) };

    assert!(ancestry_types.len() > ancestry_no_types.len());
    assert!(unsafe {
      (*ancestry_no_types.last().copied().unwrap())
        .as_type()
        .is_null()
    });
    assert!(unsafe {
      !(*ancestry_types.last().copied().unwrap())
        .as_type()
        .is_null()
    });
  }
}

mod ast_query_interior_binding_location_is_consistent_with_exterior_binding {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:402:ast_query_interior_binding_location_is_consistent_with_exterior_binding`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function findBindingAtPosition (Analysis/src/AstQuery.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item ast_query_interior_binding_location_is_consistent_with_exterior_binding

  #[cfg(test)]
  #[test]
  fn ast_query_interior_binding_location_is_consistent_with_exterior_binding() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function abcd(arg)
            abcd(arg)
        end

        abcd(0)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty());

    let module = fixture.get_main_module(false);
    let source_module = fixture.get_main_source_module();

    let decl_binding = unsafe {
      find_binding_at_position(
        &*module,
        &*source_module,
        Position {
          line: 1,
          column: 26,
        },
      )
    };
    assert!(decl_binding.is_some());
    assert_eq!(
      decl_binding.unwrap().location,
      Location {
        begin: Position {
          line: 1,
          column: 23
        },
        end: Position {
          line: 1,
          column: 27
        },
      }
    );

    let inner_call_binding = unsafe {
      find_binding_at_position(
        &*module,
        &*source_module,
        Position {
          line: 2,
          column: 15,
        },
      )
    };
    assert!(inner_call_binding.is_some());
    assert_eq!(
      inner_call_binding.unwrap().location,
      Location {
        begin: Position {
          line: 1,
          column: 23
        },
        end: Position {
          line: 1,
          column: 27
        },
      }
    );

    let outer_call_binding = unsafe {
      find_binding_at_position(&*module, &*source_module, Position { line: 5, column: 8 })
    };
    assert!(outer_call_binding.is_some());
    assert_eq!(
      outer_call_binding.unwrap().location,
      Location {
        begin: Position {
          line: 1,
          column: 23
        },
        end: Position {
          line: 1,
          column: 27
        },
      }
    );
  }
}

mod ast_query_last_argument_function_call_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:206:ast_query_last_argument_function_call_type`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method Fixture::findExpectedTypeAtPosition (tests/Fixture.cpp)
  //!   - translates_to -> rust_item ast_query_last_argument_function_call_type

  #[cfg(test)]
  #[test]
  fn ast_query_last_argument_function_call_type() {
    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function foo() return 2 end
local function bar(a: number) return -a end
bar(foo())
    "#,
      ),
      None,
    );

    let oty = fixture.find_type_at_position_position(Position { line: 3, column: 7 });
    assert!(oty.is_some());
    assert_eq!("number", to_string_type_id(oty.unwrap()));

    let expected_oty = fixture.find_expected_type_at_position(Position { line: 3, column: 7 });
    assert!(expected_oty.is_some());
    assert_eq!("number", to_string_type_id(expected_oty.unwrap()));
  }
}

mod ast_query_luau_nested_query {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:295:ast_query_luau_nested_query`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_luau_nested_query

  #[cfg(test)]
  #[test]
  fn ast_query_luau_nested_query() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        if true then
        end
    "#,
      &ParseOptions::default(),
    );

    let if_ = query::<AstStatIf>(block as *mut AstNode, vec![nth_t::<AstStatIf>(1)]);
    assert!(!if_.is_null());

    let bool_ =
      query::<AstExprConstantBool>(if_ as *mut AstNode, vec![nth_t::<AstExprConstantBool>(1)]);
    assert!(!bool_.is_null());
  }
}

mod ast_query_luau_nested_query_but_first_query_failed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:308:ast_query_luau_nested_query_but_first_query_failed`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_luau_nested_query_but_first_query_failed

  #[cfg(test)]
  #[test]
  fn ast_query_luau_nested_query_but_first_query_failed() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        if true then
        end
    "#,
      &ParseOptions::default(),
    );

    let if_ = query::<AstStatIf>(block as *mut AstNode, vec![nth_t::<AstStatIf>(2)]);
    assert!(if_.is_null());

    let bool_ =
      query::<AstExprConstantBool>(if_ as *mut AstNode, vec![nth_t::<AstExprConstantBool>(1)]);
    assert!(bool_.is_null());
  }
}

mod ast_query_luau_query {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:273:ast_query_luau_query`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - translates_to -> rust_item ast_query_luau_query

  #[cfg(test)]
  #[test]
  fn ast_query_luau_query() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        if true then
        end
    "#,
      &ParseOptions::default(),
    );

    let if_ = query::<AstStatIf>(block as *mut AstNode, vec![nth_t::<AstStatIf>(1)]);
    assert!(!if_.is_null());
  }
}

mod ast_query_luau_query_for_2nd_if_stat_which_doesnt_exist {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:284:ast_query_luau_query_for_2nd_if_stat_which_doesnt_exist`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - translates_to -> rust_item ast_query_luau_query_for_2nd_if_stat_which_doesnt_exist

  #[cfg(test)]
  #[test]
  fn ast_query_luau_query_for_2nd_if_stat_which_doesnt_exist() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        if true then
        end
    "#,
      &ParseOptions::default(),
    );

    let if_ = query::<AstStatIf>(block as *mut AstNode, vec![nth_t::<AstStatIf>(2)]);
    assert!(if_.is_null());
  }
}

mod ast_query_luau_selectively_query_for_a_different_boolean {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:321:ast_query_luau_selectively_query_for_a_different_boolean`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - calls -> function nth (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_luau_selectively_query_for_a_different_boolean

  #[cfg(test)]
  #[test]
  fn ast_query_luau_selectively_query_for_a_different_boolean() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        local x = false and true
        local y = true and false
    "#,
      &ParseOptions::default(),
    );

    let fst = query::<AstExprConstantBool>(
      block as *mut AstNode,
      vec![nth_t::<AstStatLocal>(1), nth_t::<AstExprConstantBool>(2)],
    );
    assert!(!fst.is_null());
    assert!(unsafe { (*fst).value });

    let snd = query::<AstExprConstantBool>(
      block as *mut AstNode,
      vec![nth_t::<AstStatLocal>(2), nth_t::<AstExprConstantBool>(2)],
    );
    assert!(!snd.is_null());
    assert!(!unsafe { (*snd).value });
  }
}

mod ast_query_luau_selectively_query_for_a_different_boolean_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:337:ast_query_luau_selectively_query_for_a_different_boolean_2`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - calls -> function nth (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_query_luau_selectively_query_for_a_different_boolean_2

  #[cfg(test)]
  #[test]
  fn ast_query_luau_selectively_query_for_a_different_boolean_2() {
    use ulua_ast::records::parse_options::ParseOptions;

    use super::ast_query_support::*;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      r#"
        local x = false and true
        local y = true and false
    "#,
      &ParseOptions::default(),
    );

    let snd = query::<AstExprConstantBool>(
      block as *mut AstNode,
      vec![nth_t::<AstStatLocal>(2), nth_t::<AstExprConstantBool>(1)],
    );
    assert!(!snd.is_null());
    assert!(unsafe { (*snd).value });
  }
}

mod ast_query_overloaded_class_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:106:ast_query_overloaded_class_method`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_overloaded_class_method

  #[cfg(test)]
  #[test]
  fn ast_query_overloaded_class_method() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            function bar(self, x: string): number
            function bar(self, x: number): string
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        local x: Foo = Foo.new()
        x:bar("asdf")
    "#,
      Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(
      symbol,
      Some(String::from(
        "@test/globaltype/Foo.bar/overload/(Foo, string) -> number"
      ))
    );
  }
}

mod ast_query_overloaded_fn {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:67:ast_query_overloaded_fn`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_overloaded_fn

  #[cfg(test)]
  #[test]
  fn ast_query_overloaded_fn() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare foo: ((string) -> number) & ((number) -> string)
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        foo("asdf")
    "#,
      Position {
        line: 1,
        column: 10,
      },
    );

    assert_eq!(
      symbol,
      Some(String::from("@test/global/foo/overload/(string) -> number"))
    );
  }
}

mod ast_query_parent_class_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:179:ast_query_parent_class_method`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_parent_class_method

  #[cfg(test)]
  #[test]
  fn ast_query_parent_class_method() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            function bar(self, x: string): number
        end

        declare extern type Bar extends Foo with
            function notbar(self, x: string): number
        end
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        local x: Bar = Bar.new()
        x:bar("asdf")
    "#,
      Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(symbol, Some(String::from("@test/globaltype/Foo.bar")));
  }
}

mod ast_query_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:38:ast_query_prop`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item ast_query_prop

  #[cfg(test)]
  #[test]
  fn ast_query_prop() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    let substring = fixture.get_doc_symbol(
      r#"
        local a = string.sub()
    "#,
      Position {
        line: 1,
        column: 27,
      },
    );

    assert_eq!(substring, Some(String::from("@luau/global/string.sub")));
  }
}

mod ast_query_string_metatable_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:166:ast_query_string_metatable_method`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function rep (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item ast_query_string_metatable_method

  #[cfg(test)]
  #[test]
  fn ast_query_string_metatable_method() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    let symbol = fixture.get_doc_symbol(
      r#"
        local x: string = "Foo"
        x:rep(2)
    "#,
      Position {
        line: 2,
        column: 12,
      },
    );

    assert_eq!(symbol, Some(String::from("@luau/global/string.rep")));
  }
}

pub(crate) mod ast_query_support {

  pub use ulua_analysis::functions::{
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete,
    find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position,
    find_binding_at_position::find_binding_at_position,
    to_string_to_string_alt_c::to_string_type_id,
  };
  pub use ulua_ast::{
    records::{
      ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_number::AstExprConstantNumber,
      ast_expr_function::AstExprFunction, ast_expr_index_name::AstExprIndexName,
      ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_if::AstStatIf,
      ast_stat_local::AstStatLocal, location::Location, position::Position,
    },
    rtti::ast_node_as,
  };
  pub use ulua_unit_test::{
    functions::{nth::nth_t, query::query},
    records::{documentation_symbol_fixture::DocumentationSymbolFixture, fixture::Fixture},
  };
}

mod ast_query_table_function_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:130:ast_query_table_function_prop`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_table_function_prop

  #[cfg(test)]
  #[test]
  fn ast_query_table_function_prop() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare Foo: {
            new: (number) -> string
        }
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        Foo.new("asdf")
    "#,
      Position {
        line: 1,
        column: 13,
      },
    );

    assert_eq!(symbol, Some(String::from("@test/global/Foo.new")));
  }
}

mod ast_query_table_overloaded_function_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:148:ast_query_table_overloaded_function_prop`
  //! Source: `tests/AstQuery.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstQuery.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/AstQuery.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item ast_query_table_overloaded_function_prop

  #[cfg(test)]
  #[test]
  fn ast_query_table_overloaded_function_prop() {
    use super::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
      &String::from(
        r#"
        declare Foo: {
            new: ((number) -> string) & ((string) -> number)
        }
    "#,
      ),
      false,
    );

    let symbol = fixture.get_doc_symbol(
      r#"
        Foo.new("asdf")
    "#,
      Position {
        line: 1,
        column: 13,
      },
    );

    assert_eq!(
      symbol,
      Some(String::from(
        "@test/global/Foo.new/overload/(string) -> number"
      ))
    );
  }
}
