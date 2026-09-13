use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, cst_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatCompoundAssign {
  pub base: CstNode,
  pub op_position: Position,
}

impl CstNodeClass for CstStatCompoundAssign {
  const CLASS_INDEX: i32 = cst_rtti_index("CstStatCompoundAssign");
}
