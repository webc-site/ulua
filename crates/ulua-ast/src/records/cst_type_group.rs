#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeGroup {
  pub base: CstNode,
  pub close_position: Position,
}

impl CstNodeClass for CstTypeGroup {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeGroup");
}
use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
