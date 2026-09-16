use core::ffi::c_void;

use crate::{
  records::{ast_type_table::AstTypeTable, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstTypeTable {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_type_table(self as *const Self as *mut c_void) {
      for prop in self.props.iter() {
        unsafe {
          ast_type_visit(prop.r#type, visitor);
        }
      }

      if !self.indexer.is_null() {
        unsafe {
          let indexer = &*self.indexer;
          ast_type_visit(indexer.index_type, visitor);
          ast_type_visit(indexer.result_type, visitor);
        }
      }
    }
  }
}

pub fn ast_type_table_visit<V: AstVisitor + ?Sized>(this: &AstTypeTable, visitor: &mut V) {
  this.visit(visitor);
}
