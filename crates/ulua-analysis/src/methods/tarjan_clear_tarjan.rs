use crate::records::{tarjan::Tarjan, txn_log::TxnLog};

impl Tarjan {
  pub fn clear_tarjan(&mut self, log: *const TxnLog) {
    let log = if log.is_null() { TxnLog::empty() } else { log };

    self.type_to_index.clear();
    self.pack_to_index.clear();

    self.nodes.clear();

    self.stack.clear();

    self.child_count = 0;

    self.log = log;

    self.edges_ty.clear();
    self.edges_tp.clear();
    self.worklist.clear();
  }
}
