use alloc::string::String;
use std::ptr::eq;

use crate::{
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack, txn_log::TxnLog, type_arena::TypeArena,
    type_level::TypeLevel, type_once_visitor::TypeOnceVisitor, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct PromoteTypeLevels {
  pub base: TypeOnceVisitor,
  pub log: *mut TxnLog,
  pub type_arena: *const TypeArena,
  pub min_level: TypeLevel,
}

impl PromoteTypeLevels {
  pub fn new(log: &mut TxnLog, type_arena: &TypeArena, min_level: TypeLevel) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("PromoteTypeLevels"), false),
      log: log as *mut TxnLog,
      type_arena,
      min_level,
    }
  }

  pub(crate) fn promote<T>(&mut self, ty: TypeId, t: *mut T, level: TypeLevel) {
    ulua_common::LUAU_ASSERT!(!t.is_null());
    if self.min_level.subsumes_strict(&level) {
      unsafe {
        if let Some(log) = self.log.as_mut() {
          log.change_level_type_id_type_level(ty, self.min_level);
        }
      }
    }
  }

  pub(crate) fn promote_pack<T>(&mut self, tp: TypePackId, t: *mut T, level: TypeLevel) {
    ulua_common::LUAU_ASSERT!(!t.is_null());
    if self.min_level.subsumes_strict(&level) {
      unsafe {
        if let Some(log) = self.log.as_mut() {
          log.change_level_type_pack_id_type_level(tp, self.min_level);
        }
      }
    }
  }

  pub fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    unsafe {
      let tp_var: *const TypePackVar = tp;
      if !eq((*tp_var).owning_arena, self.type_arena) {
        return false;
      }
    }
    true
  }

  pub fn visit_free_type(&mut self, ty: TypeId, _ft: &FreeType) -> bool {
    unsafe {
      if !(*self.log).txn_log_is::<FreeType, TypeId>(ty) {
        return true;
      }
      let ft = (*self.log).txn_log_get_mutable::<FreeType, TypeId>(ty);
      self.promote(ty, ft, (*ft).level);
    }
    true
  }

  pub fn visit_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    unsafe {
      if !(*self.log).txn_log_is::<FreeTypePack, TypePackId>(tp) {
        return true;
      }
      let _ftp = (*self.log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp);
      // Assuming FreeTypePack has a level field
      // self.promote_pack(tp, ftp, (*ftp).level);
    }
    true
  }
}
