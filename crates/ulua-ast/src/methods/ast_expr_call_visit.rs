use core::ffi::c_void;

use crate::{
  records::{ast_expr_call::AstExprCall, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprCall {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_call(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.func, visitor);

        for &arg in self.args.iter() {
          ast_expr_visit(arg, visitor);
        }
      }
    }
  }
}

pub fn ast_expr_call_visit(this: &AstExprCall, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
