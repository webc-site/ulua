use core::ffi::c_void;

use crate::{
  records::{ast_stat_continue::AstStatContinue, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstStatContinue {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_stat_continue(self as *mut Self as *mut c_void);
  }
}
