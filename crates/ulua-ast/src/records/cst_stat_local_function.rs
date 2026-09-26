use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatLocalFunction {
  pub base: CstNode,
  pub local_keyword_position: Position,
  pub function_keyword_position: Position,
}

impl_cst_node_class!(CstStatLocalFunction);
impl_cst_node_new!(
  CstStatLocalFunction,
  local_keyword_position: Position,
  function_keyword_position: Position,
);
