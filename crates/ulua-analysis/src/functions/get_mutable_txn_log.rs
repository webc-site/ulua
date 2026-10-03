use crate::{
  functions::{get_mutable_type, get_mutable_type_pack},
  records::{
    arena_handle::Handle, pending_type::PendingType, pending_type_pack::PendingTypePack,
  },
  type_aliases::{type_pack_variant::TypePackVariantMember, type_variant::TypeVariantMember},
};

/// 对应 C++ `T* getMutable(PendingType*)`（`cpp/Analysis/include/Luau/TxnLog.h:52-56`）。
///
/// §2（裸指针 → Rust 类型）：C++ 的「未命中返回 null」哨兵在 Rust 侧收口为
/// `Option<&mut T>`，与 [`get_mutable_type_id`] 的返回形态同构；调用方以
/// `if let`/`match` 甄别，不再手写 `is_null()` 判空。
///
/// 句柄化：形参为 [`Handle<PendingType>`]——非空由类型编码，目标指向 TxnLog
/// `type_var_changes` 内 `PendingSlot` 堆对象、日志存活且键未被移除前地址稳定
/// （由 `queue_type_id`/`pending_type_id` 的构造不变量保证），故本函数 safe。
/// 返回值 `Some` 即 arena/log 内 `T` 槽位的独占可变借用（写入期间不得有第二处
/// 可变借用同一槽位，unifier 写穿协议），`None` 即「无该变体的 txn log 记录」
/// （原 null 哨兵的同义替代）。
pub(crate) fn get_mutable_pending_type<T: TypeVariantMember + 'static>(
  pending: Handle<PendingType>,
) -> Option<&'static mut T> {
  // `pending` 非空且指向 TxnLog 存活期内地址稳定的 `PendingType`（Handle 类型
  // 契约）；`pending.get()` 仅借出其内嵌 `Type` 作 `TypeId` 输入，
  // `get_mutable_type::get_mutable` 命中返回 arena 内存活的 `&'static mut T`、
  // 未命中返回 None，别名纪律与原裸指针形态逐位同构（单线程，写穿协议由调用方维持）。
  // We use get_mutable here because this state is intended to be mutated freely.
  get_mutable_type::get_mutable::<T>(&pending.get().pending)
}

/// 对应 C++ `T* getMutable(PendingTypePack*)`（`cpp/Analysis/include/Luau/TxnLog.h:59-63`）。
///
/// §2：null 哨兵收口为 `Option<&mut T>`（同 [`get_mutable_pending_type`]）。
///
/// 句柄化：形参为 [`Handle<PendingTypePack>`]——非空由类型编码，目标指向
/// `type_pack_changes` 内 `PendingSlot` 堆对象、日志存活期内地址稳定，故本函数
/// safe。返回值 `Some` 即 arena/log 内 `T` 槽位的独占可变借用（写入期间无第二处
/// 可变借用同一槽位），`None` 即原 null 哨兵的「未命中」语义。
pub(crate) fn get_mutable_pending_type_pack<T: TypePackVariantMember + 'static>(
  pending: Handle<PendingTypePack>,
) -> Option<&'static mut T> {
  // 同 TypeId 侧：`pending.get()` 仅借出内嵌 `TypePack` 作 `TypePackId` 输入；
  // `get_mutable_type_pack::get_mutable` 命中返回 arena 内存活的 `&'static mut T`、
  // 未命中返回 None。单线程无别名。
  // We use get_mutable here because this state is intended to be mutated freely.
  get_mutable_type_pack::get_mutable::<T>(&pending.get().pending)
}
