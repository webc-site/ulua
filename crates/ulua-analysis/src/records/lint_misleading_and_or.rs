use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintMisleadingAndOr {
  pub(crate) context: *mut LintContext,
}

impl LintMisleadingAndOr {
  pub fn lint_misleading_and_or(&mut self) {
    self.context = null_mut();
  }

  pub fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }
}

impl AstVisitor for LintMisleadingAndOr {
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
