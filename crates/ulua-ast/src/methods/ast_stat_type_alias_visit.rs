use core::ffi::c_void;

use crate::{
  records::{ast_node::AstNode, ast_stat_type_alias::AstStatTypeAlias, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_node_visit, ast_type_visit},
};

impl AstVisitable for AstStatTypeAlias {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_type_alias(self as *const Self as *mut c_void) {
      for &el in self.generics.iter() {
        unsafe {
          ast_node_visit(el as *mut AstNode, visitor);
        }
      }

      for &el in self.generic_packs.iter() {
        unsafe {
          ast_node_visit(el as *mut AstNode, visitor);
        }
      }

      unsafe {
        ast_type_visit(self.type_ptr, visitor);
      }
    }
  }
}

pub fn ast_stat_type_alias_visit(this: &AstStatTypeAlias, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
