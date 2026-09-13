use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{
  ast_expr_binary::AstExprBinary, ast_expr_if_else::AstExprIfElse, ast_stat_if::AstStatIf,
  ast_visitor::AstVisitor,
};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintDuplicateCondition {
  pub(crate) context: *mut LintContext,
}

impl LintDuplicateCondition {
  pub fn lint_duplicate_condition_lint_duplicate_condition(&mut self) {
    self.context = null_mut();
  }
}

impl AstVisitor for LintDuplicateCondition {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    let _ = node;
    true
  }

  fn visit_stat_if(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_if(node as *mut AstStatIf)
  }

  fn visit_expr_if_else(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_if_else(node as *mut AstExprIfElse)
  }

  fn visit_expr_binary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_binary(node as *mut AstExprBinary)
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
