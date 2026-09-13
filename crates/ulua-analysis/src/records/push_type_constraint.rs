use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct PushTypeConstraint {
  pub(crate) expected_type: TypeId,
  pub(crate) target_type: TypeId,
  pub(crate) ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_expected_types: *const DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) expr: *const AstExpr,
}
