use crate::{methods::txn_log_get_mutable::TxnLogGetMutable, records::txn_log::TxnLog};

impl TxnLog {
  pub fn txn_log_is<T, TID>(&self, ty: TID) -> bool
  where
    T: TxnLogGetMutable<TID>,
  {
    !self.txn_log_get::<T, TID>(ty).is_null()
  }
}
