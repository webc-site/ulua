use core::ffi::c_void;

use crate::{
  records::{ast_expr_varargs::AstExprVarargs, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprVarargs {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    visitor.visit_expr_varargs(self as *const Self as *mut c_void);
  }
}

pub fn ast_expr_varargs_visit<V: AstVisitor + ?Sized>(this: &AstExprVarargs, visitor: &mut V) {
  this.visit(visitor);
}
