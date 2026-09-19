use crate::{
  records::{cst_expr_group::CstExprGroup, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstExprGroup {
  pub fn new(close_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: Self::CLASS_INDEX,
      },
      close_position,
    }
  }
}
