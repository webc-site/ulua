use crate::{
  records::{normalizer::Normalizer, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_extern_types_type_ids_type_ids(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    for there in theres.begin() {
      let there: TypeId = there;
      self.union_extern_types_with_extern_type_type_ids_type_id(heres, there);
    }
  }
}
