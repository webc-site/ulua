use core::ffi::c_void;

use crate::{
  records::{ast_expr_global::AstExprGlobal, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprGlobal {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_global(self as *const Self as *mut c_void) {
      // AstExprGlobal has no children to recurse into (it only contains an AstName which is a value type).
    }
  }
}
