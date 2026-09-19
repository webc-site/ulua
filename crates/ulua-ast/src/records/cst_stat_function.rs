use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatFunction {
  pub base: CstNode,
  pub function_keyword_position: Position,
}

impl CstNodeClass for CstStatFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatFunction");
}
