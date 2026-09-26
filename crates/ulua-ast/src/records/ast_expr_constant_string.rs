use crate::{
  enums::quote_style_ast::QuoteStyle,
  records::{ast_array::AstArray, ast_expr::AstExpr},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantString {
  pub base: AstExpr,
  pub value: AstArray<u8>,
  pub quote_style: QuoteStyle,
}
