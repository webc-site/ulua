use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_stat_return::CstStatReturn, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstStatReturn {
  pub fn new(comma_positions: AstArray<Position>) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      comma_positions,
    }
  }
}

pub fn cst_stat_return_cst_stat_return(comma_positions: AstArray<Position>) -> CstStatReturn {
  CstStatReturn::new(comma_positions)
}
