use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstGenericTypePack {
  pub base: CstNode,
  pub ellipsis_position: Position,
  pub default_equals_position: Position,
}

impl_cst_node_class!(CstGenericTypePack);
impl_cst_node_new!(
  CstGenericTypePack,
  ellipsis_position: Position,
  default_equals_position: Position,
);
