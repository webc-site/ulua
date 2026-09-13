use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_intersection::CstTypeIntersection,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypeIntersection {
  pub fn new(leading_position: Position, separator_positions: AstArray<Position>) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      leading_position,
      separator_positions,
    }
  }
}

pub fn cst_type_intersection_cst_type_intersection(
  leading_position: Position,
  separator_positions: AstArray<Position>,
) -> CstTypeIntersection {
  CstTypeIntersection::new(leading_position, separator_positions)
}
