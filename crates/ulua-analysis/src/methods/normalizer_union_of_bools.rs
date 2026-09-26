use crate::{
  functions::get_type,
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, normalizer::Normalizer,
    singleton_type::SingletonType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_of_bools(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if get_type::get::<NeverType>(here).is_some() {
      return there;
    }
    if get_type::get::<NeverType>(there).is_some() {
      return here;
    }

    if let Some(hbool) = get_type::get::<SingletonType>(here)
      .as_ref()
      .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      && let Some(tbool) = get_type::get::<SingletonType>(there)
        .as_ref()
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      && hbool.value == tbool.value
    {
      return here;
    }

    self.builtin_types.get().boolean_type
  }
}
