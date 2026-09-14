use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock};

use crate::{
  enums::control_flow::ControlFlow,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // ConstraintGenerator::visit(const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp).
  pub(crate) fn visit_scope_ptr_ast_stat_block(
    &mut self,
    scope: &ScopePtr,
    block: *mut AstStatBlock,
  ) -> ControlFlow {
    let inner_scope = unsafe { self.child_scope(block as *mut AstNode, scope) };
    let flow = unsafe {
      self
        .visit_block_without_child_scope(inner_scope.as_ref() as *const Scope as *mut Scope, block)
    };

    // An AstStatBlock has linear control flow, i.e. one entry and one exit, so we
    // can inherit all the changes to the environment occurred by the statements in
    // that block.
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
    unsafe {
      (*scope_raw).inherit_assignments(&inner_scope);
      (*scope_raw).inherit_refinements(&inner_scope);
    }

    flow
  }
}
