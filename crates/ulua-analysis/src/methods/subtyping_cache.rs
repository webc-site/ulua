use ulua_common::records::dense_hash_table::DenseHasher;

use crate::{
  records::{
    subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_pair_hash::TypePairHash,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl DenseHasher<(TypeId, TypeId)> for TypePairHash {
  fn hash(&self, x: &(TypeId, TypeId)) -> usize {
    let left = self.hash_one_type_id(x.0);
    let right = self.hash_one_type_id(x.1);
    left ^ (right << 1)
  }
}

// C++ `TypePairHash` provides `operator()` overloads for both `std::pair<TypeId,
// TypeId>` and `std::pair<TypePackId, TypePackId>` (used by `SeenTypePackSet`).
impl DenseHasher<(TypePackId, TypePackId)> for TypePairHash {
  fn hash(&self, x: &(TypePackId, TypePackId)) -> usize {
    self.hash_one_type_pack_id(x.0) ^ (self.hash_one_type_pack_id(x.1) << 1)
  }
}

impl Subtyping {
  pub fn cache(
    &mut self,
    _env: &mut SubtypingEnvironment,
    result: SubtypingResult,
    sub_ty: TypeId,
    super_ty: TypeId,
  ) -> SubtypingResult {
    let p = (sub_ty, super_ty);

    if result.is_cacheable {
      *self.result_cache.get_or_insert(p) = result.clone();
    }

    result
  }
}
