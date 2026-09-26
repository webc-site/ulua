use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type,
  records::{
    boolean_singleton::BooleanSingleton, singleton_type::SingletonType,
    string_singleton::StringSingleton, widen::Widen,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Widen {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let stv = get_type::get::<SingletonType>(ty);
    LUAU_ASSERT!(stv.is_some());

    // 紧邻 LUAU_ASSERT(stv.is_some()) 蕴含 Some。
    let stv_ref = stv.expect("紧邻 LUAU_ASSERT(stv.is_some()) 蕴含");

    if stv_ref.variant.get_if::<StringSingleton>().is_some() {
      self.builtin_types.get().string_type
    } else {
      // If this assert trips, it's likely we now have number singletons.
      LUAU_ASSERT!(stv_ref.variant.get_if::<BooleanSingleton>().is_some());
      self.builtin_types.get().boolean_type
    }
  }

  pub fn clean_type_pack_id(&mut self, _tp: TypePackId) -> TypePackId {
    panic!("Widen attempted to clean a dirty type pack?");
  }
}
