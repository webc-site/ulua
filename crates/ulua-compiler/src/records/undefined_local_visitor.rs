//! Source: `Compiler/src/Compiler.cpp`

use core::ffi::c_void;

use ulua_ast::records::{ast_local::AstLocal, ast_visitor::AstVisitor};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::compiler::Compiler;

#[derive(Debug, Clone)]
pub struct UndefinedLocalVisitor {
  pub(crate) self_: *mut Compiler,
  pub(crate) undef: *mut AstLocal,
  pub(crate) locals: DenseHashSet<*mut AstLocal>,
}

impl AstVisitor for UndefinedLocalVisitor {
  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node.cast())
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node.cast())
  }
}
