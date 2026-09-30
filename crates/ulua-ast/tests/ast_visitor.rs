//! Inline port of `luau/tests/AstVisitor.test.cpp` (`TEST_SUITE("AstVisitorTest")`).
//! Verifies the parser + AST visitor traversal end to end: a tracking visitor
//! records every node it's offered, and the tests assert the exact node
//! sequence (and RTTI class) for parsed snippets.
//!
//! Two visitor variants mirror the C++ fixtures:
//! - `Tracking` overrides only the base node hook, so it never descends into
//!   type annotations (the trait's `visit_type` defaults to `false`).
//! - `TrackingWiths` also records/descends types (C++ `AstTypeVisitorTrackingWiths`).
//!
//! 类型化 hook 落地后，跟踪者只需记录 `class_index`（RTTI 事实直接在
//! `&mut AstNode` 上读取），测试侧不再出现裸指针与 `c_void`。

use ulua_ast::{
  records::{
    allocator::Allocator, ast_expr_constant_number::AstExprConstantNumber,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_type::AstType, ast_type_reference::AstTypeReference,
    ast_visitor::AstVisitor, parse_options::ParseOptions, parser::Parser,
  },
  rtti::AstNodeClass,
  visit::AstVisitable,
};

#[derive(Default)]
struct Tracking {
  classes: Vec<i32>,
}

impl AstVisitor for Tracking {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    self.classes.push(node.class_index);
    true
  }
}

#[derive(Default)]
struct TrackingWiths {
  classes: Vec<i32>,
}

impl AstVisitor for TrackingWiths {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    self.classes.push(node.class_index);
    true
  }
  fn visit_type(&mut self, node: &mut AstType) -> bool {
    self.classes.push(node.base.class_index);
    true
  }
}

/// 记录的第 `i` 个节点是否具有 RTTI 类 `T`（越界即 false，断言处另有长度检查）。
fn is<T: AstNodeClass>(classes: &[i32], i: usize) -> bool {
  classes.get(i) == Some(&T::CLASS_INDEX)
}

/// Parse `src` (requiring no errors) and hand the root block to `f`.
fn with_block(src: &str, f: impl FnOnce(&mut AstStatBlock)) {
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(src, &mut names, &mut allocator, ParseOptions::default());
  assert!(
    result.errors.is_empty(),
    "unexpected parse errors: {}",
    result.errors.len()
  );
  assert!(!result.root.is_null(), "parse yielded null root");
  // Safety: result.root 指向上方 arena 分配的存活根块，allocator/names
  // 在闭包返回后才析构；本测试资源独占，闭包内的可变借用无并发访问。
  f(unsafe { &mut *result.root });
}

#[test]
fn type_annotations_are_not_visited() {
  with_block("local a: A<number>\n", |block| {
    let mut v = Tracking::default();
    block.visit(&mut v);
    // Only the block and the local — the annotation and its type argument
    // are NOT visited (visit_type defaults to false).
    assert!(is::<AstStatBlock>(&v.classes, 0));
    assert!(is::<AstStatLocal>(&v.classes, 1));
    assert_eq!(v.classes.len(), 2);
  });
}

#[test]
fn local_two_bindings() {
  with_block("local a, b\n", |block| {
    let mut v = Tracking::default();
    block.visit(&mut v);
    assert!(is::<AstStatBlock>(&v.classes, 0));
    assert!(is::<AstStatLocal>(&v.classes, 1));
    assert_eq!(v.classes.len(), 2);
  });
}

#[test]
fn local_two_annotated_bindings() {
  with_block("local a: A, b: B<number>\n", |block| {
    let mut v = TrackingWiths::default();
    block.visit(&mut v);
    assert!(is::<AstStatBlock>(&v.classes, 0));
    assert!(is::<AstStatLocal>(&v.classes, 1));
    assert!(is::<AstTypeReference>(&v.classes, 2));
    assert!(is::<AstTypeReference>(&v.classes, 3));
    assert!(is::<AstTypeReference>(&v.classes, 4));
  });
}

#[test]
fn local_two_annotated_bindings_with_two_values() {
  with_block("local a: A, b: B<number> = 1, 2\n", |block| {
    let mut v = TrackingWiths::default();
    block.visit(&mut v);
    assert!(is::<AstStatBlock>(&v.classes, 0));
    assert!(is::<AstStatLocal>(&v.classes, 1));
    assert!(is::<AstTypeReference>(&v.classes, 2));
    assert!(is::<AstTypeReference>(&v.classes, 3));
    assert!(is::<AstTypeReference>(&v.classes, 4));
    assert!(is::<AstExprConstantNumber>(&v.classes, 5));
    assert!(is::<AstExprConstantNumber>(&v.classes, 6));
  });
}

// b3-T4（守护 b3-H4）：cpp Ast.cpp:1314-1317 —— AstTypeOptional::visit 只回调
// 自身、不递归（cpp 该节点无 type 成员）。`string?` 解析为 union[string, optional]，
// 访问者看到的完整序列即 block → local → union → ref → opt，opt 之后无多余节点。
#[test]
fn optional_type_annotation_is_not_recursed() {
  use ulua_ast::records::{
    ast_type_optional::AstTypeOptional, ast_type_reference::AstTypeReference,
    ast_type_union::AstTypeUnion,
  };

  with_block("local x: string?\n", |block| {
    let mut v = TrackingWiths::default();
    block.visit(&mut v);
    assert_eq!(v.classes.len(), 5, "unexpected traversal shape on `T?`");
    assert!(is::<AstStatBlock>(&v.classes, 0));
    assert!(is::<AstStatLocal>(&v.classes, 1));
    assert!(is::<AstTypeUnion>(&v.classes, 2));
    assert!(is::<AstTypeReference>(&v.classes, 3));
    assert!(is::<AstTypeOptional>(&v.classes, 4));
    // optional 必须是最后一个节点：若其后再出现节点，即为对 `T?` 内层的多余递归。
  });
}
