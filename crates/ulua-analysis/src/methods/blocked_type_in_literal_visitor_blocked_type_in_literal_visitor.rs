use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  type_aliases::type_id::TypeId,
};

impl BlockedTypeInLiteralVisitor {
  pub fn blocked_type_in_literal_visitor(
    &mut self,
    ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
    to_block: *mut Vec<TypeId>,
  ) {
    self.ast_types = ast_types;
    self.to_block = to_block;
  }
}
