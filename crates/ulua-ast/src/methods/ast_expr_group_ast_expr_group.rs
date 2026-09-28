use crate::records::{
  ast_expr::AstExpr, ast_expr_group::AstExprGroup, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstExprGroup,
  AstExpr,
  location: Location, expr: Node<AstExpr>,
);
