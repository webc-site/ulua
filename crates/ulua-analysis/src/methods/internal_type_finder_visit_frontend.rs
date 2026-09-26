use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack, extern_type::ExternType,
    free_type::FreeType, free_type_pack::FreeTypePack, internal_type_finder::InternalTypeFinder,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl InternalTypeFinder {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }

  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
    LUAU_ASSERT!(false);
    false
  }

  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
    LUAU_ASSERT!(false);
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _pet: &PendingExpansionType,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }

  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }

  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    _tp: TypePackId,
    _ftp: &FreeTypePack,
  ) -> bool {
    false
  }

  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    _tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }
}
