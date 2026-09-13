use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PushFunctionTypeConstraint {
  pub(crate) expected_function_type: TypeId,
  pub(crate) function_type: TypeId,
  pub(crate) expr: *mut AstExprFunction,
  pub(crate) is_self: bool,
}
