use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypePackGeneric {
  pub base: CstNode,
  pub ellipsis_position: Position,
}

impl_cst_node_class!(CstTypePackGeneric);
impl_cst_node_new!(CstTypePackGeneric, ellipsis_position: Position);
