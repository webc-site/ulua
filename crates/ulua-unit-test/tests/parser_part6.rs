//! Port of `cpp/tests/Parser.test.cpp`（TEST_CASE_F 聚合分片 part6）。
//!
//! 语义逐条对齐 C++ 原用例；AST 下转统一走
//! [`ulua_unit_test::functions::ast_node_ref`]，调用点零 `unsafe`。

use std::{
  panic::{AssertUnwindSafe, catch_unwind},
  sync::{Mutex, MutexGuard},
};

use ulua_analysis::functions::parse_mode::parse_mode;
use ulua_ast::{
  enums::mode::Mode,
  records::{
    ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
    ast_stat_local::AstStatLocal, ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable, cst_type_group::CstTypeGroup, location::Location,
    parse_errors::ParseErrors, parse_options::ParseOptions, position::Position,
  },
  visit::AstVisitable,
};
use ulua_common::fint;
use ulua_unit_test::{
  functions::{
    ast_node_ref::{CstNodePtr, NodePtr, PtrMutRef, PtrRef, as_node_at, deref_at, elem, node_key},
    check_first_error_for_attributes::check_first_error_for_attributes,
  },
  records::{count_ast_nodes::CountAstNodes, fixture::Fixture},
  type_aliases::scoped_fast_int::ScopedFastInt,
};

/// cpp `try { parse(...) } catch (const Luau::ParseErrors& e)`：解析必须抛出，
/// 否则用例失败（对应 cpp 的 `FAIL("Expected ParseErrors to be thrown")`）。
fn catch_parse_errors(run: impl FnOnce()) -> ParseErrors {
  let payload = catch_unwind(AssertUnwindSafe(run)).expect_err("Expected ParseErrors to be thrown");
  payload
    .downcast_ref::<ParseErrors>()
    .expect("Expected ParseErrors")
    .clone()
}

/// `FInt::LuauParseErrorLimit` 是进程级全局：cpp（Catch2）用例顺序执行，Rust
/// 用例并发执行，故改写上限的用例与依赖默认上限（多错误 / 错误后根块非空）的
/// 用例须互斥。
static PARSE_LIMIT_MUTEX: Mutex<()> = Mutex::new(());

fn lock_parse_limit() -> MutexGuard<'static, ()> {
  // 锁中毒只可能来自同文件其他用例的断言失败，恢复锁继续跑即可
  PARSE_LIMIT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

/// cpp `recover_type_index_name_keyword`。
#[test]
fn parser_recover_type_index_name_keyword() {
  let mut fix = Fixture::default();

  let result = fix.try_parse("\nlocal A\nlocal b : A.do\n", &ParseOptions::default());
  assert_eq!(1, result.errors.len());

  let result = fix.try_parse(
    "\nlocal A\nlocal b : A.do\ndo end\n",
    &ParseOptions::default(),
  );
  assert_eq!(1, result.errors.len());
}

/// cpp `recover_unexpected_type_pack`。
#[test]
fn parser_recover_unexpected_type_pack() {
  let _limit_guard = lock_parse_limit();
  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\ntype X<T...> = { a: T..., b: number }\n\
       type Y<T> = { a: T..., b: number }\n\
       type Z<T> = { a: string | T..., b: number }\n",
    &ParseOptions::default(),
  );
  assert_eq!(3, result.errors.len());
}

/// cpp `recovery_error_limit_1`：限 1 时单条错误的 `what()` 即其消息。
#[test]
fn parser_recovery_error_limit_1() {
  let _limit_guard = lock_parse_limit();
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 1);
  let mut fix = Fixture::default();

  let errors = catch_parse_errors(|| {
    fix.parse("local a = ", &ParseOptions::default());
  });
  assert_eq!(1, errors.get_errors().len());
  assert_eq!(errors.get_errors()[0].get_message(), errors.what());
}

/// cpp `recovery_error_limit_2`：限 2 时报 3 条，末条为达到上限。
#[test]
fn parser_recovery_error_limit_2() {
  let _limit_guard = lock_parse_limit();
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 2);
  let mut fix = Fixture::default();

  let errors = catch_parse_errors(|| {
    fix.parse("escape escape escape", &ParseOptions::default());
  });
  assert_eq!(3, errors.get_errors().len());
  assert_eq!("3 parse errors", errors.what());
  assert_eq!(
    "Reached error limit (2)",
    errors.get_errors().last().unwrap().get_message()
  );
}

