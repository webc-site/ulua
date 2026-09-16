use alloc::{boxed::Box, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::txn_log::{SeenStorage, TxnLog};
impl TxnLog {
  pub fn new() -> Self {
    // Own the seen set in a boxed Vec (stable address, freed on drop) instead
    // of leaking it via `Box::into_raw`.
    let mut seen_box = Box::new(SeenStorage(Vec::new()));
    let shared_seen = &mut seen_box.0 as *mut _;
    Self {
      type_var_changes: DenseHashMap::new(null()),
      type_pack_changes: DenseHashMap::new(null()),
      parent: null_mut(),
      owned_seen: Vec::new(),
      shared_seen,
      owned_seen_box: Some(seen_box),
      radioactive: false,
    }
  }
}

impl Default for TxnLog {
  fn default() -> Self {
    Self::new()
  }
}
