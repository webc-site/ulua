use alloc::{collections::BTreeMap, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::follow_type::follow_type_id,
  records::{
    normalized_extern_type::NormalizedExternType, normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType, normalized_type::NormalizedType,
    normalizer::Normalizer, type_ids::TypeIds,
  },
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

    let never_type = unsafe { (*self.builtin_types).never_type };
    let mut norm = NormalizedType {
      builtin_types: self.builtin_types,
      tops: never_type,
      booleans: never_type,
      extern_types: NormalizedExternType {
        extern_types: BTreeMap::new(),
        shape_extensions: TypeIds::new(),
        ordering: Vec::new(),
      },
      errors: never_type,
      nils: never_type,
      numbers: never_type,
      integers: never_type,
      strings: NormalizedStringType::NEVER,
      threads: never_type,
      buffers: never_type,
      tables: TypeIds::new(),
      functions: NormalizedFunctionType {
        is_top: false,
        parts: TypeIds::new(),
      },
      tyvars: BTreeMap::new(),
      is_cacheable: true,
    };

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
