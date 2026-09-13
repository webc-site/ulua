use crate::{
  records::{cst_node::CstNode, cst_stat_local_function::CstStatLocalFunction, position::Position},
  rtti::CstNodeClass,
};

impl CstStatLocalFunction {
  pub fn new(local_keyword_position: Position, function_keyword_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      local_keyword_position,
      function_keyword_position,
    }
  }
}
