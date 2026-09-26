use ulua_ast::records::ast_node::AstNode;

use crate::{
  functions::push_module_scope::push_module_scope,
  records::{non_strict_type_checker::NonStrictTypeChecker, stack_pusher::StackPusher},
};

impl NonStrictTypeChecker {
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    push_module_scope(self.module, &mut self.stack, node)
  }
}
