use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow_type_id,
  records::traversal_state::TraversalState,
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack},
};
impl TraversalState {
  pub fn update_current_type_id(&mut self, ty: TypeId) {
    LUAU_ASSERT!(!ty.is_null());
    self.current = TypeOrPack::V0(follow_type_id(ty));
  }
}
