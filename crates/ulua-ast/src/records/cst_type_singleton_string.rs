use ulua_common::LUAU_ASSERT;

use crate::{
  enums::quote_style_cst::{QuoteStyle, QuoteStyle::QuotedInterp},
  records::{ast_array::AstArray, cst_node::CstNode},
  rtti::CstNodeClass,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeSingletonString {
  pub base: CstNode,
  pub source_string: AstArray<u8>,
  pub quote_style: QuoteStyle,
  pub block_depth: u32,
}

impl_cst_node_class!(CstTypeSingletonString);

impl CstTypeSingletonString {
  pub fn new(source_string: AstArray<u8>, quote_style: QuoteStyle, block_depth: u32) -> Self {
    LUAU_ASSERT!(quote_style != QuotedInterp);

    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      source_string,
      quote_style,
      block_depth,
    }
  }
}
