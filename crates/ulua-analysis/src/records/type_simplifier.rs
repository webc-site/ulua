use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct TypeSimplifier {
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) blocked_types: DenseHashSet<TypeId>,
  pub(crate) recursion_depth: i32,
}
