use core::ffi::c_void;

use crate::{
  records::{ast_expr_local::AstExprLocal, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprLocal {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_local(self as *const Self as *mut c_void);
  }
}
