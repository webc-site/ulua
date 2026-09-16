use alloc::string::String;

use crate::{
  records::{
    blocked_type::BlockedType, extern_type::ExternType, free_type::FreeType,
    free_type_pack::FreeTypePack, pending_expansion_type::PendingExpansionType,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct InternalTypeFinder {
  pub base: TypeOnceVisitor,
}

impl InternalTypeFinder {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("InternalTypeFinder"), true),
    }
  }

  pub fn visit_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }

  pub fn visit_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
    ulua_common::LUAU_ASSERT!(false);
    false
  }

  pub fn visit_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
    ulua_common::LUAU_ASSERT!(false);
    false
  }

  pub fn visit_pending_expansion_type(&mut self, _ty: TypeId, _pet: &PendingExpansionType) -> bool {
    ulua_common::LUAU_ASSERT!(false);
    false
  }

  pub fn visit_free_type_pack(&mut self, _tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    ulua_common::LUAU_ASSERT!(false);
    false
  }
}

impl Default for InternalTypeFinder {
  fn default() -> Self {
    Self::new()
  }
}
