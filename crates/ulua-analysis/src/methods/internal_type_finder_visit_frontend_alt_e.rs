use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{blocked_type_pack::BlockedTypePack, internal_type_finder::InternalTypeFinder},
  type_aliases::type_pack_id::TypePackId,
};

impl InternalTypeFinder {
  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }
}
