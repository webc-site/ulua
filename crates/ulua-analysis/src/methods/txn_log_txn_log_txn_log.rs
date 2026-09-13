use core::ptr::null_mut;

use crate::records::txn_log::TxnLog;
impl TxnLog {
  pub fn txn_log(&mut self) {
    self.clear();
    unsafe { self.txn_log_txn_log(null_mut()) };
  }
}
