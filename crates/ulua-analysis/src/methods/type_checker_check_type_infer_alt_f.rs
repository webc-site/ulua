use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::{
  enums::control_flow::ControlFlow, records::type_checker::TypeChecker,
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_repeat(
    &mut self,
    _scope: &ScopePtr,
    _statement: &AstStatRepeat,
  ) -> ControlFlow {
    let rep_scope = self.child_scope(_scope, &{ _statement.base.base.location });
    self.check_scope_ptr_ast_stat_block(&rep_scope, unsafe { &*_statement.body });
    self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      &rep_scope,
      unsafe { &*_statement.condition },
      None,
      false,
    );
    ControlFlow::None
  }
}
