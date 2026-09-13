use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_table_indexer::AstTableIndexer,
    ast_type_table::AstTypeTable,
  },
  rtti,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl<'a> TypeMapVisitor<'a> {
  pub fn try_get_table_indexer(&self, expr: *mut AstExpr) -> *mut AstTableIndexer {
    unsafe {
      if let Some(type_ptr) = self.resolved_exprs.find(&expr)
        && !(*type_ptr).is_null()
      {
        let node_ptr = *type_ptr as *mut AstNode;
        let table_ty = rtti::ast_node_as::<AstTypeTable>(node_ptr);
        if !table_ty.is_null() {
          return (*table_ty).indexer;
        }
      }
    }
    null_mut()
  }
}
