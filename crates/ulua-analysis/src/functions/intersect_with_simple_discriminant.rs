use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::is_simple_discriminant_simplify::is_simple_discriminant_type_id,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena,
    type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};
pub fn intersect_with_simple_discriminant(
  builtin_types: Handle<BuiltinTypes>,
  arena: Handle<TypeArena>,
  target: TypeId,
  discriminant: TypeId,
) -> Option<TypeId> {
  if !is_simple_discriminant_type_id(discriminant) {
    if is_simple_discriminant_type_id(target) {
      return intersect_with_simple_discriminant(builtin_types, arena, discriminant, target);
    }
    return None;
  }
  let s = TypeSimplifier {
    builtin_types,
    arena,
    blocked_types: DenseHashSet::default(),
    recursion_depth: 0,
  };
  let mut seen = DenseHashSet::default();
  s.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
    target,
    discriminant,
    &mut seen,
  )
}
