use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::records::{path_hash::PathHash, property_type_path::Property};
impl PathHash {
  pub fn operator_call_7(&self, prop: &Property) -> usize {
    let mut hasher = DefaultHasher::new();
    prop.name().hash(&mut hasher);
    let hash_name = hasher.finish() as usize;

    let hash_read = if prop.is_read() { 1usize } else { 0usize };

    hash_name ^ hash_read
  }
}
