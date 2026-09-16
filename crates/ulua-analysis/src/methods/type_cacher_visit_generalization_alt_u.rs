use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{records::type_cacher::TypeCacher, type_aliases::type_pack_id::TypePackId};

impl TypeCacher {
  pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    LUAU_ASSERT!(false);
    LUAU_UNREACHABLE!();
  }
}
