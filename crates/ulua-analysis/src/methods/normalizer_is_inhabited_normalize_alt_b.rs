use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn is_inhabited_normalized_type_set_type_id(
    &mut self,
    norm: &NormalizedType,
    seen: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    if FFlag::LuauIntegerType2.get() {
      if get_type_id::<NeverType>(norm.tops).is_none()
        || get_type_id::<NeverType>(norm.booleans).is_none()
        || get_type_id::<NeverType>(norm.errors).is_none()
        || get_type_id::<NeverType>(norm.nils).is_none()
        || get_type_id::<NeverType>(norm.numbers).is_none()
        || get_type_id::<NeverType>(norm.threads).is_none()
        || get_type_id::<NeverType>(norm.buffers).is_none()
        || !norm.extern_types.is_never()
        || get_type_id::<NeverType>(norm.integers).is_none()
        || !norm.strings.is_never()
        || !norm.functions.is_never()
      {
        return NormalizationResult::True;
      }
    } else {
      if get_type_id::<NeverType>(norm.tops).is_none()
        || get_type_id::<NeverType>(norm.booleans).is_none()
        || get_type_id::<NeverType>(norm.errors).is_none()
        || get_type_id::<NeverType>(norm.nils).is_none()
        || get_type_id::<NeverType>(norm.numbers).is_none()
        || get_type_id::<NeverType>(norm.threads).is_none()
        || get_type_id::<NeverType>(norm.buffers).is_none()
        || !norm.extern_types.is_never()
        || !norm.strings.is_never()
        || !norm.functions.is_never()
      {
        return NormalizationResult::True;
      }
    }

    for intersect in norm.tyvars.values() {
      let res = self.is_inhabited_normalized_type_set_type_id(intersect, seen);
      if res != NormalizationResult::False {
        return res;
      }
    }

    for &table in &norm.tables.order {
      let res = self.is_inhabited_type_id_set_type_id(table, seen);
      if res != NormalizationResult::False {
        return res;
      }
    }

    NormalizationResult::False
  }
}
