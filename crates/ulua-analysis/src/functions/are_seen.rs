use core::ffi::c_void;

use crate::type_aliases::seen_set_structural_type_equality::SeenSet;

pub fn are_seen(seen: &mut SeenSet, lhs: *const c_void, rhs: *const c_void) -> bool {
  if lhs == rhs {
    return true;
  }

  if seen.contains(&(lhs, rhs)) {
    return true;
  }

  seen.insert((lhs, rhs));
  false
}
