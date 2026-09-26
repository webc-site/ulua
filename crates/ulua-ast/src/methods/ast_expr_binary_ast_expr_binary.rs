use crate::records::{
  ast_expr::AstExpr,
  ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
  location::Location,
  node_handle::Node,
};

impl_ast_node_new!(
  AstExprBinary,
  AstExpr,
  location: Location,
  op: AstExprBinaryOp,
  left: Node<AstExpr>,
  right: Node<AstExpr>,
);
