use core::ffi::c_void;

use crate::{
  records::{ast_expr_varargs::AstExprVarargs, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprVarargs {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_expr_varargs(self as *mut Self as *mut c_void);
  }
}
