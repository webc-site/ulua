use ulua_ast::records::ast_expr::AstExpr;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct IncompleteInference {
  pub(crate) expected_type: TypeId,
  pub(crate) target_type: TypeId,
  pub(crate) expr: *const AstExpr,
}
