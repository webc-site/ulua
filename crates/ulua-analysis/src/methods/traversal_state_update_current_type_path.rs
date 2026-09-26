use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, follow_type_pack},
  records::traversal_state::TraversalState,
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TraversalState {
  pub fn update_current_type_id(&mut self, ty: TypeId) {
    LUAU_ASSERT!(!ty.is_null());
    self.current = TypeOrPack::V0(follow_type::follow(ty));
  }

  pub(crate) fn update_current_type_pack_id(&mut self, tp: TypePackId) {
    LUAU_ASSERT!(!tp.is_null());
    self.current = TypeOrPack::V1(follow_type_pack::follow(tp));
  }
}
