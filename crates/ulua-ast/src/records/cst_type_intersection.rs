use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeIntersection {
  pub base: CstNode,
  pub leading_position: Position,
  pub separator_positions: AstArray<Position>,
}

impl CstNodeClass for CstTypeIntersection {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeIntersection");
}
