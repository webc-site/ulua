use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, simplify_result::SimplifyResult,
    type_arena::TypeArena, type_ids::TypeIds, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};

pub fn simplify_intersection(
  builtin_types: Handle<BuiltinTypes>,
  arena: Handle<TypeArena>,
  left: TypeId,
  right: TypeId,
) -> SimplifyResult {
  let mut s = TypeSimplifier {
    builtin_types,
    arena,
    blocked_types: DenseHashSet::default(),
    recursion_depth: 0,
  };

  let res = s.intersect(left, right);

  SimplifyResult {
    result: res,
    blocked_types: s.blocked_types,
  }
}

pub fn simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
  builtin_types: Handle<BuiltinTypes>,
  arena: Handle<TypeArena>,
  parts: TypeIds,
) -> SimplifyResult {
  let mut s = TypeSimplifier {
    builtin_types,
    arena,
    blocked_types: DenseHashSet::default(),
    recursion_depth: 0,
  };

  let res = s.intersect_from_parts(parts);

  SimplifyResult {
    result: res,
    blocked_types: s.blocked_types,
  }
}
