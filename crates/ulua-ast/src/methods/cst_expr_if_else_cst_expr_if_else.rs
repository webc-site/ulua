use crate::{
  records::{cst_expr_if_else::CstExprIfElse, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstExprIfElse {
  pub fn new(then_position: Position, else_position: Position, is_else_if: bool) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      then_position,
      else_position,
      is_else_if,
    }
  }
}

pub fn cst_expr_if_else_cst_expr_if_else(
  then_position: Position,
  else_position: Position,
  is_else_if: bool,
) -> CstExprIfElse {
  CstExprIfElse::new(then_position, else_position, is_else_if)
}
