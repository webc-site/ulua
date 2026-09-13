use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::instantiation::Instantiation, type_aliases::type_pack_id::TypePackId};

impl Instantiation {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(false);
    tp
  }
}
