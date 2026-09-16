use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatDo {
  pub base: CstNode,
  pub stats_start_position: Position,
  pub end_position: Position,
}

impl CstNodeClass for CstStatDo {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatDo");
}
