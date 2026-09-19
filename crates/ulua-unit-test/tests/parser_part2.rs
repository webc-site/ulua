extern crate alloc;

use ulua_ast::{
  records::{ast_array::AstArray, position::Position},
  rtti::AstNodeClass,
};
use ulua_unit_test::functions::ast_node_ref::{
  CstNodePtr, NodePtr, PtrRef, as_node_at, deref_at, elem, node_key,
};

// Port of `cpp/tests/Parser.test.cpp`.
// Automatically aggregated test suite.

/// cpp `array.data[i]`（元素为节点指针）：取下标并解引用，空指针即判定用例失败。
fn node<'a, T>(array: &AstArray<*mut T>, index: usize) -> &'a T {
  deref_at(array, index).expect("arena 节点指针必须非空")
}

/// cpp `array.data[i]->as<T>()`：取下标并下转，类型不符即判定用例失败。
fn node_as<'a, T: AstNodeClass, U>(array: &AstArray<*mut U>, index: usize) -> &'a T {
  as_node_at(array, index).expect("节点下转必须命中目标类型")
}

/// cpp `for (T* item : array)`：指针数组元素逐个解引用后迭代。
fn nodes<T>(array: &AstArray<*mut T>) -> impl Iterator<Item = &T> {
  array.iter().filter_map(|&ptr| ptr.as_ref_opt())
}

mod parser_end_extent_doesnt_consume_comments {
  use super::*;
  #[test]
  fn parser_end_extent_doesnt_consume_comments() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source =
      String::from("\n        type F = number\n        --comment\n        print('hello')\n    ");
    let block = fixture.parse(&source, &ParseOptions::new());

    assert_eq!(2, block.body.size);
    assert_eq!(Position::new(1, 23), node(&block.body, 0).base.location.end);
  }
}
mod parser_end_extent_doesnt_consume_comments_even_with_capture {
  use super::*;

  #[test]
  fn parser_end_extent_doesnt_consume_comments_even_with_capture() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code =
      String::from("\n        type F = number\n        --comment\n        print('hello')\n    ");
    let mut opts = ParseOptions::new();
    opts.capture_comments = true;

    let block = fix.parse(&code, &opts);

    assert_eq!(2, block.body.size);
    assert_eq!(Position::new(1, 23), node(&block.body, 0).base.location.end);
  }
}
mod parser_end_extent_of_functions_unions_and_intersections {
  use super::*;
  #[test]
  fn parser_end_extent_of_functions_unions_and_intersections() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "\n        type F = (string) -> string\n        type G = string | number | boolean\n        type H = string & number & boolean\n        print('hello')\n    ",
    );
    let block = fixture.parse(&source, &ParseOptions::new());

    assert_eq!(4, block.body.size);
    assert_eq!(Position::new(1, 35), node(&block.body, 0).base.location.end);
    assert_eq!(Position::new(2, 42), node(&block.body, 1).base.location.end);
    assert_eq!(Position::new(3, 42), node(&block.body, 2).base.location.end);
  }
}
mod parser_error_const_function_reassignment {

  #[test]
  fn parser_error_const_function_reassignment() {
    use ulua_common::fflag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source = String::from("const function a() return 42 end; a = 43");
    let message = String::from("Variable 'a' is constant and may not be reassigned");

    let _result = fixture.match_parse_error(&source, &message, None);
  }
}
mod parser_error_const_not_initialized {

  #[test]
  fn parser_error_const_not_initialized() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("const c"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b = nil"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b, c = f(), 42"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b, c = ..., 42"),
      &String::from("Missing initializer in const declaration"),
      None,
    );
  }
}
mod parser_error_const_reassignment {
  use super::*;
  #[test]
  fn parser_error_const_reassignment() {
    use alloc::string::String;

    use ulua_common::fflag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    fixture.match_parse_error(
      &String::from("const a = 42; a = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; a, b = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; b, a = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; b, a = ..."),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a = 42; function a() end"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );
  }
}
mod parser_error_message_for_using_function_as_type_annotation {

  #[test]
  fn parser_error_message_for_using_function_as_type_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("type Foo = function");
    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message().as_str(),
      "Using 'function' as a type annotation is not supported, consider replacing with a function type annotation e.g. '(...any) -> ...any'"
    );
  }
}
mod parser_error_on_confusable {

