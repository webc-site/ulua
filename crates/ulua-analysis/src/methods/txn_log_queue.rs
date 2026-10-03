use crate::{
  records::{
    arena_handle::alias_ref, arena_id::ArenaId, pending_slot::PendingSlot,
    pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  /// 对应 C++ `PendingType* TxnLog::queue(TypeId)`（`cpp/Analysis/src/TxnLog.cpp:318`）。
  ///
  /// 前置：`ty` 须为指向存活 `Type` 结点的 `TypeId`（C++ `TxnLog::queue(TypeId)` 的 `NotNull`
  /// 形参；本函数会读其 `persistent` 并 `clone()` 出一份值放入日志）。返回的
  /// `*mut PendingType` 指向 `self.type_var_changes` 内 [`PendingSlot`] 的堆对象：在日志
  /// 存活且该键未被移除前地址稳定，调用方不得在其有效期内再次改写同一键（与 C++ 侧对
  /// `PendingType*` 的使用纪律一致）。结点有效性经 `arena_handle::alias_ref` 的模块级
  /// 契约承担，本函数自身无 unsafe 操作。
  pub(crate) fn queue_type_id(&mut self, ty: TypeId) -> *mut PendingType {
    if alias_ref(ty).persistent {
      self.radioactive = true;
    }

    if let Some(existing) = self.type_var_changes.find_mut(&ty) {
      if !existing.get().dead {
        return existing.get_mut() as *mut PendingType;
      }

      let mut pending = alias_ref(ty).clone();
      pending.owning_arena = ArenaId::NONE;
      *existing.get_mut() = PendingType {
        pending,
        dead: false,
      };
      return existing.get_mut() as *mut PendingType;
    }

    let mut pending = alias_ref(ty).clone();
    pending.owning_arena = ArenaId::NONE;
    let (entry, _) = self.type_var_changes.try_insert(
      ty,
      PendingSlot::new(PendingType {
        pending,
        dead: false,
      }),
    );

    entry.get_mut() as *mut PendingType
  }

  /// 对应 C++ `PendingTypePack* TxnLog::queue(TypePackId)`（`cpp/Analysis/src/TxnLog.cpp:335`）。
  ///
  /// 前置：`tp` 须为指向存活 `TypePackVar` 结点的 `TypePackId`（C++ `queue(TypePackId)` 的 `NotNull`
  /// 形参，会读 `persistent` 并 `clone()`）。返回的 `*mut PendingTypePack` 指向
  /// `self.type_pack_changes` 中 [`PendingSlot`] 的堆对象，日志存活且键未被移除前地址稳定。
  /// 结点有效性经 `arena_handle::alias_ref` 的模块级契约承担，本函数自身无 unsafe 操作。
  pub(crate) fn queue_type_pack_id(&mut self, tp: TypePackId) -> *mut PendingTypePack {
    if alias_ref(tp).persistent {
      self.radioactive = true;
    }

    if let Some(existing) = self.type_pack_changes.find_mut(&tp) {
      return existing.get_mut() as *mut PendingTypePack;
    }

    let mut pending = alias_ref(tp).clone();
    pending.owning_arena = ArenaId::NONE;
    let (entry, _) = self
      .type_pack_changes
      .try_insert(tp, PendingSlot::new(PendingTypePack { pending }));

    entry.get_mut() as *mut PendingTypePack
  }
}
