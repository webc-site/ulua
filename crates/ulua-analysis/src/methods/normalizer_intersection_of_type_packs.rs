use crate::{
  records::{fuel_initializer::FuelInitializer, normalizer::Normalizer},
  type_aliases::type_pack_id::TypePackId,
};

impl Normalizer {
  pub fn intersection_of_type_packs(
    &mut self,
    here: TypePackId,
    there: TypePackId,
  ) -> Option<TypePackId> {
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };

    fi.fuel_initializer_fuel_initializer();
    let result = self.intersection_of_type_packs_internal(here, there);
    fi.fuel_initializer_fuel_initializer_destructor();

    result
  }
}
