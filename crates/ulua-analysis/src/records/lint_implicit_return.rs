use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_stat_return::AstStatReturn, ast_visitor::AstVisitor,
  location::Location,
};

use crate::{
  methods::{
    lint_implicit_return_get_end_location::lint_implicit_return_get_end_location,
    lint_implicit_return_get_value_return::lint_implicit_return_get_value_return,
  },
  records::lint_context::LintContext,
};
#[derive(Debug, Clone)]
pub struct LintImplicitReturn {
  pub(crate) context: *mut LintContext,
}

impl LintImplicitReturn {
  pub fn process(context: &mut LintContext) {
    // implemented in separate method files
    let _ = context;
  }

  pub fn get_end_location(&mut self, node: *const c_void) -> Location {
    lint_implicit_return_get_end_location(self, node)
  }

  pub fn get_value_return(&mut self, node: *mut c_void) -> *mut AstStatReturn {
    lint_implicit_return_get_value_return(self, node)
  }

  pub fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit(node as *mut AstExprFunction) }
  }
}

impl AstVisitor for LintImplicitReturn {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    let _ = node;
    true
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_expr_function(node)
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
