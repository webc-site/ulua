use core::ptr::null_mut;

use ulua_analysis::{
  functions::follow_type_utils::follow_optional_ty, type_aliases::type_id::TypeId,
};
use ulua_common::LUAU_ASSERT;

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn require_type_string(&mut self, name: &str) -> TypeId {
    let ty = self.get_type(name, false);
    LUAU_ASSERT!(ty.is_some());
    follow_optional_ty(ty).unwrap_or(null_mut())
  }
}
