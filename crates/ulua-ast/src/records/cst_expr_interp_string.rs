use core::ffi::c_char;

use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprInterpString {
  pub base: CstNode,
  pub source_strings: AstArray<AstArray<c_char>>,
  pub string_positions: AstArray<Position>,
}

impl CstNodeClass for CstExprInterpString {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprInterpString");
}
