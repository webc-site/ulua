use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatFor {
  pub base: CstNode,
  pub annotation_colon_position: Position,
  pub equals_position: Position,
  pub end_comma_position: Position,
  pub step_comma_position: Position,
}

impl CstNodeClass for CstStatFor {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatFor");
}
