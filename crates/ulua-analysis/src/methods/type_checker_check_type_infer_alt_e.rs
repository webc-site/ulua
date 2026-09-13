use ulua_ast::records::ast_stat_while::AstStatWhile;

use crate::{
  enums::control_flow::ControlFlow, records::type_checker::TypeChecker,
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_while(
    &mut self,
    scope: &ScopePtr,
    statement: &AstStatWhile,
  ) -> ControlFlow {
    let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      scope,
      unsafe { &*statement.condition },
      None,
      false,
    );
    let while_scope = self.child_scope(scope, &unsafe { (*statement.body).base.base.location });
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &while_scope, true);
    self.check_scope_ptr_ast_stat_block(&while_scope, unsafe { &*statement.body });
    ControlFlow::None
  }
}
