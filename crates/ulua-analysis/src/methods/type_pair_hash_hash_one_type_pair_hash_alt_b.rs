use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_pack_id::TypePackId};

impl TypePairHash {
  pub fn hash_one_type_pack_id(&self, key: TypePackId) -> usize {
    ((key as usize) >> 4) ^ ((key as usize) >> 9)
  }
}
