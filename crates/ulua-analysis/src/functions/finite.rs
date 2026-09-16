//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:317:finite`
//! Source: `Analysis/src/TypePack.cpp:317-328` (hand-ported)

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{txn_log::TxnLog, type_pack::TypePack, variadic_type_pack::VariadicTypePack},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

/// # Safety
/// 调用方须保证 `log` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
/// C++ `bool finite(TypePackId tp, TxnLog* log = nullptr)`.
pub unsafe fn finite(tp: TypePackId, log: *mut TxnLog) -> bool {
  unsafe {
    let tp = if !log.is_null() {
      (*log).follow_type_pack_id(tp)
    } else {
      follow_type_pack_id(tp)
    };

    if let Some(pack) = TypePack::get_if(&(*tp).ty) {
      return match pack.tail {
        Some(tail) => finite(tail, log),
        None => true,
      };
    }

    !VariadicTypePack::get_if(&(*tp).ty).is_some()
  }
}
