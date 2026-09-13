use ulua_ast::records::{ast_expr::AstExpr, ast_expr_call::AstExprCall};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct FunctionCheckConstraint {
  pub(crate) fn_type: TypeId,
  pub(crate) args_pack: TypePackId,
  pub(crate) call_site: *mut AstExprCall,
  pub(crate) ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_expected_types: *const DenseHashMap<*const AstExpr, TypeId>,
}