  #[test]
  fn parser_error_on_confusable() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local pi = 3․13` (U+2024 ONE DOT LEADER, a `.` confusable).
    // The ported literal was double-encoded UTF-8 mojibake; use a \u escape.
    fixture.match_parse_error(
          &String::from("\n        local pi = 3\u{2024}13\n    "),
          &String::from("Expected identifier when parsing expression, got Unicode character U+2024 (did you mean '.'?)"),
            None,
          );
  }
}
mod parser_error_on_non_utf_8_sequence {
  #[test]
  fn parser_error_on_non_utf_8_sequence() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected =
      String::from("Expected identifier when parsing expression, got invalid UTF-8 sequence");

    // C++ 源用的是裸字节 \xFF 与 \xE2（非法 UTF-8）。lossy 转换会把它们换成
    // U+FFFD（合法 UTF-8），词法层就再也看不到非法序列，因此走 fixture 的字节
    // 入口，让 0xFF / 0xE2 原样抵达 lexer，与 cpp 的逐字节语义一致。
    fixture.match_parse_error_bytes(b"local pi = \xFF!", &expected, None);
    fixture.match_parse_error_bytes(b"local pi = \xE2!", &expected, None);
  }
}
mod parser_error_on_unicode {

  #[test]
  fn parser_error_on_unicode() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local ☃ = 10` (U+2603 SNOWMAN). The ported literal was
    // double-encoded UTF-8 mojibake; use an explicit \u escape (encoding-safe).
    fixture.match_parse_error(
      &String::from("local \u{2603} = 10"),
      &String::from("Expected identifier when parsing variable name, got Unicode character U+2603"),
      None,
    );
  }
}
mod parser_explicit_type_instantiation_empty_list {

  #[test]
  fn parser_explicit_type_instantiation_empty_list() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `AstStat* stat = parse("f<<>>()"); REQUIRE(stat != nullptr);`
    // Rust 侧 parse 解析失败即 panic，返回引用天然非空。
    fixture.parse("f<<>>()", &ParseOptions::default());
  }
}
mod parser_explicit_type_instantiation_errors {

  #[test]
  fn parser_explicit_type_instantiation_errors() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a = x:a<<T>>"),
      &String::from("Expected '(', '{' or <string> when parsing function call, got <eof>"),
      None,
    );
  }
}
mod parser_explicit_type_instantiation_expression {

  #[test]
  fn parser_explicit_type_instantiation_expression() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse("local x = f<<T, U>>", &ParseOptions::default());
  }
}
mod parser_explicit_type_instantiation_expression_call {
  use super::*;
  #[test]
  fn parser_explicit_type_instantiation_expression_call() {
    use ulua_ast::records::{
      ast_expr_call::AstExprCall, ast_expr_instantiate::AstExprInstantiate,
      ast_stat_local::AstStatLocal, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      functions::string_at_location::string_at_location, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("local x = f<<T, U>>()");
    let options = ParseOptions::new();
    let result = fixture.parse_ex(&source, &options);

    let block = result.root_block().expect("根块必须存在");

    let local = node_as::<AstStatLocal, _>(&block.body, 0);
    assert_eq!(1, local.vars.size);

    let call = node_as::<AstExprCall, _>(&local.values, 0);
    let explicit_type_instantiation = call
      .func
      .as_node::<AstExprInstantiate>()
      .expect("call.func 应为实例化表达式");

    let location = &explicit_type_instantiation.base.base.location;
    let expected = string_at_location(&source, location);
    assert_eq!("f<<T, U>>", expected);
  }
}
mod parser_explicit_type_instantiation_indexing {

  #[test]
  fn parser_explicit_type_instantiation_indexing() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse(
      r#"t.f<<T, U>>()
          t:f<<T, U>>()
          t["f"]<<T, U>>()"#,
      &ParseOptions::default(),
    );
  }
}
mod parser_explicit_type_instantiation_statement {

  #[test]
  fn parser_explicit_type_instantiation_statement() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse("f<<T, U>>()", &ParseOptions::default());
  }
}
mod parser_export_class {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;

  use super::*;

  #[test]
  fn parser_export_class() {
    use ulua_ast::records::ast_stat_class::AstStatClass;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    let source = "\n        export class Foo\n        end\n    ";

    let result = fixture.try_parse(&String::from(source), &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);

    let block = result.root_block().expect("根块必须存在");
    assert_eq!(block.body.size, 1);

    let class_decl = node_as::<AstStatClass, _>(&block.body, 0);
    assert!(class_decl.exported);
  }
}
mod parser_export_is_an_identifier_only_when_followed_by_type {

  use ulua_ast::records::parse_options::ParseOptions;
  #[test]
  fn parser_export_is_an_identifier_only_when_followed_by_type() {
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::default();
    let _sff = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, false);

    let result = fixture.try_parse(
      &String::from("export function a() end"),
      &ParseOptions::new(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors.first().unwrap().get_message().as_str(),
      "Incomplete statement: expected assignment or a function call"
    );
  }
}
mod parser_export_value_parse_edge_cases {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;

