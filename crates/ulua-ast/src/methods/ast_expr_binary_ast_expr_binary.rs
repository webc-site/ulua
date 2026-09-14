use crate::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprBinary {
  pub fn new(
    location: Location,
    op: AstExprBinaryOp,
    left: *mut AstExpr,
    right: *mut AstExpr,
  ) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      op,
      left,
      right,
    }
  }
}

pub fn ast_expr_binary_ast_expr_binary(
  location: Location,
  op: AstExprBinaryOp,
  left: *mut AstExpr,
  right: *mut AstExpr,
) -> AstExprBinary {
  AstExprBinary::new(location, op, left, right)
}
