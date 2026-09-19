use alloc::sync::Arc;

use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::{
  enums::control_flow::ControlFlow,
  records::{scope::Scope, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_block(
    &mut self,
    scope: &ScopePtr,
    block: &AstStatBlock,
  ) -> ControlFlow {
    let child = self.child_scope(scope, &block.base.base.location);
    let flow = self.check_block(&child, block);

    unsafe {
      let scope_mut = Arc::as_ptr(scope) as *mut Scope;
      (*scope_mut).inherit_refinements(&child);
    }

    flow
  }
}
