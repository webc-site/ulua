use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::txn_log::TxnLog;
impl TxnLog {
  pub fn clear(&mut self) {
    self.type_var_changes = DenseHashMap::new(null());
    self.type_pack_changes = DenseHashMap::new(null());
  }
}
