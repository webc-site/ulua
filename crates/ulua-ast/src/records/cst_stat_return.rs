use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatReturn {
  pub base: CstNode,
  pub comma_positions: AstArray<Position>,
}

impl_cst_node_class!(CstStatReturn);
impl_cst_node_new!(CstStatReturn, comma_positions: AstArray<Position>);
