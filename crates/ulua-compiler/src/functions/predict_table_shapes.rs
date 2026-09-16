use ulua_ast::{
  records::{ast_expr_table::AstExprTable, ast_node::AstNode},
  visit::ast_node_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{shape_visitor::ShapeVisitor, table_shape::TableShape};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn predict_table_shapes(
  shapes: &mut DenseHashMap<*mut AstExprTable, TableShape>,
  root: *mut AstNode,
) {
  let mut visitor = ShapeVisitor::new(shapes);

  unsafe {
    ast_node_visit(root, &mut visitor);
  }
}
