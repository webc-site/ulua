use ulua_ast::records::ast_node::AstNode;

use crate::{
  functions::push_module_scope::push_module_scope,
  records::{stack_pusher::StackPusher, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  // C++ `std::optional<StackPusher> TypeChecker2::pushStack(AstNode* node)`
  // (TypeChecker2.cpp:476): if the node has a recorded scope, push it onto the
  // scope stack for the lifetime of the returned guard; otherwise nullopt.
  // 实现收口于 `push_module_scope`（与旧求解器 `push_stack` 共享同一段查表-压栈）。
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    push_module_scope(self.module, &mut self.stack, node)
  }
}
