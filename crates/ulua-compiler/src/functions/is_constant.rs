use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{constant::Constant, node::Node};

#[inline]
pub(crate) fn is_constant_true(
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
  node: Node<AstExpr>,
) -> bool {
  constants
    .find(&node)
    .is_some_and(|cv| !cv.is_unknown() && cv.is_truthful())
}

#[inline]
pub(crate) fn is_constant_false(
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
  node: Node<AstExpr>,
) -> bool {
  constants
    .find(&node)
    .is_some_and(|cv| !cv.is_unknown() && !cv.is_truthful())
}
