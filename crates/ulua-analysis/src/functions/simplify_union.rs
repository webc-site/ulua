use core::{mem::replace, ptr::null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    builtin_types::BuiltinTypes, simplify_result::SimplifyResult, type_arena::TypeArena,
    type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};
pub fn simplify_union(
  builtin_types: *mut BuiltinTypes,
  arena: *mut TypeArena,
  left: TypeId,
  right: TypeId,
) -> SimplifyResult {
  simplify_union_impl(builtin_types, arena, left, right)
}

// 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn simplify_union_impl(
  builtin_types: *mut BuiltinTypes,
  arena: *mut TypeArena,
  left: TypeId,
  right: TypeId,
) -> SimplifyResult {
  let builtin_types = unsafe { builtin_types.as_ref() }.expect("builtin_types is null");
  let arena = unsafe { arena.as_ref() }.expect("arena is null");

  let mut s = TypeSimplifier {
    builtin_types,
    arena,
    blocked_types: DenseHashSet::new(null_mut()),
    recursion_depth: 0,
  };

  let res = s.union_(left, right);

  SimplifyResult {
    result: res,
    blocked_types: replace(&mut s.blocked_types, DenseHashSet::new(null_mut())),
  }
}
