//! Source: `Analysis/include/Luau/LValue.h:40-44` (hand-ported)
/// C++ `template<typename T> const T* get(const LValue& lvalue)`.
use core::ptr::null;

use crate::type_aliases::l_value::{LValue, LValueMember};
pub fn get_l_value<T: LValueMember>(lvalue: &LValue) -> *const T {
  match T::get_if(lvalue) {
    Some(r) => r as *const T,
    None => null(),
  }
}
