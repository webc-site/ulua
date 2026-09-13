use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

#[derive(Debug)]
pub struct TypeRemover {
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) arena: *mut TypeArena,
  pub(crate) needle: TypeId,
  pub(crate) seen: DenseHashSet<TypeId>,
}
