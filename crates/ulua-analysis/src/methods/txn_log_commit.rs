use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    occurs_txn_log::occurs_txn_log_type_id_type_id,
  },
  records::{pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn commit(&mut self) {
    LUAU_ASSERT!(!self.radioactive);

    // The change maps are not mutated by occurs()/followOnce() (those only read
    // pending state), and Box gives stable addresses, so we snapshot the raw
    // pointers up front. This mirrors C++ iterating the map while passing
    // `*this` to occurs().
    let type_var_entries: Vec<(TypeId, *const PendingType, bool)> = self
      .type_var_changes
      .iter()
      .map(|(ty, rep)| (*ty, rep.as_ref() as *const PendingType, rep.dead))
      .collect();

    for (ty, rep, dead) in type_var_entries {
      if !dead {
        // Safety: rep 是函数头从映射取出的 Box<PendingType> 堆地址快照——Box
        // 内容地址稳定且映射在循环内不变更，故非空、对齐、存活；取内嵌 pending
        // 字段地址仅做指针拷贝（TypeId 即 *const Type），生成的裸指针不与任何
        // &mut 借用重叠，pending 独立分配于 Box 堆中。
        let unfollowed: TypeId = unsafe { &(*rep).pending as *const _ };

        if !occurs_txn_log_type_id_type_id(self, unfollowed, ty) {
          // Safety: as_mutable_type_id 直译 C++ asMutable 的 const_cast——ty 是
          // 本会话 arena 中处于 pending 期的可变节点，occurs() 的只读借用已随
          // 上一行返回失效，单线程下此刻是唯一活动可变窗口；&*unfollowed 的来源
          // 是上方 Box<PendingType> 堆内 pending 字段，与 arena 节点异址，读写
          // 双方无别名重叠。
          unsafe {
            (*as_mutable_type_id(ty)).reassign(&*unfollowed);
          }
        }
      }
    }

    let type_pack_entries: Vec<(TypePackId, *const PendingTypePack)> = self
      .type_pack_changes
      .iter()
      .map(|(tp, rep)| (*tp, rep.as_ref() as *const PendingTypePack))
      .collect();

    for (tp, rep) in type_pack_entries {
      // Safety: 与 TypeId 分支同一不变量——rep 为 Box<PendingTypePack> 的稳定堆
      // 地址快照且映射在循环内不变更；as_mutable_type_pack 取得会话 arena 中该
      // pending pack 的唯一可变句柄，&(*rep).pending 指向 Box 堆内独立副本，
      // 与写入目标异址无别名，单线程串行。
      unsafe {
        (*as_mutable_type_pack(tp)).reassign(&(*rep).pending);
      }
    }

    self.clear();
  }
}
