use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::follow_type::follow_type_id,
  methods::fresh_normalized_type::fresh_normalized_type,
  records::normalizer::Normalizer,
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  pub fn is_intersection_inhabited_type_id_type_id_seen_table_prop_pairs_set_type_id(
    &mut self,
    left: TypeId,
    right: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    self.consume_fuel();

    let left = follow_type_id(left);
    let right = follow_type_id(right);

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

    if self.cache_inhabitance {
      if result == NormalizationResult::True {
        *self
          .cached_is_inhabited_intersection
          .get_or_insert((left, right)) = true;
      } else if result == NormalizationResult::False {
        *self
          .cached_is_inhabited_intersection
          .get_or_insert((left, right)) = false;
      }
    }

    norm.normalized_type_destructor();

    result
  }
}
