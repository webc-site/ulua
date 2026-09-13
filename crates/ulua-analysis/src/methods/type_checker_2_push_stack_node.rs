use ulua_ast::records::ast_node::AstNode;

use crate::records::{stack_pusher_type_checker_2::StackPusher, type_checker_2::TypeChecker2};

impl TypeChecker2 {
  // C++ `std::optional<StackPusher> TypeChecker2::pushStack(AstNode* node)`
  // (TypeChecker2.cpp:476): if the node has a recorded scope, push it onto the
  // scope stack for the lifetime of the returned guard; otherwise nullopt.
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    if self.module.is_null() {
      return None;
    }

    let module = unsafe { &*self.module };
    module
      .ast_scopes
      .find(&(node as *const AstNode))
      .map(|scope_ptr| unsafe { StackPusher::new(&mut self.stack, *scope_ptr) })
  }
}
