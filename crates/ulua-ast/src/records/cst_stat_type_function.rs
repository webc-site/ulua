#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatTypeFunction {
  pub base: CstNode,
  pub type_keyword_position: Position,
  pub function_keyword_position: Position,
}

impl CstNodeClass for CstStatTypeFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatTypeFunction");
}
use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
