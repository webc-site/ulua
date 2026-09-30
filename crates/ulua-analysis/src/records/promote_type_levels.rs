//! Source: `Analysis/src/Unifier.cpp:23-141`
//!
//! C++ `struct PromoteTypeLevels : TypeOnceVisitor`。visit 覆写经
//! `GenericTypeVisitorTrait`（`methods/promote_type_levels_traverse.rs`）进入
//! 遍历路径；此处只保留数据、构造与 `promote*` 辅助。

use alloc::string::String;

use crate::{
  records::{
    arena_id::ArenaId, txn_log::TxnLog, type_arena::TypeArena, type_level::TypeLevel,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct PromoteTypeLevels {
  pub base: TypeOnceVisitor,
  pub log: *mut TxnLog,
  /// 本 pass 判归属用的 arena 身份（cpp `TypeArena* typeArena`）。
  pub type_arena_id: ArenaId,
  pub min_level: TypeLevel,
}

impl PromoteTypeLevels {
  pub fn new(log: &mut TxnLog, type_arena: &TypeArena, min_level: TypeLevel) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("PromoteTypeLevels"), false),
      log: log as *mut TxnLog,
      type_arena_id: type_arena.arena_id,
      min_level,
    }
  }

  /// C++ `promote(TID ty, T* t)`（Unifier.cpp:35-41）。C++ 的 `t` 仅供模板
  /// 推导与 `assert(t)`，Rust 侧无对应用途，故去除死参，只保留提升语义。
  pub(crate) fn promote(&mut self, ty: TypeId, level: TypeLevel) {
    if self.min_level.subsumes_strict(&level) {
      unsafe {
        if let Some(log) = self.log.as_mut() {
          log.change_level_type_id_type_level(ty, self.min_level);
        }
      }
    }
  }

  pub(crate) fn promote_pack(&mut self, tp: TypePackId, level: TypeLevel) {
    if self.min_level.subsumes_strict(&level) {
      unsafe {
        if let Some(log) = self.log.as_mut() {
          log.change_level_type_pack_id_type_level(tp, self.min_level);
        }
      }
    }
  }
}
