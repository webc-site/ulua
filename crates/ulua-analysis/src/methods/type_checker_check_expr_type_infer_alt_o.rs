use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::{
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_interp_string(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprInterpString,
  ) -> WithPredicate<TypeId> {
    for &child in expr.expressions.as_slice() {
      self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*child },
        None,
        false,
      );
    }

    WithPredicate::with_predicate_t(self.string_type)
  }
}
