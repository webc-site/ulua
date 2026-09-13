use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_bool::AstExprConstantBool, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantBool {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_constant_bool(self as *const Self as *mut c_void);
  }
}

pub fn ast_expr_constant_bool_visit(this: &AstExprConstantBool, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
