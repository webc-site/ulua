use crate::{records::type_id_pair_hash::TypeIdPairHash, type_aliases::type_id::TypeId};

impl TypeIdPairHash {
  /// `size_t operator()(const std::pair<TypeId, TypeId>& x) const`
  /// (UnifierSharedState.h:21-24). `hashOne(key) = (key>>4) ^ (key>>9)`.
  pub fn operator_call(&self, x: (TypeId, TypeId)) -> usize {
    let hash_one = |key: TypeId| -> usize {
      let k = key as usize;
      (k >> 4) ^ (k >> 9)
    };
    hash_one(x.0) ^ (hash_one(x.1) << 1)
  }
}
