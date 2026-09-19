//! Source: `Compiler/src/Compiler.cpp`

use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::records::{ast_local::AstLocal, ast_visitor::AstVisitor};

use crate::records::compiler::Compiler;

#[derive(Debug, Clone)]
pub struct ConstUpvalueVisitor {
  pub(crate) self_: *mut Compiler,
  pub(crate) upvals: Vec<*mut AstLocal>,
}

impl AstVisitor for ConstUpvalueVisitor {
  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node.cast())
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node.cast())
  }
}
