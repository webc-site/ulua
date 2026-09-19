use crate::{
  records::{cst_node::CstNode, cst_stat_do::CstStatDo, position::Position},
  rtti::CstNodeClass,
};

impl CstStatDo {
  pub fn new(stats_start_position: Position, end_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      stats_start_position,
      end_position,
    }
  }
}
