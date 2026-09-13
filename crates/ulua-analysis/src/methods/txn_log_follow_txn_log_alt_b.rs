use core::ffi::c_void;

use crate::{
  functions::follow_type_pack_alt_h::follow_pack_full,
  records::{txn_log::TxnLog, type_pack_var::TypePackVar},
  type_aliases::type_pack_id::TypePackId,
};
fn pending_type_pack_mapper(context: *const c_void, tp: TypePackId) -> TypePackId {
  let log = unsafe { &*(context as *const TxnLog) };
  let state = log.pending_type_pack_id(tp);

  if state.is_null() {
    tp
  } else {
    unsafe { &(*state).pending as *const TypePackVar }
  }
}

impl TxnLog {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn follow_type_pack_id(&self, tp: TypePackId) -> TypePackId {
    unsafe {
      follow_pack_full(
        tp,
        self as *const TxnLog as *const c_void,
        pending_type_pack_mapper,
      )
    }
  }
}