mod parser_recovery_of_parenthesized_expressions {
  use super::*;

  /// cpp `sourceModule->root->visit(&counter)`：统计当前根块的节点数。
  fn count_ast_nodes(fix: &Fixture) -> u32 {
    // `as_mut_ref_opt` 需要指针槽可变（独占借用的来源），故 `root` 声明为 `mut`。
    let mut root = fix
      .source_module
      .as_deref()
      .expect("sourceModule 必须存在")
      .root;
    let mut counter = CountAstNodes::default();
    // visit 需要独占借用：cpp `AstNode::visit(AstVisitor*)` 的 this 非 const。
    root.as_mut_ref_opt().expect("根块非空").visit(&mut counter);
    counter.count
  }

  /// cpp lambda `checkAstEquivalence`：错误恢复后的 AST 与正确 AST 节点数相等。
  fn check_ast_equivalence(fix: &mut Fixture, code_with_errors: &str, code: &str) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
      fix.parse(code_with_errors, &ParseOptions::default());
    }));
    let count_with_errors = count_ast_nodes(fix);

    fix.parse(code, &ParseOptions::default());
    assert_eq!(count_with_errors, count_ast_nodes(fix));
  }

  /// cpp lambda `checkRecovery`：先验抛出与错误条数，再验 AST 等价。
  fn check_recovery(fix: &mut Fixture, code_with_errors: &str, code: &str, expected: usize) {
    let errors = catch_parse_errors(|| {
      fix.parse(code_with_errors, &ParseOptions::default());
    });
    assert_eq!(expected, errors.get_errors().len());
    check_ast_equivalence(fix, code_with_errors, code);
  }

  /// cpp 中 12 次 `checkRecovery(..., ..., 1)` 的用例表。
  const RECOVERIES: [(&str, &str); 12] = [
    (
      "function foo(a, b. c) return a + b end",
      "function foo(a, b) return a + b end",
    ),
    (
      "function foo(a, b: { a: number, b: number. c:number }) return a + b end",
      "function foo(a, b: { a: number, b: number }) return a + b end",
    ),
    (
      "function foo(a, b): (number -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
    ),
    (
      "function foo(a, b): (number, number -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
    ),
    (
      "function foo(a, b): (number; number) -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
    ),
    (
      "function foo(a, b): (number, number return a + b end",
      "function foo(a, b): (number, number) end",
    ),
    (
      "local function foo(a, b): (number, number return a + b end",
      "local function foo(a, b): (number, number) end",
    ),
    (
      "type F = (number, number -> number",
      "type F = (number, number) -> number",
    ),
    (
      "function foo(a, b: { a: number, b: number) return a + b end",
      "function foo(a, b: { a: number, b: number }) return a + b end",
    ),
    (
      "function foo(a, b: { [number: number}) return a + b end",
      "function foo(a, b: { [number]: number}) return a + b end",
    ),
    (
      "local n: (string | number = 2",
      "local n: (string | number) = 2",
    ),
    (
      "\nfunction foo(a, b\n    return a + b\nend\n",
      "function foo(a, b) return a + b end",
    ),
  ];

  /// cpp `recovery_of_parenthesized_expressions`。
  #[test]
  fn parser_recovery_of_parenthesized_expressions() {
    let _limit_guard = lock_parse_limit();
    let mut fix = Fixture::default();
    for (code_with_errors, code) in RECOVERIES {
      check_recovery(&mut fix, code_with_errors, code, 1);
    }
  }
}

/// cpp 同名用例：返回类型包首个类型是 `AstTypeIntersection`，其成员为
/// 括号组 + 函数类型。
#[test]
fn parser_return_type_is_an_intersection_type_if_led_with_one_parenthesized_type() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "local f: (string) -> (string) & (number) -> (number)",
    &ParseOptions::default(),
  );

  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 AstStatLocal");
  let var = deref_at(&local.vars, 0).expect("vars[0] 存在");
  let annotation = var
    .annotation
    .as_node::<AstTypeFunction>()
    .expect("annotation 应为 AstTypeFunction");

  let return_pack = annotation
    .return_types
    .as_node::<AstTypePackExplicit>()
    .expect("returnTypes 应为 AstTypePackExplicit");
  let first = as_node_at::<AstTypeIntersection, _>(&return_pack.type_list.types, 0)
    .expect("types[0] 应为 AstTypeIntersection");

  assert!(
    as_node_at::<AstTypeGroup, _>(&first.types, 0).is_some(),
    "types[0] 应为 AstTypeGroup"
  );
  assert!(
    as_node_at::<AstTypeFunction, _>(&first.types, 1).is_some(),
    "types[1] 应为 AstTypeFunction"
  );
}

