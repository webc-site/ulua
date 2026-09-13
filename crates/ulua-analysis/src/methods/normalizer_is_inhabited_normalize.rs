use core::ptr::null_mut;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  records::{
    fuel_initializer::FuelInitializer, normalized_type::NormalizedType, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
  },
  type_aliases::type_id::TypeId,
};
impl Normalizer {
  pub fn is_inhabited_normalized_type(&mut self, norm: &NormalizedType) -> NormalizationResult {
    match catch_unwind(AssertUnwindSafe(|| {
      let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());

      let mut fi = FuelInitializer {
        normalizer: self as *mut Normalizer,
        initialized_fuel: false,
      };
      unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
      let _fi = fi;

      self.is_inhabited_normalized_type_set_type_id(norm, &mut seen)
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => {
        NormalizationResult::HitLimits
      }
      Err(payload) => resume_unwind(payload),
    }
  }
}
