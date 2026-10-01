//! pending 表项的槽：`Box` 钉址 + `UnsafeCell` 承载就地写。
//!
//! §2 指针来源净化：`TxnLog::pending_*` 以 `&self` 形态沿父链查表，却要把可变的
//! 数据槽交给下游写穿（C++ `TxnLog::pending` + `getMutable<T>` 的对应物）。原先
//! 由 `from_ref(&T).cast_mut()` 从共享引用派生 `*mut`，属别名模型违例；现在写权限
//! 由类型本身诚实声明——`&self` 路线走 [`PendingSlot::as_ptr`]，独占路线走
//! [`PendingSlot::get_mut`]，二者都不再伪造指针来源。
//!
//! 地址稳定性与迁移前同构：`Box` 保活期内堆对象不移动，故日志存活且键未被移除期间，
//! [`PendingSlot::as_ptr`] 返回值恒等。

use alloc::boxed::Box;
use core::cell::UnsafeCell;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::{
  records::{
    any_type::AnyType, arena_id::ArenaId, pending_type::PendingType,
    pending_type_pack::PendingTypePack, r#type::Type, type_pack_var::TypePackVar, unifiable::Error,
  },
  type_aliases::{
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

#[derive(Debug)]
pub struct PendingSlot<T> {
  cell: Box<UnsafeCell<T>>,
}

impl<T> PendingSlot<T> {
  pub(crate) fn new(value: T) -> Self {
    Self {
      cell: Box::new(UnsafeCell::new(value)),
    }
  }

  /// 独占借用路线：整槽替换或就地写，无 unsafe。
  pub(crate) fn get_mut(&mut self) -> &mut T {
    self.cell.get_mut()
  }

  /// 只读物化（`&self` 路线）。
  pub(crate) fn get(&self) -> &T {
    // SAFETY: 本 crate 由会话单线程驱动，且任一时刻不存在与该读重叠的存活可变
    // 别名——写侧要么持 `&mut` 走 [`PendingSlot::get_mut`]，要么只经
    // [`PendingSlot::as_ptr`] 拿到的裸指针写入且期间不再读该槽。与原
    // `&*from_ref(rep).cast_mut()` 的读法逐位等价，只是指针来源诚实。
    unsafe { &*self.cell.get() }
  }

  /// 地址稳定的数据槽裸指针（`pending` 系列入口的返回形态）。
  pub(crate) fn as_ptr(&self) -> *mut T {
    self.cell.get()
  }
}

impl<T: Clone> Clone for PendingSlot<T> {
  fn clone(&self) -> Self {
    Self {
      cell: Box::new(UnsafeCell::new(self.get().clone())),
    }
  }
}

/// C++ `DenseHashMap` 的空槽占位（墓碑 `dead = true`），语义与原
/// `impl DenseDefault for Box<PendingType>` 逐字一致。
impl DenseDefault for PendingSlot<PendingType> {
  fn dense_default() -> Self {
    PendingSlot::new(PendingType {
      pending: Type::new(TypeVariant::Any(AnyType)),
      dead: true,
    })
  }
}

impl DenseDefault for PendingSlot<PendingTypePack> {
  fn dense_default() -> Self {
    PendingSlot::new(PendingTypePack {
      pending: TypePackVar {
        ty: TypePackVariant::Error(Error::<TypePackId>::new()),
        persistent: false,
        owning_arena: ArenaId::NONE,
      },
    })
  }
}