/// cpp `sense_hot_comment_on_first_line`。
#[test]
fn parser_sense_hot_comment_on_first_line() {
  let mut fix = Fixture::default();
  let options = ParseOptions {
    capture_comments: true,
    ..Default::default()
  };

  let result = fix.parse_ex("   --!strict ", &options);
  let mode = parse_mode(&result.hotcomments).expect("hotcomments 应解析出模式");
  assert_eq!(Mode::Strict, mode);
}

/// cpp `short_array_types`：`{string}` 展开为 number 索引、string 结果的索引器。
#[test]
fn parser_short_array_types() {
  let mut fix = Fixture::default();
  let block = fix.parse("local n: {string}", &ParseOptions::default());

  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 AstStatLocal");
  let var = deref_at(&local.vars, 0).expect("vars[0] 存在");
  let annotation = var
    .annotation
    .as_node::<AstTypeTable>()
    .expect("annotation 应为 AstTypeTable");

  assert_eq!(0, annotation.props.size);
  let indexer = annotation.indexer.as_ref_opt().expect("indexer 必须存在");

  let index_type = indexer
    .index_type
    .as_node::<AstTypeReference>()
    .expect("indexType 应为 AstTypeReference");
  assert_eq!(index_type.name, "number");

  let result_type = indexer
    .result_type
    .as_node::<AstTypeReference>()
    .expect("resultType 应为 AstTypeReference");
  assert_eq!(result_type.name, "string");
}

/// cpp 同名用例。
#[test]
fn parser_short_array_types_are_not_field_names_when_complex() {
  let mut fix = Fixture::default();
  fix.match_parse_error(
    "local n: {string | number: number}",
    "Expected '}' (to close '{' at column 10), got ':'",
    None,
  );
}

/// cpp 同名用例：`{string: number}` 仍按字段名解析，不生成索引器。
#[test]
fn parser_short_array_types_do_not_break_field_names() {
  let mut fix = Fixture::default();
  let block = fix.parse("local n: {string: number}", &ParseOptions::default());

  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 AstStatLocal");
  let var = deref_at(&local.vars, 0).expect("vars[0] 存在");
  let annotation = var
    .annotation
    .as_node::<AstTypeTable>()
    .expect("annotation 应为 AstTypeTable");

  assert_eq!(1, annotation.props.size);
  assert!(annotation.indexer.is_null());

  let prop = elem(&annotation.props, 0);
  assert_eq!(prop.name, "string");
  let prop_type = prop
    .r#type
    .as_node::<AstTypeReference>()
    .expect("prop.type 应为 AstTypeReference");
  assert_eq!(prop_type.name, "number");
}

/// cpp 同名用例。
#[test]
fn parser_short_array_types_must_be_alone() {
  let mut fix = Fixture::default();
  fix.match_parse_error(
    "local n: {string, number}",
    "Expected '}' (to close '{' at column 10), got ','",
    None,
  );
  fix.match_parse_error(
    "local n: {[number]: string, number}",
    "Expected ':' when parsing table field, got '}'",
    None,
  );
  fix.match_parse_error(
    "local n: {x: string, number}",
    "Expected ':' when parsing table field, got '}'",
    None,
  );
  fix.match_parse_error(
    "local n: {x: string, nil}",
    "Expected identifier when parsing table field, got 'nil'",
    None,
  );
}

/// cpp 同名用例：语句末尾位置含分号。
#[test]
fn parser_stat_end_includes_semicolon_position() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\n        local x = 1\n        local y = 2;\n        local z = 3  ;\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(3, block.body.len());

  let stat1 = deref_at(&block.body, 0).expect("body[0] 存在");
  assert!(!stat1.has_semicolon);
  assert_eq!(Position::new(1, 19), stat1.base.location.end);

  let stat2 = deref_at(&block.body, 1).expect("body[1] 存在");
  assert!(stat2.has_semicolon);
  assert_eq!(Position::new(2, 20), stat2.base.location.end);

  let stat3 = deref_at(&block.body, 2).expect("body[2] 存在");
  assert!(stat3.has_semicolon);
  assert_eq!(Position::new(3, 22), stat3.base.location.end);
}