  #[test]
  fn parser_export_value_parse_edge_cases() {
    use ulua_ast::records::{
      ast_stat_assign::AstStatAssign, ast_stat_compound_assign::AstStatCompoundAssign,
      ast_stat_expr::AstStatExpr,
    };
    use ulua_common::fflag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source1 = String::from("export = 5\nexport += 1\nexport()");
    let block1 = fixture.parse(&source1, &ParseOptions::default());
    assert_eq!(block1.body.size, 3);
    assert!(as_node_at::<AstStatAssign, _>(&block1.body, 0).is_some());
    assert!(as_node_at::<AstStatCompoundAssign, _>(&block1.body, 1).is_some());
    assert!(as_node_at::<AstStatExpr, _>(&block1.body, 2).is_some());

    fixture.parse(
      &String::from("export local x = 5"),
      &ParseOptions::default(),
    );
    fixture.parse(
      &String::from("export const x = 5"),
      &ParseOptions::default(),
    );
    fixture.parse(
      &String::from("export function foo()\nend"),
      &ParseOptions::default(),
    );

    fixture.match_parse_error(
      &String::from("export 42"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
    fixture.match_parse_error(
      &String::from("export if true then end"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
    fixture.match_parse_error(
      &String::from("export"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
  }
}
mod parser_export_value_parse_failures {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[test]
  fn parser_export_value_parse_failures() {
    use alloc::string::String;

    use ulua_common::fflag::{DebugLuauUserDefinedClasses, LuauExportValueSyntax};
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_debug_luau_user_defined_classes =
      ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let sources = [
      String::from("\nexport foo = 5\n    "),
      String::from("\nexport foo\n    "),
      String::from("\nfunction foo()\nend\nexport foo\n    "),
      String::from("\nexport local function foo()\nend\n    "),
    ];

    for source in sources.iter() {
      let _result = fixture.try_parse(source, &ParseOptions::default());
    }

    let duplicate_export = fixture.try_parse(
      &String::from("\nexport local foo = 1\nexport local foo = 2\n    "),
      &ParseOptions::default(),
    );
    let first_error_message = duplicate_export.errors.first().unwrap().get_message();
    assert!(first_error_message.contains("foo"));

    fixture.match_parse_error(
      &String::from("\nexport local answer = 42\nreturn {answer = answer}\n    "),
      &String::from(
        "Exporting values is not compatible with top-level return (export/return conflict)",
      ),
      None,
    );

    fixture.match_parse_error(
      &String::from("\nif skip then\n    return\nend\n\nexport local answer = 42\n    "),
      &String::from(
        "Exporting values is not compatible with top-level return (export/return conflict)",
      ),
      None,
    );

    fixture.match_parse_error(
          &String::from("\nexport class Player\n    public health: number\n    \n    function setHealth(self, health: number)\n        self.health = health\n        return self\n    end\n\n    function getHealth(self): number\n        return self.health\n    end\nend\n\nreturn Player {health = 100}\n    "),
          &String::from("Exporting values is not compatible with top-level return (export/return conflict)"),
            None,
          );

    let block_sources = [
      String::from("\nif true then\n    export local insideIf = 1\nend\n    "),
      String::from("\ndo\n    export const insideDo = 1\nend\n    "),
      String::from("\nwhile true do\n    export local insideWhile = 1\nend\n    "),
      String::from("\nrepeat\n    export local insideRepeat = 1\nuntil true\n    "),
      String::from("\nfor i = 1, 1 do\n    export local insideFor = i\nend\n    "),
      String::from("\nlocal function test()\n    export local insideFunction = 1\nend\n    "),
    ];

    for source in block_sources.iter() {
      fixture.match_parse_error(
        source,
        &String::from("'export' may only be applied to top-level statements"),
        None,
      );
    }
  }
}
mod parser_export_value_rfc {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[test]
  fn parser_export_value_rfc() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
      ast_stat_local_function::AstStatLocalFunction,
    };
    use ulua_common::fflag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source = String::from(
      "export local version = \"1.0.0\"\n\
           export const TAU = math.pi * 2\n\
           export local settings: Settings = getSettings()\n\
           export local a, b, c = 1, 2, 3\n\
           export local d\n\
           \n\
           export function add(a: number, b: number): number\n\
               return a + b\n\
           end\n\
           \n\
           export local f, g\n\
           function f()\n\
               return g()\n\
           end\n\
           \n\
           function g()\n\
               return 42\n\
           end\n\
           \n\
           local function ret(): (string, number, boolean)\n\
               return \"heh\", 42, false\n\
           end\n\
           export local x, y, z = ret()\n",
    );

    let block = fixture.parse(&source, &ParseOptions::new());

    assert_eq!(11, block.body.size);

    let version = node_as::<AstStatLocal, _>(&block.body, 0);
    assert!(version.is_exported);
    assert!(!version.is_const);
    assert_eq!(1, version.vars.size);
    assert!(node(&version.vars, 0).is_exported);
    assert!(!node(&version.vars, 0).is_const);

    let tau = node_as::<AstStatLocal, _>(&block.body, 1);
    assert!(tau.is_exported);
    assert!(tau.is_const);
    assert_eq!(1, tau.vars.size);
    assert!(node(&tau.vars, 0).is_exported);
    assert!(node(&tau.vars, 0).is_const);

    let settings = node_as::<AstStatLocal, _>(&block.body, 2);
    assert!(settings.is_exported);
    assert!(!settings.is_const);
    assert_eq!(1, settings.vars.size);
    assert!(
      node(&settings.vars, 0).annotation.as_ref_opt().is_some(),
      "settings 必须带类型标注"
    );

    let abc = node_as::<AstStatLocal, _>(&block.body, 3);
    assert!(abc.is_exported);
    assert!(!abc.is_const);
    assert_eq!(3, abc.vars.size);
    for local in nodes(&abc.vars) {
      assert!(local.is_exported);
      assert!(!local.is_const);
    }

    let d = node_as::<AstStatLocal, _>(&block.body, 4);
    assert!(d.is_exported);
    assert!(!d.is_const);
    assert_eq!(1, d.vars.size);
    assert_eq!(0, d.values.size);
    assert!(node(&d.vars, 0).is_exported);

    let add = node_as::<AstStatLocalFunction, _>(&block.body, 5);
    let add_name = add.name.as_ref_opt().expect("局部函数名必须存在");
    assert!(add_name.is_exported);
    assert!(add_name.is_const);

    let forward_decls = node_as::<AstStatLocal, _>(&block.body, 6);
    assert!(forward_decls.is_exported);
    assert!(!forward_decls.is_const);
    assert_eq!(2, forward_decls.vars.size);
    assert_eq!(0, forward_decls.values.size);
    for local in nodes(&forward_decls.vars) {
      assert!(local.is_exported);
      assert!(!local.is_const);
    }

    assert!(as_node_at::<AstStatFunction, _>(&block.body, 7).is_some());
    assert!(as_node_at::<AstStatFunction, _>(&block.body, 8).is_some());

    // C++ reads `xyz` from body.data[3] (re-checking the `export local a, b, c`
    // statement — the name is misleading but faithful to upstream).
    let xyz = node_as::<AstStatLocal, _>(&block.body, 3);
    assert!(xyz.is_exported);
    assert!(!xyz.is_const);
    assert_eq!(3, xyz.vars.size);
    for local in nodes(&xyz.vars) {
      assert!(local.is_exported);
      assert!(!local.is_const);
    }

    let source2 = String::from(
      "export type Config = {\n\
           debug: boolean,\n\
           timeout: number,\n\
           }\n\
           \n\
           return {\n\
           debug = false,\n\
           timeout = 5,\n\
           }\n",
    );

    fixture.parse(&source2, &ParseOptions::new());
  }
}
mod parser_expr_group_with_cst {
  use ulua_common::fflag;

  use super::*;

  #[test]
  fn parser_expr_group_with_cst() {
    use ulua_ast::records::{
      ast_expr_group::AstExprGroup, ast_stat_local::AstStatLocal, cst_expr_group::CstExprGroup,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _scoped_flag = ScopedFastFlag::new(&fflag::LuauCstExprGroup, true);
    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        local a = (1 + 2)\n    ");
    let mut parse_options = ParseOptions::new();
    parse_options.store_cst_data = true;

    let result = fixture.parse_ex(&source, &parse_options);

    let block = result.root_block().expect("根块必须存在");
    assert_eq!(1, block.body.size);

    let local_stmt = node_as::<AstStatLocal, _>(&block.body, 0);
    assert_eq!(1, local_stmt.values.size);

    let group_expr = node_as::<AstExprGroup, _>(&local_stmt.values, 0);

    let base_cst_node = result
      .cst_node_map
      .find(&node_key(group_expr))
      .copied()
      .expect("group 表达式必须带 CST 节点");

    let cst_node = base_cst_node
      .as_cst::<CstExprGroup>()
      .expect("CST 节点应为 CstExprGroup");
    assert_eq!(Position::new(1, 24), cst_node.close_position);
  }
}
mod parser_extern_read_write_attributes {
  use super::*;
  #[test]
  fn parser_extern_read_write_attributes() {
    use ulua_ast::{
      enums::ast_table_access::AstTableAccess,
      records::{
        ast_stat_declare_extern_type::AstStatDeclareExternType, parse_options::ParseOptions,
      },
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _ = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

    let mut fix = Fixture::default();
    let code = String::from(
      "\n        declare extern type Foo with\n            read ReadOnlyMember: string\n            write WriteOnlyMember: number\n            ReadWriteMember: vector\n            wRITE BadAttributeMember: buffer\n        end\n    ",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].get_location().begin.line, 5);
    assert_eq!(
      *result.errors[0].get_message(),
      "Expected blank or 'read' or 'write' attribute, got 'wRITE'"
    );

    let stat = result.root_block().expect("根块必须存在");
    assert_eq!(stat.body.size, 1);

    let declared_extern_type = node_as::<AstStatDeclareExternType, _>(&stat.body, 0);
    assert_eq!(declared_extern_type.props.size, 4);

    assert_eq!(
      elem(&declared_extern_type.props, 0).access,
      AstTableAccess::Read
    );
    assert_eq!(
      elem(&declared_extern_type.props, 1).access,
      AstTableAccess::Write
    );
    assert_eq!(
      elem(&declared_extern_type.props, 2).access,
      AstTableAccess::ReadWrite
    );
    assert_eq!(
      elem(&declared_extern_type.props, 3).access,
      AstTableAccess::ReadWrite
    );
  }
}
mod parser_extra_table_indexer_recovery {

  #[test]
  fn parser_extra_table_indexer_recovery() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let parse_result = fixture.try_parse(
      &String::from("local a : { [string] : number, [number] : string, count: number }"),
      &ParseOptions::default(),
    );
    assert_eq!(parse_result.errors.len(), 1);
  }
}
mod parser_extra_token_in_consume {

  use ulua_ast::records::parse_options::ParseOptions;
  #[test]
  fn parser_extra_token_in_consume() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nfunction test + (a, f) return a + f end\nreturn test(2, 3)\n";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected '(' when parsing function, got '+'"
    );
  }
}
mod parser_extra_token_in_consume_match {

  use ulua_ast::records::parse_options::ParseOptions;
  #[test]
  fn parser_extra_token_in_consume_match() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("function test(a, f+) return a + f end\nreturn test(2, 3)\n");

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected ')' (to close '(' at column 14), got '+'"
    );
  }
}
mod parser_extra_token_in_consume_match_end {

