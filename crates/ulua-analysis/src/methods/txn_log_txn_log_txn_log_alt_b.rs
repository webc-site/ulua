use alloc::{boxed::Box, vec::Vec};

use crate::records::txn_log::{SeenStorage, TxnLog};
impl TxnLog {
  /// # Safety
  /// 调用方须保证 `_parent` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn txn_log_txn_log(&mut self, _parent: *mut TxnLog) {
    self.parent = _parent;

    if !_parent.is_null() {
      // Borrow the parent's seen set; release ours if we had one.
      self.shared_seen = unsafe { (*_parent).shared_seen };
      self.owned_seen_box = None;
    } else {
      // Own a fresh seen set (freed on drop) instead of leaking it.
      let mut seen_box = Box::new(SeenStorage(Vec::new()));
      self.shared_seen = &mut seen_box.0 as *mut _;
      self.owned_seen_box = Some(seen_box);
    }
  }
}
