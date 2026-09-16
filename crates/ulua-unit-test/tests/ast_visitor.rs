//! Port of `cpp/tests/AstVisitor.test.cpp`（117 行，4 个 TEST_CASE）。
//!
//! 被测对象：`ulua_ast::visit::AstVisitable` 泛型遍历 + `AstVisitor` trait
//! （对应 C++ `AstNode::visit(AstVisitor*)` 虚遍历），解析走
//! `ulua_unit_test::records::fixture::Fixture::parse`。
//!
//! 移植说明：
//! - C++ `AstVisitorTracking` 的析构断言"每个记录的节点都被检查过"镜像为
//!   测试收尾的 `assert_all_seen`（Rust 的 `Drop` 不适合 panic）。
//! - 孤儿规则禁止在测试 crate 为 `AstVisitorTracking` /
//!   `AstTypeVisitorTrackingWiths` 实现 `AstVisitor`，故以 newtype 包装复用
//!   其 `visited_nodes` / `seen` 记账与 `operator_index` 断言。
//! - `v[i]->is<T>()` 镜像为 `is::<T>(v.index(i))`（RTTI class index）。

use core::ffi::c_void;
use std::collections::HashSet;

use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber, ast_node::AstNode,
    ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal, ast_type::AstType,
    ast_type_reference::AstTypeReference, ast_visitor::AstVisitor, parse_options::ParseOptions,
  },
  rtti::AstNodeClass,
  visit::AstVisitable,
};
use ulua_unit_test::records::{
  ast_type_visitor_tracking_withs::AstTypeVisitorTrackingWiths,
  ast_visitor_tracking::AstVisitorTracking, fixture::Fixture,
};

/// `v[i]->is<T>()`：节点指针的 RTTI 类型判定。每个节点在偏移 0 处内嵌
/// `AstNode` 基类，重解释是安全的。
fn is<T: AstNodeClass>(node: *mut AstNode) -> bool {
  unsafe { (*node).is::<T>() }
}

/// C++ `~AstVisitorTracking` 的 `CHECK(seen.size() == visitedNodes.size())`：
/// 所有被记录的节点都必须在断言中被取用过。
fn assert_all_seen(tracking: &AstVisitorTracking) {
  assert_eq!(
    tracking.seen.len(),
    tracking.visited_nodes.len(),
    "Seen {} nodes but got {}",
    tracking.seen.len(),
    tracking.visited_nodes.len()
  );
}

/// C++ `AstVisitorTracking`：基类 `visit(AstNode*)` 记录每个经过的节点。
struct Tracking(AstVisitorTracking);

impl Tracking {
  fn new() -> Self {
    Tracking(AstVisitorTracking {
      visited_nodes: Vec::new(),
      seen: HashSet::new(),
    })
  }

  /// C++ `operator[]`：取第 i 个记录节点并记账。
  fn index(&mut self, i: usize) -> *mut AstNode {
    self.0.operator_index(i)
  }

  fn assert_all_seen(&self) {
    assert_all_seen(&self.0);
  }
}

impl AstVisitor for Tracking {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.0.visit(node as *mut AstNode)
  }
}

/// C++ `AstTypeVisitorTrackingWiths`：在 Tracking 之上把 `visit(AstType*)`
/// 也路由进基类记录。
struct TrackingWiths(AstTypeVisitorTrackingWiths);

impl TrackingWiths {
  fn new() -> Self {
    TrackingWiths(AstTypeVisitorTrackingWiths {
      base: AstVisitorTracking {
        visited_nodes: Vec::new(),
        seen: HashSet::new(),
      },
    })
  }

  fn index(&mut self, i: usize) -> *mut AstNode {
    self.0.base.operator_index(i)
  }

  fn assert_all_seen(&self) {
    assert_all_seen(&self.0.base);
  }
}

impl AstVisitor for TrackingWiths {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.0.base.visit(node as *mut AstNode)
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.0.visit(node as *mut AstType)
  }
}

