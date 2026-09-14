use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{free_type::FreeType, internal_type_finder::InternalTypeFinder},
  type_aliases::type_id::TypeId,
};

impl InternalTypeFinder {
  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
    LUAU_ASSERT!(false);
    false
  }
}
