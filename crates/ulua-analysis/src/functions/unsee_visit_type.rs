use core::ffi::c_void;

use crate::type_aliases::collections::HashSet;
pub fn unsee(seen: &mut HashSet<*mut c_void>, tv: *const c_void) {
  let ttv = tv as *mut c_void;
  seen.remove(&ttv);
}
