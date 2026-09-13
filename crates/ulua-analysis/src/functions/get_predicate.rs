//! Source: `Analysis/include/Luau/Predicate.h:87-91` (hand-ported)
/// C++ `template<typename T> const T* get(const Predicate& predicate)`.
use core::ptr::null;

use crate::type_aliases::predicate::{Predicate, PredicateMember};
pub fn get_predicate<T: PredicateMember>(predicate: &Predicate) -> *const T {
  match T::get_if(predicate) {
    Some(r) => r as *const T,
    None => null(),
  }
}
