//! Source: `Analysis/include/Luau/Def.h:73-77` (hand-ported)
/// C++ `template<typename T> const T* get(DefId def)`.
use core::ptr::null;

use crate::type_aliases::{def_id_def::DefId, variant::VariantMember};
/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn get_def_id<T: VariantMember>(def: DefId) -> *const T {
  unsafe {
    match T::get_if(&(*def).v) {
      Some(r) => r as *const T,
      None => null(),
    }
  }
}
