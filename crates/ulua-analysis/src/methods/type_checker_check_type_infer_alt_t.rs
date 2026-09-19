use alloc::sync::Arc;

use ulua_ast::records::ast_stat_error::AstStatError;

use crate::{
  enums::control_flow::ControlFlow,
  records::{module::Module, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_error(
    &mut self,
    scope: &ScopePtr,
    error_statement: &AstStatError,
  ) -> ControlFlow {
    let module_ptr = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
    let old_size = unsafe { (*module_ptr).errors.len() };

    for statement in error_statement.statements.iter() {
      self.check_scope_ptr_ast_stat(scope, unsafe { &**statement });
    }

    for expr in error_statement.expressions.iter() {
      self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &**expr },
        None,
        false,
      );
    }

    unsafe {
      (*module_ptr).errors.truncate(old_size);
    }

    ControlFlow::None
  }
}