/// cpp 同名用例：缺失的 `)` 只报一条预期错误。
#[test]
fn parser_statement_error_recovery_expected() {
  let mut fix = Fixture::default();
  let errors = catch_parse_errors(|| {
    fix.parse(
      "\nfunction a(a, b) return a + b end\nsome\na(2, 5)\n",
      &ParseOptions::default(),
    );
  });
  assert_eq!(1, errors.get_errors().len());
}

/// cpp 同名用例。
#[test]
fn parser_statement_error_recovery_unexpected() {
  let mut fix = Fixture::default();
  let errors = catch_parse_errors(|| {
    fix.parse("+", &ParseOptions::default());
  });
  assert_eq!(1, errors.get_errors().len());
}

/// cpp `CHECK_THROWS_AS(parse("   -"), std::exception)`：解析必须抛出。
#[test]
fn parser_stop_if_line_ends_with_hyphen() {
  let mut fix = Fixture::default();
  let caught = catch_unwind(AssertUnwindSafe(|| {
    fix.parse("   -", &ParseOptions::default());
  }));
  assert!(caught.is_err(), "Expected ParseErrors to be thrown");
}

/// cpp 同名用例：`foo 'bar'` 解析为单字符串实参调用。
#[test]
fn parser_string_literal_call() {
  let mut fix = Fixture::default();
  let block = fix.parse("do foo 'bar' end", &ParseOptions::default());

  let dob = as_node_at::<AstStatBlock, _>(&block.body, 0).expect("body[0] 应为 AstStatBlock");
  let stc = as_node_at::<AstStatExpr, _>(&dob.body, 0).expect("body[0] 应为 AstStatExpr");
  let ec = stc
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为 AstExprCall");
  assert_eq!(1, ec.args.size);

  let arg = as_node_at::<AstExprConstantString, _>(&ec.args, 0).expect("args[0] 应为字符串字面量");
  assert_eq!(arg.value.as_bytes(), b"bar");
}

/// cpp 同名用例。
#[test]
fn parser_string_literals_broken() {
  let mut fix = Fixture::default();
  let expected = "Malformed string; did you forget to finish it?";
  fix.match_parse_error("return \"", expected, None);
  fix.match_parse_error("return \"\\", expected, None);
  fix.match_parse_error("return \"\r\r", expected, None);
}

/// cpp 同名用例：各类转义序列的解码结果。
#[test]
fn parser_string_literals_escape() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\nreturn\n\"foo\\n\\r\",\n\"foo\\0324\",\n\"foo\\x204\",\n\"foo\\u{20}\",\n\"foo\\u{0451}\"\n",
    &ParseOptions::default(),
  );

  let ret = as_node_at::<AstStatReturn, _>(&block.body, 0).expect("body[0] 应为 AstStatReturn");
  assert_eq!(5, ret.list.size);

  let expected: [&[u8]; 5] = [b"foo\n\r", b"foo 4", b"foo 4", b"foo ", b"foo\xd1\x91"];
  for (index, want) in expected.into_iter().enumerate() {
    let str_ =
      as_node_at::<AstExprConstantString, _>(&ret.list, index).expect("list 元素应为字符串字面量");
    assert_eq!(str_.value.as_bytes(), want);
  }
}

/// cpp 同名用例：`\z` 吞掉全部空白，`\<换行>` 只吞换行本身。
#[test]
fn parser_string_literals_escape_newline() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "return \"foo\\z\n   bar\", \"foo\\\n    bar\", \"foo\\\r\nbar\"",
    &ParseOptions::default(),
  );

  let ret = as_node_at::<AstStatReturn, _>(&block.body, 0).expect("body[0] 应为 AstStatReturn");
  assert_eq!(3, ret.list.size);

  let expected: [&str; 3] = ["foobar", "foo\n    bar", "foo\nbar"];
  for (index, want) in expected.into_iter().enumerate() {
    let str_ =
      as_node_at::<AstExprConstantString, _>(&ret.list, index).expect("list 元素应为字符串字面量");
    assert_eq!(str_.value.as_bytes(), want.as_bytes());
  }
}

