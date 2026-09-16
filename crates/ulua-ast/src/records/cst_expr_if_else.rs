use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprIfElse {
  pub base: CstNode,
  pub then_position: Position,
  pub else_position: Position,
  pub is_else_if: bool,
}

impl CstNodeClass for CstExprIfElse {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprIfElse");
}
