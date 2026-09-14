use core::ffi::c_char;

use crate::{
  records::{
    ast_array::AstArray, cst_expr_constant_integer::CstExprConstantInteger, cst_node::CstNode,
  },
  rtti::CstNodeClass,
};

impl CstExprConstantInteger {
  pub fn new(value: AstArray<c_char>) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      value,
    }
  }
}

pub fn cst_expr_constant_integer_cst_expr_constant_integer(
  value: AstArray<c_char>,
) -> CstExprConstantInteger {
  CstExprConstantInteger::new(value)
}
