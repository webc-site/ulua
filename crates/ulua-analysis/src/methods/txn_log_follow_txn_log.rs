//! @interface-stub
use core::ffi::c_void;

use crate::{
  enums::follow_option::FollowOption,
  functions::follow_type_alt_e::follow_full,
  records::{txn_log::TxnLog, r#type::Type},
  type_aliases::type_id::TypeId,
};
fn pending_type_mapper(context: *const c_void, ty: TypeId) -> TypeId {
  let log = unsafe { &*(context as *const TxnLog) };
  let state = log.pending_type_id(ty);

  if state.is_null() {
    ty
  } else {
    unsafe { &(*state).pending as *const Type }
  }
}

impl TxnLog {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn follow_type_id(&self, ty: TypeId) -> TypeId {
    // SAFETY: 裸指针解引用由 follow_full 内部 unsafe 承担。
    follow_full(
      ty,
      FollowOption::Normal,
      self as *const TxnLog as *const c_void,
      pending_type_mapper,
    )
  }
}
