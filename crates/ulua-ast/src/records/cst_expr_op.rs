use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprOp {
  pub base: CstNode,
  pub op_position: Position,
}

impl CstNodeClass for CstExprOp {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprOp");
}
