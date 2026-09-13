use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, normalized_type::NormalizedType,
    normalizer::Normalizer, primitive_type::PrimitiveType, singleton_type::SingletonType,
    string_singleton::StringSingleton,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn subtract_singleton(&mut self, here: &mut NormalizedType, ty: TypeId) {
    self.consume_fuel();

    let stv = get_type_id::<SingletonType>(ty).unwrap();

    if let Some(ss) = stv.variant.get_if::<StringSingleton>() {
      if here.strings.is_cofinite {
        here.strings.singletons.insert(ss.value.clone(), ty);
      } else {
        let it = here.strings.singletons.get_mut(&ss.value);
        if it.is_some() {
          here.strings.singletons.remove(&ss.value);
        }
      }
    } else if let Some(bs) = stv.variant.get_if::<BooleanSingleton>() {
      if get_type_id::<NeverType>(here.booleans).is_some() {
        // Nothing
      } else if let Some(prim) = get_type_id::<PrimitiveType>(here.booleans) {
        if prim.r#type == PrimitiveType::BOOLEAN {
          here.booleans = if bs.value {
            unsafe { (*self.builtin_types).false_type }
          } else {
            unsafe { (*self.builtin_types).true_type }
          };
        }
      } else if let Some(here_singleton) = get_type_id::<SingletonType>(here.booleans)
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      {
        // Crucial subtlety: ty (and thus bs) are the value that is being
        // negated out. We therefore reduce to never when the values match,
        // rather than when they differ.
        if bs.value == here_singleton.value {
          here.booleans = unsafe { (*self.builtin_types).never_type };
        }
      } else {
        LUAU_ASSERT!(false);
      }
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
