use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_stat::AstStat, ast_visitor::AstVisitor,
};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintUnreachableCode {
  pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintUnreachableCode {
  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprFunction;
    unsafe {
      let body = (*node).body;
      self.analyze(body as *mut AstStat);
    }
    true
  }
}
