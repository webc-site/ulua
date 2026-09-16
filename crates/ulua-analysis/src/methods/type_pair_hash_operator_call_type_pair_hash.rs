use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_id::TypeId};

impl TypePairHash {
  pub fn operator_call(&self, x: (TypeId, TypeId)) -> usize {
    let left = self.hash_one_type_id(x.0);
    let right = self.hash_one_type_id(x.1);
    left ^ (right << 1)
  }
}
