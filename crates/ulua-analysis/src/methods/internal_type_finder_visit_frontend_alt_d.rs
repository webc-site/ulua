use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    internal_type_finder::InternalTypeFinder, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl InternalTypeFinder {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _pet: &PendingExpansionType,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }
}
