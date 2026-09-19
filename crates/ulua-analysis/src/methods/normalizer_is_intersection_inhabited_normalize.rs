//! Source: `Analysis/src/Normalize.cpp:542-555` (hand-ported)
use core::ptr::null;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  records::{
    fuel_initializer::FuelInitializer, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
  },
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  /// C++ `NormalizationResult Normalizer::isIntersectionInhabited(TypeId left, TypeId right)`.
  /// Builds the seen sets, initializes normalization fuel, and delegates to the
  /// seen-set overload. The Rust port does not model C++ exceptions; the
  /// `NormalizerHitLimits` path surfaces through the delegate's return value.
  pub fn is_intersection_inhabited_type_id_type_id(
    &mut self,
    left: TypeId,
    right: TypeId,
  ) -> NormalizationResult {
    match catch_unwind(AssertUnwindSafe(|| {
      let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
      let mut seen_table_prop_pairs: SeenTablePropPairs = SeenTablePropPairs::new((null(), null()));

      let mut fi = FuelInitializer {
        normalizer: self as *mut Normalizer,
        initialized_fuel: false,
      };
      unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
      let _fi = fi;

      self.is_intersection_inhabited_type_id_type_id_seen_table_prop_pairs_set_type_id(
        left,
        right,
        &mut seen_table_prop_pairs,
        &mut seen,
      )
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => {
        NormalizationResult::HitLimits
      }
      Err(payload) => resume_unwind(payload),
    }
  }
}