  use ulua_ast::records::parse_options::ParseOptions;
  #[test]
  fn parser_extra_token_in_consume_match_end() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nif true then\n    return 12\nthen\nend\n";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected 'end' (to close 'then' at line 2), got 'then'"
    );
  }
}
mod parser_for_loop_with_single_var_has_comma_positions_of_size_zero {
  use super::*;
  #[test]
  fn parser_for_loop_with_single_var_has_comma_positions_of_size_zero() {
    use ulua_ast::records::{
      ast_stat_for_in::AstStatForIn, cst_stat_for_in::CstStatForIn, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("for value in tbl do\nend");
    let mut options = ParseOptions::new();
    options.store_cst_data = true;
    let result = fixture.parse_ex(&source, &options);

    let block = result.root_block().expect("根块必须存在");
    assert_eq!(1, block.body.size);

    let for_loop = node_as::<AstStatForIn, _>(&block.body, 0);

    let base_cst_node = result
      .cst_node_map
      .find(&node_key(for_loop))
      .copied()
      .expect("for 循环必须带 CST 节点");

    let cst_node = base_cst_node
      .as_cst::<CstStatForIn>()
      .expect("CST 节点应为 CstStatForIn");
    assert_eq!(0, cst_node.vars_comma_positions.size);
  }
}
mod parser_function_name_has_correct_start_location {
  use super::*;
  #[test]
  fn parser_function_name_has_correct_start_location() {
    use ulua_ast::records::{ast_stat_function::AstStatFunction, parse_options::ParseOptions};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "\n        function simple()\n        end\n\n        function T:complex()\n        end\n    ",
    );
    let opts = ParseOptions::new();
    let block = fix.parse(&code, &opts);
    assert_eq!(block.body.size, 2);

