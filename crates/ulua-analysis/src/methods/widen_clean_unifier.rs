use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    boolean_singleton::BooleanSingleton, singleton_type::SingletonType,
    string_singleton::StringSingleton, widen::Widen,
  },
  type_aliases::type_id::TypeId,
};
impl Widen {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let stv = get_type_id::<SingletonType>(ty);
    LUAU_ASSERT!(!stv.is_none());

    let stv_ref = stv.as_ref().unwrap();

    if stv_ref.variant.get_if::<StringSingleton>().is_some() {
      unsafe { (*self.builtin_types).string_type }
    } else {
      // If this assert trips, it's likely we now have number singletons.
      LUAU_ASSERT!(stv_ref.variant.get_if::<BooleanSingleton>().is_some());
      unsafe { (*self.builtin_types).boolean_type }
    }
  }
}
