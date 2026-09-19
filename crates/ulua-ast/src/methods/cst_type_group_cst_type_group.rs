use crate::{
  records::{cst_node::CstNode, cst_type_group::CstTypeGroup, position::Position},
  rtti::CstNodeClass,
};

impl CstTypeGroup {
  pub fn new(close_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      close_position,
    }
  }
}
