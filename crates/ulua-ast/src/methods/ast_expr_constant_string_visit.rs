use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_string::AstExprConstantString, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantString {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    visitor.visit_expr_constant_string(self as *const Self as *mut c_void);
  }
}
