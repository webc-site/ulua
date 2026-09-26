//! Source: `Analysis/src/TypePack.cpp:317-328` (hand-ported)

use crate::{
  functions::{follow_type_pack, get_type_pack::type_pack_variant_of},
  records::{txn_log::TxnLog, type_pack::TypePack, variadic_type_pack::VariadicTypePack},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

/// # Safety
/// `log` 须指向调用方持有的存活 `TxnLog`（非空、对齐，比本查询长寿）；本函数只读其 follow 链，
/// 单线程独占。对应 C++ `bool finite(TypePackId tp, TxnLog* log)` (`cpp/Analysis/src/TypePack.cpp:317`)。
/// C++ `bool finite(TypePackId tp, TxnLog* log = nullptr)`.
pub unsafe fn finite(tp: TypePackId, log: *mut TxnLog) -> bool {
  // Safety: `log` 由本函数 unsafe fn 契约保证指向存活的 TxnLog，且仅在
  // `!log.is_null()` 守卫通过后才 `(*log)` 解引用；单线程只读遍历，无别名冲突。
  // pack 变体读取收口在 `type_pack_variant_of`（arena 节点契约同 C++ get）。
  let tp = if !log.is_null() {
    // SAFETY: log 非空且存活，见函数 `# Safety` 契约。
    unsafe { (*log).follow_type_pack_id(tp) }
  } else {
    follow_type_pack::follow(tp)
  };

  if let Some(pack) = TypePack::get_if(type_pack_variant_of(tp)) {
    return match pack.tail {
      // SAFETY: tail 为同一 arena 存活的 pack 句柄；log 原样透传，契约不变。
      Some(tail) => unsafe { finite(tail, log) },
      None => true,
    };
  }

  VariadicTypePack::get_if(type_pack_variant_of(tp)).is_none()
}
