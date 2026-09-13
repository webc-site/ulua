use crate::{
  records::{cst_node::CstNode, cst_stat_repeat::CstStatRepeat, position::Position},
  rtti::CstNodeClass,
};

impl CstStatRepeat {
  pub fn new(until_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      until_position,
    }
  }
}

pub fn cst_stat_repeat_cst_stat_repeat(until_position: Position) -> CstStatRepeat {
  CstStatRepeat::new(until_position)
}
