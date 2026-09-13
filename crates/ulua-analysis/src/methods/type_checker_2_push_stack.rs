//! Faithful port of `TypeChecker2::pushStack` (TypeChecker2.cpp:476-482).
use ulua_ast::records::ast_node::AstNode;

use crate::records::{stack_pusher_type_checker_2::StackPusher, type_checker_2::TypeChecker2};

impl TypeChecker2 {
  // C++ `std::optional<StackPusher> TypeChecker2::pushStack(AstNode* node)`:
  //   if (Scope** scope = module->astScopes.find(node))
  //       return StackPusher{stack, *scope};
  //   else
  //       return std::nullopt;
  pub fn type_checker_2_push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    let scope = unsafe {
      (*self.module)
        .ast_scopes
        .find(&(node as *const AstNode))
        .copied()
    };
    match scope {
      // `StackPusher::new` pushes the scope onto the stack for the
      // lifetime of the returned guard (the C++ explicit-ctor behaviour).
      Some(scope) => Some(unsafe { StackPusher::new(&mut self.stack, scope) }),
      None => None,
    }
  }
}
