use core::ptr::NonNull;

use crate::{
  functions::{
    get_mutable_txn_log::{get_mutable_pending_type, get_mutable_pending_type_pack},
    get_mutable_type, get_mutable_type_pack,
  },
  records::{arena_handle::Handle, txn_log::TxnLog},
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};
/// txn log 可变查询面的 Rust 形态（review.md §2：可空指针 → `Option`）：
/// `Some(NonNull)` ≡ C++ `getMutable` 命中 arena/log 存活节点，`None` ≡ 原
/// null 哨兵（未命中）。底层 `get_mutable_*` 族本就返回 `Option<&'static mut T>`，
/// 这里以 `NonNull::from` 折叠回句柄形态、不再经 `null_mut` 折返裸指针。
///
/// 保留 `NonNull` 而非 `&'static mut`：TxnLog 走 cpp 的「内部可变 + arena 地址
/// 稳定」语义，消费点要么把句柄存进 `Clone`/`Copy` 记录字段（weird_iter、
/// type_pack_iterator）、要么按地址比对判等（unifier 表项换址检测），这些用法
/// 引用模型无法机械表达，故以 `Option<NonNull<T>>` 收口——判空即 `is_none`，
/// 解引用统一经 `arena_handle::alias_nn*` 门面，unsafe 不外渗。
///
/// 形参为 `&TxnLog` 与 arena 句柄（`TypeId`/`TypePackId`），体内解引用全部经
/// safe 门面（`pending_*_id`/`get_mutable*`/`Handle::from_opt_ptr`）完成，
/// 签名不再以 `unsafe fn` 表达（review.md §2 收形）。
pub trait TxnLogGetMutable<TID>: Sized {
  /// 调用序契约（正确性，非内存安全）：`ty` 须为本 `log` 所属类型 arena 中存活
  /// 节点的 `TypeId` 句柄，且 `log` 在调用期内单线程可访问——句柄的解引用由
  /// `pending_type_id`/`get_mutable_*` 各 safe 门面内部收口，其存活/对齐前提由
  /// arena 不变量保证，违反调用序只会查错节点得到 `None`/错值，不越出内存安全
  /// 边界。返回值 `Some(NonNull)` 是该 arena/log 槽位的可变句柄：调用方不得在
  /// 其借用存续期内对同一节点再取第二处可变句柄（独占性属数据竞争纪律，由
  /// unifier/substitution 等调用序保证），也不得在节点释放后继续解引用它。
  fn get_mutable_from_log(log: &TxnLog, ty: TID) -> Option<NonNull<Self>>;
}

impl<T: TypeVariantMember + 'static> TxnLogGetMutable<TypeId> for T {
  fn get_mutable_from_log(log: &TxnLog, ty: TypeId) -> Option<NonNull<Self>> {
    // `log` 为存活 `&TxnLog`，`ty` 是类型 arena 句柄；`log.pending_type_id(ty)`
    // 仅在存在待定节点时返回非空 arena 指针，`Handle::from_opt_ptr` 把 null 哨兵
    // 折叠为 `None`、非空折叠为句柄后才交给 `get_mutable_pending_type`（该函数收
    // `Handle<PendingType>`，非空由 Handle 类型编码）；命中返回
    // `Some(&'static mut T)`（未命中 `None`，原 null 哨兵），经 `NonNull::from`
    // 折叠为 `Option<NonNull<T>>`（null/非空判据不变）。
    let pending_ty = log.pending_type_id(ty);
    if let Some(pending_ty) = Handle::from_opt_ptr(pending_ty) {
      return get_mutable_pending_type::<T>(pending_ty).map(NonNull::from);
    }

    get_mutable_type::get_mutable::<T>(ty).map(NonNull::from)
  }
}

impl<T: TypePackVariantMember + 'static> TxnLogGetMutable<TypePackId> for T {
  /// 调用序契约同上（`get_mutable_from_log` 文档）：`tp` 为存活类型 pack arena
  /// 句柄，`pending_type_pack_id` 的 null 哨兵经 `Handle::from_opt_ptr` 折叠后
  /// 才交给 `get_mutable_pending_type_pack`，`get_mutable_type_pack` 给出单一
  /// 存活节点的独占可变借用，两条路径至多产生一个句柄。
  fn get_mutable_from_log(log: &TxnLog, tp: TypePackId) -> Option<NonNull<Self>> {
    let pending_tp = log.pending_type_pack_id(tp);
    if let Some(pending_tp) = Handle::from_opt_ptr(pending_tp) {
      return get_mutable_pending_type_pack::<T>(pending_tp).map(NonNull::from);
    }

    get_mutable_type_pack::get_mutable::<T>(tp).map(NonNull::from)
  }
}

impl TxnLog {
  pub fn txn_log_get_mutable<T, TID>(&self, ty: TID) -> Option<NonNull<T>>
  where
    T: TxnLogGetMutable<TID>,
  {
    T::get_mutable_from_log(self, ty)
  }
}
