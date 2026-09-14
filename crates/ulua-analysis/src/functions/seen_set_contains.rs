use core::ffi::c_void;

use crate::records::are_equal_state::AreEqualState;
pub fn seen_set_contains(seen: &mut AreEqualState, lhs: *const c_void, rhs: *const c_void) -> bool {
  if lhs == rhs {
    return true;
  }

  let p = (lhs, rhs);
  if seen.seen.contains(&p) {
    return true;
  }

  seen.seen.insert(p);
  false
}