    let function1 = node_as::<AstStatFunction, _>(&block.body, 0);
    let name1 = function1.name.as_ref_opt().expect("函数名必须存在");
    assert_eq!(Position::new(1, 17), name1.base.location.begin);

    let function2 = node_as::<AstStatFunction, _>(&block.body, 1);
    let name2 = function2.name.as_ref_opt().expect("函数名必须存在");
    assert_eq!(Position::new(4, 17), name2.base.location.begin);
  }
}
mod parser_function_return_type_should_disambiguate_from_function_type_and_multiple_returns {
  use super::*;
  #[test]
  fn parser_function_return_type_should_disambiguate_from_function_type_and_multiple_returns() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_pack_explicit::AstTypePackExplicit,
      ast_type_reference::AstTypeReference, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function f(): (number, string) return 1, \"foo\" end",
      &ParseOptions::default(),
    );

    assert!(!block.body.is_empty());

    let stat_func = node_as::<AstStatFunction, _>(&block.body, 0);

    let func = stat_func.func.as_ref_opt().expect("函数表达式必须存在");
    assert!(
      func.return_annotation.as_ref_opt().is_some(),
      "返回值标注必须存在"
    );

    let type_pack = func
      .return_annotation
      .as_node::<AstTypePackExplicit>()
      .expect("应为显式 type pack");
    assert!(
      type_pack.type_list.tail_type.as_ref_opt().is_none(),
      "不应有尾部 type pack"
    );

    let ret_types = &type_pack.type_list.types;
    assert_eq!(2, ret_types.size);

    let ty0 = node_as::<AstTypeReference, _>(ret_types, 0);
    assert_eq!(Some("number"), ty0.name.as_str());

    let ty1 = node_as::<AstTypeReference, _>(ret_types, 1);
    assert_eq!(Some("string"), ty1.name.as_str());
  }
}
mod parser_function_return_type_should_parse_as_function_type_annotation_with_no_args {
  use super::*;
  #[test]
  fn parser_function_return_type_should_parse_as_function_type_annotation_with_no_args() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_function::AstTypeFunction,
      ast_type_pack_explicit::AstTypePackExplicit, ast_type_reference::AstTypeReference,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function f(): () -> nil return nil end",
      &ParseOptions::default(),
    );

    assert!(!block.body.is_empty());

    let stat_func = node_as::<AstStatFunction, _>(&block.body, 0);

    let func = stat_func.func.as_ref_opt().expect("函数表达式必须存在");
    assert!(
      func.return_annotation.as_ref_opt().is_some(),
      "返回值标注必须存在"
    );

    let type_pack = func
      .return_annotation
      .as_node::<AstTypePackExplicit>()
      .expect("应为显式 type pack");
    assert!(
      type_pack.type_list.tail_type.as_ref_opt().is_none(),
      "不应有尾部 type pack"
    );

    let ret_types = &type_pack.type_list.types;
    assert_eq!(1, ret_types.size);

    let fun_ty = node_as::<AstTypeFunction, _>(ret_types, 0);
    assert_eq!(0, fun_ty.arg_types.types.size);
    assert!(
      fun_ty.arg_types.tail_type.as_ref_opt().is_none(),
      "参数不应有尾部 type pack"
    );

    let fun_return_pack = fun_ty
      .return_types
      .as_node::<AstTypePackExplicit>()
      .expect("返回值应为显式 type pack");
    assert!(
      fun_return_pack.type_list.tail_type.as_ref_opt().is_none(),
      "不应有尾部 type pack"
    );

    let ty = node_as::<AstTypeReference, _>(&fun_return_pack.type_list.types, 0);
    assert_eq!(Some("nil"), ty.name.as_str());
  }
}
mod parser_function_start_locations_are_before_attributes {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;

