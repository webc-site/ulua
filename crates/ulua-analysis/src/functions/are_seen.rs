use core::ffi::c_void;
use std::collections::BTreeSet;
pub fn are_seen(
  seen: &mut BTreeSet<(*mut c_void, *mut c_void)>,
  lhs: *const c_void,
  rhs: *const c_void,
) -> bool {
  if lhs == rhs {
    return true;
  }

  let p = (lhs as *mut c_void, rhs as *mut c_void);
  if seen.contains(&p) {
    return true;
  }

  seen.insert(p);
  false
}
