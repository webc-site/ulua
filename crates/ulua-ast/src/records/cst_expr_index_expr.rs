use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprIndexExpr {
  pub base: CstNode,
  pub open_bracket_position: Position,
  pub close_bracket_position: Position,
}

impl_cst_node_class!(CstExprIndexExpr);
impl_cst_node_new!(
  CstExprIndexExpr,
  open_bracket_position: Position,
  close_bracket_position: Position,
);
