use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{any_type::AnyType, never_type::NeverType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if !get_type_id::<NeverType>(here).is_none() || !get_type_id::<AnyType>(there).is_none() {
      return there;
    }

    here
  }
}