  #[test]
  fn parser_function_start_locations_are_before_attributes() {
    use ulua_ast::records::{
      ast_expr_function::AstExprFunction, ast_stat_function::AstStatFunction,
      ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
      location::Location,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
          "\n        @native\n        function globalFunction()\n        end\n\n        @native\n        local function localFunction()\n        end\n\n        local _ = @native function()\n        end\n    ",
          &ParseOptions::default(),
      );
    assert_eq!(3, block.body.size);

    let global_function = node_as::<AstStatFunction, _>(&block.body, 0);
    assert_eq!(
      Location::new(Position::new(1, 8), Position::new(3, 11)),
      global_function.base.base.location
    );

    let local_function = node_as::<AstStatLocalFunction, _>(&block.body, 1);
    assert_eq!(
      Location::new(Position::new(5, 8), Position::new(7, 11)),
      local_function.base.base.location
    );

    let local_variable = node_as::<AstStatLocal, _>(&block.body, 2);
    assert_eq!(1, local_variable.values.size);

    let anonymous_function = node_as::<AstExprFunction, _>(&local_variable.values, 0);
    assert_eq!(
      Location::new(Position::new(9, 18), Position::new(10, 11)),
      anonymous_function.base.base.location
    );
  }
}
mod parser_function_type_annotation {

