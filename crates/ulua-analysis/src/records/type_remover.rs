use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

#[derive(Debug)]
pub struct TypeRemover {
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) needle: TypeId,
  pub(crate) seen: DenseHashSet<TypeId>,
}
