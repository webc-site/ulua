use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, cst_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeUnion {
  pub base: CstNode,
  pub leading_position: Position,
  pub separator_positions: AstArray<Position>,
}

impl CstNodeClass for CstTypeUnion {
  const CLASS_INDEX: i32 = cst_rtti_index("CstTypeUnion");
}