/// cpp 同名用例：十六进制 / Unicode / 十进制转义的字节结果。
#[test]
fn parser_string_literals_escapes() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\nreturn\n\"\\xAB\",\n\"\\u{2024}\",\n\"\\121\",\n\"\\1x\",\n\"\\t\",\n\"\\n\"\n",
    &ParseOptions::default(),
  );

  let ret = as_node_at::<AstStatReturn, _>(&block.body, 0).expect("body[0] 应为 AstStatReturn");
  assert_eq!(6, ret.list.size);

  let expected: [&[u8]; 6] = [
    &[0xAB],
    &[0xE2, 0x80, 0xA4],
    &[0x79],
    &[0x01, b'x'],
    b"\t",
    b"\n",
  ];
  for (index, want) in expected.into_iter().enumerate() {
    let str_ =
      as_node_at::<AstExprConstantString, _>(&ret.list, index).expect("list 元素应为字符串字面量");
    assert_eq!(str_.value.as_bytes(), want);
  }
}

/// cpp 同名用例。
#[test]
fn parser_string_literals_escapes_broken() {
  let mut fix = Fixture::default();
  let expected = "String literal contains malformed escape sequence";

  fix.match_parse_error("return \"\\u{\"", expected, None);
  fix.match_parse_error("return \"\\u{FO}\"", expected, None);
  fix.match_parse_error("return \"\\u{123456789}\"", expected, None);
  fix.match_parse_error("return \"\\359\"", expected, None);
  fix.match_parse_error("return \"\\xFO\"", expected, None);
  fix.match_parse_error("return \"\\xF\"", expected, None);
  fix.match_parse_error("return \"\\x\"", expected, None);
}

/// cpp 同名用例。
#[test]
fn parser_table_type_keys_cant_contain_nul() {
  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\n        type Foo = { [\"\\0\"]: number }\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(1, result.errors.len());
  assert_eq!(
    Location::new(Position::new(1, 21), Position::new(1, 22)),
    *result.errors[0].get_location()
  );
  assert_eq!(
    "String literal contains malformed escape sequence or \\0",
    result.errors[0].get_message()
  );
}

/// cpp 同名用例：表类型允许尾随逗号。
#[test]
fn parser_tables_can_have_trailing_separator() {
  let mut fix = Fixture::default();
  let _stat = fix.parse(
    "\n        local zero: number\n        local one: {x: number, y: string, }\n    ",
    &ParseOptions::default(),
  );
}

/// cpp 同名用例：表类型允许分号分隔。
#[test]
fn parser_tables_can_use_semicolons() {
  let mut fix = Fixture::default();
  let _stat = fix.parse(
    "\n        local zero: number\n        local one: {x: number; y: string; }\n    ",
    &ParseOptions::default(),
  );
}

/// cpp 同名用例：索引器与字段可共存。
#[test]
fn parser_tables_should_have_an_indexer_and_keys() {
  let mut fix = Fixture::default();
  let _stat = fix.parse(
    "\n        local t: {\n            [string]: number,\n            f: () -> nil\n        }\n    ",
    &ParseOptions::default(),
  );
}

/// cpp 同名用例。
#[test]
fn parser_two_left_and_right_arrows_but_no_explicit_type_instantiation() {
  let mut fix = Fixture::default();
  let _stat = fix.parse(
    "\n        type A = C<B<<T>() -> T>>\n    ",
    &ParseOptions::default(),
  );
}

/// cpp 同名用例。
#[test]
fn parser_type_alias_error_messages() {
  let mut fix = Fixture::default();
  fix.match_parse_error(
    "type 5 = number",
    "Expected identifier when parsing type name, got '5'",
    None,
  );
  fix.match_parse_error(
    "type A",
    "Expected '=' when parsing type alias, got <eof>",
    None,
  );
  fix.match_parse_error("type A<", "Expected identifier, got <eof>", None);
  fix.match_parse_error(
    "type A<B",
    "Expected '>' (to close '<' at column 7), got <eof>",
    None,
  );
}

/// cpp 同名用例：`type` 作标识符时仍可按调用/赋值解析。
#[test]
fn parser_type_alias_should_not_interfere_with_type_function_call_or_assignment() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\n        type(\"a\")\n        type = nil\n    ",
    &ParseOptions::default(),
  );

  assert!(!block.body.is_empty());
  let first = as_node_at::<AstStatExpr, _>(&block.body, 0).expect("body[0] 应为 AstStatExpr");
  assert!(
    first.expr.as_node::<AstExprCall>().is_some(),
    "expr 应为 AstExprCall"
  );

  let second = deref_at(&block.body, 1).expect("body[1] 存在");
  assert!(second.base.is::<AstStatAssign>());
}

/// cpp 同名用例。
#[test]
fn parser_type_alias_should_point_to_string() {
  let mut fix = Fixture::default();
  let block = fix.parse("\n        type A = string\n    ", &ParseOptions::default());

  assert!(!block.body.is_empty());
  let first = deref_at(&block.body, 0).expect("body[0] 存在");
  assert!(first.base.is::<AstStatTypeAlias>());
}

/// cpp 同名用例。
#[test]
fn parser_type_alias_should_work_when_name_is_also_local() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\n        local A = nil\n        type A = string\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(2, block.body.len());
  let first = deref_at(&block.body, 0).expect("body[0] 存在");
  assert!(first.base.is::<AstStatLocal>());
  let second = deref_at(&block.body, 1).expect("body[1] 存在");
  assert!(second.base.is::<AstStatTypeAlias>());
}

