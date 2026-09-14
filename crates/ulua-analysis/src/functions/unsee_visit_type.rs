use core::ffi::c_void;
use std::collections::HashSet;
pub fn unsee(seen: &mut HashSet<*mut c_void>, tv: *const c_void) {
  let ttv = tv as *mut c_void;
  seen.remove(&ttv);
}
