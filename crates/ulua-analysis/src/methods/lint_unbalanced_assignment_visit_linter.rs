use core::ffi::c_void;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::lint_unbalanced_assignment::LintUnbalancedAssignment;
impl LintUnbalancedAssignment {
  pub fn visit_ast_stat_local(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatLocal;
    unsafe {
      self.assign(
        (*node).vars.size,
        &(*node).values,
        (*node).base.base.location,
      );
    }
    true
  }
}
