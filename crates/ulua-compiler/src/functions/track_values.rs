//! Source: `Compiler/src/ValueTracking.cpp:104`
//!
//! `trackValues`——在 AST 根上跑 `ValueVisitor`，为每个 local 记录其
//! 初始化式与是否被重新赋值（以及哪些全局被写入）。C++ visitor 持
//! `globals`/`variables` 的引用；Rust `ValueVisitor` 拥有它们，故构造时
//! 移入两张表、遍历结束后再把填充好的结果移出。

use ulua_ast::{
  records::{ast_local::AstLocal, ast_name::AstName, ast_node::AstNode},
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  records::{node::Node, value_visitor::ValueVisitor, variable::Variable},
};

/// 对应 cpp `trackValues`（cpp/Compiler/src/ValueTracking.cpp:177）：在 `root`
/// （编译入口 `AstStatBlock`，调用方持有的存活节点）上跑 `ValueVisitor`，为每个
/// local 记录初始化式/是否被重赋值，并标记被写入的全局；`variables`/`class_locals`
/// 的 `Node<AstLocal>` 句柄键即 parser 接线节点，长寿于本遍历由同一 arena 约束保证。
pub(crate) fn track_values(
  globals: &mut DenseHashMap<AstName, Global>,
  variables: &mut DenseHashMap<Node<AstLocal>, Variable>,
  class_locals: &mut DenseHashMap<AstName, Node<AstLocal>>,
  root: &mut AstNode,
) {
  let mut visitor = ValueVisitor::new(globals, variables, class_locals);

  // `dispatch_node` 沿 RTTI 下转并回调 visitor（安全门面，收 `&mut AstNode`）；
  // `&mut visitor` 独占借用传入三 map，与 AST 无别名交集。
  dispatch_node(root, &mut visitor);

  *globals = visitor.globals;
  *variables = visitor.variables;
  *class_locals = visitor.class_locals;
}
