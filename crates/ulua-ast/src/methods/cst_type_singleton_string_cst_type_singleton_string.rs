use core::ffi::c_char;

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::quote_style_cst::{QuoteStyle, QuoteStyle::QuotedInterp},
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_singleton_string::CstTypeSingletonString,
  },
  rtti::CstNodeClass,
};

impl CstTypeSingletonString {
  pub fn new(source_string: AstArray<c_char>, quote_style: QuoteStyle, block_depth: u32) -> Self {
    LUAU_ASSERT!(quote_style != QuotedInterp);

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

pub fn cst_type_singleton_string_cst_type_singleton_string(
  source_string: AstArray<c_char>,
  quote_style: QuoteStyle,
  block_depth: u32,
) -> CstTypeSingletonString {
  CstTypeSingletonString::new(source_string, quote_style, block_depth)
}
