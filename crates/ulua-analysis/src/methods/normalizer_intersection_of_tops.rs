use crate::{
  functions::get_type,
  records::{any_type::AnyType, never_type::NeverType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    let here_is_never = get_type::get::<NeverType>(here).is_some();
    let there_is_never = get_type::get::<NeverType>(there).is_some();
    let here_is_any = get_type::get::<AnyType>(here).is_some();
    let there_is_any = get_type::get::<AnyType>(there).is_some();

    if here_is_never || there_is_never {
      return self.builtin_types.get().never_type;
    }

    if here_is_any || there_is_any {
      return self.builtin_types.get().any_type;
    }

    self.builtin_types.get().unknown_type
  }
}
