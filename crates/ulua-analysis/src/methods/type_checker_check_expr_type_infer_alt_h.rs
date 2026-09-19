use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::{
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_function_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprFunction,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    let (fun_ty, fun_scope) =
      self.check_function_signature(scope, 0, expr, None, None, expected_type);

    self.check_function_body(&fun_scope, fun_ty, expr);

    WithPredicate::with_predicate_t(self.quantify(&fun_scope, fun_ty, expr.base.base.location))
  }
}
