use core::ptr::null_mut;

use crate::{
  records::{
    ast_array::AstArray, cst_expr_call::CstExprCall, cst_node::CstNode, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstExprCall {
  pub fn new(
    open_parens: Position,
    close_parens: Position,
    comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      open_parens,
      close_parens,
      comma_positions,
      explicit_types: null_mut(),
    }
  }
}

pub fn cst_expr_call_cst_expr_call(
  open_parens: Position,
  close_parens: Position,
  comma_positions: AstArray<Position>,
) -> CstExprCall {
  CstExprCall::new(open_parens, close_parens, comma_positions)
}
