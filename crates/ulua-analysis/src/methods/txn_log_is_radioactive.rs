use crate::records::txn_log::TxnLog;

impl TxnLog {
  pub fn is_radioactive(&self) -> bool {
    self.radioactive
  }
}
