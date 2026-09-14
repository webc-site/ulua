use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{records::type_cacher::TypeCacher, type_aliases::type_id::TypeId};

impl TypeCacher {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    LUAU_ASSERT!(false);
    LUAU_UNREACHABLE!();
  }
}
