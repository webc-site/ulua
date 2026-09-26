use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatFunction {
  pub base: CstNode,
  pub function_keyword_position: Position,
}

impl_cst_node_class!(CstStatFunction);
impl_cst_node_new!(CstStatFunction, function_keyword_position: Position);
