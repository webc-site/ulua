use crate::records::txn_log::TxnLog;

impl TxnLog {
  pub fn clear(&mut self) {
    self.type_var_changes.clear();
    self.type_pack_changes.clear();
  }
}
