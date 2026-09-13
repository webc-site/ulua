//! Inline port of `luau/tests/AstVisitor.test.cpp` (`TEST_SUITE("AstVisitorTest")`).
//! Verifies the parser + AST visitor traversal end to end: a tracking visitor
//! records every node it's offered, and the tests assert the exact node
//! sequence (and RTTI class) for parsed snippets.
//!
//! Two visitor variants mirror the C++ fixtures:
//! - `Tracking` overrides only the base node hook, so it never descends into
//!   type annotations (the trait's `visit_type` defaults to `false`).
//! - `TrackingWiths` also records/descends types (C++ `AstTypeVisitorTrackingWiths`).

use core::ffi::c_void;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_expr_constant_number::AstExprConstantNumber,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_type_reference::AstTypeReference, ast_visitor::AstVisitor,
    parse_options::ParseOptions, parser::Parser,
  },
  rtti::AstNodeClass,
  visit::AstVisitable,
};

#[derive(Default)]
struct Tracking {
  nodes: Vec<*mut c_void>,
}

impl AstVisitor for Tracking {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.nodes.push(node);
    true
  }
}

#[derive(Default)]
struct TrackingWiths {
  nodes: Vec<*mut c_void>,
}

impl AstVisitor for TrackingWiths {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.nodes.push(node);
    true
  }
  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.nodes.push(node);
    true
  }
}

/// `*mut c_void` AST node -> does it have RTTI class `T`? Every node embeds
/// `AstNode` at offset 0 (the `base` chain), so the cast is sound.
fn is<T: AstNodeClass>(node: *mut c_void) -> bool {
  unsafe { (*(node as *mut AstNode)).is::<T>() }
}

/// Parse `src` (requiring no errors) and hand the root block to `f`.
fn with_block(src: &str, f: impl FnOnce(*mut AstStatBlock)) {
  let mut allocator = Allocator::new();
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
    "unexpected parse errors: {}",
    result.errors.len()
  );
  f(result.root);
}

#[test]
fn type_annotations_are_not_visited() {
  with_block("local a: A<number>\n", |block| {
    let mut v = Tracking::default();
    unsafe { (*block).visit(&mut v) };
    // Only the block and the local — the annotation and its type argument
    // are NOT visited (visit_type defaults to false).
    assert!(is::<AstStatBlock>(v.nodes[0]));
    assert!(is::<AstStatLocal>(v.nodes[1]));
    assert_eq!(v.nodes.len(), 2);
  });
}

#[test]
fn local_two_bindings() {
  with_block("local a, b\n", |block| {
    let mut v = Tracking::default();
    unsafe { (*block).visit(&mut v) };
    assert!(is::<AstStatBlock>(v.nodes[0]));
    assert!(is::<AstStatLocal>(v.nodes[1]));
    assert_eq!(v.nodes.len(), 2);
  });
}

#[test]
fn local_two_annotated_bindings() {
  with_block("local a: A, b: B<number>\n", |block| {
    let mut v = TrackingWiths::default();
    unsafe { (*block).visit(&mut v) };
    assert!(is::<AstStatBlock>(v.nodes[0]));
    assert!(is::<AstStatLocal>(v.nodes[1]));
    assert!(is::<AstTypeReference>(v.nodes[2]));
    assert!(is::<AstTypeReference>(v.nodes[3]));
    assert!(is::<AstTypeReference>(v.nodes[4]));
  });
}

#[test]
fn local_two_annotated_bindings_with_two_values() {
  with_block("local a: A, b: B<number> = 1, 2\n", |block| {
    let mut v = TrackingWiths::default();
    unsafe { (*block).visit(&mut v) };
    assert!(is::<AstStatBlock>(v.nodes[0]));
    assert!(is::<AstStatLocal>(v.nodes[1]));
    assert!(is::<AstTypeReference>(v.nodes[2]));
    assert!(is::<AstTypeReference>(v.nodes[3]));
    assert!(is::<AstTypeReference>(v.nodes[4]));
    assert!(is::<AstExprConstantNumber>(v.nodes[5]));
    assert!(is::<AstExprConstantNumber>(v.nodes[6]));
  });
}
