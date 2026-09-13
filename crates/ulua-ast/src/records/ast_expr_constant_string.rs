use core::ffi::c_char;

use crate::{
  enums::quote_style_ast::QuoteStyle,
  records::{ast_array::AstArray, ast_expr::AstExpr},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantString {
  pub base: AstExpr,
  pub value: AstArray<c_char>,
  pub quote_style: QuoteStyle,
}

impl AstNodeClass for AstExprConstantString {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprConstantString");
}
