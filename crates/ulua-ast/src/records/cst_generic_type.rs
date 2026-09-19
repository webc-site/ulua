use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstGenericType {
  pub base: CstNode,
  pub default_equals_position: Position,
}

impl CstNodeClass for CstGenericType {
  const CLASS_INDEX: i32 = ast_rtti_index("CstGenericType");
}
