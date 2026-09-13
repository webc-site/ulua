use crate::{
  records::{cst_generic_type_pack::CstGenericTypePack, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstGenericTypePack {
  pub fn new(ellipsis_position: Position, default_equals_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      ellipsis_position,
      default_equals_position,
    }
  }
}

pub fn cst_generic_type_pack_cst_generic_type_pack(
  ellipsis_position: Position,
  default_equals_position: Position,
) -> CstGenericTypePack {
  CstGenericTypePack::new(ellipsis_position, default_equals_position)
}
