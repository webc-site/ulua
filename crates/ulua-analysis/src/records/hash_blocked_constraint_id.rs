use ulua_common::records::{dense_hash_table::DenseHasher, variant::Variant3};

use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;

// C++ (ConstraintGraph.h:21-24): a hash functor over BlockedConstraintId;
#[derive(Debug, Clone, Copy, Default)]
pub struct HashBlockedConstraintId;

impl HashBlockedConstraintId {
  /// C++ `size_t HashBlockedConstraintId::operator()(const BlockedConstraintId& bci) const`
  /// (ConstraintGraph.cpp:42). `std::hash<T*>` is the identity on the pointer
  /// value, so V0/V1 hashes to its raw pointer cast to `usize`; V2（§2 句柄化后）
  /// hashes to the registry `u32` id —— 同址⇔同句柄的双射保证键面判等/分桶行为
  /// 与迁移前指针哈希等价（桶内具体散列值随进程注册顺序变化，非可观测面）。
  #[inline]
  pub fn operator_call(&self, bci: &BlockedConstraintId) -> usize {
    match bci {
      Variant3::V0(ty) => *ty as usize,
      Variant3::V1(tp) => *tp as usize,
      Variant3::V2(c) => c.0 as usize,
    }
  }
}

// Bridge the C++ `Hash` template parameter (`HashBlockedConstraintId`) to the
// `DenseHasher` trait the `DenseHashMap` port is generic over.
impl DenseHasher<BlockedConstraintId> for HashBlockedConstraintId {
  #[inline]
  fn hash(&self, key: &BlockedConstraintId) -> usize {
    self.operator_call(key)
  }
}
