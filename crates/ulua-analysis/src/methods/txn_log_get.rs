use crate::{methods::txn_log_get_mutable::TxnLogGetMutable, records::txn_log::TxnLog};

impl TxnLog {
  /// C++ `TxnLog::get<T>(TID)` 的 Rust 惯用形态：命中变体返回引用，否则 `None`。
  /// 判空与解引用合并为一次安全操作；引用借用本 log（pending 存储为 Box，
  /// 地址稳定；arena 节点寿命更长），可变性场景仍走 [`TxnLog::txn_log_get_mutable`]。
  pub fn txn_log_get<T, TID>(&self, _ty: TID) -> Option<&T>
  where
    T: TxnLogGetMutable<TID>,
  {
    // SAFETY: 指针来自 txn_log_get_mutable——null 或指向 log/arena 内存活节点。
    unsafe { self.txn_log_get_mutable::<T, TID>(_ty).as_ref() }
  }
}
