use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{ast_stat_for::AstStatFor, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintForRange {
  pub(crate) context: *mut LintContext,
}

impl LintForRange {
  pub fn lint_for_range(&mut self) {
    self.context = null_mut();
  }

  pub fn get_loop_end(&self, from: f64, to: f64) -> f64 {
    from + (to - from).floor()
  }

  pub fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_stat_for_linter(node as *mut AstStatFor) }
  }
}

impl AstVisitor for LintForRange {
  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    LintForRange::visit_stat_for(self, node)
  }
}
