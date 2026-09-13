#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeSingletonString {
  pub base: CstNode,
  pub source_string: AstArray<c_char>,
  pub quote_style: QuoteStyle,
  pub block_depth: u32,
}

impl CstNodeClass for CstTypeSingletonString {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeSingletonString");
}
use core::ffi::c_char;

use crate::{
  enums::quote_style_cst::QuoteStyle,
  records::{ast_array::AstArray, cst_node::CstNode},
  rtti::{CstNodeClass, ast_rtti_index},
};
