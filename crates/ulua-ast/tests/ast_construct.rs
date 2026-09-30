//! AST 构造 + source location 测试：对照 `cpp/tests/Parser.test.cpp` 的
//! `local_with_annotation` / `end_extent_of_functions_unions_and_intersections` /
//! `end_extent_doesnt_consume_comments` 等用例。
//!
//! C++ 侧用 `Fixture::parse` + `stringAtLocation` 断言节点结构与 `location`；
//! 这里以同一策略：解析无错源码，逐节点下转（`ast_node_try_as_ptr`）并核对
//! `Location`，锁定解析器写入 arena 的 `class_index` 与源码区间——后续重构若
//! 破坏构造顺序或位置推进，会在此暴露。

use core::ptr::from_ref;

use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    allocator::Allocator, ast_array::AstArray, ast_expr::AstExpr,
    ast_expr_constant_number::AstExprConstantNumber, ast_local::AstLocal,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, location::Location, node_handle::Nodes,
    parse_options::ParseOptions, parser::Parser,
  },
  rtti::{AstNodeClass, AstNodeView, ast_node_try_as, ast_node_try_as_ptr},
};

/// 解析 `src`（要求无错误）并把根块交给闭包检查。`Box` 钉堆：
/// `AstNameTable`/`Parser` 捕获 `Allocator` 地址，宿主移动即悬垂。
fn with_block<R>(src: &str, f: impl FnOnce(&AstStatBlock) -> R) -> R {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(src, &mut names, &mut allocator, ParseOptions::default());
  assert!(
    result.errors.is_empty(),
    "unexpected parse errors: {:?}",
    result.errors
  );
  assert!(!result.root.is_null(), "parse yielded null root");
  // Safety: result.root 指向上方 arena 分配的存活根块，allocator/names
  // 在闭包返回后才析构；本测试资源独占，闭包借用不与其他访问共存。
  f(unsafe { &*result.root })
}

#[path = "common/loc.rs"]
mod loc_util;

use loc_util::{loc, p};

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
  array[index]
}

/// `Nodes<T>` 句柄数组下标访问：直接交出节点共享引用（非空由构造端证明）。
fn node_at<T>(nodes: &Nodes<T>, index: usize) -> &T {
  nodes.get(index).expect("下标应在数组长度内")
}

// cpp: local_with_annotation —— `local foo: string = "..."`，vars[0].location 覆盖 `foo`
#[test]
fn local_binding_location_spans_name() {
  let src = "local foo: string = \"Hello Types!\"\n";
  with_block(src, |block| {
    assert_eq!(block.body.len(), 1);

    let stat = node_at(&block.body, 0);
    let local =
      ast_node_try_as::<AstStatLocal>(stat.as_ast_node()).expect("首条语句应为 AstStatLocal");

    assert_eq!(local.vars.size, 1);
    let var = elem::<*mut AstLocal>(local.vars, 0);
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`ptr` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
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
    assert_eq!(block.body.len(), 4);

    // line 0 内容 27 字符：8 + 27 = 35
    let s0 = node_at(&block.body, 0);
    assert_eq!(s0.base.location.end, p(0, 35));
    // line 1 内容 34 字符：8 + 34 = 42
    let s1 = node_at(&block.body, 1);
    assert_eq!(s1.base.location.end, p(1, 42));
    // line 2 内容 34 字符：8 + 34 = 42
    let s2 = node_at(&block.body, 2);
    assert_eq!(s2.base.location.end, p(2, 42));
  });
}

// cpp: end_extent_doesnt_consume_comments —— 后续注释不并入语句 location.end
#[test]
fn end_extent_does_not_consume_comments() {
  let src = "        type F = number\n        --comment\n        print('hello')\n";
  with_block(src, |block| {
    assert_eq!(block.body.len(), 2);
    // line 0 内容 `type F = number` 长 15：8 + 15 = 23
    let s0 = node_at(&block.body, 0);
    assert_eq!(s0.base.location.end, p(0, 23));
  });
}

// 一元 `#` 表达式节点 RTTI 下转正确，且 location 覆盖 `#"abc"`
#[test]
fn unary_len_node_class_and_location() {
  use ulua_ast::records::ast_expr_unary::AstExprUnary;
  let src = "local z = #\"abc\"\n";
  with_block(src, |block| {
    let stat = node_at(&block.body, 0);
    let local = ast_node_try_as::<AstStatLocal>(stat.as_ast_node()).expect("应为 AstStatLocal");
    let value = elem::<*mut AstExpr>(local.values, 0);
    let unary =
      unsafe { ast_node_try_as_ptr::<AstExprUnary>(value) }.expect("values[0] 应为 AstExprUnary");
    assert_eq!(unary.base.base.location, loc(0, 10, 0, 16));
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
  // `ast_node_try_as_ptr` 依赖 class_index 命中做下转，构造与 RTTI 必须自洽
  // Safety: ptr 由本用例独占 allocator 刚分配，指向存活 AstExprConstantNumber，
  // 借用仅在本行断言内使用。
  let down = unsafe { ast_node_try_as_ptr::<AstExprConstantNumber>(ptr) }
    .expect("alloc 应写入与类型一致的 class_index");
  assert_eq!(from_ref(down), ptr.cast_const());
  assert_eq!(down.value, 1.5);
}

// 空源码回环 sanity：block.body 长度为 0
#[test]
fn empty_source_yields_empty_block() {
  with_block("", |block| {
    assert_eq!(block.body.len(), 0);
  });
}

// 缺口（未移植，对照 `cpp/tests/Parser.test.cpp`，tst-r05 盘点立案）：
// - prefixed_type_reference_links_to_local / unknown_prefixed_type_reference_has_no_local /
//   prefixed_type_reference_shadowing（:507/:533/:552）——核心断言是 `AstTypeReference` 的
//   `prefixLocal` 与局部作用域联动（cpp Parser.cpp:3349-3361 经 localMap 回填）。本移植
//   `ast_type_reference.rs` 无 `prefix_local` 字段，补齐需 src 记录字段 + parser 作用域
//   追踪，本轮只动 tests，挂账。
// - parse_if_local / parse_if_const / parse_elseif_local（含 expression/annotation/error
//   变体共 23 例，:6248-:6659；其中 parse_if_local_disabled_flag（:6346）为旗标
//   关闭态用例，tst-r31 已补于 ulua-unit-test parser_part6.rs，余 22 例）——依赖
//   上游 `DebugLuauIfLocalSyntax` 门控的 `if local`/`if const` 语法，本 parser
//   未接入（见 ast_expr_if_else.rs、parser_parse_if_else_expr.rs 注释），用例源码
//   无法解析。待功能落地后应补齐。
// - deprecated_declare_class_syntax_is_rejected（:2446）——断言依赖 cpp Fixture 默认
//   环境使 `class` 降级为普通全局名（上游注释所称
//   LuauDisallowExternClassInTypeDefinitions 行为），属 ulua-unit-test Fixture 域，不在
//   本 crate 裸 API 可表达范围。
