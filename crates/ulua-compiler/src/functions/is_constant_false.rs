use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::constant::Constant;

#[inline]
pub fn is_constant_false(
  constants: &DenseHashMap<*mut AstExpr, Constant>,
  node: *mut AstExpr,
) -> bool {
  match constants.find(&node) {
    Some(cv) => !cv.is_unknown() && !cv.is_truthful(),
    None => false,
  }
}
