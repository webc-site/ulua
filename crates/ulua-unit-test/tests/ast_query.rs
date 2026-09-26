extern crate alloc;

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_ac_ast_ancestry_at_number_const() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
print(3.)
    "#,
    None,
  );

  let source_module = fixture.main_source_module();
  let ancestry =
    find_ancestry_at_position_for_autocomplete(source_module, Position { line: 1, column: 8 });

  assert!(ancestry.len() >= 2);
  assert!(last_is_ast::<AstExprConstantNumber>(&ancestry));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_ac_ast_ancestry_in_workspace_colon() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
print(workspace:)
    "#,
    None,
  );

  let source_module = fixture.main_source_module();
  let ancestry = find_ancestry_at_position_for_autocomplete(
    source_module,
    Position {
      line: 1,
      column: 16,
    },
  );

  assert!(ancestry.len() >= 2);
  assert!(last_is_ast::<AstExprIndexName>(&ancestry));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_ac_ast_ancestry_in_workspace_dot() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
print(workspace.)
    "#,
    None,
  );

  let source_module = fixture.main_source_module();
  let ancestry = find_ancestry_at_position_for_autocomplete(
    source_module,
    Position {
      line: 1,
      column: 16,
    },
  );

  assert!(ancestry.len() >= 2);
  assert!(last_is_ast::<AstExprIndexName>(&ancestry));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_ast_ancestry_at_eof() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
