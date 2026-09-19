use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypePackGeneric {
  pub base: CstNode,
  pub ellipsis_position: Position,
}

impl CstNodeClass for CstTypePackGeneric {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypePackGeneric");
}
