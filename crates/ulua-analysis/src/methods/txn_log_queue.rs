use crate::{
  records::{
    arena_id::ArenaId, pending_type::PendingType, pending_type_pack::PendingTypePack,
    txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  /// 对应 C++ `PendingType* TxnLog::queue(TypeId)`（`cpp/Analysis/src/TxnLog.cpp:318`）。
  ///
  /// # Safety
  /// `ty` 须为指向存活 `Type` 结点的 `TypeId`（C++ `TxnLog::queue(TypeId)` 的 `NotNull`
  /// 形参；本函数会读其 `persistent` 并 `clone()` 出一份值放入日志）。返回的
  /// `*mut PendingType` 指向 `self.type_var_changes` 内 `Box` 的堆对象：在日志存活且该键
  /// 未被移除前地址稳定，调用方不得在其有效期内再次改写同一键（与 C++ 侧对
  /// `PendingType*` 的使用纪律一致）。
  pub unsafe fn queue_type_id(&mut self, ty: TypeId) -> *mut PendingType {
    // Safety: `ty` 按上述契约非空且指向 bump arena 内的存活 `Type`（块地址不移动）；
    // `(*ty).clone()` 是值拷贝，产出的 `pending` 由本函数的 `Box::new` 独立持有，不会与
    // arena 里的原结点形成双重所有者。`owning_arena = ArenaId::NONE` 复刻 C++ 置
    // nullptr：日志副本脱离原 arena 的回收路径。`existing.as_mut()` 经 `Box` 解引用
    // 得到堆上稳定地址，
    // `try_insert` 返回的 `entry` 同理，故两处 `as *mut PendingType` 都指向本日志独占
    // 持有的对象；`&mut self` 保证调用期间无第二个可变借用该 map。
    unsafe {
      if (*ty).persistent {
        self.radioactive = true;
      }

      if let Some(existing) = self.type_var_changes.find_mut(&ty) {
        if !existing.dead {
          return existing.as_mut() as *mut PendingType;
        }

        let mut pending = (*ty).clone();
        pending.owning_arena = ArenaId::NONE;
        **existing = PendingType {
          pending,
          dead: false,
        };
        return existing.as_mut() as *mut PendingType;
      }

      let mut pending = (*ty).clone();
      pending.owning_arena = ArenaId::NONE;
      let (entry, _) = self.type_var_changes.try_insert(
        ty,
        Box::new(PendingType {
          pending,
          dead: false,
        }),
      );

      entry.as_mut() as *mut PendingType
    }
  }

  /// 对应 C++ `PendingTypePack* TxnLog::queue(TypePackId)`（`cpp/Analysis/src/TxnLog.cpp:335`）。
  ///
  /// # Safety
  /// `tp` 须为指向存活 `TypePackVar` 结点的 `TypePackId`（C++ `queue(TypePackId)` 的 `NotNull`
  /// 形参，会读 `persistent` 并 `clone()`）。返回的 `*mut PendingTypePack` 指向
  /// `self.type_pack_changes` 中 `Box` 的堆对象，日志存活且键未被移除前地址稳定。
  pub unsafe fn queue_type_pack_id(&mut self, tp: TypePackId) -> *mut PendingTypePack {
    // Safety: 与 `queue_type_id` 同——`tp` 按契约非空且指向 arena 内存活结点；
    // `(*tp).clone()` 只复制值，结点所有权仍归 arena，不产生第二所有者；
    // `existing.as_mut()`／`entry.as_mut()` 均经 `Box` 解引用取得堆上稳定地址，
    // 由本日志独占持有，`&mut self` 排除了同一 map 的并发可变借用。
    unsafe {
      if (*tp).persistent {
        self.radioactive = true;
      }

      if let Some(existing) = self.type_pack_changes.find_mut(&tp) {
        return existing.as_mut() as *mut PendingTypePack;
      }

      let mut pending = (*tp).clone();
      pending.owning_arena = ArenaId::NONE;
      let (entry, _) = self
        .type_pack_changes
        .try_insert(tp, Box::new(PendingTypePack { pending }));

      entry.as_mut() as *mut PendingTypePack
    }
  }
}
