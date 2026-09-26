use core::convert::Infallible;

use ulua_common::records::dense_hash_table::{DenseEq, DenseHasher};

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeIdPairHash {
  pub(crate) _unused: Option<Infallible>,
}

impl TypeIdPairHash {
  /// C++ `size_t hashOne(TypeId key) const` (UnifierSharedState.h:16).
  #[inline]
  pub fn hash_one(key: TypeId) -> usize {
    let k = key as usize;
    (k >> 4) ^ (k >> 9)
  }

  /// `size_t operator()(const std::pair<TypeId, TypeId>& x) const`
  /// (UnifierSharedState.h:21-24).
  #[inline]
  pub fn operator_call(&self, x: (TypeId, TypeId)) -> usize {
    Self::hash_one(x.0) ^ (Self::hash_one(x.1) << 1)
  }
}

impl DenseHasher<(TypeId, TypeId)> for TypeIdPairHash {
  /// C++ `size_t operator()(const std::pair<TypeId, TypeId>& x) const`.
  #[inline]
  fn hash(&self, key: &(TypeId, TypeId)) -> usize {
    self.operator_call(*key)
  }
}

impl DenseEq<(TypeId, TypeId)> for TypeIdPairHash {
  #[inline]
  fn eq(&self, a: &(TypeId, TypeId), b: &(TypeId, TypeId)) -> bool {
    a == b
  }
}
