use crate::{
  records::{cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprIndexExpr {
  pub base: CstNode,
  pub open_bracket_position: Position,
  pub close_bracket_position: Position,
}

impl CstNodeClass for CstExprIndexExpr {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprIndexExpr");
}
