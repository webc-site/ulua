// ConstraintGenerator::visit(const ScopePtr&, AstStatRepeat*) (ConstraintGenerator.cpp:1707-1718).
use ulua_ast::records::{ast_node::AstNode, ast_stat_repeat::AstStatRepeat};

use crate::{
  enums::control_flow::ControlFlow,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_repeat(
    &mut self,
    scope: &ScopePtr,
    repeat: *mut AstStatRepeat,
  ) -> ControlFlow {
    let repeat_ref = unsafe { &*repeat };

    let repeat_scope: ScopePtr = unsafe {
      self.child_scope(
        &repeat_ref.base.base as *const AstNode as *mut AstNode,
        scope,
      )
    };

    unsafe {
      self.visit_block_without_child_scope(
        repeat_scope.as_ref() as *const Scope as *mut Scope,
        repeat_ref.body,
      )
    };

    self.check_scope_ptr_ast_expr(&repeat_scope, repeat_ref.condition);

    unsafe {
      (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&repeat_scope);
    }

    ControlFlow::None
  }
}
