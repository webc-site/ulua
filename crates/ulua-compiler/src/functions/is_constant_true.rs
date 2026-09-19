use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::constant::Constant;

pub(crate) fn is_constant_true(
  constants: &DenseHashMap<*mut AstExpr, Constant>,
  node: *mut AstExpr,
) -> bool {
  match constants.find(&node) {
    Some(cv) if !cv.is_unknown() => cv.is_truthful(),
    _ => false,
  }
}
