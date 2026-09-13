use crate::{
  records::{cst_generic_type::CstGenericType, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstGenericType {
  pub fn new(default_equals_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      default_equals_position,
    }
  }
}

pub fn cst_generic_type_cst_generic_type(default_equals_position: Position) -> CstGenericType {
  CstGenericType::new(default_equals_position)
}
