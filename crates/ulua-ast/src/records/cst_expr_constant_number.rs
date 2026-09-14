#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprConstantNumber {
  pub base: CstNode,
  pub value: AstArray<c_char>,
}

impl CstNodeClass for CstExprConstantNumber {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprConstantNumber");
}
use core::ffi::c_char;

use crate::{
  records::{ast_array::AstArray, cst_node::CstNode},
  rtti::{CstNodeClass, ast_rtti_index},
};
