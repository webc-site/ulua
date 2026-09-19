use core::ffi::c_void;

use crate::{
  functions::visit_type_or_pack_array::visit_type_or_pack_array,
  records::{ast_expr_instantiate::AstExprInstantiate, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprInstantiate {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_instantiate(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
      visit_type_or_pack_array(visitor, self.type_arguments);
    }
  }
}
