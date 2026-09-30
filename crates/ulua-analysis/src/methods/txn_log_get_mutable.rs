use core::ptr::{from_mut, null_mut};

use crate::{
  functions::{
    get_mutable_txn_log::{get_mutable_pending_type, get_mutable_pending_type_pack},
    get_mutable_type, get_mutable_type_pack,
  },
  records::txn_log::TxnLog,
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};
pub trait TxnLogGetMutable<TID>: Sized {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  unsafe fn get_mutable_from_log(log: &TxnLog, ty: TID) -> *mut Self;
}

impl<T: TypeVariantMember + 'static> TxnLogGetMutable<TypeId> for T {
  unsafe fn get_mutable_from_log(log: &TxnLog, ty: TypeId) -> *mut Self {
    // Safety: `log` 为调用方持有的存活 `&TxnLog`，`ty` 是类型 arena 的存活 TypeId
    // 句柄（C++ `getMutableFromLog` 同前提）。`log.pending_type_id(ty)` 仅在存在
    // 待定节点时返回非空 arena 指针，`!is_null()` 守卫后才交给 `get_mutable_pending_type`
    // 解引用；该函数命中返回 `Some(&'static mut T)`（未命中 `None`，原 null 哨兵），
    // `from_mut` 物化回裸指针以维持本 trait 既有指针面（null/非空判据不变）。
    unsafe {
      let pending_ty = log.pending_type_id(ty);
      if !pending_ty.is_null() {
        return get_mutable_pending_type::<T>(pending_ty).map_or(null_mut(), from_mut);
      }

      get_mutable_type::get_mutable::<T>(ty).map_or(null_mut(), from_mut)
    }
  }
}

impl<T: TypePackVariantMember + 'static> TxnLogGetMutable<TypePackId> for T {
  /// # Safety
  /// `tp` 须为指向存活类型 pack arena 节点的有效 `TypePackId`（内部
  /// `log.pending_type_pack_id(tp)` 与 `get_mutable_type_pack::get_mutable::<T>(tp)` 会解引用
  /// 它）。返回值要么为 null，要么是对该 arena/`log` 内部节点的独占 `*mut Self`
  /// 借用；调用方不得在借用存活期内对同一节点再取可变引用。
  unsafe fn get_mutable_from_log(log: &TxnLog, tp: TypePackId) -> *mut Self {
    // Safety: 逐条满足上方 fn 级契约——`log` 存活、`tp` 是类型 pack arena 的活句柄；
    // `pending_type_pack_id` 非空守卫后才解引用，`get_mutable_type_pack_id` 给出单一
    // 存活节点的独占可变借用，两条路径至多产生一个 *mut，无并存别名。
    unsafe {
      let pending_tp = log.pending_type_pack_id(tp);
      if !pending_tp.is_null() {
        return get_mutable_pending_type_pack::<T>(pending_tp).map_or(null_mut(), from_mut);
      }

      get_mutable_type_pack::get_mutable::<T>(tp).map_or(null_mut(), from_mut)
    }
  }
}

impl TxnLog {
  pub fn txn_log_get_mutable<T, TID>(&self, ty: TID) -> *mut T
  where
    T: TxnLogGetMutable<TID>,
  {
    // Safety: 唯一 unsafe 操作是转发到 `T::get_mutable_from_log`。`self` 是经
    // `&self` 借出的存活 `&TxnLog`（Rust 借用系统保证），`ty` 原样透传给该 fn 级
    // 契约所述的 arena 句柄前提；本包装不额外解引用任何指针，其健全性等价于
    // 调用方对底层 `get_mutable_from_log` 契约的遵守（与 C++ `getMutable` 边界一致）。
    unsafe { T::get_mutable_from_log(self, ty) }
  }
}
