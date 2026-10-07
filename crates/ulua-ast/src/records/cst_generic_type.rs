use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstGenericType {
  pub base: CstNode,
  pub default_equals_position: Position,
}

impl_cst_node_class!(CstGenericType);
impl_cst_node_new!(CstGenericType, default_equals_position: Position);
