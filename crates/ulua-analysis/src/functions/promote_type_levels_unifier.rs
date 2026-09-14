use std::ptr::eq;

use crate::{
  records::{
    generic_type_visitor::GenericTypeVisitorTrait, promote_type_levels::PromoteTypeLevels,
    txn_log::TxnLog, type_arena::TypeArena, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn promote_type_levels_txn_log_type_arena_type_level_type_id(
  log: &mut TxnLog,
  type_arena: &TypeArena,
  min_level: TypeLevel,
  ty: TypeId,
) {
  // Type levels of types from other modules are already global, so we don't need to promote anything inside
  if unsafe { !eq((*ty).owning_arena, type_arena) } {
    return;
  }

  let mut ptl = PromoteTypeLevels::new(log, type_arena, min_level);
  // C++ `ptl.traverse(ty)` (Unifier.cpp:130) — drive the real GenericTypeVisitor
  // traversal so the per-node visits recurse and `log.changeLevel(...)` fires.
  ptl.traverse_type_id(ty);
}