  #[test]
  fn parser_function_type_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse("local f: (number, string) -> nil", &ParseOptions::default());
  }
}
mod parser_function_type_matching_parenthesis {

  #[test]
  fn parser_function_type_matching_parenthesis() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a: <T>(number -> string"),
      &String::from("Expected ')' (to close '(' at column 13), got '->'"),
      None,
    );
  }
}
mod parser_function_type_named_arguments {
  use super::*;

  #[test]
  fn parser_function_type_named_arguments() {
    use ulua_ast::records::{
      ast_name::AstName, ast_stat_type_alias::AstStatTypeAlias, ast_type_function::AstTypeFunction,
      ast_type_pack_explicit::AstTypePackExplicit, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    /// cpp 各块共有的前奏：`parseEx(src)` -> 根块 -> 类型别名 -> `AstTypeFunction`。
    fn func_type<'a>(fixture: &'a mut Fixture, source: &str) -> &'a AstTypeFunction {
      let result = fixture.parse_ex(source, &ParseOptions::default());
      let block = result.root_block().expect("根块必须存在");
      let decl = node_as::<AstStatTypeAlias, _>(&block.body, 0);
      decl
        .type_ptr
        .as_node::<AstTypeFunction>()
        .expect("别名右侧应为函数类型")
    }

    /// cpp `REQUIRE(array.data[i]) && CHECK_EQ(array.data[i]->first, name)`。
    fn named_arg(func: &AstTypeFunction, index: usize) -> AstName {
      let (name, _) = elem(&func.arg_names, index)
        .as_ref()
        .expect("该位置应有具名参数");
      *name
    }

    {
      let mut fixture = Fixture::default();
      let func = func_type(
        &mut fixture,
        "type MyFunc = (a: number, b: string, c: number) -> string",
      );
      assert_eq!(func.arg_types.types.size, 3);
      assert_eq!(func.arg_names.size, 3);
      assert_eq!(Some("c"), named_arg(func, 2).as_str());
    }

    {
      let mut fixture = Fixture::default();
      let func = func_type(
        &mut fixture,
        "type MyFunc = (a: number, string, c: number) -> string",
      );
      assert_eq!(func.arg_types.types.size, 3);
      assert_eq!(func.arg_names.size, 3);
      assert!(elem(&func.arg_names, 1).is_none());
      assert_eq!(Some("c"), named_arg(func, 2).as_str());
    }

    {
      let mut fixture = Fixture::default();
      let func = func_type(
        &mut fixture,
        "type MyFunc = (a: number, string, number) -> string",
      );
      assert_eq!(func.arg_types.types.size, 3);
      assert_eq!(func.arg_names.size, 3);
      assert!(elem(&func.arg_names, 1).is_none());
      assert!(elem(&func.arg_names, 2).is_none());
    }

    {
      let mut fixture = Fixture::default();
      let func = func_type(
        &mut fixture,
        "type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number) -> string",
      );
      assert_eq!(func.arg_types.types.size, 3);
      assert_eq!(func.arg_names.size, 3);
      assert_eq!(Some("c"), named_arg(func, 2).as_str());

      let explicit_pack = func
        .return_types
        .as_node::<AstTypePackExplicit>()
        .expect("返回值应为显式 type pack");
      let func_ret = node_as::<AstTypeFunction, _>(&explicit_pack.type_list.types, 0);
      assert_eq!(func_ret.arg_types.types.size, 3);
      assert_eq!(func_ret.arg_names.size, 3);
      assert_eq!(Some("f"), named_arg(func_ret, 2).as_str());
    }

    {
      let mut fixture = Fixture::default();
      fixture.match_parse_error(
        &String::from(
          "type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number)",
        ),
        &String::from("Expected '->' when parsing function type, got <eof>"),
        None,
      );
    }

    {
      let mut fixture = Fixture::default();
      fixture.match_parse_error(
        &String::from("type MyFunc = (number) -> (d: number) <a, b, c> -> number"),
        &String::from("Expected '->' when parsing function type, got '<'"),
        None,
      );
    }
  }
}
mod parser_functions_can_have_0_arguments {

