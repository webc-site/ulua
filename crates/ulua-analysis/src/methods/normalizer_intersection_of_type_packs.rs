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
    // 对齐 cpp `FuelInitializer fi{NotNull{this}}`：构造即 initialize_fuel，
    // 命中资源上限 resume_unwind 时由 Drop 清理 fuel。
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };
    unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
    self.intersection_of_type_packs_internal(here, there)
  }
}
