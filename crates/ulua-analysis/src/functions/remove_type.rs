use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena, type_remover::TypeRemover},
  type_aliases::type_id::TypeId,
};
pub fn remove_type(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  haystack: TypeId,
  needle: TypeId,
) {
  let mut tr = TypeRemover {
    builtin_types,
    arena,
    needle,
    seen: DenseHashSet::new(null()),
  };
  tr.process(haystack);
}
