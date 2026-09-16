use core::ffi::c_char;

use crate::{
  records::{
    ast_array::AstArray, cst_expr_constant_number::CstExprConstantNumber, cst_node::CstNode,
  },
  rtti::CstNodeClass,
};

impl CstExprConstantNumber {
  pub fn new(value: AstArray<c_char>) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      value,
    }
  }
}

pub fn cst_expr_constant_number_cst_expr_constant_number(
  value: &AstArray<c_char>,
) -> CstExprConstantNumber {
  CstExprConstantNumber::new(*value)
}
