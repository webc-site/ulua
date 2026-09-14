use alloc::{collections::BTreeMap, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  records::{
    normalized_extern_type::NormalizedExternType, normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType, normalized_type::NormalizedType,
    normalizer::Normalizer, type_ids::TypeIds,
  },
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
      panic!("Normalizing types outside a module");
    }

    self.consume_fuel();

    let never_type = unsafe { (*self.builtin_types).never_type };
    let mut norm = NormalizedType {
      builtin_types: self.builtin_types,
      tops: unsafe { (*self.builtin_types).unknown_type },
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