/// cpp 同名用例：两个类型别名的跨度。
#[test]
fn parser_type_alias_span_is_correct() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\n        type Packed1<T...> = (T...) -> (T...)\n        type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(2, block.body.len());

  let t1 =
    as_node_at::<AstStatTypeAlias, _>(&block.body, 0).expect("body[0] 应为 AstStatTypeAlias");
  assert_eq!(
    Location::new(Position::new(1, 8), Position::new(1, 45)),
    t1.base.base.location
  );

  let t2 =
    as_node_at::<AstStatTypeAlias, _>(&block.body, 1).expect("body[1] 应为 AstStatTypeAlias");
  assert_eq!(
    Location::new(Position::new(2, 8), Position::new(2, 75)),
    t2.base.base.location
  );
}

/// cpp 同名用例。
#[test]
fn parser_type_alias_to_a_typeof() {
  let mut fix = Fixture::default();
  let block = fix.parse(
    "\n        type A = typeof(1)\n    ",
    &ParseOptions::default(),
  );

  assert!(!block.body.is_empty());
  let type_alias =
    as_node_at::<AstStatTypeAlias, _>(&block.body, 0).expect("body[0] 应为 AstStatTypeAlias");
  assert_eq!(
    Location::new(Position::new(1, 8), Position::new(1, 26)),
    type_alias.base.base.location
  );
}

/// cpp 同名用例。
#[test]
fn parser_type_assertion_expression() {
  let mut fix = Fixture::default();
  let _stat = fix.parse("local a = something() :: any", &ParseOptions::default());
}

/// cpp 同名用例：`::` 比 `+` 结合更紧，顶层是二元表达式。
#[test]
fn parser_type_assertion_expression_binds_tightly() {
  let mut fix = Fixture::default();
  // cpp 的 `stat->as<AstStatBlock>()` 是恒等下转：根块本身即 AstStatBlock。
  let block = fix.parse(
    "\n        local a = one :: any + two :: any\n    ",
    &ParseOptions::default(),
  );
  assert_eq!(1, block.body.len());

  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!(1, local.values.size);

  let bin = as_node_at::<AstExprBinary, _>(&local.values, 0).expect("values[0] 应为 AstExprBinary");
  assert!(
    bin.left.as_node::<AstExprTypeAssertion>().is_some(),
    "left 应为 AstExprTypeAssertion"
  );
  assert!(
    bin.right.as_node::<AstExprTypeAssertion>().is_some(),
    "right 应为 AstExprTypeAssertion"
  );
}

