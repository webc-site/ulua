use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_integer::AstExprConstantInteger, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantInteger {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_expr_constant_integer(self as *mut Self as *mut c_void);
  }
}
