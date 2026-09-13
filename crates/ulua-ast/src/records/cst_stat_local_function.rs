#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatLocalFunction {
  pub base: CstNode,
  pub local_keyword_position: Position,
  pub function_keyword_position: Position,
}

impl CstNodeClass for CstStatLocalFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatLocalFunction");
}
use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
