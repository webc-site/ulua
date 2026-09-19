use crate::{
  records::{cst_node::CstNode, cst_stat_type_function::CstStatTypeFunction, position::Position},
  rtti::CstNodeClass,
};

impl CstStatTypeFunction {
  pub fn new(type_keyword_position: Position, function_keyword_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      type_keyword_position,
      function_keyword_position,
    }
  }
}