mod type_annotations_are_not_visited {
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstVisitor.test.cpp:55:TypeAnnotationsAreNotVisited`
  //! Source: `tests/AstVisitor.test.cpp:55-68`

  use super::{AstStatBlock, AstStatLocal, AstVisitable, Fixture, ParseOptions, Tracking, is};

  #[test]
  fn type_annotations_are_not_visited() {
    let mut fix = Fixture::default();
    let block = fix.parse("\n        local a: A<number>\n    ", &ParseOptions::new());

    let mut v = Tracking::new();
    unsafe { (*block).visit(&mut v) };

    assert!(is::<AstStatBlock>(v.index(0)));
    assert!(is::<AstStatLocal>(v.index(1)));
    // We should not have v[2] that points to the annotation
    // We should not have v[3] that points to the type argument 'number' in A.
    v.assert_all_seen();
  }
}

mod local_two_bindings {
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstVisitor.test.cpp:70:LocalTwoBindings`
  //! Source: `tests/AstVisitor.test.cpp:70-81`

  use super::{AstStatBlock, AstStatLocal, AstVisitable, Fixture, ParseOptions, Tracking, is};

  #[test]
  fn local_two_bindings() {
    let mut fix = Fixture::default();
    let block = fix.parse("\n        local a, b\n    ", &ParseOptions::new());

    let mut v = Tracking::new();
    unsafe { (*block).visit(&mut v) };

    assert!(is::<AstStatBlock>(v.index(0)));
    assert!(is::<AstStatLocal>(v.index(1)));
    v.assert_all_seen();
  }
}

mod local_two_annotated_bindings {
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstVisitor.test.cpp:83:LocalTwoAnnotatedBindings`
  //! Source: `tests/AstVisitor.test.cpp:83-97`

  use super::{
    AstStatBlock, AstStatLocal, AstTypeReference, AstVisitable, Fixture, ParseOptions,
    TrackingWiths, is,
  };

  #[test]
  fn local_two_annotated_bindings() {
    let mut fix = Fixture::default();
    let block = fix.parse(
      "\n        local a: A, b: B<number>\n    ",
      &ParseOptions::new(),
    );

    let mut v = TrackingWiths::new();
    unsafe { (*block).visit(&mut v) };

    assert!(is::<AstStatBlock>(v.index(0)));
    assert!(is::<AstStatLocal>(v.index(1)));
    assert!(is::<AstTypeReference>(v.index(2)));
    assert!(is::<AstTypeReference>(v.index(3)));
    assert!(is::<AstTypeReference>(v.index(4)));
    v.assert_all_seen();
  }
}

mod local_two_annotated_bindings_with_two_values {
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstVisitor.test.cpp:99:LocalTwoAnnotatedBindingsWithTwoValues`
  //! Source: `tests/AstVisitor.test.cpp:99-115`

  use super::{
    AstExprConstantNumber, AstStatBlock, AstStatLocal, AstTypeReference, AstVisitable, Fixture,
    ParseOptions, TrackingWiths, is,
  };

  #[test]
  fn local_two_annotated_bindings_with_two_values() {
    let mut fix = Fixture::default();
    let block = fix.parse(
      "\n        local a: A, b: B<number> = 1, 2\n    ",
      &ParseOptions::new(),
    );

    let mut v = TrackingWiths::new();
    unsafe { (*block).visit(&mut v) };

    assert!(is::<AstStatBlock>(v.index(0)));
    assert!(is::<AstStatLocal>(v.index(1)));
    assert!(is::<AstTypeReference>(v.index(2)));
    assert!(is::<AstTypeReference>(v.index(3)));
    assert!(is::<AstTypeReference>(v.index(4)));
    assert!(is::<AstExprConstantNumber>(v.index(5)));
    assert!(is::<AstExprConstantNumber>(v.index(6)));
    v.assert_all_seen();
  }
}
