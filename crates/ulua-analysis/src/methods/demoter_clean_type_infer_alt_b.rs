use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{fresh_index::fresh_index, get_type_pack::get_type_pack_id},
  records::{demoter::Demoter, free_type_pack::FreeTypePack, type_pack_var::TypePackVar},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
impl Demoter {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let ftp = get_type_pack_id::<FreeTypePack>(tp);
    LUAU_ASSERT!(ftp.is_some());
    // C++ `LUAU_ASSERT(ftp)` 必命中，此处 unwrap 安全。
    let demoted_level = self.demoted_level(ftp.unwrap().level);
    let ftp_var = FreeTypePack {
      index: fresh_index(),
      level: demoted_level,
      scope: null_mut(),
      polarity: Polarity::Unknown,
    };

    let ty_pack_var = TypePackVar {
      ty: TypePackVariant::Free(ftp_var),
      persistent: false,
      owning_arena: null_mut(),
    };

    // SAFETY: arena 在 Demoter 存活期内有效。
    unsafe { (*self.arena).add_type_pack_t(ty_pack_var) }
  }
}
