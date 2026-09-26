use crate::records::{
  ast_expr::AstExpr,
  ast_expr_if_else::AstExprIfElse,
  ast_local::AstLocal,
  location::Location,
  node_handle::{Node, OptNode},
};

impl_ast_node_new!(
  AstExprIfElse,
  AstExpr,
  location: Location,
  condition: Node<AstExpr>,
  has_then: bool,
  true_expr: Node<AstExpr>,
  has_else: bool,
  false_expr: Node<AstExpr>,
  condition_local: OptNode<AstLocal>,
);
