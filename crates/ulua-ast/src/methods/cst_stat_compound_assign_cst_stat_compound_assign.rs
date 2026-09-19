use crate::{
  records::{
    cst_node::CstNode, cst_stat_compound_assign::CstStatCompoundAssign, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstStatCompoundAssign {
  pub fn new(op_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      op_position,
    }
  }
}
