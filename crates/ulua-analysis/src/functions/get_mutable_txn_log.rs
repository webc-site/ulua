use crate::{
  functions::{get_mutable_type, get_mutable_type_pack},
  records::{pending_type::PendingType, pending_type_pack::PendingTypePack},
  type_aliases::{type_pack_variant::TypePackVariantMember, type_variant::TypeVariantMember},
};

/// 对应 C++ `T* getMutable(PendingType*)`（`cpp/Analysis/include/Luau/TxnLog.h:52-56`）。
///
/// §2（裸指针 → Rust 类型）：C++ 的「未命中返回 null」哨兵在 Rust 侧收口为
/// `Option<&mut T>`，与 [`get_mutable_type_id`] 的返回形态同构；调用方以
/// `if let`/`match` 甄别，不再手写 `is_null()` 判空。
///
/// # Safety
/// `pending` 须为非空、对齐且由其所属 `TxnLog` 在调用期内保活的 `*mut PendingType`
/// （cpp 侧由 `TxnLog::queue` 产出、Box 语义地址稳定）。返回值 `Some` 即 arena/log 内
/// `T` 槽位的独占可变借用（写入期间不得有第二处可变借用同一槽位，unifier 写穿协议），
/// `None` 即「无该变体的 txn log 记录」（原 null 哨兵的同义替代）。
pub unsafe fn get_mutable_pending_type<T: TypeVariantMember + 'static>(
  pending: *mut PendingType,
) -> Option<&'static mut T> {
  unsafe {
    // Safety: `pending` 为 TxnLog 在其存活期内保活的非空 `*mut PendingType`（C++ TxnLog.h:51
    // 同款），`&(*pending).pending` 仅借出其内嵌 `Type` 作输入；`get_mutable_type_id` 命中返回
    // arena 内存活的 `&'static mut T`、未命中返回 None，别名纪律与原裸指针形态逐位同构
    // （单线程，写穿协议由调用方维持）。
    // We use get_mutable here because this state is intended to be mutated freely.
    get_mutable_type::get_mutable::<T>(&(*pending).pending)
  }
}

/// 对应 C++ `T* getMutable(PendingTypePack*)`（`cpp/Analysis/include/Luau/TxnLog.h:59-63`）。
///
/// §2：null 哨兵收口为 `Option<&mut T>`（同 [`get_mutable_pending_type`]）。
///
/// # Safety
/// `pending` 须为非空、对齐且由其所属 `TxnLog` 在调用期内保活的 `*mut PendingTypePack`
/// （cpp 侧由 `TxnLog::queue` 产出）。返回值 `Some` 即 arena/log 内 `T` 槽位的独占可变
/// 借用（写入期间无第二处可变借用同一槽位），`None` 即原 null 哨兵的「未命中」语义。
pub unsafe fn get_mutable_pending_type_pack<T: TypePackVariantMember + 'static>(
  pending: *mut PendingTypePack,
) -> Option<&'static mut T> {
  unsafe {
    // Safety: `pending` 为 TxnLog 在其存活期内保活的非空 `*mut PendingTypePack`（C++
    // TxnLog.h:58 同款），`&(*pending).pending` 仅借出其内嵌 `TypePack` 作输入；
    // `get_mutable_type_pack_id` 命中返回 arena 内存活的 `&'static mut T`、未命中返回 None。
    // 单线程无别名。
    // We use get_mutable here because this state is intended to be mutated freely.
    get_mutable_type_pack::get_mutable::<T>(&(*pending).pending)
  }
}