if true then
    "#,
    None,
  );

  let source_module = fixture.main_source_module();
  let ancestry =
    find_ast_ancestry_of_position(source_module, Position { line: 2, column: 4 }, false);

  assert!(ancestry.len() >= 2);
  let parent_stat = ancestry[ancestry.len() - 2];
  assert!(parent_stat.as_node::<AstStatIf>().is_some());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_binding() {
  use crate::ast_query_support::*;

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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_class_method() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare extern type Foo with
            function bar(self, x: string): number
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_event_callback_arg() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare function Connect(fn: (string) -> ())
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_find_binding_at_position_global_start_of_file() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.get_frontend();
  fixture
    .base
    .base
    .check_string_optional_frontend_options("local x = string.char(1)", None);
  let pos = Position {
    line: 0,
    column: 12,
  };

  let module = fixture.base.base.get_main_module(false);
  let source_module = fixture.base.base.main_source_module();
  // Safety: module 为 resolver 容器保有的存活 Module，&* 物化只读借用；
  // source_module 已是 Handle 交付的共享只读借用。
  let binding = unsafe { find_binding_at_position(&*module, source_module, pos) };

  assert!(binding.is_some());
  assert_eq!(
    binding.unwrap().location,
    Location {
      begin: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 0 },
    }
  );
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_find_expr_ancestry() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
        local tbl = {}
        function tbl:abc() end
    "#,
    None,
  );
  let pos = Position {
    line: 2,
    column: 29,
  };

  let source_module = fixture.main_source_module();
  let ancestry = find_ast_ancestry_of_position(source_module, pos, false);

  assert!(!ancestry.is_empty());
  assert!(last_is_ast::<AstExprFunction>(&ancestry));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_find_name_ancestry() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options(
    r#"
        local tbl = {}
        function tbl:abc() end
    "#,
    None,
  );
  let pos = Position {
    line: 2,
    column: 18,
  };

  let source_module = fixture.main_source_module();
  let ancestry = find_ast_ancestry_of_position(source_module, pos, false);

  assert!(!ancestry.is_empty());
  assert!(last_is_ast::<AstExprLocal>(&ancestry));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_include_types_ancestry() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  fixture.check_string_optional_frontend_options("local x: number = 4;", None);
  let pos = Position {
    line: 0,
    column: 10,
  };

  let source_module = fixture.main_source_module();
  let ancestry_no_types = find_ast_ancestry_of_position(source_module, pos, false);
  let ancestry_types = find_ast_ancestry_of_position(source_module, pos, true);

  assert!(ancestry_types.len() > ancestry_no_types.len());
  assert!(unsafe { (*ancestry_no_types.last().copied().unwrap()).as_type() }.is_none());
  assert!(unsafe { (*ancestry_types.last().copied().unwrap()).as_type() }.is_some());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_interior_binding_location_is_consistent_with_exterior_binding() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function abcd(arg)
            abcd(arg)
        end

        abcd(0)
    "#,
    None,
  );

  assert!(result.errors.is_empty());

  let module = fixture.get_main_module(false);
  let source_module = fixture.main_source_module();

  let decl_binding = unsafe {
    find_binding_at_position(
      &*module,
      source_module,
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
      source_module,
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

  let outer_call_binding =
    unsafe { find_binding_at_position(&*module, source_module, Position { line: 5, column: 8 }) };
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_last_argument_function_call_type() {
  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
  fixture.check_string_optional_frontend_options(
    r#"
local function foo() return 2 end
local function bar(a: number) return -a end
bar(foo())
    "#,
    None,
  );

  let oty = fixture.find_type_at_position_position(Position { line: 3, column: 7 });
  assert!(oty.is_some());
  assert_eq!("number", to_string_type_id(oty.unwrap()));

  let expected_oty = fixture.find_expected_type_at_position(Position { line: 3, column: 7 });
  assert!(expected_oty.is_some());
  assert_eq!("number", to_string_type_id(expected_oty.unwrap()));
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_nested_query() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        if true then
        end
    "#,
    &ParseOptions::default(),
  );

  let if_ = query::<AstStatIf>(from_mut(block), vec![nth_t::<AstStatIf>(1)]);
  assert!(!if_.is_null());

  let bool_ = query::<AstExprConstantBool>(if_, vec![nth_t::<AstExprConstantBool>(1)]);
  assert!(!bool_.is_null());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_nested_query_but_first_query_failed() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        if true then
        end
    "#,
    &ParseOptions::default(),
  );

  let if_ = query::<AstStatIf>(from_mut(block), vec![nth_t::<AstStatIf>(2)]);
  assert!(if_.is_null());

  let bool_ = query::<AstExprConstantBool>(if_, vec![nth_t::<AstExprConstantBool>(1)]);
  assert!(bool_.is_null());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_query() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        if true then
        end
    "#,
    &ParseOptions::default(),
  );

  let if_ = query::<AstStatIf>(from_mut(block), vec![nth_t::<AstStatIf>(1)]);
  assert!(!if_.is_null());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_query_for_2nd_if_stat_which_doesnt_exist() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        if true then
        end
    "#,
    &ParseOptions::default(),
  );

  let if_ = query::<AstStatIf>(from_mut(block), vec![nth_t::<AstStatIf>(2)]);
  assert!(if_.is_null());
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_selectively_query_for_a_different_boolean() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        local x = false and true
        local y = true and false
    "#,
    &ParseOptions::default(),
  );

  let fst = query::<AstExprConstantBool>(
    from_mut(block),
    vec![nth_t::<AstStatLocal>(1), nth_t::<AstExprConstantBool>(2)],
  );
  assert!(!fst.is_null());
  assert!(unsafe { (*fst).value });

  let snd = query::<AstExprConstantBool>(
    from_mut(block),
    vec![nth_t::<AstStatLocal>(2), nth_t::<AstExprConstantBool>(2)],
  );
  assert!(!snd.is_null());
  assert!(!unsafe { (*snd).value });
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_luau_selectively_query_for_a_different_boolean_2() {
  use core::ptr::from_mut;

  use ulua_ast::records::parse_options::ParseOptions;

  use crate::ast_query_support::*;

  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
        local x = false and true
        local y = true and false
    "#,
    &ParseOptions::default(),
  );

  let snd = query::<AstExprConstantBool>(
    from_mut(block),
    vec![nth_t::<AstStatLocal>(2), nth_t::<AstExprConstantBool>(1)],
  );
  assert!(!snd.is_null());
  assert!(unsafe { (*snd).value });
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_overloaded_class_method() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare extern type Foo with
            function bar(self, x: string): number
            function bar(self, x: number): string
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_overloaded_fn() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare foo: ((string) -> number) & ((number) -> string)
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_parent_class_method() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare extern type Foo with
            function bar(self, x: string): number
        end

        declare extern type Bar extends Foo with
            function notbar(self, x: string): number
        end
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_prop() {
  use crate::ast_query_support::*;

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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_string_metatable_method() {
  use crate::ast_query_support::*;

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

pub(crate) mod ast_query_support {

  pub use ulua_analysis::functions::{
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete,
    find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position,
    find_binding_at_position::find_binding_at_position, to_string_to_string::to_string_type_id,
  };
  pub use ulua_ast::{
    records::{
      ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_number::AstExprConstantNumber,
      ast_expr_function::AstExprFunction, ast_expr_index_name::AstExprIndexName,
      ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_if::AstStatIf,
      ast_stat_local::AstStatLocal, location::Location, position::Position,
    },
    rtti::AstNodeClass,
  };
  pub use ulua_unit_test::{
    functions::{ast_node_ref::NodePtr, nth::nth_t, query::query},
    records::{documentation_symbol_fixture::DocumentationSymbolFixture, fixture::Fixture},
  };

  /// `ancestry.last().copied().unwrap().as_node::<T>().is_some()` 的收敛门面：
  /// 末位节点判型成功与否；`unwrap` 语义逐字保留（空 ancestry 照样 panic）。
  pub(crate) fn last_is_ast<T: AstNodeClass>(a: &[*mut AstNode]) -> bool {
    a.last().copied().unwrap().as_node::<T>().is_some()
  }
}

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_table_function_prop() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare Foo: {
            new: (number) -> string
        }
    "#,
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

// Source: `tests/AstQuery.test.cpp`
#[test]
fn ast_query_table_overloaded_function_prop() {
  use crate::ast_query_support::*;

  let mut fixture = DocumentationSymbolFixture::default();
  fixture.base.base.load_definition(
    r#"
        declare Foo: {
            new: ((number) -> string) & ((string) -> number)
        }
    "#,
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
