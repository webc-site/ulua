use core::ffi::c_void;

use crate::{
  records::{ast_expr_varargs::AstExprVarargs, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprVarargs {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_varargs(self as *const Self as *mut c_void);
  }
}

pub fn ast_expr_varargs_visit(this: &AstExprVarargs, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
