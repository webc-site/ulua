use crate::records::{
  ast_expr::AstExpr,
  ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  location::Location,
  node_handle::Node,
};

impl_ast_node_new!(
  AstExprUnary,
  AstExpr,
  location: Location, op: AstExprUnaryOp, expr: Node<AstExpr>,
);
