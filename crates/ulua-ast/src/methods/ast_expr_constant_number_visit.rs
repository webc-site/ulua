use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_number::AstExprConstantNumber, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantNumber {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_constant_number(self as *const Self as *mut c_void);
  }
}
