use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::contains_never::contains_never as containsNever,
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn check_expr_pack(&mut self, scope: &ScopePtr, expr: &AstExpr) -> WithPredicate<TypePackId> {
    let result = self.check_expr_pack_helper_scope_ptr_ast_expr(scope, expr);
    if containsNever(result.r#type) {
      return WithPredicate::with_predicate_t_predicate_vec(
        self.uninhabitable_type_pack,
        Default::default(),
      );
    }
    result
  }
}
