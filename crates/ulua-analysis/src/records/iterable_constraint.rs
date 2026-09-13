use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct IterableConstraint {
  pub(crate) iterator: TypePackId,
  pub(crate) variables: Vec<TypeId>,
  pub(crate) next_ast_fragment: *const AstNode,
  pub(crate) ast_for_in_next_types: *mut DenseHashMap<*const AstNode, TypeId>,
}
