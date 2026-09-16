use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct TypeSimplifier {
  pub(crate) builtin_types: *const BuiltinTypes,
  pub(crate) arena: *const TypeArena,
  pub(crate) blocked_types: DenseHashSet<TypeId>,
  pub(crate) recursion_depth: i32,
}
