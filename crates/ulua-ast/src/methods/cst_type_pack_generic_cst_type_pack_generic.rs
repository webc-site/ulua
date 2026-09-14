use crate::{
  records::{cst_node::CstNode, cst_type_pack_generic::CstTypePackGeneric, position::Position},
  rtti::CstNodeClass,
};

impl CstTypePackGeneric {
  pub fn new(ellipsis_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      ellipsis_position,
    }
  }
}

pub fn cst_type_pack_generic_cst_type_pack_generic(
  ellipsis_position: Position,
) -> CstTypePackGeneric {
  CstTypePackGeneric::new(ellipsis_position)
}
