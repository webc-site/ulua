use crate::{
  records::{cst_node::CstNode, cst_type_typeof::CstTypeTypeof, position::Position},
  rtti::CstNodeClass,
};

impl CstTypeTypeof {
  pub fn new(open_position: Position, close_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      open_position,
      close_position,
    }
  }
}
