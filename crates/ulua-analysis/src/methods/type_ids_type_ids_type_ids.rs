use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::type_ids::TypeIds;
impl TypeIds {
  pub fn new() -> Self {
    Self {
      types: DenseHashMap::new(null()),
      order: Vec::new(),
      hash: 0,
    }
  }
}

impl Default for TypeIds {
  fn default() -> Self {
    Self::new()
  }
}
