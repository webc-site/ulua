use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::is_shallow_inhabited::is_shallow_inhabited,
  records::normalizer::Normalizer,
  type_aliases::{
    normalized_tyvars::NormalizedTyvars, seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId,
  },
};

impl Normalizer {
  pub fn intersect_tyvars_with_ty(
    &mut self,
    here: &mut NormalizedTyvars,
    there: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set_types: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    self.consume_fuel();

    let mut it = here.iter_mut();
    while let Some((_, inter_box)) = it.next() {
      let inter = inter_box.as_mut();
      let res = self.intersect_normal_with_ty(inter, there, seen_table_prop_pairs, seen_set_types);
      if res != NormalizationResult::True {
        return res;
      }
      if is_shallow_inhabited(inter) {
        // keep this entry
      } else {
        // remove this entry
        it = here.iter_mut();
        continue;
      }
    }
    NormalizationResult::True
  }
}
