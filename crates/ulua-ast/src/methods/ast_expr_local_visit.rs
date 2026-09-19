use core::ffi::c_void;

use crate::{
  records::{ast_expr_local::AstExprLocal, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprLocal {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_expr_local(self as *mut Self as *mut c_void);
  }
}
