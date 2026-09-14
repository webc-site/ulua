// ConstraintGenerator::visit(const ScopePtr&, AstStatWhile*) (ConstraintGenerator.cpp:1693-1705).
use ulua_ast::records::{ast_node::AstNode, ast_stat_while::AstStatWhile};

use crate::{
  enums::control_flow::ControlFlow,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::{refinement_id_refinement::RefinementId, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_while(
    &mut self,
    scope: &ScopePtr,
    while_: *mut AstStatWhile,
  ) -> ControlFlow {
    let while_ref = unsafe { &*while_ };

    let refinement: RefinementId = self
      .check_scope_ptr_ast_expr(scope, while_ref.condition)
      .refinement;

    let while_scope: ScopePtr = unsafe {
      self.child_scope(
        &while_ref.base.base as *const AstNode as *mut AstNode,
        scope,
      )
    };
    self.apply_refinements(
      &while_scope,
      unsafe { (*while_ref.condition).base.location },
      refinement,
    );

    self.visit_scope_ptr_ast_stat_block(&while_scope, while_ref.body);

    unsafe {
      (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&while_scope);
    }

    ControlFlow::None
  }
}
