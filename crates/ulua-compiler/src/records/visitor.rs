use core::ffi::c_void;

use ulua_ast::records::ast_visitor::AstVisitor;

use crate::records::compiler::Compiler;

#[derive(Debug, Clone)]
pub struct Visitor {
  pub(crate) self_: *mut Compiler,
  pub(crate) conflict: [u64; 4],
  pub(crate) assigned: [u64; 4],
}

impl AstVisitor for Visitor {
  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node.cast())
  }
}
