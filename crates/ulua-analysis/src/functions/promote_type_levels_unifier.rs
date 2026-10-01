use crate::{
  records::{
    arena_handle::alias_ref, generic_type_visitor::GenericTypeVisitorTrait,
    promote_type_levels::PromoteTypeLevels, txn_log::TxnLog, type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};
/// C++ `promoteTypeLevels(log, arena, minLevel, ty)`（Unifier.cpp:124-131）。
///
/// Rust 形态（§2）：入参已是引用（原 `unsafe fn` 只是照抄 C++ 指针面的残留），
/// `ty` 的归属比较经 `alias_ref` 门面读取 `owning_arena`，业务侧无裸指针解引用。
pub fn promote_type_levels_txn_log_type_arena_type_level_type_id(
  log: &mut TxnLog,
  type_arena: &TypeArena,
  min_level: TypeLevel,
  ty: TypeId,
) {
  // Type levels of types from other modules are already global, so we don't need to promote anything inside
  if alias_ref(ty).owning_arena != type_arena.arena_id {
    return;
  }

  let mut ptl = PromoteTypeLevels::new(log, type_arena, min_level);
  // C++ `ptl.traverse(ty)` (Unifier.cpp:130) — drive the real GenericTypeVisitor
  // traversal so the per-node visits recurse and `log.changeLevel(...)` fires.
  ptl.traverse_type_id(ty);
}
