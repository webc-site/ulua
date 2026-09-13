use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_unary::AstExprUnary, ast_visitor::AstVisitor,
};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintTableOperations {
  pub(crate) context: *mut LintContext,
}

impl LintTableOperations {
  pub fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }
}

impl AstVisitor for LintTableOperations {
  fn visit_expr_unary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_unary(node as *mut AstExprUnary)
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_call(node as *mut AstExprCall)
  }

  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.visit_node(node)
  }

  fn visit_attr(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