/// cpp 同名用例：括号类型登记 `CstTypeGroup`，右括号位置为 {1, 24}。
#[test]
fn parser_type_group_with_cst() {
  let mut fix = Fixture::default();
  let options = ParseOptions {
    store_cst_data: true,
    ..Default::default()
  };

  let result = fix.parse_ex("\n        type t = (number)\n    ", &options);
  let root = result.root_block().expect("根块必须存在");
  assert_eq!(1, root.body.len());

  let type_alias =
    as_node_at::<AstStatTypeAlias, _>(&root.body, 0).expect("body[0] 应为 AstStatTypeAlias");
  let group = type_alias
    .type_ptr
    .as_node::<AstTypeGroup>()
    .expect("type 应为 AstTypeGroup");

  let cst_ptr = *result
    .cst_node_map
    .find(&node_key(group))
    .expect("CST 节点必须登记");
  let group_cst = cst_ptr.as_cst::<CstTypeGroup>().expect("应为 CstTypeGroup");
  assert_eq!(Position::new(1, 24), group_cst.close_position);
}

/// cpp 同名用例。
#[test]
fn parser_type_names_can_contain_dots() {
  let mut fix = Fixture::default();
  let _block = fix.parse("local foo: SomeModule.CoolType", &ParseOptions::default());
}

/// cpp 同名用例。
#[test]
fn parser_unfinished_string_literal_types_get_reported_but_parsing_continues() {
  let _limit_guard = lock_parse_limit();
  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\n        type Foo = \"hi\n        print(foo)\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(1, result.errors.len());
  assert_eq!(
    Location::new(Position::new(1, 19), Position::new(1, 22)),
    *result.errors[0].get_location()
  );
  assert_eq!(
    "Malformed string; did you forget to finish it?",
    result.errors[0].get_message()
  );
  assert_eq!(2, result.root_block().expect("根块必须存在").body.len());
}

/// cpp 同名用例。
#[test]
fn parser_unfinished_string_literals_get_reported_but_parsing_continues() {
  let _limit_guard = lock_parse_limit();
  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\n        local foo = \"hi\n        print(foo)\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(1, result.errors.len());
  assert_eq!(
    Location::new(Position::new(1, 20), Position::new(1, 23)),
    *result.errors[0].get_location()
  );
  assert_eq!(
    "Malformed string; did you forget to finish it?",
    result.errors[0].get_message()
  );
  assert_eq!(2, result.root_block().expect("根块必须存在").body.len());
}

/// cpp 同名用例：`@deprecated` 参数形态的四条错误。
#[test]
fn parser_unknown_arguments_for_depricated_is_not_allowed() {
  let mut fix = Fixture::default();

  let result = fix.try_parse(
    "\n@[deprecated({}, \"Very deprecated\")]\nfunction hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );
  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(1, 2), Position::new(1, 12)),
    "@deprecated can be parametrized only by 1 argument",
  );

  let result = fix.try_parse(
    "\n@[deprecated \"Very deprecated\"]\nfunction hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );
  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(1, 13), Position::new(1, 30)),
    "Unknown argument type for @deprecated",
  );

  let result = fix.try_parse(
    "\n@[deprecated{ foo = \"bar\" }]\nfunction hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );
  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(1, 14), Position::new(1, 17)),
    "Unknown argument 'foo' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
  );

  let result = fix.try_parse(
    "\n@[deprecated{ use = 5 }]\nfunction hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );
  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(1, 20), Position::new(1, 21)),
    "Only constant string allowed as value for 'use'",
  );
}

/// cpp 同名用例。
#[test]
fn parser_unparenthesized_function_return_type_list() {
  let mut fix = Fixture::default();
  let expected = "Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?";
  fix.match_parse_error("function foo(): string, number end", expected, None);
  fix.match_parse_error("function foo(): (number) -> string, string", expected, None);

  // cpp 注释：解析失败即抛出
  let _stat = fix.parse(
    "\n        type Vector3MT = {\n            __add: (Vector3MT, Vector3MT) -> Vector3MT,\n            __mul: (Vector3MT, Vector3MT|number) -> Vector3MT\n        }\n    ",
    &ParseOptions::default(),
  );
}

/// cpp 同名用例。
#[test]
fn parser_unsupported_attributes_are_not_allowed() {
  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\n@checked\n    @cool_attribute\nfunction hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );
  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(2, 4), Position::new(2, 19)),
    "Invalid attribute '@cool_attribute'",
  );
}

