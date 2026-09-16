use core::ffi::c_void;

use ulua_ast::records::ast_visitor::AstVisitor;

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintUnbalancedAssignment {
  pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintUnbalancedAssignment {
  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node)
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node)
  }
}