  #[test]
  fn parser_functions_can_have_0_arguments() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse("local f: () -> number", &ParseOptions::default());
  }
}
mod parser_functions_can_have_a_function_type_annotation {
  use super::*;
  #[test]
  fn parser_functions_can_have_a_function_type_annotation() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_function::AstTypeFunction,
      ast_type_pack_explicit::AstTypePackExplicit, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function f(): (number) -> nil return nil end",
      &ParseOptions::default(),
    );
    assert!(!block.body.is_empty());

    let stat_func = node_as::<AstStatFunction, _>(&block.body, 0);

    let func = stat_func.func.as_ref_opt().expect("函数表达式必须存在");
    assert!(
      func.return_annotation.as_ref_opt().is_some(),
      "返回值标注必须存在"
    );

    let type_pack = func
      .return_annotation
      .as_node::<AstTypePackExplicit>()
      .expect("应为显式 type pack");
    let ret_types = &type_pack.type_list.types;
    assert!(
      type_pack.type_list.tail_type.as_ref_opt().is_none(),
      "不应有尾部 type pack"
    );
    assert_eq!(ret_types.size, 1);

    assert!(
      as_node_at::<AstTypeFunction, _>(ret_types, 0).is_some(),
      "返回类型应为函数类型"
    );
  }
}
mod parser_functions_can_have_return_annotations {
  use super::*;
  #[test]
  fn parser_functions_can_have_return_annotations() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_pack_explicit::AstTypePackExplicit,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function foo(): number return 55 end",
      &ParseOptions::default(),
    );

    assert!(!block.body.is_empty());

    let stat_function = node_as::<AstStatFunction, _>(&block.body, 0);

    let func = stat_function.func.as_ref_opt().expect("函数表达式必须存在");
    assert!(
      func.return_annotation.as_ref_opt().is_some(),
      "返回值标注必须存在"
    );

    let type_pack_explicit = func
      .return_annotation
      .as_node::<AstTypePackExplicit>()
      .expect("应为显式 type pack");

    let type_list = &type_pack_explicit.type_list;
    assert_eq!(type_list.types.size, 1);
    assert!(
      type_list.tail_type.as_ref_opt().is_none(),
      "不应有尾部 type pack"
    );
  }
}
mod parser_functions_can_return_0_values {

  #[test]
  fn parser_functions_can_return_0_values() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(block != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse("local f: (number) -> ()", &ParseOptions::default());
  }
}
mod parser_functions_can_return_multiple_values {

  #[test]
  fn parser_functions_can_return_multiple_values() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // cpp: `REQUIRE(stat != nullptr)`；Rust 侧 parse 出错即 panic，引用必非空。
    fixture.parse(
      "local f: (number) -> (number, number)",
      &ParseOptions::default(),
    );
  }
}
mod parser_generic_function_declaration_parsing {
  use super::*;
  #[test]
  fn parser_generic_function_declaration_parsing() {
    use ulua_ast::records::{
      ast_stat_declare_function::AstStatDeclareFunction, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from("declare function f<a, b, c...>()"),
      &ParseOptions::default(),
    );

    let block = result.root_block().expect("根块必须存在");
    assert!(!block.body.is_empty());

    let decl = node_as::<AstStatDeclareFunction, _>(&block.body, 0);
    assert_eq!(2, decl.generics.size);
    assert_eq!(1, decl.generic_packs.size);
  }
}
mod parser_generic_pack_parsing {
  use super::*;

  #[test]
  fn parser_generic_pack_parsing() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_stat_type_alias::AstStatTypeAlias,
      ast_type_function::AstTypeFunction, ast_type_pack_generic::AstTypePackGeneric,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from(
        "function f<a...>(...: a...)\n\
               end\n\
               \n\
               type A = (a...) -> b...",
      ),
      &ParseOptions::default(),
    );

    let block = result.root_block().expect("根块必须存在");
    assert!(!block.body.is_empty());

    let fn_stat = node_as::<AstStatFunction, _>(&block.body, 0);
    let func = fn_stat.func.as_ref_opt().expect("函数表达式必须存在");
    assert!(
      func.vararg_annotation.as_ref_opt().is_some(),
      "可变参数标注必须存在"
    );

    let vararg_annot = func
      .vararg_annotation
      .as_node::<AstTypePackGeneric>()
      .expect("可变参数标注应为 generic type pack");
    assert_eq!(Some("a"), vararg_annot.generic_name.as_str());

    let alias = node_as::<AstStatTypeAlias, _>(&block.body, 1);
    let fn_ty = alias
      .type_ptr
      .as_node::<AstTypeFunction>()
      .expect("别名右侧应为函数类型");

    let arg_annot = fn_ty
      .arg_types
      .tail_type
      .as_node::<AstTypePackGeneric>()
      .expect("参数尾包应为 generic type pack");
    assert_eq!(Some("a"), arg_annot.generic_name.as_str());

    let ret_annot = fn_ty
      .return_types
      .as_node::<AstTypePackGeneric>()
      .expect("返回值应为 generic type pack");
    assert_eq!(Some("b"), ret_annot.generic_name.as_str());
  }
}
