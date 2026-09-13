use core::ffi::c_void;

use ulua_ast::records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintUnknownType {
  pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintUnknownType {
  fn visit_expr_binary(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit(node as *mut AstExprBinary) }
  }
}
