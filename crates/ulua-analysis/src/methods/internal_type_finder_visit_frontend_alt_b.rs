use crate::{
  records::{blocked_type::BlockedType, internal_type_finder::InternalTypeFinder},
  type_aliases::type_id::TypeId,
};

impl InternalTypeFinder {
  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
    ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    false
  }
}
