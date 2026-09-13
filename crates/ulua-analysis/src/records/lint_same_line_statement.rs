use core::ffi::c_void;

use ulua_ast::records::ast_visitor::AstVisitor;

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintSameLineStatement {
  pub(crate) context: *mut LintContext,
  pub(crate) last_line: u32,
}

impl LintSameLineStatement {
  pub fn new(context: *mut LintContext) -> Self {
    Self {
      context,
      last_line: u32::MAX,
    }
  }
}

impl AstVisitor for LintSameLineStatement {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_stat_block(node)
  }

  fn visit_expr(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_stat(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
