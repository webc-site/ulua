#[repr(C)]
#[derive(Debug)]
pub struct AstExprInterpString {
  pub base: AstExpr,
  pub strings: AstArray<AstArray<c_char>>,
  pub expressions: AstArray<*mut AstExpr>,
}

impl AstNodeClass for AstExprInterpString {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprInterpString");
}
use core::ffi::c_char;

use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr},
  rtti::{AstNodeClass, ast_rtti_index},
};
