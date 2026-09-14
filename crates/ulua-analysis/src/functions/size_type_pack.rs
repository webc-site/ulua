//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:308:size`
//! Source: `Analysis/src/TypePack.cpp:308-315` (hand-ported)

/// C++ `size_t size(TypePackId tp, TxnLog* log = nullptr)`.
use crate::functions::size_type_pack_alt_b;
use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{txn_log::TxnLog, type_pack::TypePack},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};
/// # Safety
/// 调用方须保证 `log` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn size(tp: TypePackId, log: *mut TxnLog) -> usize {
  unsafe {
    let tp = if !log.is_null() {
      (*log).follow_type_pack_id(tp)
    } else {
      follow_type_pack_id(tp)
    };
    if let Some(pack) = TypePack::get_if(&(*tp).ty) {
      size_type_pack_alt_b::size(pack, log)
    } else {
      0
    }
  }
}
