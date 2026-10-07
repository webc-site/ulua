use ulua_ast::{
  records::{ast_expr_table::AstExprTable, ast_node::AstNode},
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{node::Node, shape_visitor::ShapeVisitor, table_shape::TableShape};

/// 对应 cpp `predictTableShapes`（cpp/Compiler/src/TableShape.cpp:148）：以
/// `ShapeVisitor` 只读遍历 `root`（调用方持有的 arena 存活编译入口树），返回预测出的
/// `TableShape` 映射（以 `*mut AstExprTable` 地址为键，键全部来自子树内的表字面量节点）。
pub(crate) fn predict_table_shapes(
  root: &mut AstNode,
) -> DenseHashMap<Node<AstExprTable>, TableShape> {
  let mut shapes = DenseHashMap::default();
  {
    let mut visitor = ShapeVisitor::new(&mut shapes);

    // `dispatch_node` 以调用方独占的 `&mut root` 沿 parser 接线子指针只读遍历，
    // `&mut visitor` 独占内部 `shapes` map，与 AST 无别名交集。
    dispatch_node(root, &mut visitor);
  }
  shapes
}
