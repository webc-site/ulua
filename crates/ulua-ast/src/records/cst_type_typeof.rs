use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeTypeof {
  pub base: CstNode,
  pub open_position: Position,
  pub close_position: Position,
}

impl CstNodeClass for CstTypeTypeof {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeTypeof");
}
