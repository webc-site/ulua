use alloc::vec::Vec;
use core::ptr::null;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::follow_type,
  methods::fresh_normalized_type::fresh_normalized_type,
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
      let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
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

  pub fn is_intersection_inhabited_type_id_type_id_seen_table_prop_pairs_set_type_id(
    &mut self,
    left: TypeId,
    right: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    self.consume_fuel();

    let left = follow_type::follow(left);
    let right = follow_type::follow(right);

    if self.cache_inhabitance
      && let Some(result) = self.cached_is_inhabited_intersection.find(&(left, right))
    {
      return if *result {
        NormalizationResult::True
      } else {
        NormalizationResult::False
      };
    }

    let mut norm = fresh_normalized_type(self.builtin_types);

    let res = self.normalize_intersections(
      &Vec::from([left, right]),
      &mut norm,
      seen_table_prop_pairs,
      seen_set,
    );

    if res != NormalizationResult::True {
      if self.cache_inhabitance && res == NormalizationResult::False {
        *self
          .cached_is_inhabited_intersection
          .get_or_insert((left, right)) = false;
      }
      return res;
    }

    let result = self.is_inhabited_normalized_type_set_type_id(&norm, seen_set);

    // 缓存判定结果;HitLimits 不入缓存
    if self.cache_inhabitance
      && let Some(inhabited) = match result {
        NormalizationResult::True => Some(true),
        NormalizationResult::False => Some(false),
        NormalizationResult::HitLimits => None,
      }
    {
      *self
        .cached_is_inhabited_intersection
        .get_or_insert((left, right)) = inhabited;
    }

    result
  }
}
