use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  builtin_types::BuiltinTypes, simplify_result::SimplifyResult, type_arena::TypeArena,
  type_ids::TypeIds, type_simplifier::TypeSimplifier,
};
pub fn simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
  builtin_types: *mut BuiltinTypes,
  arena: *mut TypeArena,
  parts: TypeIds,
) -> SimplifyResult {
  let mut s = TypeSimplifier {
    builtin_types: builtin_types as *const BuiltinTypes,
    arena: arena as *const TypeArena,
    blocked_types: DenseHashSet::new(null_mut()),
    recursion_depth: 0,
  };

  let res = s.intersect_from_parts(parts);

  SimplifyResult {
    result: res,
    blocked_types: s.blocked_types,
  }
}
