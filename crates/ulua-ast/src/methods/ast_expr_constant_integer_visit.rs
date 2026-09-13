use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_integer::AstExprConstantInteger, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantInteger {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_constant_integer(self as *const Self as *mut c_void);
  }
}

pub fn ast_expr_constant_integer_visit(
  this: &AstExprConstantInteger,
  visitor: &mut dyn AstVisitor,
) {
  this.visit(visitor);
}
