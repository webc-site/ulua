use alloc::{boxed::Box, vec::Vec};
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog,
};
impl TxnLog {
  pub fn inverse(&self) -> TxnLog {
    // C++ `TxnLog inversed(sharedSeen)` — the sharedSeen-taking constructor.
    let mut inversed = TxnLog {
      type_var_changes: DenseHashMap::default(),
      type_pack_changes: DenseHashMap::default(),
      parent: null_mut(),
      owned_seen: Vec::new(),
      // Borrows the original's seen set (C++ `inversed(sharedSeen)`).
      shared_seen: self.shared_seen,
      owned_seen_box: None,
      radioactive: false,
    };

    for (ty, rep) in self.type_var_changes.iter() {
      if !rep.dead {
        // Safety: 键是本次事务记录时写入的 *const Type arena 节点地址——
        // TypedAllocator bump 块地址不移动、节点在会话 arena 存活期内常驻，
        // 而本 TxnLog 的使用严格嵌于分配它的 arena 存活期；`(**ty)` 只读克隆
        // 当前节点值快照（C++ `TypeVar(entry.first->ty)` 同语义），无并发写。
        let pending_ty = unsafe { (**ty).clone() };
        inversed.type_var_changes.try_insert(
          *ty,
          Box::new(PendingType {
            pending: pending_ty,
            dead: false,
          }),
        );
      }
    }

    for (tp, _rep) in self.type_pack_changes.iter() {
      // Safety: 同 type_var_changes——键为事务期记录的类型 pack arena 节点地址，
      // bump 块不移动、节点比本 log 长寿；只读克隆作回滚快照，单线程无别名。
      let pending_tp = unsafe { (**tp).clone() };
      inversed.type_pack_changes.try_insert(
        *tp,
        Box::new(PendingTypePack {
          pending: pending_tp,
        }),
      );
    }

    inversed.radioactive = self.radioactive;

    inversed
  }
}
