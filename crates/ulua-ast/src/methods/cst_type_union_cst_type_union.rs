use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_union::CstTypeUnion, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypeUnion {
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
