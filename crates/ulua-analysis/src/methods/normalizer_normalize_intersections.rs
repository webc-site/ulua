use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{normalized_type::NormalizedType, normalizer::Normalizer},
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  pub fn normalize_intersections(
    &mut self,
    intersections: &Vec<TypeId>,
    out_type: &mut NormalizedType,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    if self.arena.is_null() {
      // C++: sharedState->iceHandler->ice("Normalizing types outside a module")
      unsafe {
        (*(*self.shared_state).ice_handler).ice_string("Normalizing types outside a module");
      }
    }

    self.consume_fuel();

    // NormalizedType norm{builtinTypes}; norm.tops = builtinTypes->unknownType;
    let mut norm = fresh_normalized_type(self.builtin_types);
    norm.tops = unsafe { (*self.builtin_types).unknown_type };

    for &ty in intersections {
      let res = self.intersect_normal_with_ty(&mut norm, ty, seen_table_prop_pairs, seen_set);
      if res != NormalizationResult::True {
        return res;
      }
    }

    let res = self.union_normals(out_type, &norm, -1);
    if res != NormalizationResult::True {
      return res;
    }

    NormalizationResult::True
  }
}