/// cpp 同名用例：声明语法的可变参数注解，缺注解时报错。
#[test]
fn parser_variadic_definition_parsing() {
  let mut fix = Fixture::default();
  {
    let result = fix.parse_ex(
      "\n        declare function foo(...: string): ...string\n        declare extern type Foo with\n            function a(self, ...: string): ...string\n        end\n    ",
      &ParseOptions::default(),
    );
    assert!(result.root_block().is_some(), "根块必须存在");
  }

  fix.match_parse_error(
    "declare function foo(...)",
    "All declaration parameters must be annotated",
    None,
  );
  fix.match_parse_error(
    "declare extern type Foo with function a(self, ...) end",
    "All declaration parameters aside from 'self' must be annotated",
    None,
  );
}

/// cpp 同名用例。
#[test]
fn parser_variadics_must_be_last() {
  let mut fix = Fixture::default();
  fix.match_parse_error(
    "function foo(): (...number, string) end",
    "Expected ')' (to close '(' at column 17), got ','",
    None,
  );
  fix.match_parse_error(
    "type Foo = (...number, string) -> (...string, number)",
    "Expected ')' (to close '(' at column 12), got ','",
    None,
  );
}

/// cpp `vertical_space`：`a()\vb()` 中的垂直制表符是合法空白。
#[test]
fn parser_vertical_space() {
  let mut fix = Fixture::default();
  let result = fix.parse_ex("a()\u{0B}b()", &ParseOptions::default());
  assert!(result.errors.is_empty());
}

// Source: `tests/Parser.test.cpp:6346`
#[test]
fn parser_parse_if_local_disabled_flag() {
  use ulua_unit_test::records::fixture::Fixture;

  // cpp 侧 `ScopedFastFlag{DebugLuauIfLocalSyntax, false}` 显式关闭旗标；本移植
  // 从未接入 `if local`/`if const` 语法（fflag.rs 亦未登记该旗标），关闭态即
  // 恒常态，故无需（也不能）开 SFF，断言与 cpp 逐字一致。
  let mut fix = Fixture::default();
  fix.match_parse_error(
    "if local x = getValue() then end",
    "Expected identifier when parsing expression, got 'local'",
    None,
  );
}

// 缺口（未移植，对照 `tests/Parser.test.cpp`，tst-r31 逐名清点 29 项名不匹配，
// 复核后真缺 1 例已补于上方，其余判定如下）：
// - parse_if_local / parse_if_const / parse_elseif_local（含 expression/annotation/
//   error/嵌套变体共 22 例，:6248 起，除 :6346 disabled_flag 外全部）——依赖上游
//   `DebugLuauIfLocalSyntax` 门控的 `if local`/`if const` 语法，本 parser 未接入
//   （见 ulua-ast `ast_expr_if_else.rs`、`parser_parse_if_else_expr.rs` 注），且
//   `AstStatIf` 的 `condition_local`/`condition_is_const` 恒 None/false，用例源码
//   无法解析。待功能落地后应补齐。
// - prefixed_type_reference_links_to_local / unknown_prefixed_type_reference_has_no_local
//   / prefixed_type_reference_shadowing（:507/:533/:552）——核心断言是
//   `AstTypeReference::prefixLocal` 经 localMap 回填（cpp Parser.cpp:3349-3361）；
//   本移植 `ast_type_reference.rs` 无 `prefix_local` 字段，补齐需动 src 记录字段 +
//   parser 作用域追踪，禁改生产码，挂账。（同判见 ulua-ast `ast_construct.rs` 尾注。）
// - deprecated_declare_class_syntax_is_rejected（:2446）——cpp 上游已删除
//   `declare class` 接受分支（现 Parser.cpp 仅存 `extern` 分支），Fixture 默认态下
//   `class` 降级为普通名并报 "Expected ':' … got 'Foo'"；本移植
//   `parser_parse_declaration.rs:112` 仍走旧 `declare class` 接受路径（实测该输入
//   0 错），对齐需删旧分支/接旗标，属 src 行为收敛，禁改生产码，挂账。
// - error_on_non_utf8_sequence（:943）、cannot_use_@_as_variable_name（:5084）——
//   tst-r31 复核为**假不匹配**（驼峰→snake 归一化差异）：已由
//   `parser_error_on_non_utf_8_sequence`（parser_part2.rs:180）与
//   `parser_cannot_use_as_variable_name`（parser_part1.rs:255，断言体含
//   `local @blah = 3`）逐字覆盖，非缺口。
