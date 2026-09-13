use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_table::AstExprTable, ast_stat_block::AstStatBlock,
  ast_stat_repeat::AstStatRepeat, ast_visitor::AstVisitor,
};

use crate::records::{lint_context::LintContext, statement::Statement};
#[derive(Debug, Clone)]
pub struct LintMultiLineStatement {
  pub(crate) context: *mut LintContext,
  pub(crate) stack: Vec<Statement>,
}

impl LintMultiLineStatement {
  pub fn new(context: *mut LintContext) -> Self {
    Self {
      context,
      stack: Vec::new(),
    }
  }

  pub fn lint_multi_line_statement(&mut self, _node: *mut c_void) {
    // implemented in separate method files
  }

  pub fn visit_expr(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_expr(node as *mut AstExpr) }
  }

  pub fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_table(node as *mut AstExprTable)
  }

  pub fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_repeat(node as *mut AstStatRepeat)
  }

  pub fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_block(node as *mut AstStatBlock)
  }
}

impl AstVisitor for LintMultiLineStatement {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.lint_multi_line_statement(node);
    true
  }

  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_expr(node)
  }

  fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
    self.visit_expr_table(node)
  }

  fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
    self.visit_stat_repeat(node)
  }

  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_stat_block(node)
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
