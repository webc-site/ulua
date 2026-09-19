//! AST 构造 + source location 测试：对照 `cpp/tests/Parser.test.cpp` 的
//! `local_with_annotation` / `end_extent_of_functions_unions_and_intersections` /
//! `end_extent_doesnt_consume_comments` 等用例。
//!
//! C++ 侧用 `Fixture::parse` + `stringAtLocation` 断言节点结构与 `location`；
//! 这里以同一策略：解析无错源码，逐节点下转（`ast_node_as`）并核对
//! `Location`，锁定解析器写入 arena 的 `class_index` 与源码区间——后续重构若
//! 破坏构造顺序或位置推进，会在此暴露。

use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    allocator::Allocator, ast_array::AstArray, ast_expr::AstExpr,
    ast_expr_constant_number::AstExprConstantNumber, ast_local::AstLocal,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal, location::Location,
    parse_options::ParseOptions, parser::Parser, position::Position,
  },
  rtti::{AstNodeClass, ast_node_as},
};

/// 解析 `src`（要求无错误）并把根块交给闭包检查。`Box` 钉堆：
/// `AstNameTable`/`Parser` 捕获 `Allocator` 地址，宿主移动即悬垂。
fn with_block<R>(src: &str, f: impl FnOnce(*mut AstStatBlock) -> R) -> R {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    src,
    src.len(),
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  );
  assert!(
    result.errors.is_empty(),
    "unexpected parse errors: {:?}",
    result.errors
  );
  assert!(!result.root.is_null(), "parse yielded null root");
  f(result.root)
}

fn p(line: u32, column: u32) -> Position {
  Position { line, column }
}

fn loc(bl: u32, bc: u32, el: u32, ec: u32) -> Location {
  Location {
    begin: p(bl, bc),
    end: p(el, ec),
  }
}

/// 取 location 对应源码子串（cpp `stringAtLocation` 的单行简化版）。
fn text_at(src: &str, location: Location) -> &str {
  assert_eq!(
    location.begin.line, location.end.line,
    "跨行 location 需扩展此 helper"
  );
  let line = src.lines().nth(location.begin.line as usize).unwrap();
  &line[location.begin.column as usize..location.end.column as usize]
}

/// `AstArray<T>` 下标访问（元素均为 arena 裸指针，Copy）。
fn elem<T: Copy>(array: AstArray<T>, index: usize) -> T {
  array.as_slice()[index]
}

/// `*mut AstStat`（或其派生节点基址）转 `*mut AstNode`：所有节点首字段即基类，
/// 直至 `AstNode`，故指针值不变。
fn as_node<T>(ptr: *mut T) -> *mut AstNode {
  ptr.cast()
}

// cpp: local_with_annotation —— `local foo: string = "..."`，vars[0].location 覆盖 `foo`
#[test]
fn local_binding_location_spans_name() {
  let src = "local foo: string = \"Hello Types!\"\n";
  with_block(src, |block| {
    let block = unsafe { &*block };
    assert_eq!(block.body.size, 1);

    let stat = elem::<*mut AstStat>(block.body, 0);
    let local = unsafe { ast_node_as::<AstStatLocal>(as_node(stat)) };
    assert!(!local.is_null(), "首条语句应为 AstStatLocal");
    let local = unsafe { &*local };

    assert_eq!(local.vars.size, 1);
    let var = elem::<*mut AstLocal>(local.vars, 0);
    assert_eq!(text_at(src, unsafe { (*var).location }), "foo");
  });
}

// cpp: end_extent_of_functions_unions_and_intersections —— 类型别名语句的
// location.end 精确到行尾（列 = 内容列，不含尾部空白）。
// cpp 的 R"(...)" 串以换行开头，故其断言行号比本测试（无前置换行）大 1。
#[test]
fn type_alias_end_extent_excludes_trailing_whitespace() {
  let src = "        type F = (string) -> string\n        type G = string | number | boolean\n        type H = string & number & boolean\n        print('hello')\n";
  with_block(src, |block| {
    let block = unsafe { &*block };
    assert_eq!(block.body.size, 4);

    // line 0 内容 27 字符：8 + 27 = 35
    let s0 = elem::<*mut AstStat>(block.body, 0);
    assert_eq!(unsafe { (*s0).base.location }.end, p(0, 35));
    // line 1 内容 34 字符：8 + 34 = 42
    let s1 = elem::<*mut AstStat>(block.body, 1);
    assert_eq!(unsafe { (*s1).base.location }.end, p(1, 42));
    // line 2 内容 34 字符：8 + 34 = 42
    let s2 = elem::<*mut AstStat>(block.body, 2);
    assert_eq!(unsafe { (*s2).base.location }.end, p(2, 42));
  });
}

// cpp: end_extent_doesnt_consume_comments —— 后续注释不并入语句 location.end
#[test]
fn end_extent_does_not_consume_comments() {
  let src = "        type F = number\n        --comment\n        print('hello')\n";
  with_block(src, |block| {
    let block = unsafe { &*block };
    assert_eq!(block.body.size, 2);
    // line 0 内容 `type F = number` 长 15：8 + 15 = 23
    let s0 = elem::<*mut AstStat>(block.body, 0);
    assert_eq!(unsafe { (*s0).base.location }.end, p(0, 23));
  });
}

// 一元 `#` 表达式节点 RTTI 下转正确，且 location 覆盖 `#"abc"`
#[test]
fn unary_len_node_class_and_location() {
  use ulua_ast::records::ast_expr_unary::AstExprUnary;
  let src = "local z = #\"abc\"\n";
  with_block(src, |block| {
    let block = unsafe { &*block };
    let stat = elem::<*mut AstStat>(block.body, 0);
    let local = unsafe { &*ast_node_as::<AstStatLocal>(as_node(stat)) };
    let value = elem::<*mut AstExpr>(local.values, 0);
    let unary = unsafe { ast_node_as::<AstExprUnary>(as_node(value)) };
    assert!(!unary.is_null(), "values[0] 应为 AstExprUnary");
    assert_eq!(unsafe { (*unary).base.base.location }, loc(0, 10, 0, 16));
  });
}

// 构造 arena 节点：`Allocator::alloc` 写入的 class_index 与类型一致（RTTI 契约）
#[test]
fn allocator_constructs_node_with_class_index() {
  let mut allocator = Box::new(Allocator::new());
  let ptr = allocator.alloc(AstExprConstantNumber {
    base: AstExpr {
      base: AstNode {
        class_index: AstExprConstantNumber::CLASS_INDEX,
        location: loc(0, 0, 0, 3),
      },
    },
    value: 1.5,
    parse_result: ConstantNumberParseResult::Ok,
  });
  assert!(!ptr.is_null());
  // `ast_node_as` 依赖 class_index 命中做下转，构造与 RTTI 必须自洽
  let down = unsafe { ast_node_as::<AstExprConstantNumber>(as_node(ptr)) };
  assert_eq!(down, ptr);
  assert_eq!(unsafe { (*down).value }, 1.5);
}

// 空源码回环 sanity：block.body.size == 0
#[test]
fn empty_source_yields_empty_block() {
  with_block("", |block| {
    assert_eq!(unsafe { (*block).body.size }, 0);
  });
}
