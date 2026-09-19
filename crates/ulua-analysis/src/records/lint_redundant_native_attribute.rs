use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_visitor::AstVisitor};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintRedundantNativeAttribute {
  pub(crate) context: *mut LintContext,
}

impl LintRedundantNativeAttribute {
  pub fn lint_redundant_native_attribute(&mut self) {
    self.context = null_mut();
  }
}

impl AstVisitor for LintRedundantNativeAttribute {
  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  // visit_node 沿用 trait 默认实现（返回 true）
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
