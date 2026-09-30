use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatTypeFunction {
  pub base: CstNode,
  pub type_keyword_position: Position,
  pub function_keyword_position: Position,
}

impl_cst_node_class!(CstStatTypeFunction);
impl_cst_node_new!(
  CstStatTypeFunction,
  type_keyword_position: Position,
  function_keyword_position: Position,
);
