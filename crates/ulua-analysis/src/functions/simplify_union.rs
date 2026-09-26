//! Source: `Analysis/src/Simplify.cpp:2019-2028` (hand-ported)

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, simplify_result::SimplifyResult,
    type_arena::TypeArena, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};
pub fn simplify_union(
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

  let res = s.union_(left, right);

  SimplifyResult {
    result: res,
    blocked_types: s.blocked_types,
  }
}
