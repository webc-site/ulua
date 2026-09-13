use ulua_ast::records::ast_stat_error::AstStatError;

use crate::{
  enums::control_flow::ControlFlow, records::constraint_generator::ConstraintGenerator,
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_error(
    &mut self,
    scope: &ScopePtr,
    error: *mut AstStatError,
  ) -> ControlFlow {
    unsafe {
      for i in 0..(*error).statements.size {
        self.visit_scope_ptr_ast_stat(scope, *(*error).statements.data.add(i));
      }
      for &expr in (*error).expressions.as_slice() {
        self.check_scope_ptr_ast_expr(scope, expr);
      }
    }
    ControlFlow::None
  }
}
