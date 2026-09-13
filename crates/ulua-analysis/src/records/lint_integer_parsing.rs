use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{ast_expr_constant_number::AstExprConstantNumber, ast_visitor::AstVisitor};

use crate::{
  methods::lint_integer_parsing_visit::lint_integer_parsing_visit,
  records::lint_context::LintContext,
};
#[derive(Debug, Clone)]
pub struct LintIntegerParsing {
  pub(crate) context: *mut LintContext,
}

impl LintIntegerParsing {
  pub fn lint_integer_parsing(&mut self) {
    self.context = null_mut();
  }

  pub fn visit_expr_constant_integer(&mut self, node: *mut c_void) -> bool {
    unsafe { lint_integer_parsing_visit(self, node as *mut AstExprConstantNumber) }
  }

  pub fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }
}

impl AstVisitor for LintIntegerParsing {
  fn visit_expr_constant_number(&mut self, node: *mut c_void) -> bool {
    LintIntegerParsing::visit_expr_constant_integer(self, node)
  }

  fn visit_node(&mut self, node: *mut c_void) -> bool {
    LintIntegerParsing::visit_node(self, node)
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
