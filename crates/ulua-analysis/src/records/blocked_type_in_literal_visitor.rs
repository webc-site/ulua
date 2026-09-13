use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct BlockedTypeInLiteralVisitor {
  pub(crate) ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) to_block: *mut Vec<TypeId>,
}
