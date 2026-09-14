use ulua_ast::records::ast_expr_instantiate::AstExprInstantiate;

use crate::{
  FFlag,
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_instantiate(
    &mut self,
    scope: &ScopePtr,
    explicit_type_instantiation: &AstExprInstantiate,
  ) -> WithPredicate<TypeId> {
    if !FFlag::LuauExplicitTypeInstantiationSupport.get() {
      return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
    }

    let base_expr = unsafe { &*explicit_type_instantiation.expr };
    let base_type =
      self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, base_expr, None, false);

    WithPredicate::with_predicate_t(unsafe {
      self.instantiate_type_parameters(
        scope.clone(),
        base_type.r#type,
        explicit_type_instantiation.type_arguments,
        base_expr as *const _,
        &base_expr.base.location,
      )
    })
  }
}
