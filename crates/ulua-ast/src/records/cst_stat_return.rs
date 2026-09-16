use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatReturn {
  pub base: CstNode,
  pub comma_positions: AstArray<Position>,
}

impl CstNodeClass for CstStatReturn {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatReturn");
}
