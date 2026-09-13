use crate::{
  records::{
    ast_expr::AstExpr,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprUnary {
  pub fn new(location: Location, op: AstExprUnaryOp, expr: *mut AstExpr) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      op,
      expr,
    }
  }
}

pub fn ast_expr_unary_ast_expr_unary(
  location: Location,
  op: AstExprUnaryOp,
  expr: *mut AstExpr,
) -> AstExprUnary {
  AstExprUnary::new(location, op, expr)
}
