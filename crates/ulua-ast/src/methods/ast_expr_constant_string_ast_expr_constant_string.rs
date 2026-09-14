use core::ffi::c_char;

use crate::{
  enums::quote_style_ast::QuoteStyle,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_node::AstNode, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprConstantString {
  pub fn new(location: Location, value: AstArray<c_char>, quote_style: QuoteStyle) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      value,
      quote_style,
    }
  }
}

pub fn ast_expr_constant_string_ast_expr_constant_string(
  location: Location,
  value: AstArray<c_char>,
  quote_style: QuoteStyle,
) -> AstExprConstantString {
  AstExprConstantString::new(location, value, quote_style)
}
