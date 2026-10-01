use crate::records::{
  ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstExprIndexExpr,
  AstExpr,
  location: Location, expr: Node<AstExpr>, index: Node<AstExpr>,
);
