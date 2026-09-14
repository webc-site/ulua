use crate::{methods::txn_log_get_mutable::TxnLogGetMutable, records::txn_log::TxnLog};

impl TxnLog {
  pub fn txn_log_get<T, TID>(&self, _ty: TID) -> *const T
  where
    T: TxnLogGetMutable<TID>,
  {
    self.txn_log_get_mutable::<T, TID>(_ty) as *const T
  }
}
