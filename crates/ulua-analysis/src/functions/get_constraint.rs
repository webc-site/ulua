//! Source: `Analysis/include/Luau/Constraint.h:374-382` (hand-ported)
/// C++ `template<typename T> const T* get(const Constraint& c)`.
use core::ptr::null;

use crate::{records::constraint::Constraint, type_aliases::constraint_v::ConstraintVMember};
pub fn get_constraint<T: ConstraintVMember>(c: &Constraint) -> *const T {
  match T::get_if(&c.c) {
    Some(r) => r as *const T,
    None => null(),
  }
}
