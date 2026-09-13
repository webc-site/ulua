use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_string::AstExprConstantString, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantString {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_constant_string(self as *const Self as *mut c_void);
  }
}

pub fn ast_expr_constant_string_visit(this: &AstExprConstantString, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
