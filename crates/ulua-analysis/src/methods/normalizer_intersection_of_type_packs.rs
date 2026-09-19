use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::{
  records::{
    fuel_initializer::FuelInitializer, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
  },
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

    // 对齐 cpp try/catch：仅捕获 NormalizerHitLimits 并返回 None，其余 panic 继续传播。
    match catch_unwind(AssertUnwindSafe(|| {
      self.intersection_of_type_packs_internal(here, there)
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => None,
      Err(payload) => resume_unwind(payload),
    }
  }
}
