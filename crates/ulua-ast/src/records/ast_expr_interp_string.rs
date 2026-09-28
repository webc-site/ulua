#[repr(C)]
#[derive(Debug)]
pub struct AstExprInterpString {
  pub base: AstExpr,
  pub strings: AstArray<AstArray<u8>>,
  pub expressions: AstArray<*mut AstExpr>,
}

use crate::records::{ast_array::AstArray, ast_expr::AstExpr};
