use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::records::{generic_pack_mapping::GenericPackMapping, path_hash::PathHash};

impl PathHash {
  pub fn operator_call_2(&self, mapping: &GenericPackMapping) -> usize {
    let mapped_type = mapping.mapped_type;
    let mut s = DefaultHasher::new();
    mapped_type.hash(&mut s);
    s.finish() as usize
  }
}
