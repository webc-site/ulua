use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::is_simple_discriminant_simplify_alt_b::is_simple_discriminant_type_id,
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};
pub fn intersect_with_simple_discriminant(
  builtin_types: *mut BuiltinTypes,
  arena: *mut TypeArena,
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
    builtin_types: builtin_types as *const _,
    arena: arena as *const _,
    blocked_types: DenseHashSet::new(null_mut()),
    recursion_depth: 0,
  };
  s.intersect_with_simple_discriminant_type_id_type_id(target, discriminant)
}
