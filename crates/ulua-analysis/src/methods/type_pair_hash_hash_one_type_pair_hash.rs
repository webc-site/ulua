use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_id::TypeId};

impl TypePairHash {
  pub fn hash_one_type_id(&self, key: TypeId) -> usize {
    ((key as usize) >> 4) ^ ((key as usize) >> 9)
  }
}
