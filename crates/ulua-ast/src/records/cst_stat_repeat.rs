use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatRepeat {
  pub base: CstNode,
  pub until_position: Position,
}

impl CstNodeClass for CstStatRepeat {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatRepeat");
}
