use core::ffi::c_char;

use crate::{
  records::{
    ast_array::AstArray, cst_expr_interp_string::CstExprInterpString, cst_node::CstNode,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstExprInterpString {
  pub fn new(
    source_strings: AstArray<AstArray<c_char>>,
    string_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      source_strings,
      string_positions,
    }
  }
}

pub fn cst_expr_interp_string_cst_expr_interp_string(
  source_strings: AstArray<AstArray<c_char>>,
  string_positions: AstArray<Position>,
) -> CstExprInterpString {
  CstExprInterpString::new(source_strings, string_positions)
}
