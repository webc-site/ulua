//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:330:size`
//! Source: `Analysis/src/TypePack.cpp:330-340` (hand-ported)

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{txn_log::TxnLog, type_pack::TypePack},
  type_aliases::type_pack_variant::TypePackVariantMember,
};

/// # Safety
/// 调用方须保证 `log` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
/// C++ `size_t size(const TypePack& tp, TxnLog* log = nullptr)`.
pub unsafe fn size(tp: &TypePack, log: *mut TxnLog) -> usize {
  unsafe {
    let mut result = tp.head.len();
    if let Some(tp_tail) = tp.tail {
      let followed = if !log.is_null() {
        (*log).follow_type_pack_id(tp_tail)
      } else {
        follow_type_pack_id(tp_tail)
      };
      if let Some(tail) = TypePack::get_if(&(*followed).ty) {
        result += size(tail, log);
      }
    }
    result
  }
}
