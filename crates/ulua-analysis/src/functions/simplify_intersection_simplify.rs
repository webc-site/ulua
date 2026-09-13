use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    builtin_types::BuiltinTypes, simplify_result::SimplifyResult, type_arena::TypeArena,
    type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};
pub fn simplify_intersection(
  builtin_types: *mut BuiltinTypes,
  arena: *mut TypeArena,
  left: TypeId,
  right: TypeId,
) -> SimplifyResult {
  let mut s = TypeSimplifier {
    builtin_types: builtin_types as *const BuiltinTypes,
    arena: arena as *const TypeArena,
    blocked_types: DenseHashSet::new(null_mut()),
    recursion_depth: 0,
  };

  let res = s.intersect(left, right);

  SimplifyResult {
    result: res,
    blocked_types: s.blocked_types,
  }
}
