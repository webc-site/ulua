use core::ffi::c_void;

use ulua_ast::records::{ast_expr_call::AstExprCall, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintFormatString {
  pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintFormatString {
  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprCall;
    unsafe { self.match_call(node) };
    true
  }
}
