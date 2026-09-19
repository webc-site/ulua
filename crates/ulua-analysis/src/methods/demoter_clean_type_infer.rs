use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{demoter::Demoter, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

impl Demoter {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let ftv = get_type_id::<FreeType>(ty);
    LUAU_ASSERT!(ftv.is_some());
    // C++ `LUAU_ASSERT(ftv)` 必命中，此处 unwrap 安全。
    let demoted_level = self.demoted_level(ftv.unwrap().level);
    let arena = unsafe { &mut *self.arena };
    arena.fresh_type_not_null_builtin_types_type_level(unsafe { &*self.builtins }, demoted_level)
  }
}
