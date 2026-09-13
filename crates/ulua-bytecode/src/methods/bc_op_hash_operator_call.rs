use std::{collections::hash_map::DefaultHasher, hash::BuildHasher};

use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::{bc_op::BcOp, bc_op_hash::BcOpHash};

impl BcOpHash {
  pub fn operator_call(&self, p: &BcOp) -> usize {
    ((p.kind as usize) & 0x0F) | ((p.index as usize) << 4)
  }
}

impl BuildHasher for BcOpHash {
  type Hasher = DefaultHasher;
  fn build_hasher(&self) -> Self::Hasher {
    DefaultHasher::new()
  }
}

impl DenseHasher<BcOp> for BcOpHash {
  fn hash(&self, key: &BcOp) -> usize {
    self.operator_call(key)
  }
}
