#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprTypeAssertion {
  pub base: CstNode,
  pub op_position: Position,
}

impl CstNodeClass for CstExprTypeAssertion {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprTypeAssertion");
}
use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
