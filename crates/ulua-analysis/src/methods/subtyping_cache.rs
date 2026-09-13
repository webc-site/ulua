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
    self.operator_call(*x)
  }
}

// C++ `TypePairHash` provides `operator()` overloads for both `std::pair<TypeId,
// TypeId>` and `std::pair<TypePackId, TypePackId>` (used by `SeenTypePackSet`).
impl DenseHasher<(TypePackId, TypePackId)> for TypePairHash {
  fn hash(&self, x: &(TypePackId, TypePackId)) -> usize {
    self.operator_call_2(*x)
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
