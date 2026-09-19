use core::ffi::c_void;

use crate::{
  records::{ast_stat_break::AstStatBreak, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstStatBreak {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    visitor.visit_stat_break(self as *const Self as *mut c_void);
  }
}
