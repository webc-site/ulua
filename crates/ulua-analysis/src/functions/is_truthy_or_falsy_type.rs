use core::{ffi::c_void, ptr::null};

use crate::{
  enums::follow_option::FollowOption,
  functions::{
    follow_type_alt_e::follow_full, is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
  },
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn is_truthy_or_falsy_type(ty: TypeId) -> bool {
  // SAFETY: ty 的有效性由调用方按 C++ 同契约保证；unsafe 由 follow_full 承担。
  let ty = follow_full(ty, FollowOption::Normal, null(), identity_mapper);
  is_approximately_truthy_type(ty) || is_approximately_falsy_type(ty)
}

fn identity_mapper(_context: *const c_void, t: TypeId) -> TypeId {
  t
}
