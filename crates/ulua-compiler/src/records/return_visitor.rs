//! Source: `Compiler/src/Compiler.cpp`

use core::ffi::c_void;

use ulua_ast::records::ast_visitor::AstVisitor;

use crate::records::compiler::Compiler;

#[derive(Debug, Clone, Copy)]
pub struct ReturnVisitor {
  pub(crate) self_: *mut Compiler,
  pub(crate) returns_one: bool,
}

impl AstVisitor for ReturnVisitor {
  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr(node.cast())
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_return(node.cast())
  }
}
