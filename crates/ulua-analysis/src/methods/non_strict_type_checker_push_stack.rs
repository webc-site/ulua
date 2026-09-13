use ulua_ast::records::ast_node::AstNode;

use crate::records::{
  non_strict_type_checker::NonStrictTypeChecker, stack_pusher_non_strict_type_checker::StackPusher,
};

impl NonStrictTypeChecker {
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    if self.module.is_null() {
      return None;
    }

    let module = unsafe { &*self.module };
    // module.ast_scopes: DenseHashMap<*const AstNode, *mut Scope>
    // C++ lookup: module->astScopes.find(node)
    module
      .ast_scopes
      .find(&(node as *const AstNode))
      .map(|scope_ptr| unsafe { StackPusher::new(&mut self.stack, *scope_ptr) })
  }
}
