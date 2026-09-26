use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena,
    type_remover::TypeRemover,
  },
  type_aliases::type_id::TypeId,
};
pub fn remove_type(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  haystack: TypeId,
  needle: TypeId,
) {
  let mut tr = TypeRemover {
    builtin_types,
    arena,
    needle,
    seen: DenseHashSet::default(),
  };
  tr.process(haystack);
}
