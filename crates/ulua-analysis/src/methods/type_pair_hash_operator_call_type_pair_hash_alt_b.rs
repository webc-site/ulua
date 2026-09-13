use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_pack_id::TypePackId};

impl TypePairHash {
  pub fn operator_call_2(&self, x: (TypePackId, TypePackId)) -> usize {
    self.hash_one_type_pack_id(x.0) ^ (self.hash_one_type_pack_id(x.1) << 1)
  }
}
