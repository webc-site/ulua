use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, normalizer::Normalizer,
    singleton_type::SingletonType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_of_bools(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if !get_type_id::<NeverType>(here).is_none() {
      return here;
    }
    if !get_type_id::<NeverType>(there).is_none() {
      return there;
    }

    if let Some(hbool) = get_type_id::<SingletonType>(here)
      .as_ref()
      .and_then(|s| s.variant.get_if::<BooleanSingleton>())
    {
      if let Some(tbool) = get_type_id::<SingletonType>(there)
        .as_ref()
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      {
        if hbool.value == tbool.value {
          return here;
        } else {
          return unsafe { (*self.builtin_types).never_type };
        }
      }

      return here;
    }

    there
  }
}
