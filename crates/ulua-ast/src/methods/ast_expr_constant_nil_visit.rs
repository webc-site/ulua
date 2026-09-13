use core::ffi::c_void;

use crate::{
  records::{ast_expr_constant_nil::AstExprConstantNil, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstExprConstantNil {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_expr_constant_nil(self as *const Self as *mut c_void);
  }
}
