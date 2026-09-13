use core::ffi::c_void;

use ulua_ast::records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintComparisonPrecedence {
  pub(crate) context: *mut LintContext,
}

impl LintComparisonPrecedence {
  pub fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }
}

impl AstVisitor for LintComparisonPrecedence {
  fn visit_expr_binary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_binary(node as *mut AstExprBinary)
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
