use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_stat_assign::CstStatAssign, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstStatAssign {
  pub fn new(
    vars_comma_positions: AstArray<Position>,
    equals_position: Position,
    values_comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      vars_comma_positions,
      equals_position,
      values_comma_positions,
    }
  }
}

pub fn cst_stat_assign_cst_stat_assign(
  vars_comma_positions: AstArray<Position>,
  equals_position: Position,
  values_comma_positions: AstArray<Position>,
) -> CstStatAssign {
  CstStatAssign::new(
    vars_comma_positions,
    equals_position,
    values_comma_positions,
  )
}
