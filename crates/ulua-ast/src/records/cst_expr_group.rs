use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprGroup {
  pub base: CstNode,
  pub close_position: Position,
}

impl CstNodeClass for CstExprGroup {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprGroup");
}
