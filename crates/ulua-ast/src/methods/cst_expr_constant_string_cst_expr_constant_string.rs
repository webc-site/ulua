use core::ffi::c_char;

use crate::{
  enums::quote_style_cst::{QuoteStyle, QuoteStyle::QuotedRaw},
  records::{
    ast_array::AstArray, cst_expr_constant_string::CstExprConstantString, cst_node::CstNode,
  },
  rtti::CstNodeClass,
};

impl CstExprConstantString {
  pub fn new(source_string: AstArray<c_char>, quote_style: QuoteStyle, block_depth: u32) -> Self {
    ulua_common::LUAU_ASSERT!(block_depth == 0 || quote_style == QuotedRaw);

    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      source_string,
      quote_style,
      block_depth,
    }
  }
}

pub fn cst_expr_constant_string_cst_expr_constant_string(
  source_string: AstArray<c_char>,
  quote_style: QuoteStyle,
  block_depth: u32,
) -> CstExprConstantString {
  CstExprConstantString::new(source_string, quote_style, block_depth)
}
