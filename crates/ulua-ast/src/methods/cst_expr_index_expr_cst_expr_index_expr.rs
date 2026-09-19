use crate::{
  records::{cst_expr_index_expr::CstExprIndexExpr, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstExprIndexExpr {
  pub fn new(open_bracket_position: Position, close_bracket_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      open_bracket_position,
      close_bracket_position,
    }
  }
}
