use crate::{
  functions::get_type,
  records::{any_type::AnyType, never_type::NeverType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if get_type::get::<NeverType>(here).is_some() || get_type::get::<AnyType>(there).is_some() {
      return there;
    }

    here
  }
}
